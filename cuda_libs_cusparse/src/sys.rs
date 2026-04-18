use cuda_libs_cudart::sys::*;
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage> {
    storage: Storage,
}
impl<Storage> __BindgenBitfieldUnit<Storage> {
    #[inline]
    pub const fn new(storage: Storage) -> Self {
        Self { storage }
    }
}
impl<Storage> __BindgenBitfieldUnit<Storage>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    fn extract_bit(byte: u8, index: usize) -> bool {
        let bit_index = if cfg!(target_endian = "big") { 7 - (index % 8) } else { index % 8 };
        let mask = 1 << bit_index;
        byte & mask == mask
    }
    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        Self::extract_bit(byte, index)
    }
    #[inline]
    pub unsafe fn raw_get_bit(this: *const Self, index: usize) -> bool {
        debug_assert!(index / 8 < core::mem::size_of::<Storage>());
        let byte_index = index / 8;
        let byte = unsafe { *(core::ptr::addr_of!((*this).storage) as *const u8).offset(byte_index as isize) };
        Self::extract_bit(byte, index)
    }
    #[inline]
    fn change_bit(byte: u8, index: usize, val: bool) -> u8 {
        let bit_index = if cfg!(target_endian = "big") { 7 - (index % 8) } else { index % 8 };
        let mask = 1 << bit_index;
        if val { byte | mask } else { byte & !mask }
    }
    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        *byte = Self::change_bit(*byte, index, val);
    }
    #[inline]
    pub unsafe fn raw_set_bit(this: *mut Self, index: usize, val: bool) {
        debug_assert!(index / 8 < core::mem::size_of::<Storage>());
        let byte_index = index / 8;
        let byte = unsafe { (core::ptr::addr_of_mut!((*this).storage) as *mut u8).offset(byte_index as isize) };
        unsafe { *byte = Self::change_bit(*byte, index, val) };
    }
    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                let index = if cfg!(target_endian = "big") { bit_width as usize - 1 - i } else { i };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub unsafe fn raw_get(this: *const Self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < core::mem::size_of::<Storage>());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= core::mem::size_of::<Storage>());
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if unsafe { Self::raw_get_bit(this, i + bit_offset) } {
                let index = if cfg!(target_endian = "big") { bit_width as usize - 1 - i } else { i };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") { bit_width as usize - 1 - i } else { i };
            self.set_bit(index + bit_offset, val_bit_is_set);
        }
    }
    #[inline]
    pub unsafe fn raw_set(this: *mut Self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < core::mem::size_of::<Storage>());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= core::mem::size_of::<Storage>());
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") { bit_width as usize - 1 - i } else { i };
            unsafe { Self::raw_set_bit(this, index + bit_offset, val_bit_is_set) };
        }
    }
}
pub const CUSPARSE_VER_MAJOR: u32 = 12;
pub const CUSPARSE_VER_MINOR: u32 = 7;
pub const CUSPARSE_VER_PATCH: u32 = 10;
pub const CUSPARSE_VER_BUILD: u32 = 1;
pub const CUSPARSE_VERSION: u32 = 12710;
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
pub type __uint64_t = ::std::os::raw::c_ulong;
pub type __off_t = ::std::os::raw::c_long;
pub type __off64_t = ::std::os::raw::c_long;
pub type FILE = _IO_FILE;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_marker {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_codecvt {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_wide_data {
    _unused: [u8; 0],
}
pub type _IO_lock_t = ::std::os::raw::c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_FILE {
    pub _flags: ::std::os::raw::c_int,
    pub _IO_read_ptr: *mut ::std::os::raw::c_char,
    pub _IO_read_end: *mut ::std::os::raw::c_char,
    pub _IO_read_base: *mut ::std::os::raw::c_char,
    pub _IO_write_base: *mut ::std::os::raw::c_char,
    pub _IO_write_ptr: *mut ::std::os::raw::c_char,
    pub _IO_write_end: *mut ::std::os::raw::c_char,
    pub _IO_buf_base: *mut ::std::os::raw::c_char,
    pub _IO_buf_end: *mut ::std::os::raw::c_char,
    pub _IO_save_base: *mut ::std::os::raw::c_char,
    pub _IO_backup_base: *mut ::std::os::raw::c_char,
    pub _IO_save_end: *mut ::std::os::raw::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::std::os::raw::c_int,
    pub _bitfield_align_1: [u32; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 3usize]>,
    pub _short_backupbuf: [::std::os::raw::c_char; 1usize],
    pub _old_offset: __off_t,
    pub _cur_column: ::std::os::raw::c_ushort,
    pub _vtable_offset: ::std::os::raw::c_schar,
    pub _shortbuf: [::std::os::raw::c_char; 1usize],
    pub _lock: *mut _IO_lock_t,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::std::os::raw::c_void,
    pub _prevchain: *mut *mut _IO_FILE,
    pub _mode: ::std::os::raw::c_int,
    pub _unused3: ::std::os::raw::c_int,
    pub _total_written: __uint64_t,
    pub _unused2: [::std::os::raw::c_char; 8usize],
}
impl Default for _IO_FILE {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl _IO_FILE {
    #[inline]
    pub fn _flags2(&self) -> ::std::os::raw::c_int {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 24u8) as u32) }
    }
    #[inline]
    pub fn set__flags2(&mut self, val: ::std::os::raw::c_int) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 24u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn _flags2_raw(this: *const Self) -> ::std::os::raw::c_int {
        unsafe { ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 3usize]>>::raw_get(::std::ptr::addr_of!((*this)._bitfield_1), 0usize, 24u8) as u32) }
    }
    #[inline]
    pub unsafe fn set__flags2_raw(this: *mut Self, val: ::std::os::raw::c_int) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 3usize]>>::raw_set(::std::ptr::addr_of_mut!((*this)._bitfield_1), 0usize, 24u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(_flags2: ::std::os::raw::c_int) -> __BindgenBitfieldUnit<[u8; 3usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 3usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 24u8, {
            let _flags2: u32 = unsafe { ::std::mem::transmute(_flags2) };
            _flags2 as u64
        });
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cusparseContext {
    _unused: [u8; 0],
}
pub type cusparseHandle_t = *mut cusparseContext;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cusparseMatDescr {
    _unused: [u8; 0],
}
pub type cusparseMatDescr_t = *mut cusparseMatDescr;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bsrsv2Info {
    _unused: [u8; 0],
}
pub type bsrsv2Info_t = *mut bsrsv2Info;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bsrsm2Info {
    _unused: [u8; 0],
}
pub type bsrsm2Info_t = *mut bsrsm2Info;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct csric02Info {
    _unused: [u8; 0],
}
pub type csric02Info_t = *mut csric02Info;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bsric02Info {
    _unused: [u8; 0],
}
pub type bsric02Info_t = *mut bsric02Info;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct csrilu02Info {
    _unused: [u8; 0],
}
pub type csrilu02Info_t = *mut csrilu02Info;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bsrilu02Info {
    _unused: [u8; 0],
}
pub type bsrilu02Info_t = *mut bsrilu02Info;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct csru2csrInfo {
    _unused: [u8; 0],
}
pub type csru2csrInfo_t = *mut csru2csrInfo;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cusparseColorInfo {
    _unused: [u8; 0],
}
pub type cusparseColorInfo_t = *mut cusparseColorInfo;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pruneInfo {
    _unused: [u8; 0],
}
pub type pruneInfo_t = *mut pruneInfo;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cusparseStatus_t {
    CUSPARSE_STATUS_SUCCESS = 0,
    CUSPARSE_STATUS_NOT_INITIALIZED = 1,
    CUSPARSE_STATUS_ALLOC_FAILED = 2,
    CUSPARSE_STATUS_INVALID_VALUE = 3,
    CUSPARSE_STATUS_ARCH_MISMATCH = 4,
    CUSPARSE_STATUS_MAPPING_ERROR = 5,
    CUSPARSE_STATUS_EXECUTION_FAILED = 6,
    CUSPARSE_STATUS_INTERNAL_ERROR = 7,
    CUSPARSE_STATUS_MATRIX_TYPE_NOT_SUPPORTED = 8,
    CUSPARSE_STATUS_ZERO_PIVOT = 9,
    CUSPARSE_STATUS_NOT_SUPPORTED = 10,
    CUSPARSE_STATUS_INSUFFICIENT_RESOURCES = 11,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cusparsePointerMode_t {
    CUSPARSE_POINTER_MODE_HOST = 0,
    CUSPARSE_POINTER_MODE_DEVICE = 1,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cusparseAction_t {
    CUSPARSE_ACTION_SYMBOLIC = 0,
    CUSPARSE_ACTION_NUMERIC = 1,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cusparseMatrixType_t {
    CUSPARSE_MATRIX_TYPE_GENERAL = 0,
    CUSPARSE_MATRIX_TYPE_SYMMETRIC = 1,
    CUSPARSE_MATRIX_TYPE_HERMITIAN = 2,
    CUSPARSE_MATRIX_TYPE_TRIANGULAR = 3,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cusparseFillMode_t {
    CUSPARSE_FILL_MODE_LOWER = 0,
    CUSPARSE_FILL_MODE_UPPER = 1,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cusparseDiagType_t {
    CUSPARSE_DIAG_TYPE_NON_UNIT = 0,
    CUSPARSE_DIAG_TYPE_UNIT = 1,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cusparseIndexBase_t {
    CUSPARSE_INDEX_BASE_ZERO = 0,
    CUSPARSE_INDEX_BASE_ONE = 1,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cusparseOperation_t {
    CUSPARSE_OPERATION_NON_TRANSPOSE = 0,
    CUSPARSE_OPERATION_TRANSPOSE = 1,
    CUSPARSE_OPERATION_CONJUGATE_TRANSPOSE = 2,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cusparseDirection_t {
    CUSPARSE_DIRECTION_ROW = 0,
    CUSPARSE_DIRECTION_COLUMN = 1,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cusparseSolvePolicy_t {
    CUSPARSE_SOLVE_POLICY_NO_LEVEL = 0,
    CUSPARSE_SOLVE_POLICY_USE_LEVEL = 1,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cusparseColorAlg_t {
    CUSPARSE_COLOR_ALG0 = 0,
    CUSPARSE_COLOR_ALG1 = 1,
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCreate(handle: *mut cusparseHandle_t) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDestroy(handle: cusparseHandle_t) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseGetVersion(handle: cusparseHandle_t, version: *mut ::std::os::raw::c_int) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseGetProperty(type_: libraryPropertyType, value: *mut ::std::os::raw::c_int) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseGetErrorName(status: cusparseStatus_t) -> *const ::std::os::raw::c_char;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseGetErrorString(status: cusparseStatus_t) -> *const ::std::os::raw::c_char;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSetStream(handle: cusparseHandle_t, streamId: cudaStream_t) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseGetStream(handle: cusparseHandle_t, streamId: *mut cudaStream_t) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseGetPointerMode(handle: cusparseHandle_t, mode: *mut cusparsePointerMode_t) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSetPointerMode(handle: cusparseHandle_t, mode: cusparsePointerMode_t) -> cusparseStatus_t;
}
pub type cusparseLoggerCallback_t = ::std::option::Option<unsafe extern "C" fn(logLevel: ::std::os::raw::c_int, functionName: *const ::std::os::raw::c_char, message: *const ::std::os::raw::c_char)>;
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseLoggerSetCallback(callback: cusparseLoggerCallback_t) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseLoggerSetFile(file: *mut FILE) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseLoggerOpenFile(logFile: *const ::std::os::raw::c_char) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseLoggerSetLevel(level: ::std::os::raw::c_int) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseLoggerSetMask(mask: ::std::os::raw::c_int) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseLoggerForceDisable() -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCreateMatDescr(descrA: *mut cusparseMatDescr_t) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDestroyMatDescr(descrA: cusparseMatDescr_t) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSetMatType(descrA: cusparseMatDescr_t, type_: cusparseMatrixType_t) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseGetMatType(descrA: cusparseMatDescr_t) -> cusparseMatrixType_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSetMatFillMode(descrA: cusparseMatDescr_t, fillMode: cusparseFillMode_t) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseGetMatFillMode(descrA: cusparseMatDescr_t) -> cusparseFillMode_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSetMatDiagType(descrA: cusparseMatDescr_t, diagType: cusparseDiagType_t) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseGetMatDiagType(descrA: cusparseMatDescr_t) -> cusparseDiagType_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSetMatIndexBase(descrA: cusparseMatDescr_t, base: cusparseIndexBase_t) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseGetMatIndexBase(descrA: cusparseMatDescr_t) -> cusparseIndexBase_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCreateCsric02Info(info: *mut csric02Info_t) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDestroyCsric02Info(info: csric02Info_t) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCreateBsric02Info(info: *mut bsric02Info_t) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDestroyBsric02Info(info: bsric02Info_t) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCreateCsrilu02Info(info: *mut csrilu02Info_t) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDestroyCsrilu02Info(info: csrilu02Info_t) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCreateBsrilu02Info(info: *mut bsrilu02Info_t) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDestroyBsrilu02Info(info: bsrilu02Info_t) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCreateBsrsv2Info(info: *mut bsrsv2Info_t) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDestroyBsrsv2Info(info: bsrsv2Info_t) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCreateBsrsm2Info(info: *mut bsrsm2Info_t) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDestroyBsrsm2Info(info: bsrsm2Info_t) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCreateCsru2csrInfo(info: *mut csru2csrInfo_t) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDestroyCsru2csrInfo(info: csru2csrInfo_t) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCreateColorInfo(info: *mut cusparseColorInfo_t) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDestroyColorInfo(info: cusparseColorInfo_t) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCreatePruneInfo(info: *mut pruneInfo_t) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDestroyPruneInfo(info: pruneInfo_t) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSgemvi(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSgemvi_bufferSize(handle: cusparseHandle_t, transA: cusparseOperation_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, nnz: ::std::os::raw::c_int, pBufferSize: *mut ::std::os::raw::c_int) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDgemvi(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDgemvi_bufferSize(handle: cusparseHandle_t, transA: cusparseOperation_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, nnz: ::std::os::raw::c_int, pBufferSize: *mut ::std::os::raw::c_int) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCgemvi(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCgemvi_bufferSize(handle: cusparseHandle_t, transA: cusparseOperation_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, nnz: ::std::os::raw::c_int, pBufferSize: *mut ::std::os::raw::c_int) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseZgemvi(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseZgemvi_bufferSize(handle: cusparseHandle_t, transA: cusparseOperation_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, nnz: ::std::os::raw::c_int, pBufferSize: *mut ::std::os::raw::c_int) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSbsrmv(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDbsrmv(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCbsrmv(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseZbsrmv(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSbsrxmv(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDbsrxmv(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCbsrxmv(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseZbsrxmv(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseXbsrsv2_zeroPivot(handle: cusparseHandle_t, info: bsrsv2Info_t, position: *mut ::std::os::raw::c_int) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSbsrsv2_bufferSize(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDbsrsv2_bufferSize(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCbsrsv2_bufferSize(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseZbsrsv2_bufferSize(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSbsrsv2_bufferSizeExt(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDbsrsv2_bufferSizeExt(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCbsrsv2_bufferSizeExt(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseZbsrsv2_bufferSizeExt(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSbsrsv2_analysis(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDbsrsv2_analysis(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCbsrsv2_analysis(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseZbsrsv2_analysis(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSbsrsv2_solve(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDbsrsv2_solve(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCbsrsv2_solve(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseZbsrsv2_solve(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSbsrmm(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDbsrmm(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCbsrmm(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseZbsrmm(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseXbsrsm2_zeroPivot(handle: cusparseHandle_t, info: bsrsm2Info_t, position: *mut ::std::os::raw::c_int) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSbsrsm2_bufferSize(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDbsrsm2_bufferSize(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCbsrsm2_bufferSize(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseZbsrsm2_bufferSize(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSbsrsm2_bufferSizeExt(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDbsrsm2_bufferSizeExt(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCbsrsm2_bufferSizeExt(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseZbsrsm2_bufferSizeExt(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSbsrsm2_analysis(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDbsrsm2_analysis(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCbsrsm2_analysis(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseZbsrsm2_analysis(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSbsrsm2_solve(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDbsrsm2_solve(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCbsrsm2_solve(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseZbsrsm2_solve(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseScsrilu02_numericBoost(handle: cusparseHandle_t, info: csrilu02Info_t, enable_boost: ::std::os::raw::c_int, tol: *mut f64, boost_val: *mut f32) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDcsrilu02_numericBoost(handle: cusparseHandle_t, info: csrilu02Info_t, enable_boost: ::std::os::raw::c_int, tol: *mut f64, boost_val: *mut f64) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCcsrilu02_numericBoost(handle: cusparseHandle_t, info: csrilu02Info_t, enable_boost: ::std::os::raw::c_int, tol: *mut f64, boost_val: *mut cuComplex) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseZcsrilu02_numericBoost(handle: cusparseHandle_t, info: csrilu02Info_t, enable_boost: ::std::os::raw::c_int, tol: *mut f64, boost_val: *mut cuDoubleComplex) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseXcsrilu02_zeroPivot(handle: cusparseHandle_t, info: csrilu02Info_t, position: *mut ::std::os::raw::c_int) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseScsrilu02_bufferSize(
        handle: cusparseHandle_t,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrSortedValA: *mut f32,
        csrSortedRowPtrA: *const ::std::os::raw::c_int,
        csrSortedColIndA: *const ::std::os::raw::c_int,
        info: csrilu02Info_t,
        pBufferSizeInBytes: *mut ::std::os::raw::c_int,
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDcsrilu02_bufferSize(
        handle: cusparseHandle_t,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrSortedValA: *mut f64,
        csrSortedRowPtrA: *const ::std::os::raw::c_int,
        csrSortedColIndA: *const ::std::os::raw::c_int,
        info: csrilu02Info_t,
        pBufferSizeInBytes: *mut ::std::os::raw::c_int,
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCcsrilu02_bufferSize(
        handle: cusparseHandle_t,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrSortedValA: *mut cuComplex,
        csrSortedRowPtrA: *const ::std::os::raw::c_int,
        csrSortedColIndA: *const ::std::os::raw::c_int,
        info: csrilu02Info_t,
        pBufferSizeInBytes: *mut ::std::os::raw::c_int,
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseZcsrilu02_bufferSize(
        handle: cusparseHandle_t,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrSortedValA: *mut cuDoubleComplex,
        csrSortedRowPtrA: *const ::std::os::raw::c_int,
        csrSortedColIndA: *const ::std::os::raw::c_int,
        info: csrilu02Info_t,
        pBufferSizeInBytes: *mut ::std::os::raw::c_int,
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseScsrilu02_bufferSizeExt(
        handle: cusparseHandle_t,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrSortedVal: *mut f32,
        csrSortedRowPtr: *const ::std::os::raw::c_int,
        csrSortedColInd: *const ::std::os::raw::c_int,
        info: csrilu02Info_t,
        pBufferSize: *mut usize,
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDcsrilu02_bufferSizeExt(
        handle: cusparseHandle_t,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrSortedVal: *mut f64,
        csrSortedRowPtr: *const ::std::os::raw::c_int,
        csrSortedColInd: *const ::std::os::raw::c_int,
        info: csrilu02Info_t,
        pBufferSize: *mut usize,
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCcsrilu02_bufferSizeExt(
        handle: cusparseHandle_t,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrSortedVal: *mut cuComplex,
        csrSortedRowPtr: *const ::std::os::raw::c_int,
        csrSortedColInd: *const ::std::os::raw::c_int,
        info: csrilu02Info_t,
        pBufferSize: *mut usize,
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseZcsrilu02_bufferSizeExt(
        handle: cusparseHandle_t,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrSortedVal: *mut cuDoubleComplex,
        csrSortedRowPtr: *const ::std::os::raw::c_int,
        csrSortedColInd: *const ::std::os::raw::c_int,
        info: csrilu02Info_t,
        pBufferSize: *mut usize,
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseScsrilu02_analysis(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDcsrilu02_analysis(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCcsrilu02_analysis(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseZcsrilu02_analysis(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseScsrilu02(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDcsrilu02(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCcsrilu02(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseZcsrilu02(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSbsrilu02_numericBoost(handle: cusparseHandle_t, info: bsrilu02Info_t, enable_boost: ::std::os::raw::c_int, tol: *mut f64, boost_val: *mut f32) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDbsrilu02_numericBoost(handle: cusparseHandle_t, info: bsrilu02Info_t, enable_boost: ::std::os::raw::c_int, tol: *mut f64, boost_val: *mut f64) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCbsrilu02_numericBoost(handle: cusparseHandle_t, info: bsrilu02Info_t, enable_boost: ::std::os::raw::c_int, tol: *mut f64, boost_val: *mut cuComplex) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseZbsrilu02_numericBoost(handle: cusparseHandle_t, info: bsrilu02Info_t, enable_boost: ::std::os::raw::c_int, tol: *mut f64, boost_val: *mut cuDoubleComplex) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseXbsrilu02_zeroPivot(handle: cusparseHandle_t, info: bsrilu02Info_t, position: *mut ::std::os::raw::c_int) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSbsrilu02_bufferSize(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDbsrilu02_bufferSize(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCbsrilu02_bufferSize(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseZbsrilu02_bufferSize(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSbsrilu02_bufferSizeExt(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDbsrilu02_bufferSizeExt(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCbsrilu02_bufferSizeExt(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseZbsrilu02_bufferSizeExt(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSbsrilu02_analysis(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDbsrilu02_analysis(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCbsrilu02_analysis(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseZbsrilu02_analysis(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSbsrilu02(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDbsrilu02(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCbsrilu02(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseZbsrilu02(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseXcsric02_zeroPivot(handle: cusparseHandle_t, info: csric02Info_t, position: *mut ::std::os::raw::c_int) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseScsric02_bufferSize(
        handle: cusparseHandle_t,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrSortedValA: *mut f32,
        csrSortedRowPtrA: *const ::std::os::raw::c_int,
        csrSortedColIndA: *const ::std::os::raw::c_int,
        info: csric02Info_t,
        pBufferSizeInBytes: *mut ::std::os::raw::c_int,
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDcsric02_bufferSize(
        handle: cusparseHandle_t,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrSortedValA: *mut f64,
        csrSortedRowPtrA: *const ::std::os::raw::c_int,
        csrSortedColIndA: *const ::std::os::raw::c_int,
        info: csric02Info_t,
        pBufferSizeInBytes: *mut ::std::os::raw::c_int,
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCcsric02_bufferSize(
        handle: cusparseHandle_t,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrSortedValA: *mut cuComplex,
        csrSortedRowPtrA: *const ::std::os::raw::c_int,
        csrSortedColIndA: *const ::std::os::raw::c_int,
        info: csric02Info_t,
        pBufferSizeInBytes: *mut ::std::os::raw::c_int,
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseZcsric02_bufferSize(
        handle: cusparseHandle_t,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrSortedValA: *mut cuDoubleComplex,
        csrSortedRowPtrA: *const ::std::os::raw::c_int,
        csrSortedColIndA: *const ::std::os::raw::c_int,
        info: csric02Info_t,
        pBufferSizeInBytes: *mut ::std::os::raw::c_int,
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseScsric02_bufferSizeExt(
        handle: cusparseHandle_t,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrSortedVal: *mut f32,
        csrSortedRowPtr: *const ::std::os::raw::c_int,
        csrSortedColInd: *const ::std::os::raw::c_int,
        info: csric02Info_t,
        pBufferSize: *mut usize,
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDcsric02_bufferSizeExt(
        handle: cusparseHandle_t,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrSortedVal: *mut f64,
        csrSortedRowPtr: *const ::std::os::raw::c_int,
        csrSortedColInd: *const ::std::os::raw::c_int,
        info: csric02Info_t,
        pBufferSize: *mut usize,
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCcsric02_bufferSizeExt(
        handle: cusparseHandle_t,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrSortedVal: *mut cuComplex,
        csrSortedRowPtr: *const ::std::os::raw::c_int,
        csrSortedColInd: *const ::std::os::raw::c_int,
        info: csric02Info_t,
        pBufferSize: *mut usize,
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseZcsric02_bufferSizeExt(
        handle: cusparseHandle_t,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrSortedVal: *mut cuDoubleComplex,
        csrSortedRowPtr: *const ::std::os::raw::c_int,
        csrSortedColInd: *const ::std::os::raw::c_int,
        info: csric02Info_t,
        pBufferSize: *mut usize,
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseScsric02_analysis(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDcsric02_analysis(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCcsric02_analysis(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseZcsric02_analysis(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseScsric02(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDcsric02(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCcsric02(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseZcsric02(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseXbsric02_zeroPivot(handle: cusparseHandle_t, info: bsric02Info_t, position: *mut ::std::os::raw::c_int) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSbsric02_bufferSize(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDbsric02_bufferSize(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCbsric02_bufferSize(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseZbsric02_bufferSize(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSbsric02_bufferSizeExt(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDbsric02_bufferSizeExt(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCbsric02_bufferSizeExt(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseZbsric02_bufferSizeExt(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSbsric02_analysis(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDbsric02_analysis(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCbsric02_analysis(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseZbsric02_analysis(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSbsric02(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDbsric02(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCbsric02(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseZbsric02(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSgtsv2_bufferSizeExt(handle: cusparseHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, dl: *const f32, d: *const f32, du: *const f32, B: *const f32, ldb: ::std::os::raw::c_int, bufferSizeInBytes: *mut usize) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDgtsv2_bufferSizeExt(handle: cusparseHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, dl: *const f64, d: *const f64, du: *const f64, B: *const f64, ldb: ::std::os::raw::c_int, bufferSizeInBytes: *mut usize) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCgtsv2_bufferSizeExt(handle: cusparseHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, dl: *const cuComplex, d: *const cuComplex, du: *const cuComplex, B: *const cuComplex, ldb: ::std::os::raw::c_int, bufferSizeInBytes: *mut usize) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseZgtsv2_bufferSizeExt(
        handle: cusparseHandle_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        dl: *const cuDoubleComplex,
        d: *const cuDoubleComplex,
        du: *const cuDoubleComplex,
        B: *const cuDoubleComplex,
        ldb: ::std::os::raw::c_int,
        bufferSizeInBytes: *mut usize,
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSgtsv2(handle: cusparseHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, dl: *const f32, d: *const f32, du: *const f32, B: *mut f32, ldb: ::std::os::raw::c_int, pBuffer: *mut ::std::os::raw::c_void) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDgtsv2(handle: cusparseHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, dl: *const f64, d: *const f64, du: *const f64, B: *mut f64, ldb: ::std::os::raw::c_int, pBuffer: *mut ::std::os::raw::c_void) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCgtsv2(handle: cusparseHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, dl: *const cuComplex, d: *const cuComplex, du: *const cuComplex, B: *mut cuComplex, ldb: ::std::os::raw::c_int, pBuffer: *mut ::std::os::raw::c_void) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseZgtsv2(handle: cusparseHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, dl: *const cuDoubleComplex, d: *const cuDoubleComplex, du: *const cuDoubleComplex, B: *mut cuDoubleComplex, ldb: ::std::os::raw::c_int, pBuffer: *mut ::std::os::raw::c_void) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSgtsv2_nopivot_bufferSizeExt(handle: cusparseHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, dl: *const f32, d: *const f32, du: *const f32, B: *const f32, ldb: ::std::os::raw::c_int, bufferSizeInBytes: *mut usize) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDgtsv2_nopivot_bufferSizeExt(handle: cusparseHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, dl: *const f64, d: *const f64, du: *const f64, B: *const f64, ldb: ::std::os::raw::c_int, bufferSizeInBytes: *mut usize) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCgtsv2_nopivot_bufferSizeExt(handle: cusparseHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, dl: *const cuComplex, d: *const cuComplex, du: *const cuComplex, B: *const cuComplex, ldb: ::std::os::raw::c_int, bufferSizeInBytes: *mut usize) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseZgtsv2_nopivot_bufferSizeExt(
        handle: cusparseHandle_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        dl: *const cuDoubleComplex,
        d: *const cuDoubleComplex,
        du: *const cuDoubleComplex,
        B: *const cuDoubleComplex,
        ldb: ::std::os::raw::c_int,
        bufferSizeInBytes: *mut usize,
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSgtsv2_nopivot(handle: cusparseHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, dl: *const f32, d: *const f32, du: *const f32, B: *mut f32, ldb: ::std::os::raw::c_int, pBuffer: *mut ::std::os::raw::c_void) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDgtsv2_nopivot(handle: cusparseHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, dl: *const f64, d: *const f64, du: *const f64, B: *mut f64, ldb: ::std::os::raw::c_int, pBuffer: *mut ::std::os::raw::c_void) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCgtsv2_nopivot(handle: cusparseHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, dl: *const cuComplex, d: *const cuComplex, du: *const cuComplex, B: *mut cuComplex, ldb: ::std::os::raw::c_int, pBuffer: *mut ::std::os::raw::c_void) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseZgtsv2_nopivot(
        handle: cusparseHandle_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        dl: *const cuDoubleComplex,
        d: *const cuDoubleComplex,
        du: *const cuDoubleComplex,
        B: *mut cuDoubleComplex,
        ldb: ::std::os::raw::c_int,
        pBuffer: *mut ::std::os::raw::c_void,
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSgtsv2StridedBatch_bufferSizeExt(handle: cusparseHandle_t, m: ::std::os::raw::c_int, dl: *const f32, d: *const f32, du: *const f32, x: *const f32, batchCount: ::std::os::raw::c_int, batchStride: ::std::os::raw::c_int, bufferSizeInBytes: *mut usize) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDgtsv2StridedBatch_bufferSizeExt(handle: cusparseHandle_t, m: ::std::os::raw::c_int, dl: *const f64, d: *const f64, du: *const f64, x: *const f64, batchCount: ::std::os::raw::c_int, batchStride: ::std::os::raw::c_int, bufferSizeInBytes: *mut usize) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCgtsv2StridedBatch_bufferSizeExt(
        handle: cusparseHandle_t,
        m: ::std::os::raw::c_int,
        dl: *const cuComplex,
        d: *const cuComplex,
        du: *const cuComplex,
        x: *const cuComplex,
        batchCount: ::std::os::raw::c_int,
        batchStride: ::std::os::raw::c_int,
        bufferSizeInBytes: *mut usize,
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseZgtsv2StridedBatch_bufferSizeExt(
        handle: cusparseHandle_t,
        m: ::std::os::raw::c_int,
        dl: *const cuDoubleComplex,
        d: *const cuDoubleComplex,
        du: *const cuDoubleComplex,
        x: *const cuDoubleComplex,
        batchCount: ::std::os::raw::c_int,
        batchStride: ::std::os::raw::c_int,
        bufferSizeInBytes: *mut usize,
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSgtsv2StridedBatch(handle: cusparseHandle_t, m: ::std::os::raw::c_int, dl: *const f32, d: *const f32, du: *const f32, x: *mut f32, batchCount: ::std::os::raw::c_int, batchStride: ::std::os::raw::c_int, pBuffer: *mut ::std::os::raw::c_void) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDgtsv2StridedBatch(handle: cusparseHandle_t, m: ::std::os::raw::c_int, dl: *const f64, d: *const f64, du: *const f64, x: *mut f64, batchCount: ::std::os::raw::c_int, batchStride: ::std::os::raw::c_int, pBuffer: *mut ::std::os::raw::c_void) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCgtsv2StridedBatch(
        handle: cusparseHandle_t,
        m: ::std::os::raw::c_int,
        dl: *const cuComplex,
        d: *const cuComplex,
        du: *const cuComplex,
        x: *mut cuComplex,
        batchCount: ::std::os::raw::c_int,
        batchStride: ::std::os::raw::c_int,
        pBuffer: *mut ::std::os::raw::c_void,
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseZgtsv2StridedBatch(
        handle: cusparseHandle_t,
        m: ::std::os::raw::c_int,
        dl: *const cuDoubleComplex,
        d: *const cuDoubleComplex,
        du: *const cuDoubleComplex,
        x: *mut cuDoubleComplex,
        batchCount: ::std::os::raw::c_int,
        batchStride: ::std::os::raw::c_int,
        pBuffer: *mut ::std::os::raw::c_void,
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSgtsvInterleavedBatch_bufferSizeExt(handle: cusparseHandle_t, algo: ::std::os::raw::c_int, m: ::std::os::raw::c_int, dl: *const f32, d: *const f32, du: *const f32, x: *const f32, batchCount: ::std::os::raw::c_int, pBufferSizeInBytes: *mut usize) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDgtsvInterleavedBatch_bufferSizeExt(handle: cusparseHandle_t, algo: ::std::os::raw::c_int, m: ::std::os::raw::c_int, dl: *const f64, d: *const f64, du: *const f64, x: *const f64, batchCount: ::std::os::raw::c_int, pBufferSizeInBytes: *mut usize) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCgtsvInterleavedBatch_bufferSizeExt(
        handle: cusparseHandle_t,
        algo: ::std::os::raw::c_int,
        m: ::std::os::raw::c_int,
        dl: *const cuComplex,
        d: *const cuComplex,
        du: *const cuComplex,
        x: *const cuComplex,
        batchCount: ::std::os::raw::c_int,
        pBufferSizeInBytes: *mut usize,
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseZgtsvInterleavedBatch_bufferSizeExt(
        handle: cusparseHandle_t,
        algo: ::std::os::raw::c_int,
        m: ::std::os::raw::c_int,
        dl: *const cuDoubleComplex,
        d: *const cuDoubleComplex,
        du: *const cuDoubleComplex,
        x: *const cuDoubleComplex,
        batchCount: ::std::os::raw::c_int,
        pBufferSizeInBytes: *mut usize,
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSgtsvInterleavedBatch(handle: cusparseHandle_t, algo: ::std::os::raw::c_int, m: ::std::os::raw::c_int, dl: *mut f32, d: *mut f32, du: *mut f32, x: *mut f32, batchCount: ::std::os::raw::c_int, pBuffer: *mut ::std::os::raw::c_void) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDgtsvInterleavedBatch(handle: cusparseHandle_t, algo: ::std::os::raw::c_int, m: ::std::os::raw::c_int, dl: *mut f64, d: *mut f64, du: *mut f64, x: *mut f64, batchCount: ::std::os::raw::c_int, pBuffer: *mut ::std::os::raw::c_void) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCgtsvInterleavedBatch(handle: cusparseHandle_t, algo: ::std::os::raw::c_int, m: ::std::os::raw::c_int, dl: *mut cuComplex, d: *mut cuComplex, du: *mut cuComplex, x: *mut cuComplex, batchCount: ::std::os::raw::c_int, pBuffer: *mut ::std::os::raw::c_void) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseZgtsvInterleavedBatch(
        handle: cusparseHandle_t,
        algo: ::std::os::raw::c_int,
        m: ::std::os::raw::c_int,
        dl: *mut cuDoubleComplex,
        d: *mut cuDoubleComplex,
        du: *mut cuDoubleComplex,
        x: *mut cuDoubleComplex,
        batchCount: ::std::os::raw::c_int,
        pBuffer: *mut ::std::os::raw::c_void,
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSgpsvInterleavedBatch_bufferSizeExt(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDgpsvInterleavedBatch_bufferSizeExt(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCgpsvInterleavedBatch_bufferSizeExt(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseZgpsvInterleavedBatch_bufferSizeExt(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSgpsvInterleavedBatch(handle: cusparseHandle_t, algo: ::std::os::raw::c_int, m: ::std::os::raw::c_int, ds: *mut f32, dl: *mut f32, d: *mut f32, du: *mut f32, dw: *mut f32, x: *mut f32, batchCount: ::std::os::raw::c_int, pBuffer: *mut ::std::os::raw::c_void) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDgpsvInterleavedBatch(handle: cusparseHandle_t, algo: ::std::os::raw::c_int, m: ::std::os::raw::c_int, ds: *mut f64, dl: *mut f64, d: *mut f64, du: *mut f64, dw: *mut f64, x: *mut f64, batchCount: ::std::os::raw::c_int, pBuffer: *mut ::std::os::raw::c_void) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCgpsvInterleavedBatch(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseZgpsvInterleavedBatch(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseScsrgeam2_bufferSizeExt(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDcsrgeam2_bufferSizeExt(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCcsrgeam2_bufferSizeExt(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseZcsrgeam2_bufferSizeExt(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseXcsrgeam2Nnz(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseScsrgeam2(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDcsrgeam2(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCcsrgeam2(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseZcsrgeam2(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseScsrcolor(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDcsrcolor(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCcsrcolor(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseZcsrcolor(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSnnz(
        handle: cusparseHandle_t,
        dirA: cusparseDirection_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        A: *const f32,
        lda: ::std::os::raw::c_int,
        nnzPerRowCol: *mut ::std::os::raw::c_int,
        nnzTotalDevHostPtr: *mut ::std::os::raw::c_int,
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDnnz(
        handle: cusparseHandle_t,
        dirA: cusparseDirection_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        A: *const f64,
        lda: ::std::os::raw::c_int,
        nnzPerRowCol: *mut ::std::os::raw::c_int,
        nnzTotalDevHostPtr: *mut ::std::os::raw::c_int,
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCnnz(
        handle: cusparseHandle_t,
        dirA: cusparseDirection_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        A: *const cuComplex,
        lda: ::std::os::raw::c_int,
        nnzPerRowCol: *mut ::std::os::raw::c_int,
        nnzTotalDevHostPtr: *mut ::std::os::raw::c_int,
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseZnnz(
        handle: cusparseHandle_t,
        dirA: cusparseDirection_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        A: *const cuDoubleComplex,
        lda: ::std::os::raw::c_int,
        nnzPerRowCol: *mut ::std::os::raw::c_int,
        nnzTotalDevHostPtr: *mut ::std::os::raw::c_int,
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSnnz_compress(handle: cusparseHandle_t, m: ::std::os::raw::c_int, descr: cusparseMatDescr_t, csrSortedValA: *const f32, csrSortedRowPtrA: *const ::std::os::raw::c_int, nnzPerRow: *mut ::std::os::raw::c_int, nnzC: *mut ::std::os::raw::c_int, tol: f32) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDnnz_compress(handle: cusparseHandle_t, m: ::std::os::raw::c_int, descr: cusparseMatDescr_t, csrSortedValA: *const f64, csrSortedRowPtrA: *const ::std::os::raw::c_int, nnzPerRow: *mut ::std::os::raw::c_int, nnzC: *mut ::std::os::raw::c_int, tol: f64) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCnnz_compress(
        handle: cusparseHandle_t,
        m: ::std::os::raw::c_int,
        descr: cusparseMatDescr_t,
        csrSortedValA: *const cuComplex,
        csrSortedRowPtrA: *const ::std::os::raw::c_int,
        nnzPerRow: *mut ::std::os::raw::c_int,
        nnzC: *mut ::std::os::raw::c_int,
        tol: cuComplex,
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseZnnz_compress(
        handle: cusparseHandle_t,
        m: ::std::os::raw::c_int,
        descr: cusparseMatDescr_t,
        csrSortedValA: *const cuDoubleComplex,
        csrSortedRowPtrA: *const ::std::os::raw::c_int,
        nnzPerRow: *mut ::std::os::raw::c_int,
        nnzC: *mut ::std::os::raw::c_int,
        tol: cuDoubleComplex,
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseScsr2csr_compress(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDcsr2csr_compress(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCcsr2csr_compress(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseZcsr2csr_compress(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseXcoo2csr(handle: cusparseHandle_t, cooRowInd: *const ::std::os::raw::c_int, nnz: ::std::os::raw::c_int, m: ::std::os::raw::c_int, csrSortedRowPtr: *mut ::std::os::raw::c_int, idxBase: cusparseIndexBase_t) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseXcsr2coo(handle: cusparseHandle_t, csrSortedRowPtr: *const ::std::os::raw::c_int, nnz: ::std::os::raw::c_int, m: ::std::os::raw::c_int, cooRowInd: *mut ::std::os::raw::c_int, idxBase: cusparseIndexBase_t) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseXcsr2bsrNnz(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseScsr2bsr(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDcsr2bsr(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCcsr2bsr(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseZcsr2bsr(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSbsr2csr(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDbsr2csr(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCbsr2csr(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseZbsr2csr(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSgebsr2gebsc_bufferSize(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDgebsr2gebsc_bufferSize(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCgebsr2gebsc_bufferSize(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseZgebsr2gebsc_bufferSize(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSgebsr2gebsc_bufferSizeExt(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDgebsr2gebsc_bufferSizeExt(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCgebsr2gebsc_bufferSizeExt(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseZgebsr2gebsc_bufferSizeExt(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSgebsr2gebsc(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDgebsr2gebsc(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCgebsr2gebsc(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseZgebsr2gebsc(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseXgebsr2csr(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSgebsr2csr(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDgebsr2csr(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCgebsr2csr(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseZgebsr2csr(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseScsr2gebsr_bufferSize(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDcsr2gebsr_bufferSize(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCcsr2gebsr_bufferSize(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseZcsr2gebsr_bufferSize(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseScsr2gebsr_bufferSizeExt(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDcsr2gebsr_bufferSizeExt(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCcsr2gebsr_bufferSizeExt(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseZcsr2gebsr_bufferSizeExt(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseXcsr2gebsrNnz(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseScsr2gebsr(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDcsr2gebsr(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCcsr2gebsr(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseZcsr2gebsr(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSgebsr2gebsr_bufferSize(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDgebsr2gebsr_bufferSize(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCgebsr2gebsr_bufferSize(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseZgebsr2gebsr_bufferSize(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSgebsr2gebsr_bufferSizeExt(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDgebsr2gebsr_bufferSizeExt(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCgebsr2gebsr_bufferSizeExt(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseZgebsr2gebsr_bufferSizeExt(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseXgebsr2gebsrNnz(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSgebsr2gebsr(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDgebsr2gebsr(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCgebsr2gebsr(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseZgebsr2gebsr(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCreateIdentityPermutation(handle: cusparseHandle_t, n: ::std::os::raw::c_int, p: *mut ::std::os::raw::c_int) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseXcoosort_bufferSizeExt(handle: cusparseHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, nnz: ::std::os::raw::c_int, cooRowsA: *const ::std::os::raw::c_int, cooColsA: *const ::std::os::raw::c_int, pBufferSizeInBytes: *mut usize) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseXcoosortByRow(
        handle: cusparseHandle_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        cooRowsA: *mut ::std::os::raw::c_int,
        cooColsA: *mut ::std::os::raw::c_int,
        P: *mut ::std::os::raw::c_int,
        pBuffer: *mut ::std::os::raw::c_void,
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseXcoosortByColumn(
        handle: cusparseHandle_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        cooRowsA: *mut ::std::os::raw::c_int,
        cooColsA: *mut ::std::os::raw::c_int,
        P: *mut ::std::os::raw::c_int,
        pBuffer: *mut ::std::os::raw::c_void,
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseXcsrsort_bufferSizeExt(handle: cusparseHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, nnz: ::std::os::raw::c_int, csrRowPtrA: *const ::std::os::raw::c_int, csrColIndA: *const ::std::os::raw::c_int, pBufferSizeInBytes: *mut usize) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseXcsrsort(
        handle: cusparseHandle_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrRowPtrA: *const ::std::os::raw::c_int,
        csrColIndA: *mut ::std::os::raw::c_int,
        P: *mut ::std::os::raw::c_int,
        pBuffer: *mut ::std::os::raw::c_void,
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseXcscsort_bufferSizeExt(handle: cusparseHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, nnz: ::std::os::raw::c_int, cscColPtrA: *const ::std::os::raw::c_int, cscRowIndA: *const ::std::os::raw::c_int, pBufferSizeInBytes: *mut usize) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseXcscsort(
        handle: cusparseHandle_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        cscColPtrA: *const ::std::os::raw::c_int,
        cscRowIndA: *mut ::std::os::raw::c_int,
        P: *mut ::std::os::raw::c_int,
        pBuffer: *mut ::std::os::raw::c_void,
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseScsru2csr_bufferSizeExt(
        handle: cusparseHandle_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        csrVal: *mut f32,
        csrRowPtr: *const ::std::os::raw::c_int,
        csrColInd: *mut ::std::os::raw::c_int,
        info: csru2csrInfo_t,
        pBufferSizeInBytes: *mut usize,
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDcsru2csr_bufferSizeExt(
        handle: cusparseHandle_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        csrVal: *mut f64,
        csrRowPtr: *const ::std::os::raw::c_int,
        csrColInd: *mut ::std::os::raw::c_int,
        info: csru2csrInfo_t,
        pBufferSizeInBytes: *mut usize,
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCcsru2csr_bufferSizeExt(
        handle: cusparseHandle_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        csrVal: *mut cuComplex,
        csrRowPtr: *const ::std::os::raw::c_int,
        csrColInd: *mut ::std::os::raw::c_int,
        info: csru2csrInfo_t,
        pBufferSizeInBytes: *mut usize,
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseZcsru2csr_bufferSizeExt(
        handle: cusparseHandle_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        csrVal: *mut cuDoubleComplex,
        csrRowPtr: *const ::std::os::raw::c_int,
        csrColInd: *mut ::std::os::raw::c_int,
        info: csru2csrInfo_t,
        pBufferSizeInBytes: *mut usize,
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseScsru2csr(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDcsru2csr(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCcsru2csr(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseZcsru2csr(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseScsr2csru(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDcsr2csru(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCcsr2csru(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseZcsr2csru(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSpruneDense2csr_bufferSizeExt(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDpruneDense2csr_bufferSizeExt(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSpruneDense2csrNnz(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDpruneDense2csrNnz(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSpruneDense2csr(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDpruneDense2csr(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSpruneCsr2csr_bufferSizeExt(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDpruneCsr2csr_bufferSizeExt(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSpruneCsr2csrNnz(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDpruneCsr2csrNnz(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSpruneCsr2csr(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDpruneCsr2csr(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSpruneDense2csrByPercentage_bufferSizeExt(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDpruneDense2csrByPercentage_bufferSizeExt(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSpruneDense2csrNnzByPercentage(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDpruneDense2csrNnzByPercentage(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSpruneDense2csrByPercentage(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDpruneDense2csrByPercentage(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSpruneCsr2csrByPercentage_bufferSizeExt(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDpruneCsr2csrByPercentage_bufferSizeExt(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSpruneCsr2csrNnzByPercentage(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDpruneCsr2csrNnzByPercentage(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSpruneCsr2csrByPercentage(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDpruneCsr2csrByPercentage(
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
    ) -> cusparseStatus_t;
}
impl cusparseCsr2CscAlg_t {
    pub const CUSPARSE_CSR2CSC_ALG1: cusparseCsr2CscAlg_t = cusparseCsr2CscAlg_t::CUSPARSE_CSR2CSC_ALG_DEFAULT;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cusparseCsr2CscAlg_t {
    CUSPARSE_CSR2CSC_ALG_DEFAULT = 1,
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCsr2cscEx2(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCsr2cscEx2_bufferSize(
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
    ) -> cusparseStatus_t;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cusparseFormat_t {
    #[doc = "< Compressed Sparse Row (CSR)"]
    CUSPARSE_FORMAT_CSR = 1,
    #[doc = "< Compressed Sparse Column (CSC)"]
    CUSPARSE_FORMAT_CSC = 2,
    #[doc = "< Coordinate (COO) - Structure of Arrays"]
    CUSPARSE_FORMAT_COO = 3,
    #[doc = "< Blocked ELL"]
    CUSPARSE_FORMAT_BLOCKED_ELL = 5,
    #[doc = "< Blocked Compressed Sparse Row (BSR)"]
    CUSPARSE_FORMAT_BSR = 6,
    #[doc = "< Sliced ELL"]
    CUSPARSE_FORMAT_SLICED_ELLPACK = 7,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cusparseOrder_t {
    #[doc = "< Column-Major Order - Matrix memory layout"]
    CUSPARSE_ORDER_COL = 1,
    #[doc = "< Row-Major Order - Matrix memory layout"]
    CUSPARSE_ORDER_ROW = 2,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cusparseIndexType_t {
    #[doc = "< 16-bit unsigned integer for matrix/vector\n< indices"]
    CUSPARSE_INDEX_16U = 1,
    #[doc = "< 32-bit signed integer for matrix/vector indices"]
    CUSPARSE_INDEX_32I = 2,
    #[doc = "< 64-bit signed integer for matrix/vector indices"]
    CUSPARSE_INDEX_64I = 3,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cusparseSpVecDescr {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cusparseDnVecDescr {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cusparseSpMatDescr {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cusparseDnMatDescr {
    _unused: [u8; 0],
}
pub type cusparseSpVecDescr_t = *mut cusparseSpVecDescr;
pub type cusparseDnVecDescr_t = *mut cusparseDnVecDescr;
pub type cusparseSpMatDescr_t = *mut cusparseSpMatDescr;
pub type cusparseDnMatDescr_t = *mut cusparseDnMatDescr;
pub type cusparseConstSpVecDescr_t = *const cusparseSpVecDescr;
pub type cusparseConstDnVecDescr_t = *const cusparseDnVecDescr;
pub type cusparseConstSpMatDescr_t = *const cusparseSpMatDescr;
pub type cusparseConstDnMatDescr_t = *const cusparseDnMatDescr;
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCreateSpVec(spVecDescr: *mut cusparseSpVecDescr_t, size: i64, nnz: i64, indices: *mut ::std::os::raw::c_void, values: *mut ::std::os::raw::c_void, idxType: cusparseIndexType_t, idxBase: cusparseIndexBase_t, valueType: cudaDataType) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCreateConstSpVec(spVecDescr: *mut cusparseConstSpVecDescr_t, size: i64, nnz: i64, indices: *const ::std::os::raw::c_void, values: *const ::std::os::raw::c_void, idxType: cusparseIndexType_t, idxBase: cusparseIndexBase_t, valueType: cudaDataType) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDestroySpVec(spVecDescr: cusparseConstSpVecDescr_t) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSpVecGet(spVecDescr: cusparseSpVecDescr_t, size: *mut i64, nnz: *mut i64, indices: *mut *mut ::std::os::raw::c_void, values: *mut *mut ::std::os::raw::c_void, idxType: *mut cusparseIndexType_t, idxBase: *mut cusparseIndexBase_t, valueType: *mut cudaDataType) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseConstSpVecGet(
        spVecDescr: cusparseConstSpVecDescr_t,
        size: *mut i64,
        nnz: *mut i64,
        indices: *mut *const ::std::os::raw::c_void,
        values: *mut *const ::std::os::raw::c_void,
        idxType: *mut cusparseIndexType_t,
        idxBase: *mut cusparseIndexBase_t,
        valueType: *mut cudaDataType,
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSpVecGetIndexBase(spVecDescr: cusparseConstSpVecDescr_t, idxBase: *mut cusparseIndexBase_t) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSpVecGetValues(spVecDescr: cusparseSpVecDescr_t, values: *mut *mut ::std::os::raw::c_void) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseConstSpVecGetValues(spVecDescr: cusparseConstSpVecDescr_t, values: *mut *const ::std::os::raw::c_void) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSpVecSetValues(spVecDescr: cusparseSpVecDescr_t, values: *mut ::std::os::raw::c_void) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCreateDnVec(dnVecDescr: *mut cusparseDnVecDescr_t, size: i64, values: *mut ::std::os::raw::c_void, valueType: cudaDataType) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCreateConstDnVec(dnVecDescr: *mut cusparseConstDnVecDescr_t, size: i64, values: *const ::std::os::raw::c_void, valueType: cudaDataType) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDestroyDnVec(dnVecDescr: cusparseConstDnVecDescr_t) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDnVecGet(dnVecDescr: cusparseDnVecDescr_t, size: *mut i64, values: *mut *mut ::std::os::raw::c_void, valueType: *mut cudaDataType) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseConstDnVecGet(dnVecDescr: cusparseConstDnVecDescr_t, size: *mut i64, values: *mut *const ::std::os::raw::c_void, valueType: *mut cudaDataType) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDnVecGetValues(dnVecDescr: cusparseDnVecDescr_t, values: *mut *mut ::std::os::raw::c_void) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseConstDnVecGetValues(dnVecDescr: cusparseConstDnVecDescr_t, values: *mut *const ::std::os::raw::c_void) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDnVecSetValues(dnVecDescr: cusparseDnVecDescr_t, values: *mut ::std::os::raw::c_void) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDestroySpMat(spMatDescr: cusparseConstSpMatDescr_t) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSpMatGetFormat(spMatDescr: cusparseConstSpMatDescr_t, format: *mut cusparseFormat_t) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSpMatGetIndexBase(spMatDescr: cusparseConstSpMatDescr_t, idxBase: *mut cusparseIndexBase_t) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSpMatGetValues(spMatDescr: cusparseSpMatDescr_t, values: *mut *mut ::std::os::raw::c_void) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseConstSpMatGetValues(spMatDescr: cusparseConstSpMatDescr_t, values: *mut *const ::std::os::raw::c_void) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSpMatSetValues(spMatDescr: cusparseSpMatDescr_t, values: *mut ::std::os::raw::c_void) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSpMatGetSize(spMatDescr: cusparseConstSpMatDescr_t, rows: *mut i64, cols: *mut i64, nnz: *mut i64) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSpMatGetStridedBatch(spMatDescr: cusparseConstSpMatDescr_t, batchCount: *mut ::std::os::raw::c_int) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCooSetStridedBatch(spMatDescr: cusparseSpMatDescr_t, batchCount: ::std::os::raw::c_int, batchStride: i64) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCsrSetStridedBatch(spMatDescr: cusparseSpMatDescr_t, batchCount: ::std::os::raw::c_int, offsetsBatchStride: i64, columnsValuesBatchStride: i64) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseBsrSetStridedBatch(spMatDescr: cusparseSpMatDescr_t, batchCount: ::std::os::raw::c_int, offsetsBatchStride: i64, columnsBatchStride: i64, ValuesBatchStride: i64) -> cusparseStatus_t;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cusparseSpMatAttribute_t {
    CUSPARSE_SPMAT_FILL_MODE = 0,
    CUSPARSE_SPMAT_DIAG_TYPE = 1,
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSpMatGetAttribute(spMatDescr: cusparseConstSpMatDescr_t, attribute: cusparseSpMatAttribute_t, data: *mut ::std::os::raw::c_void, dataSize: usize) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSpMatSetAttribute(spMatDescr: cusparseSpMatDescr_t, attribute: cusparseSpMatAttribute_t, data: *mut ::std::os::raw::c_void, dataSize: usize) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCreateCsr(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCreateConstCsr(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCreateCsc(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCreateConstCsc(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCsrGet(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseConstCsrGet(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCscGet(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseConstCscGet(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCsrSetPointers(spMatDescr: cusparseSpMatDescr_t, csrRowOffsets: *mut ::std::os::raw::c_void, csrColInd: *mut ::std::os::raw::c_void, csrValues: *mut ::std::os::raw::c_void) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCscSetPointers(spMatDescr: cusparseSpMatDescr_t, cscColOffsets: *mut ::std::os::raw::c_void, cscRowInd: *mut ::std::os::raw::c_void, cscValues: *mut ::std::os::raw::c_void) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCreateBsr(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCreateConstBsr(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCreateCoo(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCreateConstCoo(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCooGet(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseConstCooGet(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCooSetPointers(spMatDescr: cusparseSpMatDescr_t, cooRows: *mut ::std::os::raw::c_void, cooColumns: *mut ::std::os::raw::c_void, cooValues: *mut ::std::os::raw::c_void) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCreateBlockedEll(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCreateConstBlockedEll(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseBlockedEllGet(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseConstBlockedEllGet(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCreateSlicedEll(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCreateConstSlicedEll(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCreateDnMat(dnMatDescr: *mut cusparseDnMatDescr_t, rows: i64, cols: i64, ld: i64, values: *mut ::std::os::raw::c_void, valueType: cudaDataType, order: cusparseOrder_t) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseCreateConstDnMat(dnMatDescr: *mut cusparseConstDnMatDescr_t, rows: i64, cols: i64, ld: i64, values: *const ::std::os::raw::c_void, valueType: cudaDataType, order: cusparseOrder_t) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDestroyDnMat(dnMatDescr: cusparseConstDnMatDescr_t) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDnMatGet(dnMatDescr: cusparseDnMatDescr_t, rows: *mut i64, cols: *mut i64, ld: *mut i64, values: *mut *mut ::std::os::raw::c_void, type_: *mut cudaDataType, order: *mut cusparseOrder_t) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseConstDnMatGet(dnMatDescr: cusparseConstDnMatDescr_t, rows: *mut i64, cols: *mut i64, ld: *mut i64, values: *mut *const ::std::os::raw::c_void, type_: *mut cudaDataType, order: *mut cusparseOrder_t) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDnMatGetValues(dnMatDescr: cusparseDnMatDescr_t, values: *mut *mut ::std::os::raw::c_void) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseConstDnMatGetValues(dnMatDescr: cusparseConstDnMatDescr_t, values: *mut *const ::std::os::raw::c_void) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDnMatSetValues(dnMatDescr: cusparseDnMatDescr_t, values: *mut ::std::os::raw::c_void) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDnMatSetStridedBatch(dnMatDescr: cusparseDnMatDescr_t, batchCount: ::std::os::raw::c_int, batchStride: i64) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDnMatGetStridedBatch(dnMatDescr: cusparseConstDnMatDescr_t, batchCount: *mut ::std::os::raw::c_int, batchStride: *mut i64) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseAxpby(handle: cusparseHandle_t, alpha: *const ::std::os::raw::c_void, vecX: cusparseConstSpVecDescr_t, beta: *const ::std::os::raw::c_void, vecY: cusparseDnVecDescr_t) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseGather(handle: cusparseHandle_t, vecY: cusparseConstDnVecDescr_t, vecX: cusparseSpVecDescr_t) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseScatter(handle: cusparseHandle_t, vecX: cusparseConstSpVecDescr_t, vecY: cusparseDnVecDescr_t) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseRot(handle: cusparseHandle_t, c_coeff: *const ::std::os::raw::c_void, s_coeff: *const ::std::os::raw::c_void, vecX: cusparseSpVecDescr_t, vecY: cusparseDnVecDescr_t) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSpVV_bufferSize(handle: cusparseHandle_t, opX: cusparseOperation_t, vecX: cusparseConstSpVecDescr_t, vecY: cusparseConstDnVecDescr_t, result: *const ::std::os::raw::c_void, computeType: cudaDataType, bufferSize: *mut usize) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSpVV(handle: cusparseHandle_t, opX: cusparseOperation_t, vecX: cusparseConstSpVecDescr_t, vecY: cusparseConstDnVecDescr_t, result: *mut ::std::os::raw::c_void, computeType: cudaDataType, externalBuffer: *mut ::std::os::raw::c_void) -> cusparseStatus_t;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cusparseSparseToDenseAlg_t {
    CUSPARSE_SPARSETODENSE_ALG_DEFAULT = 0,
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSparseToDense_bufferSize(handle: cusparseHandle_t, matA: cusparseConstSpMatDescr_t, matB: cusparseDnMatDescr_t, alg: cusparseSparseToDenseAlg_t, bufferSize: *mut usize) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSparseToDense(handle: cusparseHandle_t, matA: cusparseConstSpMatDescr_t, matB: cusparseDnMatDescr_t, alg: cusparseSparseToDenseAlg_t, externalBuffer: *mut ::std::os::raw::c_void) -> cusparseStatus_t;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cusparseDenseToSparseAlg_t {
    CUSPARSE_DENSETOSPARSE_ALG_DEFAULT = 0,
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDenseToSparse_bufferSize(handle: cusparseHandle_t, matA: cusparseConstDnMatDescr_t, matB: cusparseSpMatDescr_t, alg: cusparseDenseToSparseAlg_t, bufferSize: *mut usize) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDenseToSparse_analysis(handle: cusparseHandle_t, matA: cusparseConstDnMatDescr_t, matB: cusparseSpMatDescr_t, alg: cusparseDenseToSparseAlg_t, externalBuffer: *mut ::std::os::raw::c_void) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseDenseToSparse_convert(handle: cusparseHandle_t, matA: cusparseConstDnMatDescr_t, matB: cusparseSpMatDescr_t, alg: cusparseDenseToSparseAlg_t, externalBuffer: *mut ::std::os::raw::c_void) -> cusparseStatus_t;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cusparseSpMVAlg_t {
    CUSPARSE_SPMV_ALG_DEFAULT = 0,
    CUSPARSE_SPMV_CSR_ALG1 = 2,
    CUSPARSE_SPMV_CSR_ALG2 = 3,
    CUSPARSE_SPMV_COO_ALG1 = 1,
    CUSPARSE_SPMV_COO_ALG2 = 4,
    CUSPARSE_SPMV_SELL_ALG1 = 5,
    CUSPARSE_SPMV_BSR_ALG1 = 6,
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSpMV(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSpMV_bufferSize(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSpMV_preprocess(
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
    ) -> cusparseStatus_t;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cusparseSpSVAlg_t {
    CUSPARSE_SPSV_ALG_DEFAULT = 0,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cusparseSpSVUpdate_t {
    CUSPARSE_SPSV_UPDATE_GENERAL = 0,
    CUSPARSE_SPSV_UPDATE_DIAGONAL = 1,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cusparseSpSVDescr {
    _unused: [u8; 0],
}
pub type cusparseSpSVDescr_t = *mut cusparseSpSVDescr;
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSpSV_createDescr(descr: *mut cusparseSpSVDescr_t) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSpSV_destroyDescr(descr: cusparseSpSVDescr_t) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSpSV_bufferSize(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSpSV_analysis(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSpSV_solve(
        handle: cusparseHandle_t,
        opA: cusparseOperation_t,
        alpha: *const ::std::os::raw::c_void,
        matA: cusparseConstSpMatDescr_t,
        vecX: cusparseConstDnVecDescr_t,
        vecY: cusparseDnVecDescr_t,
        computeType: cudaDataType,
        alg: cusparseSpSVAlg_t,
        spsvDescr: cusparseSpSVDescr_t,
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSpSV_updateMatrix(handle: cusparseHandle_t, spsvDescr: cusparseSpSVDescr_t, newValues: *mut ::std::os::raw::c_void, updatePart: cusparseSpSVUpdate_t) -> cusparseStatus_t;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cusparseSpSMAlg_t {
    CUSPARSE_SPSM_ALG_DEFAULT = 0,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cusparseSpSMUpdate_t {
    CUSPARSE_SPSM_UPDATE_GENERAL = 0,
    CUSPARSE_SPSM_UPDATE_DIAGONAL = 1,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cusparseSpSMDescr {
    _unused: [u8; 0],
}
pub type cusparseSpSMDescr_t = *mut cusparseSpSMDescr;
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSpSM_createDescr(descr: *mut cusparseSpSMDescr_t) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSpSM_destroyDescr(descr: cusparseSpSMDescr_t) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSpSM_bufferSize(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSpSM_analysis(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSpSM_solve(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSpSM_updateMatrix(handle: cusparseHandle_t, spsmDescr: cusparseSpSMDescr_t, newValues: *mut ::std::os::raw::c_void, updatePart: cusparseSpSMUpdate_t) -> cusparseStatus_t;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cusparseSpMMAlg_t {
    CUSPARSE_SPMM_ALG_DEFAULT = 0,
    CUSPARSE_SPMM_COO_ALG1 = 1,
    CUSPARSE_SPMM_COO_ALG2 = 2,
    CUSPARSE_SPMM_COO_ALG3 = 3,
    CUSPARSE_SPMM_COO_ALG4 = 5,
    CUSPARSE_SPMM_CSR_ALG1 = 4,
    CUSPARSE_SPMM_CSR_ALG2 = 6,
    CUSPARSE_SPMM_CSR_ALG3 = 12,
    CUSPARSE_SPMM_BLOCKED_ELL_ALG1 = 13,
    CUSPARSE_SPMM_BSR_ALG1 = 14,
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSpMM_bufferSize(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSpMM_preprocess(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSpMM(
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
    ) -> cusparseStatus_t;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cusparseSpGEMMAlg_t {
    CUSPARSE_SPGEMM_DEFAULT = 0,
    CUSPARSE_SPGEMM_CSR_ALG_DETERMINITIC = 1,
    CUSPARSE_SPGEMM_CSR_ALG_NONDETERMINITIC = 2,
    CUSPARSE_SPGEMM_ALG1 = 3,
    CUSPARSE_SPGEMM_ALG2 = 4,
    CUSPARSE_SPGEMM_ALG3 = 5,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cusparseSpGEMMDescr {
    _unused: [u8; 0],
}
pub type cusparseSpGEMMDescr_t = *mut cusparseSpGEMMDescr;
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSpGEMM_createDescr(descr: *mut cusparseSpGEMMDescr_t) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSpGEMM_destroyDescr(descr: cusparseSpGEMMDescr_t) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSpGEMM_workEstimation(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSpGEMM_getNumProducts(spgemmDescr: cusparseSpGEMMDescr_t, num_prods: *mut i64) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSpGEMM_estimateMemory(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSpGEMM_compute(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSpGEMM_copy(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSpGEMMreuse_workEstimation(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSpGEMMreuse_nnz(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSpGEMMreuse_copy(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSpGEMMreuse_compute(
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
    ) -> cusparseStatus_t;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cusparseSDDMMAlg_t {
    CUSPARSE_SDDMM_ALG_DEFAULT = 0,
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSDDMM_bufferSize(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSDDMM_preprocess(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSDDMM(
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
    ) -> cusparseStatus_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cusparseSpMMOpPlan {
    _unused: [u8; 0],
}
pub type cusparseSpMMOpPlan_t = *mut cusparseSpMMOpPlan;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cusparseSpMMOpAlg_t {
    CUSPARSE_SPMM_OP_ALG_DEFAULT = 0,
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSpMMOp_createPlan(
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
    ) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSpMMOp(plan: cusparseSpMMOpPlan_t, externalBuffer: *mut ::std::os::raw::c_void) -> cusparseStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusparseSpMMOp_destroyPlan(plan: cusparseSpMMOpPlan_t) -> cusparseStatus_t;
}
#[cfg(feature = "runtime-link")]
pub struct DynamicBindings {
    pub cusparseCreate: Option<unsafe extern "C" fn(handle: *mut cusparseHandle_t) -> cusparseStatus_t>,
    pub cusparseDestroy: Option<unsafe extern "C" fn(handle: cusparseHandle_t) -> cusparseStatus_t>,
    pub cusparseGetVersion: Option<unsafe extern "C" fn(handle: cusparseHandle_t, version: *mut ::std::os::raw::c_int) -> cusparseStatus_t>,
    pub cusparseGetProperty: Option<unsafe extern "C" fn(type_: libraryPropertyType, value: *mut ::std::os::raw::c_int) -> cusparseStatus_t>,
    pub cusparseGetErrorName: Option<unsafe extern "C" fn(status: cusparseStatus_t) -> *const ::std::os::raw::c_char>,
    pub cusparseGetErrorString: Option<unsafe extern "C" fn(status: cusparseStatus_t) -> *const ::std::os::raw::c_char>,
    pub cusparseSetStream: Option<unsafe extern "C" fn(handle: cusparseHandle_t, streamId: cudaStream_t) -> cusparseStatus_t>,
    pub cusparseGetStream: Option<unsafe extern "C" fn(handle: cusparseHandle_t, streamId: *mut cudaStream_t) -> cusparseStatus_t>,
    pub cusparseGetPointerMode: Option<unsafe extern "C" fn(handle: cusparseHandle_t, mode: *mut cusparsePointerMode_t) -> cusparseStatus_t>,
    pub cusparseSetPointerMode: Option<unsafe extern "C" fn(handle: cusparseHandle_t, mode: cusparsePointerMode_t) -> cusparseStatus_t>,
    pub cusparseLoggerSetCallback: Option<unsafe extern "C" fn(callback: cusparseLoggerCallback_t) -> cusparseStatus_t>,
    pub cusparseLoggerSetFile: Option<unsafe extern "C" fn(file: *mut FILE) -> cusparseStatus_t>,
    pub cusparseLoggerOpenFile: Option<unsafe extern "C" fn(logFile: *const ::std::os::raw::c_char) -> cusparseStatus_t>,
    pub cusparseLoggerSetLevel: Option<unsafe extern "C" fn(level: ::std::os::raw::c_int) -> cusparseStatus_t>,
    pub cusparseLoggerSetMask: Option<unsafe extern "C" fn(mask: ::std::os::raw::c_int) -> cusparseStatus_t>,
    pub cusparseLoggerForceDisable: Option<unsafe extern "C" fn() -> cusparseStatus_t>,
    pub cusparseCreateMatDescr: Option<unsafe extern "C" fn(descrA: *mut cusparseMatDescr_t) -> cusparseStatus_t>,
    pub cusparseDestroyMatDescr: Option<unsafe extern "C" fn(descrA: cusparseMatDescr_t) -> cusparseStatus_t>,
    pub cusparseSetMatType: Option<unsafe extern "C" fn(descrA: cusparseMatDescr_t, type_: cusparseMatrixType_t) -> cusparseStatus_t>,
    pub cusparseGetMatType: Option<unsafe extern "C" fn(descrA: cusparseMatDescr_t) -> cusparseMatrixType_t>,
    pub cusparseSetMatFillMode: Option<unsafe extern "C" fn(descrA: cusparseMatDescr_t, fillMode: cusparseFillMode_t) -> cusparseStatus_t>,
    pub cusparseGetMatFillMode: Option<unsafe extern "C" fn(descrA: cusparseMatDescr_t) -> cusparseFillMode_t>,
    pub cusparseSetMatDiagType: Option<unsafe extern "C" fn(descrA: cusparseMatDescr_t, diagType: cusparseDiagType_t) -> cusparseStatus_t>,
    pub cusparseGetMatDiagType: Option<unsafe extern "C" fn(descrA: cusparseMatDescr_t) -> cusparseDiagType_t>,
    pub cusparseSetMatIndexBase: Option<unsafe extern "C" fn(descrA: cusparseMatDescr_t, base: cusparseIndexBase_t) -> cusparseStatus_t>,
    pub cusparseGetMatIndexBase: Option<unsafe extern "C" fn(descrA: cusparseMatDescr_t) -> cusparseIndexBase_t>,
    pub cusparseCreateCsric02Info: Option<unsafe extern "C" fn(info: *mut csric02Info_t) -> cusparseStatus_t>,
    pub cusparseDestroyCsric02Info: Option<unsafe extern "C" fn(info: csric02Info_t) -> cusparseStatus_t>,
    pub cusparseCreateBsric02Info: Option<unsafe extern "C" fn(info: *mut bsric02Info_t) -> cusparseStatus_t>,
    pub cusparseDestroyBsric02Info: Option<unsafe extern "C" fn(info: bsric02Info_t) -> cusparseStatus_t>,
    pub cusparseCreateCsrilu02Info: Option<unsafe extern "C" fn(info: *mut csrilu02Info_t) -> cusparseStatus_t>,
    pub cusparseDestroyCsrilu02Info: Option<unsafe extern "C" fn(info: csrilu02Info_t) -> cusparseStatus_t>,
    pub cusparseCreateBsrilu02Info: Option<unsafe extern "C" fn(info: *mut bsrilu02Info_t) -> cusparseStatus_t>,
    pub cusparseDestroyBsrilu02Info: Option<unsafe extern "C" fn(info: bsrilu02Info_t) -> cusparseStatus_t>,
    pub cusparseCreateBsrsv2Info: Option<unsafe extern "C" fn(info: *mut bsrsv2Info_t) -> cusparseStatus_t>,
    pub cusparseDestroyBsrsv2Info: Option<unsafe extern "C" fn(info: bsrsv2Info_t) -> cusparseStatus_t>,
    pub cusparseCreateBsrsm2Info: Option<unsafe extern "C" fn(info: *mut bsrsm2Info_t) -> cusparseStatus_t>,
    pub cusparseDestroyBsrsm2Info: Option<unsafe extern "C" fn(info: bsrsm2Info_t) -> cusparseStatus_t>,
    pub cusparseCreateCsru2csrInfo: Option<unsafe extern "C" fn(info: *mut csru2csrInfo_t) -> cusparseStatus_t>,
    pub cusparseDestroyCsru2csrInfo: Option<unsafe extern "C" fn(info: csru2csrInfo_t) -> cusparseStatus_t>,
    pub cusparseCreateColorInfo: Option<unsafe extern "C" fn(info: *mut cusparseColorInfo_t) -> cusparseStatus_t>,
    pub cusparseDestroyColorInfo: Option<unsafe extern "C" fn(info: cusparseColorInfo_t) -> cusparseStatus_t>,
    pub cusparseCreatePruneInfo: Option<unsafe extern "C" fn(info: *mut pruneInfo_t) -> cusparseStatus_t>,
    pub cusparseDestroyPruneInfo: Option<unsafe extern "C" fn(info: pruneInfo_t) -> cusparseStatus_t>,
    pub cusparseSgemvi: Option<
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
    pub cusparseSgemvi_bufferSize: Option<unsafe extern "C" fn(handle: cusparseHandle_t, transA: cusparseOperation_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, nnz: ::std::os::raw::c_int, pBufferSize: *mut ::std::os::raw::c_int) -> cusparseStatus_t>,
    pub cusparseDgemvi: Option<
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
    pub cusparseDgemvi_bufferSize: Option<unsafe extern "C" fn(handle: cusparseHandle_t, transA: cusparseOperation_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, nnz: ::std::os::raw::c_int, pBufferSize: *mut ::std::os::raw::c_int) -> cusparseStatus_t>,
    pub cusparseCgemvi: Option<
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
    pub cusparseCgemvi_bufferSize: Option<unsafe extern "C" fn(handle: cusparseHandle_t, transA: cusparseOperation_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, nnz: ::std::os::raw::c_int, pBufferSize: *mut ::std::os::raw::c_int) -> cusparseStatus_t>,
    pub cusparseZgemvi: Option<
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
    pub cusparseZgemvi_bufferSize: Option<unsafe extern "C" fn(handle: cusparseHandle_t, transA: cusparseOperation_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, nnz: ::std::os::raw::c_int, pBufferSize: *mut ::std::os::raw::c_int) -> cusparseStatus_t>,
    pub cusparseSbsrmv: Option<
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
    pub cusparseDbsrmv: Option<
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
    pub cusparseCbsrmv: Option<
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
    pub cusparseZbsrmv: Option<
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
    pub cusparseSbsrxmv: Option<
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
    pub cusparseDbsrxmv: Option<
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
    pub cusparseCbsrxmv: Option<
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
    pub cusparseZbsrxmv: Option<
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
    pub cusparseXbsrsv2_zeroPivot: Option<unsafe extern "C" fn(handle: cusparseHandle_t, info: bsrsv2Info_t, position: *mut ::std::os::raw::c_int) -> cusparseStatus_t>,
    pub cusparseSbsrsv2_bufferSize: Option<
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
    pub cusparseDbsrsv2_bufferSize: Option<
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
    pub cusparseCbsrsv2_bufferSize: Option<
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
    pub cusparseZbsrsv2_bufferSize: Option<
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
    pub cusparseSbsrsv2_bufferSizeExt: Option<
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
    pub cusparseDbsrsv2_bufferSizeExt: Option<
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
    pub cusparseCbsrsv2_bufferSizeExt: Option<
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
    pub cusparseZbsrsv2_bufferSizeExt: Option<
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
    pub cusparseSbsrsv2_analysis: Option<
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
    pub cusparseDbsrsv2_analysis: Option<
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
    pub cusparseCbsrsv2_analysis: Option<
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
    pub cusparseZbsrsv2_analysis: Option<
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
    pub cusparseSbsrsv2_solve: Option<
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
    pub cusparseDbsrsv2_solve: Option<
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
    pub cusparseCbsrsv2_solve: Option<
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
    pub cusparseZbsrsv2_solve: Option<
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
    pub cusparseSbsrmm: Option<
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
    pub cusparseDbsrmm: Option<
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
    pub cusparseCbsrmm: Option<
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
    pub cusparseZbsrmm: Option<
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
    pub cusparseXbsrsm2_zeroPivot: Option<unsafe extern "C" fn(handle: cusparseHandle_t, info: bsrsm2Info_t, position: *mut ::std::os::raw::c_int) -> cusparseStatus_t>,
    pub cusparseSbsrsm2_bufferSize: Option<
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
    pub cusparseDbsrsm2_bufferSize: Option<
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
    pub cusparseCbsrsm2_bufferSize: Option<
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
    pub cusparseZbsrsm2_bufferSize: Option<
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
    pub cusparseSbsrsm2_bufferSizeExt: Option<
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
    pub cusparseDbsrsm2_bufferSizeExt: Option<
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
    pub cusparseCbsrsm2_bufferSizeExt: Option<
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
    pub cusparseZbsrsm2_bufferSizeExt: Option<
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
    pub cusparseSbsrsm2_analysis: Option<
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
    pub cusparseDbsrsm2_analysis: Option<
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
    pub cusparseCbsrsm2_analysis: Option<
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
    pub cusparseZbsrsm2_analysis: Option<
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
    pub cusparseSbsrsm2_solve: Option<
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
    pub cusparseDbsrsm2_solve: Option<
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
    pub cusparseCbsrsm2_solve: Option<
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
    pub cusparseZbsrsm2_solve: Option<
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
    pub cusparseScsrilu02_numericBoost: Option<unsafe extern "C" fn(handle: cusparseHandle_t, info: csrilu02Info_t, enable_boost: ::std::os::raw::c_int, tol: *mut f64, boost_val: *mut f32) -> cusparseStatus_t>,
    pub cusparseDcsrilu02_numericBoost: Option<unsafe extern "C" fn(handle: cusparseHandle_t, info: csrilu02Info_t, enable_boost: ::std::os::raw::c_int, tol: *mut f64, boost_val: *mut f64) -> cusparseStatus_t>,
    pub cusparseCcsrilu02_numericBoost: Option<unsafe extern "C" fn(handle: cusparseHandle_t, info: csrilu02Info_t, enable_boost: ::std::os::raw::c_int, tol: *mut f64, boost_val: *mut cuComplex) -> cusparseStatus_t>,
    pub cusparseZcsrilu02_numericBoost: Option<unsafe extern "C" fn(handle: cusparseHandle_t, info: csrilu02Info_t, enable_boost: ::std::os::raw::c_int, tol: *mut f64, boost_val: *mut cuDoubleComplex) -> cusparseStatus_t>,
    pub cusparseXcsrilu02_zeroPivot: Option<unsafe extern "C" fn(handle: cusparseHandle_t, info: csrilu02Info_t, position: *mut ::std::os::raw::c_int) -> cusparseStatus_t>,
    pub cusparseScsrilu02_bufferSize: Option<
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
    pub cusparseDcsrilu02_bufferSize: Option<
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
    pub cusparseCcsrilu02_bufferSize: Option<
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
    pub cusparseZcsrilu02_bufferSize: Option<
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
    pub cusparseScsrilu02_bufferSizeExt: Option<
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
    pub cusparseDcsrilu02_bufferSizeExt: Option<
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
    pub cusparseCcsrilu02_bufferSizeExt: Option<
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
    pub cusparseZcsrilu02_bufferSizeExt: Option<
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
    pub cusparseScsrilu02_analysis: Option<
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
    pub cusparseDcsrilu02_analysis: Option<
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
    pub cusparseCcsrilu02_analysis: Option<
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
    pub cusparseZcsrilu02_analysis: Option<
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
    pub cusparseScsrilu02: Option<
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
    pub cusparseDcsrilu02: Option<
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
    pub cusparseCcsrilu02: Option<
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
    pub cusparseZcsrilu02: Option<
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
    pub cusparseSbsrilu02_numericBoost: Option<unsafe extern "C" fn(handle: cusparseHandle_t, info: bsrilu02Info_t, enable_boost: ::std::os::raw::c_int, tol: *mut f64, boost_val: *mut f32) -> cusparseStatus_t>,
    pub cusparseDbsrilu02_numericBoost: Option<unsafe extern "C" fn(handle: cusparseHandle_t, info: bsrilu02Info_t, enable_boost: ::std::os::raw::c_int, tol: *mut f64, boost_val: *mut f64) -> cusparseStatus_t>,
    pub cusparseCbsrilu02_numericBoost: Option<unsafe extern "C" fn(handle: cusparseHandle_t, info: bsrilu02Info_t, enable_boost: ::std::os::raw::c_int, tol: *mut f64, boost_val: *mut cuComplex) -> cusparseStatus_t>,
    pub cusparseZbsrilu02_numericBoost: Option<unsafe extern "C" fn(handle: cusparseHandle_t, info: bsrilu02Info_t, enable_boost: ::std::os::raw::c_int, tol: *mut f64, boost_val: *mut cuDoubleComplex) -> cusparseStatus_t>,
    pub cusparseXbsrilu02_zeroPivot: Option<unsafe extern "C" fn(handle: cusparseHandle_t, info: bsrilu02Info_t, position: *mut ::std::os::raw::c_int) -> cusparseStatus_t>,
    pub cusparseSbsrilu02_bufferSize: Option<
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
    pub cusparseDbsrilu02_bufferSize: Option<
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
    pub cusparseCbsrilu02_bufferSize: Option<
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
    pub cusparseZbsrilu02_bufferSize: Option<
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
    pub cusparseSbsrilu02_bufferSizeExt: Option<
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
    pub cusparseDbsrilu02_bufferSizeExt: Option<
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
    pub cusparseCbsrilu02_bufferSizeExt: Option<
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
    pub cusparseZbsrilu02_bufferSizeExt: Option<
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
    pub cusparseSbsrilu02_analysis: Option<
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
    pub cusparseDbsrilu02_analysis: Option<
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
    pub cusparseCbsrilu02_analysis: Option<
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
    pub cusparseZbsrilu02_analysis: Option<
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
    pub cusparseSbsrilu02: Option<
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
    pub cusparseDbsrilu02: Option<
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
    pub cusparseCbsrilu02: Option<
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
    pub cusparseZbsrilu02: Option<
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
    pub cusparseXcsric02_zeroPivot: Option<unsafe extern "C" fn(handle: cusparseHandle_t, info: csric02Info_t, position: *mut ::std::os::raw::c_int) -> cusparseStatus_t>,
    pub cusparseScsric02_bufferSize: Option<
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
    pub cusparseDcsric02_bufferSize: Option<
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
    pub cusparseCcsric02_bufferSize: Option<
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
    pub cusparseZcsric02_bufferSize: Option<
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
    pub cusparseScsric02_bufferSizeExt: Option<
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
    pub cusparseDcsric02_bufferSizeExt: Option<
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
    pub cusparseCcsric02_bufferSizeExt: Option<
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
    pub cusparseZcsric02_bufferSizeExt: Option<
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
    pub cusparseScsric02_analysis: Option<
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
    pub cusparseDcsric02_analysis: Option<
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
    pub cusparseCcsric02_analysis: Option<
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
    pub cusparseZcsric02_analysis: Option<
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
    pub cusparseScsric02: Option<
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
    pub cusparseDcsric02: Option<
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
    pub cusparseCcsric02: Option<
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
    pub cusparseZcsric02: Option<
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
    pub cusparseXbsric02_zeroPivot: Option<unsafe extern "C" fn(handle: cusparseHandle_t, info: bsric02Info_t, position: *mut ::std::os::raw::c_int) -> cusparseStatus_t>,
    pub cusparseSbsric02_bufferSize: Option<
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
    pub cusparseDbsric02_bufferSize: Option<
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
    pub cusparseCbsric02_bufferSize: Option<
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
    pub cusparseZbsric02_bufferSize: Option<
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
    pub cusparseSbsric02_bufferSizeExt: Option<
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
    pub cusparseDbsric02_bufferSizeExt: Option<
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
    pub cusparseCbsric02_bufferSizeExt: Option<
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
    pub cusparseZbsric02_bufferSizeExt: Option<
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
    pub cusparseSbsric02_analysis: Option<
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
    pub cusparseDbsric02_analysis: Option<
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
    pub cusparseCbsric02_analysis: Option<
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
    pub cusparseZbsric02_analysis: Option<
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
    pub cusparseSbsric02: Option<
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
    pub cusparseDbsric02: Option<
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
    pub cusparseCbsric02: Option<
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
    pub cusparseZbsric02: Option<
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
    pub cusparseSgtsv2_bufferSizeExt: Option<unsafe extern "C" fn(handle: cusparseHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, dl: *const f32, d: *const f32, du: *const f32, B: *const f32, ldb: ::std::os::raw::c_int, bufferSizeInBytes: *mut usize) -> cusparseStatus_t>,
    pub cusparseDgtsv2_bufferSizeExt: Option<unsafe extern "C" fn(handle: cusparseHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, dl: *const f64, d: *const f64, du: *const f64, B: *const f64, ldb: ::std::os::raw::c_int, bufferSizeInBytes: *mut usize) -> cusparseStatus_t>,
    pub cusparseCgtsv2_bufferSizeExt:
        Option<unsafe extern "C" fn(handle: cusparseHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, dl: *const cuComplex, d: *const cuComplex, du: *const cuComplex, B: *const cuComplex, ldb: ::std::os::raw::c_int, bufferSizeInBytes: *mut usize) -> cusparseStatus_t>,
    pub cusparseZgtsv2_bufferSizeExt: Option<
        unsafe extern "C" fn(handle: cusparseHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, dl: *const cuDoubleComplex, d: *const cuDoubleComplex, du: *const cuDoubleComplex, B: *const cuDoubleComplex, ldb: ::std::os::raw::c_int, bufferSizeInBytes: *mut usize) -> cusparseStatus_t,
    >,
    pub cusparseSgtsv2: Option<unsafe extern "C" fn(handle: cusparseHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, dl: *const f32, d: *const f32, du: *const f32, B: *mut f32, ldb: ::std::os::raw::c_int, pBuffer: *mut ::std::os::raw::c_void) -> cusparseStatus_t>,
    pub cusparseDgtsv2: Option<unsafe extern "C" fn(handle: cusparseHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, dl: *const f64, d: *const f64, du: *const f64, B: *mut f64, ldb: ::std::os::raw::c_int, pBuffer: *mut ::std::os::raw::c_void) -> cusparseStatus_t>,
    pub cusparseCgtsv2:
        Option<unsafe extern "C" fn(handle: cusparseHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, dl: *const cuComplex, d: *const cuComplex, du: *const cuComplex, B: *mut cuComplex, ldb: ::std::os::raw::c_int, pBuffer: *mut ::std::os::raw::c_void) -> cusparseStatus_t>,
    pub cusparseZgtsv2: Option<
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
    pub cusparseSgtsv2_nopivot_bufferSizeExt: Option<unsafe extern "C" fn(handle: cusparseHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, dl: *const f32, d: *const f32, du: *const f32, B: *const f32, ldb: ::std::os::raw::c_int, bufferSizeInBytes: *mut usize) -> cusparseStatus_t>,
    pub cusparseDgtsv2_nopivot_bufferSizeExt: Option<unsafe extern "C" fn(handle: cusparseHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, dl: *const f64, d: *const f64, du: *const f64, B: *const f64, ldb: ::std::os::raw::c_int, bufferSizeInBytes: *mut usize) -> cusparseStatus_t>,
    pub cusparseCgtsv2_nopivot_bufferSizeExt:
        Option<unsafe extern "C" fn(handle: cusparseHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, dl: *const cuComplex, d: *const cuComplex, du: *const cuComplex, B: *const cuComplex, ldb: ::std::os::raw::c_int, bufferSizeInBytes: *mut usize) -> cusparseStatus_t>,
    pub cusparseZgtsv2_nopivot_bufferSizeExt: Option<
        unsafe extern "C" fn(handle: cusparseHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, dl: *const cuDoubleComplex, d: *const cuDoubleComplex, du: *const cuDoubleComplex, B: *const cuDoubleComplex, ldb: ::std::os::raw::c_int, bufferSizeInBytes: *mut usize) -> cusparseStatus_t,
    >,
    pub cusparseSgtsv2_nopivot: Option<unsafe extern "C" fn(handle: cusparseHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, dl: *const f32, d: *const f32, du: *const f32, B: *mut f32, ldb: ::std::os::raw::c_int, pBuffer: *mut ::std::os::raw::c_void) -> cusparseStatus_t>,
    pub cusparseDgtsv2_nopivot: Option<unsafe extern "C" fn(handle: cusparseHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, dl: *const f64, d: *const f64, du: *const f64, B: *mut f64, ldb: ::std::os::raw::c_int, pBuffer: *mut ::std::os::raw::c_void) -> cusparseStatus_t>,
    pub cusparseCgtsv2_nopivot:
        Option<unsafe extern "C" fn(handle: cusparseHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, dl: *const cuComplex, d: *const cuComplex, du: *const cuComplex, B: *mut cuComplex, ldb: ::std::os::raw::c_int, pBuffer: *mut ::std::os::raw::c_void) -> cusparseStatus_t>,
    pub cusparseZgtsv2_nopivot: Option<
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
    pub cusparseSgtsv2StridedBatch_bufferSizeExt:
        Option<unsafe extern "C" fn(handle: cusparseHandle_t, m: ::std::os::raw::c_int, dl: *const f32, d: *const f32, du: *const f32, x: *const f32, batchCount: ::std::os::raw::c_int, batchStride: ::std::os::raw::c_int, bufferSizeInBytes: *mut usize) -> cusparseStatus_t>,
    pub cusparseDgtsv2StridedBatch_bufferSizeExt:
        Option<unsafe extern "C" fn(handle: cusparseHandle_t, m: ::std::os::raw::c_int, dl: *const f64, d: *const f64, du: *const f64, x: *const f64, batchCount: ::std::os::raw::c_int, batchStride: ::std::os::raw::c_int, bufferSizeInBytes: *mut usize) -> cusparseStatus_t>,
    pub cusparseCgtsv2StridedBatch_bufferSizeExt:
        Option<unsafe extern "C" fn(handle: cusparseHandle_t, m: ::std::os::raw::c_int, dl: *const cuComplex, d: *const cuComplex, du: *const cuComplex, x: *const cuComplex, batchCount: ::std::os::raw::c_int, batchStride: ::std::os::raw::c_int, bufferSizeInBytes: *mut usize) -> cusparseStatus_t>,
    pub cusparseZgtsv2StridedBatch_bufferSizeExt: Option<
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
    pub cusparseSgtsv2StridedBatch:
        Option<unsafe extern "C" fn(handle: cusparseHandle_t, m: ::std::os::raw::c_int, dl: *const f32, d: *const f32, du: *const f32, x: *mut f32, batchCount: ::std::os::raw::c_int, batchStride: ::std::os::raw::c_int, pBuffer: *mut ::std::os::raw::c_void) -> cusparseStatus_t>,
    pub cusparseDgtsv2StridedBatch:
        Option<unsafe extern "C" fn(handle: cusparseHandle_t, m: ::std::os::raw::c_int, dl: *const f64, d: *const f64, du: *const f64, x: *mut f64, batchCount: ::std::os::raw::c_int, batchStride: ::std::os::raw::c_int, pBuffer: *mut ::std::os::raw::c_void) -> cusparseStatus_t>,
    pub cusparseCgtsv2StridedBatch: Option<
        unsafe extern "C" fn(handle: cusparseHandle_t, m: ::std::os::raw::c_int, dl: *const cuComplex, d: *const cuComplex, du: *const cuComplex, x: *mut cuComplex, batchCount: ::std::os::raw::c_int, batchStride: ::std::os::raw::c_int, pBuffer: *mut ::std::os::raw::c_void) -> cusparseStatus_t,
    >,
    pub cusparseZgtsv2StridedBatch: Option<
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
    pub cusparseSgtsvInterleavedBatch_bufferSizeExt:
        Option<unsafe extern "C" fn(handle: cusparseHandle_t, algo: ::std::os::raw::c_int, m: ::std::os::raw::c_int, dl: *const f32, d: *const f32, du: *const f32, x: *const f32, batchCount: ::std::os::raw::c_int, pBufferSizeInBytes: *mut usize) -> cusparseStatus_t>,
    pub cusparseDgtsvInterleavedBatch_bufferSizeExt:
        Option<unsafe extern "C" fn(handle: cusparseHandle_t, algo: ::std::os::raw::c_int, m: ::std::os::raw::c_int, dl: *const f64, d: *const f64, du: *const f64, x: *const f64, batchCount: ::std::os::raw::c_int, pBufferSizeInBytes: *mut usize) -> cusparseStatus_t>,
    pub cusparseCgtsvInterleavedBatch_bufferSizeExt:
        Option<unsafe extern "C" fn(handle: cusparseHandle_t, algo: ::std::os::raw::c_int, m: ::std::os::raw::c_int, dl: *const cuComplex, d: *const cuComplex, du: *const cuComplex, x: *const cuComplex, batchCount: ::std::os::raw::c_int, pBufferSizeInBytes: *mut usize) -> cusparseStatus_t>,
    pub cusparseZgtsvInterleavedBatch_bufferSizeExt: Option<
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
    pub cusparseSgtsvInterleavedBatch: Option<unsafe extern "C" fn(handle: cusparseHandle_t, algo: ::std::os::raw::c_int, m: ::std::os::raw::c_int, dl: *mut f32, d: *mut f32, du: *mut f32, x: *mut f32, batchCount: ::std::os::raw::c_int, pBuffer: *mut ::std::os::raw::c_void) -> cusparseStatus_t>,
    pub cusparseDgtsvInterleavedBatch: Option<unsafe extern "C" fn(handle: cusparseHandle_t, algo: ::std::os::raw::c_int, m: ::std::os::raw::c_int, dl: *mut f64, d: *mut f64, du: *mut f64, x: *mut f64, batchCount: ::std::os::raw::c_int, pBuffer: *mut ::std::os::raw::c_void) -> cusparseStatus_t>,
    pub cusparseCgtsvInterleavedBatch:
        Option<unsafe extern "C" fn(handle: cusparseHandle_t, algo: ::std::os::raw::c_int, m: ::std::os::raw::c_int, dl: *mut cuComplex, d: *mut cuComplex, du: *mut cuComplex, x: *mut cuComplex, batchCount: ::std::os::raw::c_int, pBuffer: *mut ::std::os::raw::c_void) -> cusparseStatus_t>,
    pub cusparseZgtsvInterleavedBatch: Option<
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
    pub cusparseSgpsvInterleavedBatch_bufferSizeExt:
        Option<unsafe extern "C" fn(handle: cusparseHandle_t, algo: ::std::os::raw::c_int, m: ::std::os::raw::c_int, ds: *const f32, dl: *const f32, d: *const f32, du: *const f32, dw: *const f32, x: *const f32, batchCount: ::std::os::raw::c_int, pBufferSizeInBytes: *mut usize) -> cusparseStatus_t>,
    pub cusparseDgpsvInterleavedBatch_bufferSizeExt:
        Option<unsafe extern "C" fn(handle: cusparseHandle_t, algo: ::std::os::raw::c_int, m: ::std::os::raw::c_int, ds: *const f64, dl: *const f64, d: *const f64, du: *const f64, dw: *const f64, x: *const f64, batchCount: ::std::os::raw::c_int, pBufferSizeInBytes: *mut usize) -> cusparseStatus_t>,
    pub cusparseCgpsvInterleavedBatch_bufferSizeExt: Option<
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
    pub cusparseZgpsvInterleavedBatch_bufferSizeExt: Option<
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
    pub cusparseSgpsvInterleavedBatch:
        Option<unsafe extern "C" fn(handle: cusparseHandle_t, algo: ::std::os::raw::c_int, m: ::std::os::raw::c_int, ds: *mut f32, dl: *mut f32, d: *mut f32, du: *mut f32, dw: *mut f32, x: *mut f32, batchCount: ::std::os::raw::c_int, pBuffer: *mut ::std::os::raw::c_void) -> cusparseStatus_t>,
    pub cusparseDgpsvInterleavedBatch:
        Option<unsafe extern "C" fn(handle: cusparseHandle_t, algo: ::std::os::raw::c_int, m: ::std::os::raw::c_int, ds: *mut f64, dl: *mut f64, d: *mut f64, du: *mut f64, dw: *mut f64, x: *mut f64, batchCount: ::std::os::raw::c_int, pBuffer: *mut ::std::os::raw::c_void) -> cusparseStatus_t>,
    pub cusparseCgpsvInterleavedBatch: Option<
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
    pub cusparseZgpsvInterleavedBatch: Option<
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
    pub cusparseScsrgeam2_bufferSizeExt: Option<
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
    pub cusparseDcsrgeam2_bufferSizeExt: Option<
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
    pub cusparseCcsrgeam2_bufferSizeExt: Option<
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
    pub cusparseZcsrgeam2_bufferSizeExt: Option<
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
    pub cusparseXcsrgeam2Nnz: Option<
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
    pub cusparseScsrgeam2: Option<
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
    pub cusparseDcsrgeam2: Option<
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
    pub cusparseCcsrgeam2: Option<
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
    pub cusparseZcsrgeam2: Option<
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
    pub cusparseScsrcolor: Option<
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
    pub cusparseDcsrcolor: Option<
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
    pub cusparseCcsrcolor: Option<
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
    pub cusparseZcsrcolor: Option<
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
    pub cusparseSnnz: Option<
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
    pub cusparseDnnz: Option<
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
    pub cusparseCnnz: Option<
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
    pub cusparseZnnz: Option<
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
    pub cusparseSnnz_compress:
        Option<unsafe extern "C" fn(handle: cusparseHandle_t, m: ::std::os::raw::c_int, descr: cusparseMatDescr_t, csrSortedValA: *const f32, csrSortedRowPtrA: *const ::std::os::raw::c_int, nnzPerRow: *mut ::std::os::raw::c_int, nnzC: *mut ::std::os::raw::c_int, tol: f32) -> cusparseStatus_t>,
    pub cusparseDnnz_compress:
        Option<unsafe extern "C" fn(handle: cusparseHandle_t, m: ::std::os::raw::c_int, descr: cusparseMatDescr_t, csrSortedValA: *const f64, csrSortedRowPtrA: *const ::std::os::raw::c_int, nnzPerRow: *mut ::std::os::raw::c_int, nnzC: *mut ::std::os::raw::c_int, tol: f64) -> cusparseStatus_t>,
    pub cusparseCnnz_compress: Option<
        unsafe extern "C" fn(handle: cusparseHandle_t, m: ::std::os::raw::c_int, descr: cusparseMatDescr_t, csrSortedValA: *const cuComplex, csrSortedRowPtrA: *const ::std::os::raw::c_int, nnzPerRow: *mut ::std::os::raw::c_int, nnzC: *mut ::std::os::raw::c_int, tol: cuComplex) -> cusparseStatus_t,
    >,
    pub cusparseZnnz_compress: Option<
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
    pub cusparseScsr2csr_compress: Option<
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
    pub cusparseDcsr2csr_compress: Option<
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
    pub cusparseCcsr2csr_compress: Option<
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
    pub cusparseZcsr2csr_compress: Option<
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
    pub cusparseXcoo2csr: Option<unsafe extern "C" fn(handle: cusparseHandle_t, cooRowInd: *const ::std::os::raw::c_int, nnz: ::std::os::raw::c_int, m: ::std::os::raw::c_int, csrSortedRowPtr: *mut ::std::os::raw::c_int, idxBase: cusparseIndexBase_t) -> cusparseStatus_t>,
    pub cusparseXcsr2coo: Option<unsafe extern "C" fn(handle: cusparseHandle_t, csrSortedRowPtr: *const ::std::os::raw::c_int, nnz: ::std::os::raw::c_int, m: ::std::os::raw::c_int, cooRowInd: *mut ::std::os::raw::c_int, idxBase: cusparseIndexBase_t) -> cusparseStatus_t>,
    pub cusparseXcsr2bsrNnz: Option<
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
    pub cusparseScsr2bsr: Option<
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
    pub cusparseDcsr2bsr: Option<
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
    pub cusparseCcsr2bsr: Option<
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
    pub cusparseZcsr2bsr: Option<
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
    pub cusparseSbsr2csr: Option<
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
    pub cusparseDbsr2csr: Option<
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
    pub cusparseCbsr2csr: Option<
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
    pub cusparseZbsr2csr: Option<
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
    pub cusparseSgebsr2gebsc_bufferSize: Option<
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
    pub cusparseDgebsr2gebsc_bufferSize: Option<
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
    pub cusparseCgebsr2gebsc_bufferSize: Option<
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
    pub cusparseZgebsr2gebsc_bufferSize: Option<
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
    pub cusparseSgebsr2gebsc_bufferSizeExt: Option<
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
    pub cusparseDgebsr2gebsc_bufferSizeExt: Option<
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
    pub cusparseCgebsr2gebsc_bufferSizeExt: Option<
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
    pub cusparseZgebsr2gebsc_bufferSizeExt: Option<
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
    pub cusparseSgebsr2gebsc: Option<
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
    pub cusparseDgebsr2gebsc: Option<
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
    pub cusparseCgebsr2gebsc: Option<
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
    pub cusparseZgebsr2gebsc: Option<
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
    pub cusparseXgebsr2csr: Option<
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
    pub cusparseSgebsr2csr: Option<
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
    pub cusparseDgebsr2csr: Option<
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
    pub cusparseCgebsr2csr: Option<
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
    pub cusparseZgebsr2csr: Option<
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
    pub cusparseScsr2gebsr_bufferSize: Option<
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
    pub cusparseDcsr2gebsr_bufferSize: Option<
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
    pub cusparseCcsr2gebsr_bufferSize: Option<
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
    pub cusparseZcsr2gebsr_bufferSize: Option<
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
    pub cusparseScsr2gebsr_bufferSizeExt: Option<
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
    pub cusparseDcsr2gebsr_bufferSizeExt: Option<
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
    pub cusparseCcsr2gebsr_bufferSizeExt: Option<
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
    pub cusparseZcsr2gebsr_bufferSizeExt: Option<
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
    pub cusparseXcsr2gebsrNnz: Option<
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
    pub cusparseScsr2gebsr: Option<
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
    pub cusparseDcsr2gebsr: Option<
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
    pub cusparseCcsr2gebsr: Option<
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
    pub cusparseZcsr2gebsr: Option<
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
    pub cusparseSgebsr2gebsr_bufferSize: Option<
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
    pub cusparseDgebsr2gebsr_bufferSize: Option<
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
    pub cusparseCgebsr2gebsr_bufferSize: Option<
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
    pub cusparseZgebsr2gebsr_bufferSize: Option<
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
    pub cusparseSgebsr2gebsr_bufferSizeExt: Option<
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
    pub cusparseDgebsr2gebsr_bufferSizeExt: Option<
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
    pub cusparseCgebsr2gebsr_bufferSizeExt: Option<
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
    pub cusparseZgebsr2gebsr_bufferSizeExt: Option<
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
    pub cusparseXgebsr2gebsrNnz: Option<
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
    pub cusparseSgebsr2gebsr: Option<
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
    pub cusparseDgebsr2gebsr: Option<
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
    pub cusparseCgebsr2gebsr: Option<
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
    pub cusparseZgebsr2gebsr: Option<
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
    pub cusparseCreateIdentityPermutation: Option<unsafe extern "C" fn(handle: cusparseHandle_t, n: ::std::os::raw::c_int, p: *mut ::std::os::raw::c_int) -> cusparseStatus_t>,
    pub cusparseXcoosort_bufferSizeExt:
        Option<unsafe extern "C" fn(handle: cusparseHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, nnz: ::std::os::raw::c_int, cooRowsA: *const ::std::os::raw::c_int, cooColsA: *const ::std::os::raw::c_int, pBufferSizeInBytes: *mut usize) -> cusparseStatus_t>,
    pub cusparseXcoosortByRow: Option<
        unsafe extern "C" fn(handle: cusparseHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, nnz: ::std::os::raw::c_int, cooRowsA: *mut ::std::os::raw::c_int, cooColsA: *mut ::std::os::raw::c_int, P: *mut ::std::os::raw::c_int, pBuffer: *mut ::std::os::raw::c_void) -> cusparseStatus_t,
    >,
    pub cusparseXcoosortByColumn: Option<
        unsafe extern "C" fn(handle: cusparseHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, nnz: ::std::os::raw::c_int, cooRowsA: *mut ::std::os::raw::c_int, cooColsA: *mut ::std::os::raw::c_int, P: *mut ::std::os::raw::c_int, pBuffer: *mut ::std::os::raw::c_void) -> cusparseStatus_t,
    >,
    pub cusparseXcsrsort_bufferSizeExt:
        Option<unsafe extern "C" fn(handle: cusparseHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, nnz: ::std::os::raw::c_int, csrRowPtrA: *const ::std::os::raw::c_int, csrColIndA: *const ::std::os::raw::c_int, pBufferSizeInBytes: *mut usize) -> cusparseStatus_t>,
    pub cusparseXcsrsort: Option<
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
    pub cusparseXcscsort_bufferSizeExt:
        Option<unsafe extern "C" fn(handle: cusparseHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, nnz: ::std::os::raw::c_int, cscColPtrA: *const ::std::os::raw::c_int, cscRowIndA: *const ::std::os::raw::c_int, pBufferSizeInBytes: *mut usize) -> cusparseStatus_t>,
    pub cusparseXcscsort: Option<
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
    pub cusparseScsru2csr_bufferSizeExt: Option<
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
    pub cusparseDcsru2csr_bufferSizeExt: Option<
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
    pub cusparseCcsru2csr_bufferSizeExt: Option<
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
    pub cusparseZcsru2csr_bufferSizeExt: Option<
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
    pub cusparseScsru2csr: Option<
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
    pub cusparseDcsru2csr: Option<
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
    pub cusparseCcsru2csr: Option<
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
    pub cusparseZcsru2csr: Option<
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
    pub cusparseScsr2csru: Option<
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
    pub cusparseDcsr2csru: Option<
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
    pub cusparseCcsr2csru: Option<
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
    pub cusparseZcsr2csru: Option<
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
    pub cusparseSpruneDense2csr_bufferSizeExt: Option<
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
    pub cusparseDpruneDense2csr_bufferSizeExt: Option<
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
    pub cusparseSpruneDense2csrNnz: Option<
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
    pub cusparseDpruneDense2csrNnz: Option<
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
    pub cusparseSpruneDense2csr: Option<
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
    pub cusparseDpruneDense2csr: Option<
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
    pub cusparseSpruneCsr2csr_bufferSizeExt: Option<
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
    pub cusparseDpruneCsr2csr_bufferSizeExt: Option<
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
    pub cusparseSpruneCsr2csrNnz: Option<
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
    pub cusparseDpruneCsr2csrNnz: Option<
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
    pub cusparseSpruneCsr2csr: Option<
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
    pub cusparseDpruneCsr2csr: Option<
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
    pub cusparseSpruneDense2csrByPercentage_bufferSizeExt: Option<
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
    pub cusparseDpruneDense2csrByPercentage_bufferSizeExt: Option<
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
    pub cusparseSpruneDense2csrNnzByPercentage: Option<
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
    pub cusparseDpruneDense2csrNnzByPercentage: Option<
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
    pub cusparseSpruneDense2csrByPercentage: Option<
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
    pub cusparseDpruneDense2csrByPercentage: Option<
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
    pub cusparseSpruneCsr2csrByPercentage_bufferSizeExt: Option<
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
    pub cusparseDpruneCsr2csrByPercentage_bufferSizeExt: Option<
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
    pub cusparseSpruneCsr2csrNnzByPercentage: Option<
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
    pub cusparseDpruneCsr2csrNnzByPercentage: Option<
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
    pub cusparseSpruneCsr2csrByPercentage: Option<
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
    pub cusparseDpruneCsr2csrByPercentage: Option<
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
    pub cusparseCsr2cscEx2: Option<
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
    pub cusparseCsr2cscEx2_bufferSize: Option<
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
    pub cusparseCreateSpVec: Option<unsafe extern "C" fn(spVecDescr: *mut cusparseSpVecDescr_t, size: i64, nnz: i64, indices: *mut ::std::os::raw::c_void, values: *mut ::std::os::raw::c_void, idxType: cusparseIndexType_t, idxBase: cusparseIndexBase_t, valueType: cudaDataType) -> cusparseStatus_t>,
    pub cusparseCreateConstSpVec:
        Option<unsafe extern "C" fn(spVecDescr: *mut cusparseConstSpVecDescr_t, size: i64, nnz: i64, indices: *const ::std::os::raw::c_void, values: *const ::std::os::raw::c_void, idxType: cusparseIndexType_t, idxBase: cusparseIndexBase_t, valueType: cudaDataType) -> cusparseStatus_t>,
    pub cusparseDestroySpVec: Option<unsafe extern "C" fn(spVecDescr: cusparseConstSpVecDescr_t) -> cusparseStatus_t>,
    pub cusparseSpVecGet: Option<
        unsafe extern "C" fn(spVecDescr: cusparseSpVecDescr_t, size: *mut i64, nnz: *mut i64, indices: *mut *mut ::std::os::raw::c_void, values: *mut *mut ::std::os::raw::c_void, idxType: *mut cusparseIndexType_t, idxBase: *mut cusparseIndexBase_t, valueType: *mut cudaDataType) -> cusparseStatus_t,
    >,
    pub cusparseConstSpVecGet: Option<
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
    pub cusparseSpVecGetIndexBase: Option<unsafe extern "C" fn(spVecDescr: cusparseConstSpVecDescr_t, idxBase: *mut cusparseIndexBase_t) -> cusparseStatus_t>,
    pub cusparseSpVecGetValues: Option<unsafe extern "C" fn(spVecDescr: cusparseSpVecDescr_t, values: *mut *mut ::std::os::raw::c_void) -> cusparseStatus_t>,
    pub cusparseConstSpVecGetValues: Option<unsafe extern "C" fn(spVecDescr: cusparseConstSpVecDescr_t, values: *mut *const ::std::os::raw::c_void) -> cusparseStatus_t>,
    pub cusparseSpVecSetValues: Option<unsafe extern "C" fn(spVecDescr: cusparseSpVecDescr_t, values: *mut ::std::os::raw::c_void) -> cusparseStatus_t>,
    pub cusparseCreateDnVec: Option<unsafe extern "C" fn(dnVecDescr: *mut cusparseDnVecDescr_t, size: i64, values: *mut ::std::os::raw::c_void, valueType: cudaDataType) -> cusparseStatus_t>,
    pub cusparseCreateConstDnVec: Option<unsafe extern "C" fn(dnVecDescr: *mut cusparseConstDnVecDescr_t, size: i64, values: *const ::std::os::raw::c_void, valueType: cudaDataType) -> cusparseStatus_t>,
    pub cusparseDestroyDnVec: Option<unsafe extern "C" fn(dnVecDescr: cusparseConstDnVecDescr_t) -> cusparseStatus_t>,
    pub cusparseDnVecGet: Option<unsafe extern "C" fn(dnVecDescr: cusparseDnVecDescr_t, size: *mut i64, values: *mut *mut ::std::os::raw::c_void, valueType: *mut cudaDataType) -> cusparseStatus_t>,
    pub cusparseConstDnVecGet: Option<unsafe extern "C" fn(dnVecDescr: cusparseConstDnVecDescr_t, size: *mut i64, values: *mut *const ::std::os::raw::c_void, valueType: *mut cudaDataType) -> cusparseStatus_t>,
    pub cusparseDnVecGetValues: Option<unsafe extern "C" fn(dnVecDescr: cusparseDnVecDescr_t, values: *mut *mut ::std::os::raw::c_void) -> cusparseStatus_t>,
    pub cusparseConstDnVecGetValues: Option<unsafe extern "C" fn(dnVecDescr: cusparseConstDnVecDescr_t, values: *mut *const ::std::os::raw::c_void) -> cusparseStatus_t>,
    pub cusparseDnVecSetValues: Option<unsafe extern "C" fn(dnVecDescr: cusparseDnVecDescr_t, values: *mut ::std::os::raw::c_void) -> cusparseStatus_t>,
    pub cusparseDestroySpMat: Option<unsafe extern "C" fn(spMatDescr: cusparseConstSpMatDescr_t) -> cusparseStatus_t>,
    pub cusparseSpMatGetFormat: Option<unsafe extern "C" fn(spMatDescr: cusparseConstSpMatDescr_t, format: *mut cusparseFormat_t) -> cusparseStatus_t>,
    pub cusparseSpMatGetIndexBase: Option<unsafe extern "C" fn(spMatDescr: cusparseConstSpMatDescr_t, idxBase: *mut cusparseIndexBase_t) -> cusparseStatus_t>,
    pub cusparseSpMatGetValues: Option<unsafe extern "C" fn(spMatDescr: cusparseSpMatDescr_t, values: *mut *mut ::std::os::raw::c_void) -> cusparseStatus_t>,
    pub cusparseConstSpMatGetValues: Option<unsafe extern "C" fn(spMatDescr: cusparseConstSpMatDescr_t, values: *mut *const ::std::os::raw::c_void) -> cusparseStatus_t>,
    pub cusparseSpMatSetValues: Option<unsafe extern "C" fn(spMatDescr: cusparseSpMatDescr_t, values: *mut ::std::os::raw::c_void) -> cusparseStatus_t>,
    pub cusparseSpMatGetSize: Option<unsafe extern "C" fn(spMatDescr: cusparseConstSpMatDescr_t, rows: *mut i64, cols: *mut i64, nnz: *mut i64) -> cusparseStatus_t>,
    pub cusparseSpMatGetStridedBatch: Option<unsafe extern "C" fn(spMatDescr: cusparseConstSpMatDescr_t, batchCount: *mut ::std::os::raw::c_int) -> cusparseStatus_t>,
    pub cusparseCooSetStridedBatch: Option<unsafe extern "C" fn(spMatDescr: cusparseSpMatDescr_t, batchCount: ::std::os::raw::c_int, batchStride: i64) -> cusparseStatus_t>,
    pub cusparseCsrSetStridedBatch: Option<unsafe extern "C" fn(spMatDescr: cusparseSpMatDescr_t, batchCount: ::std::os::raw::c_int, offsetsBatchStride: i64, columnsValuesBatchStride: i64) -> cusparseStatus_t>,
    pub cusparseBsrSetStridedBatch: Option<unsafe extern "C" fn(spMatDescr: cusparseSpMatDescr_t, batchCount: ::std::os::raw::c_int, offsetsBatchStride: i64, columnsBatchStride: i64, ValuesBatchStride: i64) -> cusparseStatus_t>,
    pub cusparseSpMatGetAttribute: Option<unsafe extern "C" fn(spMatDescr: cusparseConstSpMatDescr_t, attribute: cusparseSpMatAttribute_t, data: *mut ::std::os::raw::c_void, dataSize: usize) -> cusparseStatus_t>,
    pub cusparseSpMatSetAttribute: Option<unsafe extern "C" fn(spMatDescr: cusparseSpMatDescr_t, attribute: cusparseSpMatAttribute_t, data: *mut ::std::os::raw::c_void, dataSize: usize) -> cusparseStatus_t>,
    pub cusparseCreateCsr: Option<
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
    pub cusparseCreateConstCsr: Option<
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
    pub cusparseCreateCsc: Option<
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
    pub cusparseCreateConstCsc: Option<
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
    pub cusparseCsrGet: Option<
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
    pub cusparseConstCsrGet: Option<
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
    pub cusparseCscGet: Option<
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
    pub cusparseConstCscGet: Option<
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
    pub cusparseCsrSetPointers: Option<unsafe extern "C" fn(spMatDescr: cusparseSpMatDescr_t, csrRowOffsets: *mut ::std::os::raw::c_void, csrColInd: *mut ::std::os::raw::c_void, csrValues: *mut ::std::os::raw::c_void) -> cusparseStatus_t>,
    pub cusparseCscSetPointers: Option<unsafe extern "C" fn(spMatDescr: cusparseSpMatDescr_t, cscColOffsets: *mut ::std::os::raw::c_void, cscRowInd: *mut ::std::os::raw::c_void, cscValues: *mut ::std::os::raw::c_void) -> cusparseStatus_t>,
    pub cusparseCreateBsr: Option<
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
    pub cusparseCreateConstBsr: Option<
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
    pub cusparseCreateCoo: Option<
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
    pub cusparseCreateConstCoo: Option<
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
    pub cusparseCooGet: Option<
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
    pub cusparseConstCooGet: Option<
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
    pub cusparseCooSetPointers: Option<unsafe extern "C" fn(spMatDescr: cusparseSpMatDescr_t, cooRows: *mut ::std::os::raw::c_void, cooColumns: *mut ::std::os::raw::c_void, cooValues: *mut ::std::os::raw::c_void) -> cusparseStatus_t>,
    pub cusparseCreateBlockedEll: Option<
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
    pub cusparseCreateConstBlockedEll: Option<
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
    pub cusparseBlockedEllGet: Option<
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
    pub cusparseConstBlockedEllGet: Option<
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
    pub cusparseCreateSlicedEll: Option<
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
    pub cusparseCreateConstSlicedEll: Option<
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
    pub cusparseCreateDnMat: Option<unsafe extern "C" fn(dnMatDescr: *mut cusparseDnMatDescr_t, rows: i64, cols: i64, ld: i64, values: *mut ::std::os::raw::c_void, valueType: cudaDataType, order: cusparseOrder_t) -> cusparseStatus_t>,
    pub cusparseCreateConstDnMat: Option<unsafe extern "C" fn(dnMatDescr: *mut cusparseConstDnMatDescr_t, rows: i64, cols: i64, ld: i64, values: *const ::std::os::raw::c_void, valueType: cudaDataType, order: cusparseOrder_t) -> cusparseStatus_t>,
    pub cusparseDestroyDnMat: Option<unsafe extern "C" fn(dnMatDescr: cusparseConstDnMatDescr_t) -> cusparseStatus_t>,
    pub cusparseDnMatGet: Option<unsafe extern "C" fn(dnMatDescr: cusparseDnMatDescr_t, rows: *mut i64, cols: *mut i64, ld: *mut i64, values: *mut *mut ::std::os::raw::c_void, type_: *mut cudaDataType, order: *mut cusparseOrder_t) -> cusparseStatus_t>,
    pub cusparseConstDnMatGet: Option<unsafe extern "C" fn(dnMatDescr: cusparseConstDnMatDescr_t, rows: *mut i64, cols: *mut i64, ld: *mut i64, values: *mut *const ::std::os::raw::c_void, type_: *mut cudaDataType, order: *mut cusparseOrder_t) -> cusparseStatus_t>,
    pub cusparseDnMatGetValues: Option<unsafe extern "C" fn(dnMatDescr: cusparseDnMatDescr_t, values: *mut *mut ::std::os::raw::c_void) -> cusparseStatus_t>,
    pub cusparseConstDnMatGetValues: Option<unsafe extern "C" fn(dnMatDescr: cusparseConstDnMatDescr_t, values: *mut *const ::std::os::raw::c_void) -> cusparseStatus_t>,
    pub cusparseDnMatSetValues: Option<unsafe extern "C" fn(dnMatDescr: cusparseDnMatDescr_t, values: *mut ::std::os::raw::c_void) -> cusparseStatus_t>,
    pub cusparseDnMatSetStridedBatch: Option<unsafe extern "C" fn(dnMatDescr: cusparseDnMatDescr_t, batchCount: ::std::os::raw::c_int, batchStride: i64) -> cusparseStatus_t>,
    pub cusparseDnMatGetStridedBatch: Option<unsafe extern "C" fn(dnMatDescr: cusparseConstDnMatDescr_t, batchCount: *mut ::std::os::raw::c_int, batchStride: *mut i64) -> cusparseStatus_t>,
    pub cusparseAxpby: Option<unsafe extern "C" fn(handle: cusparseHandle_t, alpha: *const ::std::os::raw::c_void, vecX: cusparseConstSpVecDescr_t, beta: *const ::std::os::raw::c_void, vecY: cusparseDnVecDescr_t) -> cusparseStatus_t>,
    pub cusparseGather: Option<unsafe extern "C" fn(handle: cusparseHandle_t, vecY: cusparseConstDnVecDescr_t, vecX: cusparseSpVecDescr_t) -> cusparseStatus_t>,
    pub cusparseScatter: Option<unsafe extern "C" fn(handle: cusparseHandle_t, vecX: cusparseConstSpVecDescr_t, vecY: cusparseDnVecDescr_t) -> cusparseStatus_t>,
    pub cusparseRot: Option<unsafe extern "C" fn(handle: cusparseHandle_t, c_coeff: *const ::std::os::raw::c_void, s_coeff: *const ::std::os::raw::c_void, vecX: cusparseSpVecDescr_t, vecY: cusparseDnVecDescr_t) -> cusparseStatus_t>,
    pub cusparseSpVV_bufferSize: Option<unsafe extern "C" fn(handle: cusparseHandle_t, opX: cusparseOperation_t, vecX: cusparseConstSpVecDescr_t, vecY: cusparseConstDnVecDescr_t, result: *const ::std::os::raw::c_void, computeType: cudaDataType, bufferSize: *mut usize) -> cusparseStatus_t>,
    pub cusparseSpVV: Option<unsafe extern "C" fn(handle: cusparseHandle_t, opX: cusparseOperation_t, vecX: cusparseConstSpVecDescr_t, vecY: cusparseConstDnVecDescr_t, result: *mut ::std::os::raw::c_void, computeType: cudaDataType, externalBuffer: *mut ::std::os::raw::c_void) -> cusparseStatus_t>,
    pub cusparseSparseToDense_bufferSize: Option<unsafe extern "C" fn(handle: cusparseHandle_t, matA: cusparseConstSpMatDescr_t, matB: cusparseDnMatDescr_t, alg: cusparseSparseToDenseAlg_t, bufferSize: *mut usize) -> cusparseStatus_t>,
    pub cusparseSparseToDense: Option<unsafe extern "C" fn(handle: cusparseHandle_t, matA: cusparseConstSpMatDescr_t, matB: cusparseDnMatDescr_t, alg: cusparseSparseToDenseAlg_t, externalBuffer: *mut ::std::os::raw::c_void) -> cusparseStatus_t>,
    pub cusparseDenseToSparse_bufferSize: Option<unsafe extern "C" fn(handle: cusparseHandle_t, matA: cusparseConstDnMatDescr_t, matB: cusparseSpMatDescr_t, alg: cusparseDenseToSparseAlg_t, bufferSize: *mut usize) -> cusparseStatus_t>,
    pub cusparseDenseToSparse_analysis: Option<unsafe extern "C" fn(handle: cusparseHandle_t, matA: cusparseConstDnMatDescr_t, matB: cusparseSpMatDescr_t, alg: cusparseDenseToSparseAlg_t, externalBuffer: *mut ::std::os::raw::c_void) -> cusparseStatus_t>,
    pub cusparseDenseToSparse_convert: Option<unsafe extern "C" fn(handle: cusparseHandle_t, matA: cusparseConstDnMatDescr_t, matB: cusparseSpMatDescr_t, alg: cusparseDenseToSparseAlg_t, externalBuffer: *mut ::std::os::raw::c_void) -> cusparseStatus_t>,
    pub cusparseSpMV: Option<
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
    pub cusparseSpMV_bufferSize: Option<
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
    pub cusparseSpMV_preprocess: Option<
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
    pub cusparseSpSV_createDescr: Option<unsafe extern "C" fn(descr: *mut cusparseSpSVDescr_t) -> cusparseStatus_t>,
    pub cusparseSpSV_destroyDescr: Option<unsafe extern "C" fn(descr: cusparseSpSVDescr_t) -> cusparseStatus_t>,
    pub cusparseSpSV_bufferSize: Option<
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
    pub cusparseSpSV_analysis: Option<
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
    pub cusparseSpSV_solve: Option<
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
    pub cusparseSpSV_updateMatrix: Option<unsafe extern "C" fn(handle: cusparseHandle_t, spsvDescr: cusparseSpSVDescr_t, newValues: *mut ::std::os::raw::c_void, updatePart: cusparseSpSVUpdate_t) -> cusparseStatus_t>,
    pub cusparseSpSM_createDescr: Option<unsafe extern "C" fn(descr: *mut cusparseSpSMDescr_t) -> cusparseStatus_t>,
    pub cusparseSpSM_destroyDescr: Option<unsafe extern "C" fn(descr: cusparseSpSMDescr_t) -> cusparseStatus_t>,
    pub cusparseSpSM_bufferSize: Option<
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
    pub cusparseSpSM_analysis: Option<
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
    pub cusparseSpSM_solve: Option<
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
    pub cusparseSpSM_updateMatrix: Option<unsafe extern "C" fn(handle: cusparseHandle_t, spsmDescr: cusparseSpSMDescr_t, newValues: *mut ::std::os::raw::c_void, updatePart: cusparseSpSMUpdate_t) -> cusparseStatus_t>,
    pub cusparseSpMM_bufferSize: Option<
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
    pub cusparseSpMM_preprocess: Option<
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
    pub cusparseSpMM: Option<
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
    pub cusparseSpGEMM_createDescr: Option<unsafe extern "C" fn(descr: *mut cusparseSpGEMMDescr_t) -> cusparseStatus_t>,
    pub cusparseSpGEMM_destroyDescr: Option<unsafe extern "C" fn(descr: cusparseSpGEMMDescr_t) -> cusparseStatus_t>,
    pub cusparseSpGEMM_workEstimation: Option<
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
    pub cusparseSpGEMM_getNumProducts: Option<unsafe extern "C" fn(spgemmDescr: cusparseSpGEMMDescr_t, num_prods: *mut i64) -> cusparseStatus_t>,
    pub cusparseSpGEMM_estimateMemory: Option<
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
    pub cusparseSpGEMM_compute: Option<
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
    pub cusparseSpGEMM_copy: Option<
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
    pub cusparseSpGEMMreuse_workEstimation: Option<
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
    pub cusparseSpGEMMreuse_nnz: Option<
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
    pub cusparseSpGEMMreuse_copy: Option<
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
    pub cusparseSpGEMMreuse_compute: Option<
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
    pub cusparseSDDMM_bufferSize: Option<
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
    pub cusparseSDDMM_preprocess: Option<
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
    pub cusparseSDDMM: Option<
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
    pub cusparseSpMMOp_createPlan: Option<
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
    pub cusparseSpMMOp: Option<unsafe extern "C" fn(plan: cusparseSpMMOpPlan_t, externalBuffer: *mut ::std::os::raw::c_void) -> cusparseStatus_t>,
    pub cusparseSpMMOp_destroyPlan: Option<unsafe extern "C" fn(plan: cusparseSpMMOpPlan_t) -> cusparseStatus_t>,
}
#[cfg(feature = "runtime-link")]
unsafe impl Send for DynamicBindings {}
#[cfg(feature = "runtime-link")]
unsafe impl Sync for DynamicBindings {}
#[cfg(feature = "runtime-link")]
pub static DYNAMIC_BINDINGS: std::sync::OnceLock<Box<DynamicBindings>> = std::sync::OnceLock::new();
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCreate(handle: *mut cusparseHandle_t) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCreate {
        Some(____func) => unsafe { ____func(handle) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseCreate"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDestroy(handle: cusparseHandle_t) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDestroy {
        Some(____func) => unsafe { ____func(handle) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseDestroy"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseGetVersion(handle: cusparseHandle_t, version: *mut ::std::os::raw::c_int) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseGetVersion {
        Some(____func) => unsafe { ____func(handle, version) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseGetVersion"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseGetProperty(type_: libraryPropertyType, value: *mut ::std::os::raw::c_int) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseGetProperty {
        Some(____func) => unsafe { ____func(type_, value) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseGetProperty"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseGetErrorName(status: cusparseStatus_t) -> *const ::std::os::raw::c_char {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseGetErrorName {
        Some(____func) => unsafe { ____func(status) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseGetErrorName"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseGetErrorString(status: cusparseStatus_t) -> *const ::std::os::raw::c_char {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseGetErrorString {
        Some(____func) => unsafe { ____func(status) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseGetErrorString"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSetStream(handle: cusparseHandle_t, streamId: cudaStream_t) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSetStream {
        Some(____func) => unsafe { ____func(handle, streamId) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseSetStream"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseGetStream(handle: cusparseHandle_t, streamId: *mut cudaStream_t) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseGetStream {
        Some(____func) => unsafe { ____func(handle, streamId) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseGetStream"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseGetPointerMode(handle: cusparseHandle_t, mode: *mut cusparsePointerMode_t) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseGetPointerMode {
        Some(____func) => unsafe { ____func(handle, mode) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseGetPointerMode"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSetPointerMode(handle: cusparseHandle_t, mode: cusparsePointerMode_t) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSetPointerMode {
        Some(____func) => unsafe { ____func(handle, mode) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseSetPointerMode"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseLoggerSetCallback(callback: cusparseLoggerCallback_t) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseLoggerSetCallback {
        Some(____func) => unsafe { ____func(callback) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseLoggerSetCallback"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseLoggerSetFile(file: *mut FILE) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseLoggerSetFile {
        Some(____func) => unsafe { ____func(file) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseLoggerSetFile"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseLoggerOpenFile(logFile: *const ::std::os::raw::c_char) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseLoggerOpenFile {
        Some(____func) => unsafe { ____func(logFile) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseLoggerOpenFile"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseLoggerSetLevel(level: ::std::os::raw::c_int) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseLoggerSetLevel {
        Some(____func) => unsafe { ____func(level) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseLoggerSetLevel"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseLoggerSetMask(mask: ::std::os::raw::c_int) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseLoggerSetMask {
        Some(____func) => unsafe { ____func(mask) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseLoggerSetMask"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseLoggerForceDisable() -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseLoggerForceDisable {
        Some(____func) => unsafe { ____func() },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseLoggerForceDisable"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCreateMatDescr(descrA: *mut cusparseMatDescr_t) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCreateMatDescr {
        Some(____func) => unsafe { ____func(descrA) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseCreateMatDescr"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDestroyMatDescr(descrA: cusparseMatDescr_t) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDestroyMatDescr {
        Some(____func) => unsafe { ____func(descrA) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseDestroyMatDescr"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSetMatType(descrA: cusparseMatDescr_t, type_: cusparseMatrixType_t) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSetMatType {
        Some(____func) => unsafe { ____func(descrA, type_) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseSetMatType"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseGetMatType(descrA: cusparseMatDescr_t) -> cusparseMatrixType_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseGetMatType {
        Some(____func) => unsafe { ____func(descrA) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseGetMatType"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSetMatFillMode(descrA: cusparseMatDescr_t, fillMode: cusparseFillMode_t) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSetMatFillMode {
        Some(____func) => unsafe { ____func(descrA, fillMode) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseSetMatFillMode"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseGetMatFillMode(descrA: cusparseMatDescr_t) -> cusparseFillMode_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseGetMatFillMode {
        Some(____func) => unsafe { ____func(descrA) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseGetMatFillMode"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSetMatDiagType(descrA: cusparseMatDescr_t, diagType: cusparseDiagType_t) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSetMatDiagType {
        Some(____func) => unsafe { ____func(descrA, diagType) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseSetMatDiagType"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseGetMatDiagType(descrA: cusparseMatDescr_t) -> cusparseDiagType_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseGetMatDiagType {
        Some(____func) => unsafe { ____func(descrA) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseGetMatDiagType"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSetMatIndexBase(descrA: cusparseMatDescr_t, base: cusparseIndexBase_t) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSetMatIndexBase {
        Some(____func) => unsafe { ____func(descrA, base) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseSetMatIndexBase"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseGetMatIndexBase(descrA: cusparseMatDescr_t) -> cusparseIndexBase_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseGetMatIndexBase {
        Some(____func) => unsafe { ____func(descrA) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseGetMatIndexBase"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCreateCsric02Info(info: *mut csric02Info_t) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCreateCsric02Info {
        Some(____func) => unsafe { ____func(info) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseCreateCsric02Info"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDestroyCsric02Info(info: csric02Info_t) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDestroyCsric02Info {
        Some(____func) => unsafe { ____func(info) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseDestroyCsric02Info"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCreateBsric02Info(info: *mut bsric02Info_t) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCreateBsric02Info {
        Some(____func) => unsafe { ____func(info) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseCreateBsric02Info"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDestroyBsric02Info(info: bsric02Info_t) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDestroyBsric02Info {
        Some(____func) => unsafe { ____func(info) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseDestroyBsric02Info"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCreateCsrilu02Info(info: *mut csrilu02Info_t) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCreateCsrilu02Info {
        Some(____func) => unsafe { ____func(info) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseCreateCsrilu02Info"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDestroyCsrilu02Info(info: csrilu02Info_t) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDestroyCsrilu02Info {
        Some(____func) => unsafe { ____func(info) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseDestroyCsrilu02Info"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCreateBsrilu02Info(info: *mut bsrilu02Info_t) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCreateBsrilu02Info {
        Some(____func) => unsafe { ____func(info) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseCreateBsrilu02Info"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDestroyBsrilu02Info(info: bsrilu02Info_t) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDestroyBsrilu02Info {
        Some(____func) => unsafe { ____func(info) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseDestroyBsrilu02Info"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCreateBsrsv2Info(info: *mut bsrsv2Info_t) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCreateBsrsv2Info {
        Some(____func) => unsafe { ____func(info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseCreateBsrsv2Info"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDestroyBsrsv2Info(info: bsrsv2Info_t) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDestroyBsrsv2Info {
        Some(____func) => unsafe { ____func(info) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseDestroyBsrsv2Info"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCreateBsrsm2Info(info: *mut bsrsm2Info_t) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCreateBsrsm2Info {
        Some(____func) => unsafe { ____func(info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseCreateBsrsm2Info"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDestroyBsrsm2Info(info: bsrsm2Info_t) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDestroyBsrsm2Info {
        Some(____func) => unsafe { ____func(info) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseDestroyBsrsm2Info"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCreateCsru2csrInfo(info: *mut csru2csrInfo_t) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCreateCsru2csrInfo {
        Some(____func) => unsafe { ____func(info) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseCreateCsru2csrInfo"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDestroyCsru2csrInfo(info: csru2csrInfo_t) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDestroyCsru2csrInfo {
        Some(____func) => unsafe { ____func(info) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseDestroyCsru2csrInfo"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCreateColorInfo(info: *mut cusparseColorInfo_t) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCreateColorInfo {
        Some(____func) => unsafe { ____func(info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseCreateColorInfo"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDestroyColorInfo(info: cusparseColorInfo_t) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDestroyColorInfo {
        Some(____func) => unsafe { ____func(info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseDestroyColorInfo"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCreatePruneInfo(info: *mut pruneInfo_t) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCreatePruneInfo {
        Some(____func) => unsafe { ____func(info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseCreatePruneInfo"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDestroyPruneInfo(info: pruneInfo_t) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDestroyPruneInfo {
        Some(____func) => unsafe { ____func(info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseDestroyPruneInfo"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSgemvi(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSgemvi {
        Some(____func) => unsafe { ____func(handle, transA, m, n, alpha, A, lda, nnz, xVal, xInd, beta, y, idxBase, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseSgemvi"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSgemvi_bufferSize(handle: cusparseHandle_t, transA: cusparseOperation_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, nnz: ::std::os::raw::c_int, pBufferSize: *mut ::std::os::raw::c_int) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSgemvi_bufferSize {
        Some(____func) => unsafe { ____func(handle, transA, m, n, nnz, pBufferSize) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseSgemvi_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDgemvi(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDgemvi {
        Some(____func) => unsafe { ____func(handle, transA, m, n, alpha, A, lda, nnz, xVal, xInd, beta, y, idxBase, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseDgemvi"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDgemvi_bufferSize(handle: cusparseHandle_t, transA: cusparseOperation_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, nnz: ::std::os::raw::c_int, pBufferSize: *mut ::std::os::raw::c_int) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDgemvi_bufferSize {
        Some(____func) => unsafe { ____func(handle, transA, m, n, nnz, pBufferSize) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseDgemvi_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCgemvi(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCgemvi {
        Some(____func) => unsafe { ____func(handle, transA, m, n, alpha, A, lda, nnz, xVal, xInd, beta, y, idxBase, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseCgemvi"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCgemvi_bufferSize(handle: cusparseHandle_t, transA: cusparseOperation_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, nnz: ::std::os::raw::c_int, pBufferSize: *mut ::std::os::raw::c_int) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCgemvi_bufferSize {
        Some(____func) => unsafe { ____func(handle, transA, m, n, nnz, pBufferSize) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseCgemvi_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseZgemvi(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseZgemvi {
        Some(____func) => unsafe { ____func(handle, transA, m, n, alpha, A, lda, nnz, xVal, xInd, beta, y, idxBase, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseZgemvi"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseZgemvi_bufferSize(handle: cusparseHandle_t, transA: cusparseOperation_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, nnz: ::std::os::raw::c_int, pBufferSize: *mut ::std::os::raw::c_int) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseZgemvi_bufferSize {
        Some(____func) => unsafe { ____func(handle, transA, m, n, nnz, pBufferSize) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseZgemvi_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSbsrmv(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSbsrmv {
        Some(____func) => unsafe { ____func(handle, dirA, transA, mb, nb, nnzb, alpha, descrA, bsrSortedValA, bsrSortedRowPtrA, bsrSortedColIndA, blockDim, x, beta, y) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseSbsrmv"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDbsrmv(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDbsrmv {
        Some(____func) => unsafe { ____func(handle, dirA, transA, mb, nb, nnzb, alpha, descrA, bsrSortedValA, bsrSortedRowPtrA, bsrSortedColIndA, blockDim, x, beta, y) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseDbsrmv"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCbsrmv(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCbsrmv {
        Some(____func) => unsafe { ____func(handle, dirA, transA, mb, nb, nnzb, alpha, descrA, bsrSortedValA, bsrSortedRowPtrA, bsrSortedColIndA, blockDim, x, beta, y) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseCbsrmv"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseZbsrmv(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseZbsrmv {
        Some(____func) => unsafe { ____func(handle, dirA, transA, mb, nb, nnzb, alpha, descrA, bsrSortedValA, bsrSortedRowPtrA, bsrSortedColIndA, blockDim, x, beta, y) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseZbsrmv"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSbsrxmv(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSbsrxmv {
        Some(____func) => unsafe { ____func(handle, dirA, transA, sizeOfMask, mb, nb, nnzb, alpha, descrA, bsrSortedValA, bsrSortedMaskPtrA, bsrSortedRowPtrA, bsrSortedEndPtrA, bsrSortedColIndA, blockDim, x, beta, y) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseSbsrxmv"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDbsrxmv(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDbsrxmv {
        Some(____func) => unsafe { ____func(handle, dirA, transA, sizeOfMask, mb, nb, nnzb, alpha, descrA, bsrSortedValA, bsrSortedMaskPtrA, bsrSortedRowPtrA, bsrSortedEndPtrA, bsrSortedColIndA, blockDim, x, beta, y) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseDbsrxmv"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCbsrxmv(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCbsrxmv {
        Some(____func) => unsafe { ____func(handle, dirA, transA, sizeOfMask, mb, nb, nnzb, alpha, descrA, bsrSortedValA, bsrSortedMaskPtrA, bsrSortedRowPtrA, bsrSortedEndPtrA, bsrSortedColIndA, blockDim, x, beta, y) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseCbsrxmv"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseZbsrxmv(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseZbsrxmv {
        Some(____func) => unsafe { ____func(handle, dirA, transA, sizeOfMask, mb, nb, nnzb, alpha, descrA, bsrSortedValA, bsrSortedMaskPtrA, bsrSortedRowPtrA, bsrSortedEndPtrA, bsrSortedColIndA, blockDim, x, beta, y) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseZbsrxmv"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseXbsrsv2_zeroPivot(handle: cusparseHandle_t, info: bsrsv2Info_t, position: *mut ::std::os::raw::c_int) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseXbsrsv2_zeroPivot {
        Some(____func) => unsafe { ____func(handle, info, position) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseXbsrsv2_zeroPivot"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSbsrsv2_bufferSize(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSbsrsv2_bufferSize {
        Some(____func) => unsafe { ____func(handle, dirA, transA, mb, nnzb, descrA, bsrSortedValA, bsrSortedRowPtrA, bsrSortedColIndA, blockDim, info, pBufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseSbsrsv2_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDbsrsv2_bufferSize(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDbsrsv2_bufferSize {
        Some(____func) => unsafe { ____func(handle, dirA, transA, mb, nnzb, descrA, bsrSortedValA, bsrSortedRowPtrA, bsrSortedColIndA, blockDim, info, pBufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseDbsrsv2_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCbsrsv2_bufferSize(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCbsrsv2_bufferSize {
        Some(____func) => unsafe { ____func(handle, dirA, transA, mb, nnzb, descrA, bsrSortedValA, bsrSortedRowPtrA, bsrSortedColIndA, blockDim, info, pBufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseCbsrsv2_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseZbsrsv2_bufferSize(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseZbsrsv2_bufferSize {
        Some(____func) => unsafe { ____func(handle, dirA, transA, mb, nnzb, descrA, bsrSortedValA, bsrSortedRowPtrA, bsrSortedColIndA, blockDim, info, pBufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseZbsrsv2_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSbsrsv2_bufferSizeExt(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSbsrsv2_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, dirA, transA, mb, nnzb, descrA, bsrSortedValA, bsrSortedRowPtrA, bsrSortedColIndA, blockSize, info, pBufferSize) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseSbsrsv2_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDbsrsv2_bufferSizeExt(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDbsrsv2_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, dirA, transA, mb, nnzb, descrA, bsrSortedValA, bsrSortedRowPtrA, bsrSortedColIndA, blockSize, info, pBufferSize) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseDbsrsv2_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCbsrsv2_bufferSizeExt(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCbsrsv2_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, dirA, transA, mb, nnzb, descrA, bsrSortedValA, bsrSortedRowPtrA, bsrSortedColIndA, blockSize, info, pBufferSize) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseCbsrsv2_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseZbsrsv2_bufferSizeExt(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseZbsrsv2_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, dirA, transA, mb, nnzb, descrA, bsrSortedValA, bsrSortedRowPtrA, bsrSortedColIndA, blockSize, info, pBufferSize) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseZbsrsv2_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSbsrsv2_analysis(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSbsrsv2_analysis {
        Some(____func) => unsafe { ____func(handle, dirA, transA, mb, nnzb, descrA, bsrSortedValA, bsrSortedRowPtrA, bsrSortedColIndA, blockDim, info, policy, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseSbsrsv2_analysis"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDbsrsv2_analysis(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDbsrsv2_analysis {
        Some(____func) => unsafe { ____func(handle, dirA, transA, mb, nnzb, descrA, bsrSortedValA, bsrSortedRowPtrA, bsrSortedColIndA, blockDim, info, policy, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseDbsrsv2_analysis"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCbsrsv2_analysis(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCbsrsv2_analysis {
        Some(____func) => unsafe { ____func(handle, dirA, transA, mb, nnzb, descrA, bsrSortedValA, bsrSortedRowPtrA, bsrSortedColIndA, blockDim, info, policy, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseCbsrsv2_analysis"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseZbsrsv2_analysis(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseZbsrsv2_analysis {
        Some(____func) => unsafe { ____func(handle, dirA, transA, mb, nnzb, descrA, bsrSortedValA, bsrSortedRowPtrA, bsrSortedColIndA, blockDim, info, policy, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseZbsrsv2_analysis"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSbsrsv2_solve(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSbsrsv2_solve {
        Some(____func) => unsafe { ____func(handle, dirA, transA, mb, nnzb, alpha, descrA, bsrSortedValA, bsrSortedRowPtrA, bsrSortedColIndA, blockDim, info, f, x, policy, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseSbsrsv2_solve"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDbsrsv2_solve(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDbsrsv2_solve {
        Some(____func) => unsafe { ____func(handle, dirA, transA, mb, nnzb, alpha, descrA, bsrSortedValA, bsrSortedRowPtrA, bsrSortedColIndA, blockDim, info, f, x, policy, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseDbsrsv2_solve"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCbsrsv2_solve(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCbsrsv2_solve {
        Some(____func) => unsafe { ____func(handle, dirA, transA, mb, nnzb, alpha, descrA, bsrSortedValA, bsrSortedRowPtrA, bsrSortedColIndA, blockDim, info, f, x, policy, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseCbsrsv2_solve"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseZbsrsv2_solve(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseZbsrsv2_solve {
        Some(____func) => unsafe { ____func(handle, dirA, transA, mb, nnzb, alpha, descrA, bsrSortedValA, bsrSortedRowPtrA, bsrSortedColIndA, blockDim, info, f, x, policy, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseZbsrsv2_solve"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSbsrmm(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSbsrmm {
        Some(____func) => unsafe { ____func(handle, dirA, transA, transB, mb, n, kb, nnzb, alpha, descrA, bsrSortedValA, bsrSortedRowPtrA, bsrSortedColIndA, blockSize, B, ldb, beta, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseSbsrmm"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDbsrmm(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDbsrmm {
        Some(____func) => unsafe { ____func(handle, dirA, transA, transB, mb, n, kb, nnzb, alpha, descrA, bsrSortedValA, bsrSortedRowPtrA, bsrSortedColIndA, blockSize, B, ldb, beta, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseDbsrmm"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCbsrmm(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCbsrmm {
        Some(____func) => unsafe { ____func(handle, dirA, transA, transB, mb, n, kb, nnzb, alpha, descrA, bsrSortedValA, bsrSortedRowPtrA, bsrSortedColIndA, blockSize, B, ldb, beta, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseCbsrmm"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseZbsrmm(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseZbsrmm {
        Some(____func) => unsafe { ____func(handle, dirA, transA, transB, mb, n, kb, nnzb, alpha, descrA, bsrSortedValA, bsrSortedRowPtrA, bsrSortedColIndA, blockSize, B, ldb, beta, C, ldc) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseZbsrmm"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseXbsrsm2_zeroPivot(handle: cusparseHandle_t, info: bsrsm2Info_t, position: *mut ::std::os::raw::c_int) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseXbsrsm2_zeroPivot {
        Some(____func) => unsafe { ____func(handle, info, position) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseXbsrsm2_zeroPivot"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSbsrsm2_bufferSize(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSbsrsm2_bufferSize {
        Some(____func) => unsafe { ____func(handle, dirA, transA, transXY, mb, n, nnzb, descrA, bsrSortedVal, bsrSortedRowPtr, bsrSortedColInd, blockSize, info, pBufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseSbsrsm2_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDbsrsm2_bufferSize(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDbsrsm2_bufferSize {
        Some(____func) => unsafe { ____func(handle, dirA, transA, transXY, mb, n, nnzb, descrA, bsrSortedVal, bsrSortedRowPtr, bsrSortedColInd, blockSize, info, pBufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseDbsrsm2_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCbsrsm2_bufferSize(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCbsrsm2_bufferSize {
        Some(____func) => unsafe { ____func(handle, dirA, transA, transXY, mb, n, nnzb, descrA, bsrSortedVal, bsrSortedRowPtr, bsrSortedColInd, blockSize, info, pBufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseCbsrsm2_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseZbsrsm2_bufferSize(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseZbsrsm2_bufferSize {
        Some(____func) => unsafe { ____func(handle, dirA, transA, transXY, mb, n, nnzb, descrA, bsrSortedVal, bsrSortedRowPtr, bsrSortedColInd, blockSize, info, pBufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseZbsrsm2_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSbsrsm2_bufferSizeExt(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSbsrsm2_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, dirA, transA, transB, mb, n, nnzb, descrA, bsrSortedVal, bsrSortedRowPtr, bsrSortedColInd, blockSize, info, pBufferSize) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseSbsrsm2_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDbsrsm2_bufferSizeExt(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDbsrsm2_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, dirA, transA, transB, mb, n, nnzb, descrA, bsrSortedVal, bsrSortedRowPtr, bsrSortedColInd, blockSize, info, pBufferSize) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseDbsrsm2_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCbsrsm2_bufferSizeExt(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCbsrsm2_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, dirA, transA, transB, mb, n, nnzb, descrA, bsrSortedVal, bsrSortedRowPtr, bsrSortedColInd, blockSize, info, pBufferSize) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseCbsrsm2_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseZbsrsm2_bufferSizeExt(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseZbsrsm2_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, dirA, transA, transB, mb, n, nnzb, descrA, bsrSortedVal, bsrSortedRowPtr, bsrSortedColInd, blockSize, info, pBufferSize) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseZbsrsm2_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSbsrsm2_analysis(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSbsrsm2_analysis {
        Some(____func) => unsafe { ____func(handle, dirA, transA, transXY, mb, n, nnzb, descrA, bsrSortedVal, bsrSortedRowPtr, bsrSortedColInd, blockSize, info, policy, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseSbsrsm2_analysis"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDbsrsm2_analysis(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDbsrsm2_analysis {
        Some(____func) => unsafe { ____func(handle, dirA, transA, transXY, mb, n, nnzb, descrA, bsrSortedVal, bsrSortedRowPtr, bsrSortedColInd, blockSize, info, policy, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseDbsrsm2_analysis"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCbsrsm2_analysis(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCbsrsm2_analysis {
        Some(____func) => unsafe { ____func(handle, dirA, transA, transXY, mb, n, nnzb, descrA, bsrSortedVal, bsrSortedRowPtr, bsrSortedColInd, blockSize, info, policy, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseCbsrsm2_analysis"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseZbsrsm2_analysis(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseZbsrsm2_analysis {
        Some(____func) => unsafe { ____func(handle, dirA, transA, transXY, mb, n, nnzb, descrA, bsrSortedVal, bsrSortedRowPtr, bsrSortedColInd, blockSize, info, policy, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseZbsrsm2_analysis"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSbsrsm2_solve(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSbsrsm2_solve {
        Some(____func) => unsafe { ____func(handle, dirA, transA, transXY, mb, n, nnzb, alpha, descrA, bsrSortedVal, bsrSortedRowPtr, bsrSortedColInd, blockSize, info, B, ldb, X, ldx, policy, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseSbsrsm2_solve"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDbsrsm2_solve(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDbsrsm2_solve {
        Some(____func) => unsafe { ____func(handle, dirA, transA, transXY, mb, n, nnzb, alpha, descrA, bsrSortedVal, bsrSortedRowPtr, bsrSortedColInd, blockSize, info, B, ldb, X, ldx, policy, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseDbsrsm2_solve"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCbsrsm2_solve(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCbsrsm2_solve {
        Some(____func) => unsafe { ____func(handle, dirA, transA, transXY, mb, n, nnzb, alpha, descrA, bsrSortedVal, bsrSortedRowPtr, bsrSortedColInd, blockSize, info, B, ldb, X, ldx, policy, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseCbsrsm2_solve"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseZbsrsm2_solve(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseZbsrsm2_solve {
        Some(____func) => unsafe { ____func(handle, dirA, transA, transXY, mb, n, nnzb, alpha, descrA, bsrSortedVal, bsrSortedRowPtr, bsrSortedColInd, blockSize, info, B, ldb, X, ldx, policy, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseZbsrsm2_solve"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseScsrilu02_numericBoost(handle: cusparseHandle_t, info: csrilu02Info_t, enable_boost: ::std::os::raw::c_int, tol: *mut f64, boost_val: *mut f32) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseScsrilu02_numericBoost {
        Some(____func) => unsafe { ____func(handle, info, enable_boost, tol, boost_val) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseScsrilu02_numericBoost"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDcsrilu02_numericBoost(handle: cusparseHandle_t, info: csrilu02Info_t, enable_boost: ::std::os::raw::c_int, tol: *mut f64, boost_val: *mut f64) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDcsrilu02_numericBoost {
        Some(____func) => unsafe { ____func(handle, info, enable_boost, tol, boost_val) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseDcsrilu02_numericBoost"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCcsrilu02_numericBoost(handle: cusparseHandle_t, info: csrilu02Info_t, enable_boost: ::std::os::raw::c_int, tol: *mut f64, boost_val: *mut cuComplex) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCcsrilu02_numericBoost {
        Some(____func) => unsafe { ____func(handle, info, enable_boost, tol, boost_val) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseCcsrilu02_numericBoost"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseZcsrilu02_numericBoost(handle: cusparseHandle_t, info: csrilu02Info_t, enable_boost: ::std::os::raw::c_int, tol: *mut f64, boost_val: *mut cuDoubleComplex) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseZcsrilu02_numericBoost {
        Some(____func) => unsafe { ____func(handle, info, enable_boost, tol, boost_val) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseZcsrilu02_numericBoost"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseXcsrilu02_zeroPivot(handle: cusparseHandle_t, info: csrilu02Info_t, position: *mut ::std::os::raw::c_int) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseXcsrilu02_zeroPivot {
        Some(____func) => unsafe { ____func(handle, info, position) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseXcsrilu02_zeroPivot"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseScsrilu02_bufferSize(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedValA: *mut f32,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    info: csrilu02Info_t,
    pBufferSizeInBytes: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseScsrilu02_bufferSize {
        Some(____func) => unsafe { ____func(handle, m, nnz, descrA, csrSortedValA, csrSortedRowPtrA, csrSortedColIndA, info, pBufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseScsrilu02_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDcsrilu02_bufferSize(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedValA: *mut f64,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    info: csrilu02Info_t,
    pBufferSizeInBytes: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDcsrilu02_bufferSize {
        Some(____func) => unsafe { ____func(handle, m, nnz, descrA, csrSortedValA, csrSortedRowPtrA, csrSortedColIndA, info, pBufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseDcsrilu02_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCcsrilu02_bufferSize(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedValA: *mut cuComplex,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    info: csrilu02Info_t,
    pBufferSizeInBytes: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCcsrilu02_bufferSize {
        Some(____func) => unsafe { ____func(handle, m, nnz, descrA, csrSortedValA, csrSortedRowPtrA, csrSortedColIndA, info, pBufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseCcsrilu02_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseZcsrilu02_bufferSize(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedValA: *mut cuDoubleComplex,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    info: csrilu02Info_t,
    pBufferSizeInBytes: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseZcsrilu02_bufferSize {
        Some(____func) => unsafe { ____func(handle, m, nnz, descrA, csrSortedValA, csrSortedRowPtrA, csrSortedColIndA, info, pBufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseZcsrilu02_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseScsrilu02_bufferSizeExt(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedVal: *mut f32,
    csrSortedRowPtr: *const ::std::os::raw::c_int,
    csrSortedColInd: *const ::std::os::raw::c_int,
    info: csrilu02Info_t,
    pBufferSize: *mut usize,
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseScsrilu02_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, m, nnz, descrA, csrSortedVal, csrSortedRowPtr, csrSortedColInd, info, pBufferSize) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseScsrilu02_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDcsrilu02_bufferSizeExt(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedVal: *mut f64,
    csrSortedRowPtr: *const ::std::os::raw::c_int,
    csrSortedColInd: *const ::std::os::raw::c_int,
    info: csrilu02Info_t,
    pBufferSize: *mut usize,
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDcsrilu02_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, m, nnz, descrA, csrSortedVal, csrSortedRowPtr, csrSortedColInd, info, pBufferSize) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseDcsrilu02_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCcsrilu02_bufferSizeExt(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedVal: *mut cuComplex,
    csrSortedRowPtr: *const ::std::os::raw::c_int,
    csrSortedColInd: *const ::std::os::raw::c_int,
    info: csrilu02Info_t,
    pBufferSize: *mut usize,
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCcsrilu02_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, m, nnz, descrA, csrSortedVal, csrSortedRowPtr, csrSortedColInd, info, pBufferSize) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseCcsrilu02_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseZcsrilu02_bufferSizeExt(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedVal: *mut cuDoubleComplex,
    csrSortedRowPtr: *const ::std::os::raw::c_int,
    csrSortedColInd: *const ::std::os::raw::c_int,
    info: csrilu02Info_t,
    pBufferSize: *mut usize,
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseZcsrilu02_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, m, nnz, descrA, csrSortedVal, csrSortedRowPtr, csrSortedColInd, info, pBufferSize) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseZcsrilu02_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseScsrilu02_analysis(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseScsrilu02_analysis {
        Some(____func) => unsafe { ____func(handle, m, nnz, descrA, csrSortedValA, csrSortedRowPtrA, csrSortedColIndA, info, policy, pBuffer) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseScsrilu02_analysis"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDcsrilu02_analysis(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDcsrilu02_analysis {
        Some(____func) => unsafe { ____func(handle, m, nnz, descrA, csrSortedValA, csrSortedRowPtrA, csrSortedColIndA, info, policy, pBuffer) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseDcsrilu02_analysis"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCcsrilu02_analysis(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCcsrilu02_analysis {
        Some(____func) => unsafe { ____func(handle, m, nnz, descrA, csrSortedValA, csrSortedRowPtrA, csrSortedColIndA, info, policy, pBuffer) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseCcsrilu02_analysis"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseZcsrilu02_analysis(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseZcsrilu02_analysis {
        Some(____func) => unsafe { ____func(handle, m, nnz, descrA, csrSortedValA, csrSortedRowPtrA, csrSortedColIndA, info, policy, pBuffer) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseZcsrilu02_analysis"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseScsrilu02(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseScsrilu02 {
        Some(____func) => unsafe { ____func(handle, m, nnz, descrA, csrSortedValA_valM, csrSortedRowPtrA, csrSortedColIndA, info, policy, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseScsrilu02"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDcsrilu02(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDcsrilu02 {
        Some(____func) => unsafe { ____func(handle, m, nnz, descrA, csrSortedValA_valM, csrSortedRowPtrA, csrSortedColIndA, info, policy, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseDcsrilu02"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCcsrilu02(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCcsrilu02 {
        Some(____func) => unsafe { ____func(handle, m, nnz, descrA, csrSortedValA_valM, csrSortedRowPtrA, csrSortedColIndA, info, policy, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseCcsrilu02"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseZcsrilu02(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseZcsrilu02 {
        Some(____func) => unsafe { ____func(handle, m, nnz, descrA, csrSortedValA_valM, csrSortedRowPtrA, csrSortedColIndA, info, policy, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseZcsrilu02"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSbsrilu02_numericBoost(handle: cusparseHandle_t, info: bsrilu02Info_t, enable_boost: ::std::os::raw::c_int, tol: *mut f64, boost_val: *mut f32) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSbsrilu02_numericBoost {
        Some(____func) => unsafe { ____func(handle, info, enable_boost, tol, boost_val) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseSbsrilu02_numericBoost"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDbsrilu02_numericBoost(handle: cusparseHandle_t, info: bsrilu02Info_t, enable_boost: ::std::os::raw::c_int, tol: *mut f64, boost_val: *mut f64) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDbsrilu02_numericBoost {
        Some(____func) => unsafe { ____func(handle, info, enable_boost, tol, boost_val) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseDbsrilu02_numericBoost"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCbsrilu02_numericBoost(handle: cusparseHandle_t, info: bsrilu02Info_t, enable_boost: ::std::os::raw::c_int, tol: *mut f64, boost_val: *mut cuComplex) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCbsrilu02_numericBoost {
        Some(____func) => unsafe { ____func(handle, info, enable_boost, tol, boost_val) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseCbsrilu02_numericBoost"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseZbsrilu02_numericBoost(handle: cusparseHandle_t, info: bsrilu02Info_t, enable_boost: ::std::os::raw::c_int, tol: *mut f64, boost_val: *mut cuDoubleComplex) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseZbsrilu02_numericBoost {
        Some(____func) => unsafe { ____func(handle, info, enable_boost, tol, boost_val) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseZbsrilu02_numericBoost"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseXbsrilu02_zeroPivot(handle: cusparseHandle_t, info: bsrilu02Info_t, position: *mut ::std::os::raw::c_int) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseXbsrilu02_zeroPivot {
        Some(____func) => unsafe { ____func(handle, info, position) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseXbsrilu02_zeroPivot"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSbsrilu02_bufferSize(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSbsrilu02_bufferSize {
        Some(____func) => unsafe { ____func(handle, dirA, mb, nnzb, descrA, bsrSortedVal, bsrSortedRowPtr, bsrSortedColInd, blockDim, info, pBufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseSbsrilu02_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDbsrilu02_bufferSize(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDbsrilu02_bufferSize {
        Some(____func) => unsafe { ____func(handle, dirA, mb, nnzb, descrA, bsrSortedVal, bsrSortedRowPtr, bsrSortedColInd, blockDim, info, pBufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseDbsrilu02_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCbsrilu02_bufferSize(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCbsrilu02_bufferSize {
        Some(____func) => unsafe { ____func(handle, dirA, mb, nnzb, descrA, bsrSortedVal, bsrSortedRowPtr, bsrSortedColInd, blockDim, info, pBufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseCbsrilu02_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseZbsrilu02_bufferSize(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseZbsrilu02_bufferSize {
        Some(____func) => unsafe { ____func(handle, dirA, mb, nnzb, descrA, bsrSortedVal, bsrSortedRowPtr, bsrSortedColInd, blockDim, info, pBufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseZbsrilu02_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSbsrilu02_bufferSizeExt(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSbsrilu02_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, dirA, mb, nnzb, descrA, bsrSortedVal, bsrSortedRowPtr, bsrSortedColInd, blockSize, info, pBufferSize) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseSbsrilu02_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDbsrilu02_bufferSizeExt(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDbsrilu02_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, dirA, mb, nnzb, descrA, bsrSortedVal, bsrSortedRowPtr, bsrSortedColInd, blockSize, info, pBufferSize) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseDbsrilu02_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCbsrilu02_bufferSizeExt(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCbsrilu02_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, dirA, mb, nnzb, descrA, bsrSortedVal, bsrSortedRowPtr, bsrSortedColInd, blockSize, info, pBufferSize) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseCbsrilu02_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseZbsrilu02_bufferSizeExt(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseZbsrilu02_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, dirA, mb, nnzb, descrA, bsrSortedVal, bsrSortedRowPtr, bsrSortedColInd, blockSize, info, pBufferSize) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseZbsrilu02_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSbsrilu02_analysis(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSbsrilu02_analysis {
        Some(____func) => unsafe { ____func(handle, dirA, mb, nnzb, descrA, bsrSortedVal, bsrSortedRowPtr, bsrSortedColInd, blockDim, info, policy, pBuffer) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseSbsrilu02_analysis"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDbsrilu02_analysis(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDbsrilu02_analysis {
        Some(____func) => unsafe { ____func(handle, dirA, mb, nnzb, descrA, bsrSortedVal, bsrSortedRowPtr, bsrSortedColInd, blockDim, info, policy, pBuffer) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseDbsrilu02_analysis"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCbsrilu02_analysis(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCbsrilu02_analysis {
        Some(____func) => unsafe { ____func(handle, dirA, mb, nnzb, descrA, bsrSortedVal, bsrSortedRowPtr, bsrSortedColInd, blockDim, info, policy, pBuffer) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseCbsrilu02_analysis"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseZbsrilu02_analysis(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseZbsrilu02_analysis {
        Some(____func) => unsafe { ____func(handle, dirA, mb, nnzb, descrA, bsrSortedVal, bsrSortedRowPtr, bsrSortedColInd, blockDim, info, policy, pBuffer) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseZbsrilu02_analysis"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSbsrilu02(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSbsrilu02 {
        Some(____func) => unsafe { ____func(handle, dirA, mb, nnzb, descrA, bsrSortedVal, bsrSortedRowPtr, bsrSortedColInd, blockDim, info, policy, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseSbsrilu02"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDbsrilu02(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDbsrilu02 {
        Some(____func) => unsafe { ____func(handle, dirA, mb, nnzb, descrA, bsrSortedVal, bsrSortedRowPtr, bsrSortedColInd, blockDim, info, policy, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseDbsrilu02"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCbsrilu02(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCbsrilu02 {
        Some(____func) => unsafe { ____func(handle, dirA, mb, nnzb, descrA, bsrSortedVal, bsrSortedRowPtr, bsrSortedColInd, blockDim, info, policy, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseCbsrilu02"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseZbsrilu02(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseZbsrilu02 {
        Some(____func) => unsafe { ____func(handle, dirA, mb, nnzb, descrA, bsrSortedVal, bsrSortedRowPtr, bsrSortedColInd, blockDim, info, policy, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseZbsrilu02"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseXcsric02_zeroPivot(handle: cusparseHandle_t, info: csric02Info_t, position: *mut ::std::os::raw::c_int) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseXcsric02_zeroPivot {
        Some(____func) => unsafe { ____func(handle, info, position) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseXcsric02_zeroPivot"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseScsric02_bufferSize(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedValA: *mut f32,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    info: csric02Info_t,
    pBufferSizeInBytes: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseScsric02_bufferSize {
        Some(____func) => unsafe { ____func(handle, m, nnz, descrA, csrSortedValA, csrSortedRowPtrA, csrSortedColIndA, info, pBufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseScsric02_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDcsric02_bufferSize(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedValA: *mut f64,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    info: csric02Info_t,
    pBufferSizeInBytes: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDcsric02_bufferSize {
        Some(____func) => unsafe { ____func(handle, m, nnz, descrA, csrSortedValA, csrSortedRowPtrA, csrSortedColIndA, info, pBufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseDcsric02_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCcsric02_bufferSize(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedValA: *mut cuComplex,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    info: csric02Info_t,
    pBufferSizeInBytes: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCcsric02_bufferSize {
        Some(____func) => unsafe { ____func(handle, m, nnz, descrA, csrSortedValA, csrSortedRowPtrA, csrSortedColIndA, info, pBufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseCcsric02_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseZcsric02_bufferSize(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedValA: *mut cuDoubleComplex,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    info: csric02Info_t,
    pBufferSizeInBytes: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseZcsric02_bufferSize {
        Some(____func) => unsafe { ____func(handle, m, nnz, descrA, csrSortedValA, csrSortedRowPtrA, csrSortedColIndA, info, pBufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseZcsric02_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseScsric02_bufferSizeExt(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedVal: *mut f32,
    csrSortedRowPtr: *const ::std::os::raw::c_int,
    csrSortedColInd: *const ::std::os::raw::c_int,
    info: csric02Info_t,
    pBufferSize: *mut usize,
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseScsric02_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, m, nnz, descrA, csrSortedVal, csrSortedRowPtr, csrSortedColInd, info, pBufferSize) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseScsric02_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDcsric02_bufferSizeExt(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedVal: *mut f64,
    csrSortedRowPtr: *const ::std::os::raw::c_int,
    csrSortedColInd: *const ::std::os::raw::c_int,
    info: csric02Info_t,
    pBufferSize: *mut usize,
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDcsric02_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, m, nnz, descrA, csrSortedVal, csrSortedRowPtr, csrSortedColInd, info, pBufferSize) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseDcsric02_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCcsric02_bufferSizeExt(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedVal: *mut cuComplex,
    csrSortedRowPtr: *const ::std::os::raw::c_int,
    csrSortedColInd: *const ::std::os::raw::c_int,
    info: csric02Info_t,
    pBufferSize: *mut usize,
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCcsric02_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, m, nnz, descrA, csrSortedVal, csrSortedRowPtr, csrSortedColInd, info, pBufferSize) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseCcsric02_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseZcsric02_bufferSizeExt(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedVal: *mut cuDoubleComplex,
    csrSortedRowPtr: *const ::std::os::raw::c_int,
    csrSortedColInd: *const ::std::os::raw::c_int,
    info: csric02Info_t,
    pBufferSize: *mut usize,
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseZcsric02_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, m, nnz, descrA, csrSortedVal, csrSortedRowPtr, csrSortedColInd, info, pBufferSize) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseZcsric02_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseScsric02_analysis(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseScsric02_analysis {
        Some(____func) => unsafe { ____func(handle, m, nnz, descrA, csrSortedValA, csrSortedRowPtrA, csrSortedColIndA, info, policy, pBuffer) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseScsric02_analysis"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDcsric02_analysis(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDcsric02_analysis {
        Some(____func) => unsafe { ____func(handle, m, nnz, descrA, csrSortedValA, csrSortedRowPtrA, csrSortedColIndA, info, policy, pBuffer) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseDcsric02_analysis"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCcsric02_analysis(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCcsric02_analysis {
        Some(____func) => unsafe { ____func(handle, m, nnz, descrA, csrSortedValA, csrSortedRowPtrA, csrSortedColIndA, info, policy, pBuffer) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseCcsric02_analysis"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseZcsric02_analysis(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseZcsric02_analysis {
        Some(____func) => unsafe { ____func(handle, m, nnz, descrA, csrSortedValA, csrSortedRowPtrA, csrSortedColIndA, info, policy, pBuffer) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseZcsric02_analysis"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseScsric02(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseScsric02 {
        Some(____func) => unsafe { ____func(handle, m, nnz, descrA, csrSortedValA_valM, csrSortedRowPtrA, csrSortedColIndA, info, policy, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseScsric02"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDcsric02(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDcsric02 {
        Some(____func) => unsafe { ____func(handle, m, nnz, descrA, csrSortedValA_valM, csrSortedRowPtrA, csrSortedColIndA, info, policy, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseDcsric02"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCcsric02(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCcsric02 {
        Some(____func) => unsafe { ____func(handle, m, nnz, descrA, csrSortedValA_valM, csrSortedRowPtrA, csrSortedColIndA, info, policy, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseCcsric02"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseZcsric02(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseZcsric02 {
        Some(____func) => unsafe { ____func(handle, m, nnz, descrA, csrSortedValA_valM, csrSortedRowPtrA, csrSortedColIndA, info, policy, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseZcsric02"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseXbsric02_zeroPivot(handle: cusparseHandle_t, info: bsric02Info_t, position: *mut ::std::os::raw::c_int) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseXbsric02_zeroPivot {
        Some(____func) => unsafe { ____func(handle, info, position) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseXbsric02_zeroPivot"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSbsric02_bufferSize(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSbsric02_bufferSize {
        Some(____func) => unsafe { ____func(handle, dirA, mb, nnzb, descrA, bsrSortedVal, bsrSortedRowPtr, bsrSortedColInd, blockDim, info, pBufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseSbsric02_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDbsric02_bufferSize(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDbsric02_bufferSize {
        Some(____func) => unsafe { ____func(handle, dirA, mb, nnzb, descrA, bsrSortedVal, bsrSortedRowPtr, bsrSortedColInd, blockDim, info, pBufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseDbsric02_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCbsric02_bufferSize(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCbsric02_bufferSize {
        Some(____func) => unsafe { ____func(handle, dirA, mb, nnzb, descrA, bsrSortedVal, bsrSortedRowPtr, bsrSortedColInd, blockDim, info, pBufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseCbsric02_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseZbsric02_bufferSize(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseZbsric02_bufferSize {
        Some(____func) => unsafe { ____func(handle, dirA, mb, nnzb, descrA, bsrSortedVal, bsrSortedRowPtr, bsrSortedColInd, blockDim, info, pBufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseZbsric02_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSbsric02_bufferSizeExt(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSbsric02_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, dirA, mb, nnzb, descrA, bsrSortedVal, bsrSortedRowPtr, bsrSortedColInd, blockSize, info, pBufferSize) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseSbsric02_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDbsric02_bufferSizeExt(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDbsric02_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, dirA, mb, nnzb, descrA, bsrSortedVal, bsrSortedRowPtr, bsrSortedColInd, blockSize, info, pBufferSize) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseDbsric02_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCbsric02_bufferSizeExt(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCbsric02_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, dirA, mb, nnzb, descrA, bsrSortedVal, bsrSortedRowPtr, bsrSortedColInd, blockSize, info, pBufferSize) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseCbsric02_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseZbsric02_bufferSizeExt(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseZbsric02_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, dirA, mb, nnzb, descrA, bsrSortedVal, bsrSortedRowPtr, bsrSortedColInd, blockSize, info, pBufferSize) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseZbsric02_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSbsric02_analysis(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSbsric02_analysis {
        Some(____func) => unsafe { ____func(handle, dirA, mb, nnzb, descrA, bsrSortedVal, bsrSortedRowPtr, bsrSortedColInd, blockDim, info, policy, pInputBuffer) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseSbsric02_analysis"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDbsric02_analysis(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDbsric02_analysis {
        Some(____func) => unsafe { ____func(handle, dirA, mb, nnzb, descrA, bsrSortedVal, bsrSortedRowPtr, bsrSortedColInd, blockDim, info, policy, pInputBuffer) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseDbsric02_analysis"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCbsric02_analysis(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCbsric02_analysis {
        Some(____func) => unsafe { ____func(handle, dirA, mb, nnzb, descrA, bsrSortedVal, bsrSortedRowPtr, bsrSortedColInd, blockDim, info, policy, pInputBuffer) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseCbsric02_analysis"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseZbsric02_analysis(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseZbsric02_analysis {
        Some(____func) => unsafe { ____func(handle, dirA, mb, nnzb, descrA, bsrSortedVal, bsrSortedRowPtr, bsrSortedColInd, blockDim, info, policy, pInputBuffer) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseZbsric02_analysis"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSbsric02(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSbsric02 {
        Some(____func) => unsafe { ____func(handle, dirA, mb, nnzb, descrA, bsrSortedVal, bsrSortedRowPtr, bsrSortedColInd, blockDim, info, policy, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseSbsric02"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDbsric02(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDbsric02 {
        Some(____func) => unsafe { ____func(handle, dirA, mb, nnzb, descrA, bsrSortedVal, bsrSortedRowPtr, bsrSortedColInd, blockDim, info, policy, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseDbsric02"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCbsric02(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCbsric02 {
        Some(____func) => unsafe { ____func(handle, dirA, mb, nnzb, descrA, bsrSortedVal, bsrSortedRowPtr, bsrSortedColInd, blockDim, info, policy, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseCbsric02"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseZbsric02(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseZbsric02 {
        Some(____func) => unsafe { ____func(handle, dirA, mb, nnzb, descrA, bsrSortedVal, bsrSortedRowPtr, bsrSortedColInd, blockDim, info, policy, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseZbsric02"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSgtsv2_bufferSizeExt(handle: cusparseHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, dl: *const f32, d: *const f32, du: *const f32, B: *const f32, ldb: ::std::os::raw::c_int, bufferSizeInBytes: *mut usize) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSgtsv2_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, m, n, dl, d, du, B, ldb, bufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseSgtsv2_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDgtsv2_bufferSizeExt(handle: cusparseHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, dl: *const f64, d: *const f64, du: *const f64, B: *const f64, ldb: ::std::os::raw::c_int, bufferSizeInBytes: *mut usize) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDgtsv2_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, m, n, dl, d, du, B, ldb, bufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseDgtsv2_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCgtsv2_bufferSizeExt(handle: cusparseHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, dl: *const cuComplex, d: *const cuComplex, du: *const cuComplex, B: *const cuComplex, ldb: ::std::os::raw::c_int, bufferSizeInBytes: *mut usize) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCgtsv2_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, m, n, dl, d, du, B, ldb, bufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseCgtsv2_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseZgtsv2_bufferSizeExt(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    dl: *const cuDoubleComplex,
    d: *const cuDoubleComplex,
    du: *const cuDoubleComplex,
    B: *const cuDoubleComplex,
    ldb: ::std::os::raw::c_int,
    bufferSizeInBytes: *mut usize,
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseZgtsv2_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, m, n, dl, d, du, B, ldb, bufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseZgtsv2_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSgtsv2(handle: cusparseHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, dl: *const f32, d: *const f32, du: *const f32, B: *mut f32, ldb: ::std::os::raw::c_int, pBuffer: *mut ::std::os::raw::c_void) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSgtsv2 {
        Some(____func) => unsafe { ____func(handle, m, n, dl, d, du, B, ldb, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseSgtsv2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDgtsv2(handle: cusparseHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, dl: *const f64, d: *const f64, du: *const f64, B: *mut f64, ldb: ::std::os::raw::c_int, pBuffer: *mut ::std::os::raw::c_void) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDgtsv2 {
        Some(____func) => unsafe { ____func(handle, m, n, dl, d, du, B, ldb, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseDgtsv2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCgtsv2(handle: cusparseHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, dl: *const cuComplex, d: *const cuComplex, du: *const cuComplex, B: *mut cuComplex, ldb: ::std::os::raw::c_int, pBuffer: *mut ::std::os::raw::c_void) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCgtsv2 {
        Some(____func) => unsafe { ____func(handle, m, n, dl, d, du, B, ldb, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseCgtsv2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseZgtsv2(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    dl: *const cuDoubleComplex,
    d: *const cuDoubleComplex,
    du: *const cuDoubleComplex,
    B: *mut cuDoubleComplex,
    ldb: ::std::os::raw::c_int,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseZgtsv2 {
        Some(____func) => unsafe { ____func(handle, m, n, dl, d, du, B, ldb, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseZgtsv2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSgtsv2_nopivot_bufferSizeExt(handle: cusparseHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, dl: *const f32, d: *const f32, du: *const f32, B: *const f32, ldb: ::std::os::raw::c_int, bufferSizeInBytes: *mut usize) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSgtsv2_nopivot_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, m, n, dl, d, du, B, ldb, bufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseSgtsv2_nopivot_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDgtsv2_nopivot_bufferSizeExt(handle: cusparseHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, dl: *const f64, d: *const f64, du: *const f64, B: *const f64, ldb: ::std::os::raw::c_int, bufferSizeInBytes: *mut usize) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDgtsv2_nopivot_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, m, n, dl, d, du, B, ldb, bufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseDgtsv2_nopivot_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCgtsv2_nopivot_bufferSizeExt(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    dl: *const cuComplex,
    d: *const cuComplex,
    du: *const cuComplex,
    B: *const cuComplex,
    ldb: ::std::os::raw::c_int,
    bufferSizeInBytes: *mut usize,
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCgtsv2_nopivot_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, m, n, dl, d, du, B, ldb, bufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseCgtsv2_nopivot_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseZgtsv2_nopivot_bufferSizeExt(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    dl: *const cuDoubleComplex,
    d: *const cuDoubleComplex,
    du: *const cuDoubleComplex,
    B: *const cuDoubleComplex,
    ldb: ::std::os::raw::c_int,
    bufferSizeInBytes: *mut usize,
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseZgtsv2_nopivot_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, m, n, dl, d, du, B, ldb, bufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseZgtsv2_nopivot_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSgtsv2_nopivot(handle: cusparseHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, dl: *const f32, d: *const f32, du: *const f32, B: *mut f32, ldb: ::std::os::raw::c_int, pBuffer: *mut ::std::os::raw::c_void) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSgtsv2_nopivot {
        Some(____func) => unsafe { ____func(handle, m, n, dl, d, du, B, ldb, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseSgtsv2_nopivot"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDgtsv2_nopivot(handle: cusparseHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, dl: *const f64, d: *const f64, du: *const f64, B: *mut f64, ldb: ::std::os::raw::c_int, pBuffer: *mut ::std::os::raw::c_void) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDgtsv2_nopivot {
        Some(____func) => unsafe { ____func(handle, m, n, dl, d, du, B, ldb, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseDgtsv2_nopivot"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCgtsv2_nopivot(handle: cusparseHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, dl: *const cuComplex, d: *const cuComplex, du: *const cuComplex, B: *mut cuComplex, ldb: ::std::os::raw::c_int, pBuffer: *mut ::std::os::raw::c_void) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCgtsv2_nopivot {
        Some(____func) => unsafe { ____func(handle, m, n, dl, d, du, B, ldb, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseCgtsv2_nopivot"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseZgtsv2_nopivot(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    dl: *const cuDoubleComplex,
    d: *const cuDoubleComplex,
    du: *const cuDoubleComplex,
    B: *mut cuDoubleComplex,
    ldb: ::std::os::raw::c_int,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseZgtsv2_nopivot {
        Some(____func) => unsafe { ____func(handle, m, n, dl, d, du, B, ldb, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseZgtsv2_nopivot"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSgtsv2StridedBatch_bufferSizeExt(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    dl: *const f32,
    d: *const f32,
    du: *const f32,
    x: *const f32,
    batchCount: ::std::os::raw::c_int,
    batchStride: ::std::os::raw::c_int,
    bufferSizeInBytes: *mut usize,
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSgtsv2StridedBatch_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, m, dl, d, du, x, batchCount, batchStride, bufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseSgtsv2StridedBatch_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDgtsv2StridedBatch_bufferSizeExt(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    dl: *const f64,
    d: *const f64,
    du: *const f64,
    x: *const f64,
    batchCount: ::std::os::raw::c_int,
    batchStride: ::std::os::raw::c_int,
    bufferSizeInBytes: *mut usize,
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDgtsv2StridedBatch_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, m, dl, d, du, x, batchCount, batchStride, bufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseDgtsv2StridedBatch_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCgtsv2StridedBatch_bufferSizeExt(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    dl: *const cuComplex,
    d: *const cuComplex,
    du: *const cuComplex,
    x: *const cuComplex,
    batchCount: ::std::os::raw::c_int,
    batchStride: ::std::os::raw::c_int,
    bufferSizeInBytes: *mut usize,
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCgtsv2StridedBatch_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, m, dl, d, du, x, batchCount, batchStride, bufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseCgtsv2StridedBatch_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseZgtsv2StridedBatch_bufferSizeExt(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    dl: *const cuDoubleComplex,
    d: *const cuDoubleComplex,
    du: *const cuDoubleComplex,
    x: *const cuDoubleComplex,
    batchCount: ::std::os::raw::c_int,
    batchStride: ::std::os::raw::c_int,
    bufferSizeInBytes: *mut usize,
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseZgtsv2StridedBatch_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, m, dl, d, du, x, batchCount, batchStride, bufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseZgtsv2StridedBatch_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSgtsv2StridedBatch(handle: cusparseHandle_t, m: ::std::os::raw::c_int, dl: *const f32, d: *const f32, du: *const f32, x: *mut f32, batchCount: ::std::os::raw::c_int, batchStride: ::std::os::raw::c_int, pBuffer: *mut ::std::os::raw::c_void) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSgtsv2StridedBatch {
        Some(____func) => unsafe { ____func(handle, m, dl, d, du, x, batchCount, batchStride, pBuffer) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseSgtsv2StridedBatch"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDgtsv2StridedBatch(handle: cusparseHandle_t, m: ::std::os::raw::c_int, dl: *const f64, d: *const f64, du: *const f64, x: *mut f64, batchCount: ::std::os::raw::c_int, batchStride: ::std::os::raw::c_int, pBuffer: *mut ::std::os::raw::c_void) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDgtsv2StridedBatch {
        Some(____func) => unsafe { ____func(handle, m, dl, d, du, x, batchCount, batchStride, pBuffer) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseDgtsv2StridedBatch"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCgtsv2StridedBatch(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    dl: *const cuComplex,
    d: *const cuComplex,
    du: *const cuComplex,
    x: *mut cuComplex,
    batchCount: ::std::os::raw::c_int,
    batchStride: ::std::os::raw::c_int,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCgtsv2StridedBatch {
        Some(____func) => unsafe { ____func(handle, m, dl, d, du, x, batchCount, batchStride, pBuffer) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseCgtsv2StridedBatch"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseZgtsv2StridedBatch(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    dl: *const cuDoubleComplex,
    d: *const cuDoubleComplex,
    du: *const cuDoubleComplex,
    x: *mut cuDoubleComplex,
    batchCount: ::std::os::raw::c_int,
    batchStride: ::std::os::raw::c_int,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseZgtsv2StridedBatch {
        Some(____func) => unsafe { ____func(handle, m, dl, d, du, x, batchCount, batchStride, pBuffer) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseZgtsv2StridedBatch"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSgtsvInterleavedBatch_bufferSizeExt(handle: cusparseHandle_t, algo: ::std::os::raw::c_int, m: ::std::os::raw::c_int, dl: *const f32, d: *const f32, du: *const f32, x: *const f32, batchCount: ::std::os::raw::c_int, pBufferSizeInBytes: *mut usize) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSgtsvInterleavedBatch_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, algo, m, dl, d, du, x, batchCount, pBufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseSgtsvInterleavedBatch_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDgtsvInterleavedBatch_bufferSizeExt(handle: cusparseHandle_t, algo: ::std::os::raw::c_int, m: ::std::os::raw::c_int, dl: *const f64, d: *const f64, du: *const f64, x: *const f64, batchCount: ::std::os::raw::c_int, pBufferSizeInBytes: *mut usize) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDgtsvInterleavedBatch_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, algo, m, dl, d, du, x, batchCount, pBufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseDgtsvInterleavedBatch_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCgtsvInterleavedBatch_bufferSizeExt(
    handle: cusparseHandle_t,
    algo: ::std::os::raw::c_int,
    m: ::std::os::raw::c_int,
    dl: *const cuComplex,
    d: *const cuComplex,
    du: *const cuComplex,
    x: *const cuComplex,
    batchCount: ::std::os::raw::c_int,
    pBufferSizeInBytes: *mut usize,
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCgtsvInterleavedBatch_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, algo, m, dl, d, du, x, batchCount, pBufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseCgtsvInterleavedBatch_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseZgtsvInterleavedBatch_bufferSizeExt(
    handle: cusparseHandle_t,
    algo: ::std::os::raw::c_int,
    m: ::std::os::raw::c_int,
    dl: *const cuDoubleComplex,
    d: *const cuDoubleComplex,
    du: *const cuDoubleComplex,
    x: *const cuDoubleComplex,
    batchCount: ::std::os::raw::c_int,
    pBufferSizeInBytes: *mut usize,
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseZgtsvInterleavedBatch_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, algo, m, dl, d, du, x, batchCount, pBufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseZgtsvInterleavedBatch_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSgtsvInterleavedBatch(handle: cusparseHandle_t, algo: ::std::os::raw::c_int, m: ::std::os::raw::c_int, dl: *mut f32, d: *mut f32, du: *mut f32, x: *mut f32, batchCount: ::std::os::raw::c_int, pBuffer: *mut ::std::os::raw::c_void) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSgtsvInterleavedBatch {
        Some(____func) => unsafe { ____func(handle, algo, m, dl, d, du, x, batchCount, pBuffer) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseSgtsvInterleavedBatch"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDgtsvInterleavedBatch(handle: cusparseHandle_t, algo: ::std::os::raw::c_int, m: ::std::os::raw::c_int, dl: *mut f64, d: *mut f64, du: *mut f64, x: *mut f64, batchCount: ::std::os::raw::c_int, pBuffer: *mut ::std::os::raw::c_void) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDgtsvInterleavedBatch {
        Some(____func) => unsafe { ____func(handle, algo, m, dl, d, du, x, batchCount, pBuffer) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseDgtsvInterleavedBatch"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCgtsvInterleavedBatch(
    handle: cusparseHandle_t,
    algo: ::std::os::raw::c_int,
    m: ::std::os::raw::c_int,
    dl: *mut cuComplex,
    d: *mut cuComplex,
    du: *mut cuComplex,
    x: *mut cuComplex,
    batchCount: ::std::os::raw::c_int,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCgtsvInterleavedBatch {
        Some(____func) => unsafe { ____func(handle, algo, m, dl, d, du, x, batchCount, pBuffer) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseCgtsvInterleavedBatch"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseZgtsvInterleavedBatch(
    handle: cusparseHandle_t,
    algo: ::std::os::raw::c_int,
    m: ::std::os::raw::c_int,
    dl: *mut cuDoubleComplex,
    d: *mut cuDoubleComplex,
    du: *mut cuDoubleComplex,
    x: *mut cuDoubleComplex,
    batchCount: ::std::os::raw::c_int,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseZgtsvInterleavedBatch {
        Some(____func) => unsafe { ____func(handle, algo, m, dl, d, du, x, batchCount, pBuffer) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseZgtsvInterleavedBatch"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSgpsvInterleavedBatch_bufferSizeExt(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSgpsvInterleavedBatch_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, algo, m, ds, dl, d, du, dw, x, batchCount, pBufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseSgpsvInterleavedBatch_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDgpsvInterleavedBatch_bufferSizeExt(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDgpsvInterleavedBatch_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, algo, m, ds, dl, d, du, dw, x, batchCount, pBufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseDgpsvInterleavedBatch_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCgpsvInterleavedBatch_bufferSizeExt(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCgpsvInterleavedBatch_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, algo, m, ds, dl, d, du, dw, x, batchCount, pBufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseCgpsvInterleavedBatch_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseZgpsvInterleavedBatch_bufferSizeExt(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseZgpsvInterleavedBatch_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, algo, m, ds, dl, d, du, dw, x, batchCount, pBufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseZgpsvInterleavedBatch_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSgpsvInterleavedBatch(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSgpsvInterleavedBatch {
        Some(____func) => unsafe { ____func(handle, algo, m, ds, dl, d, du, dw, x, batchCount, pBuffer) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseSgpsvInterleavedBatch"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDgpsvInterleavedBatch(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDgpsvInterleavedBatch {
        Some(____func) => unsafe { ____func(handle, algo, m, ds, dl, d, du, dw, x, batchCount, pBuffer) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseDgpsvInterleavedBatch"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCgpsvInterleavedBatch(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCgpsvInterleavedBatch {
        Some(____func) => unsafe { ____func(handle, algo, m, ds, dl, d, du, dw, x, batchCount, pBuffer) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseCgpsvInterleavedBatch"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseZgpsvInterleavedBatch(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseZgpsvInterleavedBatch {
        Some(____func) => unsafe { ____func(handle, algo, m, ds, dl, d, du, dw, x, batchCount, pBuffer) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseZgpsvInterleavedBatch"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseScsrgeam2_bufferSizeExt(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseScsrgeam2_bufferSizeExt {
        Some(____func) => unsafe {
            ____func(
                handle,
                m,
                n,
                alpha,
                descrA,
                nnzA,
                csrSortedValA,
                csrSortedRowPtrA,
                csrSortedColIndA,
                beta,
                descrB,
                nnzB,
                csrSortedValB,
                csrSortedRowPtrB,
                csrSortedColIndB,
                descrC,
                csrSortedValC,
                csrSortedRowPtrC,
                csrSortedColIndC,
                pBufferSizeInBytes,
            )
        },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseScsrgeam2_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDcsrgeam2_bufferSizeExt(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDcsrgeam2_bufferSizeExt {
        Some(____func) => unsafe {
            ____func(
                handle,
                m,
                n,
                alpha,
                descrA,
                nnzA,
                csrSortedValA,
                csrSortedRowPtrA,
                csrSortedColIndA,
                beta,
                descrB,
                nnzB,
                csrSortedValB,
                csrSortedRowPtrB,
                csrSortedColIndB,
                descrC,
                csrSortedValC,
                csrSortedRowPtrC,
                csrSortedColIndC,
                pBufferSizeInBytes,
            )
        },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseDcsrgeam2_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCcsrgeam2_bufferSizeExt(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCcsrgeam2_bufferSizeExt {
        Some(____func) => unsafe {
            ____func(
                handle,
                m,
                n,
                alpha,
                descrA,
                nnzA,
                csrSortedValA,
                csrSortedRowPtrA,
                csrSortedColIndA,
                beta,
                descrB,
                nnzB,
                csrSortedValB,
                csrSortedRowPtrB,
                csrSortedColIndB,
                descrC,
                csrSortedValC,
                csrSortedRowPtrC,
                csrSortedColIndC,
                pBufferSizeInBytes,
            )
        },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseCcsrgeam2_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseZcsrgeam2_bufferSizeExt(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseZcsrgeam2_bufferSizeExt {
        Some(____func) => unsafe {
            ____func(
                handle,
                m,
                n,
                alpha,
                descrA,
                nnzA,
                csrSortedValA,
                csrSortedRowPtrA,
                csrSortedColIndA,
                beta,
                descrB,
                nnzB,
                csrSortedValB,
                csrSortedRowPtrB,
                csrSortedColIndB,
                descrC,
                csrSortedValC,
                csrSortedRowPtrC,
                csrSortedColIndC,
                pBufferSizeInBytes,
            )
        },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseZcsrgeam2_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseXcsrgeam2Nnz(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseXcsrgeam2Nnz {
        Some(____func) => unsafe { ____func(handle, m, n, descrA, nnzA, csrSortedRowPtrA, csrSortedColIndA, descrB, nnzB, csrSortedRowPtrB, csrSortedColIndB, descrC, csrSortedRowPtrC, nnzTotalDevHostPtr, workspace) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseXcsrgeam2Nnz"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseScsrgeam2(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseScsrgeam2 {
        Some(____func) => unsafe {
            ____func(
                handle,
                m,
                n,
                alpha,
                descrA,
                nnzA,
                csrSortedValA,
                csrSortedRowPtrA,
                csrSortedColIndA,
                beta,
                descrB,
                nnzB,
                csrSortedValB,
                csrSortedRowPtrB,
                csrSortedColIndB,
                descrC,
                csrSortedValC,
                csrSortedRowPtrC,
                csrSortedColIndC,
                pBuffer,
            )
        },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseScsrgeam2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDcsrgeam2(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDcsrgeam2 {
        Some(____func) => unsafe {
            ____func(
                handle,
                m,
                n,
                alpha,
                descrA,
                nnzA,
                csrSortedValA,
                csrSortedRowPtrA,
                csrSortedColIndA,
                beta,
                descrB,
                nnzB,
                csrSortedValB,
                csrSortedRowPtrB,
                csrSortedColIndB,
                descrC,
                csrSortedValC,
                csrSortedRowPtrC,
                csrSortedColIndC,
                pBuffer,
            )
        },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseDcsrgeam2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCcsrgeam2(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCcsrgeam2 {
        Some(____func) => unsafe {
            ____func(
                handle,
                m,
                n,
                alpha,
                descrA,
                nnzA,
                csrSortedValA,
                csrSortedRowPtrA,
                csrSortedColIndA,
                beta,
                descrB,
                nnzB,
                csrSortedValB,
                csrSortedRowPtrB,
                csrSortedColIndB,
                descrC,
                csrSortedValC,
                csrSortedRowPtrC,
                csrSortedColIndC,
                pBuffer,
            )
        },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseCcsrgeam2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseZcsrgeam2(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseZcsrgeam2 {
        Some(____func) => unsafe {
            ____func(
                handle,
                m,
                n,
                alpha,
                descrA,
                nnzA,
                csrSortedValA,
                csrSortedRowPtrA,
                csrSortedColIndA,
                beta,
                descrB,
                nnzB,
                csrSortedValB,
                csrSortedRowPtrB,
                csrSortedColIndB,
                descrC,
                csrSortedValC,
                csrSortedRowPtrC,
                csrSortedColIndC,
                pBuffer,
            )
        },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseZcsrgeam2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseScsrcolor(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseScsrcolor {
        Some(____func) => unsafe { ____func(handle, m, nnz, descrA, csrSortedValA, csrSortedRowPtrA, csrSortedColIndA, fractionToColor, ncolors, coloring, reordering, info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseScsrcolor"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDcsrcolor(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDcsrcolor {
        Some(____func) => unsafe { ____func(handle, m, nnz, descrA, csrSortedValA, csrSortedRowPtrA, csrSortedColIndA, fractionToColor, ncolors, coloring, reordering, info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseDcsrcolor"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCcsrcolor(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCcsrcolor {
        Some(____func) => unsafe { ____func(handle, m, nnz, descrA, csrSortedValA, csrSortedRowPtrA, csrSortedColIndA, fractionToColor, ncolors, coloring, reordering, info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseCcsrcolor"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseZcsrcolor(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseZcsrcolor {
        Some(____func) => unsafe { ____func(handle, m, nnz, descrA, csrSortedValA, csrSortedRowPtrA, csrSortedColIndA, fractionToColor, ncolors, coloring, reordering, info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseZcsrcolor"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSnnz(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    nnzPerRowCol: *mut ::std::os::raw::c_int,
    nnzTotalDevHostPtr: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSnnz {
        Some(____func) => unsafe { ____func(handle, dirA, m, n, descrA, A, lda, nnzPerRowCol, nnzTotalDevHostPtr) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseSnnz"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDnnz(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    nnzPerRowCol: *mut ::std::os::raw::c_int,
    nnzTotalDevHostPtr: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDnnz {
        Some(____func) => unsafe { ____func(handle, dirA, m, n, descrA, A, lda, nnzPerRowCol, nnzTotalDevHostPtr) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseDnnz"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCnnz(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    nnzPerRowCol: *mut ::std::os::raw::c_int,
    nnzTotalDevHostPtr: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCnnz {
        Some(____func) => unsafe { ____func(handle, dirA, m, n, descrA, A, lda, nnzPerRowCol, nnzTotalDevHostPtr) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseCnnz"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseZnnz(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    nnzPerRowCol: *mut ::std::os::raw::c_int,
    nnzTotalDevHostPtr: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseZnnz {
        Some(____func) => unsafe { ____func(handle, dirA, m, n, descrA, A, lda, nnzPerRowCol, nnzTotalDevHostPtr) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseZnnz"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSnnz_compress(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    descr: cusparseMatDescr_t,
    csrSortedValA: *const f32,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    nnzPerRow: *mut ::std::os::raw::c_int,
    nnzC: *mut ::std::os::raw::c_int,
    tol: f32,
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSnnz_compress {
        Some(____func) => unsafe { ____func(handle, m, descr, csrSortedValA, csrSortedRowPtrA, nnzPerRow, nnzC, tol) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseSnnz_compress"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDnnz_compress(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    descr: cusparseMatDescr_t,
    csrSortedValA: *const f64,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    nnzPerRow: *mut ::std::os::raw::c_int,
    nnzC: *mut ::std::os::raw::c_int,
    tol: f64,
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDnnz_compress {
        Some(____func) => unsafe { ____func(handle, m, descr, csrSortedValA, csrSortedRowPtrA, nnzPerRow, nnzC, tol) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseDnnz_compress"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCnnz_compress(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    descr: cusparseMatDescr_t,
    csrSortedValA: *const cuComplex,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    nnzPerRow: *mut ::std::os::raw::c_int,
    nnzC: *mut ::std::os::raw::c_int,
    tol: cuComplex,
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCnnz_compress {
        Some(____func) => unsafe { ____func(handle, m, descr, csrSortedValA, csrSortedRowPtrA, nnzPerRow, nnzC, tol) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseCnnz_compress"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseZnnz_compress(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    descr: cusparseMatDescr_t,
    csrSortedValA: *const cuDoubleComplex,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    nnzPerRow: *mut ::std::os::raw::c_int,
    nnzC: *mut ::std::os::raw::c_int,
    tol: cuDoubleComplex,
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseZnnz_compress {
        Some(____func) => unsafe { ____func(handle, m, descr, csrSortedValA, csrSortedRowPtrA, nnzPerRow, nnzC, tol) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseZnnz_compress"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseScsr2csr_compress(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseScsr2csr_compress {
        Some(____func) => unsafe { ____func(handle, m, n, descrA, csrSortedValA, csrSortedColIndA, csrSortedRowPtrA, nnzA, nnzPerRow, csrSortedValC, csrSortedColIndC, csrSortedRowPtrC, tol) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseScsr2csr_compress"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDcsr2csr_compress(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDcsr2csr_compress {
        Some(____func) => unsafe { ____func(handle, m, n, descrA, csrSortedValA, csrSortedColIndA, csrSortedRowPtrA, nnzA, nnzPerRow, csrSortedValC, csrSortedColIndC, csrSortedRowPtrC, tol) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseDcsr2csr_compress"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCcsr2csr_compress(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCcsr2csr_compress {
        Some(____func) => unsafe { ____func(handle, m, n, descrA, csrSortedValA, csrSortedColIndA, csrSortedRowPtrA, nnzA, nnzPerRow, csrSortedValC, csrSortedColIndC, csrSortedRowPtrC, tol) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseCcsr2csr_compress"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseZcsr2csr_compress(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseZcsr2csr_compress {
        Some(____func) => unsafe { ____func(handle, m, n, descrA, csrSortedValA, csrSortedColIndA, csrSortedRowPtrA, nnzA, nnzPerRow, csrSortedValC, csrSortedColIndC, csrSortedRowPtrC, tol) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseZcsr2csr_compress"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseXcoo2csr(handle: cusparseHandle_t, cooRowInd: *const ::std::os::raw::c_int, nnz: ::std::os::raw::c_int, m: ::std::os::raw::c_int, csrSortedRowPtr: *mut ::std::os::raw::c_int, idxBase: cusparseIndexBase_t) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseXcoo2csr {
        Some(____func) => unsafe { ____func(handle, cooRowInd, nnz, m, csrSortedRowPtr, idxBase) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseXcoo2csr"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseXcsr2coo(handle: cusparseHandle_t, csrSortedRowPtr: *const ::std::os::raw::c_int, nnz: ::std::os::raw::c_int, m: ::std::os::raw::c_int, cooRowInd: *mut ::std::os::raw::c_int, idxBase: cusparseIndexBase_t) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseXcsr2coo {
        Some(____func) => unsafe { ____func(handle, csrSortedRowPtr, nnz, m, cooRowInd, idxBase) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseXcsr2coo"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseXcsr2bsrNnz(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseXcsr2bsrNnz {
        Some(____func) => unsafe { ____func(handle, dirA, m, n, descrA, csrSortedRowPtrA, csrSortedColIndA, blockDim, descrC, bsrSortedRowPtrC, nnzTotalDevHostPtr) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseXcsr2bsrNnz"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseScsr2bsr(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseScsr2bsr {
        Some(____func) => unsafe { ____func(handle, dirA, m, n, descrA, csrSortedValA, csrSortedRowPtrA, csrSortedColIndA, blockDim, descrC, bsrSortedValC, bsrSortedRowPtrC, bsrSortedColIndC) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseScsr2bsr"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDcsr2bsr(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDcsr2bsr {
        Some(____func) => unsafe { ____func(handle, dirA, m, n, descrA, csrSortedValA, csrSortedRowPtrA, csrSortedColIndA, blockDim, descrC, bsrSortedValC, bsrSortedRowPtrC, bsrSortedColIndC) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseDcsr2bsr"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCcsr2bsr(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCcsr2bsr {
        Some(____func) => unsafe { ____func(handle, dirA, m, n, descrA, csrSortedValA, csrSortedRowPtrA, csrSortedColIndA, blockDim, descrC, bsrSortedValC, bsrSortedRowPtrC, bsrSortedColIndC) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseCcsr2bsr"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseZcsr2bsr(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseZcsr2bsr {
        Some(____func) => unsafe { ____func(handle, dirA, m, n, descrA, csrSortedValA, csrSortedRowPtrA, csrSortedColIndA, blockDim, descrC, bsrSortedValC, bsrSortedRowPtrC, bsrSortedColIndC) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseZcsr2bsr"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSbsr2csr(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSbsr2csr {
        Some(____func) => unsafe { ____func(handle, dirA, mb, nb, descrA, bsrSortedValA, bsrSortedRowPtrA, bsrSortedColIndA, blockDim, descrC, csrSortedValC, csrSortedRowPtrC, csrSortedColIndC) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseSbsr2csr"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDbsr2csr(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDbsr2csr {
        Some(____func) => unsafe { ____func(handle, dirA, mb, nb, descrA, bsrSortedValA, bsrSortedRowPtrA, bsrSortedColIndA, blockDim, descrC, csrSortedValC, csrSortedRowPtrC, csrSortedColIndC) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseDbsr2csr"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCbsr2csr(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCbsr2csr {
        Some(____func) => unsafe { ____func(handle, dirA, mb, nb, descrA, bsrSortedValA, bsrSortedRowPtrA, bsrSortedColIndA, blockDim, descrC, csrSortedValC, csrSortedRowPtrC, csrSortedColIndC) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseCbsr2csr"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseZbsr2csr(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseZbsr2csr {
        Some(____func) => unsafe { ____func(handle, dirA, mb, nb, descrA, bsrSortedValA, bsrSortedRowPtrA, bsrSortedColIndA, blockDim, descrC, csrSortedValC, csrSortedRowPtrC, csrSortedColIndC) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseZbsr2csr"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSgebsr2gebsc_bufferSize(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSgebsr2gebsc_bufferSize {
        Some(____func) => unsafe { ____func(handle, mb, nb, nnzb, bsrSortedVal, bsrSortedRowPtr, bsrSortedColInd, rowBlockDim, colBlockDim, pBufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseSgebsr2gebsc_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDgebsr2gebsc_bufferSize(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDgebsr2gebsc_bufferSize {
        Some(____func) => unsafe { ____func(handle, mb, nb, nnzb, bsrSortedVal, bsrSortedRowPtr, bsrSortedColInd, rowBlockDim, colBlockDim, pBufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseDgebsr2gebsc_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCgebsr2gebsc_bufferSize(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCgebsr2gebsc_bufferSize {
        Some(____func) => unsafe { ____func(handle, mb, nb, nnzb, bsrSortedVal, bsrSortedRowPtr, bsrSortedColInd, rowBlockDim, colBlockDim, pBufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseCgebsr2gebsc_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseZgebsr2gebsc_bufferSize(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseZgebsr2gebsc_bufferSize {
        Some(____func) => unsafe { ____func(handle, mb, nb, nnzb, bsrSortedVal, bsrSortedRowPtr, bsrSortedColInd, rowBlockDim, colBlockDim, pBufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseZgebsr2gebsc_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSgebsr2gebsc_bufferSizeExt(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSgebsr2gebsc_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, mb, nb, nnzb, bsrSortedVal, bsrSortedRowPtr, bsrSortedColInd, rowBlockDim, colBlockDim, pBufferSize) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseSgebsr2gebsc_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDgebsr2gebsc_bufferSizeExt(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDgebsr2gebsc_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, mb, nb, nnzb, bsrSortedVal, bsrSortedRowPtr, bsrSortedColInd, rowBlockDim, colBlockDim, pBufferSize) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseDgebsr2gebsc_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCgebsr2gebsc_bufferSizeExt(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCgebsr2gebsc_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, mb, nb, nnzb, bsrSortedVal, bsrSortedRowPtr, bsrSortedColInd, rowBlockDim, colBlockDim, pBufferSize) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseCgebsr2gebsc_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseZgebsr2gebsc_bufferSizeExt(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseZgebsr2gebsc_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, mb, nb, nnzb, bsrSortedVal, bsrSortedRowPtr, bsrSortedColInd, rowBlockDim, colBlockDim, pBufferSize) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseZgebsr2gebsc_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSgebsr2gebsc(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSgebsr2gebsc {
        Some(____func) => unsafe { ____func(handle, mb, nb, nnzb, bsrSortedVal, bsrSortedRowPtr, bsrSortedColInd, rowBlockDim, colBlockDim, bscVal, bscRowInd, bscColPtr, copyValues, idxBase, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseSgebsr2gebsc"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDgebsr2gebsc(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDgebsr2gebsc {
        Some(____func) => unsafe { ____func(handle, mb, nb, nnzb, bsrSortedVal, bsrSortedRowPtr, bsrSortedColInd, rowBlockDim, colBlockDim, bscVal, bscRowInd, bscColPtr, copyValues, idxBase, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseDgebsr2gebsc"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCgebsr2gebsc(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCgebsr2gebsc {
        Some(____func) => unsafe { ____func(handle, mb, nb, nnzb, bsrSortedVal, bsrSortedRowPtr, bsrSortedColInd, rowBlockDim, colBlockDim, bscVal, bscRowInd, bscColPtr, copyValues, idxBase, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseCgebsr2gebsc"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseZgebsr2gebsc(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseZgebsr2gebsc {
        Some(____func) => unsafe { ____func(handle, mb, nb, nnzb, bsrSortedVal, bsrSortedRowPtr, bsrSortedColInd, rowBlockDim, colBlockDim, bscVal, bscRowInd, bscColPtr, copyValues, idxBase, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseZgebsr2gebsc"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseXgebsr2csr(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseXgebsr2csr {
        Some(____func) => unsafe { ____func(handle, dirA, mb, nb, descrA, bsrSortedRowPtrA, bsrSortedColIndA, rowBlockDim, colBlockDim, descrC, csrSortedRowPtrC, csrSortedColIndC) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseXgebsr2csr"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSgebsr2csr(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSgebsr2csr {
        Some(____func) => unsafe { ____func(handle, dirA, mb, nb, descrA, bsrSortedValA, bsrSortedRowPtrA, bsrSortedColIndA, rowBlockDim, colBlockDim, descrC, csrSortedValC, csrSortedRowPtrC, csrSortedColIndC) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseSgebsr2csr"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDgebsr2csr(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDgebsr2csr {
        Some(____func) => unsafe { ____func(handle, dirA, mb, nb, descrA, bsrSortedValA, bsrSortedRowPtrA, bsrSortedColIndA, rowBlockDim, colBlockDim, descrC, csrSortedValC, csrSortedRowPtrC, csrSortedColIndC) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseDgebsr2csr"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCgebsr2csr(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCgebsr2csr {
        Some(____func) => unsafe { ____func(handle, dirA, mb, nb, descrA, bsrSortedValA, bsrSortedRowPtrA, bsrSortedColIndA, rowBlockDim, colBlockDim, descrC, csrSortedValC, csrSortedRowPtrC, csrSortedColIndC) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseCgebsr2csr"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseZgebsr2csr(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseZgebsr2csr {
        Some(____func) => unsafe { ____func(handle, dirA, mb, nb, descrA, bsrSortedValA, bsrSortedRowPtrA, bsrSortedColIndA, rowBlockDim, colBlockDim, descrC, csrSortedValC, csrSortedRowPtrC, csrSortedColIndC) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseZgebsr2csr"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseScsr2gebsr_bufferSize(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseScsr2gebsr_bufferSize {
        Some(____func) => unsafe { ____func(handle, dirA, m, n, descrA, csrSortedValA, csrSortedRowPtrA, csrSortedColIndA, rowBlockDim, colBlockDim, pBufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseScsr2gebsr_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDcsr2gebsr_bufferSize(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDcsr2gebsr_bufferSize {
        Some(____func) => unsafe { ____func(handle, dirA, m, n, descrA, csrSortedValA, csrSortedRowPtrA, csrSortedColIndA, rowBlockDim, colBlockDim, pBufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseDcsr2gebsr_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCcsr2gebsr_bufferSize(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCcsr2gebsr_bufferSize {
        Some(____func) => unsafe { ____func(handle, dirA, m, n, descrA, csrSortedValA, csrSortedRowPtrA, csrSortedColIndA, rowBlockDim, colBlockDim, pBufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseCcsr2gebsr_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseZcsr2gebsr_bufferSize(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseZcsr2gebsr_bufferSize {
        Some(____func) => unsafe { ____func(handle, dirA, m, n, descrA, csrSortedValA, csrSortedRowPtrA, csrSortedColIndA, rowBlockDim, colBlockDim, pBufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseZcsr2gebsr_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseScsr2gebsr_bufferSizeExt(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseScsr2gebsr_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, dirA, m, n, descrA, csrSortedValA, csrSortedRowPtrA, csrSortedColIndA, rowBlockDim, colBlockDim, pBufferSize) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseScsr2gebsr_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDcsr2gebsr_bufferSizeExt(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDcsr2gebsr_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, dirA, m, n, descrA, csrSortedValA, csrSortedRowPtrA, csrSortedColIndA, rowBlockDim, colBlockDim, pBufferSize) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseDcsr2gebsr_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCcsr2gebsr_bufferSizeExt(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCcsr2gebsr_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, dirA, m, n, descrA, csrSortedValA, csrSortedRowPtrA, csrSortedColIndA, rowBlockDim, colBlockDim, pBufferSize) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseCcsr2gebsr_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseZcsr2gebsr_bufferSizeExt(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseZcsr2gebsr_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, dirA, m, n, descrA, csrSortedValA, csrSortedRowPtrA, csrSortedColIndA, rowBlockDim, colBlockDim, pBufferSize) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseZcsr2gebsr_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseXcsr2gebsrNnz(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseXcsr2gebsrNnz {
        Some(____func) => unsafe { ____func(handle, dirA, m, n, descrA, csrSortedRowPtrA, csrSortedColIndA, descrC, bsrSortedRowPtrC, rowBlockDim, colBlockDim, nnzTotalDevHostPtr, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseXcsr2gebsrNnz"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseScsr2gebsr(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseScsr2gebsr {
        Some(____func) => unsafe { ____func(handle, dirA, m, n, descrA, csrSortedValA, csrSortedRowPtrA, csrSortedColIndA, descrC, bsrSortedValC, bsrSortedRowPtrC, bsrSortedColIndC, rowBlockDim, colBlockDim, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseScsr2gebsr"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDcsr2gebsr(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDcsr2gebsr {
        Some(____func) => unsafe { ____func(handle, dirA, m, n, descrA, csrSortedValA, csrSortedRowPtrA, csrSortedColIndA, descrC, bsrSortedValC, bsrSortedRowPtrC, bsrSortedColIndC, rowBlockDim, colBlockDim, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseDcsr2gebsr"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCcsr2gebsr(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCcsr2gebsr {
        Some(____func) => unsafe { ____func(handle, dirA, m, n, descrA, csrSortedValA, csrSortedRowPtrA, csrSortedColIndA, descrC, bsrSortedValC, bsrSortedRowPtrC, bsrSortedColIndC, rowBlockDim, colBlockDim, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseCcsr2gebsr"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseZcsr2gebsr(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseZcsr2gebsr {
        Some(____func) => unsafe { ____func(handle, dirA, m, n, descrA, csrSortedValA, csrSortedRowPtrA, csrSortedColIndA, descrC, bsrSortedValC, bsrSortedRowPtrC, bsrSortedColIndC, rowBlockDim, colBlockDim, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseZcsr2gebsr"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSgebsr2gebsr_bufferSize(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSgebsr2gebsr_bufferSize {
        Some(____func) => unsafe { ____func(handle, dirA, mb, nb, nnzb, descrA, bsrSortedValA, bsrSortedRowPtrA, bsrSortedColIndA, rowBlockDimA, colBlockDimA, rowBlockDimC, colBlockDimC, pBufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseSgebsr2gebsr_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDgebsr2gebsr_bufferSize(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDgebsr2gebsr_bufferSize {
        Some(____func) => unsafe { ____func(handle, dirA, mb, nb, nnzb, descrA, bsrSortedValA, bsrSortedRowPtrA, bsrSortedColIndA, rowBlockDimA, colBlockDimA, rowBlockDimC, colBlockDimC, pBufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseDgebsr2gebsr_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCgebsr2gebsr_bufferSize(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCgebsr2gebsr_bufferSize {
        Some(____func) => unsafe { ____func(handle, dirA, mb, nb, nnzb, descrA, bsrSortedValA, bsrSortedRowPtrA, bsrSortedColIndA, rowBlockDimA, colBlockDimA, rowBlockDimC, colBlockDimC, pBufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseCgebsr2gebsr_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseZgebsr2gebsr_bufferSize(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseZgebsr2gebsr_bufferSize {
        Some(____func) => unsafe { ____func(handle, dirA, mb, nb, nnzb, descrA, bsrSortedValA, bsrSortedRowPtrA, bsrSortedColIndA, rowBlockDimA, colBlockDimA, rowBlockDimC, colBlockDimC, pBufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseZgebsr2gebsr_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSgebsr2gebsr_bufferSizeExt(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSgebsr2gebsr_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, dirA, mb, nb, nnzb, descrA, bsrSortedValA, bsrSortedRowPtrA, bsrSortedColIndA, rowBlockDimA, colBlockDimA, rowBlockDimC, colBlockDimC, pBufferSize) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseSgebsr2gebsr_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDgebsr2gebsr_bufferSizeExt(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDgebsr2gebsr_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, dirA, mb, nb, nnzb, descrA, bsrSortedValA, bsrSortedRowPtrA, bsrSortedColIndA, rowBlockDimA, colBlockDimA, rowBlockDimC, colBlockDimC, pBufferSize) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseDgebsr2gebsr_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCgebsr2gebsr_bufferSizeExt(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCgebsr2gebsr_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, dirA, mb, nb, nnzb, descrA, bsrSortedValA, bsrSortedRowPtrA, bsrSortedColIndA, rowBlockDimA, colBlockDimA, rowBlockDimC, colBlockDimC, pBufferSize) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseCgebsr2gebsr_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseZgebsr2gebsr_bufferSizeExt(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseZgebsr2gebsr_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, dirA, mb, nb, nnzb, descrA, bsrSortedValA, bsrSortedRowPtrA, bsrSortedColIndA, rowBlockDimA, colBlockDimA, rowBlockDimC, colBlockDimC, pBufferSize) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseZgebsr2gebsr_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseXgebsr2gebsrNnz(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseXgebsr2gebsrNnz {
        Some(____func) => unsafe {
            ____func(
                handle,
                dirA,
                mb,
                nb,
                nnzb,
                descrA,
                bsrSortedRowPtrA,
                bsrSortedColIndA,
                rowBlockDimA,
                colBlockDimA,
                descrC,
                bsrSortedRowPtrC,
                rowBlockDimC,
                colBlockDimC,
                nnzTotalDevHostPtr,
                pBuffer,
            )
        },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseXgebsr2gebsrNnz"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSgebsr2gebsr(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSgebsr2gebsr {
        Some(____func) => unsafe {
            ____func(
                handle,
                dirA,
                mb,
                nb,
                nnzb,
                descrA,
                bsrSortedValA,
                bsrSortedRowPtrA,
                bsrSortedColIndA,
                rowBlockDimA,
                colBlockDimA,
                descrC,
                bsrSortedValC,
                bsrSortedRowPtrC,
                bsrSortedColIndC,
                rowBlockDimC,
                colBlockDimC,
                pBuffer,
            )
        },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseSgebsr2gebsr"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDgebsr2gebsr(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDgebsr2gebsr {
        Some(____func) => unsafe {
            ____func(
                handle,
                dirA,
                mb,
                nb,
                nnzb,
                descrA,
                bsrSortedValA,
                bsrSortedRowPtrA,
                bsrSortedColIndA,
                rowBlockDimA,
                colBlockDimA,
                descrC,
                bsrSortedValC,
                bsrSortedRowPtrC,
                bsrSortedColIndC,
                rowBlockDimC,
                colBlockDimC,
                pBuffer,
            )
        },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseDgebsr2gebsr"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCgebsr2gebsr(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCgebsr2gebsr {
        Some(____func) => unsafe {
            ____func(
                handle,
                dirA,
                mb,
                nb,
                nnzb,
                descrA,
                bsrSortedValA,
                bsrSortedRowPtrA,
                bsrSortedColIndA,
                rowBlockDimA,
                colBlockDimA,
                descrC,
                bsrSortedValC,
                bsrSortedRowPtrC,
                bsrSortedColIndC,
                rowBlockDimC,
                colBlockDimC,
                pBuffer,
            )
        },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseCgebsr2gebsr"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseZgebsr2gebsr(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseZgebsr2gebsr {
        Some(____func) => unsafe {
            ____func(
                handle,
                dirA,
                mb,
                nb,
                nnzb,
                descrA,
                bsrSortedValA,
                bsrSortedRowPtrA,
                bsrSortedColIndA,
                rowBlockDimA,
                colBlockDimA,
                descrC,
                bsrSortedValC,
                bsrSortedRowPtrC,
                bsrSortedColIndC,
                rowBlockDimC,
                colBlockDimC,
                pBuffer,
            )
        },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseZgebsr2gebsr"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCreateIdentityPermutation(handle: cusparseHandle_t, n: ::std::os::raw::c_int, p: *mut ::std::os::raw::c_int) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCreateIdentityPermutation {
        Some(____func) => unsafe { ____func(handle, n, p) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseCreateIdentityPermutation"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseXcoosort_bufferSizeExt(handle: cusparseHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, nnz: ::std::os::raw::c_int, cooRowsA: *const ::std::os::raw::c_int, cooColsA: *const ::std::os::raw::c_int, pBufferSizeInBytes: *mut usize) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseXcoosort_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, m, n, nnz, cooRowsA, cooColsA, pBufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseXcoosort_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseXcoosortByRow(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    cooRowsA: *mut ::std::os::raw::c_int,
    cooColsA: *mut ::std::os::raw::c_int,
    P: *mut ::std::os::raw::c_int,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseXcoosortByRow {
        Some(____func) => unsafe { ____func(handle, m, n, nnz, cooRowsA, cooColsA, P, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseXcoosortByRow"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseXcoosortByColumn(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    cooRowsA: *mut ::std::os::raw::c_int,
    cooColsA: *mut ::std::os::raw::c_int,
    P: *mut ::std::os::raw::c_int,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseXcoosortByColumn {
        Some(____func) => unsafe { ____func(handle, m, n, nnz, cooRowsA, cooColsA, P, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseXcoosortByColumn"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseXcsrsort_bufferSizeExt(handle: cusparseHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, nnz: ::std::os::raw::c_int, csrRowPtrA: *const ::std::os::raw::c_int, csrColIndA: *const ::std::os::raw::c_int, pBufferSizeInBytes: *mut usize) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseXcsrsort_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, m, n, nnz, csrRowPtrA, csrColIndA, pBufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseXcsrsort_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseXcsrsort(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *mut ::std::os::raw::c_int,
    P: *mut ::std::os::raw::c_int,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseXcsrsort {
        Some(____func) => unsafe { ____func(handle, m, n, nnz, descrA, csrRowPtrA, csrColIndA, P, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseXcsrsort"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseXcscsort_bufferSizeExt(handle: cusparseHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, nnz: ::std::os::raw::c_int, cscColPtrA: *const ::std::os::raw::c_int, cscRowIndA: *const ::std::os::raw::c_int, pBufferSizeInBytes: *mut usize) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseXcscsort_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, m, n, nnz, cscColPtrA, cscRowIndA, pBufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseXcscsort_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseXcscsort(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    cscColPtrA: *const ::std::os::raw::c_int,
    cscRowIndA: *mut ::std::os::raw::c_int,
    P: *mut ::std::os::raw::c_int,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseXcscsort {
        Some(____func) => unsafe { ____func(handle, m, n, nnz, descrA, cscColPtrA, cscRowIndA, P, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseXcscsort"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseScsru2csr_bufferSizeExt(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    csrVal: *mut f32,
    csrRowPtr: *const ::std::os::raw::c_int,
    csrColInd: *mut ::std::os::raw::c_int,
    info: csru2csrInfo_t,
    pBufferSizeInBytes: *mut usize,
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseScsru2csr_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, m, n, nnz, csrVal, csrRowPtr, csrColInd, info, pBufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseScsru2csr_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDcsru2csr_bufferSizeExt(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    csrVal: *mut f64,
    csrRowPtr: *const ::std::os::raw::c_int,
    csrColInd: *mut ::std::os::raw::c_int,
    info: csru2csrInfo_t,
    pBufferSizeInBytes: *mut usize,
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDcsru2csr_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, m, n, nnz, csrVal, csrRowPtr, csrColInd, info, pBufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseDcsru2csr_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCcsru2csr_bufferSizeExt(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    csrVal: *mut cuComplex,
    csrRowPtr: *const ::std::os::raw::c_int,
    csrColInd: *mut ::std::os::raw::c_int,
    info: csru2csrInfo_t,
    pBufferSizeInBytes: *mut usize,
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCcsru2csr_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, m, n, nnz, csrVal, csrRowPtr, csrColInd, info, pBufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseCcsru2csr_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseZcsru2csr_bufferSizeExt(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    csrVal: *mut cuDoubleComplex,
    csrRowPtr: *const ::std::os::raw::c_int,
    csrColInd: *mut ::std::os::raw::c_int,
    info: csru2csrInfo_t,
    pBufferSizeInBytes: *mut usize,
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseZcsru2csr_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, m, n, nnz, csrVal, csrRowPtr, csrColInd, info, pBufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseZcsru2csr_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseScsru2csr(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseScsru2csr {
        Some(____func) => unsafe { ____func(handle, m, n, nnz, descrA, csrVal, csrRowPtr, csrColInd, info, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseScsru2csr"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDcsru2csr(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDcsru2csr {
        Some(____func) => unsafe { ____func(handle, m, n, nnz, descrA, csrVal, csrRowPtr, csrColInd, info, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseDcsru2csr"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCcsru2csr(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCcsru2csr {
        Some(____func) => unsafe { ____func(handle, m, n, nnz, descrA, csrVal, csrRowPtr, csrColInd, info, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseCcsru2csr"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseZcsru2csr(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseZcsru2csr {
        Some(____func) => unsafe { ____func(handle, m, n, nnz, descrA, csrVal, csrRowPtr, csrColInd, info, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseZcsru2csr"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseScsr2csru(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseScsr2csru {
        Some(____func) => unsafe { ____func(handle, m, n, nnz, descrA, csrVal, csrRowPtr, csrColInd, info, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseScsr2csru"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDcsr2csru(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDcsr2csru {
        Some(____func) => unsafe { ____func(handle, m, n, nnz, descrA, csrVal, csrRowPtr, csrColInd, info, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseDcsr2csru"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCcsr2csru(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCcsr2csru {
        Some(____func) => unsafe { ____func(handle, m, n, nnz, descrA, csrVal, csrRowPtr, csrColInd, info, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseCcsr2csru"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseZcsr2csru(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseZcsr2csru {
        Some(____func) => unsafe { ____func(handle, m, n, nnz, descrA, csrVal, csrRowPtr, csrColInd, info, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseZcsr2csru"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSpruneDense2csr_bufferSizeExt(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSpruneDense2csr_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, m, n, A, lda, threshold, descrC, csrSortedValC, csrSortedRowPtrC, csrSortedColIndC, pBufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseSpruneDense2csr_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDpruneDense2csr_bufferSizeExt(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDpruneDense2csr_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, m, n, A, lda, threshold, descrC, csrSortedValC, csrSortedRowPtrC, csrSortedColIndC, pBufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseDpruneDense2csr_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSpruneDense2csrNnz(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSpruneDense2csrNnz {
        Some(____func) => unsafe { ____func(handle, m, n, A, lda, threshold, descrC, csrRowPtrC, nnzTotalDevHostPtr, pBuffer) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseSpruneDense2csrNnz"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDpruneDense2csrNnz(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDpruneDense2csrNnz {
        Some(____func) => unsafe { ____func(handle, m, n, A, lda, threshold, descrC, csrSortedRowPtrC, nnzTotalDevHostPtr, pBuffer) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseDpruneDense2csrNnz"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSpruneDense2csr(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSpruneDense2csr {
        Some(____func) => unsafe { ____func(handle, m, n, A, lda, threshold, descrC, csrSortedValC, csrSortedRowPtrC, csrSortedColIndC, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseSpruneDense2csr"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDpruneDense2csr(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDpruneDense2csr {
        Some(____func) => unsafe { ____func(handle, m, n, A, lda, threshold, descrC, csrSortedValC, csrSortedRowPtrC, csrSortedColIndC, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseDpruneDense2csr"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSpruneCsr2csr_bufferSizeExt(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSpruneCsr2csr_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, m, n, nnzA, descrA, csrSortedValA, csrSortedRowPtrA, csrSortedColIndA, threshold, descrC, csrSortedValC, csrSortedRowPtrC, csrSortedColIndC, pBufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseSpruneCsr2csr_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDpruneCsr2csr_bufferSizeExt(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDpruneCsr2csr_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, m, n, nnzA, descrA, csrSortedValA, csrSortedRowPtrA, csrSortedColIndA, threshold, descrC, csrSortedValC, csrSortedRowPtrC, csrSortedColIndC, pBufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseDpruneCsr2csr_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSpruneCsr2csrNnz(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSpruneCsr2csrNnz {
        Some(____func) => unsafe { ____func(handle, m, n, nnzA, descrA, csrSortedValA, csrSortedRowPtrA, csrSortedColIndA, threshold, descrC, csrSortedRowPtrC, nnzTotalDevHostPtr, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseSpruneCsr2csrNnz"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDpruneCsr2csrNnz(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDpruneCsr2csrNnz {
        Some(____func) => unsafe { ____func(handle, m, n, nnzA, descrA, csrSortedValA, csrSortedRowPtrA, csrSortedColIndA, threshold, descrC, csrSortedRowPtrC, nnzTotalDevHostPtr, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseDpruneCsr2csrNnz"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSpruneCsr2csr(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSpruneCsr2csr {
        Some(____func) => unsafe { ____func(handle, m, n, nnzA, descrA, csrSortedValA, csrSortedRowPtrA, csrSortedColIndA, threshold, descrC, csrSortedValC, csrSortedRowPtrC, csrSortedColIndC, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseSpruneCsr2csr"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDpruneCsr2csr(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDpruneCsr2csr {
        Some(____func) => unsafe { ____func(handle, m, n, nnzA, descrA, csrSortedValA, csrSortedRowPtrA, csrSortedColIndA, threshold, descrC, csrSortedValC, csrSortedRowPtrC, csrSortedColIndC, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseDpruneCsr2csr"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSpruneDense2csrByPercentage_bufferSizeExt(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSpruneDense2csrByPercentage_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, m, n, A, lda, percentage, descrC, csrSortedValC, csrSortedRowPtrC, csrSortedColIndC, info, pBufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseSpruneDense2csrByPercentage_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDpruneDense2csrByPercentage_bufferSizeExt(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDpruneDense2csrByPercentage_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, m, n, A, lda, percentage, descrC, csrSortedValC, csrSortedRowPtrC, csrSortedColIndC, info, pBufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseDpruneDense2csrByPercentage_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSpruneDense2csrNnzByPercentage(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSpruneDense2csrNnzByPercentage {
        Some(____func) => unsafe { ____func(handle, m, n, A, lda, percentage, descrC, csrRowPtrC, nnzTotalDevHostPtr, info, pBuffer) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseSpruneDense2csrNnzByPercentage"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDpruneDense2csrNnzByPercentage(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDpruneDense2csrNnzByPercentage {
        Some(____func) => unsafe { ____func(handle, m, n, A, lda, percentage, descrC, csrRowPtrC, nnzTotalDevHostPtr, info, pBuffer) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseDpruneDense2csrNnzByPercentage"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSpruneDense2csrByPercentage(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSpruneDense2csrByPercentage {
        Some(____func) => unsafe { ____func(handle, m, n, A, lda, percentage, descrC, csrSortedValC, csrSortedRowPtrC, csrSortedColIndC, info, pBuffer) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseSpruneDense2csrByPercentage"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDpruneDense2csrByPercentage(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDpruneDense2csrByPercentage {
        Some(____func) => unsafe { ____func(handle, m, n, A, lda, percentage, descrC, csrSortedValC, csrSortedRowPtrC, csrSortedColIndC, info, pBuffer) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseDpruneDense2csrByPercentage"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSpruneCsr2csrByPercentage_bufferSizeExt(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSpruneCsr2csrByPercentage_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, m, n, nnzA, descrA, csrSortedValA, csrSortedRowPtrA, csrSortedColIndA, percentage, descrC, csrSortedValC, csrSortedRowPtrC, csrSortedColIndC, info, pBufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseSpruneCsr2csrByPercentage_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDpruneCsr2csrByPercentage_bufferSizeExt(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDpruneCsr2csrByPercentage_bufferSizeExt {
        Some(____func) => unsafe { ____func(handle, m, n, nnzA, descrA, csrSortedValA, csrSortedRowPtrA, csrSortedColIndA, percentage, descrC, csrSortedValC, csrSortedRowPtrC, csrSortedColIndC, info, pBufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseDpruneCsr2csrByPercentage_bufferSizeExt"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSpruneCsr2csrNnzByPercentage(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSpruneCsr2csrNnzByPercentage {
        Some(____func) => unsafe { ____func(handle, m, n, nnzA, descrA, csrSortedValA, csrSortedRowPtrA, csrSortedColIndA, percentage, descrC, csrSortedRowPtrC, nnzTotalDevHostPtr, info, pBuffer) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseSpruneCsr2csrNnzByPercentage"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDpruneCsr2csrNnzByPercentage(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDpruneCsr2csrNnzByPercentage {
        Some(____func) => unsafe { ____func(handle, m, n, nnzA, descrA, csrSortedValA, csrSortedRowPtrA, csrSortedColIndA, percentage, descrC, csrSortedRowPtrC, nnzTotalDevHostPtr, info, pBuffer) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseDpruneCsr2csrNnzByPercentage"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSpruneCsr2csrByPercentage(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSpruneCsr2csrByPercentage {
        Some(____func) => unsafe { ____func(handle, m, n, nnzA, descrA, csrSortedValA, csrSortedRowPtrA, csrSortedColIndA, percentage, descrC, csrSortedValC, csrSortedRowPtrC, csrSortedColIndC, info, pBuffer) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseSpruneCsr2csrByPercentage"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDpruneCsr2csrByPercentage(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDpruneCsr2csrByPercentage {
        Some(____func) => unsafe { ____func(handle, m, n, nnzA, descrA, csrSortedValA, csrSortedRowPtrA, csrSortedColIndA, percentage, descrC, csrSortedValC, csrSortedRowPtrC, csrSortedColIndC, info, pBuffer) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseDpruneCsr2csrByPercentage"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCsr2cscEx2(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCsr2cscEx2 {
        Some(____func) => unsafe { ____func(handle, m, n, nnz, csrVal, csrRowPtr, csrColInd, cscVal, cscColPtr, cscRowInd, valType, copyValues, idxBase, alg, buffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseCsr2cscEx2"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCsr2cscEx2_bufferSize(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCsr2cscEx2_bufferSize {
        Some(____func) => unsafe { ____func(handle, m, n, nnz, csrVal, csrRowPtr, csrColInd, cscVal, cscColPtr, cscRowInd, valType, copyValues, idxBase, alg, bufferSize) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseCsr2cscEx2_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCreateSpVec(spVecDescr: *mut cusparseSpVecDescr_t, size: i64, nnz: i64, indices: *mut ::std::os::raw::c_void, values: *mut ::std::os::raw::c_void, idxType: cusparseIndexType_t, idxBase: cusparseIndexBase_t, valueType: cudaDataType) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCreateSpVec {
        Some(____func) => unsafe { ____func(spVecDescr, size, nnz, indices, values, idxType, idxBase, valueType) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseCreateSpVec"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCreateConstSpVec(spVecDescr: *mut cusparseConstSpVecDescr_t, size: i64, nnz: i64, indices: *const ::std::os::raw::c_void, values: *const ::std::os::raw::c_void, idxType: cusparseIndexType_t, idxBase: cusparseIndexBase_t, valueType: cudaDataType) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCreateConstSpVec {
        Some(____func) => unsafe { ____func(spVecDescr, size, nnz, indices, values, idxType, idxBase, valueType) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseCreateConstSpVec"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDestroySpVec(spVecDescr: cusparseConstSpVecDescr_t) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDestroySpVec {
        Some(____func) => unsafe { ____func(spVecDescr) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseDestroySpVec"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSpVecGet(
    spVecDescr: cusparseSpVecDescr_t,
    size: *mut i64,
    nnz: *mut i64,
    indices: *mut *mut ::std::os::raw::c_void,
    values: *mut *mut ::std::os::raw::c_void,
    idxType: *mut cusparseIndexType_t,
    idxBase: *mut cusparseIndexBase_t,
    valueType: *mut cudaDataType,
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSpVecGet {
        Some(____func) => unsafe { ____func(spVecDescr, size, nnz, indices, values, idxType, idxBase, valueType) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseSpVecGet"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseConstSpVecGet(
    spVecDescr: cusparseConstSpVecDescr_t,
    size: *mut i64,
    nnz: *mut i64,
    indices: *mut *const ::std::os::raw::c_void,
    values: *mut *const ::std::os::raw::c_void,
    idxType: *mut cusparseIndexType_t,
    idxBase: *mut cusparseIndexBase_t,
    valueType: *mut cudaDataType,
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseConstSpVecGet {
        Some(____func) => unsafe { ____func(spVecDescr, size, nnz, indices, values, idxType, idxBase, valueType) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseConstSpVecGet"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSpVecGetIndexBase(spVecDescr: cusparseConstSpVecDescr_t, idxBase: *mut cusparseIndexBase_t) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSpVecGetIndexBase {
        Some(____func) => unsafe { ____func(spVecDescr, idxBase) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseSpVecGetIndexBase"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSpVecGetValues(spVecDescr: cusparseSpVecDescr_t, values: *mut *mut ::std::os::raw::c_void) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSpVecGetValues {
        Some(____func) => unsafe { ____func(spVecDescr, values) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseSpVecGetValues"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseConstSpVecGetValues(spVecDescr: cusparseConstSpVecDescr_t, values: *mut *const ::std::os::raw::c_void) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseConstSpVecGetValues {
        Some(____func) => unsafe { ____func(spVecDescr, values) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseConstSpVecGetValues"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSpVecSetValues(spVecDescr: cusparseSpVecDescr_t, values: *mut ::std::os::raw::c_void) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSpVecSetValues {
        Some(____func) => unsafe { ____func(spVecDescr, values) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseSpVecSetValues"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCreateDnVec(dnVecDescr: *mut cusparseDnVecDescr_t, size: i64, values: *mut ::std::os::raw::c_void, valueType: cudaDataType) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCreateDnVec {
        Some(____func) => unsafe { ____func(dnVecDescr, size, values, valueType) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseCreateDnVec"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCreateConstDnVec(dnVecDescr: *mut cusparseConstDnVecDescr_t, size: i64, values: *const ::std::os::raw::c_void, valueType: cudaDataType) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCreateConstDnVec {
        Some(____func) => unsafe { ____func(dnVecDescr, size, values, valueType) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseCreateConstDnVec"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDestroyDnVec(dnVecDescr: cusparseConstDnVecDescr_t) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDestroyDnVec {
        Some(____func) => unsafe { ____func(dnVecDescr) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseDestroyDnVec"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDnVecGet(dnVecDescr: cusparseDnVecDescr_t, size: *mut i64, values: *mut *mut ::std::os::raw::c_void, valueType: *mut cudaDataType) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDnVecGet {
        Some(____func) => unsafe { ____func(dnVecDescr, size, values, valueType) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseDnVecGet"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseConstDnVecGet(dnVecDescr: cusparseConstDnVecDescr_t, size: *mut i64, values: *mut *const ::std::os::raw::c_void, valueType: *mut cudaDataType) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseConstDnVecGet {
        Some(____func) => unsafe { ____func(dnVecDescr, size, values, valueType) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseConstDnVecGet"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDnVecGetValues(dnVecDescr: cusparseDnVecDescr_t, values: *mut *mut ::std::os::raw::c_void) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDnVecGetValues {
        Some(____func) => unsafe { ____func(dnVecDescr, values) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseDnVecGetValues"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseConstDnVecGetValues(dnVecDescr: cusparseConstDnVecDescr_t, values: *mut *const ::std::os::raw::c_void) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseConstDnVecGetValues {
        Some(____func) => unsafe { ____func(dnVecDescr, values) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseConstDnVecGetValues"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDnVecSetValues(dnVecDescr: cusparseDnVecDescr_t, values: *mut ::std::os::raw::c_void) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDnVecSetValues {
        Some(____func) => unsafe { ____func(dnVecDescr, values) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseDnVecSetValues"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDestroySpMat(spMatDescr: cusparseConstSpMatDescr_t) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDestroySpMat {
        Some(____func) => unsafe { ____func(spMatDescr) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseDestroySpMat"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSpMatGetFormat(spMatDescr: cusparseConstSpMatDescr_t, format: *mut cusparseFormat_t) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSpMatGetFormat {
        Some(____func) => unsafe { ____func(spMatDescr, format) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseSpMatGetFormat"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSpMatGetIndexBase(spMatDescr: cusparseConstSpMatDescr_t, idxBase: *mut cusparseIndexBase_t) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSpMatGetIndexBase {
        Some(____func) => unsafe { ____func(spMatDescr, idxBase) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseSpMatGetIndexBase"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSpMatGetValues(spMatDescr: cusparseSpMatDescr_t, values: *mut *mut ::std::os::raw::c_void) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSpMatGetValues {
        Some(____func) => unsafe { ____func(spMatDescr, values) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseSpMatGetValues"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseConstSpMatGetValues(spMatDescr: cusparseConstSpMatDescr_t, values: *mut *const ::std::os::raw::c_void) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseConstSpMatGetValues {
        Some(____func) => unsafe { ____func(spMatDescr, values) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseConstSpMatGetValues"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSpMatSetValues(spMatDescr: cusparseSpMatDescr_t, values: *mut ::std::os::raw::c_void) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSpMatSetValues {
        Some(____func) => unsafe { ____func(spMatDescr, values) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseSpMatSetValues"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSpMatGetSize(spMatDescr: cusparseConstSpMatDescr_t, rows: *mut i64, cols: *mut i64, nnz: *mut i64) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSpMatGetSize {
        Some(____func) => unsafe { ____func(spMatDescr, rows, cols, nnz) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseSpMatGetSize"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSpMatGetStridedBatch(spMatDescr: cusparseConstSpMatDescr_t, batchCount: *mut ::std::os::raw::c_int) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSpMatGetStridedBatch {
        Some(____func) => unsafe { ____func(spMatDescr, batchCount) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseSpMatGetStridedBatch"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCooSetStridedBatch(spMatDescr: cusparseSpMatDescr_t, batchCount: ::std::os::raw::c_int, batchStride: i64) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCooSetStridedBatch {
        Some(____func) => unsafe { ____func(spMatDescr, batchCount, batchStride) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseCooSetStridedBatch"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCsrSetStridedBatch(spMatDescr: cusparseSpMatDescr_t, batchCount: ::std::os::raw::c_int, offsetsBatchStride: i64, columnsValuesBatchStride: i64) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCsrSetStridedBatch {
        Some(____func) => unsafe { ____func(spMatDescr, batchCount, offsetsBatchStride, columnsValuesBatchStride) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseCsrSetStridedBatch"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseBsrSetStridedBatch(spMatDescr: cusparseSpMatDescr_t, batchCount: ::std::os::raw::c_int, offsetsBatchStride: i64, columnsBatchStride: i64, ValuesBatchStride: i64) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseBsrSetStridedBatch {
        Some(____func) => unsafe { ____func(spMatDescr, batchCount, offsetsBatchStride, columnsBatchStride, ValuesBatchStride) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseBsrSetStridedBatch"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSpMatGetAttribute(spMatDescr: cusparseConstSpMatDescr_t, attribute: cusparseSpMatAttribute_t, data: *mut ::std::os::raw::c_void, dataSize: usize) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSpMatGetAttribute {
        Some(____func) => unsafe { ____func(spMatDescr, attribute, data, dataSize) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseSpMatGetAttribute"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSpMatSetAttribute(spMatDescr: cusparseSpMatDescr_t, attribute: cusparseSpMatAttribute_t, data: *mut ::std::os::raw::c_void, dataSize: usize) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSpMatSetAttribute {
        Some(____func) => unsafe { ____func(spMatDescr, attribute, data, dataSize) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseSpMatSetAttribute"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCreateCsr(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCreateCsr {
        Some(____func) => unsafe { ____func(spMatDescr, rows, cols, nnz, csrRowOffsets, csrColInd, csrValues, csrRowOffsetsType, csrColIndType, idxBase, valueType) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseCreateCsr"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCreateConstCsr(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCreateConstCsr {
        Some(____func) => unsafe { ____func(spMatDescr, rows, cols, nnz, csrRowOffsets, csrColInd, csrValues, csrRowOffsetsType, csrColIndType, idxBase, valueType) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseCreateConstCsr"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCreateCsc(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCreateCsc {
        Some(____func) => unsafe { ____func(spMatDescr, rows, cols, nnz, cscColOffsets, cscRowInd, cscValues, cscColOffsetsType, cscRowIndType, idxBase, valueType) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseCreateCsc"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCreateConstCsc(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCreateConstCsc {
        Some(____func) => unsafe { ____func(spMatDescr, rows, cols, nnz, cscColOffsets, cscRowInd, cscValues, cscColOffsetsType, cscRowIndType, idxBase, valueType) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseCreateConstCsc"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCsrGet(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCsrGet {
        Some(____func) => unsafe { ____func(spMatDescr, rows, cols, nnz, csrRowOffsets, csrColInd, csrValues, csrRowOffsetsType, csrColIndType, idxBase, valueType) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseCsrGet"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseConstCsrGet(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseConstCsrGet {
        Some(____func) => unsafe { ____func(spMatDescr, rows, cols, nnz, csrRowOffsets, csrColInd, csrValues, csrRowOffsetsType, csrColIndType, idxBase, valueType) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseConstCsrGet"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCscGet(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCscGet {
        Some(____func) => unsafe { ____func(spMatDescr, rows, cols, nnz, cscColOffsets, cscRowInd, cscValues, cscColOffsetsType, cscRowIndType, idxBase, valueType) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseCscGet"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseConstCscGet(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseConstCscGet {
        Some(____func) => unsafe { ____func(spMatDescr, rows, cols, nnz, cscColOffsets, cscRowInd, cscValues, cscColOffsetsType, cscRowIndType, idxBase, valueType) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseConstCscGet"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCsrSetPointers(spMatDescr: cusparseSpMatDescr_t, csrRowOffsets: *mut ::std::os::raw::c_void, csrColInd: *mut ::std::os::raw::c_void, csrValues: *mut ::std::os::raw::c_void) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCsrSetPointers {
        Some(____func) => unsafe { ____func(spMatDescr, csrRowOffsets, csrColInd, csrValues) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseCsrSetPointers"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCscSetPointers(spMatDescr: cusparseSpMatDescr_t, cscColOffsets: *mut ::std::os::raw::c_void, cscRowInd: *mut ::std::os::raw::c_void, cscValues: *mut ::std::os::raw::c_void) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCscSetPointers {
        Some(____func) => unsafe { ____func(spMatDescr, cscColOffsets, cscRowInd, cscValues) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseCscSetPointers"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCreateBsr(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCreateBsr {
        Some(____func) => unsafe { ____func(spMatDescr, brows, bcols, bnnz, rowBlockSize, colBlockSize, bsrRowOffsets, bsrColInd, bsrValues, bsrRowOffsetsType, bsrColIndType, idxBase, valueType, order) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseCreateBsr"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCreateConstBsr(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCreateConstBsr {
        Some(____func) => unsafe { ____func(spMatDescr, brows, bcols, bnnz, rowBlockDim, colBlockDim, bsrRowOffsets, bsrColInd, bsrValues, bsrRowOffsetsType, bsrColIndType, idxBase, valueType, order) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseCreateConstBsr"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCreateCoo(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCreateCoo {
        Some(____func) => unsafe { ____func(spMatDescr, rows, cols, nnz, cooRowInd, cooColInd, cooValues, cooIdxType, idxBase, valueType) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseCreateCoo"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCreateConstCoo(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCreateConstCoo {
        Some(____func) => unsafe { ____func(spMatDescr, rows, cols, nnz, cooRowInd, cooColInd, cooValues, cooIdxType, idxBase, valueType) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseCreateConstCoo"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCooGet(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCooGet {
        Some(____func) => unsafe { ____func(spMatDescr, rows, cols, nnz, cooRowInd, cooColInd, cooValues, idxType, idxBase, valueType) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseCooGet"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseConstCooGet(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseConstCooGet {
        Some(____func) => unsafe { ____func(spMatDescr, rows, cols, nnz, cooRowInd, cooColInd, cooValues, idxType, idxBase, valueType) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseConstCooGet"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCooSetPointers(spMatDescr: cusparseSpMatDescr_t, cooRows: *mut ::std::os::raw::c_void, cooColumns: *mut ::std::os::raw::c_void, cooValues: *mut ::std::os::raw::c_void) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCooSetPointers {
        Some(____func) => unsafe { ____func(spMatDescr, cooRows, cooColumns, cooValues) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseCooSetPointers"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCreateBlockedEll(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCreateBlockedEll {
        Some(____func) => unsafe { ____func(spMatDescr, rows, cols, ellBlockSize, ellCols, ellColInd, ellValue, ellIdxType, idxBase, valueType) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseCreateBlockedEll"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCreateConstBlockedEll(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCreateConstBlockedEll {
        Some(____func) => unsafe { ____func(spMatDescr, rows, cols, ellBlockSize, ellCols, ellColInd, ellValue, ellIdxType, idxBase, valueType) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseCreateConstBlockedEll"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseBlockedEllGet(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseBlockedEllGet {
        Some(____func) => unsafe { ____func(spMatDescr, rows, cols, ellBlockSize, ellCols, ellColInd, ellValue, ellIdxType, idxBase, valueType) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseBlockedEllGet"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseConstBlockedEllGet(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseConstBlockedEllGet {
        Some(____func) => unsafe { ____func(spMatDescr, rows, cols, ellBlockSize, ellCols, ellColInd, ellValue, ellIdxType, idxBase, valueType) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseConstBlockedEllGet"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCreateSlicedEll(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCreateSlicedEll {
        Some(____func) => unsafe { ____func(spMatDescr, rows, cols, nnz, sellValuesSize, sliceSize, sellSliceOffsets, sellColInd, sellValues, sellSliceOffsetsType, sellColIndType, idxBase, valueType) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseCreateSlicedEll"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCreateConstSlicedEll(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCreateConstSlicedEll {
        Some(____func) => unsafe { ____func(spMatDescr, rows, cols, nnz, sellValuesSize, sliceSize, sellSliceOffsets, sellColInd, sellValues, sellSliceOffsetsType, sellColIndType, idxBase, valueType) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseCreateConstSlicedEll"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCreateDnMat(dnMatDescr: *mut cusparseDnMatDescr_t, rows: i64, cols: i64, ld: i64, values: *mut ::std::os::raw::c_void, valueType: cudaDataType, order: cusparseOrder_t) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCreateDnMat {
        Some(____func) => unsafe { ____func(dnMatDescr, rows, cols, ld, values, valueType, order) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseCreateDnMat"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseCreateConstDnMat(dnMatDescr: *mut cusparseConstDnMatDescr_t, rows: i64, cols: i64, ld: i64, values: *const ::std::os::raw::c_void, valueType: cudaDataType, order: cusparseOrder_t) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseCreateConstDnMat {
        Some(____func) => unsafe { ____func(dnMatDescr, rows, cols, ld, values, valueType, order) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseCreateConstDnMat"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDestroyDnMat(dnMatDescr: cusparseConstDnMatDescr_t) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDestroyDnMat {
        Some(____func) => unsafe { ____func(dnMatDescr) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseDestroyDnMat"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDnMatGet(dnMatDescr: cusparseDnMatDescr_t, rows: *mut i64, cols: *mut i64, ld: *mut i64, values: *mut *mut ::std::os::raw::c_void, type_: *mut cudaDataType, order: *mut cusparseOrder_t) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDnMatGet {
        Some(____func) => unsafe { ____func(dnMatDescr, rows, cols, ld, values, type_, order) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseDnMatGet"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseConstDnMatGet(dnMatDescr: cusparseConstDnMatDescr_t, rows: *mut i64, cols: *mut i64, ld: *mut i64, values: *mut *const ::std::os::raw::c_void, type_: *mut cudaDataType, order: *mut cusparseOrder_t) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseConstDnMatGet {
        Some(____func) => unsafe { ____func(dnMatDescr, rows, cols, ld, values, type_, order) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseConstDnMatGet"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDnMatGetValues(dnMatDescr: cusparseDnMatDescr_t, values: *mut *mut ::std::os::raw::c_void) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDnMatGetValues {
        Some(____func) => unsafe { ____func(dnMatDescr, values) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseDnMatGetValues"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseConstDnMatGetValues(dnMatDescr: cusparseConstDnMatDescr_t, values: *mut *const ::std::os::raw::c_void) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseConstDnMatGetValues {
        Some(____func) => unsafe { ____func(dnMatDescr, values) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseConstDnMatGetValues"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDnMatSetValues(dnMatDescr: cusparseDnMatDescr_t, values: *mut ::std::os::raw::c_void) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDnMatSetValues {
        Some(____func) => unsafe { ____func(dnMatDescr, values) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseDnMatSetValues"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDnMatSetStridedBatch(dnMatDescr: cusparseDnMatDescr_t, batchCount: ::std::os::raw::c_int, batchStride: i64) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDnMatSetStridedBatch {
        Some(____func) => unsafe { ____func(dnMatDescr, batchCount, batchStride) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseDnMatSetStridedBatch"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDnMatGetStridedBatch(dnMatDescr: cusparseConstDnMatDescr_t, batchCount: *mut ::std::os::raw::c_int, batchStride: *mut i64) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDnMatGetStridedBatch {
        Some(____func) => unsafe { ____func(dnMatDescr, batchCount, batchStride) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseDnMatGetStridedBatch"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseAxpby(handle: cusparseHandle_t, alpha: *const ::std::os::raw::c_void, vecX: cusparseConstSpVecDescr_t, beta: *const ::std::os::raw::c_void, vecY: cusparseDnVecDescr_t) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseAxpby {
        Some(____func) => unsafe { ____func(handle, alpha, vecX, beta, vecY) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseAxpby"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseGather(handle: cusparseHandle_t, vecY: cusparseConstDnVecDescr_t, vecX: cusparseSpVecDescr_t) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseGather {
        Some(____func) => unsafe { ____func(handle, vecY, vecX) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseGather"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseScatter(handle: cusparseHandle_t, vecX: cusparseConstSpVecDescr_t, vecY: cusparseDnVecDescr_t) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseScatter {
        Some(____func) => unsafe { ____func(handle, vecX, vecY) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseScatter"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseRot(handle: cusparseHandle_t, c_coeff: *const ::std::os::raw::c_void, s_coeff: *const ::std::os::raw::c_void, vecX: cusparseSpVecDescr_t, vecY: cusparseDnVecDescr_t) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseRot {
        Some(____func) => unsafe { ____func(handle, c_coeff, s_coeff, vecX, vecY) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseRot"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSpVV_bufferSize(handle: cusparseHandle_t, opX: cusparseOperation_t, vecX: cusparseConstSpVecDescr_t, vecY: cusparseConstDnVecDescr_t, result: *const ::std::os::raw::c_void, computeType: cudaDataType, bufferSize: *mut usize) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSpVV_bufferSize {
        Some(____func) => unsafe { ____func(handle, opX, vecX, vecY, result, computeType, bufferSize) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseSpVV_bufferSize"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSpVV(handle: cusparseHandle_t, opX: cusparseOperation_t, vecX: cusparseConstSpVecDescr_t, vecY: cusparseConstDnVecDescr_t, result: *mut ::std::os::raw::c_void, computeType: cudaDataType, externalBuffer: *mut ::std::os::raw::c_void) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSpVV {
        Some(____func) => unsafe { ____func(handle, opX, vecX, vecY, result, computeType, externalBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseSpVV"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSparseToDense_bufferSize(handle: cusparseHandle_t, matA: cusparseConstSpMatDescr_t, matB: cusparseDnMatDescr_t, alg: cusparseSparseToDenseAlg_t, bufferSize: *mut usize) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSparseToDense_bufferSize {
        Some(____func) => unsafe { ____func(handle, matA, matB, alg, bufferSize) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseSparseToDense_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSparseToDense(handle: cusparseHandle_t, matA: cusparseConstSpMatDescr_t, matB: cusparseDnMatDescr_t, alg: cusparseSparseToDenseAlg_t, externalBuffer: *mut ::std::os::raw::c_void) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSparseToDense {
        Some(____func) => unsafe { ____func(handle, matA, matB, alg, externalBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseSparseToDense"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDenseToSparse_bufferSize(handle: cusparseHandle_t, matA: cusparseConstDnMatDescr_t, matB: cusparseSpMatDescr_t, alg: cusparseDenseToSparseAlg_t, bufferSize: *mut usize) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDenseToSparse_bufferSize {
        Some(____func) => unsafe { ____func(handle, matA, matB, alg, bufferSize) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseDenseToSparse_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDenseToSparse_analysis(handle: cusparseHandle_t, matA: cusparseConstDnMatDescr_t, matB: cusparseSpMatDescr_t, alg: cusparseDenseToSparseAlg_t, externalBuffer: *mut ::std::os::raw::c_void) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDenseToSparse_analysis {
        Some(____func) => unsafe { ____func(handle, matA, matB, alg, externalBuffer) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseDenseToSparse_analysis"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseDenseToSparse_convert(handle: cusparseHandle_t, matA: cusparseConstDnMatDescr_t, matB: cusparseSpMatDescr_t, alg: cusparseDenseToSparseAlg_t, externalBuffer: *mut ::std::os::raw::c_void) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseDenseToSparse_convert {
        Some(____func) => unsafe { ____func(handle, matA, matB, alg, externalBuffer) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseDenseToSparse_convert"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSpMV(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSpMV {
        Some(____func) => unsafe { ____func(handle, opA, alpha, matA, vecX, beta, vecY, computeType, alg, externalBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseSpMV"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSpMV_bufferSize(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSpMV_bufferSize {
        Some(____func) => unsafe { ____func(handle, opA, alpha, matA, vecX, beta, vecY, computeType, alg, bufferSize) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseSpMV_bufferSize"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSpMV_preprocess(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSpMV_preprocess {
        Some(____func) => unsafe { ____func(handle, opA, alpha, matA, vecX, beta, vecY, computeType, alg, externalBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseSpMV_preprocess"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSpSV_createDescr(descr: *mut cusparseSpSVDescr_t) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSpSV_createDescr {
        Some(____func) => unsafe { ____func(descr) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseSpSV_createDescr"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSpSV_destroyDescr(descr: cusparseSpSVDescr_t) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSpSV_destroyDescr {
        Some(____func) => unsafe { ____func(descr) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseSpSV_destroyDescr"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSpSV_bufferSize(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSpSV_bufferSize {
        Some(____func) => unsafe { ____func(handle, opA, alpha, matA, vecX, vecY, computeType, alg, spsvDescr, bufferSize) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseSpSV_bufferSize"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSpSV_analysis(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSpSV_analysis {
        Some(____func) => unsafe { ____func(handle, opA, alpha, matA, vecX, vecY, computeType, alg, spsvDescr, externalBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseSpSV_analysis"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSpSV_solve(
    handle: cusparseHandle_t,
    opA: cusparseOperation_t,
    alpha: *const ::std::os::raw::c_void,
    matA: cusparseConstSpMatDescr_t,
    vecX: cusparseConstDnVecDescr_t,
    vecY: cusparseDnVecDescr_t,
    computeType: cudaDataType,
    alg: cusparseSpSVAlg_t,
    spsvDescr: cusparseSpSVDescr_t,
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSpSV_solve {
        Some(____func) => unsafe { ____func(handle, opA, alpha, matA, vecX, vecY, computeType, alg, spsvDescr) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseSpSV_solve"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSpSV_updateMatrix(handle: cusparseHandle_t, spsvDescr: cusparseSpSVDescr_t, newValues: *mut ::std::os::raw::c_void, updatePart: cusparseSpSVUpdate_t) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSpSV_updateMatrix {
        Some(____func) => unsafe { ____func(handle, spsvDescr, newValues, updatePart) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseSpSV_updateMatrix"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSpSM_createDescr(descr: *mut cusparseSpSMDescr_t) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSpSM_createDescr {
        Some(____func) => unsafe { ____func(descr) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseSpSM_createDescr"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSpSM_destroyDescr(descr: cusparseSpSMDescr_t) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSpSM_destroyDescr {
        Some(____func) => unsafe { ____func(descr) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseSpSM_destroyDescr"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSpSM_bufferSize(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSpSM_bufferSize {
        Some(____func) => unsafe { ____func(handle, opA, opB, alpha, matA, matB, matC, computeType, alg, spsmDescr, bufferSize) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseSpSM_bufferSize"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSpSM_analysis(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSpSM_analysis {
        Some(____func) => unsafe { ____func(handle, opA, opB, alpha, matA, matB, matC, computeType, alg, spsmDescr, externalBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseSpSM_analysis"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSpSM_solve(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSpSM_solve {
        Some(____func) => unsafe { ____func(handle, opA, opB, alpha, matA, matB, matC, computeType, alg, spsmDescr) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseSpSM_solve"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSpSM_updateMatrix(handle: cusparseHandle_t, spsmDescr: cusparseSpSMDescr_t, newValues: *mut ::std::os::raw::c_void, updatePart: cusparseSpSMUpdate_t) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSpSM_updateMatrix {
        Some(____func) => unsafe { ____func(handle, spsmDescr, newValues, updatePart) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseSpSM_updateMatrix"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSpMM_bufferSize(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSpMM_bufferSize {
        Some(____func) => unsafe { ____func(handle, opA, opB, alpha, matA, matB, beta, matC, computeType, alg, bufferSize) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseSpMM_bufferSize"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSpMM_preprocess(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSpMM_preprocess {
        Some(____func) => unsafe { ____func(handle, opA, opB, alpha, matA, matB, beta, matC, computeType, alg, externalBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseSpMM_preprocess"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSpMM(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSpMM {
        Some(____func) => unsafe { ____func(handle, opA, opB, alpha, matA, matB, beta, matC, computeType, alg, externalBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseSpMM"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSpGEMM_createDescr(descr: *mut cusparseSpGEMMDescr_t) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSpGEMM_createDescr {
        Some(____func) => unsafe { ____func(descr) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseSpGEMM_createDescr"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSpGEMM_destroyDescr(descr: cusparseSpGEMMDescr_t) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSpGEMM_destroyDescr {
        Some(____func) => unsafe { ____func(descr) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseSpGEMM_destroyDescr"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSpGEMM_workEstimation(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSpGEMM_workEstimation {
        Some(____func) => unsafe { ____func(handle, opA, opB, alpha, matA, matB, beta, matC, computeType, alg, spgemmDescr, bufferSize1, externalBuffer1) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseSpGEMM_workEstimation"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSpGEMM_getNumProducts(spgemmDescr: cusparseSpGEMMDescr_t, num_prods: *mut i64) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSpGEMM_getNumProducts {
        Some(____func) => unsafe { ____func(spgemmDescr, num_prods) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseSpGEMM_getNumProducts"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSpGEMM_estimateMemory(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSpGEMM_estimateMemory {
        Some(____func) => unsafe { ____func(handle, opA, opB, alpha, matA, matB, beta, matC, computeType, alg, spgemmDescr, chunk_fraction, bufferSize3, externalBuffer3, bufferSize2) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseSpGEMM_estimateMemory"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSpGEMM_compute(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSpGEMM_compute {
        Some(____func) => unsafe { ____func(handle, opA, opB, alpha, matA, matB, beta, matC, computeType, alg, spgemmDescr, bufferSize2, externalBuffer2) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseSpGEMM_compute"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSpGEMM_copy(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSpGEMM_copy {
        Some(____func) => unsafe { ____func(handle, opA, opB, alpha, matA, matB, beta, matC, computeType, alg, spgemmDescr) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseSpGEMM_copy"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSpGEMMreuse_workEstimation(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSpGEMMreuse_workEstimation {
        Some(____func) => unsafe { ____func(handle, opA, opB, matA, matB, matC, alg, spgemmDescr, bufferSize1, externalBuffer1) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseSpGEMMreuse_workEstimation"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSpGEMMreuse_nnz(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSpGEMMreuse_nnz {
        Some(____func) => unsafe { ____func(handle, opA, opB, matA, matB, matC, alg, spgemmDescr, bufferSize2, externalBuffer2, bufferSize3, externalBuffer3, bufferSize4, externalBuffer4) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseSpGEMMreuse_nnz"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSpGEMMreuse_copy(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSpGEMMreuse_copy {
        Some(____func) => unsafe { ____func(handle, opA, opB, matA, matB, matC, alg, spgemmDescr, bufferSize5, externalBuffer5) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseSpGEMMreuse_copy"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSpGEMMreuse_compute(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSpGEMMreuse_compute {
        Some(____func) => unsafe { ____func(handle, opA, opB, alpha, matA, matB, beta, matC, computeType, alg, spgemmDescr) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseSpGEMMreuse_compute"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSDDMM_bufferSize(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSDDMM_bufferSize {
        Some(____func) => unsafe { ____func(handle, opA, opB, alpha, matA, matB, beta, matC, computeType, alg, bufferSize) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseSDDMM_bufferSize"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSDDMM_preprocess(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSDDMM_preprocess {
        Some(____func) => unsafe { ____func(handle, opA, opB, alpha, matA, matB, beta, matC, computeType, alg, externalBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseSDDMM_preprocess"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSDDMM(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSDDMM {
        Some(____func) => unsafe { ____func(handle, opA, opB, alpha, matA, matB, beta, matC, computeType, alg, externalBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseSDDMM"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSpMMOp_createPlan(
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
) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSpMMOp_createPlan {
        Some(____func) => unsafe {
            ____func(
                handle,
                plan,
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
                SpMMWorkspaceSize,
            )
        },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseSpMMOp_createPlan"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSpMMOp(plan: cusparseSpMMOpPlan_t, externalBuffer: *mut ::std::os::raw::c_void) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSpMMOp {
        Some(____func) => unsafe { ____func(plan, externalBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusparseSpMMOp"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusparseSpMMOp_destroyPlan(plan: cusparseSpMMOpPlan_t) -> cusparseStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusparseSpMMOp_destroyPlan {
        Some(____func) => unsafe { ____func(plan) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusparseSpMMOp_destroyPlan"
        ),
    }
}
#[cfg(feature = "runtime-link")]
pub unsafe fn load_dynamic_bindings(lib: *mut std::ffi::c_void, get_proc_addr: unsafe fn(*mut std::ffi::c_void, *const u8) -> *mut std::ffi::c_void) {
    let bindings = unsafe {
        Box::new(DynamicBindings {
            cusparseCreate: {
                let p = get_proc_addr(lib, b"cusparseCreate\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDestroy: {
                let p = get_proc_addr(lib, b"cusparseDestroy\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseGetVersion: {
                let p = get_proc_addr(lib, b"cusparseGetVersion\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseGetProperty: {
                let p = get_proc_addr(lib, b"cusparseGetProperty\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseGetErrorName: {
                let p = get_proc_addr(lib, b"cusparseGetErrorName\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseGetErrorString: {
                let p = get_proc_addr(lib, b"cusparseGetErrorString\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSetStream: {
                let p = get_proc_addr(lib, b"cusparseSetStream\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseGetStream: {
                let p = get_proc_addr(lib, b"cusparseGetStream\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseGetPointerMode: {
                let p = get_proc_addr(lib, b"cusparseGetPointerMode\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSetPointerMode: {
                let p = get_proc_addr(lib, b"cusparseSetPointerMode\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseLoggerSetCallback: {
                let p = get_proc_addr(lib, b"cusparseLoggerSetCallback\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseLoggerSetFile: {
                let p = get_proc_addr(lib, b"cusparseLoggerSetFile\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseLoggerOpenFile: {
                let p = get_proc_addr(lib, b"cusparseLoggerOpenFile\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseLoggerSetLevel: {
                let p = get_proc_addr(lib, b"cusparseLoggerSetLevel\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseLoggerSetMask: {
                let p = get_proc_addr(lib, b"cusparseLoggerSetMask\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseLoggerForceDisable: {
                let p = get_proc_addr(lib, b"cusparseLoggerForceDisable\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCreateMatDescr: {
                let p = get_proc_addr(lib, b"cusparseCreateMatDescr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDestroyMatDescr: {
                let p = get_proc_addr(lib, b"cusparseDestroyMatDescr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSetMatType: {
                let p = get_proc_addr(lib, b"cusparseSetMatType\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseGetMatType: {
                let p = get_proc_addr(lib, b"cusparseGetMatType\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSetMatFillMode: {
                let p = get_proc_addr(lib, b"cusparseSetMatFillMode\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseGetMatFillMode: {
                let p = get_proc_addr(lib, b"cusparseGetMatFillMode\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSetMatDiagType: {
                let p = get_proc_addr(lib, b"cusparseSetMatDiagType\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseGetMatDiagType: {
                let p = get_proc_addr(lib, b"cusparseGetMatDiagType\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSetMatIndexBase: {
                let p = get_proc_addr(lib, b"cusparseSetMatIndexBase\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseGetMatIndexBase: {
                let p = get_proc_addr(lib, b"cusparseGetMatIndexBase\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCreateCsric02Info: {
                let p = get_proc_addr(lib, b"cusparseCreateCsric02Info\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDestroyCsric02Info: {
                let p = get_proc_addr(lib, b"cusparseDestroyCsric02Info\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCreateBsric02Info: {
                let p = get_proc_addr(lib, b"cusparseCreateBsric02Info\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDestroyBsric02Info: {
                let p = get_proc_addr(lib, b"cusparseDestroyBsric02Info\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCreateCsrilu02Info: {
                let p = get_proc_addr(lib, b"cusparseCreateCsrilu02Info\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDestroyCsrilu02Info: {
                let p = get_proc_addr(lib, b"cusparseDestroyCsrilu02Info\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCreateBsrilu02Info: {
                let p = get_proc_addr(lib, b"cusparseCreateBsrilu02Info\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDestroyBsrilu02Info: {
                let p = get_proc_addr(lib, b"cusparseDestroyBsrilu02Info\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCreateBsrsv2Info: {
                let p = get_proc_addr(lib, b"cusparseCreateBsrsv2Info\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDestroyBsrsv2Info: {
                let p = get_proc_addr(lib, b"cusparseDestroyBsrsv2Info\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCreateBsrsm2Info: {
                let p = get_proc_addr(lib, b"cusparseCreateBsrsm2Info\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDestroyBsrsm2Info: {
                let p = get_proc_addr(lib, b"cusparseDestroyBsrsm2Info\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCreateCsru2csrInfo: {
                let p = get_proc_addr(lib, b"cusparseCreateCsru2csrInfo\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDestroyCsru2csrInfo: {
                let p = get_proc_addr(lib, b"cusparseDestroyCsru2csrInfo\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCreateColorInfo: {
                let p = get_proc_addr(lib, b"cusparseCreateColorInfo\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDestroyColorInfo: {
                let p = get_proc_addr(lib, b"cusparseDestroyColorInfo\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCreatePruneInfo: {
                let p = get_proc_addr(lib, b"cusparseCreatePruneInfo\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDestroyPruneInfo: {
                let p = get_proc_addr(lib, b"cusparseDestroyPruneInfo\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSgemvi: {
                let p = get_proc_addr(lib, b"cusparseSgemvi\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSgemvi_bufferSize: {
                let p = get_proc_addr(lib, b"cusparseSgemvi_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDgemvi: {
                let p = get_proc_addr(lib, b"cusparseDgemvi\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDgemvi_bufferSize: {
                let p = get_proc_addr(lib, b"cusparseDgemvi_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCgemvi: {
                let p = get_proc_addr(lib, b"cusparseCgemvi\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCgemvi_bufferSize: {
                let p = get_proc_addr(lib, b"cusparseCgemvi_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseZgemvi: {
                let p = get_proc_addr(lib, b"cusparseZgemvi\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseZgemvi_bufferSize: {
                let p = get_proc_addr(lib, b"cusparseZgemvi_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSbsrmv: {
                let p = get_proc_addr(lib, b"cusparseSbsrmv\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDbsrmv: {
                let p = get_proc_addr(lib, b"cusparseDbsrmv\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCbsrmv: {
                let p = get_proc_addr(lib, b"cusparseCbsrmv\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseZbsrmv: {
                let p = get_proc_addr(lib, b"cusparseZbsrmv\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSbsrxmv: {
                let p = get_proc_addr(lib, b"cusparseSbsrxmv\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDbsrxmv: {
                let p = get_proc_addr(lib, b"cusparseDbsrxmv\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCbsrxmv: {
                let p = get_proc_addr(lib, b"cusparseCbsrxmv\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseZbsrxmv: {
                let p = get_proc_addr(lib, b"cusparseZbsrxmv\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseXbsrsv2_zeroPivot: {
                let p = get_proc_addr(lib, b"cusparseXbsrsv2_zeroPivot\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSbsrsv2_bufferSize: {
                let p = get_proc_addr(lib, b"cusparseSbsrsv2_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDbsrsv2_bufferSize: {
                let p = get_proc_addr(lib, b"cusparseDbsrsv2_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCbsrsv2_bufferSize: {
                let p = get_proc_addr(lib, b"cusparseCbsrsv2_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseZbsrsv2_bufferSize: {
                let p = get_proc_addr(lib, b"cusparseZbsrsv2_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSbsrsv2_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseSbsrsv2_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDbsrsv2_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseDbsrsv2_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCbsrsv2_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseCbsrsv2_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseZbsrsv2_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseZbsrsv2_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSbsrsv2_analysis: {
                let p = get_proc_addr(lib, b"cusparseSbsrsv2_analysis\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDbsrsv2_analysis: {
                let p = get_proc_addr(lib, b"cusparseDbsrsv2_analysis\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCbsrsv2_analysis: {
                let p = get_proc_addr(lib, b"cusparseCbsrsv2_analysis\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseZbsrsv2_analysis: {
                let p = get_proc_addr(lib, b"cusparseZbsrsv2_analysis\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSbsrsv2_solve: {
                let p = get_proc_addr(lib, b"cusparseSbsrsv2_solve\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDbsrsv2_solve: {
                let p = get_proc_addr(lib, b"cusparseDbsrsv2_solve\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCbsrsv2_solve: {
                let p = get_proc_addr(lib, b"cusparseCbsrsv2_solve\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseZbsrsv2_solve: {
                let p = get_proc_addr(lib, b"cusparseZbsrsv2_solve\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSbsrmm: {
                let p = get_proc_addr(lib, b"cusparseSbsrmm\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDbsrmm: {
                let p = get_proc_addr(lib, b"cusparseDbsrmm\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCbsrmm: {
                let p = get_proc_addr(lib, b"cusparseCbsrmm\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseZbsrmm: {
                let p = get_proc_addr(lib, b"cusparseZbsrmm\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseXbsrsm2_zeroPivot: {
                let p = get_proc_addr(lib, b"cusparseXbsrsm2_zeroPivot\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSbsrsm2_bufferSize: {
                let p = get_proc_addr(lib, b"cusparseSbsrsm2_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDbsrsm2_bufferSize: {
                let p = get_proc_addr(lib, b"cusparseDbsrsm2_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCbsrsm2_bufferSize: {
                let p = get_proc_addr(lib, b"cusparseCbsrsm2_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseZbsrsm2_bufferSize: {
                let p = get_proc_addr(lib, b"cusparseZbsrsm2_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSbsrsm2_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseSbsrsm2_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDbsrsm2_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseDbsrsm2_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCbsrsm2_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseCbsrsm2_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseZbsrsm2_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseZbsrsm2_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSbsrsm2_analysis: {
                let p = get_proc_addr(lib, b"cusparseSbsrsm2_analysis\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDbsrsm2_analysis: {
                let p = get_proc_addr(lib, b"cusparseDbsrsm2_analysis\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCbsrsm2_analysis: {
                let p = get_proc_addr(lib, b"cusparseCbsrsm2_analysis\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseZbsrsm2_analysis: {
                let p = get_proc_addr(lib, b"cusparseZbsrsm2_analysis\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSbsrsm2_solve: {
                let p = get_proc_addr(lib, b"cusparseSbsrsm2_solve\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDbsrsm2_solve: {
                let p = get_proc_addr(lib, b"cusparseDbsrsm2_solve\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCbsrsm2_solve: {
                let p = get_proc_addr(lib, b"cusparseCbsrsm2_solve\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseZbsrsm2_solve: {
                let p = get_proc_addr(lib, b"cusparseZbsrsm2_solve\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseScsrilu02_numericBoost: {
                let p = get_proc_addr(lib, b"cusparseScsrilu02_numericBoost\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDcsrilu02_numericBoost: {
                let p = get_proc_addr(lib, b"cusparseDcsrilu02_numericBoost\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCcsrilu02_numericBoost: {
                let p = get_proc_addr(lib, b"cusparseCcsrilu02_numericBoost\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseZcsrilu02_numericBoost: {
                let p = get_proc_addr(lib, b"cusparseZcsrilu02_numericBoost\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseXcsrilu02_zeroPivot: {
                let p = get_proc_addr(lib, b"cusparseXcsrilu02_zeroPivot\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseScsrilu02_bufferSize: {
                let p = get_proc_addr(lib, b"cusparseScsrilu02_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDcsrilu02_bufferSize: {
                let p = get_proc_addr(lib, b"cusparseDcsrilu02_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCcsrilu02_bufferSize: {
                let p = get_proc_addr(lib, b"cusparseCcsrilu02_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseZcsrilu02_bufferSize: {
                let p = get_proc_addr(lib, b"cusparseZcsrilu02_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseScsrilu02_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseScsrilu02_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDcsrilu02_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseDcsrilu02_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCcsrilu02_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseCcsrilu02_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseZcsrilu02_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseZcsrilu02_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseScsrilu02_analysis: {
                let p = get_proc_addr(lib, b"cusparseScsrilu02_analysis\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDcsrilu02_analysis: {
                let p = get_proc_addr(lib, b"cusparseDcsrilu02_analysis\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCcsrilu02_analysis: {
                let p = get_proc_addr(lib, b"cusparseCcsrilu02_analysis\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseZcsrilu02_analysis: {
                let p = get_proc_addr(lib, b"cusparseZcsrilu02_analysis\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseScsrilu02: {
                let p = get_proc_addr(lib, b"cusparseScsrilu02\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDcsrilu02: {
                let p = get_proc_addr(lib, b"cusparseDcsrilu02\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCcsrilu02: {
                let p = get_proc_addr(lib, b"cusparseCcsrilu02\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseZcsrilu02: {
                let p = get_proc_addr(lib, b"cusparseZcsrilu02\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSbsrilu02_numericBoost: {
                let p = get_proc_addr(lib, b"cusparseSbsrilu02_numericBoost\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDbsrilu02_numericBoost: {
                let p = get_proc_addr(lib, b"cusparseDbsrilu02_numericBoost\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCbsrilu02_numericBoost: {
                let p = get_proc_addr(lib, b"cusparseCbsrilu02_numericBoost\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseZbsrilu02_numericBoost: {
                let p = get_proc_addr(lib, b"cusparseZbsrilu02_numericBoost\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseXbsrilu02_zeroPivot: {
                let p = get_proc_addr(lib, b"cusparseXbsrilu02_zeroPivot\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSbsrilu02_bufferSize: {
                let p = get_proc_addr(lib, b"cusparseSbsrilu02_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDbsrilu02_bufferSize: {
                let p = get_proc_addr(lib, b"cusparseDbsrilu02_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCbsrilu02_bufferSize: {
                let p = get_proc_addr(lib, b"cusparseCbsrilu02_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseZbsrilu02_bufferSize: {
                let p = get_proc_addr(lib, b"cusparseZbsrilu02_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSbsrilu02_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseSbsrilu02_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDbsrilu02_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseDbsrilu02_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCbsrilu02_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseCbsrilu02_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseZbsrilu02_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseZbsrilu02_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSbsrilu02_analysis: {
                let p = get_proc_addr(lib, b"cusparseSbsrilu02_analysis\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDbsrilu02_analysis: {
                let p = get_proc_addr(lib, b"cusparseDbsrilu02_analysis\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCbsrilu02_analysis: {
                let p = get_proc_addr(lib, b"cusparseCbsrilu02_analysis\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseZbsrilu02_analysis: {
                let p = get_proc_addr(lib, b"cusparseZbsrilu02_analysis\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSbsrilu02: {
                let p = get_proc_addr(lib, b"cusparseSbsrilu02\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDbsrilu02: {
                let p = get_proc_addr(lib, b"cusparseDbsrilu02\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCbsrilu02: {
                let p = get_proc_addr(lib, b"cusparseCbsrilu02\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseZbsrilu02: {
                let p = get_proc_addr(lib, b"cusparseZbsrilu02\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseXcsric02_zeroPivot: {
                let p = get_proc_addr(lib, b"cusparseXcsric02_zeroPivot\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseScsric02_bufferSize: {
                let p = get_proc_addr(lib, b"cusparseScsric02_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDcsric02_bufferSize: {
                let p = get_proc_addr(lib, b"cusparseDcsric02_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCcsric02_bufferSize: {
                let p = get_proc_addr(lib, b"cusparseCcsric02_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseZcsric02_bufferSize: {
                let p = get_proc_addr(lib, b"cusparseZcsric02_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseScsric02_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseScsric02_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDcsric02_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseDcsric02_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCcsric02_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseCcsric02_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseZcsric02_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseZcsric02_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseScsric02_analysis: {
                let p = get_proc_addr(lib, b"cusparseScsric02_analysis\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDcsric02_analysis: {
                let p = get_proc_addr(lib, b"cusparseDcsric02_analysis\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCcsric02_analysis: {
                let p = get_proc_addr(lib, b"cusparseCcsric02_analysis\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseZcsric02_analysis: {
                let p = get_proc_addr(lib, b"cusparseZcsric02_analysis\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseScsric02: {
                let p = get_proc_addr(lib, b"cusparseScsric02\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDcsric02: {
                let p = get_proc_addr(lib, b"cusparseDcsric02\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCcsric02: {
                let p = get_proc_addr(lib, b"cusparseCcsric02\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseZcsric02: {
                let p = get_proc_addr(lib, b"cusparseZcsric02\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseXbsric02_zeroPivot: {
                let p = get_proc_addr(lib, b"cusparseXbsric02_zeroPivot\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSbsric02_bufferSize: {
                let p = get_proc_addr(lib, b"cusparseSbsric02_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDbsric02_bufferSize: {
                let p = get_proc_addr(lib, b"cusparseDbsric02_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCbsric02_bufferSize: {
                let p = get_proc_addr(lib, b"cusparseCbsric02_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseZbsric02_bufferSize: {
                let p = get_proc_addr(lib, b"cusparseZbsric02_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSbsric02_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseSbsric02_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDbsric02_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseDbsric02_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCbsric02_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseCbsric02_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseZbsric02_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseZbsric02_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSbsric02_analysis: {
                let p = get_proc_addr(lib, b"cusparseSbsric02_analysis\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDbsric02_analysis: {
                let p = get_proc_addr(lib, b"cusparseDbsric02_analysis\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCbsric02_analysis: {
                let p = get_proc_addr(lib, b"cusparseCbsric02_analysis\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseZbsric02_analysis: {
                let p = get_proc_addr(lib, b"cusparseZbsric02_analysis\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSbsric02: {
                let p = get_proc_addr(lib, b"cusparseSbsric02\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDbsric02: {
                let p = get_proc_addr(lib, b"cusparseDbsric02\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCbsric02: {
                let p = get_proc_addr(lib, b"cusparseCbsric02\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseZbsric02: {
                let p = get_proc_addr(lib, b"cusparseZbsric02\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSgtsv2_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseSgtsv2_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDgtsv2_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseDgtsv2_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCgtsv2_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseCgtsv2_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseZgtsv2_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseZgtsv2_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSgtsv2: {
                let p = get_proc_addr(lib, b"cusparseSgtsv2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDgtsv2: {
                let p = get_proc_addr(lib, b"cusparseDgtsv2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCgtsv2: {
                let p = get_proc_addr(lib, b"cusparseCgtsv2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseZgtsv2: {
                let p = get_proc_addr(lib, b"cusparseZgtsv2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSgtsv2_nopivot_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseSgtsv2_nopivot_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDgtsv2_nopivot_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseDgtsv2_nopivot_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCgtsv2_nopivot_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseCgtsv2_nopivot_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseZgtsv2_nopivot_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseZgtsv2_nopivot_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSgtsv2_nopivot: {
                let p = get_proc_addr(lib, b"cusparseSgtsv2_nopivot\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDgtsv2_nopivot: {
                let p = get_proc_addr(lib, b"cusparseDgtsv2_nopivot\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCgtsv2_nopivot: {
                let p = get_proc_addr(lib, b"cusparseCgtsv2_nopivot\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseZgtsv2_nopivot: {
                let p = get_proc_addr(lib, b"cusparseZgtsv2_nopivot\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSgtsv2StridedBatch_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseSgtsv2StridedBatch_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDgtsv2StridedBatch_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseDgtsv2StridedBatch_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCgtsv2StridedBatch_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseCgtsv2StridedBatch_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseZgtsv2StridedBatch_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseZgtsv2StridedBatch_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSgtsv2StridedBatch: {
                let p = get_proc_addr(lib, b"cusparseSgtsv2StridedBatch\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDgtsv2StridedBatch: {
                let p = get_proc_addr(lib, b"cusparseDgtsv2StridedBatch\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCgtsv2StridedBatch: {
                let p = get_proc_addr(lib, b"cusparseCgtsv2StridedBatch\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseZgtsv2StridedBatch: {
                let p = get_proc_addr(lib, b"cusparseZgtsv2StridedBatch\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSgtsvInterleavedBatch_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseSgtsvInterleavedBatch_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDgtsvInterleavedBatch_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseDgtsvInterleavedBatch_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCgtsvInterleavedBatch_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseCgtsvInterleavedBatch_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseZgtsvInterleavedBatch_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseZgtsvInterleavedBatch_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSgtsvInterleavedBatch: {
                let p = get_proc_addr(lib, b"cusparseSgtsvInterleavedBatch\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDgtsvInterleavedBatch: {
                let p = get_proc_addr(lib, b"cusparseDgtsvInterleavedBatch\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCgtsvInterleavedBatch: {
                let p = get_proc_addr(lib, b"cusparseCgtsvInterleavedBatch\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseZgtsvInterleavedBatch: {
                let p = get_proc_addr(lib, b"cusparseZgtsvInterleavedBatch\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSgpsvInterleavedBatch_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseSgpsvInterleavedBatch_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDgpsvInterleavedBatch_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseDgpsvInterleavedBatch_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCgpsvInterleavedBatch_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseCgpsvInterleavedBatch_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseZgpsvInterleavedBatch_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseZgpsvInterleavedBatch_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSgpsvInterleavedBatch: {
                let p = get_proc_addr(lib, b"cusparseSgpsvInterleavedBatch\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDgpsvInterleavedBatch: {
                let p = get_proc_addr(lib, b"cusparseDgpsvInterleavedBatch\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCgpsvInterleavedBatch: {
                let p = get_proc_addr(lib, b"cusparseCgpsvInterleavedBatch\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseZgpsvInterleavedBatch: {
                let p = get_proc_addr(lib, b"cusparseZgpsvInterleavedBatch\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseScsrgeam2_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseScsrgeam2_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDcsrgeam2_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseDcsrgeam2_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCcsrgeam2_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseCcsrgeam2_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseZcsrgeam2_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseZcsrgeam2_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseXcsrgeam2Nnz: {
                let p = get_proc_addr(lib, b"cusparseXcsrgeam2Nnz\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseScsrgeam2: {
                let p = get_proc_addr(lib, b"cusparseScsrgeam2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDcsrgeam2: {
                let p = get_proc_addr(lib, b"cusparseDcsrgeam2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCcsrgeam2: {
                let p = get_proc_addr(lib, b"cusparseCcsrgeam2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseZcsrgeam2: {
                let p = get_proc_addr(lib, b"cusparseZcsrgeam2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseScsrcolor: {
                let p = get_proc_addr(lib, b"cusparseScsrcolor\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDcsrcolor: {
                let p = get_proc_addr(lib, b"cusparseDcsrcolor\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCcsrcolor: {
                let p = get_proc_addr(lib, b"cusparseCcsrcolor\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseZcsrcolor: {
                let p = get_proc_addr(lib, b"cusparseZcsrcolor\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSnnz: {
                let p = get_proc_addr(lib, b"cusparseSnnz\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDnnz: {
                let p = get_proc_addr(lib, b"cusparseDnnz\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCnnz: {
                let p = get_proc_addr(lib, b"cusparseCnnz\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseZnnz: {
                let p = get_proc_addr(lib, b"cusparseZnnz\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSnnz_compress: {
                let p = get_proc_addr(lib, b"cusparseSnnz_compress\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDnnz_compress: {
                let p = get_proc_addr(lib, b"cusparseDnnz_compress\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCnnz_compress: {
                let p = get_proc_addr(lib, b"cusparseCnnz_compress\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseZnnz_compress: {
                let p = get_proc_addr(lib, b"cusparseZnnz_compress\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseScsr2csr_compress: {
                let p = get_proc_addr(lib, b"cusparseScsr2csr_compress\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDcsr2csr_compress: {
                let p = get_proc_addr(lib, b"cusparseDcsr2csr_compress\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCcsr2csr_compress: {
                let p = get_proc_addr(lib, b"cusparseCcsr2csr_compress\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseZcsr2csr_compress: {
                let p = get_proc_addr(lib, b"cusparseZcsr2csr_compress\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseXcoo2csr: {
                let p = get_proc_addr(lib, b"cusparseXcoo2csr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseXcsr2coo: {
                let p = get_proc_addr(lib, b"cusparseXcsr2coo\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseXcsr2bsrNnz: {
                let p = get_proc_addr(lib, b"cusparseXcsr2bsrNnz\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseScsr2bsr: {
                let p = get_proc_addr(lib, b"cusparseScsr2bsr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDcsr2bsr: {
                let p = get_proc_addr(lib, b"cusparseDcsr2bsr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCcsr2bsr: {
                let p = get_proc_addr(lib, b"cusparseCcsr2bsr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseZcsr2bsr: {
                let p = get_proc_addr(lib, b"cusparseZcsr2bsr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSbsr2csr: {
                let p = get_proc_addr(lib, b"cusparseSbsr2csr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDbsr2csr: {
                let p = get_proc_addr(lib, b"cusparseDbsr2csr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCbsr2csr: {
                let p = get_proc_addr(lib, b"cusparseCbsr2csr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseZbsr2csr: {
                let p = get_proc_addr(lib, b"cusparseZbsr2csr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSgebsr2gebsc_bufferSize: {
                let p = get_proc_addr(lib, b"cusparseSgebsr2gebsc_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDgebsr2gebsc_bufferSize: {
                let p = get_proc_addr(lib, b"cusparseDgebsr2gebsc_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCgebsr2gebsc_bufferSize: {
                let p = get_proc_addr(lib, b"cusparseCgebsr2gebsc_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseZgebsr2gebsc_bufferSize: {
                let p = get_proc_addr(lib, b"cusparseZgebsr2gebsc_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSgebsr2gebsc_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseSgebsr2gebsc_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDgebsr2gebsc_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseDgebsr2gebsc_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCgebsr2gebsc_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseCgebsr2gebsc_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseZgebsr2gebsc_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseZgebsr2gebsc_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSgebsr2gebsc: {
                let p = get_proc_addr(lib, b"cusparseSgebsr2gebsc\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDgebsr2gebsc: {
                let p = get_proc_addr(lib, b"cusparseDgebsr2gebsc\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCgebsr2gebsc: {
                let p = get_proc_addr(lib, b"cusparseCgebsr2gebsc\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseZgebsr2gebsc: {
                let p = get_proc_addr(lib, b"cusparseZgebsr2gebsc\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseXgebsr2csr: {
                let p = get_proc_addr(lib, b"cusparseXgebsr2csr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSgebsr2csr: {
                let p = get_proc_addr(lib, b"cusparseSgebsr2csr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDgebsr2csr: {
                let p = get_proc_addr(lib, b"cusparseDgebsr2csr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCgebsr2csr: {
                let p = get_proc_addr(lib, b"cusparseCgebsr2csr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseZgebsr2csr: {
                let p = get_proc_addr(lib, b"cusparseZgebsr2csr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseScsr2gebsr_bufferSize: {
                let p = get_proc_addr(lib, b"cusparseScsr2gebsr_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDcsr2gebsr_bufferSize: {
                let p = get_proc_addr(lib, b"cusparseDcsr2gebsr_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCcsr2gebsr_bufferSize: {
                let p = get_proc_addr(lib, b"cusparseCcsr2gebsr_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseZcsr2gebsr_bufferSize: {
                let p = get_proc_addr(lib, b"cusparseZcsr2gebsr_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseScsr2gebsr_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseScsr2gebsr_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDcsr2gebsr_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseDcsr2gebsr_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCcsr2gebsr_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseCcsr2gebsr_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseZcsr2gebsr_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseZcsr2gebsr_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseXcsr2gebsrNnz: {
                let p = get_proc_addr(lib, b"cusparseXcsr2gebsrNnz\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseScsr2gebsr: {
                let p = get_proc_addr(lib, b"cusparseScsr2gebsr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDcsr2gebsr: {
                let p = get_proc_addr(lib, b"cusparseDcsr2gebsr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCcsr2gebsr: {
                let p = get_proc_addr(lib, b"cusparseCcsr2gebsr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseZcsr2gebsr: {
                let p = get_proc_addr(lib, b"cusparseZcsr2gebsr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSgebsr2gebsr_bufferSize: {
                let p = get_proc_addr(lib, b"cusparseSgebsr2gebsr_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDgebsr2gebsr_bufferSize: {
                let p = get_proc_addr(lib, b"cusparseDgebsr2gebsr_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCgebsr2gebsr_bufferSize: {
                let p = get_proc_addr(lib, b"cusparseCgebsr2gebsr_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseZgebsr2gebsr_bufferSize: {
                let p = get_proc_addr(lib, b"cusparseZgebsr2gebsr_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSgebsr2gebsr_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseSgebsr2gebsr_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDgebsr2gebsr_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseDgebsr2gebsr_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCgebsr2gebsr_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseCgebsr2gebsr_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseZgebsr2gebsr_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseZgebsr2gebsr_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseXgebsr2gebsrNnz: {
                let p = get_proc_addr(lib, b"cusparseXgebsr2gebsrNnz\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSgebsr2gebsr: {
                let p = get_proc_addr(lib, b"cusparseSgebsr2gebsr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDgebsr2gebsr: {
                let p = get_proc_addr(lib, b"cusparseDgebsr2gebsr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCgebsr2gebsr: {
                let p = get_proc_addr(lib, b"cusparseCgebsr2gebsr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseZgebsr2gebsr: {
                let p = get_proc_addr(lib, b"cusparseZgebsr2gebsr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCreateIdentityPermutation: {
                let p = get_proc_addr(lib, b"cusparseCreateIdentityPermutation\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseXcoosort_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseXcoosort_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseXcoosortByRow: {
                let p = get_proc_addr(lib, b"cusparseXcoosortByRow\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseXcoosortByColumn: {
                let p = get_proc_addr(lib, b"cusparseXcoosortByColumn\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseXcsrsort_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseXcsrsort_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseXcsrsort: {
                let p = get_proc_addr(lib, b"cusparseXcsrsort\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseXcscsort_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseXcscsort_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseXcscsort: {
                let p = get_proc_addr(lib, b"cusparseXcscsort\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseScsru2csr_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseScsru2csr_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDcsru2csr_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseDcsru2csr_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCcsru2csr_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseCcsru2csr_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseZcsru2csr_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseZcsru2csr_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseScsru2csr: {
                let p = get_proc_addr(lib, b"cusparseScsru2csr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDcsru2csr: {
                let p = get_proc_addr(lib, b"cusparseDcsru2csr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCcsru2csr: {
                let p = get_proc_addr(lib, b"cusparseCcsru2csr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseZcsru2csr: {
                let p = get_proc_addr(lib, b"cusparseZcsru2csr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseScsr2csru: {
                let p = get_proc_addr(lib, b"cusparseScsr2csru\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDcsr2csru: {
                let p = get_proc_addr(lib, b"cusparseDcsr2csru\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCcsr2csru: {
                let p = get_proc_addr(lib, b"cusparseCcsr2csru\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseZcsr2csru: {
                let p = get_proc_addr(lib, b"cusparseZcsr2csru\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSpruneDense2csr_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseSpruneDense2csr_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDpruneDense2csr_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseDpruneDense2csr_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSpruneDense2csrNnz: {
                let p = get_proc_addr(lib, b"cusparseSpruneDense2csrNnz\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDpruneDense2csrNnz: {
                let p = get_proc_addr(lib, b"cusparseDpruneDense2csrNnz\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSpruneDense2csr: {
                let p = get_proc_addr(lib, b"cusparseSpruneDense2csr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDpruneDense2csr: {
                let p = get_proc_addr(lib, b"cusparseDpruneDense2csr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSpruneCsr2csr_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseSpruneCsr2csr_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDpruneCsr2csr_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseDpruneCsr2csr_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSpruneCsr2csrNnz: {
                let p = get_proc_addr(lib, b"cusparseSpruneCsr2csrNnz\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDpruneCsr2csrNnz: {
                let p = get_proc_addr(lib, b"cusparseDpruneCsr2csrNnz\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSpruneCsr2csr: {
                let p = get_proc_addr(lib, b"cusparseSpruneCsr2csr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDpruneCsr2csr: {
                let p = get_proc_addr(lib, b"cusparseDpruneCsr2csr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSpruneDense2csrByPercentage_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseSpruneDense2csrByPercentage_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDpruneDense2csrByPercentage_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseDpruneDense2csrByPercentage_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSpruneDense2csrNnzByPercentage: {
                let p = get_proc_addr(lib, b"cusparseSpruneDense2csrNnzByPercentage\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDpruneDense2csrNnzByPercentage: {
                let p = get_proc_addr(lib, b"cusparseDpruneDense2csrNnzByPercentage\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSpruneDense2csrByPercentage: {
                let p = get_proc_addr(lib, b"cusparseSpruneDense2csrByPercentage\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDpruneDense2csrByPercentage: {
                let p = get_proc_addr(lib, b"cusparseDpruneDense2csrByPercentage\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSpruneCsr2csrByPercentage_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseSpruneCsr2csrByPercentage_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDpruneCsr2csrByPercentage_bufferSizeExt: {
                let p = get_proc_addr(lib, b"cusparseDpruneCsr2csrByPercentage_bufferSizeExt\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSpruneCsr2csrNnzByPercentage: {
                let p = get_proc_addr(lib, b"cusparseSpruneCsr2csrNnzByPercentage\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDpruneCsr2csrNnzByPercentage: {
                let p = get_proc_addr(lib, b"cusparseDpruneCsr2csrNnzByPercentage\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSpruneCsr2csrByPercentage: {
                let p = get_proc_addr(lib, b"cusparseSpruneCsr2csrByPercentage\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDpruneCsr2csrByPercentage: {
                let p = get_proc_addr(lib, b"cusparseDpruneCsr2csrByPercentage\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCsr2cscEx2: {
                let p = get_proc_addr(lib, b"cusparseCsr2cscEx2\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCsr2cscEx2_bufferSize: {
                let p = get_proc_addr(lib, b"cusparseCsr2cscEx2_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCreateSpVec: {
                let p = get_proc_addr(lib, b"cusparseCreateSpVec\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCreateConstSpVec: {
                let p = get_proc_addr(lib, b"cusparseCreateConstSpVec\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDestroySpVec: {
                let p = get_proc_addr(lib, b"cusparseDestroySpVec\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSpVecGet: {
                let p = get_proc_addr(lib, b"cusparseSpVecGet\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseConstSpVecGet: {
                let p = get_proc_addr(lib, b"cusparseConstSpVecGet\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSpVecGetIndexBase: {
                let p = get_proc_addr(lib, b"cusparseSpVecGetIndexBase\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSpVecGetValues: {
                let p = get_proc_addr(lib, b"cusparseSpVecGetValues\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseConstSpVecGetValues: {
                let p = get_proc_addr(lib, b"cusparseConstSpVecGetValues\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSpVecSetValues: {
                let p = get_proc_addr(lib, b"cusparseSpVecSetValues\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCreateDnVec: {
                let p = get_proc_addr(lib, b"cusparseCreateDnVec\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCreateConstDnVec: {
                let p = get_proc_addr(lib, b"cusparseCreateConstDnVec\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDestroyDnVec: {
                let p = get_proc_addr(lib, b"cusparseDestroyDnVec\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDnVecGet: {
                let p = get_proc_addr(lib, b"cusparseDnVecGet\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseConstDnVecGet: {
                let p = get_proc_addr(lib, b"cusparseConstDnVecGet\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDnVecGetValues: {
                let p = get_proc_addr(lib, b"cusparseDnVecGetValues\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseConstDnVecGetValues: {
                let p = get_proc_addr(lib, b"cusparseConstDnVecGetValues\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDnVecSetValues: {
                let p = get_proc_addr(lib, b"cusparseDnVecSetValues\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDestroySpMat: {
                let p = get_proc_addr(lib, b"cusparseDestroySpMat\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSpMatGetFormat: {
                let p = get_proc_addr(lib, b"cusparseSpMatGetFormat\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSpMatGetIndexBase: {
                let p = get_proc_addr(lib, b"cusparseSpMatGetIndexBase\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSpMatGetValues: {
                let p = get_proc_addr(lib, b"cusparseSpMatGetValues\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseConstSpMatGetValues: {
                let p = get_proc_addr(lib, b"cusparseConstSpMatGetValues\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSpMatSetValues: {
                let p = get_proc_addr(lib, b"cusparseSpMatSetValues\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSpMatGetSize: {
                let p = get_proc_addr(lib, b"cusparseSpMatGetSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSpMatGetStridedBatch: {
                let p = get_proc_addr(lib, b"cusparseSpMatGetStridedBatch\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCooSetStridedBatch: {
                let p = get_proc_addr(lib, b"cusparseCooSetStridedBatch\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCsrSetStridedBatch: {
                let p = get_proc_addr(lib, b"cusparseCsrSetStridedBatch\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseBsrSetStridedBatch: {
                let p = get_proc_addr(lib, b"cusparseBsrSetStridedBatch\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSpMatGetAttribute: {
                let p = get_proc_addr(lib, b"cusparseSpMatGetAttribute\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSpMatSetAttribute: {
                let p = get_proc_addr(lib, b"cusparseSpMatSetAttribute\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCreateCsr: {
                let p = get_proc_addr(lib, b"cusparseCreateCsr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCreateConstCsr: {
                let p = get_proc_addr(lib, b"cusparseCreateConstCsr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCreateCsc: {
                let p = get_proc_addr(lib, b"cusparseCreateCsc\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCreateConstCsc: {
                let p = get_proc_addr(lib, b"cusparseCreateConstCsc\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCsrGet: {
                let p = get_proc_addr(lib, b"cusparseCsrGet\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseConstCsrGet: {
                let p = get_proc_addr(lib, b"cusparseConstCsrGet\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCscGet: {
                let p = get_proc_addr(lib, b"cusparseCscGet\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseConstCscGet: {
                let p = get_proc_addr(lib, b"cusparseConstCscGet\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCsrSetPointers: {
                let p = get_proc_addr(lib, b"cusparseCsrSetPointers\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCscSetPointers: {
                let p = get_proc_addr(lib, b"cusparseCscSetPointers\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCreateBsr: {
                let p = get_proc_addr(lib, b"cusparseCreateBsr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCreateConstBsr: {
                let p = get_proc_addr(lib, b"cusparseCreateConstBsr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCreateCoo: {
                let p = get_proc_addr(lib, b"cusparseCreateCoo\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCreateConstCoo: {
                let p = get_proc_addr(lib, b"cusparseCreateConstCoo\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCooGet: {
                let p = get_proc_addr(lib, b"cusparseCooGet\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseConstCooGet: {
                let p = get_proc_addr(lib, b"cusparseConstCooGet\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCooSetPointers: {
                let p = get_proc_addr(lib, b"cusparseCooSetPointers\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCreateBlockedEll: {
                let p = get_proc_addr(lib, b"cusparseCreateBlockedEll\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCreateConstBlockedEll: {
                let p = get_proc_addr(lib, b"cusparseCreateConstBlockedEll\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseBlockedEllGet: {
                let p = get_proc_addr(lib, b"cusparseBlockedEllGet\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseConstBlockedEllGet: {
                let p = get_proc_addr(lib, b"cusparseConstBlockedEllGet\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCreateSlicedEll: {
                let p = get_proc_addr(lib, b"cusparseCreateSlicedEll\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCreateConstSlicedEll: {
                let p = get_proc_addr(lib, b"cusparseCreateConstSlicedEll\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCreateDnMat: {
                let p = get_proc_addr(lib, b"cusparseCreateDnMat\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseCreateConstDnMat: {
                let p = get_proc_addr(lib, b"cusparseCreateConstDnMat\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDestroyDnMat: {
                let p = get_proc_addr(lib, b"cusparseDestroyDnMat\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDnMatGet: {
                let p = get_proc_addr(lib, b"cusparseDnMatGet\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseConstDnMatGet: {
                let p = get_proc_addr(lib, b"cusparseConstDnMatGet\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDnMatGetValues: {
                let p = get_proc_addr(lib, b"cusparseDnMatGetValues\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseConstDnMatGetValues: {
                let p = get_proc_addr(lib, b"cusparseConstDnMatGetValues\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDnMatSetValues: {
                let p = get_proc_addr(lib, b"cusparseDnMatSetValues\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDnMatSetStridedBatch: {
                let p = get_proc_addr(lib, b"cusparseDnMatSetStridedBatch\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDnMatGetStridedBatch: {
                let p = get_proc_addr(lib, b"cusparseDnMatGetStridedBatch\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseAxpby: {
                let p = get_proc_addr(lib, b"cusparseAxpby\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseGather: {
                let p = get_proc_addr(lib, b"cusparseGather\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseScatter: {
                let p = get_proc_addr(lib, b"cusparseScatter\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseRot: {
                let p = get_proc_addr(lib, b"cusparseRot\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSpVV_bufferSize: {
                let p = get_proc_addr(lib, b"cusparseSpVV_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSpVV: {
                let p = get_proc_addr(lib, b"cusparseSpVV\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSparseToDense_bufferSize: {
                let p = get_proc_addr(lib, b"cusparseSparseToDense_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSparseToDense: {
                let p = get_proc_addr(lib, b"cusparseSparseToDense\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDenseToSparse_bufferSize: {
                let p = get_proc_addr(lib, b"cusparseDenseToSparse_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDenseToSparse_analysis: {
                let p = get_proc_addr(lib, b"cusparseDenseToSparse_analysis\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseDenseToSparse_convert: {
                let p = get_proc_addr(lib, b"cusparseDenseToSparse_convert\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSpMV: {
                let p = get_proc_addr(lib, b"cusparseSpMV\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSpMV_bufferSize: {
                let p = get_proc_addr(lib, b"cusparseSpMV_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSpMV_preprocess: {
                let p = get_proc_addr(lib, b"cusparseSpMV_preprocess\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSpSV_createDescr: {
                let p = get_proc_addr(lib, b"cusparseSpSV_createDescr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSpSV_destroyDescr: {
                let p = get_proc_addr(lib, b"cusparseSpSV_destroyDescr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSpSV_bufferSize: {
                let p = get_proc_addr(lib, b"cusparseSpSV_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSpSV_analysis: {
                let p = get_proc_addr(lib, b"cusparseSpSV_analysis\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSpSV_solve: {
                let p = get_proc_addr(lib, b"cusparseSpSV_solve\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSpSV_updateMatrix: {
                let p = get_proc_addr(lib, b"cusparseSpSV_updateMatrix\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSpSM_createDescr: {
                let p = get_proc_addr(lib, b"cusparseSpSM_createDescr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSpSM_destroyDescr: {
                let p = get_proc_addr(lib, b"cusparseSpSM_destroyDescr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSpSM_bufferSize: {
                let p = get_proc_addr(lib, b"cusparseSpSM_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSpSM_analysis: {
                let p = get_proc_addr(lib, b"cusparseSpSM_analysis\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSpSM_solve: {
                let p = get_proc_addr(lib, b"cusparseSpSM_solve\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSpSM_updateMatrix: {
                let p = get_proc_addr(lib, b"cusparseSpSM_updateMatrix\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSpMM_bufferSize: {
                let p = get_proc_addr(lib, b"cusparseSpMM_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSpMM_preprocess: {
                let p = get_proc_addr(lib, b"cusparseSpMM_preprocess\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSpMM: {
                let p = get_proc_addr(lib, b"cusparseSpMM\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSpGEMM_createDescr: {
                let p = get_proc_addr(lib, b"cusparseSpGEMM_createDescr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSpGEMM_destroyDescr: {
                let p = get_proc_addr(lib, b"cusparseSpGEMM_destroyDescr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSpGEMM_workEstimation: {
                let p = get_proc_addr(lib, b"cusparseSpGEMM_workEstimation\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSpGEMM_getNumProducts: {
                let p = get_proc_addr(lib, b"cusparseSpGEMM_getNumProducts\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSpGEMM_estimateMemory: {
                let p = get_proc_addr(lib, b"cusparseSpGEMM_estimateMemory\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSpGEMM_compute: {
                let p = get_proc_addr(lib, b"cusparseSpGEMM_compute\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSpGEMM_copy: {
                let p = get_proc_addr(lib, b"cusparseSpGEMM_copy\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSpGEMMreuse_workEstimation: {
                let p = get_proc_addr(lib, b"cusparseSpGEMMreuse_workEstimation\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSpGEMMreuse_nnz: {
                let p = get_proc_addr(lib, b"cusparseSpGEMMreuse_nnz\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSpGEMMreuse_copy: {
                let p = get_proc_addr(lib, b"cusparseSpGEMMreuse_copy\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSpGEMMreuse_compute: {
                let p = get_proc_addr(lib, b"cusparseSpGEMMreuse_compute\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSDDMM_bufferSize: {
                let p = get_proc_addr(lib, b"cusparseSDDMM_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSDDMM_preprocess: {
                let p = get_proc_addr(lib, b"cusparseSDDMM_preprocess\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSDDMM: {
                let p = get_proc_addr(lib, b"cusparseSDDMM\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSpMMOp_createPlan: {
                let p = get_proc_addr(lib, b"cusparseSpMMOp_createPlan\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSpMMOp: {
                let p = get_proc_addr(lib, b"cusparseSpMMOp\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusparseSpMMOp_destroyPlan: {
                let p = get_proc_addr(lib, b"cusparseSpMMOp_destroyPlan\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
        })
    };
    DYNAMIC_BINDINGS.set(bindings).ok();
}
unsafe impl<Storage: Send + Sync> Send for __BindgenBitfieldUnit<Storage> {}
unsafe impl<Storage: Send + Sync> Sync for __BindgenBitfieldUnit<Storage> {}
unsafe impl Send for float2 {}
unsafe impl Sync for float2 {}
unsafe impl Send for double2 {}
unsafe impl Sync for double2 {}
unsafe impl Send for CUstream_st {}
unsafe impl Sync for CUstream_st {}
unsafe impl Send for libraryPropertyType_t {}
unsafe impl Sync for libraryPropertyType_t {}
unsafe impl Send for _IO_marker {}
unsafe impl Sync for _IO_marker {}
unsafe impl Send for _IO_codecvt {}
unsafe impl Sync for _IO_codecvt {}
unsafe impl Send for _IO_wide_data {}
unsafe impl Sync for _IO_wide_data {}
unsafe impl Send for _IO_FILE {}
unsafe impl Sync for _IO_FILE {}
unsafe impl Send for cusparseContext {}
unsafe impl Sync for cusparseContext {}
unsafe impl Send for cusparseMatDescr {}
unsafe impl Sync for cusparseMatDescr {}
unsafe impl Send for bsrsv2Info {}
unsafe impl Sync for bsrsv2Info {}
unsafe impl Send for bsrsm2Info {}
unsafe impl Sync for bsrsm2Info {}
unsafe impl Send for csric02Info {}
unsafe impl Sync for csric02Info {}
unsafe impl Send for bsric02Info {}
unsafe impl Sync for bsric02Info {}
unsafe impl Send for csrilu02Info {}
unsafe impl Sync for csrilu02Info {}
unsafe impl Send for bsrilu02Info {}
unsafe impl Sync for bsrilu02Info {}
unsafe impl Send for csru2csrInfo {}
unsafe impl Sync for csru2csrInfo {}
unsafe impl Send for cusparseColorInfo {}
unsafe impl Sync for cusparseColorInfo {}
unsafe impl Send for pruneInfo {}
unsafe impl Sync for pruneInfo {}
unsafe impl Send for cusparseStatus_t {}
unsafe impl Sync for cusparseStatus_t {}
unsafe impl Send for cusparsePointerMode_t {}
unsafe impl Sync for cusparsePointerMode_t {}
unsafe impl Send for cusparseAction_t {}
unsafe impl Sync for cusparseAction_t {}
unsafe impl Send for cusparseMatrixType_t {}
unsafe impl Sync for cusparseMatrixType_t {}
unsafe impl Send for cusparseFillMode_t {}
unsafe impl Sync for cusparseFillMode_t {}
unsafe impl Send for cusparseDiagType_t {}
unsafe impl Sync for cusparseDiagType_t {}
unsafe impl Send for cusparseIndexBase_t {}
unsafe impl Sync for cusparseIndexBase_t {}
unsafe impl Send for cusparseOperation_t {}
unsafe impl Sync for cusparseOperation_t {}
unsafe impl Send for cusparseDirection_t {}
unsafe impl Sync for cusparseDirection_t {}
unsafe impl Send for cusparseSolvePolicy_t {}
unsafe impl Sync for cusparseSolvePolicy_t {}
unsafe impl Send for cusparseColorAlg_t {}
unsafe impl Sync for cusparseColorAlg_t {}
unsafe impl Send for cusparseCsr2CscAlg_t {}
unsafe impl Sync for cusparseCsr2CscAlg_t {}
unsafe impl Send for cusparseFormat_t {}
unsafe impl Sync for cusparseFormat_t {}
unsafe impl Send for cusparseOrder_t {}
unsafe impl Sync for cusparseOrder_t {}
unsafe impl Send for cusparseIndexType_t {}
unsafe impl Sync for cusparseIndexType_t {}
unsafe impl Send for cusparseSpVecDescr {}
unsafe impl Sync for cusparseSpVecDescr {}
unsafe impl Send for cusparseDnVecDescr {}
unsafe impl Sync for cusparseDnVecDescr {}
unsafe impl Send for cusparseSpMatDescr {}
unsafe impl Sync for cusparseSpMatDescr {}
unsafe impl Send for cusparseDnMatDescr {}
unsafe impl Sync for cusparseDnMatDescr {}
unsafe impl Send for cusparseSpMatAttribute_t {}
unsafe impl Sync for cusparseSpMatAttribute_t {}
unsafe impl Send for cusparseSparseToDenseAlg_t {}
unsafe impl Sync for cusparseSparseToDenseAlg_t {}
unsafe impl Send for cusparseDenseToSparseAlg_t {}
unsafe impl Sync for cusparseDenseToSparseAlg_t {}
unsafe impl Send for cusparseSpMVAlg_t {}
unsafe impl Sync for cusparseSpMVAlg_t {}
unsafe impl Send for cusparseSpSVAlg_t {}
unsafe impl Sync for cusparseSpSVAlg_t {}
unsafe impl Send for cusparseSpSVUpdate_t {}
unsafe impl Sync for cusparseSpSVUpdate_t {}
unsafe impl Send for cusparseSpSVDescr {}
unsafe impl Sync for cusparseSpSVDescr {}
unsafe impl Send for cusparseSpSMAlg_t {}
unsafe impl Sync for cusparseSpSMAlg_t {}
unsafe impl Send for cusparseSpSMUpdate_t {}
unsafe impl Sync for cusparseSpSMUpdate_t {}
unsafe impl Send for cusparseSpSMDescr {}
unsafe impl Sync for cusparseSpSMDescr {}
unsafe impl Send for cusparseSpMMAlg_t {}
unsafe impl Sync for cusparseSpMMAlg_t {}
unsafe impl Send for cusparseSpGEMMAlg_t {}
unsafe impl Sync for cusparseSpGEMMAlg_t {}
unsafe impl Send for cusparseSpGEMMDescr {}
unsafe impl Sync for cusparseSpGEMMDescr {}
unsafe impl Send for cusparseSDDMMAlg_t {}
unsafe impl Sync for cusparseSDDMMAlg_t {}
unsafe impl Send for cusparseSpMMOpPlan {}
unsafe impl Sync for cusparseSpMMOpPlan {}
unsafe impl Send for cusparseSpMMOpAlg_t {}
unsafe impl Sync for cusparseSpMMOpAlg_t {}
