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
pub const CUSOLVER_VER_MAJOR: u32 = 12;
pub const CUSOLVER_VER_MINOR: u32 = 2;
pub const CUSOLVER_VER_PATCH: u32 = 0;
pub const CUSOLVER_VER_BUILD: u32 = 1;
pub const CUSOLVER_VERSION: u32 = 12200;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cusolverDnContext {
    _unused: [u8; 0],
}
pub type cusolverDnHandle_t = *mut cusolverDnContext;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct syevjInfo {
    _unused: [u8; 0],
}
pub type syevjInfo_t = *mut syevjInfo;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gesvdjInfo {
    _unused: [u8; 0],
}
pub type gesvdjInfo_t = *mut gesvdjInfo;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cusolverDnIRSParams {
    _unused: [u8; 0],
}
pub type cusolverDnIRSParams_t = *mut cusolverDnIRSParams;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cusolverDnIRSInfos {
    _unused: [u8; 0],
}
pub type cusolverDnIRSInfos_t = *mut cusolverDnIRSInfos;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cusolverDnParams {
    _unused: [u8; 0],
}
pub type cusolverDnParams_t = *mut cusolverDnParams;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cusolverDnFunction_t {
    CUSOLVERDN_GETRF = 0,
    CUSOLVERDN_POTRF = 1,
    CUSOLVERDN_SYEVBATCHED = 2,
    CUSOLVERDN_GEQRF = 3,
}
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
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum libraryPropertyType_t {
    MAJOR_VERSION = 0,
    MINOR_VERSION = 1,
    PATCH_LEVEL = 2,
}
pub use self::libraryPropertyType_t as libraryPropertyType;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cublasFillMode_t {
    CUBLAS_FILL_MODE_LOWER = 0,
    CUBLAS_FILL_MODE_UPPER = 1,
    CUBLAS_FILL_MODE_FULL = 2,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cublasDiagType_t {
    CUBLAS_DIAG_NON_UNIT = 0,
    CUBLAS_DIAG_UNIT = 1,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cublasSideMode_t {
    CUBLAS_SIDE_LEFT = 0,
    CUBLAS_SIDE_RIGHT = 1,
}
impl cublasOperation_t {
    pub const CUBLAS_OP_HERMITAN: cublasOperation_t = cublasOperation_t::CUBLAS_OP_C;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cublasOperation_t {
    CUBLAS_OP_N = 0,
    CUBLAS_OP_T = 1,
    CUBLAS_OP_C = 2,
    CUBLAS_OP_CONJG = 3,
}
pub type cusolver_int_t = ::std::os::raw::c_int;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cusolverStatus_t {
    CUSOLVER_STATUS_SUCCESS = 0,
    CUSOLVER_STATUS_NOT_INITIALIZED = 1,
    CUSOLVER_STATUS_ALLOC_FAILED = 2,
    CUSOLVER_STATUS_INVALID_VALUE = 3,
    CUSOLVER_STATUS_ARCH_MISMATCH = 4,
    CUSOLVER_STATUS_MAPPING_ERROR = 5,
    CUSOLVER_STATUS_EXECUTION_FAILED = 6,
    CUSOLVER_STATUS_INTERNAL_ERROR = 7,
    CUSOLVER_STATUS_MATRIX_TYPE_NOT_SUPPORTED = 8,
    CUSOLVER_STATUS_NOT_SUPPORTED = 9,
    CUSOLVER_STATUS_ZERO_PIVOT = 10,
    CUSOLVER_STATUS_INVALID_LICENSE = 11,
    CUSOLVER_STATUS_IRS_PARAMS_NOT_INITIALIZED = 12,
    CUSOLVER_STATUS_IRS_PARAMS_INVALID = 13,
    CUSOLVER_STATUS_IRS_PARAMS_INVALID_PREC = 14,
    CUSOLVER_STATUS_IRS_PARAMS_INVALID_REFINE = 15,
    CUSOLVER_STATUS_IRS_PARAMS_INVALID_MAXITER = 16,
    CUSOLVER_STATUS_IRS_INTERNAL_ERROR = 20,
    CUSOLVER_STATUS_IRS_NOT_SUPPORTED = 21,
    CUSOLVER_STATUS_IRS_OUT_OF_RANGE = 22,
    CUSOLVER_STATUS_IRS_NRHS_NOT_SUPPORTED_FOR_REFINE_GMRES = 23,
    CUSOLVER_STATUS_IRS_INFOS_NOT_INITIALIZED = 25,
    CUSOLVER_STATUS_IRS_INFOS_NOT_DESTROYED = 26,
    CUSOLVER_STATUS_IRS_MATRIX_SINGULAR = 30,
    CUSOLVER_STATUS_INVALID_WORKSPACE = 31,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cusolverEigType_t {
    CUSOLVER_EIG_TYPE_1 = 1,
    CUSOLVER_EIG_TYPE_2 = 2,
    CUSOLVER_EIG_TYPE_3 = 3,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cusolverEigMode_t {
    CUSOLVER_EIG_MODE_NOVECTOR = 0,
    CUSOLVER_EIG_MODE_VECTOR = 1,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cusolverEigRange_t {
    CUSOLVER_EIG_RANGE_ALL = 1001,
    CUSOLVER_EIG_RANGE_I = 1002,
    CUSOLVER_EIG_RANGE_V = 1003,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cusolverEigComp_t {
    CUSOLVER_EIG_COMP_N = 10,
    CUSOLVER_EIG_COMP_I = 11,
    CUSOLVER_EIG_COMP_V = 12,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cusolverNorm_t {
    CUSOLVER_INF_NORM = 104,
    CUSOLVER_MAX_NORM = 105,
    CUSOLVER_ONE_NORM = 106,
    CUSOLVER_FRO_NORM = 107,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cusolverIRSRefinement_t {
    CUSOLVER_IRS_REFINE_NOT_SET = 1100,
    CUSOLVER_IRS_REFINE_NONE = 1101,
    CUSOLVER_IRS_REFINE_CLASSICAL = 1102,
    CUSOLVER_IRS_REFINE_CLASSICAL_GMRES = 1103,
    CUSOLVER_IRS_REFINE_GMRES = 1104,
    CUSOLVER_IRS_REFINE_GMRES_GMRES = 1105,
    CUSOLVER_IRS_REFINE_GMRES_NOPCOND = 1106,
    CUSOLVER_PREC_DD = 1150,
    CUSOLVER_PREC_SS = 1151,
    CUSOLVER_PREC_SHT = 1152,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cusolverPrecType_t {
    CUSOLVER_R_8I = 1201,
    CUSOLVER_R_8U = 1202,
    CUSOLVER_R_64F = 1203,
    CUSOLVER_R_32F = 1204,
    CUSOLVER_R_16F = 1205,
    CUSOLVER_R_16BF = 1206,
    CUSOLVER_R_TF32 = 1207,
    CUSOLVER_R_AP = 1208,
    CUSOLVER_C_8I = 1211,
    CUSOLVER_C_8U = 1212,
    CUSOLVER_C_64F = 1213,
    CUSOLVER_C_32F = 1214,
    CUSOLVER_C_16F = 1215,
    CUSOLVER_C_16BF = 1216,
    CUSOLVER_C_TF32 = 1217,
    CUSOLVER_C_AP = 1218,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cusolverAlgMode_t {
    CUSOLVER_ALG_0 = 0,
    CUSOLVER_ALG_1 = 1,
    CUSOLVER_ALG_2 = 2,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cusolverStorevMode_t {
    CUBLAS_STOREV_COLUMNWISE = 0,
    CUBLAS_STOREV_ROWWISE = 1,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cusolverDirectMode_t {
    CUBLAS_DIRECT_FORWARD = 0,
    CUBLAS_DIRECT_BACKWARD = 1,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cusolverDeterministicMode_t {
    CUSOLVER_DETERMINISTIC_RESULTS = 1,
    CUSOLVER_ALLOW_NON_DETERMINISTIC_RESULTS = 2,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cusolverMathMode_t {
    CUSOLVER_DEFAULT_MATH = 1,
    CUSOLVER_FP32_EMULATED_BF16X9_MATH = 2,
    CUSOLVER_FP64_EMULATED_FIXEDPOINT_MATH = 4,
    CUSOLVER_FP32_FP64_EMULATED_MATH = 6,
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverGetProperty(type_: libraryPropertyType, value: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverGetVersion(version: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnCreate(handle: *mut cusolverDnHandle_t) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDestroy(handle: cusolverDnHandle_t) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSetStream(handle: cusolverDnHandle_t, streamId: cudaStream_t) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnGetStream(handle: cusolverDnHandle_t, streamId: *mut cudaStream_t) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSetDeterministicMode(handle: cusolverDnHandle_t, mode: cusolverDeterministicMode_t) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnGetDeterministicMode(handle: cusolverDnHandle_t, mode: *mut cusolverDeterministicMode_t) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSetMathMode(handle: cusolverDnHandle_t, mode: cusolverMathMode_t) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnGetMathMode(handle: cusolverDnHandle_t, mode: *mut cusolverMathMode_t) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSetEmulationStrategy(handle: cusolverDnHandle_t, strategy: cudaEmulationStrategy_t) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnGetEmulationStrategy(handle: cusolverDnHandle_t, strategy: *mut cudaEmulationStrategy_t) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSetFixedPointEmulationMantissaControl(handle: cusolverDnHandle_t, control: cudaEmulationMantissaControl_t) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnGetFixedPointEmulationMantissaControl(handle: cusolverDnHandle_t, control: *mut cudaEmulationMantissaControl_t) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSetFixedPointEmulationMaxMantissaBitCount(handle: cusolverDnHandle_t, mantissaBitCount: ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnGetFixedPointEmulationMaxMantissaBitCount(handle: cusolverDnHandle_t, mantissaBitCount: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSetFixedPointEmulationMantissaBitOffset(handle: cusolverDnHandle_t, mantissaBitOffset: ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnGetFixedPointEmulationMantissaBitOffset(handle: cusolverDnHandle_t, mantissaBitOffset: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSetEmulationSpecialValuesSupport(handle: cusolverDnHandle_t, mask: cudaEmulationSpecialValuesSupport_t) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnGetEmulationSpecialValuesSupport(handle: cusolverDnHandle_t, mask: *mut cudaEmulationSpecialValuesSupport_t) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnIRSParamsCreate(params_ptr: *mut cusolverDnIRSParams_t) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnIRSParamsDestroy(params: cusolverDnIRSParams_t) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnIRSParamsSetRefinementSolver(params: cusolverDnIRSParams_t, refinement_solver: cusolverIRSRefinement_t) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnIRSParamsSetSolverMainPrecision(params: cusolverDnIRSParams_t, solver_main_precision: cusolverPrecType_t) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnIRSParamsSetSolverLowestPrecision(params: cusolverDnIRSParams_t, solver_lowest_precision: cusolverPrecType_t) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnIRSParamsSetSolverPrecisions(params: cusolverDnIRSParams_t, solver_main_precision: cusolverPrecType_t, solver_lowest_precision: cusolverPrecType_t) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnIRSParamsSetTol(params: cusolverDnIRSParams_t, val: f64) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnIRSParamsSetTolInner(params: cusolverDnIRSParams_t, val: f64) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnIRSParamsSetMaxIters(params: cusolverDnIRSParams_t, maxiters: cusolver_int_t) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnIRSParamsSetMaxItersInner(params: cusolverDnIRSParams_t, maxiters_inner: cusolver_int_t) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnIRSParamsGetMaxIters(params: cusolverDnIRSParams_t, maxiters: *mut cusolver_int_t) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnIRSParamsEnableFallback(params: cusolverDnIRSParams_t) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnIRSParamsDisableFallback(params: cusolverDnIRSParams_t) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnIRSInfosDestroy(infos: cusolverDnIRSInfos_t) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnIRSInfosCreate(infos_ptr: *mut cusolverDnIRSInfos_t) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnIRSInfosGetNiters(infos: cusolverDnIRSInfos_t, niters: *mut cusolver_int_t) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnIRSInfosGetOuterNiters(infos: cusolverDnIRSInfos_t, outer_niters: *mut cusolver_int_t) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnIRSInfosRequestResidual(infos: cusolverDnIRSInfos_t) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnIRSInfosGetResidualHistory(infos: cusolverDnIRSInfos_t, residual_history: *mut *mut ::std::os::raw::c_void) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnIRSInfosGetMaxIters(infos: cusolverDnIRSInfos_t, maxiters: *mut cusolver_int_t) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZZgesv(
        handle: cusolverDnHandle_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut cuDoubleComplex,
        ldda: cusolver_int_t,
        dipiv: *mut cusolver_int_t,
        dB: *mut cuDoubleComplex,
        lddb: cusolver_int_t,
        dX: *mut cuDoubleComplex,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: usize,
        iter: *mut cusolver_int_t,
        d_info: *mut cusolver_int_t,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZCgesv(
        handle: cusolverDnHandle_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut cuDoubleComplex,
        ldda: cusolver_int_t,
        dipiv: *mut cusolver_int_t,
        dB: *mut cuDoubleComplex,
        lddb: cusolver_int_t,
        dX: *mut cuDoubleComplex,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: usize,
        iter: *mut cusolver_int_t,
        d_info: *mut cusolver_int_t,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZKgesv(
        handle: cusolverDnHandle_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut cuDoubleComplex,
        ldda: cusolver_int_t,
        dipiv: *mut cusolver_int_t,
        dB: *mut cuDoubleComplex,
        lddb: cusolver_int_t,
        dX: *mut cuDoubleComplex,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: usize,
        iter: *mut cusolver_int_t,
        d_info: *mut cusolver_int_t,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZEgesv(
        handle: cusolverDnHandle_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut cuDoubleComplex,
        ldda: cusolver_int_t,
        dipiv: *mut cusolver_int_t,
        dB: *mut cuDoubleComplex,
        lddb: cusolver_int_t,
        dX: *mut cuDoubleComplex,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: usize,
        iter: *mut cusolver_int_t,
        d_info: *mut cusolver_int_t,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZYgesv(
        handle: cusolverDnHandle_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut cuDoubleComplex,
        ldda: cusolver_int_t,
        dipiv: *mut cusolver_int_t,
        dB: *mut cuDoubleComplex,
        lddb: cusolver_int_t,
        dX: *mut cuDoubleComplex,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: usize,
        iter: *mut cusolver_int_t,
        d_info: *mut cusolver_int_t,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnCCgesv(
        handle: cusolverDnHandle_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut cuComplex,
        ldda: cusolver_int_t,
        dipiv: *mut cusolver_int_t,
        dB: *mut cuComplex,
        lddb: cusolver_int_t,
        dX: *mut cuComplex,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: usize,
        iter: *mut cusolver_int_t,
        d_info: *mut cusolver_int_t,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnCEgesv(
        handle: cusolverDnHandle_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut cuComplex,
        ldda: cusolver_int_t,
        dipiv: *mut cusolver_int_t,
        dB: *mut cuComplex,
        lddb: cusolver_int_t,
        dX: *mut cuComplex,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: usize,
        iter: *mut cusolver_int_t,
        d_info: *mut cusolver_int_t,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnCKgesv(
        handle: cusolverDnHandle_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut cuComplex,
        ldda: cusolver_int_t,
        dipiv: *mut cusolver_int_t,
        dB: *mut cuComplex,
        lddb: cusolver_int_t,
        dX: *mut cuComplex,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: usize,
        iter: *mut cusolver_int_t,
        d_info: *mut cusolver_int_t,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnCYgesv(
        handle: cusolverDnHandle_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut cuComplex,
        ldda: cusolver_int_t,
        dipiv: *mut cusolver_int_t,
        dB: *mut cuComplex,
        lddb: cusolver_int_t,
        dX: *mut cuComplex,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: usize,
        iter: *mut cusolver_int_t,
        d_info: *mut cusolver_int_t,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDDgesv(
        handle: cusolverDnHandle_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut f64,
        ldda: cusolver_int_t,
        dipiv: *mut cusolver_int_t,
        dB: *mut f64,
        lddb: cusolver_int_t,
        dX: *mut f64,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: usize,
        iter: *mut cusolver_int_t,
        d_info: *mut cusolver_int_t,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDSgesv(
        handle: cusolverDnHandle_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut f64,
        ldda: cusolver_int_t,
        dipiv: *mut cusolver_int_t,
        dB: *mut f64,
        lddb: cusolver_int_t,
        dX: *mut f64,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: usize,
        iter: *mut cusolver_int_t,
        d_info: *mut cusolver_int_t,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDHgesv(
        handle: cusolverDnHandle_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut f64,
        ldda: cusolver_int_t,
        dipiv: *mut cusolver_int_t,
        dB: *mut f64,
        lddb: cusolver_int_t,
        dX: *mut f64,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: usize,
        iter: *mut cusolver_int_t,
        d_info: *mut cusolver_int_t,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDBgesv(
        handle: cusolverDnHandle_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut f64,
        ldda: cusolver_int_t,
        dipiv: *mut cusolver_int_t,
        dB: *mut f64,
        lddb: cusolver_int_t,
        dX: *mut f64,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: usize,
        iter: *mut cusolver_int_t,
        d_info: *mut cusolver_int_t,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDXgesv(
        handle: cusolverDnHandle_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut f64,
        ldda: cusolver_int_t,
        dipiv: *mut cusolver_int_t,
        dB: *mut f64,
        lddb: cusolver_int_t,
        dX: *mut f64,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: usize,
        iter: *mut cusolver_int_t,
        d_info: *mut cusolver_int_t,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSSgesv(
        handle: cusolverDnHandle_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut f32,
        ldda: cusolver_int_t,
        dipiv: *mut cusolver_int_t,
        dB: *mut f32,
        lddb: cusolver_int_t,
        dX: *mut f32,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: usize,
        iter: *mut cusolver_int_t,
        d_info: *mut cusolver_int_t,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSHgesv(
        handle: cusolverDnHandle_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut f32,
        ldda: cusolver_int_t,
        dipiv: *mut cusolver_int_t,
        dB: *mut f32,
        lddb: cusolver_int_t,
        dX: *mut f32,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: usize,
        iter: *mut cusolver_int_t,
        d_info: *mut cusolver_int_t,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSBgesv(
        handle: cusolverDnHandle_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut f32,
        ldda: cusolver_int_t,
        dipiv: *mut cusolver_int_t,
        dB: *mut f32,
        lddb: cusolver_int_t,
        dX: *mut f32,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: usize,
        iter: *mut cusolver_int_t,
        d_info: *mut cusolver_int_t,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSXgesv(
        handle: cusolverDnHandle_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut f32,
        ldda: cusolver_int_t,
        dipiv: *mut cusolver_int_t,
        dB: *mut f32,
        lddb: cusolver_int_t,
        dX: *mut f32,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: usize,
        iter: *mut cusolver_int_t,
        d_info: *mut cusolver_int_t,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZZgesv_bufferSize(
        handle: cusolverDnHandle_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut cuDoubleComplex,
        ldda: cusolver_int_t,
        dipiv: *mut cusolver_int_t,
        dB: *mut cuDoubleComplex,
        lddb: cusolver_int_t,
        dX: *mut cuDoubleComplex,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: *mut usize,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZCgesv_bufferSize(
        handle: cusolverDnHandle_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut cuDoubleComplex,
        ldda: cusolver_int_t,
        dipiv: *mut cusolver_int_t,
        dB: *mut cuDoubleComplex,
        lddb: cusolver_int_t,
        dX: *mut cuDoubleComplex,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: *mut usize,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZKgesv_bufferSize(
        handle: cusolverDnHandle_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut cuDoubleComplex,
        ldda: cusolver_int_t,
        dipiv: *mut cusolver_int_t,
        dB: *mut cuDoubleComplex,
        lddb: cusolver_int_t,
        dX: *mut cuDoubleComplex,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: *mut usize,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZEgesv_bufferSize(
        handle: cusolverDnHandle_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut cuDoubleComplex,
        ldda: cusolver_int_t,
        dipiv: *mut cusolver_int_t,
        dB: *mut cuDoubleComplex,
        lddb: cusolver_int_t,
        dX: *mut cuDoubleComplex,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: *mut usize,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZYgesv_bufferSize(
        handle: cusolverDnHandle_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut cuDoubleComplex,
        ldda: cusolver_int_t,
        dipiv: *mut cusolver_int_t,
        dB: *mut cuDoubleComplex,
        lddb: cusolver_int_t,
        dX: *mut cuDoubleComplex,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: *mut usize,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnCCgesv_bufferSize(
        handle: cusolverDnHandle_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut cuComplex,
        ldda: cusolver_int_t,
        dipiv: *mut cusolver_int_t,
        dB: *mut cuComplex,
        lddb: cusolver_int_t,
        dX: *mut cuComplex,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: *mut usize,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnCKgesv_bufferSize(
        handle: cusolverDnHandle_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut cuComplex,
        ldda: cusolver_int_t,
        dipiv: *mut cusolver_int_t,
        dB: *mut cuComplex,
        lddb: cusolver_int_t,
        dX: *mut cuComplex,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: *mut usize,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnCEgesv_bufferSize(
        handle: cusolverDnHandle_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut cuComplex,
        ldda: cusolver_int_t,
        dipiv: *mut cusolver_int_t,
        dB: *mut cuComplex,
        lddb: cusolver_int_t,
        dX: *mut cuComplex,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: *mut usize,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnCYgesv_bufferSize(
        handle: cusolverDnHandle_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut cuComplex,
        ldda: cusolver_int_t,
        dipiv: *mut cusolver_int_t,
        dB: *mut cuComplex,
        lddb: cusolver_int_t,
        dX: *mut cuComplex,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: *mut usize,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDDgesv_bufferSize(
        handle: cusolverDnHandle_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut f64,
        ldda: cusolver_int_t,
        dipiv: *mut cusolver_int_t,
        dB: *mut f64,
        lddb: cusolver_int_t,
        dX: *mut f64,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: *mut usize,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDSgesv_bufferSize(
        handle: cusolverDnHandle_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut f64,
        ldda: cusolver_int_t,
        dipiv: *mut cusolver_int_t,
        dB: *mut f64,
        lddb: cusolver_int_t,
        dX: *mut f64,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: *mut usize,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDHgesv_bufferSize(
        handle: cusolverDnHandle_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut f64,
        ldda: cusolver_int_t,
        dipiv: *mut cusolver_int_t,
        dB: *mut f64,
        lddb: cusolver_int_t,
        dX: *mut f64,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: *mut usize,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDBgesv_bufferSize(
        handle: cusolverDnHandle_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut f64,
        ldda: cusolver_int_t,
        dipiv: *mut cusolver_int_t,
        dB: *mut f64,
        lddb: cusolver_int_t,
        dX: *mut f64,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: *mut usize,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDXgesv_bufferSize(
        handle: cusolverDnHandle_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut f64,
        ldda: cusolver_int_t,
        dipiv: *mut cusolver_int_t,
        dB: *mut f64,
        lddb: cusolver_int_t,
        dX: *mut f64,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: *mut usize,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSSgesv_bufferSize(
        handle: cusolverDnHandle_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut f32,
        ldda: cusolver_int_t,
        dipiv: *mut cusolver_int_t,
        dB: *mut f32,
        lddb: cusolver_int_t,
        dX: *mut f32,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: *mut usize,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSHgesv_bufferSize(
        handle: cusolverDnHandle_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut f32,
        ldda: cusolver_int_t,
        dipiv: *mut cusolver_int_t,
        dB: *mut f32,
        lddb: cusolver_int_t,
        dX: *mut f32,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: *mut usize,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSBgesv_bufferSize(
        handle: cusolverDnHandle_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut f32,
        ldda: cusolver_int_t,
        dipiv: *mut cusolver_int_t,
        dB: *mut f32,
        lddb: cusolver_int_t,
        dX: *mut f32,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: *mut usize,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSXgesv_bufferSize(
        handle: cusolverDnHandle_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut f32,
        ldda: cusolver_int_t,
        dipiv: *mut cusolver_int_t,
        dB: *mut f32,
        lddb: cusolver_int_t,
        dX: *mut f32,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: *mut usize,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZZgels(
        handle: cusolverDnHandle_t,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut cuDoubleComplex,
        ldda: cusolver_int_t,
        dB: *mut cuDoubleComplex,
        lddb: cusolver_int_t,
        dX: *mut cuDoubleComplex,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: usize,
        iter: *mut cusolver_int_t,
        d_info: *mut cusolver_int_t,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZCgels(
        handle: cusolverDnHandle_t,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut cuDoubleComplex,
        ldda: cusolver_int_t,
        dB: *mut cuDoubleComplex,
        lddb: cusolver_int_t,
        dX: *mut cuDoubleComplex,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: usize,
        iter: *mut cusolver_int_t,
        d_info: *mut cusolver_int_t,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZKgels(
        handle: cusolverDnHandle_t,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut cuDoubleComplex,
        ldda: cusolver_int_t,
        dB: *mut cuDoubleComplex,
        lddb: cusolver_int_t,
        dX: *mut cuDoubleComplex,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: usize,
        iter: *mut cusolver_int_t,
        d_info: *mut cusolver_int_t,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZEgels(
        handle: cusolverDnHandle_t,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut cuDoubleComplex,
        ldda: cusolver_int_t,
        dB: *mut cuDoubleComplex,
        lddb: cusolver_int_t,
        dX: *mut cuDoubleComplex,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: usize,
        iter: *mut cusolver_int_t,
        d_info: *mut cusolver_int_t,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZYgels(
        handle: cusolverDnHandle_t,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut cuDoubleComplex,
        ldda: cusolver_int_t,
        dB: *mut cuDoubleComplex,
        lddb: cusolver_int_t,
        dX: *mut cuDoubleComplex,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: usize,
        iter: *mut cusolver_int_t,
        d_info: *mut cusolver_int_t,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnCCgels(
        handle: cusolverDnHandle_t,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut cuComplex,
        ldda: cusolver_int_t,
        dB: *mut cuComplex,
        lddb: cusolver_int_t,
        dX: *mut cuComplex,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: usize,
        iter: *mut cusolver_int_t,
        d_info: *mut cusolver_int_t,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnCKgels(
        handle: cusolverDnHandle_t,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut cuComplex,
        ldda: cusolver_int_t,
        dB: *mut cuComplex,
        lddb: cusolver_int_t,
        dX: *mut cuComplex,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: usize,
        iter: *mut cusolver_int_t,
        d_info: *mut cusolver_int_t,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnCEgels(
        handle: cusolverDnHandle_t,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut cuComplex,
        ldda: cusolver_int_t,
        dB: *mut cuComplex,
        lddb: cusolver_int_t,
        dX: *mut cuComplex,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: usize,
        iter: *mut cusolver_int_t,
        d_info: *mut cusolver_int_t,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnCYgels(
        handle: cusolverDnHandle_t,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut cuComplex,
        ldda: cusolver_int_t,
        dB: *mut cuComplex,
        lddb: cusolver_int_t,
        dX: *mut cuComplex,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: usize,
        iter: *mut cusolver_int_t,
        d_info: *mut cusolver_int_t,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDDgels(
        handle: cusolverDnHandle_t,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut f64,
        ldda: cusolver_int_t,
        dB: *mut f64,
        lddb: cusolver_int_t,
        dX: *mut f64,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: usize,
        iter: *mut cusolver_int_t,
        d_info: *mut cusolver_int_t,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDSgels(
        handle: cusolverDnHandle_t,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut f64,
        ldda: cusolver_int_t,
        dB: *mut f64,
        lddb: cusolver_int_t,
        dX: *mut f64,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: usize,
        iter: *mut cusolver_int_t,
        d_info: *mut cusolver_int_t,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDHgels(
        handle: cusolverDnHandle_t,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut f64,
        ldda: cusolver_int_t,
        dB: *mut f64,
        lddb: cusolver_int_t,
        dX: *mut f64,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: usize,
        iter: *mut cusolver_int_t,
        d_info: *mut cusolver_int_t,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDBgels(
        handle: cusolverDnHandle_t,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut f64,
        ldda: cusolver_int_t,
        dB: *mut f64,
        lddb: cusolver_int_t,
        dX: *mut f64,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: usize,
        iter: *mut cusolver_int_t,
        d_info: *mut cusolver_int_t,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDXgels(
        handle: cusolverDnHandle_t,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut f64,
        ldda: cusolver_int_t,
        dB: *mut f64,
        lddb: cusolver_int_t,
        dX: *mut f64,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: usize,
        iter: *mut cusolver_int_t,
        d_info: *mut cusolver_int_t,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSSgels(
        handle: cusolverDnHandle_t,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut f32,
        ldda: cusolver_int_t,
        dB: *mut f32,
        lddb: cusolver_int_t,
        dX: *mut f32,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: usize,
        iter: *mut cusolver_int_t,
        d_info: *mut cusolver_int_t,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSHgels(
        handle: cusolverDnHandle_t,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut f32,
        ldda: cusolver_int_t,
        dB: *mut f32,
        lddb: cusolver_int_t,
        dX: *mut f32,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: usize,
        iter: *mut cusolver_int_t,
        d_info: *mut cusolver_int_t,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSBgels(
        handle: cusolverDnHandle_t,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut f32,
        ldda: cusolver_int_t,
        dB: *mut f32,
        lddb: cusolver_int_t,
        dX: *mut f32,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: usize,
        iter: *mut cusolver_int_t,
        d_info: *mut cusolver_int_t,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSXgels(
        handle: cusolverDnHandle_t,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut f32,
        ldda: cusolver_int_t,
        dB: *mut f32,
        lddb: cusolver_int_t,
        dX: *mut f32,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: usize,
        iter: *mut cusolver_int_t,
        d_info: *mut cusolver_int_t,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZZgels_bufferSize(
        handle: cusolverDnHandle_t,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut cuDoubleComplex,
        ldda: cusolver_int_t,
        dB: *mut cuDoubleComplex,
        lddb: cusolver_int_t,
        dX: *mut cuDoubleComplex,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: *mut usize,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZCgels_bufferSize(
        handle: cusolverDnHandle_t,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut cuDoubleComplex,
        ldda: cusolver_int_t,
        dB: *mut cuDoubleComplex,
        lddb: cusolver_int_t,
        dX: *mut cuDoubleComplex,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: *mut usize,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZKgels_bufferSize(
        handle: cusolverDnHandle_t,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut cuDoubleComplex,
        ldda: cusolver_int_t,
        dB: *mut cuDoubleComplex,
        lddb: cusolver_int_t,
        dX: *mut cuDoubleComplex,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: *mut usize,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZEgels_bufferSize(
        handle: cusolverDnHandle_t,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut cuDoubleComplex,
        ldda: cusolver_int_t,
        dB: *mut cuDoubleComplex,
        lddb: cusolver_int_t,
        dX: *mut cuDoubleComplex,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: *mut usize,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZYgels_bufferSize(
        handle: cusolverDnHandle_t,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut cuDoubleComplex,
        ldda: cusolver_int_t,
        dB: *mut cuDoubleComplex,
        lddb: cusolver_int_t,
        dX: *mut cuDoubleComplex,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: *mut usize,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnCCgels_bufferSize(
        handle: cusolverDnHandle_t,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut cuComplex,
        ldda: cusolver_int_t,
        dB: *mut cuComplex,
        lddb: cusolver_int_t,
        dX: *mut cuComplex,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: *mut usize,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnCKgels_bufferSize(
        handle: cusolverDnHandle_t,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut cuComplex,
        ldda: cusolver_int_t,
        dB: *mut cuComplex,
        lddb: cusolver_int_t,
        dX: *mut cuComplex,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: *mut usize,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnCEgels_bufferSize(
        handle: cusolverDnHandle_t,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut cuComplex,
        ldda: cusolver_int_t,
        dB: *mut cuComplex,
        lddb: cusolver_int_t,
        dX: *mut cuComplex,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: *mut usize,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnCYgels_bufferSize(
        handle: cusolverDnHandle_t,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut cuComplex,
        ldda: cusolver_int_t,
        dB: *mut cuComplex,
        lddb: cusolver_int_t,
        dX: *mut cuComplex,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: *mut usize,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDDgels_bufferSize(
        handle: cusolverDnHandle_t,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut f64,
        ldda: cusolver_int_t,
        dB: *mut f64,
        lddb: cusolver_int_t,
        dX: *mut f64,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: *mut usize,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDSgels_bufferSize(
        handle: cusolverDnHandle_t,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut f64,
        ldda: cusolver_int_t,
        dB: *mut f64,
        lddb: cusolver_int_t,
        dX: *mut f64,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: *mut usize,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDHgels_bufferSize(
        handle: cusolverDnHandle_t,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut f64,
        ldda: cusolver_int_t,
        dB: *mut f64,
        lddb: cusolver_int_t,
        dX: *mut f64,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: *mut usize,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDBgels_bufferSize(
        handle: cusolverDnHandle_t,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut f64,
        ldda: cusolver_int_t,
        dB: *mut f64,
        lddb: cusolver_int_t,
        dX: *mut f64,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: *mut usize,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDXgels_bufferSize(
        handle: cusolverDnHandle_t,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut f64,
        ldda: cusolver_int_t,
        dB: *mut f64,
        lddb: cusolver_int_t,
        dX: *mut f64,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: *mut usize,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSSgels_bufferSize(
        handle: cusolverDnHandle_t,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut f32,
        ldda: cusolver_int_t,
        dB: *mut f32,
        lddb: cusolver_int_t,
        dX: *mut f32,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: *mut usize,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSHgels_bufferSize(
        handle: cusolverDnHandle_t,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut f32,
        ldda: cusolver_int_t,
        dB: *mut f32,
        lddb: cusolver_int_t,
        dX: *mut f32,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: *mut usize,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSBgels_bufferSize(
        handle: cusolverDnHandle_t,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut f32,
        ldda: cusolver_int_t,
        dB: *mut f32,
        lddb: cusolver_int_t,
        dX: *mut f32,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: *mut usize,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSXgels_bufferSize(
        handle: cusolverDnHandle_t,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut f32,
        ldda: cusolver_int_t,
        dB: *mut f32,
        lddb: cusolver_int_t,
        dX: *mut f32,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: *mut usize,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnIRSXgesv(
        handle: cusolverDnHandle_t,
        gesv_irs_params: cusolverDnIRSParams_t,
        gesv_irs_infos: cusolverDnIRSInfos_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut ::std::os::raw::c_void,
        ldda: cusolver_int_t,
        dB: *mut ::std::os::raw::c_void,
        lddb: cusolver_int_t,
        dX: *mut ::std::os::raw::c_void,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: usize,
        niters: *mut cusolver_int_t,
        d_info: *mut cusolver_int_t,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnIRSXgesv_bufferSize(handle: cusolverDnHandle_t, params: cusolverDnIRSParams_t, n: cusolver_int_t, nrhs: cusolver_int_t, lwork_bytes: *mut usize) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnIRSXgels(
        handle: cusolverDnHandle_t,
        gels_irs_params: cusolverDnIRSParams_t,
        gels_irs_infos: cusolverDnIRSInfos_t,
        m: cusolver_int_t,
        n: cusolver_int_t,
        nrhs: cusolver_int_t,
        dA: *mut ::std::os::raw::c_void,
        ldda: cusolver_int_t,
        dB: *mut ::std::os::raw::c_void,
        lddb: cusolver_int_t,
        dX: *mut ::std::os::raw::c_void,
        lddx: cusolver_int_t,
        dWorkspace: *mut ::std::os::raw::c_void,
        lwork_bytes: usize,
        niters: *mut cusolver_int_t,
        d_info: *mut cusolver_int_t,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnIRSXgels_bufferSize(handle: cusolverDnHandle_t, params: cusolverDnIRSParams_t, m: cusolver_int_t, n: cusolver_int_t, nrhs: cusolver_int_t, lwork_bytes: *mut usize) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSpotrf_bufferSize(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f32, lda: ::std::os::raw::c_int, Lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDpotrf_bufferSize(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f64, lda: ::std::os::raw::c_int, Lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnCpotrf_bufferSize(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut cuComplex, lda: ::std::os::raw::c_int, Lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZpotrf_bufferSize(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut cuDoubleComplex, lda: ::std::os::raw::c_int, Lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSpotrf(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f32, lda: ::std::os::raw::c_int, Workspace: *mut f32, Lwork: ::std::os::raw::c_int, devInfo: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDpotrf(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f64, lda: ::std::os::raw::c_int, Workspace: *mut f64, Lwork: ::std::os::raw::c_int, devInfo: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnCpotrf(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut cuComplex, lda: ::std::os::raw::c_int, Workspace: *mut cuComplex, Lwork: ::std::os::raw::c_int, devInfo: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZpotrf(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut cuDoubleComplex, lda: ::std::os::raw::c_int, Workspace: *mut cuDoubleComplex, Lwork: ::std::os::raw::c_int, devInfo: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSpotrs(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, nrhs: ::std::os::raw::c_int, A: *const f32, lda: ::std::os::raw::c_int, B: *mut f32, ldb: ::std::os::raw::c_int, devInfo: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDpotrs(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, nrhs: ::std::os::raw::c_int, A: *const f64, lda: ::std::os::raw::c_int, B: *mut f64, ldb: ::std::os::raw::c_int, devInfo: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnCpotrs(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, nrhs: ::std::os::raw::c_int, A: *const cuComplex, lda: ::std::os::raw::c_int, B: *mut cuComplex, ldb: ::std::os::raw::c_int, devInfo: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZpotrs(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, nrhs: ::std::os::raw::c_int, A: *const cuDoubleComplex, lda: ::std::os::raw::c_int, B: *mut cuDoubleComplex, ldb: ::std::os::raw::c_int, devInfo: *mut ::std::os::raw::c_int)
    -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSpotrfBatched(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, Aarray: *mut *mut f32, lda: ::std::os::raw::c_int, infoArray: *mut ::std::os::raw::c_int, batchSize: ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDpotrfBatched(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, Aarray: *mut *mut f64, lda: ::std::os::raw::c_int, infoArray: *mut ::std::os::raw::c_int, batchSize: ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnCpotrfBatched(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, Aarray: *mut *mut cuComplex, lda: ::std::os::raw::c_int, infoArray: *mut ::std::os::raw::c_int, batchSize: ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZpotrfBatched(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, Aarray: *mut *mut cuDoubleComplex, lda: ::std::os::raw::c_int, infoArray: *mut ::std::os::raw::c_int, batchSize: ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSpotrsBatched(
        handle: cusolverDnHandle_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        nrhs: ::std::os::raw::c_int,
        A: *mut *mut f32,
        lda: ::std::os::raw::c_int,
        B: *mut *mut f32,
        ldb: ::std::os::raw::c_int,
        d_info: *mut ::std::os::raw::c_int,
        batchSize: ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDpotrsBatched(
        handle: cusolverDnHandle_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        nrhs: ::std::os::raw::c_int,
        A: *mut *mut f64,
        lda: ::std::os::raw::c_int,
        B: *mut *mut f64,
        ldb: ::std::os::raw::c_int,
        d_info: *mut ::std::os::raw::c_int,
        batchSize: ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnCpotrsBatched(
        handle: cusolverDnHandle_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        nrhs: ::std::os::raw::c_int,
        A: *mut *mut cuComplex,
        lda: ::std::os::raw::c_int,
        B: *mut *mut cuComplex,
        ldb: ::std::os::raw::c_int,
        d_info: *mut ::std::os::raw::c_int,
        batchSize: ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZpotrsBatched(
        handle: cusolverDnHandle_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        nrhs: ::std::os::raw::c_int,
        A: *mut *mut cuDoubleComplex,
        lda: ::std::os::raw::c_int,
        B: *mut *mut cuDoubleComplex,
        ldb: ::std::os::raw::c_int,
        d_info: *mut ::std::os::raw::c_int,
        batchSize: ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSpotri_bufferSize(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f32, lda: ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDpotri_bufferSize(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f64, lda: ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnCpotri_bufferSize(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut cuComplex, lda: ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZpotri_bufferSize(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut cuDoubleComplex, lda: ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSpotri(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f32, lda: ::std::os::raw::c_int, work: *mut f32, lwork: ::std::os::raw::c_int, devInfo: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDpotri(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f64, lda: ::std::os::raw::c_int, work: *mut f64, lwork: ::std::os::raw::c_int, devInfo: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnCpotri(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut cuComplex, lda: ::std::os::raw::c_int, work: *mut cuComplex, lwork: ::std::os::raw::c_int, devInfo: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZpotri(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut cuDoubleComplex, lda: ::std::os::raw::c_int, work: *mut cuDoubleComplex, lwork: ::std::os::raw::c_int, devInfo: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnXtrtri_bufferSize(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, diag: cublasDiagType_t, n: i64, dataTypeA: cudaDataType, A: *mut ::std::os::raw::c_void, lda: i64, workspaceInBytesOnDevice: *mut usize, workspaceInBytesOnHost: *mut usize) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnXtrtri(
        handle: cusolverDnHandle_t,
        uplo: cublasFillMode_t,
        diag: cublasDiagType_t,
        n: i64,
        dataTypeA: cudaDataType,
        A: *mut ::std::os::raw::c_void,
        lda: i64,
        bufferOnDevice: *mut ::std::os::raw::c_void,
        workspaceInBytesOnDevice: usize,
        bufferOnHost: *mut ::std::os::raw::c_void,
        workspaceInBytesOnHost: usize,
        devInfo: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSlauum_bufferSize(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f32, lda: ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDlauum_bufferSize(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f64, lda: ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnClauum_bufferSize(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut cuComplex, lda: ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZlauum_bufferSize(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut cuDoubleComplex, lda: ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSlauum(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f32, lda: ::std::os::raw::c_int, work: *mut f32, lwork: ::std::os::raw::c_int, devInfo: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDlauum(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f64, lda: ::std::os::raw::c_int, work: *mut f64, lwork: ::std::os::raw::c_int, devInfo: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnClauum(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut cuComplex, lda: ::std::os::raw::c_int, work: *mut cuComplex, lwork: ::std::os::raw::c_int, devInfo: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZlauum(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut cuDoubleComplex, lda: ::std::os::raw::c_int, work: *mut cuDoubleComplex, lwork: ::std::os::raw::c_int, devInfo: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSgetrf_bufferSize(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, A: *mut f32, lda: ::std::os::raw::c_int, Lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDgetrf_bufferSize(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, A: *mut f64, lda: ::std::os::raw::c_int, Lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnCgetrf_bufferSize(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, A: *mut cuComplex, lda: ::std::os::raw::c_int, Lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZgetrf_bufferSize(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, A: *mut cuDoubleComplex, lda: ::std::os::raw::c_int, Lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSgetrf(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, A: *mut f32, lda: ::std::os::raw::c_int, Workspace: *mut f32, devIpiv: *mut ::std::os::raw::c_int, devInfo: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDgetrf(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, A: *mut f64, lda: ::std::os::raw::c_int, Workspace: *mut f64, devIpiv: *mut ::std::os::raw::c_int, devInfo: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnCgetrf(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, A: *mut cuComplex, lda: ::std::os::raw::c_int, Workspace: *mut cuComplex, devIpiv: *mut ::std::os::raw::c_int, devInfo: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZgetrf(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, A: *mut cuDoubleComplex, lda: ::std::os::raw::c_int, Workspace: *mut cuDoubleComplex, devIpiv: *mut ::std::os::raw::c_int, devInfo: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSlaswp(handle: cusolverDnHandle_t, n: ::std::os::raw::c_int, A: *mut f32, lda: ::std::os::raw::c_int, k1: ::std::os::raw::c_int, k2: ::std::os::raw::c_int, devIpiv: *const ::std::os::raw::c_int, incx: ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDlaswp(handle: cusolverDnHandle_t, n: ::std::os::raw::c_int, A: *mut f64, lda: ::std::os::raw::c_int, k1: ::std::os::raw::c_int, k2: ::std::os::raw::c_int, devIpiv: *const ::std::os::raw::c_int, incx: ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnClaswp(handle: cusolverDnHandle_t, n: ::std::os::raw::c_int, A: *mut cuComplex, lda: ::std::os::raw::c_int, k1: ::std::os::raw::c_int, k2: ::std::os::raw::c_int, devIpiv: *const ::std::os::raw::c_int, incx: ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZlaswp(handle: cusolverDnHandle_t, n: ::std::os::raw::c_int, A: *mut cuDoubleComplex, lda: ::std::os::raw::c_int, k1: ::std::os::raw::c_int, k2: ::std::os::raw::c_int, devIpiv: *const ::std::os::raw::c_int, incx: ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSgetrs(
        handle: cusolverDnHandle_t,
        trans: cublasOperation_t,
        n: ::std::os::raw::c_int,
        nrhs: ::std::os::raw::c_int,
        A: *const f32,
        lda: ::std::os::raw::c_int,
        devIpiv: *const ::std::os::raw::c_int,
        B: *mut f32,
        ldb: ::std::os::raw::c_int,
        devInfo: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDgetrs(
        handle: cusolverDnHandle_t,
        trans: cublasOperation_t,
        n: ::std::os::raw::c_int,
        nrhs: ::std::os::raw::c_int,
        A: *const f64,
        lda: ::std::os::raw::c_int,
        devIpiv: *const ::std::os::raw::c_int,
        B: *mut f64,
        ldb: ::std::os::raw::c_int,
        devInfo: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnCgetrs(
        handle: cusolverDnHandle_t,
        trans: cublasOperation_t,
        n: ::std::os::raw::c_int,
        nrhs: ::std::os::raw::c_int,
        A: *const cuComplex,
        lda: ::std::os::raw::c_int,
        devIpiv: *const ::std::os::raw::c_int,
        B: *mut cuComplex,
        ldb: ::std::os::raw::c_int,
        devInfo: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZgetrs(
        handle: cusolverDnHandle_t,
        trans: cublasOperation_t,
        n: ::std::os::raw::c_int,
        nrhs: ::std::os::raw::c_int,
        A: *const cuDoubleComplex,
        lda: ::std::os::raw::c_int,
        devIpiv: *const ::std::os::raw::c_int,
        B: *mut cuDoubleComplex,
        ldb: ::std::os::raw::c_int,
        devInfo: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSgeqrf_bufferSize(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, A: *mut f32, lda: ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDgeqrf_bufferSize(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, A: *mut f64, lda: ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnCgeqrf_bufferSize(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, A: *mut cuComplex, lda: ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZgeqrf_bufferSize(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, A: *mut cuDoubleComplex, lda: ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSgeqrf(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, A: *mut f32, lda: ::std::os::raw::c_int, TAU: *mut f32, Workspace: *mut f32, Lwork: ::std::os::raw::c_int, devInfo: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDgeqrf(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, A: *mut f64, lda: ::std::os::raw::c_int, TAU: *mut f64, Workspace: *mut f64, Lwork: ::std::os::raw::c_int, devInfo: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnCgeqrf(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, A: *mut cuComplex, lda: ::std::os::raw::c_int, TAU: *mut cuComplex, Workspace: *mut cuComplex, Lwork: ::std::os::raw::c_int, devInfo: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZgeqrf(
        handle: cusolverDnHandle_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        A: *mut cuDoubleComplex,
        lda: ::std::os::raw::c_int,
        TAU: *mut cuDoubleComplex,
        Workspace: *mut cuDoubleComplex,
        Lwork: ::std::os::raw::c_int,
        devInfo: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSorgqr_bufferSize(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, k: ::std::os::raw::c_int, A: *const f32, lda: ::std::os::raw::c_int, tau: *const f32, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDorgqr_bufferSize(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, k: ::std::os::raw::c_int, A: *const f64, lda: ::std::os::raw::c_int, tau: *const f64, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnCungqr_bufferSize(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, k: ::std::os::raw::c_int, A: *const cuComplex, lda: ::std::os::raw::c_int, tau: *const cuComplex, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZungqr_bufferSize(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, k: ::std::os::raw::c_int, A: *const cuDoubleComplex, lda: ::std::os::raw::c_int, tau: *const cuDoubleComplex, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSorgqr(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, k: ::std::os::raw::c_int, A: *mut f32, lda: ::std::os::raw::c_int, tau: *const f32, work: *mut f32, lwork: ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDorgqr(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, k: ::std::os::raw::c_int, A: *mut f64, lda: ::std::os::raw::c_int, tau: *const f64, work: *mut f64, lwork: ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnCungqr(
        handle: cusolverDnHandle_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        A: *mut cuComplex,
        lda: ::std::os::raw::c_int,
        tau: *const cuComplex,
        work: *mut cuComplex,
        lwork: ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZungqr(
        handle: cusolverDnHandle_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        A: *mut cuDoubleComplex,
        lda: ::std::os::raw::c_int,
        tau: *const cuDoubleComplex,
        work: *mut cuDoubleComplex,
        lwork: ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSormqr_bufferSize(
        handle: cusolverDnHandle_t,
        side: cublasSideMode_t,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        A: *const f32,
        lda: ::std::os::raw::c_int,
        tau: *const f32,
        C: *const f32,
        ldc: ::std::os::raw::c_int,
        lwork: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDormqr_bufferSize(
        handle: cusolverDnHandle_t,
        side: cublasSideMode_t,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        A: *const f64,
        lda: ::std::os::raw::c_int,
        tau: *const f64,
        C: *const f64,
        ldc: ::std::os::raw::c_int,
        lwork: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnCunmqr_bufferSize(
        handle: cusolverDnHandle_t,
        side: cublasSideMode_t,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        A: *const cuComplex,
        lda: ::std::os::raw::c_int,
        tau: *const cuComplex,
        C: *const cuComplex,
        ldc: ::std::os::raw::c_int,
        lwork: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZunmqr_bufferSize(
        handle: cusolverDnHandle_t,
        side: cublasSideMode_t,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        A: *const cuDoubleComplex,
        lda: ::std::os::raw::c_int,
        tau: *const cuDoubleComplex,
        C: *const cuDoubleComplex,
        ldc: ::std::os::raw::c_int,
        lwork: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSormqr(
        handle: cusolverDnHandle_t,
        side: cublasSideMode_t,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        A: *const f32,
        lda: ::std::os::raw::c_int,
        tau: *const f32,
        C: *mut f32,
        ldc: ::std::os::raw::c_int,
        work: *mut f32,
        lwork: ::std::os::raw::c_int,
        devInfo: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDormqr(
        handle: cusolverDnHandle_t,
        side: cublasSideMode_t,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        A: *const f64,
        lda: ::std::os::raw::c_int,
        tau: *const f64,
        C: *mut f64,
        ldc: ::std::os::raw::c_int,
        work: *mut f64,
        lwork: ::std::os::raw::c_int,
        devInfo: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnCunmqr(
        handle: cusolverDnHandle_t,
        side: cublasSideMode_t,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        A: *const cuComplex,
        lda: ::std::os::raw::c_int,
        tau: *const cuComplex,
        C: *mut cuComplex,
        ldc: ::std::os::raw::c_int,
        work: *mut cuComplex,
        lwork: ::std::os::raw::c_int,
        devInfo: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZunmqr(
        handle: cusolverDnHandle_t,
        side: cublasSideMode_t,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        A: *const cuDoubleComplex,
        lda: ::std::os::raw::c_int,
        tau: *const cuDoubleComplex,
        C: *mut cuDoubleComplex,
        ldc: ::std::os::raw::c_int,
        work: *mut cuDoubleComplex,
        lwork: ::std::os::raw::c_int,
        devInfo: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSsytrf_bufferSize(handle: cusolverDnHandle_t, n: ::std::os::raw::c_int, A: *mut f32, lda: ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDsytrf_bufferSize(handle: cusolverDnHandle_t, n: ::std::os::raw::c_int, A: *mut f64, lda: ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnCsytrf_bufferSize(handle: cusolverDnHandle_t, n: ::std::os::raw::c_int, A: *mut cuComplex, lda: ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZsytrf_bufferSize(handle: cusolverDnHandle_t, n: ::std::os::raw::c_int, A: *mut cuDoubleComplex, lda: ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSsytrf(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f32, lda: ::std::os::raw::c_int, ipiv: *mut ::std::os::raw::c_int, work: *mut f32, lwork: ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDsytrf(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f64, lda: ::std::os::raw::c_int, ipiv: *mut ::std::os::raw::c_int, work: *mut f64, lwork: ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnCsytrf(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut cuComplex, lda: ::std::os::raw::c_int, ipiv: *mut ::std::os::raw::c_int, work: *mut cuComplex, lwork: ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZsytrf(
        handle: cusolverDnHandle_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: *mut cuDoubleComplex,
        lda: ::std::os::raw::c_int,
        ipiv: *mut ::std::os::raw::c_int,
        work: *mut cuDoubleComplex,
        lwork: ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnXsytrs_bufferSize(
        handle: cusolverDnHandle_t,
        uplo: cublasFillMode_t,
        n: i64,
        nrhs: i64,
        dataTypeA: cudaDataType,
        A: *const ::std::os::raw::c_void,
        lda: i64,
        ipiv: *const i64,
        dataTypeB: cudaDataType,
        B: *mut ::std::os::raw::c_void,
        ldb: i64,
        workspaceInBytesOnDevice: *mut usize,
        workspaceInBytesOnHost: *mut usize,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnXsytrs(
        handle: cusolverDnHandle_t,
        uplo: cublasFillMode_t,
        n: i64,
        nrhs: i64,
        dataTypeA: cudaDataType,
        A: *const ::std::os::raw::c_void,
        lda: i64,
        ipiv: *const i64,
        dataTypeB: cudaDataType,
        B: *mut ::std::os::raw::c_void,
        ldb: i64,
        bufferOnDevice: *mut ::std::os::raw::c_void,
        workspaceInBytesOnDevice: usize,
        bufferOnHost: *mut ::std::os::raw::c_void,
        workspaceInBytesOnHost: usize,
        info: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSsytri_bufferSize(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f32, lda: ::std::os::raw::c_int, ipiv: *const ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDsytri_bufferSize(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f64, lda: ::std::os::raw::c_int, ipiv: *const ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnCsytri_bufferSize(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut cuComplex, lda: ::std::os::raw::c_int, ipiv: *const ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZsytri_bufferSize(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut cuDoubleComplex, lda: ::std::os::raw::c_int, ipiv: *const ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSsytri(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f32, lda: ::std::os::raw::c_int, ipiv: *const ::std::os::raw::c_int, work: *mut f32, lwork: ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDsytri(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f64, lda: ::std::os::raw::c_int, ipiv: *const ::std::os::raw::c_int, work: *mut f64, lwork: ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnCsytri(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut cuComplex, lda: ::std::os::raw::c_int, ipiv: *const ::std::os::raw::c_int, work: *mut cuComplex, lwork: ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZsytri(
        handle: cusolverDnHandle_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: *mut cuDoubleComplex,
        lda: ::std::os::raw::c_int,
        ipiv: *const ::std::os::raw::c_int,
        work: *mut cuDoubleComplex,
        lwork: ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSgebrd_bufferSize(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, Lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDgebrd_bufferSize(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, Lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnCgebrd_bufferSize(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, Lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZgebrd_bufferSize(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, Lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSgebrd(
        handle: cusolverDnHandle_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        A: *mut f32,
        lda: ::std::os::raw::c_int,
        D: *mut f32,
        E: *mut f32,
        TAUQ: *mut f32,
        TAUP: *mut f32,
        Work: *mut f32,
        Lwork: ::std::os::raw::c_int,
        devInfo: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDgebrd(
        handle: cusolverDnHandle_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        A: *mut f64,
        lda: ::std::os::raw::c_int,
        D: *mut f64,
        E: *mut f64,
        TAUQ: *mut f64,
        TAUP: *mut f64,
        Work: *mut f64,
        Lwork: ::std::os::raw::c_int,
        devInfo: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnCgebrd(
        handle: cusolverDnHandle_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        A: *mut cuComplex,
        lda: ::std::os::raw::c_int,
        D: *mut f32,
        E: *mut f32,
        TAUQ: *mut cuComplex,
        TAUP: *mut cuComplex,
        Work: *mut cuComplex,
        Lwork: ::std::os::raw::c_int,
        devInfo: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZgebrd(
        handle: cusolverDnHandle_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        A: *mut cuDoubleComplex,
        lda: ::std::os::raw::c_int,
        D: *mut f64,
        E: *mut f64,
        TAUQ: *mut cuDoubleComplex,
        TAUP: *mut cuDoubleComplex,
        Work: *mut cuDoubleComplex,
        Lwork: ::std::os::raw::c_int,
        devInfo: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSorgbr_bufferSize(handle: cusolverDnHandle_t, side: cublasSideMode_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, k: ::std::os::raw::c_int, A: *const f32, lda: ::std::os::raw::c_int, tau: *const f32, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDorgbr_bufferSize(handle: cusolverDnHandle_t, side: cublasSideMode_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, k: ::std::os::raw::c_int, A: *const f64, lda: ::std::os::raw::c_int, tau: *const f64, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnCungbr_bufferSize(handle: cusolverDnHandle_t, side: cublasSideMode_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, k: ::std::os::raw::c_int, A: *const cuComplex, lda: ::std::os::raw::c_int, tau: *const cuComplex, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZungbr_bufferSize(
        handle: cusolverDnHandle_t,
        side: cublasSideMode_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        A: *const cuDoubleComplex,
        lda: ::std::os::raw::c_int,
        tau: *const cuDoubleComplex,
        lwork: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSorgbr(
        handle: cusolverDnHandle_t,
        side: cublasSideMode_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        A: *mut f32,
        lda: ::std::os::raw::c_int,
        tau: *const f32,
        work: *mut f32,
        lwork: ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDorgbr(
        handle: cusolverDnHandle_t,
        side: cublasSideMode_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        A: *mut f64,
        lda: ::std::os::raw::c_int,
        tau: *const f64,
        work: *mut f64,
        lwork: ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnCungbr(
        handle: cusolverDnHandle_t,
        side: cublasSideMode_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        A: *mut cuComplex,
        lda: ::std::os::raw::c_int,
        tau: *const cuComplex,
        work: *mut cuComplex,
        lwork: ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZungbr(
        handle: cusolverDnHandle_t,
        side: cublasSideMode_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        A: *mut cuDoubleComplex,
        lda: ::std::os::raw::c_int,
        tau: *const cuDoubleComplex,
        work: *mut cuDoubleComplex,
        lwork: ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSsytrd_bufferSize(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *const f32, lda: ::std::os::raw::c_int, d: *const f32, e: *const f32, tau: *const f32, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDsytrd_bufferSize(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *const f64, lda: ::std::os::raw::c_int, d: *const f64, e: *const f64, tau: *const f64, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnChetrd_bufferSize(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *const cuComplex, lda: ::std::os::raw::c_int, d: *const f32, e: *const f32, tau: *const cuComplex, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZhetrd_bufferSize(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *const cuDoubleComplex, lda: ::std::os::raw::c_int, d: *const f64, e: *const f64, tau: *const cuDoubleComplex, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSsytrd(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f32, lda: ::std::os::raw::c_int, d: *mut f32, e: *mut f32, tau: *mut f32, work: *mut f32, lwork: ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDsytrd(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f64, lda: ::std::os::raw::c_int, d: *mut f64, e: *mut f64, tau: *mut f64, work: *mut f64, lwork: ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnChetrd(
        handle: cusolverDnHandle_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: *mut cuComplex,
        lda: ::std::os::raw::c_int,
        d: *mut f32,
        e: *mut f32,
        tau: *mut cuComplex,
        work: *mut cuComplex,
        lwork: ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZhetrd(
        handle: cusolverDnHandle_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: *mut cuDoubleComplex,
        lda: ::std::os::raw::c_int,
        d: *mut f64,
        e: *mut f64,
        tau: *mut cuDoubleComplex,
        work: *mut cuDoubleComplex,
        lwork: ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSorgtr_bufferSize(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *const f32, lda: ::std::os::raw::c_int, tau: *const f32, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDorgtr_bufferSize(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *const f64, lda: ::std::os::raw::c_int, tau: *const f64, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnCungtr_bufferSize(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *const cuComplex, lda: ::std::os::raw::c_int, tau: *const cuComplex, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZungtr_bufferSize(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *const cuDoubleComplex, lda: ::std::os::raw::c_int, tau: *const cuDoubleComplex, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSorgtr(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f32, lda: ::std::os::raw::c_int, tau: *const f32, work: *mut f32, lwork: ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDorgtr(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f64, lda: ::std::os::raw::c_int, tau: *const f64, work: *mut f64, lwork: ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnCungtr(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut cuComplex, lda: ::std::os::raw::c_int, tau: *const cuComplex, work: *mut cuComplex, lwork: ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZungtr(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut cuDoubleComplex, lda: ::std::os::raw::c_int, tau: *const cuDoubleComplex, work: *mut cuDoubleComplex, lwork: ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int)
    -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSormtr_bufferSize(
        handle: cusolverDnHandle_t,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        A: *const f32,
        lda: ::std::os::raw::c_int,
        tau: *const f32,
        C: *const f32,
        ldc: ::std::os::raw::c_int,
        lwork: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDormtr_bufferSize(
        handle: cusolverDnHandle_t,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        A: *const f64,
        lda: ::std::os::raw::c_int,
        tau: *const f64,
        C: *const f64,
        ldc: ::std::os::raw::c_int,
        lwork: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnCunmtr_bufferSize(
        handle: cusolverDnHandle_t,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        A: *const cuComplex,
        lda: ::std::os::raw::c_int,
        tau: *const cuComplex,
        C: *const cuComplex,
        ldc: ::std::os::raw::c_int,
        lwork: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZunmtr_bufferSize(
        handle: cusolverDnHandle_t,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        A: *const cuDoubleComplex,
        lda: ::std::os::raw::c_int,
        tau: *const cuDoubleComplex,
        C: *const cuDoubleComplex,
        ldc: ::std::os::raw::c_int,
        lwork: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSormtr(
        handle: cusolverDnHandle_t,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        A: *mut f32,
        lda: ::std::os::raw::c_int,
        tau: *mut f32,
        C: *mut f32,
        ldc: ::std::os::raw::c_int,
        work: *mut f32,
        lwork: ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDormtr(
        handle: cusolverDnHandle_t,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        A: *mut f64,
        lda: ::std::os::raw::c_int,
        tau: *mut f64,
        C: *mut f64,
        ldc: ::std::os::raw::c_int,
        work: *mut f64,
        lwork: ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnCunmtr(
        handle: cusolverDnHandle_t,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        A: *mut cuComplex,
        lda: ::std::os::raw::c_int,
        tau: *mut cuComplex,
        C: *mut cuComplex,
        ldc: ::std::os::raw::c_int,
        work: *mut cuComplex,
        lwork: ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZunmtr(
        handle: cusolverDnHandle_t,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        A: *mut cuDoubleComplex,
        lda: ::std::os::raw::c_int,
        tau: *mut cuDoubleComplex,
        C: *mut cuDoubleComplex,
        ldc: ::std::os::raw::c_int,
        work: *mut cuDoubleComplex,
        lwork: ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSgesvd_bufferSize(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDgesvd_bufferSize(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnCgesvd_bufferSize(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZgesvd_bufferSize(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSgesvd(
        handle: cusolverDnHandle_t,
        jobu: ::std::os::raw::c_schar,
        jobvt: ::std::os::raw::c_schar,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        A: *mut f32,
        lda: ::std::os::raw::c_int,
        S: *mut f32,
        U: *mut f32,
        ldu: ::std::os::raw::c_int,
        VT: *mut f32,
        ldvt: ::std::os::raw::c_int,
        work: *mut f32,
        lwork: ::std::os::raw::c_int,
        rwork: *mut f32,
        info: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDgesvd(
        handle: cusolverDnHandle_t,
        jobu: ::std::os::raw::c_schar,
        jobvt: ::std::os::raw::c_schar,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        A: *mut f64,
        lda: ::std::os::raw::c_int,
        S: *mut f64,
        U: *mut f64,
        ldu: ::std::os::raw::c_int,
        VT: *mut f64,
        ldvt: ::std::os::raw::c_int,
        work: *mut f64,
        lwork: ::std::os::raw::c_int,
        rwork: *mut f64,
        info: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnCgesvd(
        handle: cusolverDnHandle_t,
        jobu: ::std::os::raw::c_schar,
        jobvt: ::std::os::raw::c_schar,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        A: *mut cuComplex,
        lda: ::std::os::raw::c_int,
        S: *mut f32,
        U: *mut cuComplex,
        ldu: ::std::os::raw::c_int,
        VT: *mut cuComplex,
        ldvt: ::std::os::raw::c_int,
        work: *mut cuComplex,
        lwork: ::std::os::raw::c_int,
        rwork: *mut f32,
        info: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZgesvd(
        handle: cusolverDnHandle_t,
        jobu: ::std::os::raw::c_schar,
        jobvt: ::std::os::raw::c_schar,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        A: *mut cuDoubleComplex,
        lda: ::std::os::raw::c_int,
        S: *mut f64,
        U: *mut cuDoubleComplex,
        ldu: ::std::os::raw::c_int,
        VT: *mut cuDoubleComplex,
        ldvt: ::std::os::raw::c_int,
        work: *mut cuDoubleComplex,
        lwork: ::std::os::raw::c_int,
        rwork: *mut f64,
        info: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSsyevd_bufferSize(handle: cusolverDnHandle_t, jobz: cusolverEigMode_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *const f32, lda: ::std::os::raw::c_int, W: *const f32, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDsyevd_bufferSize(handle: cusolverDnHandle_t, jobz: cusolverEigMode_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *const f64, lda: ::std::os::raw::c_int, W: *const f64, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnCheevd_bufferSize(handle: cusolverDnHandle_t, jobz: cusolverEigMode_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *const cuComplex, lda: ::std::os::raw::c_int, W: *const f32, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZheevd_bufferSize(handle: cusolverDnHandle_t, jobz: cusolverEigMode_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *const cuDoubleComplex, lda: ::std::os::raw::c_int, W: *const f64, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSsyevd(handle: cusolverDnHandle_t, jobz: cusolverEigMode_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f32, lda: ::std::os::raw::c_int, W: *mut f32, work: *mut f32, lwork: ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDsyevd(handle: cusolverDnHandle_t, jobz: cusolverEigMode_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f64, lda: ::std::os::raw::c_int, W: *mut f64, work: *mut f64, lwork: ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnCheevd(handle: cusolverDnHandle_t, jobz: cusolverEigMode_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut cuComplex, lda: ::std::os::raw::c_int, W: *mut f32, work: *mut cuComplex, lwork: ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZheevd(
        handle: cusolverDnHandle_t,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: *mut cuDoubleComplex,
        lda: ::std::os::raw::c_int,
        W: *mut f64,
        work: *mut cuDoubleComplex,
        lwork: ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSsyevdx_bufferSize(
        handle: cusolverDnHandle_t,
        jobz: cusolverEigMode_t,
        range: cusolverEigRange_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: *const f32,
        lda: ::std::os::raw::c_int,
        vl: f32,
        vu: f32,
        il: ::std::os::raw::c_int,
        iu: ::std::os::raw::c_int,
        meig: *mut ::std::os::raw::c_int,
        W: *const f32,
        lwork: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDsyevdx_bufferSize(
        handle: cusolverDnHandle_t,
        jobz: cusolverEigMode_t,
        range: cusolverEigRange_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: *const f64,
        lda: ::std::os::raw::c_int,
        vl: f64,
        vu: f64,
        il: ::std::os::raw::c_int,
        iu: ::std::os::raw::c_int,
        meig: *mut ::std::os::raw::c_int,
        W: *const f64,
        lwork: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnCheevdx_bufferSize(
        handle: cusolverDnHandle_t,
        jobz: cusolverEigMode_t,
        range: cusolverEigRange_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: *const cuComplex,
        lda: ::std::os::raw::c_int,
        vl: f32,
        vu: f32,
        il: ::std::os::raw::c_int,
        iu: ::std::os::raw::c_int,
        meig: *mut ::std::os::raw::c_int,
        W: *const f32,
        lwork: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZheevdx_bufferSize(
        handle: cusolverDnHandle_t,
        jobz: cusolverEigMode_t,
        range: cusolverEigRange_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: *const cuDoubleComplex,
        lda: ::std::os::raw::c_int,
        vl: f64,
        vu: f64,
        il: ::std::os::raw::c_int,
        iu: ::std::os::raw::c_int,
        meig: *mut ::std::os::raw::c_int,
        W: *const f64,
        lwork: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSsyevdx(
        handle: cusolverDnHandle_t,
        jobz: cusolverEigMode_t,
        range: cusolverEigRange_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: *mut f32,
        lda: ::std::os::raw::c_int,
        vl: f32,
        vu: f32,
        il: ::std::os::raw::c_int,
        iu: ::std::os::raw::c_int,
        meig: *mut ::std::os::raw::c_int,
        W: *mut f32,
        work: *mut f32,
        lwork: ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDsyevdx(
        handle: cusolverDnHandle_t,
        jobz: cusolverEigMode_t,
        range: cusolverEigRange_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: *mut f64,
        lda: ::std::os::raw::c_int,
        vl: f64,
        vu: f64,
        il: ::std::os::raw::c_int,
        iu: ::std::os::raw::c_int,
        meig: *mut ::std::os::raw::c_int,
        W: *mut f64,
        work: *mut f64,
        lwork: ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnCheevdx(
        handle: cusolverDnHandle_t,
        jobz: cusolverEigMode_t,
        range: cusolverEigRange_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: *mut cuComplex,
        lda: ::std::os::raw::c_int,
        vl: f32,
        vu: f32,
        il: ::std::os::raw::c_int,
        iu: ::std::os::raw::c_int,
        meig: *mut ::std::os::raw::c_int,
        W: *mut f32,
        work: *mut cuComplex,
        lwork: ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZheevdx(
        handle: cusolverDnHandle_t,
        jobz: cusolverEigMode_t,
        range: cusolverEigRange_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: *mut cuDoubleComplex,
        lda: ::std::os::raw::c_int,
        vl: f64,
        vu: f64,
        il: ::std::os::raw::c_int,
        iu: ::std::os::raw::c_int,
        meig: *mut ::std::os::raw::c_int,
        W: *mut f64,
        work: *mut cuDoubleComplex,
        lwork: ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSsygvdx_bufferSize(
        handle: cusolverDnHandle_t,
        itype: cusolverEigType_t,
        jobz: cusolverEigMode_t,
        range: cusolverEigRange_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: *const f32,
        lda: ::std::os::raw::c_int,
        B: *const f32,
        ldb: ::std::os::raw::c_int,
        vl: f32,
        vu: f32,
        il: ::std::os::raw::c_int,
        iu: ::std::os::raw::c_int,
        meig: *mut ::std::os::raw::c_int,
        W: *const f32,
        lwork: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDsygvdx_bufferSize(
        handle: cusolverDnHandle_t,
        itype: cusolverEigType_t,
        jobz: cusolverEigMode_t,
        range: cusolverEigRange_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: *const f64,
        lda: ::std::os::raw::c_int,
        B: *const f64,
        ldb: ::std::os::raw::c_int,
        vl: f64,
        vu: f64,
        il: ::std::os::raw::c_int,
        iu: ::std::os::raw::c_int,
        meig: *mut ::std::os::raw::c_int,
        W: *const f64,
        lwork: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnChegvdx_bufferSize(
        handle: cusolverDnHandle_t,
        itype: cusolverEigType_t,
        jobz: cusolverEigMode_t,
        range: cusolverEigRange_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: *const cuComplex,
        lda: ::std::os::raw::c_int,
        B: *const cuComplex,
        ldb: ::std::os::raw::c_int,
        vl: f32,
        vu: f32,
        il: ::std::os::raw::c_int,
        iu: ::std::os::raw::c_int,
        meig: *mut ::std::os::raw::c_int,
        W: *const f32,
        lwork: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZhegvdx_bufferSize(
        handle: cusolverDnHandle_t,
        itype: cusolverEigType_t,
        jobz: cusolverEigMode_t,
        range: cusolverEigRange_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: *const cuDoubleComplex,
        lda: ::std::os::raw::c_int,
        B: *const cuDoubleComplex,
        ldb: ::std::os::raw::c_int,
        vl: f64,
        vu: f64,
        il: ::std::os::raw::c_int,
        iu: ::std::os::raw::c_int,
        meig: *mut ::std::os::raw::c_int,
        W: *const f64,
        lwork: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSsygvdx(
        handle: cusolverDnHandle_t,
        itype: cusolverEigType_t,
        jobz: cusolverEigMode_t,
        range: cusolverEigRange_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: *mut f32,
        lda: ::std::os::raw::c_int,
        B: *mut f32,
        ldb: ::std::os::raw::c_int,
        vl: f32,
        vu: f32,
        il: ::std::os::raw::c_int,
        iu: ::std::os::raw::c_int,
        meig: *mut ::std::os::raw::c_int,
        W: *mut f32,
        work: *mut f32,
        lwork: ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDsygvdx(
        handle: cusolverDnHandle_t,
        itype: cusolverEigType_t,
        jobz: cusolverEigMode_t,
        range: cusolverEigRange_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: *mut f64,
        lda: ::std::os::raw::c_int,
        B: *mut f64,
        ldb: ::std::os::raw::c_int,
        vl: f64,
        vu: f64,
        il: ::std::os::raw::c_int,
        iu: ::std::os::raw::c_int,
        meig: *mut ::std::os::raw::c_int,
        W: *mut f64,
        work: *mut f64,
        lwork: ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnChegvdx(
        handle: cusolverDnHandle_t,
        itype: cusolverEigType_t,
        jobz: cusolverEigMode_t,
        range: cusolverEigRange_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: *mut cuComplex,
        lda: ::std::os::raw::c_int,
        B: *mut cuComplex,
        ldb: ::std::os::raw::c_int,
        vl: f32,
        vu: f32,
        il: ::std::os::raw::c_int,
        iu: ::std::os::raw::c_int,
        meig: *mut ::std::os::raw::c_int,
        W: *mut f32,
        work: *mut cuComplex,
        lwork: ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZhegvdx(
        handle: cusolverDnHandle_t,
        itype: cusolverEigType_t,
        jobz: cusolverEigMode_t,
        range: cusolverEigRange_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: *mut cuDoubleComplex,
        lda: ::std::os::raw::c_int,
        B: *mut cuDoubleComplex,
        ldb: ::std::os::raw::c_int,
        vl: f64,
        vu: f64,
        il: ::std::os::raw::c_int,
        iu: ::std::os::raw::c_int,
        meig: *mut ::std::os::raw::c_int,
        W: *mut f64,
        work: *mut cuDoubleComplex,
        lwork: ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSsygvd_bufferSize(
        handle: cusolverDnHandle_t,
        itype: cusolverEigType_t,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: *const f32,
        lda: ::std::os::raw::c_int,
        B: *const f32,
        ldb: ::std::os::raw::c_int,
        W: *const f32,
        lwork: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDsygvd_bufferSize(
        handle: cusolverDnHandle_t,
        itype: cusolverEigType_t,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: *const f64,
        lda: ::std::os::raw::c_int,
        B: *const f64,
        ldb: ::std::os::raw::c_int,
        W: *const f64,
        lwork: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnChegvd_bufferSize(
        handle: cusolverDnHandle_t,
        itype: cusolverEigType_t,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: *const cuComplex,
        lda: ::std::os::raw::c_int,
        B: *const cuComplex,
        ldb: ::std::os::raw::c_int,
        W: *const f32,
        lwork: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZhegvd_bufferSize(
        handle: cusolverDnHandle_t,
        itype: cusolverEigType_t,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: *const cuDoubleComplex,
        lda: ::std::os::raw::c_int,
        B: *const cuDoubleComplex,
        ldb: ::std::os::raw::c_int,
        W: *const f64,
        lwork: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSsygvd(
        handle: cusolverDnHandle_t,
        itype: cusolverEigType_t,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: *mut f32,
        lda: ::std::os::raw::c_int,
        B: *mut f32,
        ldb: ::std::os::raw::c_int,
        W: *mut f32,
        work: *mut f32,
        lwork: ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDsygvd(
        handle: cusolverDnHandle_t,
        itype: cusolverEigType_t,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: *mut f64,
        lda: ::std::os::raw::c_int,
        B: *mut f64,
        ldb: ::std::os::raw::c_int,
        W: *mut f64,
        work: *mut f64,
        lwork: ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnChegvd(
        handle: cusolverDnHandle_t,
        itype: cusolverEigType_t,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: *mut cuComplex,
        lda: ::std::os::raw::c_int,
        B: *mut cuComplex,
        ldb: ::std::os::raw::c_int,
        W: *mut f32,
        work: *mut cuComplex,
        lwork: ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZhegvd(
        handle: cusolverDnHandle_t,
        itype: cusolverEigType_t,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: *mut cuDoubleComplex,
        lda: ::std::os::raw::c_int,
        B: *mut cuDoubleComplex,
        ldb: ::std::os::raw::c_int,
        W: *mut f64,
        work: *mut cuDoubleComplex,
        lwork: ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnXsygvd_bufferSize(
        handle: cusolverDnHandle_t,
        params: cusolverDnParams_t,
        itype: cusolverEigType_t,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: i64,
        dataTypeA: cudaDataType,
        d_A: *const ::std::os::raw::c_void,
        lda: i64,
        dataTypeB: cudaDataType,
        d_B: *const ::std::os::raw::c_void,
        ldb: i64,
        dataTypeW: cudaDataType,
        d_W: *const ::std::os::raw::c_void,
        computeType: cudaDataType,
        workspaceInBytesOnDevice: *mut usize,
        workspaceInBytesOnHost: *mut usize,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnXsygvd(
        handle: cusolverDnHandle_t,
        params: cusolverDnParams_t,
        itype: cusolverEigType_t,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: i64,
        dataTypeA: cudaDataType,
        d_A: *mut ::std::os::raw::c_void,
        lda: i64,
        dataTypeB: cudaDataType,
        d_B: *mut ::std::os::raw::c_void,
        ldb: i64,
        dataTypeW: cudaDataType,
        d_W: *mut ::std::os::raw::c_void,
        computeType: cudaDataType,
        bufferOnDevice: *mut ::std::os::raw::c_void,
        workspaceInBytesOnDevice: usize,
        bufferOnHost: *mut ::std::os::raw::c_void,
        workspaceInBytesOnHost: usize,
        d_info: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnXsygvdx_bufferSize(
        handle: cusolverDnHandle_t,
        params: cusolverDnParams_t,
        itype: cusolverEigType_t,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: i64,
        dataTypeA: cudaDataType,
        d_A: *const ::std::os::raw::c_void,
        lda: i64,
        dataTypeB: cudaDataType,
        d_B: *const ::std::os::raw::c_void,
        ldb: i64,
        vl: *mut ::std::os::raw::c_void,
        vu: *mut ::std::os::raw::c_void,
        il: i64,
        iu: i64,
        meig: *mut i64,
        dataTypeW: cudaDataType,
        d_W: *const ::std::os::raw::c_void,
        computeType: cudaDataType,
        workspaceInBytesOnDevice: *mut usize,
        workspaceInBytesOnHost: *mut usize,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnXsygvdx(
        handle: cusolverDnHandle_t,
        params: cusolverDnParams_t,
        itype: cusolverEigType_t,
        jobz: cusolverEigMode_t,
        range: cusolverEigRange_t,
        uplo: cublasFillMode_t,
        n: i64,
        dataTypeA: cudaDataType,
        d_A: *mut ::std::os::raw::c_void,
        lda: i64,
        dataTypeB: cudaDataType,
        d_B: *mut ::std::os::raw::c_void,
        ldb: i64,
        vl: *mut ::std::os::raw::c_void,
        vu: *mut ::std::os::raw::c_void,
        il: i64,
        iu: i64,
        meig: *mut i64,
        dataTypeW: cudaDataType,
        d_W: *mut ::std::os::raw::c_void,
        computeType: cudaDataType,
        bufferOnDevice: *mut ::std::os::raw::c_void,
        workspaceInBytesOnDevice: usize,
        bufferOnHost: *mut ::std::os::raw::c_void,
        workspaceInBytesOnHost: usize,
        d_info: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnCreateSyevjInfo(info: *mut syevjInfo_t) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDestroySyevjInfo(info: syevjInfo_t) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnXsyevjSetTolerance(info: syevjInfo_t, tolerance: f64) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnXsyevjSetMaxSweeps(info: syevjInfo_t, max_sweeps: ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnXsyevjSetSortEig(info: syevjInfo_t, sort_eig: ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnXsyevjGetResidual(handle: cusolverDnHandle_t, info: syevjInfo_t, residual: *mut f64) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnXsyevjGetSweeps(handle: cusolverDnHandle_t, info: syevjInfo_t, executed_sweeps: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSsyevjBatched_bufferSize(
        handle: cusolverDnHandle_t,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: *const f32,
        lda: ::std::os::raw::c_int,
        W: *const f32,
        lwork: *mut ::std::os::raw::c_int,
        params: syevjInfo_t,
        batchSize: ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDsyevjBatched_bufferSize(
        handle: cusolverDnHandle_t,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: *const f64,
        lda: ::std::os::raw::c_int,
        W: *const f64,
        lwork: *mut ::std::os::raw::c_int,
        params: syevjInfo_t,
        batchSize: ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnCheevjBatched_bufferSize(
        handle: cusolverDnHandle_t,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: *const cuComplex,
        lda: ::std::os::raw::c_int,
        W: *const f32,
        lwork: *mut ::std::os::raw::c_int,
        params: syevjInfo_t,
        batchSize: ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZheevjBatched_bufferSize(
        handle: cusolverDnHandle_t,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: *const cuDoubleComplex,
        lda: ::std::os::raw::c_int,
        W: *const f64,
        lwork: *mut ::std::os::raw::c_int,
        params: syevjInfo_t,
        batchSize: ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSsyevjBatched(
        handle: cusolverDnHandle_t,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: *mut f32,
        lda: ::std::os::raw::c_int,
        W: *mut f32,
        work: *mut f32,
        lwork: ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
        params: syevjInfo_t,
        batchSize: ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDsyevjBatched(
        handle: cusolverDnHandle_t,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: *mut f64,
        lda: ::std::os::raw::c_int,
        W: *mut f64,
        work: *mut f64,
        lwork: ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
        params: syevjInfo_t,
        batchSize: ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnCheevjBatched(
        handle: cusolverDnHandle_t,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: *mut cuComplex,
        lda: ::std::os::raw::c_int,
        W: *mut f32,
        work: *mut cuComplex,
        lwork: ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
        params: syevjInfo_t,
        batchSize: ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZheevjBatched(
        handle: cusolverDnHandle_t,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: *mut cuDoubleComplex,
        lda: ::std::os::raw::c_int,
        W: *mut f64,
        work: *mut cuDoubleComplex,
        lwork: ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
        params: syevjInfo_t,
        batchSize: ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSsyevj_bufferSize(handle: cusolverDnHandle_t, jobz: cusolverEigMode_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *const f32, lda: ::std::os::raw::c_int, W: *const f32, lwork: *mut ::std::os::raw::c_int, params: syevjInfo_t) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDsyevj_bufferSize(handle: cusolverDnHandle_t, jobz: cusolverEigMode_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *const f64, lda: ::std::os::raw::c_int, W: *const f64, lwork: *mut ::std::os::raw::c_int, params: syevjInfo_t) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnCheevj_bufferSize(handle: cusolverDnHandle_t, jobz: cusolverEigMode_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *const cuComplex, lda: ::std::os::raw::c_int, W: *const f32, lwork: *mut ::std::os::raw::c_int, params: syevjInfo_t) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZheevj_bufferSize(handle: cusolverDnHandle_t, jobz: cusolverEigMode_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *const cuDoubleComplex, lda: ::std::os::raw::c_int, W: *const f64, lwork: *mut ::std::os::raw::c_int, params: syevjInfo_t) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSsyevj(
        handle: cusolverDnHandle_t,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: *mut f32,
        lda: ::std::os::raw::c_int,
        W: *mut f32,
        work: *mut f32,
        lwork: ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
        params: syevjInfo_t,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDsyevj(
        handle: cusolverDnHandle_t,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: *mut f64,
        lda: ::std::os::raw::c_int,
        W: *mut f64,
        work: *mut f64,
        lwork: ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
        params: syevjInfo_t,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnCheevj(
        handle: cusolverDnHandle_t,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: *mut cuComplex,
        lda: ::std::os::raw::c_int,
        W: *mut f32,
        work: *mut cuComplex,
        lwork: ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
        params: syevjInfo_t,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZheevj(
        handle: cusolverDnHandle_t,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: *mut cuDoubleComplex,
        lda: ::std::os::raw::c_int,
        W: *mut f64,
        work: *mut cuDoubleComplex,
        lwork: ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
        params: syevjInfo_t,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSsygvj_bufferSize(
        handle: cusolverDnHandle_t,
        itype: cusolverEigType_t,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: *const f32,
        lda: ::std::os::raw::c_int,
        B: *const f32,
        ldb: ::std::os::raw::c_int,
        W: *const f32,
        lwork: *mut ::std::os::raw::c_int,
        params: syevjInfo_t,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDsygvj_bufferSize(
        handle: cusolverDnHandle_t,
        itype: cusolverEigType_t,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: *const f64,
        lda: ::std::os::raw::c_int,
        B: *const f64,
        ldb: ::std::os::raw::c_int,
        W: *const f64,
        lwork: *mut ::std::os::raw::c_int,
        params: syevjInfo_t,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnChegvj_bufferSize(
        handle: cusolverDnHandle_t,
        itype: cusolverEigType_t,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: *const cuComplex,
        lda: ::std::os::raw::c_int,
        B: *const cuComplex,
        ldb: ::std::os::raw::c_int,
        W: *const f32,
        lwork: *mut ::std::os::raw::c_int,
        params: syevjInfo_t,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZhegvj_bufferSize(
        handle: cusolverDnHandle_t,
        itype: cusolverEigType_t,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: *const cuDoubleComplex,
        lda: ::std::os::raw::c_int,
        B: *const cuDoubleComplex,
        ldb: ::std::os::raw::c_int,
        W: *const f64,
        lwork: *mut ::std::os::raw::c_int,
        params: syevjInfo_t,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSsygvj(
        handle: cusolverDnHandle_t,
        itype: cusolverEigType_t,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: *mut f32,
        lda: ::std::os::raw::c_int,
        B: *mut f32,
        ldb: ::std::os::raw::c_int,
        W: *mut f32,
        work: *mut f32,
        lwork: ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
        params: syevjInfo_t,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDsygvj(
        handle: cusolverDnHandle_t,
        itype: cusolverEigType_t,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: *mut f64,
        lda: ::std::os::raw::c_int,
        B: *mut f64,
        ldb: ::std::os::raw::c_int,
        W: *mut f64,
        work: *mut f64,
        lwork: ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
        params: syevjInfo_t,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnChegvj(
        handle: cusolverDnHandle_t,
        itype: cusolverEigType_t,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: *mut cuComplex,
        lda: ::std::os::raw::c_int,
        B: *mut cuComplex,
        ldb: ::std::os::raw::c_int,
        W: *mut f32,
        work: *mut cuComplex,
        lwork: ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
        params: syevjInfo_t,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZhegvj(
        handle: cusolverDnHandle_t,
        itype: cusolverEigType_t,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: ::std::os::raw::c_int,
        A: *mut cuDoubleComplex,
        lda: ::std::os::raw::c_int,
        B: *mut cuDoubleComplex,
        ldb: ::std::os::raw::c_int,
        W: *mut f64,
        work: *mut cuDoubleComplex,
        lwork: ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
        params: syevjInfo_t,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnCreateGesvdjInfo(info: *mut gesvdjInfo_t) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDestroyGesvdjInfo(info: gesvdjInfo_t) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnXgesvdjSetTolerance(info: gesvdjInfo_t, tolerance: f64) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnXgesvdjSetMaxSweeps(info: gesvdjInfo_t, max_sweeps: ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnXgesvdjSetSortEig(info: gesvdjInfo_t, sort_svd: ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnXgesvdjGetResidual(handle: cusolverDnHandle_t, info: gesvdjInfo_t, residual: *mut f64) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnXgesvdjGetSweeps(handle: cusolverDnHandle_t, info: gesvdjInfo_t, executed_sweeps: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSgesvdjBatched_bufferSize(
        handle: cusolverDnHandle_t,
        jobz: cusolverEigMode_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        A: *const f32,
        lda: ::std::os::raw::c_int,
        S: *const f32,
        U: *const f32,
        ldu: ::std::os::raw::c_int,
        V: *const f32,
        ldv: ::std::os::raw::c_int,
        lwork: *mut ::std::os::raw::c_int,
        params: gesvdjInfo_t,
        batchSize: ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDgesvdjBatched_bufferSize(
        handle: cusolverDnHandle_t,
        jobz: cusolverEigMode_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        A: *const f64,
        lda: ::std::os::raw::c_int,
        S: *const f64,
        U: *const f64,
        ldu: ::std::os::raw::c_int,
        V: *const f64,
        ldv: ::std::os::raw::c_int,
        lwork: *mut ::std::os::raw::c_int,
        params: gesvdjInfo_t,
        batchSize: ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnCgesvdjBatched_bufferSize(
        handle: cusolverDnHandle_t,
        jobz: cusolverEigMode_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        A: *const cuComplex,
        lda: ::std::os::raw::c_int,
        S: *const f32,
        U: *const cuComplex,
        ldu: ::std::os::raw::c_int,
        V: *const cuComplex,
        ldv: ::std::os::raw::c_int,
        lwork: *mut ::std::os::raw::c_int,
        params: gesvdjInfo_t,
        batchSize: ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZgesvdjBatched_bufferSize(
        handle: cusolverDnHandle_t,
        jobz: cusolverEigMode_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        A: *const cuDoubleComplex,
        lda: ::std::os::raw::c_int,
        S: *const f64,
        U: *const cuDoubleComplex,
        ldu: ::std::os::raw::c_int,
        V: *const cuDoubleComplex,
        ldv: ::std::os::raw::c_int,
        lwork: *mut ::std::os::raw::c_int,
        params: gesvdjInfo_t,
        batchSize: ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSgesvdjBatched(
        handle: cusolverDnHandle_t,
        jobz: cusolverEigMode_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        A: *mut f32,
        lda: ::std::os::raw::c_int,
        S: *mut f32,
        U: *mut f32,
        ldu: ::std::os::raw::c_int,
        V: *mut f32,
        ldv: ::std::os::raw::c_int,
        work: *mut f32,
        lwork: ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
        params: gesvdjInfo_t,
        batchSize: ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDgesvdjBatched(
        handle: cusolverDnHandle_t,
        jobz: cusolverEigMode_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        A: *mut f64,
        lda: ::std::os::raw::c_int,
        S: *mut f64,
        U: *mut f64,
        ldu: ::std::os::raw::c_int,
        V: *mut f64,
        ldv: ::std::os::raw::c_int,
        work: *mut f64,
        lwork: ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
        params: gesvdjInfo_t,
        batchSize: ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnCgesvdjBatched(
        handle: cusolverDnHandle_t,
        jobz: cusolverEigMode_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        A: *mut cuComplex,
        lda: ::std::os::raw::c_int,
        S: *mut f32,
        U: *mut cuComplex,
        ldu: ::std::os::raw::c_int,
        V: *mut cuComplex,
        ldv: ::std::os::raw::c_int,
        work: *mut cuComplex,
        lwork: ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
        params: gesvdjInfo_t,
        batchSize: ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZgesvdjBatched(
        handle: cusolverDnHandle_t,
        jobz: cusolverEigMode_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        A: *mut cuDoubleComplex,
        lda: ::std::os::raw::c_int,
        S: *mut f64,
        U: *mut cuDoubleComplex,
        ldu: ::std::os::raw::c_int,
        V: *mut cuDoubleComplex,
        ldv: ::std::os::raw::c_int,
        work: *mut cuDoubleComplex,
        lwork: ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
        params: gesvdjInfo_t,
        batchSize: ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSgesvdj_bufferSize(
        handle: cusolverDnHandle_t,
        jobz: cusolverEigMode_t,
        econ: ::std::os::raw::c_int,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        A: *const f32,
        lda: ::std::os::raw::c_int,
        S: *const f32,
        U: *const f32,
        ldu: ::std::os::raw::c_int,
        V: *const f32,
        ldv: ::std::os::raw::c_int,
        lwork: *mut ::std::os::raw::c_int,
        params: gesvdjInfo_t,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDgesvdj_bufferSize(
        handle: cusolverDnHandle_t,
        jobz: cusolverEigMode_t,
        econ: ::std::os::raw::c_int,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        A: *const f64,
        lda: ::std::os::raw::c_int,
        S: *const f64,
        U: *const f64,
        ldu: ::std::os::raw::c_int,
        V: *const f64,
        ldv: ::std::os::raw::c_int,
        lwork: *mut ::std::os::raw::c_int,
        params: gesvdjInfo_t,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnCgesvdj_bufferSize(
        handle: cusolverDnHandle_t,
        jobz: cusolverEigMode_t,
        econ: ::std::os::raw::c_int,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        A: *const cuComplex,
        lda: ::std::os::raw::c_int,
        S: *const f32,
        U: *const cuComplex,
        ldu: ::std::os::raw::c_int,
        V: *const cuComplex,
        ldv: ::std::os::raw::c_int,
        lwork: *mut ::std::os::raw::c_int,
        params: gesvdjInfo_t,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZgesvdj_bufferSize(
        handle: cusolverDnHandle_t,
        jobz: cusolverEigMode_t,
        econ: ::std::os::raw::c_int,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        A: *const cuDoubleComplex,
        lda: ::std::os::raw::c_int,
        S: *const f64,
        U: *const cuDoubleComplex,
        ldu: ::std::os::raw::c_int,
        V: *const cuDoubleComplex,
        ldv: ::std::os::raw::c_int,
        lwork: *mut ::std::os::raw::c_int,
        params: gesvdjInfo_t,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSgesvdj(
        handle: cusolverDnHandle_t,
        jobz: cusolverEigMode_t,
        econ: ::std::os::raw::c_int,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        A: *mut f32,
        lda: ::std::os::raw::c_int,
        S: *mut f32,
        U: *mut f32,
        ldu: ::std::os::raw::c_int,
        V: *mut f32,
        ldv: ::std::os::raw::c_int,
        work: *mut f32,
        lwork: ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
        params: gesvdjInfo_t,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDgesvdj(
        handle: cusolverDnHandle_t,
        jobz: cusolverEigMode_t,
        econ: ::std::os::raw::c_int,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        A: *mut f64,
        lda: ::std::os::raw::c_int,
        S: *mut f64,
        U: *mut f64,
        ldu: ::std::os::raw::c_int,
        V: *mut f64,
        ldv: ::std::os::raw::c_int,
        work: *mut f64,
        lwork: ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
        params: gesvdjInfo_t,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnCgesvdj(
        handle: cusolverDnHandle_t,
        jobz: cusolverEigMode_t,
        econ: ::std::os::raw::c_int,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        A: *mut cuComplex,
        lda: ::std::os::raw::c_int,
        S: *mut f32,
        U: *mut cuComplex,
        ldu: ::std::os::raw::c_int,
        V: *mut cuComplex,
        ldv: ::std::os::raw::c_int,
        work: *mut cuComplex,
        lwork: ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
        params: gesvdjInfo_t,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZgesvdj(
        handle: cusolverDnHandle_t,
        jobz: cusolverEigMode_t,
        econ: ::std::os::raw::c_int,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        A: *mut cuDoubleComplex,
        lda: ::std::os::raw::c_int,
        S: *mut f64,
        U: *mut cuDoubleComplex,
        ldu: ::std::os::raw::c_int,
        V: *mut cuDoubleComplex,
        ldv: ::std::os::raw::c_int,
        work: *mut cuDoubleComplex,
        lwork: ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
        params: gesvdjInfo_t,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSgesvdaStridedBatched_bufferSize(
        handle: cusolverDnHandle_t,
        jobz: cusolverEigMode_t,
        rank: ::std::os::raw::c_int,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        d_A: *const f32,
        lda: ::std::os::raw::c_int,
        strideA: ::std::os::raw::c_longlong,
        d_S: *const f32,
        strideS: ::std::os::raw::c_longlong,
        d_U: *const f32,
        ldu: ::std::os::raw::c_int,
        strideU: ::std::os::raw::c_longlong,
        d_V: *const f32,
        ldv: ::std::os::raw::c_int,
        strideV: ::std::os::raw::c_longlong,
        lwork: *mut ::std::os::raw::c_int,
        batchSize: ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDgesvdaStridedBatched_bufferSize(
        handle: cusolverDnHandle_t,
        jobz: cusolverEigMode_t,
        rank: ::std::os::raw::c_int,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        d_A: *const f64,
        lda: ::std::os::raw::c_int,
        strideA: ::std::os::raw::c_longlong,
        d_S: *const f64,
        strideS: ::std::os::raw::c_longlong,
        d_U: *const f64,
        ldu: ::std::os::raw::c_int,
        strideU: ::std::os::raw::c_longlong,
        d_V: *const f64,
        ldv: ::std::os::raw::c_int,
        strideV: ::std::os::raw::c_longlong,
        lwork: *mut ::std::os::raw::c_int,
        batchSize: ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnCgesvdaStridedBatched_bufferSize(
        handle: cusolverDnHandle_t,
        jobz: cusolverEigMode_t,
        rank: ::std::os::raw::c_int,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        d_A: *const cuComplex,
        lda: ::std::os::raw::c_int,
        strideA: ::std::os::raw::c_longlong,
        d_S: *const f32,
        strideS: ::std::os::raw::c_longlong,
        d_U: *const cuComplex,
        ldu: ::std::os::raw::c_int,
        strideU: ::std::os::raw::c_longlong,
        d_V: *const cuComplex,
        ldv: ::std::os::raw::c_int,
        strideV: ::std::os::raw::c_longlong,
        lwork: *mut ::std::os::raw::c_int,
        batchSize: ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZgesvdaStridedBatched_bufferSize(
        handle: cusolverDnHandle_t,
        jobz: cusolverEigMode_t,
        rank: ::std::os::raw::c_int,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        d_A: *const cuDoubleComplex,
        lda: ::std::os::raw::c_int,
        strideA: ::std::os::raw::c_longlong,
        d_S: *const f64,
        strideS: ::std::os::raw::c_longlong,
        d_U: *const cuDoubleComplex,
        ldu: ::std::os::raw::c_int,
        strideU: ::std::os::raw::c_longlong,
        d_V: *const cuDoubleComplex,
        ldv: ::std::os::raw::c_int,
        strideV: ::std::os::raw::c_longlong,
        lwork: *mut ::std::os::raw::c_int,
        batchSize: ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSgesvdaStridedBatched(
        handle: cusolverDnHandle_t,
        jobz: cusolverEigMode_t,
        rank: ::std::os::raw::c_int,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        d_A: *const f32,
        lda: ::std::os::raw::c_int,
        strideA: ::std::os::raw::c_longlong,
        d_S: *mut f32,
        strideS: ::std::os::raw::c_longlong,
        d_U: *mut f32,
        ldu: ::std::os::raw::c_int,
        strideU: ::std::os::raw::c_longlong,
        d_V: *mut f32,
        ldv: ::std::os::raw::c_int,
        strideV: ::std::os::raw::c_longlong,
        d_work: *mut f32,
        lwork: ::std::os::raw::c_int,
        d_info: *mut ::std::os::raw::c_int,
        h_R_nrmF: *mut f64,
        batchSize: ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDgesvdaStridedBatched(
        handle: cusolverDnHandle_t,
        jobz: cusolverEigMode_t,
        rank: ::std::os::raw::c_int,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        d_A: *const f64,
        lda: ::std::os::raw::c_int,
        strideA: ::std::os::raw::c_longlong,
        d_S: *mut f64,
        strideS: ::std::os::raw::c_longlong,
        d_U: *mut f64,
        ldu: ::std::os::raw::c_int,
        strideU: ::std::os::raw::c_longlong,
        d_V: *mut f64,
        ldv: ::std::os::raw::c_int,
        strideV: ::std::os::raw::c_longlong,
        d_work: *mut f64,
        lwork: ::std::os::raw::c_int,
        d_info: *mut ::std::os::raw::c_int,
        h_R_nrmF: *mut f64,
        batchSize: ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnCgesvdaStridedBatched(
        handle: cusolverDnHandle_t,
        jobz: cusolverEigMode_t,
        rank: ::std::os::raw::c_int,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        d_A: *const cuComplex,
        lda: ::std::os::raw::c_int,
        strideA: ::std::os::raw::c_longlong,
        d_S: *mut f32,
        strideS: ::std::os::raw::c_longlong,
        d_U: *mut cuComplex,
        ldu: ::std::os::raw::c_int,
        strideU: ::std::os::raw::c_longlong,
        d_V: *mut cuComplex,
        ldv: ::std::os::raw::c_int,
        strideV: ::std::os::raw::c_longlong,
        d_work: *mut cuComplex,
        lwork: ::std::os::raw::c_int,
        d_info: *mut ::std::os::raw::c_int,
        h_R_nrmF: *mut f64,
        batchSize: ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnZgesvdaStridedBatched(
        handle: cusolverDnHandle_t,
        jobz: cusolverEigMode_t,
        rank: ::std::os::raw::c_int,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        d_A: *const cuDoubleComplex,
        lda: ::std::os::raw::c_int,
        strideA: ::std::os::raw::c_longlong,
        d_S: *mut f64,
        strideS: ::std::os::raw::c_longlong,
        d_U: *mut cuDoubleComplex,
        ldu: ::std::os::raw::c_int,
        strideU: ::std::os::raw::c_longlong,
        d_V: *mut cuDoubleComplex,
        ldv: ::std::os::raw::c_int,
        strideV: ::std::os::raw::c_longlong,
        d_work: *mut cuDoubleComplex,
        lwork: ::std::os::raw::c_int,
        d_info: *mut ::std::os::raw::c_int,
        h_R_nrmF: *mut f64,
        batchSize: ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnCreateParams(params: *mut cusolverDnParams_t) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnDestroyParams(params: cusolverDnParams_t) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnSetAdvOptions(params: cusolverDnParams_t, function: cusolverDnFunction_t, algo: cusolverAlgMode_t) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnXpotrf_bufferSize(
        handle: cusolverDnHandle_t,
        params: cusolverDnParams_t,
        uplo: cublasFillMode_t,
        n: i64,
        dataTypeA: cudaDataType,
        A: *const ::std::os::raw::c_void,
        lda: i64,
        computeType: cudaDataType,
        workspaceInBytesOnDevice: *mut usize,
        workspaceInBytesOnHost: *mut usize,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnXpotrf(
        handle: cusolverDnHandle_t,
        params: cusolverDnParams_t,
        uplo: cublasFillMode_t,
        n: i64,
        dataTypeA: cudaDataType,
        A: *mut ::std::os::raw::c_void,
        lda: i64,
        computeType: cudaDataType,
        bufferOnDevice: *mut ::std::os::raw::c_void,
        workspaceInBytesOnDevice: usize,
        bufferOnHost: *mut ::std::os::raw::c_void,
        workspaceInBytesOnHost: usize,
        info: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnXpotrs(
        handle: cusolverDnHandle_t,
        params: cusolverDnParams_t,
        uplo: cublasFillMode_t,
        n: i64,
        nrhs: i64,
        dataTypeA: cudaDataType,
        A: *const ::std::os::raw::c_void,
        lda: i64,
        dataTypeB: cudaDataType,
        B: *mut ::std::os::raw::c_void,
        ldb: i64,
        info: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnXgeqrf_bufferSize(
        handle: cusolverDnHandle_t,
        params: cusolverDnParams_t,
        m: i64,
        n: i64,
        dataTypeA: cudaDataType,
        A: *const ::std::os::raw::c_void,
        lda: i64,
        dataTypeTau: cudaDataType,
        tau: *const ::std::os::raw::c_void,
        computeType: cudaDataType,
        workspaceInBytesOnDevice: *mut usize,
        workspaceInBytesOnHost: *mut usize,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnXgeqrf(
        handle: cusolverDnHandle_t,
        params: cusolverDnParams_t,
        m: i64,
        n: i64,
        dataTypeA: cudaDataType,
        A: *mut ::std::os::raw::c_void,
        lda: i64,
        dataTypeTau: cudaDataType,
        tau: *mut ::std::os::raw::c_void,
        computeType: cudaDataType,
        bufferOnDevice: *mut ::std::os::raw::c_void,
        workspaceInBytesOnDevice: usize,
        bufferOnHost: *mut ::std::os::raw::c_void,
        workspaceInBytesOnHost: usize,
        info: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnXgetrf_bufferSize(
        handle: cusolverDnHandle_t,
        params: cusolverDnParams_t,
        m: i64,
        n: i64,
        dataTypeA: cudaDataType,
        A: *const ::std::os::raw::c_void,
        lda: i64,
        computeType: cudaDataType,
        workspaceInBytesOnDevice: *mut usize,
        workspaceInBytesOnHost: *mut usize,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnXgetrf(
        handle: cusolverDnHandle_t,
        params: cusolverDnParams_t,
        m: i64,
        n: i64,
        dataTypeA: cudaDataType,
        A: *mut ::std::os::raw::c_void,
        lda: i64,
        ipiv: *mut i64,
        computeType: cudaDataType,
        bufferOnDevice: *mut ::std::os::raw::c_void,
        workspaceInBytesOnDevice: usize,
        bufferOnHost: *mut ::std::os::raw::c_void,
        workspaceInBytesOnHost: usize,
        info: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnXgetrs(
        handle: cusolverDnHandle_t,
        params: cusolverDnParams_t,
        trans: cublasOperation_t,
        n: i64,
        nrhs: i64,
        dataTypeA: cudaDataType,
        A: *const ::std::os::raw::c_void,
        lda: i64,
        ipiv: *const i64,
        dataTypeB: cudaDataType,
        B: *mut ::std::os::raw::c_void,
        ldb: i64,
        info: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnXsyevd_bufferSize(
        handle: cusolverDnHandle_t,
        params: cusolverDnParams_t,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: i64,
        dataTypeA: cudaDataType,
        A: *const ::std::os::raw::c_void,
        lda: i64,
        dataTypeW: cudaDataType,
        W: *const ::std::os::raw::c_void,
        computeType: cudaDataType,
        workspaceInBytesOnDevice: *mut usize,
        workspaceInBytesOnHost: *mut usize,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnXsyevd(
        handle: cusolverDnHandle_t,
        params: cusolverDnParams_t,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: i64,
        dataTypeA: cudaDataType,
        A: *mut ::std::os::raw::c_void,
        lda: i64,
        dataTypeW: cudaDataType,
        W: *mut ::std::os::raw::c_void,
        computeType: cudaDataType,
        bufferOnDevice: *mut ::std::os::raw::c_void,
        workspaceInBytesOnDevice: usize,
        bufferOnHost: *mut ::std::os::raw::c_void,
        workspaceInBytesOnHost: usize,
        info: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnXstedc_bufferSize(
        handle: cusolverDnHandle_t,
        params: cusolverDnParams_t,
        compz: cusolverEigComp_t,
        n: i64,
        dataTypeDE: cudaDataType,
        D: *const ::std::os::raw::c_void,
        E: *const ::std::os::raw::c_void,
        dataTypeZ: cudaDataType,
        Z: *const ::std::os::raw::c_void,
        ldz: i64,
        computeType: cudaDataType,
        workspaceInBytesOnDevice: *mut usize,
        workspaceInBytesOnHost: *mut usize,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnXstedc(
        handle: cusolverDnHandle_t,
        params: cusolverDnParams_t,
        compz: cusolverEigComp_t,
        n: i64,
        dataTypeDE: cudaDataType,
        D: *mut ::std::os::raw::c_void,
        E: *mut ::std::os::raw::c_void,
        dataTypeZ: cudaDataType,
        Z: *mut ::std::os::raw::c_void,
        ldz: i64,
        computeType: cudaDataType,
        bufferOnDevice: *mut ::std::os::raw::c_void,
        workspaceInBytesOnDevice: usize,
        bufferOnHost: *mut ::std::os::raw::c_void,
        workspaceInBytesOnHost: usize,
        info: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnXsyevBatched_bufferSize(
        handle: cusolverDnHandle_t,
        params: cusolverDnParams_t,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: i64,
        dataTypeA: cudaDataType,
        A: *const ::std::os::raw::c_void,
        lda: i64,
        dataTypeW: cudaDataType,
        W: *const ::std::os::raw::c_void,
        computeType: cudaDataType,
        workspaceInBytesOnDevice: *mut usize,
        workspaceInBytesOnHost: *mut usize,
        batchSize: i64,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnXsyevBatched(
        handle: cusolverDnHandle_t,
        params: cusolverDnParams_t,
        jobz: cusolverEigMode_t,
        uplo: cublasFillMode_t,
        n: i64,
        dataTypeA: cudaDataType,
        A: *mut ::std::os::raw::c_void,
        lda: i64,
        dataTypeW: cudaDataType,
        W: *mut ::std::os::raw::c_void,
        computeType: cudaDataType,
        bufferOnDevice: *mut ::std::os::raw::c_void,
        workspaceInBytesOnDevice: usize,
        bufferOnHost: *mut ::std::os::raw::c_void,
        workspaceInBytesOnHost: usize,
        info: *mut ::std::os::raw::c_int,
        batchSize: i64,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnXsyevdx_bufferSize(
        handle: cusolverDnHandle_t,
        params: cusolverDnParams_t,
        jobz: cusolverEigMode_t,
        range: cusolverEigRange_t,
        uplo: cublasFillMode_t,
        n: i64,
        dataTypeA: cudaDataType,
        A: *const ::std::os::raw::c_void,
        lda: i64,
        vl: *mut ::std::os::raw::c_void,
        vu: *mut ::std::os::raw::c_void,
        il: i64,
        iu: i64,
        h_meig: *mut i64,
        dataTypeW: cudaDataType,
        W: *const ::std::os::raw::c_void,
        computeType: cudaDataType,
        workspaceInBytesOnDevice: *mut usize,
        workspaceInBytesOnHost: *mut usize,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnXsyevdx(
        handle: cusolverDnHandle_t,
        params: cusolverDnParams_t,
        jobz: cusolverEigMode_t,
        range: cusolverEigRange_t,
        uplo: cublasFillMode_t,
        n: i64,
        dataTypeA: cudaDataType,
        A: *mut ::std::os::raw::c_void,
        lda: i64,
        vl: *mut ::std::os::raw::c_void,
        vu: *mut ::std::os::raw::c_void,
        il: i64,
        iu: i64,
        meig64: *mut i64,
        dataTypeW: cudaDataType,
        W: *mut ::std::os::raw::c_void,
        computeType: cudaDataType,
        bufferOnDevice: *mut ::std::os::raw::c_void,
        workspaceInBytesOnDevice: usize,
        bufferOnHost: *mut ::std::os::raw::c_void,
        workspaceInBytesOnHost: usize,
        info: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnXgeev_bufferSize(
        handle: cusolverDnHandle_t,
        params: cusolverDnParams_t,
        jobvl: cusolverEigMode_t,
        jobvr: cusolverEigMode_t,
        n: i64,
        dataTypeA: cudaDataType,
        A: *const ::std::os::raw::c_void,
        lda: i64,
        dataTypeW: cudaDataType,
        W: *const ::std::os::raw::c_void,
        dataTypeVL: cudaDataType,
        VL: *const ::std::os::raw::c_void,
        ldvl: i64,
        dataTypeVR: cudaDataType,
        VR: *const ::std::os::raw::c_void,
        ldvr: i64,
        computeType: cudaDataType,
        workspaceInBytesOnDevice: *mut usize,
        workspaceInBytesOnHost: *mut usize,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnXgeev(
        handle: cusolverDnHandle_t,
        params: cusolverDnParams_t,
        jobvl: cusolverEigMode_t,
        jobvr: cusolverEigMode_t,
        n: i64,
        dataTypeA: cudaDataType,
        A: *mut ::std::os::raw::c_void,
        lda: i64,
        dataTypeW: cudaDataType,
        W: *mut ::std::os::raw::c_void,
        dataTypeVL: cudaDataType,
        VL: *mut ::std::os::raw::c_void,
        ldvl: i64,
        dataTypeVR: cudaDataType,
        VR: *mut ::std::os::raw::c_void,
        ldvr: i64,
        computeType: cudaDataType,
        bufferOnDevice: *mut ::std::os::raw::c_void,
        workspaceInBytesOnDevice: usize,
        bufferOnHost: *mut ::std::os::raw::c_void,
        workspaceInBytesOnHost: usize,
        info: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnXgesvd_bufferSize(
        handle: cusolverDnHandle_t,
        params: cusolverDnParams_t,
        jobu: ::std::os::raw::c_schar,
        jobvt: ::std::os::raw::c_schar,
        m: i64,
        n: i64,
        dataTypeA: cudaDataType,
        A: *const ::std::os::raw::c_void,
        lda: i64,
        dataTypeS: cudaDataType,
        S: *const ::std::os::raw::c_void,
        dataTypeU: cudaDataType,
        U: *const ::std::os::raw::c_void,
        ldu: i64,
        dataTypeVT: cudaDataType,
        VT: *const ::std::os::raw::c_void,
        ldvt: i64,
        computeType: cudaDataType,
        workspaceInBytesOnDevice: *mut usize,
        workspaceInBytesOnHost: *mut usize,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnXgesvd(
        handle: cusolverDnHandle_t,
        params: cusolverDnParams_t,
        jobu: ::std::os::raw::c_schar,
        jobvt: ::std::os::raw::c_schar,
        m: i64,
        n: i64,
        dataTypeA: cudaDataType,
        A: *mut ::std::os::raw::c_void,
        lda: i64,
        dataTypeS: cudaDataType,
        S: *mut ::std::os::raw::c_void,
        dataTypeU: cudaDataType,
        U: *mut ::std::os::raw::c_void,
        ldu: i64,
        dataTypeVT: cudaDataType,
        VT: *mut ::std::os::raw::c_void,
        ldvt: i64,
        computeType: cudaDataType,
        bufferOnDevice: *mut ::std::os::raw::c_void,
        workspaceInBytesOnDevice: usize,
        bufferOnHost: *mut ::std::os::raw::c_void,
        workspaceInBytesOnHost: usize,
        info: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnXgesvdp_bufferSize(
        handle: cusolverDnHandle_t,
        params: cusolverDnParams_t,
        jobz: cusolverEigMode_t,
        econ: ::std::os::raw::c_int,
        m: i64,
        n: i64,
        dataTypeA: cudaDataType,
        A: *const ::std::os::raw::c_void,
        lda: i64,
        dataTypeS: cudaDataType,
        S: *const ::std::os::raw::c_void,
        dataTypeU: cudaDataType,
        U: *const ::std::os::raw::c_void,
        ldu: i64,
        dataTypeV: cudaDataType,
        V: *const ::std::os::raw::c_void,
        ldv: i64,
        computeType: cudaDataType,
        workspaceInBytesOnDevice: *mut usize,
        workspaceInBytesOnHost: *mut usize,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnXgesvdp(
        handle: cusolverDnHandle_t,
        params: cusolverDnParams_t,
        jobz: cusolverEigMode_t,
        econ: ::std::os::raw::c_int,
        m: i64,
        n: i64,
        dataTypeA: cudaDataType,
        A: *mut ::std::os::raw::c_void,
        lda: i64,
        dataTypeS: cudaDataType,
        S: *mut ::std::os::raw::c_void,
        dataTypeU: cudaDataType,
        U: *mut ::std::os::raw::c_void,
        ldu: i64,
        dataTypeV: cudaDataType,
        V: *mut ::std::os::raw::c_void,
        ldv: i64,
        computeType: cudaDataType,
        bufferOnDevice: *mut ::std::os::raw::c_void,
        workspaceInBytesOnDevice: usize,
        bufferOnHost: *mut ::std::os::raw::c_void,
        workspaceInBytesOnHost: usize,
        d_info: *mut ::std::os::raw::c_int,
        h_err_sigma: *mut f64,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnXgesvdr_bufferSize(
        handle: cusolverDnHandle_t,
        params: cusolverDnParams_t,
        jobu: ::std::os::raw::c_schar,
        jobv: ::std::os::raw::c_schar,
        m: i64,
        n: i64,
        k: i64,
        p: i64,
        niters: i64,
        dataTypeA: cudaDataType,
        A: *const ::std::os::raw::c_void,
        lda: i64,
        dataTypeSrand: cudaDataType,
        Srand: *const ::std::os::raw::c_void,
        dataTypeUrand: cudaDataType,
        Urand: *const ::std::os::raw::c_void,
        ldUrand: i64,
        dataTypeVrand: cudaDataType,
        Vrand: *const ::std::os::raw::c_void,
        ldVrand: i64,
        computeType: cudaDataType,
        workspaceInBytesOnDevice: *mut usize,
        workspaceInBytesOnHost: *mut usize,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnXgesvdr(
        handle: cusolverDnHandle_t,
        params: cusolverDnParams_t,
        jobu: ::std::os::raw::c_schar,
        jobv: ::std::os::raw::c_schar,
        m: i64,
        n: i64,
        k: i64,
        p: i64,
        niters: i64,
        dataTypeA: cudaDataType,
        A: *mut ::std::os::raw::c_void,
        lda: i64,
        dataTypeSrand: cudaDataType,
        Srand: *mut ::std::os::raw::c_void,
        dataTypeUrand: cudaDataType,
        Urand: *mut ::std::os::raw::c_void,
        ldUrand: i64,
        dataTypeVrand: cudaDataType,
        Vrand: *mut ::std::os::raw::c_void,
        ldVrand: i64,
        computeType: cudaDataType,
        bufferOnDevice: *mut ::std::os::raw::c_void,
        workspaceInBytesOnDevice: usize,
        bufferOnHost: *mut ::std::os::raw::c_void,
        workspaceInBytesOnHost: usize,
        d_info: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnXlarft_bufferSize(
        handle: cusolverDnHandle_t,
        params: cusolverDnParams_t,
        direct: cusolverDirectMode_t,
        storev: cusolverStorevMode_t,
        n: i64,
        k: i64,
        dataTypeV: cudaDataType,
        V: *const ::std::os::raw::c_void,
        ldv: i64,
        dataTypeTau: cudaDataType,
        tau: *const ::std::os::raw::c_void,
        dataTypeT: cudaDataType,
        T: *mut ::std::os::raw::c_void,
        ldt: i64,
        computeType: cudaDataType,
        workspaceInBytesOnDevice: *mut usize,
        workspaceInBytesOnHost: *mut usize,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnXlarft(
        handle: cusolverDnHandle_t,
        params: cusolverDnParams_t,
        direct: cusolverDirectMode_t,
        storev: cusolverStorevMode_t,
        n: i64,
        k: i64,
        dataTypeV: cudaDataType,
        V: *const ::std::os::raw::c_void,
        ldv: i64,
        dataTypeTau: cudaDataType,
        tau: *const ::std::os::raw::c_void,
        dataTypeT: cudaDataType,
        T: *mut ::std::os::raw::c_void,
        ldt: i64,
        computeType: cudaDataType,
        bufferOnDevice: *mut ::std::os::raw::c_void,
        workspaceInBytesOnDevice: usize,
        bufferOnHost: *mut ::std::os::raw::c_void,
        workspaceInBytesOnHost: usize,
    ) -> cusolverStatus_t;
}
pub type cusolverDnLoggerCallback_t = ::std::option::Option<unsafe extern "C" fn(logLevel: ::std::os::raw::c_int, functionName: *const ::std::os::raw::c_char, message: *const ::std::os::raw::c_char)>;
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnLoggerSetCallback(callback: cusolverDnLoggerCallback_t) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnLoggerSetFile(file: *mut FILE) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnLoggerOpenFile(logFile: *const ::std::os::raw::c_char) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnLoggerSetLevel(level: ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnLoggerSetMask(mask: ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnLoggerForceDisable() -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnXpolar_bufferSize(
        handle: cusolverDnHandle_t,
        params: cusolverDnParams_t,
        uplo: cublasFillMode_t,
        M: i64,
        N: i64,
        dataTypeA: cudaDataType,
        A: *const ::std::os::raw::c_void,
        lda: i64,
        dataTypeH: cudaDataType,
        H: *const ::std::os::raw::c_void,
        ldh: i64,
        computeType: cudaDataType,
        workspaceInBytesOnDevice: *mut usize,
        workspaceInBytesOnHost: *mut usize,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverDnXpolar(
        handle: cusolverDnHandle_t,
        params: cusolverDnParams_t,
        uplo: cublasFillMode_t,
        M: i64,
        N: i64,
        dataTypeA: cudaDataType,
        A: *mut ::std::os::raw::c_void,
        lda: i64,
        dataTypeH: cudaDataType,
        H: *mut ::std::os::raw::c_void,
        ldh: i64,
        computeType: cudaDataType,
        bufferOnDevice: *mut ::std::os::raw::c_void,
        workspaceInBytesOnDevice: usize,
        bufferOnHost: *mut ::std::os::raw::c_void,
        workspaceInBytesOnHost: usize,
        d_res_nrm: *mut f64,
        d_A_nrmF: *mut f64,
        d_rcond: *mut f64,
        d_info: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cusparseMatDescr {
    _unused: [u8; 0],
}
pub type cusparseMatDescr_t = *mut cusparseMatDescr;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cusolverSpContext {
    _unused: [u8; 0],
}
pub type cusolverSpHandle_t = *mut cusolverSpContext;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct csrqrInfo {
    _unused: [u8; 0],
}
pub type csrqrInfo_t = *mut csrqrInfo;
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverSpCreate(handle: *mut cusolverSpHandle_t) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverSpDestroy(handle: cusolverSpHandle_t) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverSpSetStream(handle: cusolverSpHandle_t, streamId: cudaStream_t) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverSpGetStream(handle: cusolverSpHandle_t, streamId: *mut cudaStream_t) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverSpXcsrissymHost(
        handle: cusolverSpHandle_t,
        m: ::std::os::raw::c_int,
        nnzA: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrRowPtrA: *const ::std::os::raw::c_int,
        csrEndPtrA: *const ::std::os::raw::c_int,
        csrColIndA: *const ::std::os::raw::c_int,
        issym: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverSpScsrlsvluHost(
        handle: cusolverSpHandle_t,
        n: ::std::os::raw::c_int,
        nnzA: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrValA: *const f32,
        csrRowPtrA: *const ::std::os::raw::c_int,
        csrColIndA: *const ::std::os::raw::c_int,
        b: *const f32,
        tol: f32,
        reorder: ::std::os::raw::c_int,
        x: *mut f32,
        singularity: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverSpDcsrlsvluHost(
        handle: cusolverSpHandle_t,
        n: ::std::os::raw::c_int,
        nnzA: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrValA: *const f64,
        csrRowPtrA: *const ::std::os::raw::c_int,
        csrColIndA: *const ::std::os::raw::c_int,
        b: *const f64,
        tol: f64,
        reorder: ::std::os::raw::c_int,
        x: *mut f64,
        singularity: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverSpCcsrlsvluHost(
        handle: cusolverSpHandle_t,
        n: ::std::os::raw::c_int,
        nnzA: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrValA: *const cuComplex,
        csrRowPtrA: *const ::std::os::raw::c_int,
        csrColIndA: *const ::std::os::raw::c_int,
        b: *const cuComplex,
        tol: f32,
        reorder: ::std::os::raw::c_int,
        x: *mut cuComplex,
        singularity: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverSpZcsrlsvluHost(
        handle: cusolverSpHandle_t,
        n: ::std::os::raw::c_int,
        nnzA: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrValA: *const cuDoubleComplex,
        csrRowPtrA: *const ::std::os::raw::c_int,
        csrColIndA: *const ::std::os::raw::c_int,
        b: *const cuDoubleComplex,
        tol: f64,
        reorder: ::std::os::raw::c_int,
        x: *mut cuDoubleComplex,
        singularity: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverSpScsrlsvqr(
        handle: cusolverSpHandle_t,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrVal: *const f32,
        csrRowPtr: *const ::std::os::raw::c_int,
        csrColInd: *const ::std::os::raw::c_int,
        b: *const f32,
        tol: f32,
        reorder: ::std::os::raw::c_int,
        x: *mut f32,
        singularity: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverSpDcsrlsvqr(
        handle: cusolverSpHandle_t,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrVal: *const f64,
        csrRowPtr: *const ::std::os::raw::c_int,
        csrColInd: *const ::std::os::raw::c_int,
        b: *const f64,
        tol: f64,
        reorder: ::std::os::raw::c_int,
        x: *mut f64,
        singularity: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverSpCcsrlsvqr(
        handle: cusolverSpHandle_t,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrVal: *const cuComplex,
        csrRowPtr: *const ::std::os::raw::c_int,
        csrColInd: *const ::std::os::raw::c_int,
        b: *const cuComplex,
        tol: f32,
        reorder: ::std::os::raw::c_int,
        x: *mut cuComplex,
        singularity: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverSpZcsrlsvqr(
        handle: cusolverSpHandle_t,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrVal: *const cuDoubleComplex,
        csrRowPtr: *const ::std::os::raw::c_int,
        csrColInd: *const ::std::os::raw::c_int,
        b: *const cuDoubleComplex,
        tol: f64,
        reorder: ::std::os::raw::c_int,
        x: *mut cuDoubleComplex,
        singularity: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverSpScsrlsvqrHost(
        handle: cusolverSpHandle_t,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrValA: *const f32,
        csrRowPtrA: *const ::std::os::raw::c_int,
        csrColIndA: *const ::std::os::raw::c_int,
        b: *const f32,
        tol: f32,
        reorder: ::std::os::raw::c_int,
        x: *mut f32,
        singularity: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverSpDcsrlsvqrHost(
        handle: cusolverSpHandle_t,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrValA: *const f64,
        csrRowPtrA: *const ::std::os::raw::c_int,
        csrColIndA: *const ::std::os::raw::c_int,
        b: *const f64,
        tol: f64,
        reorder: ::std::os::raw::c_int,
        x: *mut f64,
        singularity: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverSpCcsrlsvqrHost(
        handle: cusolverSpHandle_t,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrValA: *const cuComplex,
        csrRowPtrA: *const ::std::os::raw::c_int,
        csrColIndA: *const ::std::os::raw::c_int,
        b: *const cuComplex,
        tol: f32,
        reorder: ::std::os::raw::c_int,
        x: *mut cuComplex,
        singularity: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverSpZcsrlsvqrHost(
        handle: cusolverSpHandle_t,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrValA: *const cuDoubleComplex,
        csrRowPtrA: *const ::std::os::raw::c_int,
        csrColIndA: *const ::std::os::raw::c_int,
        b: *const cuDoubleComplex,
        tol: f64,
        reorder: ::std::os::raw::c_int,
        x: *mut cuDoubleComplex,
        singularity: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverSpScsrlsvcholHost(
        handle: cusolverSpHandle_t,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrVal: *const f32,
        csrRowPtr: *const ::std::os::raw::c_int,
        csrColInd: *const ::std::os::raw::c_int,
        b: *const f32,
        tol: f32,
        reorder: ::std::os::raw::c_int,
        x: *mut f32,
        singularity: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverSpDcsrlsvcholHost(
        handle: cusolverSpHandle_t,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrVal: *const f64,
        csrRowPtr: *const ::std::os::raw::c_int,
        csrColInd: *const ::std::os::raw::c_int,
        b: *const f64,
        tol: f64,
        reorder: ::std::os::raw::c_int,
        x: *mut f64,
        singularity: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverSpCcsrlsvcholHost(
        handle: cusolverSpHandle_t,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrVal: *const cuComplex,
        csrRowPtr: *const ::std::os::raw::c_int,
        csrColInd: *const ::std::os::raw::c_int,
        b: *const cuComplex,
        tol: f32,
        reorder: ::std::os::raw::c_int,
        x: *mut cuComplex,
        singularity: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverSpZcsrlsvcholHost(
        handle: cusolverSpHandle_t,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrVal: *const cuDoubleComplex,
        csrRowPtr: *const ::std::os::raw::c_int,
        csrColInd: *const ::std::os::raw::c_int,
        b: *const cuDoubleComplex,
        tol: f64,
        reorder: ::std::os::raw::c_int,
        x: *mut cuDoubleComplex,
        singularity: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverSpScsrlsvchol(
        handle: cusolverSpHandle_t,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrVal: *const f32,
        csrRowPtr: *const ::std::os::raw::c_int,
        csrColInd: *const ::std::os::raw::c_int,
        b: *const f32,
        tol: f32,
        reorder: ::std::os::raw::c_int,
        x: *mut f32,
        singularity: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverSpDcsrlsvchol(
        handle: cusolverSpHandle_t,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrVal: *const f64,
        csrRowPtr: *const ::std::os::raw::c_int,
        csrColInd: *const ::std::os::raw::c_int,
        b: *const f64,
        tol: f64,
        reorder: ::std::os::raw::c_int,
        x: *mut f64,
        singularity: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverSpCcsrlsvchol(
        handle: cusolverSpHandle_t,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrVal: *const cuComplex,
        csrRowPtr: *const ::std::os::raw::c_int,
        csrColInd: *const ::std::os::raw::c_int,
        b: *const cuComplex,
        tol: f32,
        reorder: ::std::os::raw::c_int,
        x: *mut cuComplex,
        singularity: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverSpZcsrlsvchol(
        handle: cusolverSpHandle_t,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrVal: *const cuDoubleComplex,
        csrRowPtr: *const ::std::os::raw::c_int,
        csrColInd: *const ::std::os::raw::c_int,
        b: *const cuDoubleComplex,
        tol: f64,
        reorder: ::std::os::raw::c_int,
        x: *mut cuDoubleComplex,
        singularity: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverSpScsrlsqvqrHost(
        handle: cusolverSpHandle_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrValA: *const f32,
        csrRowPtrA: *const ::std::os::raw::c_int,
        csrColIndA: *const ::std::os::raw::c_int,
        b: *const f32,
        tol: f32,
        rankA: *mut ::std::os::raw::c_int,
        x: *mut f32,
        p: *mut ::std::os::raw::c_int,
        min_norm: *mut f32,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverSpDcsrlsqvqrHost(
        handle: cusolverSpHandle_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrValA: *const f64,
        csrRowPtrA: *const ::std::os::raw::c_int,
        csrColIndA: *const ::std::os::raw::c_int,
        b: *const f64,
        tol: f64,
        rankA: *mut ::std::os::raw::c_int,
        x: *mut f64,
        p: *mut ::std::os::raw::c_int,
        min_norm: *mut f64,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverSpCcsrlsqvqrHost(
        handle: cusolverSpHandle_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrValA: *const cuComplex,
        csrRowPtrA: *const ::std::os::raw::c_int,
        csrColIndA: *const ::std::os::raw::c_int,
        b: *const cuComplex,
        tol: f32,
        rankA: *mut ::std::os::raw::c_int,
        x: *mut cuComplex,
        p: *mut ::std::os::raw::c_int,
        min_norm: *mut f32,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverSpZcsrlsqvqrHost(
        handle: cusolverSpHandle_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrValA: *const cuDoubleComplex,
        csrRowPtrA: *const ::std::os::raw::c_int,
        csrColIndA: *const ::std::os::raw::c_int,
        b: *const cuDoubleComplex,
        tol: f64,
        rankA: *mut ::std::os::raw::c_int,
        x: *mut cuDoubleComplex,
        p: *mut ::std::os::raw::c_int,
        min_norm: *mut f64,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverSpScsreigvsiHost(
        handle: cusolverSpHandle_t,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrValA: *const f32,
        csrRowPtrA: *const ::std::os::raw::c_int,
        csrColIndA: *const ::std::os::raw::c_int,
        mu0: f32,
        x0: *const f32,
        maxite: ::std::os::raw::c_int,
        tol: f32,
        mu: *mut f32,
        x: *mut f32,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverSpDcsreigvsiHost(
        handle: cusolverSpHandle_t,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrValA: *const f64,
        csrRowPtrA: *const ::std::os::raw::c_int,
        csrColIndA: *const ::std::os::raw::c_int,
        mu0: f64,
        x0: *const f64,
        maxite: ::std::os::raw::c_int,
        tol: f64,
        mu: *mut f64,
        x: *mut f64,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverSpCcsreigvsiHost(
        handle: cusolverSpHandle_t,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrValA: *const cuComplex,
        csrRowPtrA: *const ::std::os::raw::c_int,
        csrColIndA: *const ::std::os::raw::c_int,
        mu0: cuComplex,
        x0: *const cuComplex,
        maxite: ::std::os::raw::c_int,
        tol: f32,
        mu: *mut cuComplex,
        x: *mut cuComplex,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverSpZcsreigvsiHost(
        handle: cusolverSpHandle_t,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrValA: *const cuDoubleComplex,
        csrRowPtrA: *const ::std::os::raw::c_int,
        csrColIndA: *const ::std::os::raw::c_int,
        mu0: cuDoubleComplex,
        x0: *const cuDoubleComplex,
        maxite: ::std::os::raw::c_int,
        tol: f64,
        mu: *mut cuDoubleComplex,
        x: *mut cuDoubleComplex,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverSpScsreigvsi(
        handle: cusolverSpHandle_t,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrValA: *const f32,
        csrRowPtrA: *const ::std::os::raw::c_int,
        csrColIndA: *const ::std::os::raw::c_int,
        mu0: f32,
        x0: *const f32,
        maxite: ::std::os::raw::c_int,
        eps: f32,
        mu: *mut f32,
        x: *mut f32,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverSpDcsreigvsi(
        handle: cusolverSpHandle_t,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrValA: *const f64,
        csrRowPtrA: *const ::std::os::raw::c_int,
        csrColIndA: *const ::std::os::raw::c_int,
        mu0: f64,
        x0: *const f64,
        maxite: ::std::os::raw::c_int,
        eps: f64,
        mu: *mut f64,
        x: *mut f64,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverSpCcsreigvsi(
        handle: cusolverSpHandle_t,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrValA: *const cuComplex,
        csrRowPtrA: *const ::std::os::raw::c_int,
        csrColIndA: *const ::std::os::raw::c_int,
        mu0: cuComplex,
        x0: *const cuComplex,
        maxite: ::std::os::raw::c_int,
        eps: f32,
        mu: *mut cuComplex,
        x: *mut cuComplex,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverSpZcsreigvsi(
        handle: cusolverSpHandle_t,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrValA: *const cuDoubleComplex,
        csrRowPtrA: *const ::std::os::raw::c_int,
        csrColIndA: *const ::std::os::raw::c_int,
        mu0: cuDoubleComplex,
        x0: *const cuDoubleComplex,
        maxite: ::std::os::raw::c_int,
        eps: f64,
        mu: *mut cuDoubleComplex,
        x: *mut cuDoubleComplex,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverSpScsreigsHost(
        handle: cusolverSpHandle_t,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrValA: *const f32,
        csrRowPtrA: *const ::std::os::raw::c_int,
        csrColIndA: *const ::std::os::raw::c_int,
        left_bottom_corner: cuComplex,
        right_upper_corner: cuComplex,
        num_eigs: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverSpDcsreigsHost(
        handle: cusolverSpHandle_t,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrValA: *const f64,
        csrRowPtrA: *const ::std::os::raw::c_int,
        csrColIndA: *const ::std::os::raw::c_int,
        left_bottom_corner: cuDoubleComplex,
        right_upper_corner: cuDoubleComplex,
        num_eigs: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverSpCcsreigsHost(
        handle: cusolverSpHandle_t,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrValA: *const cuComplex,
        csrRowPtrA: *const ::std::os::raw::c_int,
        csrColIndA: *const ::std::os::raw::c_int,
        left_bottom_corner: cuComplex,
        right_upper_corner: cuComplex,
        num_eigs: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverSpZcsreigsHost(
        handle: cusolverSpHandle_t,
        m: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrValA: *const cuDoubleComplex,
        csrRowPtrA: *const ::std::os::raw::c_int,
        csrColIndA: *const ::std::os::raw::c_int,
        left_bottom_corner: cuDoubleComplex,
        right_upper_corner: cuDoubleComplex,
        num_eigs: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverSpXcsrsymrcmHost(handle: cusolverSpHandle_t, n: ::std::os::raw::c_int, nnzA: ::std::os::raw::c_int, descrA: cusparseMatDescr_t, csrRowPtrA: *const ::std::os::raw::c_int, csrColIndA: *const ::std::os::raw::c_int, p: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverSpXcsrsymmdqHost(handle: cusolverSpHandle_t, n: ::std::os::raw::c_int, nnzA: ::std::os::raw::c_int, descrA: cusparseMatDescr_t, csrRowPtrA: *const ::std::os::raw::c_int, csrColIndA: *const ::std::os::raw::c_int, p: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverSpXcsrsymamdHost(handle: cusolverSpHandle_t, n: ::std::os::raw::c_int, nnzA: ::std::os::raw::c_int, descrA: cusparseMatDescr_t, csrRowPtrA: *const ::std::os::raw::c_int, csrColIndA: *const ::std::os::raw::c_int, p: *mut ::std::os::raw::c_int) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverSpXcsrmetisndHost(
        handle: cusolverSpHandle_t,
        n: ::std::os::raw::c_int,
        nnzA: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrRowPtrA: *const ::std::os::raw::c_int,
        csrColIndA: *const ::std::os::raw::c_int,
        options: *const i64,
        p: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverSpScsrzfdHost(
        handle: cusolverSpHandle_t,
        n: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrValA: *const f32,
        csrRowPtrA: *const ::std::os::raw::c_int,
        csrColIndA: *const ::std::os::raw::c_int,
        P: *mut ::std::os::raw::c_int,
        numnz: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverSpDcsrzfdHost(
        handle: cusolverSpHandle_t,
        n: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrValA: *const f64,
        csrRowPtrA: *const ::std::os::raw::c_int,
        csrColIndA: *const ::std::os::raw::c_int,
        P: *mut ::std::os::raw::c_int,
        numnz: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverSpCcsrzfdHost(
        handle: cusolverSpHandle_t,
        n: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrValA: *const cuComplex,
        csrRowPtrA: *const ::std::os::raw::c_int,
        csrColIndA: *const ::std::os::raw::c_int,
        P: *mut ::std::os::raw::c_int,
        numnz: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverSpZcsrzfdHost(
        handle: cusolverSpHandle_t,
        n: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrValA: *const cuDoubleComplex,
        csrRowPtrA: *const ::std::os::raw::c_int,
        csrColIndA: *const ::std::os::raw::c_int,
        P: *mut ::std::os::raw::c_int,
        numnz: *mut ::std::os::raw::c_int,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverSpXcsrperm_bufferSizeHost(
        handle: cusolverSpHandle_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nnzA: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrRowPtrA: *const ::std::os::raw::c_int,
        csrColIndA: *const ::std::os::raw::c_int,
        p: *const ::std::os::raw::c_int,
        q: *const ::std::os::raw::c_int,
        bufferSizeInBytes: *mut usize,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverSpXcsrpermHost(
        handle: cusolverSpHandle_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nnzA: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrRowPtrA: *mut ::std::os::raw::c_int,
        csrColIndA: *mut ::std::os::raw::c_int,
        p: *const ::std::os::raw::c_int,
        q: *const ::std::os::raw::c_int,
        map: *mut ::std::os::raw::c_int,
        pBuffer: *mut ::std::os::raw::c_void,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverSpCreateCsrqrInfo(info: *mut csrqrInfo_t) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverSpDestroyCsrqrInfo(info: csrqrInfo_t) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverSpXcsrqrAnalysisBatched(
        handle: cusolverSpHandle_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nnzA: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrRowPtrA: *const ::std::os::raw::c_int,
        csrColIndA: *const ::std::os::raw::c_int,
        info: csrqrInfo_t,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverSpScsrqrBufferInfoBatched(
        handle: cusolverSpHandle_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrVal: *const f32,
        csrRowPtr: *const ::std::os::raw::c_int,
        csrColInd: *const ::std::os::raw::c_int,
        batchSize: ::std::os::raw::c_int,
        info: csrqrInfo_t,
        internalDataInBytes: *mut usize,
        workspaceInBytes: *mut usize,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverSpDcsrqrBufferInfoBatched(
        handle: cusolverSpHandle_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrVal: *const f64,
        csrRowPtr: *const ::std::os::raw::c_int,
        csrColInd: *const ::std::os::raw::c_int,
        batchSize: ::std::os::raw::c_int,
        info: csrqrInfo_t,
        internalDataInBytes: *mut usize,
        workspaceInBytes: *mut usize,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverSpCcsrqrBufferInfoBatched(
        handle: cusolverSpHandle_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrVal: *const cuComplex,
        csrRowPtr: *const ::std::os::raw::c_int,
        csrColInd: *const ::std::os::raw::c_int,
        batchSize: ::std::os::raw::c_int,
        info: csrqrInfo_t,
        internalDataInBytes: *mut usize,
        workspaceInBytes: *mut usize,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverSpZcsrqrBufferInfoBatched(
        handle: cusolverSpHandle_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrVal: *const cuDoubleComplex,
        csrRowPtr: *const ::std::os::raw::c_int,
        csrColInd: *const ::std::os::raw::c_int,
        batchSize: ::std::os::raw::c_int,
        info: csrqrInfo_t,
        internalDataInBytes: *mut usize,
        workspaceInBytes: *mut usize,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverSpScsrqrsvBatched(
        handle: cusolverSpHandle_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrValA: *const f32,
        csrRowPtrA: *const ::std::os::raw::c_int,
        csrColIndA: *const ::std::os::raw::c_int,
        b: *const f32,
        x: *mut f32,
        batchSize: ::std::os::raw::c_int,
        info: csrqrInfo_t,
        pBuffer: *mut ::std::os::raw::c_void,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverSpDcsrqrsvBatched(
        handle: cusolverSpHandle_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrValA: *const f64,
        csrRowPtrA: *const ::std::os::raw::c_int,
        csrColIndA: *const ::std::os::raw::c_int,
        b: *const f64,
        x: *mut f64,
        batchSize: ::std::os::raw::c_int,
        info: csrqrInfo_t,
        pBuffer: *mut ::std::os::raw::c_void,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverSpCcsrqrsvBatched(
        handle: cusolverSpHandle_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrValA: *const cuComplex,
        csrRowPtrA: *const ::std::os::raw::c_int,
        csrColIndA: *const ::std::os::raw::c_int,
        b: *const cuComplex,
        x: *mut cuComplex,
        batchSize: ::std::os::raw::c_int,
        info: csrqrInfo_t,
        pBuffer: *mut ::std::os::raw::c_void,
    ) -> cusolverStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn cusolverSpZcsrqrsvBatched(
        handle: cusolverSpHandle_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        descrA: cusparseMatDescr_t,
        csrValA: *const cuDoubleComplex,
        csrRowPtrA: *const ::std::os::raw::c_int,
        csrColIndA: *const ::std::os::raw::c_int,
        b: *const cuDoubleComplex,
        x: *mut cuDoubleComplex,
        batchSize: ::std::os::raw::c_int,
        info: csrqrInfo_t,
        pBuffer: *mut ::std::os::raw::c_void,
    ) -> cusolverStatus_t;
}
#[cfg(feature = "runtime-link")]
pub struct DynamicBindings {
    pub cusolverGetProperty: Option<unsafe extern "C" fn(type_: libraryPropertyType, value: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverGetVersion: Option<unsafe extern "C" fn(version: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnCreate: Option<unsafe extern "C" fn(handle: *mut cusolverDnHandle_t) -> cusolverStatus_t>,
    pub cusolverDnDestroy: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t) -> cusolverStatus_t>,
    pub cusolverDnSetStream: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, streamId: cudaStream_t) -> cusolverStatus_t>,
    pub cusolverDnGetStream: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, streamId: *mut cudaStream_t) -> cusolverStatus_t>,
    pub cusolverDnSetDeterministicMode: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, mode: cusolverDeterministicMode_t) -> cusolverStatus_t>,
    pub cusolverDnGetDeterministicMode: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, mode: *mut cusolverDeterministicMode_t) -> cusolverStatus_t>,
    pub cusolverDnSetMathMode: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, mode: cusolverMathMode_t) -> cusolverStatus_t>,
    pub cusolverDnGetMathMode: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, mode: *mut cusolverMathMode_t) -> cusolverStatus_t>,
    pub cusolverDnSetEmulationStrategy: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, strategy: cudaEmulationStrategy_t) -> cusolverStatus_t>,
    pub cusolverDnGetEmulationStrategy: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, strategy: *mut cudaEmulationStrategy_t) -> cusolverStatus_t>,
    pub cusolverDnSetFixedPointEmulationMantissaControl: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, control: cudaEmulationMantissaControl_t) -> cusolverStatus_t>,
    pub cusolverDnGetFixedPointEmulationMantissaControl: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, control: *mut cudaEmulationMantissaControl_t) -> cusolverStatus_t>,
    pub cusolverDnSetFixedPointEmulationMaxMantissaBitCount: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, mantissaBitCount: ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnGetFixedPointEmulationMaxMantissaBitCount: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, mantissaBitCount: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnSetFixedPointEmulationMantissaBitOffset: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, mantissaBitOffset: ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnGetFixedPointEmulationMantissaBitOffset: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, mantissaBitOffset: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnSetEmulationSpecialValuesSupport: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, mask: cudaEmulationSpecialValuesSupport_t) -> cusolverStatus_t>,
    pub cusolverDnGetEmulationSpecialValuesSupport: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, mask: *mut cudaEmulationSpecialValuesSupport_t) -> cusolverStatus_t>,
    pub cusolverDnIRSParamsCreate: Option<unsafe extern "C" fn(params_ptr: *mut cusolverDnIRSParams_t) -> cusolverStatus_t>,
    pub cusolverDnIRSParamsDestroy: Option<unsafe extern "C" fn(params: cusolverDnIRSParams_t) -> cusolverStatus_t>,
    pub cusolverDnIRSParamsSetRefinementSolver: Option<unsafe extern "C" fn(params: cusolverDnIRSParams_t, refinement_solver: cusolverIRSRefinement_t) -> cusolverStatus_t>,
    pub cusolverDnIRSParamsSetSolverMainPrecision: Option<unsafe extern "C" fn(params: cusolverDnIRSParams_t, solver_main_precision: cusolverPrecType_t) -> cusolverStatus_t>,
    pub cusolverDnIRSParamsSetSolverLowestPrecision: Option<unsafe extern "C" fn(params: cusolverDnIRSParams_t, solver_lowest_precision: cusolverPrecType_t) -> cusolverStatus_t>,
    pub cusolverDnIRSParamsSetSolverPrecisions: Option<unsafe extern "C" fn(params: cusolverDnIRSParams_t, solver_main_precision: cusolverPrecType_t, solver_lowest_precision: cusolverPrecType_t) -> cusolverStatus_t>,
    pub cusolverDnIRSParamsSetTol: Option<unsafe extern "C" fn(params: cusolverDnIRSParams_t, val: f64) -> cusolverStatus_t>,
    pub cusolverDnIRSParamsSetTolInner: Option<unsafe extern "C" fn(params: cusolverDnIRSParams_t, val: f64) -> cusolverStatus_t>,
    pub cusolverDnIRSParamsSetMaxIters: Option<unsafe extern "C" fn(params: cusolverDnIRSParams_t, maxiters: cusolver_int_t) -> cusolverStatus_t>,
    pub cusolverDnIRSParamsSetMaxItersInner: Option<unsafe extern "C" fn(params: cusolverDnIRSParams_t, maxiters_inner: cusolver_int_t) -> cusolverStatus_t>,
    pub cusolverDnIRSParamsGetMaxIters: Option<unsafe extern "C" fn(params: cusolverDnIRSParams_t, maxiters: *mut cusolver_int_t) -> cusolverStatus_t>,
    pub cusolverDnIRSParamsEnableFallback: Option<unsafe extern "C" fn(params: cusolverDnIRSParams_t) -> cusolverStatus_t>,
    pub cusolverDnIRSParamsDisableFallback: Option<unsafe extern "C" fn(params: cusolverDnIRSParams_t) -> cusolverStatus_t>,
    pub cusolverDnIRSInfosDestroy: Option<unsafe extern "C" fn(infos: cusolverDnIRSInfos_t) -> cusolverStatus_t>,
    pub cusolverDnIRSInfosCreate: Option<unsafe extern "C" fn(infos_ptr: *mut cusolverDnIRSInfos_t) -> cusolverStatus_t>,
    pub cusolverDnIRSInfosGetNiters: Option<unsafe extern "C" fn(infos: cusolverDnIRSInfos_t, niters: *mut cusolver_int_t) -> cusolverStatus_t>,
    pub cusolverDnIRSInfosGetOuterNiters: Option<unsafe extern "C" fn(infos: cusolverDnIRSInfos_t, outer_niters: *mut cusolver_int_t) -> cusolverStatus_t>,
    pub cusolverDnIRSInfosRequestResidual: Option<unsafe extern "C" fn(infos: cusolverDnIRSInfos_t) -> cusolverStatus_t>,
    pub cusolverDnIRSInfosGetResidualHistory: Option<unsafe extern "C" fn(infos: cusolverDnIRSInfos_t, residual_history: *mut *mut ::std::os::raw::c_void) -> cusolverStatus_t>,
    pub cusolverDnIRSInfosGetMaxIters: Option<unsafe extern "C" fn(infos: cusolverDnIRSInfos_t, maxiters: *mut cusolver_int_t) -> cusolverStatus_t>,
    pub cusolverDnZZgesv: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut cuDoubleComplex,
            ldda: cusolver_int_t,
            dipiv: *mut cusolver_int_t,
            dB: *mut cuDoubleComplex,
            lddb: cusolver_int_t,
            dX: *mut cuDoubleComplex,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: usize,
            iter: *mut cusolver_int_t,
            d_info: *mut cusolver_int_t,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnZCgesv: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut cuDoubleComplex,
            ldda: cusolver_int_t,
            dipiv: *mut cusolver_int_t,
            dB: *mut cuDoubleComplex,
            lddb: cusolver_int_t,
            dX: *mut cuDoubleComplex,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: usize,
            iter: *mut cusolver_int_t,
            d_info: *mut cusolver_int_t,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnZKgesv: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut cuDoubleComplex,
            ldda: cusolver_int_t,
            dipiv: *mut cusolver_int_t,
            dB: *mut cuDoubleComplex,
            lddb: cusolver_int_t,
            dX: *mut cuDoubleComplex,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: usize,
            iter: *mut cusolver_int_t,
            d_info: *mut cusolver_int_t,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnZEgesv: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut cuDoubleComplex,
            ldda: cusolver_int_t,
            dipiv: *mut cusolver_int_t,
            dB: *mut cuDoubleComplex,
            lddb: cusolver_int_t,
            dX: *mut cuDoubleComplex,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: usize,
            iter: *mut cusolver_int_t,
            d_info: *mut cusolver_int_t,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnZYgesv: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut cuDoubleComplex,
            ldda: cusolver_int_t,
            dipiv: *mut cusolver_int_t,
            dB: *mut cuDoubleComplex,
            lddb: cusolver_int_t,
            dX: *mut cuDoubleComplex,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: usize,
            iter: *mut cusolver_int_t,
            d_info: *mut cusolver_int_t,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnCCgesv: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut cuComplex,
            ldda: cusolver_int_t,
            dipiv: *mut cusolver_int_t,
            dB: *mut cuComplex,
            lddb: cusolver_int_t,
            dX: *mut cuComplex,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: usize,
            iter: *mut cusolver_int_t,
            d_info: *mut cusolver_int_t,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnCEgesv: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut cuComplex,
            ldda: cusolver_int_t,
            dipiv: *mut cusolver_int_t,
            dB: *mut cuComplex,
            lddb: cusolver_int_t,
            dX: *mut cuComplex,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: usize,
            iter: *mut cusolver_int_t,
            d_info: *mut cusolver_int_t,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnCKgesv: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut cuComplex,
            ldda: cusolver_int_t,
            dipiv: *mut cusolver_int_t,
            dB: *mut cuComplex,
            lddb: cusolver_int_t,
            dX: *mut cuComplex,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: usize,
            iter: *mut cusolver_int_t,
            d_info: *mut cusolver_int_t,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnCYgesv: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut cuComplex,
            ldda: cusolver_int_t,
            dipiv: *mut cusolver_int_t,
            dB: *mut cuComplex,
            lddb: cusolver_int_t,
            dX: *mut cuComplex,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: usize,
            iter: *mut cusolver_int_t,
            d_info: *mut cusolver_int_t,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnDDgesv: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut f64,
            ldda: cusolver_int_t,
            dipiv: *mut cusolver_int_t,
            dB: *mut f64,
            lddb: cusolver_int_t,
            dX: *mut f64,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: usize,
            iter: *mut cusolver_int_t,
            d_info: *mut cusolver_int_t,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnDSgesv: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut f64,
            ldda: cusolver_int_t,
            dipiv: *mut cusolver_int_t,
            dB: *mut f64,
            lddb: cusolver_int_t,
            dX: *mut f64,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: usize,
            iter: *mut cusolver_int_t,
            d_info: *mut cusolver_int_t,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnDHgesv: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut f64,
            ldda: cusolver_int_t,
            dipiv: *mut cusolver_int_t,
            dB: *mut f64,
            lddb: cusolver_int_t,
            dX: *mut f64,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: usize,
            iter: *mut cusolver_int_t,
            d_info: *mut cusolver_int_t,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnDBgesv: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut f64,
            ldda: cusolver_int_t,
            dipiv: *mut cusolver_int_t,
            dB: *mut f64,
            lddb: cusolver_int_t,
            dX: *mut f64,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: usize,
            iter: *mut cusolver_int_t,
            d_info: *mut cusolver_int_t,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnDXgesv: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut f64,
            ldda: cusolver_int_t,
            dipiv: *mut cusolver_int_t,
            dB: *mut f64,
            lddb: cusolver_int_t,
            dX: *mut f64,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: usize,
            iter: *mut cusolver_int_t,
            d_info: *mut cusolver_int_t,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnSSgesv: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut f32,
            ldda: cusolver_int_t,
            dipiv: *mut cusolver_int_t,
            dB: *mut f32,
            lddb: cusolver_int_t,
            dX: *mut f32,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: usize,
            iter: *mut cusolver_int_t,
            d_info: *mut cusolver_int_t,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnSHgesv: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut f32,
            ldda: cusolver_int_t,
            dipiv: *mut cusolver_int_t,
            dB: *mut f32,
            lddb: cusolver_int_t,
            dX: *mut f32,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: usize,
            iter: *mut cusolver_int_t,
            d_info: *mut cusolver_int_t,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnSBgesv: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut f32,
            ldda: cusolver_int_t,
            dipiv: *mut cusolver_int_t,
            dB: *mut f32,
            lddb: cusolver_int_t,
            dX: *mut f32,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: usize,
            iter: *mut cusolver_int_t,
            d_info: *mut cusolver_int_t,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnSXgesv: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut f32,
            ldda: cusolver_int_t,
            dipiv: *mut cusolver_int_t,
            dB: *mut f32,
            lddb: cusolver_int_t,
            dX: *mut f32,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: usize,
            iter: *mut cusolver_int_t,
            d_info: *mut cusolver_int_t,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnZZgesv_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut cuDoubleComplex,
            ldda: cusolver_int_t,
            dipiv: *mut cusolver_int_t,
            dB: *mut cuDoubleComplex,
            lddb: cusolver_int_t,
            dX: *mut cuDoubleComplex,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: *mut usize,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnZCgesv_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut cuDoubleComplex,
            ldda: cusolver_int_t,
            dipiv: *mut cusolver_int_t,
            dB: *mut cuDoubleComplex,
            lddb: cusolver_int_t,
            dX: *mut cuDoubleComplex,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: *mut usize,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnZKgesv_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut cuDoubleComplex,
            ldda: cusolver_int_t,
            dipiv: *mut cusolver_int_t,
            dB: *mut cuDoubleComplex,
            lddb: cusolver_int_t,
            dX: *mut cuDoubleComplex,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: *mut usize,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnZEgesv_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut cuDoubleComplex,
            ldda: cusolver_int_t,
            dipiv: *mut cusolver_int_t,
            dB: *mut cuDoubleComplex,
            lddb: cusolver_int_t,
            dX: *mut cuDoubleComplex,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: *mut usize,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnZYgesv_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut cuDoubleComplex,
            ldda: cusolver_int_t,
            dipiv: *mut cusolver_int_t,
            dB: *mut cuDoubleComplex,
            lddb: cusolver_int_t,
            dX: *mut cuDoubleComplex,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: *mut usize,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnCCgesv_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut cuComplex,
            ldda: cusolver_int_t,
            dipiv: *mut cusolver_int_t,
            dB: *mut cuComplex,
            lddb: cusolver_int_t,
            dX: *mut cuComplex,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: *mut usize,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnCKgesv_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut cuComplex,
            ldda: cusolver_int_t,
            dipiv: *mut cusolver_int_t,
            dB: *mut cuComplex,
            lddb: cusolver_int_t,
            dX: *mut cuComplex,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: *mut usize,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnCEgesv_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut cuComplex,
            ldda: cusolver_int_t,
            dipiv: *mut cusolver_int_t,
            dB: *mut cuComplex,
            lddb: cusolver_int_t,
            dX: *mut cuComplex,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: *mut usize,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnCYgesv_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut cuComplex,
            ldda: cusolver_int_t,
            dipiv: *mut cusolver_int_t,
            dB: *mut cuComplex,
            lddb: cusolver_int_t,
            dX: *mut cuComplex,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: *mut usize,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnDDgesv_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut f64,
            ldda: cusolver_int_t,
            dipiv: *mut cusolver_int_t,
            dB: *mut f64,
            lddb: cusolver_int_t,
            dX: *mut f64,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: *mut usize,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnDSgesv_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut f64,
            ldda: cusolver_int_t,
            dipiv: *mut cusolver_int_t,
            dB: *mut f64,
            lddb: cusolver_int_t,
            dX: *mut f64,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: *mut usize,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnDHgesv_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut f64,
            ldda: cusolver_int_t,
            dipiv: *mut cusolver_int_t,
            dB: *mut f64,
            lddb: cusolver_int_t,
            dX: *mut f64,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: *mut usize,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnDBgesv_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut f64,
            ldda: cusolver_int_t,
            dipiv: *mut cusolver_int_t,
            dB: *mut f64,
            lddb: cusolver_int_t,
            dX: *mut f64,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: *mut usize,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnDXgesv_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut f64,
            ldda: cusolver_int_t,
            dipiv: *mut cusolver_int_t,
            dB: *mut f64,
            lddb: cusolver_int_t,
            dX: *mut f64,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: *mut usize,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnSSgesv_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut f32,
            ldda: cusolver_int_t,
            dipiv: *mut cusolver_int_t,
            dB: *mut f32,
            lddb: cusolver_int_t,
            dX: *mut f32,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: *mut usize,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnSHgesv_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut f32,
            ldda: cusolver_int_t,
            dipiv: *mut cusolver_int_t,
            dB: *mut f32,
            lddb: cusolver_int_t,
            dX: *mut f32,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: *mut usize,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnSBgesv_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut f32,
            ldda: cusolver_int_t,
            dipiv: *mut cusolver_int_t,
            dB: *mut f32,
            lddb: cusolver_int_t,
            dX: *mut f32,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: *mut usize,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnSXgesv_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut f32,
            ldda: cusolver_int_t,
            dipiv: *mut cusolver_int_t,
            dB: *mut f32,
            lddb: cusolver_int_t,
            dX: *mut f32,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: *mut usize,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnZZgels: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            m: cusolver_int_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut cuDoubleComplex,
            ldda: cusolver_int_t,
            dB: *mut cuDoubleComplex,
            lddb: cusolver_int_t,
            dX: *mut cuDoubleComplex,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: usize,
            iter: *mut cusolver_int_t,
            d_info: *mut cusolver_int_t,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnZCgels: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            m: cusolver_int_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut cuDoubleComplex,
            ldda: cusolver_int_t,
            dB: *mut cuDoubleComplex,
            lddb: cusolver_int_t,
            dX: *mut cuDoubleComplex,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: usize,
            iter: *mut cusolver_int_t,
            d_info: *mut cusolver_int_t,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnZKgels: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            m: cusolver_int_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut cuDoubleComplex,
            ldda: cusolver_int_t,
            dB: *mut cuDoubleComplex,
            lddb: cusolver_int_t,
            dX: *mut cuDoubleComplex,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: usize,
            iter: *mut cusolver_int_t,
            d_info: *mut cusolver_int_t,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnZEgels: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            m: cusolver_int_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut cuDoubleComplex,
            ldda: cusolver_int_t,
            dB: *mut cuDoubleComplex,
            lddb: cusolver_int_t,
            dX: *mut cuDoubleComplex,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: usize,
            iter: *mut cusolver_int_t,
            d_info: *mut cusolver_int_t,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnZYgels: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            m: cusolver_int_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut cuDoubleComplex,
            ldda: cusolver_int_t,
            dB: *mut cuDoubleComplex,
            lddb: cusolver_int_t,
            dX: *mut cuDoubleComplex,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: usize,
            iter: *mut cusolver_int_t,
            d_info: *mut cusolver_int_t,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnCCgels: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            m: cusolver_int_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut cuComplex,
            ldda: cusolver_int_t,
            dB: *mut cuComplex,
            lddb: cusolver_int_t,
            dX: *mut cuComplex,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: usize,
            iter: *mut cusolver_int_t,
            d_info: *mut cusolver_int_t,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnCKgels: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            m: cusolver_int_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut cuComplex,
            ldda: cusolver_int_t,
            dB: *mut cuComplex,
            lddb: cusolver_int_t,
            dX: *mut cuComplex,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: usize,
            iter: *mut cusolver_int_t,
            d_info: *mut cusolver_int_t,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnCEgels: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            m: cusolver_int_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut cuComplex,
            ldda: cusolver_int_t,
            dB: *mut cuComplex,
            lddb: cusolver_int_t,
            dX: *mut cuComplex,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: usize,
            iter: *mut cusolver_int_t,
            d_info: *mut cusolver_int_t,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnCYgels: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            m: cusolver_int_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut cuComplex,
            ldda: cusolver_int_t,
            dB: *mut cuComplex,
            lddb: cusolver_int_t,
            dX: *mut cuComplex,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: usize,
            iter: *mut cusolver_int_t,
            d_info: *mut cusolver_int_t,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnDDgels: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            m: cusolver_int_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut f64,
            ldda: cusolver_int_t,
            dB: *mut f64,
            lddb: cusolver_int_t,
            dX: *mut f64,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: usize,
            iter: *mut cusolver_int_t,
            d_info: *mut cusolver_int_t,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnDSgels: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            m: cusolver_int_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut f64,
            ldda: cusolver_int_t,
            dB: *mut f64,
            lddb: cusolver_int_t,
            dX: *mut f64,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: usize,
            iter: *mut cusolver_int_t,
            d_info: *mut cusolver_int_t,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnDHgels: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            m: cusolver_int_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut f64,
            ldda: cusolver_int_t,
            dB: *mut f64,
            lddb: cusolver_int_t,
            dX: *mut f64,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: usize,
            iter: *mut cusolver_int_t,
            d_info: *mut cusolver_int_t,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnDBgels: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            m: cusolver_int_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut f64,
            ldda: cusolver_int_t,
            dB: *mut f64,
            lddb: cusolver_int_t,
            dX: *mut f64,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: usize,
            iter: *mut cusolver_int_t,
            d_info: *mut cusolver_int_t,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnDXgels: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            m: cusolver_int_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut f64,
            ldda: cusolver_int_t,
            dB: *mut f64,
            lddb: cusolver_int_t,
            dX: *mut f64,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: usize,
            iter: *mut cusolver_int_t,
            d_info: *mut cusolver_int_t,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnSSgels: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            m: cusolver_int_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut f32,
            ldda: cusolver_int_t,
            dB: *mut f32,
            lddb: cusolver_int_t,
            dX: *mut f32,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: usize,
            iter: *mut cusolver_int_t,
            d_info: *mut cusolver_int_t,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnSHgels: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            m: cusolver_int_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut f32,
            ldda: cusolver_int_t,
            dB: *mut f32,
            lddb: cusolver_int_t,
            dX: *mut f32,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: usize,
            iter: *mut cusolver_int_t,
            d_info: *mut cusolver_int_t,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnSBgels: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            m: cusolver_int_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut f32,
            ldda: cusolver_int_t,
            dB: *mut f32,
            lddb: cusolver_int_t,
            dX: *mut f32,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: usize,
            iter: *mut cusolver_int_t,
            d_info: *mut cusolver_int_t,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnSXgels: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            m: cusolver_int_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut f32,
            ldda: cusolver_int_t,
            dB: *mut f32,
            lddb: cusolver_int_t,
            dX: *mut f32,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: usize,
            iter: *mut cusolver_int_t,
            d_info: *mut cusolver_int_t,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnZZgels_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            m: cusolver_int_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut cuDoubleComplex,
            ldda: cusolver_int_t,
            dB: *mut cuDoubleComplex,
            lddb: cusolver_int_t,
            dX: *mut cuDoubleComplex,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: *mut usize,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnZCgels_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            m: cusolver_int_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut cuDoubleComplex,
            ldda: cusolver_int_t,
            dB: *mut cuDoubleComplex,
            lddb: cusolver_int_t,
            dX: *mut cuDoubleComplex,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: *mut usize,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnZKgels_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            m: cusolver_int_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut cuDoubleComplex,
            ldda: cusolver_int_t,
            dB: *mut cuDoubleComplex,
            lddb: cusolver_int_t,
            dX: *mut cuDoubleComplex,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: *mut usize,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnZEgels_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            m: cusolver_int_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut cuDoubleComplex,
            ldda: cusolver_int_t,
            dB: *mut cuDoubleComplex,
            lddb: cusolver_int_t,
            dX: *mut cuDoubleComplex,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: *mut usize,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnZYgels_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            m: cusolver_int_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut cuDoubleComplex,
            ldda: cusolver_int_t,
            dB: *mut cuDoubleComplex,
            lddb: cusolver_int_t,
            dX: *mut cuDoubleComplex,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: *mut usize,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnCCgels_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            m: cusolver_int_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut cuComplex,
            ldda: cusolver_int_t,
            dB: *mut cuComplex,
            lddb: cusolver_int_t,
            dX: *mut cuComplex,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: *mut usize,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnCKgels_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            m: cusolver_int_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut cuComplex,
            ldda: cusolver_int_t,
            dB: *mut cuComplex,
            lddb: cusolver_int_t,
            dX: *mut cuComplex,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: *mut usize,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnCEgels_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            m: cusolver_int_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut cuComplex,
            ldda: cusolver_int_t,
            dB: *mut cuComplex,
            lddb: cusolver_int_t,
            dX: *mut cuComplex,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: *mut usize,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnCYgels_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            m: cusolver_int_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut cuComplex,
            ldda: cusolver_int_t,
            dB: *mut cuComplex,
            lddb: cusolver_int_t,
            dX: *mut cuComplex,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: *mut usize,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnDDgels_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            m: cusolver_int_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut f64,
            ldda: cusolver_int_t,
            dB: *mut f64,
            lddb: cusolver_int_t,
            dX: *mut f64,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: *mut usize,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnDSgels_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            m: cusolver_int_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut f64,
            ldda: cusolver_int_t,
            dB: *mut f64,
            lddb: cusolver_int_t,
            dX: *mut f64,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: *mut usize,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnDHgels_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            m: cusolver_int_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut f64,
            ldda: cusolver_int_t,
            dB: *mut f64,
            lddb: cusolver_int_t,
            dX: *mut f64,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: *mut usize,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnDBgels_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            m: cusolver_int_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut f64,
            ldda: cusolver_int_t,
            dB: *mut f64,
            lddb: cusolver_int_t,
            dX: *mut f64,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: *mut usize,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnDXgels_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            m: cusolver_int_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut f64,
            ldda: cusolver_int_t,
            dB: *mut f64,
            lddb: cusolver_int_t,
            dX: *mut f64,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: *mut usize,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnSSgels_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            m: cusolver_int_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut f32,
            ldda: cusolver_int_t,
            dB: *mut f32,
            lddb: cusolver_int_t,
            dX: *mut f32,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: *mut usize,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnSHgels_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            m: cusolver_int_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut f32,
            ldda: cusolver_int_t,
            dB: *mut f32,
            lddb: cusolver_int_t,
            dX: *mut f32,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: *mut usize,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnSBgels_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            m: cusolver_int_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut f32,
            ldda: cusolver_int_t,
            dB: *mut f32,
            lddb: cusolver_int_t,
            dX: *mut f32,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: *mut usize,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnSXgels_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            m: cusolver_int_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut f32,
            ldda: cusolver_int_t,
            dB: *mut f32,
            lddb: cusolver_int_t,
            dX: *mut f32,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: *mut usize,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnIRSXgesv: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            gesv_irs_params: cusolverDnIRSParams_t,
            gesv_irs_infos: cusolverDnIRSInfos_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut ::std::os::raw::c_void,
            ldda: cusolver_int_t,
            dB: *mut ::std::os::raw::c_void,
            lddb: cusolver_int_t,
            dX: *mut ::std::os::raw::c_void,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: usize,
            niters: *mut cusolver_int_t,
            d_info: *mut cusolver_int_t,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnIRSXgesv_bufferSize: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, params: cusolverDnIRSParams_t, n: cusolver_int_t, nrhs: cusolver_int_t, lwork_bytes: *mut usize) -> cusolverStatus_t>,
    pub cusolverDnIRSXgels: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            gels_irs_params: cusolverDnIRSParams_t,
            gels_irs_infos: cusolverDnIRSInfos_t,
            m: cusolver_int_t,
            n: cusolver_int_t,
            nrhs: cusolver_int_t,
            dA: *mut ::std::os::raw::c_void,
            ldda: cusolver_int_t,
            dB: *mut ::std::os::raw::c_void,
            lddb: cusolver_int_t,
            dX: *mut ::std::os::raw::c_void,
            lddx: cusolver_int_t,
            dWorkspace: *mut ::std::os::raw::c_void,
            lwork_bytes: usize,
            niters: *mut cusolver_int_t,
            d_info: *mut cusolver_int_t,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnIRSXgels_bufferSize: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, params: cusolverDnIRSParams_t, m: cusolver_int_t, n: cusolver_int_t, nrhs: cusolver_int_t, lwork_bytes: *mut usize) -> cusolverStatus_t>,
    pub cusolverDnSpotrf_bufferSize: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f32, lda: ::std::os::raw::c_int, Lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnDpotrf_bufferSize: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f64, lda: ::std::os::raw::c_int, Lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnCpotrf_bufferSize: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut cuComplex, lda: ::std::os::raw::c_int, Lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnZpotrf_bufferSize: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut cuDoubleComplex, lda: ::std::os::raw::c_int, Lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnSpotrf: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f32, lda: ::std::os::raw::c_int, Workspace: *mut f32, Lwork: ::std::os::raw::c_int, devInfo: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnDpotrf: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f64, lda: ::std::os::raw::c_int, Workspace: *mut f64, Lwork: ::std::os::raw::c_int, devInfo: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnCpotrf: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut cuComplex, lda: ::std::os::raw::c_int, Workspace: *mut cuComplex, Lwork: ::std::os::raw::c_int, devInfo: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnZpotrf:
        Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut cuDoubleComplex, lda: ::std::os::raw::c_int, Workspace: *mut cuDoubleComplex, Lwork: ::std::os::raw::c_int, devInfo: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnSpotrs:
        Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, nrhs: ::std::os::raw::c_int, A: *const f32, lda: ::std::os::raw::c_int, B: *mut f32, ldb: ::std::os::raw::c_int, devInfo: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnDpotrs:
        Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, nrhs: ::std::os::raw::c_int, A: *const f64, lda: ::std::os::raw::c_int, B: *mut f64, ldb: ::std::os::raw::c_int, devInfo: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnCpotrs:
        Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, nrhs: ::std::os::raw::c_int, A: *const cuComplex, lda: ::std::os::raw::c_int, B: *mut cuComplex, ldb: ::std::os::raw::c_int, devInfo: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnZpotrs: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            uplo: cublasFillMode_t,
            n: ::std::os::raw::c_int,
            nrhs: ::std::os::raw::c_int,
            A: *const cuDoubleComplex,
            lda: ::std::os::raw::c_int,
            B: *mut cuDoubleComplex,
            ldb: ::std::os::raw::c_int,
            devInfo: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnSpotrfBatched: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, Aarray: *mut *mut f32, lda: ::std::os::raw::c_int, infoArray: *mut ::std::os::raw::c_int, batchSize: ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnDpotrfBatched: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, Aarray: *mut *mut f64, lda: ::std::os::raw::c_int, infoArray: *mut ::std::os::raw::c_int, batchSize: ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnCpotrfBatched: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, Aarray: *mut *mut cuComplex, lda: ::std::os::raw::c_int, infoArray: *mut ::std::os::raw::c_int, batchSize: ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnZpotrfBatched: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, Aarray: *mut *mut cuDoubleComplex, lda: ::std::os::raw::c_int, infoArray: *mut ::std::os::raw::c_int, batchSize: ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnSpotrsBatched: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            uplo: cublasFillMode_t,
            n: ::std::os::raw::c_int,
            nrhs: ::std::os::raw::c_int,
            A: *mut *mut f32,
            lda: ::std::os::raw::c_int,
            B: *mut *mut f32,
            ldb: ::std::os::raw::c_int,
            d_info: *mut ::std::os::raw::c_int,
            batchSize: ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnDpotrsBatched: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            uplo: cublasFillMode_t,
            n: ::std::os::raw::c_int,
            nrhs: ::std::os::raw::c_int,
            A: *mut *mut f64,
            lda: ::std::os::raw::c_int,
            B: *mut *mut f64,
            ldb: ::std::os::raw::c_int,
            d_info: *mut ::std::os::raw::c_int,
            batchSize: ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnCpotrsBatched: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            uplo: cublasFillMode_t,
            n: ::std::os::raw::c_int,
            nrhs: ::std::os::raw::c_int,
            A: *mut *mut cuComplex,
            lda: ::std::os::raw::c_int,
            B: *mut *mut cuComplex,
            ldb: ::std::os::raw::c_int,
            d_info: *mut ::std::os::raw::c_int,
            batchSize: ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnZpotrsBatched: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            uplo: cublasFillMode_t,
            n: ::std::os::raw::c_int,
            nrhs: ::std::os::raw::c_int,
            A: *mut *mut cuDoubleComplex,
            lda: ::std::os::raw::c_int,
            B: *mut *mut cuDoubleComplex,
            ldb: ::std::os::raw::c_int,
            d_info: *mut ::std::os::raw::c_int,
            batchSize: ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnSpotri_bufferSize: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f32, lda: ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnDpotri_bufferSize: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f64, lda: ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnCpotri_bufferSize: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut cuComplex, lda: ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnZpotri_bufferSize: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut cuDoubleComplex, lda: ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnSpotri: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f32, lda: ::std::os::raw::c_int, work: *mut f32, lwork: ::std::os::raw::c_int, devInfo: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnDpotri: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f64, lda: ::std::os::raw::c_int, work: *mut f64, lwork: ::std::os::raw::c_int, devInfo: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnCpotri: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut cuComplex, lda: ::std::os::raw::c_int, work: *mut cuComplex, lwork: ::std::os::raw::c_int, devInfo: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnZpotri:
        Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut cuDoubleComplex, lda: ::std::os::raw::c_int, work: *mut cuDoubleComplex, lwork: ::std::os::raw::c_int, devInfo: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnXtrtri_bufferSize:
        Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, diag: cublasDiagType_t, n: i64, dataTypeA: cudaDataType, A: *mut ::std::os::raw::c_void, lda: i64, workspaceInBytesOnDevice: *mut usize, workspaceInBytesOnHost: *mut usize) -> cusolverStatus_t>,
    pub cusolverDnXtrtri: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            uplo: cublasFillMode_t,
            diag: cublasDiagType_t,
            n: i64,
            dataTypeA: cudaDataType,
            A: *mut ::std::os::raw::c_void,
            lda: i64,
            bufferOnDevice: *mut ::std::os::raw::c_void,
            workspaceInBytesOnDevice: usize,
            bufferOnHost: *mut ::std::os::raw::c_void,
            workspaceInBytesOnHost: usize,
            devInfo: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnSlauum_bufferSize: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f32, lda: ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnDlauum_bufferSize: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f64, lda: ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnClauum_bufferSize: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut cuComplex, lda: ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnZlauum_bufferSize: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut cuDoubleComplex, lda: ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnSlauum: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f32, lda: ::std::os::raw::c_int, work: *mut f32, lwork: ::std::os::raw::c_int, devInfo: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnDlauum: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f64, lda: ::std::os::raw::c_int, work: *mut f64, lwork: ::std::os::raw::c_int, devInfo: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnClauum: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut cuComplex, lda: ::std::os::raw::c_int, work: *mut cuComplex, lwork: ::std::os::raw::c_int, devInfo: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnZlauum:
        Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut cuDoubleComplex, lda: ::std::os::raw::c_int, work: *mut cuDoubleComplex, lwork: ::std::os::raw::c_int, devInfo: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnSgetrf_bufferSize: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, A: *mut f32, lda: ::std::os::raw::c_int, Lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnDgetrf_bufferSize: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, A: *mut f64, lda: ::std::os::raw::c_int, Lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnCgetrf_bufferSize: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, A: *mut cuComplex, lda: ::std::os::raw::c_int, Lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnZgetrf_bufferSize: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, A: *mut cuDoubleComplex, lda: ::std::os::raw::c_int, Lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnSgetrf: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, A: *mut f32, lda: ::std::os::raw::c_int, Workspace: *mut f32, devIpiv: *mut ::std::os::raw::c_int, devInfo: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnDgetrf: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, A: *mut f64, lda: ::std::os::raw::c_int, Workspace: *mut f64, devIpiv: *mut ::std::os::raw::c_int, devInfo: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnCgetrf:
        Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, A: *mut cuComplex, lda: ::std::os::raw::c_int, Workspace: *mut cuComplex, devIpiv: *mut ::std::os::raw::c_int, devInfo: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnZgetrf:
        Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, A: *mut cuDoubleComplex, lda: ::std::os::raw::c_int, Workspace: *mut cuDoubleComplex, devIpiv: *mut ::std::os::raw::c_int, devInfo: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnSlaswp: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, n: ::std::os::raw::c_int, A: *mut f32, lda: ::std::os::raw::c_int, k1: ::std::os::raw::c_int, k2: ::std::os::raw::c_int, devIpiv: *const ::std::os::raw::c_int, incx: ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnDlaswp: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, n: ::std::os::raw::c_int, A: *mut f64, lda: ::std::os::raw::c_int, k1: ::std::os::raw::c_int, k2: ::std::os::raw::c_int, devIpiv: *const ::std::os::raw::c_int, incx: ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnClaswp: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, n: ::std::os::raw::c_int, A: *mut cuComplex, lda: ::std::os::raw::c_int, k1: ::std::os::raw::c_int, k2: ::std::os::raw::c_int, devIpiv: *const ::std::os::raw::c_int, incx: ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnZlaswp:
        Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, n: ::std::os::raw::c_int, A: *mut cuDoubleComplex, lda: ::std::os::raw::c_int, k1: ::std::os::raw::c_int, k2: ::std::os::raw::c_int, devIpiv: *const ::std::os::raw::c_int, incx: ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnSgetrs: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            trans: cublasOperation_t,
            n: ::std::os::raw::c_int,
            nrhs: ::std::os::raw::c_int,
            A: *const f32,
            lda: ::std::os::raw::c_int,
            devIpiv: *const ::std::os::raw::c_int,
            B: *mut f32,
            ldb: ::std::os::raw::c_int,
            devInfo: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnDgetrs: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            trans: cublasOperation_t,
            n: ::std::os::raw::c_int,
            nrhs: ::std::os::raw::c_int,
            A: *const f64,
            lda: ::std::os::raw::c_int,
            devIpiv: *const ::std::os::raw::c_int,
            B: *mut f64,
            ldb: ::std::os::raw::c_int,
            devInfo: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnCgetrs: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            trans: cublasOperation_t,
            n: ::std::os::raw::c_int,
            nrhs: ::std::os::raw::c_int,
            A: *const cuComplex,
            lda: ::std::os::raw::c_int,
            devIpiv: *const ::std::os::raw::c_int,
            B: *mut cuComplex,
            ldb: ::std::os::raw::c_int,
            devInfo: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnZgetrs: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            trans: cublasOperation_t,
            n: ::std::os::raw::c_int,
            nrhs: ::std::os::raw::c_int,
            A: *const cuDoubleComplex,
            lda: ::std::os::raw::c_int,
            devIpiv: *const ::std::os::raw::c_int,
            B: *mut cuDoubleComplex,
            ldb: ::std::os::raw::c_int,
            devInfo: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnSgeqrf_bufferSize: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, A: *mut f32, lda: ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnDgeqrf_bufferSize: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, A: *mut f64, lda: ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnCgeqrf_bufferSize: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, A: *mut cuComplex, lda: ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnZgeqrf_bufferSize: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, A: *mut cuDoubleComplex, lda: ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnSgeqrf: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, A: *mut f32, lda: ::std::os::raw::c_int, TAU: *mut f32, Workspace: *mut f32, Lwork: ::std::os::raw::c_int, devInfo: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnDgeqrf: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, A: *mut f64, lda: ::std::os::raw::c_int, TAU: *mut f64, Workspace: *mut f64, Lwork: ::std::os::raw::c_int, devInfo: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnCgeqrf:
        Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, A: *mut cuComplex, lda: ::std::os::raw::c_int, TAU: *mut cuComplex, Workspace: *mut cuComplex, Lwork: ::std::os::raw::c_int, devInfo: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnZgeqrf: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            A: *mut cuDoubleComplex,
            lda: ::std::os::raw::c_int,
            TAU: *mut cuDoubleComplex,
            Workspace: *mut cuDoubleComplex,
            Lwork: ::std::os::raw::c_int,
            devInfo: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnSorgqr_bufferSize: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, k: ::std::os::raw::c_int, A: *const f32, lda: ::std::os::raw::c_int, tau: *const f32, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnDorgqr_bufferSize: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, k: ::std::os::raw::c_int, A: *const f64, lda: ::std::os::raw::c_int, tau: *const f64, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnCungqr_bufferSize: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, k: ::std::os::raw::c_int, A: *const cuComplex, lda: ::std::os::raw::c_int, tau: *const cuComplex, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnZungqr_bufferSize:
        Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, k: ::std::os::raw::c_int, A: *const cuDoubleComplex, lda: ::std::os::raw::c_int, tau: *const cuDoubleComplex, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnSorgqr: Option<
        unsafe extern "C" fn(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, k: ::std::os::raw::c_int, A: *mut f32, lda: ::std::os::raw::c_int, tau: *const f32, work: *mut f32, lwork: ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int) -> cusolverStatus_t,
    >,
    pub cusolverDnDorgqr: Option<
        unsafe extern "C" fn(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, k: ::std::os::raw::c_int, A: *mut f64, lda: ::std::os::raw::c_int, tau: *const f64, work: *mut f64, lwork: ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int) -> cusolverStatus_t,
    >,
    pub cusolverDnCungqr: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            k: ::std::os::raw::c_int,
            A: *mut cuComplex,
            lda: ::std::os::raw::c_int,
            tau: *const cuComplex,
            work: *mut cuComplex,
            lwork: ::std::os::raw::c_int,
            info: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnZungqr: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            k: ::std::os::raw::c_int,
            A: *mut cuDoubleComplex,
            lda: ::std::os::raw::c_int,
            tau: *const cuDoubleComplex,
            work: *mut cuDoubleComplex,
            lwork: ::std::os::raw::c_int,
            info: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnSormqr_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            side: cublasSideMode_t,
            trans: cublasOperation_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            k: ::std::os::raw::c_int,
            A: *const f32,
            lda: ::std::os::raw::c_int,
            tau: *const f32,
            C: *const f32,
            ldc: ::std::os::raw::c_int,
            lwork: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnDormqr_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            side: cublasSideMode_t,
            trans: cublasOperation_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            k: ::std::os::raw::c_int,
            A: *const f64,
            lda: ::std::os::raw::c_int,
            tau: *const f64,
            C: *const f64,
            ldc: ::std::os::raw::c_int,
            lwork: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnCunmqr_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            side: cublasSideMode_t,
            trans: cublasOperation_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            k: ::std::os::raw::c_int,
            A: *const cuComplex,
            lda: ::std::os::raw::c_int,
            tau: *const cuComplex,
            C: *const cuComplex,
            ldc: ::std::os::raw::c_int,
            lwork: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnZunmqr_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            side: cublasSideMode_t,
            trans: cublasOperation_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            k: ::std::os::raw::c_int,
            A: *const cuDoubleComplex,
            lda: ::std::os::raw::c_int,
            tau: *const cuDoubleComplex,
            C: *const cuDoubleComplex,
            ldc: ::std::os::raw::c_int,
            lwork: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnSormqr: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            side: cublasSideMode_t,
            trans: cublasOperation_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            k: ::std::os::raw::c_int,
            A: *const f32,
            lda: ::std::os::raw::c_int,
            tau: *const f32,
            C: *mut f32,
            ldc: ::std::os::raw::c_int,
            work: *mut f32,
            lwork: ::std::os::raw::c_int,
            devInfo: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnDormqr: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            side: cublasSideMode_t,
            trans: cublasOperation_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            k: ::std::os::raw::c_int,
            A: *const f64,
            lda: ::std::os::raw::c_int,
            tau: *const f64,
            C: *mut f64,
            ldc: ::std::os::raw::c_int,
            work: *mut f64,
            lwork: ::std::os::raw::c_int,
            devInfo: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnCunmqr: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            side: cublasSideMode_t,
            trans: cublasOperation_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            k: ::std::os::raw::c_int,
            A: *const cuComplex,
            lda: ::std::os::raw::c_int,
            tau: *const cuComplex,
            C: *mut cuComplex,
            ldc: ::std::os::raw::c_int,
            work: *mut cuComplex,
            lwork: ::std::os::raw::c_int,
            devInfo: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnZunmqr: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            side: cublasSideMode_t,
            trans: cublasOperation_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            k: ::std::os::raw::c_int,
            A: *const cuDoubleComplex,
            lda: ::std::os::raw::c_int,
            tau: *const cuDoubleComplex,
            C: *mut cuDoubleComplex,
            ldc: ::std::os::raw::c_int,
            work: *mut cuDoubleComplex,
            lwork: ::std::os::raw::c_int,
            devInfo: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnSsytrf_bufferSize: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, n: ::std::os::raw::c_int, A: *mut f32, lda: ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnDsytrf_bufferSize: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, n: ::std::os::raw::c_int, A: *mut f64, lda: ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnCsytrf_bufferSize: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, n: ::std::os::raw::c_int, A: *mut cuComplex, lda: ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnZsytrf_bufferSize: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, n: ::std::os::raw::c_int, A: *mut cuDoubleComplex, lda: ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnSsytrf:
        Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f32, lda: ::std::os::raw::c_int, ipiv: *mut ::std::os::raw::c_int, work: *mut f32, lwork: ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnDsytrf:
        Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f64, lda: ::std::os::raw::c_int, ipiv: *mut ::std::os::raw::c_int, work: *mut f64, lwork: ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnCsytrf: Option<
        unsafe extern "C" fn(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut cuComplex, lda: ::std::os::raw::c_int, ipiv: *mut ::std::os::raw::c_int, work: *mut cuComplex, lwork: ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int) -> cusolverStatus_t,
    >,
    pub cusolverDnZsytrf: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            uplo: cublasFillMode_t,
            n: ::std::os::raw::c_int,
            A: *mut cuDoubleComplex,
            lda: ::std::os::raw::c_int,
            ipiv: *mut ::std::os::raw::c_int,
            work: *mut cuDoubleComplex,
            lwork: ::std::os::raw::c_int,
            info: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnXsytrs_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            uplo: cublasFillMode_t,
            n: i64,
            nrhs: i64,
            dataTypeA: cudaDataType,
            A: *const ::std::os::raw::c_void,
            lda: i64,
            ipiv: *const i64,
            dataTypeB: cudaDataType,
            B: *mut ::std::os::raw::c_void,
            ldb: i64,
            workspaceInBytesOnDevice: *mut usize,
            workspaceInBytesOnHost: *mut usize,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnXsytrs: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            uplo: cublasFillMode_t,
            n: i64,
            nrhs: i64,
            dataTypeA: cudaDataType,
            A: *const ::std::os::raw::c_void,
            lda: i64,
            ipiv: *const i64,
            dataTypeB: cudaDataType,
            B: *mut ::std::os::raw::c_void,
            ldb: i64,
            bufferOnDevice: *mut ::std::os::raw::c_void,
            workspaceInBytesOnDevice: usize,
            bufferOnHost: *mut ::std::os::raw::c_void,
            workspaceInBytesOnHost: usize,
            info: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnSsytri_bufferSize: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f32, lda: ::std::os::raw::c_int, ipiv: *const ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnDsytri_bufferSize: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f64, lda: ::std::os::raw::c_int, ipiv: *const ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnCsytri_bufferSize: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut cuComplex, lda: ::std::os::raw::c_int, ipiv: *const ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnZsytri_bufferSize: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut cuDoubleComplex, lda: ::std::os::raw::c_int, ipiv: *const ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnSsytri:
        Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f32, lda: ::std::os::raw::c_int, ipiv: *const ::std::os::raw::c_int, work: *mut f32, lwork: ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnDsytri:
        Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f64, lda: ::std::os::raw::c_int, ipiv: *const ::std::os::raw::c_int, work: *mut f64, lwork: ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnCsytri: Option<
        unsafe extern "C" fn(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut cuComplex, lda: ::std::os::raw::c_int, ipiv: *const ::std::os::raw::c_int, work: *mut cuComplex, lwork: ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int) -> cusolverStatus_t,
    >,
    pub cusolverDnZsytri: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            uplo: cublasFillMode_t,
            n: ::std::os::raw::c_int,
            A: *mut cuDoubleComplex,
            lda: ::std::os::raw::c_int,
            ipiv: *const ::std::os::raw::c_int,
            work: *mut cuDoubleComplex,
            lwork: ::std::os::raw::c_int,
            info: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnSgebrd_bufferSize: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, Lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnDgebrd_bufferSize: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, Lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnCgebrd_bufferSize: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, Lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnZgebrd_bufferSize: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, Lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnSgebrd: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            A: *mut f32,
            lda: ::std::os::raw::c_int,
            D: *mut f32,
            E: *mut f32,
            TAUQ: *mut f32,
            TAUP: *mut f32,
            Work: *mut f32,
            Lwork: ::std::os::raw::c_int,
            devInfo: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnDgebrd: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            A: *mut f64,
            lda: ::std::os::raw::c_int,
            D: *mut f64,
            E: *mut f64,
            TAUQ: *mut f64,
            TAUP: *mut f64,
            Work: *mut f64,
            Lwork: ::std::os::raw::c_int,
            devInfo: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnCgebrd: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            A: *mut cuComplex,
            lda: ::std::os::raw::c_int,
            D: *mut f32,
            E: *mut f32,
            TAUQ: *mut cuComplex,
            TAUP: *mut cuComplex,
            Work: *mut cuComplex,
            Lwork: ::std::os::raw::c_int,
            devInfo: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnZgebrd: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            A: *mut cuDoubleComplex,
            lda: ::std::os::raw::c_int,
            D: *mut f64,
            E: *mut f64,
            TAUQ: *mut cuDoubleComplex,
            TAUP: *mut cuDoubleComplex,
            Work: *mut cuDoubleComplex,
            Lwork: ::std::os::raw::c_int,
            devInfo: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnSorgbr_bufferSize:
        Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, side: cublasSideMode_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, k: ::std::os::raw::c_int, A: *const f32, lda: ::std::os::raw::c_int, tau: *const f32, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnDorgbr_bufferSize:
        Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, side: cublasSideMode_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, k: ::std::os::raw::c_int, A: *const f64, lda: ::std::os::raw::c_int, tau: *const f64, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnCungbr_bufferSize:
        Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, side: cublasSideMode_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, k: ::std::os::raw::c_int, A: *const cuComplex, lda: ::std::os::raw::c_int, tau: *const cuComplex, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnZungbr_bufferSize: Option<
        unsafe extern "C" fn(handle: cusolverDnHandle_t, side: cublasSideMode_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, k: ::std::os::raw::c_int, A: *const cuDoubleComplex, lda: ::std::os::raw::c_int, tau: *const cuDoubleComplex, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t,
    >,
    pub cusolverDnSorgbr: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            side: cublasSideMode_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            k: ::std::os::raw::c_int,
            A: *mut f32,
            lda: ::std::os::raw::c_int,
            tau: *const f32,
            work: *mut f32,
            lwork: ::std::os::raw::c_int,
            info: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnDorgbr: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            side: cublasSideMode_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            k: ::std::os::raw::c_int,
            A: *mut f64,
            lda: ::std::os::raw::c_int,
            tau: *const f64,
            work: *mut f64,
            lwork: ::std::os::raw::c_int,
            info: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnCungbr: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            side: cublasSideMode_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            k: ::std::os::raw::c_int,
            A: *mut cuComplex,
            lda: ::std::os::raw::c_int,
            tau: *const cuComplex,
            work: *mut cuComplex,
            lwork: ::std::os::raw::c_int,
            info: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnZungbr: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            side: cublasSideMode_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            k: ::std::os::raw::c_int,
            A: *mut cuDoubleComplex,
            lda: ::std::os::raw::c_int,
            tau: *const cuDoubleComplex,
            work: *mut cuDoubleComplex,
            lwork: ::std::os::raw::c_int,
            info: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnSsytrd_bufferSize: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *const f32, lda: ::std::os::raw::c_int, d: *const f32, e: *const f32, tau: *const f32, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnDsytrd_bufferSize: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *const f64, lda: ::std::os::raw::c_int, d: *const f64, e: *const f64, tau: *const f64, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnChetrd_bufferSize:
        Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *const cuComplex, lda: ::std::os::raw::c_int, d: *const f32, e: *const f32, tau: *const cuComplex, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnZhetrd_bufferSize:
        Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *const cuDoubleComplex, lda: ::std::os::raw::c_int, d: *const f64, e: *const f64, tau: *const cuDoubleComplex, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnSsytrd:
        Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f32, lda: ::std::os::raw::c_int, d: *mut f32, e: *mut f32, tau: *mut f32, work: *mut f32, lwork: ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnDsytrd:
        Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f64, lda: ::std::os::raw::c_int, d: *mut f64, e: *mut f64, tau: *mut f64, work: *mut f64, lwork: ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnChetrd: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            uplo: cublasFillMode_t,
            n: ::std::os::raw::c_int,
            A: *mut cuComplex,
            lda: ::std::os::raw::c_int,
            d: *mut f32,
            e: *mut f32,
            tau: *mut cuComplex,
            work: *mut cuComplex,
            lwork: ::std::os::raw::c_int,
            info: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnZhetrd: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            uplo: cublasFillMode_t,
            n: ::std::os::raw::c_int,
            A: *mut cuDoubleComplex,
            lda: ::std::os::raw::c_int,
            d: *mut f64,
            e: *mut f64,
            tau: *mut cuDoubleComplex,
            work: *mut cuDoubleComplex,
            lwork: ::std::os::raw::c_int,
            info: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnSorgtr_bufferSize: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *const f32, lda: ::std::os::raw::c_int, tau: *const f32, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnDorgtr_bufferSize: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *const f64, lda: ::std::os::raw::c_int, tau: *const f64, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnCungtr_bufferSize: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *const cuComplex, lda: ::std::os::raw::c_int, tau: *const cuComplex, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnZungtr_bufferSize: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *const cuDoubleComplex, lda: ::std::os::raw::c_int, tau: *const cuDoubleComplex, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnSorgtr: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f32, lda: ::std::os::raw::c_int, tau: *const f32, work: *mut f32, lwork: ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnDorgtr: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f64, lda: ::std::os::raw::c_int, tau: *const f64, work: *mut f64, lwork: ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnCungtr:
        Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut cuComplex, lda: ::std::os::raw::c_int, tau: *const cuComplex, work: *mut cuComplex, lwork: ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnZungtr: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            uplo: cublasFillMode_t,
            n: ::std::os::raw::c_int,
            A: *mut cuDoubleComplex,
            lda: ::std::os::raw::c_int,
            tau: *const cuDoubleComplex,
            work: *mut cuDoubleComplex,
            lwork: ::std::os::raw::c_int,
            info: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnSormtr_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            side: cublasSideMode_t,
            uplo: cublasFillMode_t,
            trans: cublasOperation_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            A: *const f32,
            lda: ::std::os::raw::c_int,
            tau: *const f32,
            C: *const f32,
            ldc: ::std::os::raw::c_int,
            lwork: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnDormtr_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            side: cublasSideMode_t,
            uplo: cublasFillMode_t,
            trans: cublasOperation_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            A: *const f64,
            lda: ::std::os::raw::c_int,
            tau: *const f64,
            C: *const f64,
            ldc: ::std::os::raw::c_int,
            lwork: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnCunmtr_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            side: cublasSideMode_t,
            uplo: cublasFillMode_t,
            trans: cublasOperation_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            A: *const cuComplex,
            lda: ::std::os::raw::c_int,
            tau: *const cuComplex,
            C: *const cuComplex,
            ldc: ::std::os::raw::c_int,
            lwork: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnZunmtr_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            side: cublasSideMode_t,
            uplo: cublasFillMode_t,
            trans: cublasOperation_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            A: *const cuDoubleComplex,
            lda: ::std::os::raw::c_int,
            tau: *const cuDoubleComplex,
            C: *const cuDoubleComplex,
            ldc: ::std::os::raw::c_int,
            lwork: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnSormtr: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            side: cublasSideMode_t,
            uplo: cublasFillMode_t,
            trans: cublasOperation_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            A: *mut f32,
            lda: ::std::os::raw::c_int,
            tau: *mut f32,
            C: *mut f32,
            ldc: ::std::os::raw::c_int,
            work: *mut f32,
            lwork: ::std::os::raw::c_int,
            info: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnDormtr: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            side: cublasSideMode_t,
            uplo: cublasFillMode_t,
            trans: cublasOperation_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            A: *mut f64,
            lda: ::std::os::raw::c_int,
            tau: *mut f64,
            C: *mut f64,
            ldc: ::std::os::raw::c_int,
            work: *mut f64,
            lwork: ::std::os::raw::c_int,
            info: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnCunmtr: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            side: cublasSideMode_t,
            uplo: cublasFillMode_t,
            trans: cublasOperation_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            A: *mut cuComplex,
            lda: ::std::os::raw::c_int,
            tau: *mut cuComplex,
            C: *mut cuComplex,
            ldc: ::std::os::raw::c_int,
            work: *mut cuComplex,
            lwork: ::std::os::raw::c_int,
            info: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnZunmtr: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            side: cublasSideMode_t,
            uplo: cublasFillMode_t,
            trans: cublasOperation_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            A: *mut cuDoubleComplex,
            lda: ::std::os::raw::c_int,
            tau: *mut cuDoubleComplex,
            C: *mut cuDoubleComplex,
            ldc: ::std::os::raw::c_int,
            work: *mut cuDoubleComplex,
            lwork: ::std::os::raw::c_int,
            info: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnSgesvd_bufferSize: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnDgesvd_bufferSize: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnCgesvd_bufferSize: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnZgesvd_bufferSize: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnSgesvd: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            jobu: ::std::os::raw::c_schar,
            jobvt: ::std::os::raw::c_schar,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            A: *mut f32,
            lda: ::std::os::raw::c_int,
            S: *mut f32,
            U: *mut f32,
            ldu: ::std::os::raw::c_int,
            VT: *mut f32,
            ldvt: ::std::os::raw::c_int,
            work: *mut f32,
            lwork: ::std::os::raw::c_int,
            rwork: *mut f32,
            info: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnDgesvd: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            jobu: ::std::os::raw::c_schar,
            jobvt: ::std::os::raw::c_schar,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            A: *mut f64,
            lda: ::std::os::raw::c_int,
            S: *mut f64,
            U: *mut f64,
            ldu: ::std::os::raw::c_int,
            VT: *mut f64,
            ldvt: ::std::os::raw::c_int,
            work: *mut f64,
            lwork: ::std::os::raw::c_int,
            rwork: *mut f64,
            info: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnCgesvd: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            jobu: ::std::os::raw::c_schar,
            jobvt: ::std::os::raw::c_schar,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            A: *mut cuComplex,
            lda: ::std::os::raw::c_int,
            S: *mut f32,
            U: *mut cuComplex,
            ldu: ::std::os::raw::c_int,
            VT: *mut cuComplex,
            ldvt: ::std::os::raw::c_int,
            work: *mut cuComplex,
            lwork: ::std::os::raw::c_int,
            rwork: *mut f32,
            info: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnZgesvd: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            jobu: ::std::os::raw::c_schar,
            jobvt: ::std::os::raw::c_schar,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            A: *mut cuDoubleComplex,
            lda: ::std::os::raw::c_int,
            S: *mut f64,
            U: *mut cuDoubleComplex,
            ldu: ::std::os::raw::c_int,
            VT: *mut cuDoubleComplex,
            ldvt: ::std::os::raw::c_int,
            work: *mut cuDoubleComplex,
            lwork: ::std::os::raw::c_int,
            rwork: *mut f64,
            info: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnSsyevd_bufferSize: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, jobz: cusolverEigMode_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *const f32, lda: ::std::os::raw::c_int, W: *const f32, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnDsyevd_bufferSize: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, jobz: cusolverEigMode_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *const f64, lda: ::std::os::raw::c_int, W: *const f64, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnCheevd_bufferSize: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, jobz: cusolverEigMode_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *const cuComplex, lda: ::std::os::raw::c_int, W: *const f32, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnZheevd_bufferSize: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, jobz: cusolverEigMode_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *const cuDoubleComplex, lda: ::std::os::raw::c_int, W: *const f64, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnSsyevd:
        Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, jobz: cusolverEigMode_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f32, lda: ::std::os::raw::c_int, W: *mut f32, work: *mut f32, lwork: ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnDsyevd:
        Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, jobz: cusolverEigMode_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f64, lda: ::std::os::raw::c_int, W: *mut f64, work: *mut f64, lwork: ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnCheevd: Option<
        unsafe extern "C" fn(handle: cusolverDnHandle_t, jobz: cusolverEigMode_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut cuComplex, lda: ::std::os::raw::c_int, W: *mut f32, work: *mut cuComplex, lwork: ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int) -> cusolverStatus_t,
    >,
    pub cusolverDnZheevd: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            jobz: cusolverEigMode_t,
            uplo: cublasFillMode_t,
            n: ::std::os::raw::c_int,
            A: *mut cuDoubleComplex,
            lda: ::std::os::raw::c_int,
            W: *mut f64,
            work: *mut cuDoubleComplex,
            lwork: ::std::os::raw::c_int,
            info: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnSsyevdx_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            jobz: cusolverEigMode_t,
            range: cusolverEigRange_t,
            uplo: cublasFillMode_t,
            n: ::std::os::raw::c_int,
            A: *const f32,
            lda: ::std::os::raw::c_int,
            vl: f32,
            vu: f32,
            il: ::std::os::raw::c_int,
            iu: ::std::os::raw::c_int,
            meig: *mut ::std::os::raw::c_int,
            W: *const f32,
            lwork: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnDsyevdx_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            jobz: cusolverEigMode_t,
            range: cusolverEigRange_t,
            uplo: cublasFillMode_t,
            n: ::std::os::raw::c_int,
            A: *const f64,
            lda: ::std::os::raw::c_int,
            vl: f64,
            vu: f64,
            il: ::std::os::raw::c_int,
            iu: ::std::os::raw::c_int,
            meig: *mut ::std::os::raw::c_int,
            W: *const f64,
            lwork: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnCheevdx_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            jobz: cusolverEigMode_t,
            range: cusolverEigRange_t,
            uplo: cublasFillMode_t,
            n: ::std::os::raw::c_int,
            A: *const cuComplex,
            lda: ::std::os::raw::c_int,
            vl: f32,
            vu: f32,
            il: ::std::os::raw::c_int,
            iu: ::std::os::raw::c_int,
            meig: *mut ::std::os::raw::c_int,
            W: *const f32,
            lwork: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnZheevdx_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            jobz: cusolverEigMode_t,
            range: cusolverEigRange_t,
            uplo: cublasFillMode_t,
            n: ::std::os::raw::c_int,
            A: *const cuDoubleComplex,
            lda: ::std::os::raw::c_int,
            vl: f64,
            vu: f64,
            il: ::std::os::raw::c_int,
            iu: ::std::os::raw::c_int,
            meig: *mut ::std::os::raw::c_int,
            W: *const f64,
            lwork: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnSsyevdx: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            jobz: cusolverEigMode_t,
            range: cusolverEigRange_t,
            uplo: cublasFillMode_t,
            n: ::std::os::raw::c_int,
            A: *mut f32,
            lda: ::std::os::raw::c_int,
            vl: f32,
            vu: f32,
            il: ::std::os::raw::c_int,
            iu: ::std::os::raw::c_int,
            meig: *mut ::std::os::raw::c_int,
            W: *mut f32,
            work: *mut f32,
            lwork: ::std::os::raw::c_int,
            info: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnDsyevdx: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            jobz: cusolverEigMode_t,
            range: cusolverEigRange_t,
            uplo: cublasFillMode_t,
            n: ::std::os::raw::c_int,
            A: *mut f64,
            lda: ::std::os::raw::c_int,
            vl: f64,
            vu: f64,
            il: ::std::os::raw::c_int,
            iu: ::std::os::raw::c_int,
            meig: *mut ::std::os::raw::c_int,
            W: *mut f64,
            work: *mut f64,
            lwork: ::std::os::raw::c_int,
            info: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnCheevdx: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            jobz: cusolverEigMode_t,
            range: cusolverEigRange_t,
            uplo: cublasFillMode_t,
            n: ::std::os::raw::c_int,
            A: *mut cuComplex,
            lda: ::std::os::raw::c_int,
            vl: f32,
            vu: f32,
            il: ::std::os::raw::c_int,
            iu: ::std::os::raw::c_int,
            meig: *mut ::std::os::raw::c_int,
            W: *mut f32,
            work: *mut cuComplex,
            lwork: ::std::os::raw::c_int,
            info: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnZheevdx: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            jobz: cusolverEigMode_t,
            range: cusolverEigRange_t,
            uplo: cublasFillMode_t,
            n: ::std::os::raw::c_int,
            A: *mut cuDoubleComplex,
            lda: ::std::os::raw::c_int,
            vl: f64,
            vu: f64,
            il: ::std::os::raw::c_int,
            iu: ::std::os::raw::c_int,
            meig: *mut ::std::os::raw::c_int,
            W: *mut f64,
            work: *mut cuDoubleComplex,
            lwork: ::std::os::raw::c_int,
            info: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnSsygvdx_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            itype: cusolverEigType_t,
            jobz: cusolverEigMode_t,
            range: cusolverEigRange_t,
            uplo: cublasFillMode_t,
            n: ::std::os::raw::c_int,
            A: *const f32,
            lda: ::std::os::raw::c_int,
            B: *const f32,
            ldb: ::std::os::raw::c_int,
            vl: f32,
            vu: f32,
            il: ::std::os::raw::c_int,
            iu: ::std::os::raw::c_int,
            meig: *mut ::std::os::raw::c_int,
            W: *const f32,
            lwork: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnDsygvdx_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            itype: cusolverEigType_t,
            jobz: cusolverEigMode_t,
            range: cusolverEigRange_t,
            uplo: cublasFillMode_t,
            n: ::std::os::raw::c_int,
            A: *const f64,
            lda: ::std::os::raw::c_int,
            B: *const f64,
            ldb: ::std::os::raw::c_int,
            vl: f64,
            vu: f64,
            il: ::std::os::raw::c_int,
            iu: ::std::os::raw::c_int,
            meig: *mut ::std::os::raw::c_int,
            W: *const f64,
            lwork: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnChegvdx_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            itype: cusolverEigType_t,
            jobz: cusolverEigMode_t,
            range: cusolverEigRange_t,
            uplo: cublasFillMode_t,
            n: ::std::os::raw::c_int,
            A: *const cuComplex,
            lda: ::std::os::raw::c_int,
            B: *const cuComplex,
            ldb: ::std::os::raw::c_int,
            vl: f32,
            vu: f32,
            il: ::std::os::raw::c_int,
            iu: ::std::os::raw::c_int,
            meig: *mut ::std::os::raw::c_int,
            W: *const f32,
            lwork: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnZhegvdx_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            itype: cusolverEigType_t,
            jobz: cusolverEigMode_t,
            range: cusolverEigRange_t,
            uplo: cublasFillMode_t,
            n: ::std::os::raw::c_int,
            A: *const cuDoubleComplex,
            lda: ::std::os::raw::c_int,
            B: *const cuDoubleComplex,
            ldb: ::std::os::raw::c_int,
            vl: f64,
            vu: f64,
            il: ::std::os::raw::c_int,
            iu: ::std::os::raw::c_int,
            meig: *mut ::std::os::raw::c_int,
            W: *const f64,
            lwork: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnSsygvdx: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            itype: cusolverEigType_t,
            jobz: cusolverEigMode_t,
            range: cusolverEigRange_t,
            uplo: cublasFillMode_t,
            n: ::std::os::raw::c_int,
            A: *mut f32,
            lda: ::std::os::raw::c_int,
            B: *mut f32,
            ldb: ::std::os::raw::c_int,
            vl: f32,
            vu: f32,
            il: ::std::os::raw::c_int,
            iu: ::std::os::raw::c_int,
            meig: *mut ::std::os::raw::c_int,
            W: *mut f32,
            work: *mut f32,
            lwork: ::std::os::raw::c_int,
            info: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnDsygvdx: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            itype: cusolverEigType_t,
            jobz: cusolverEigMode_t,
            range: cusolverEigRange_t,
            uplo: cublasFillMode_t,
            n: ::std::os::raw::c_int,
            A: *mut f64,
            lda: ::std::os::raw::c_int,
            B: *mut f64,
            ldb: ::std::os::raw::c_int,
            vl: f64,
            vu: f64,
            il: ::std::os::raw::c_int,
            iu: ::std::os::raw::c_int,
            meig: *mut ::std::os::raw::c_int,
            W: *mut f64,
            work: *mut f64,
            lwork: ::std::os::raw::c_int,
            info: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnChegvdx: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            itype: cusolverEigType_t,
            jobz: cusolverEigMode_t,
            range: cusolverEigRange_t,
            uplo: cublasFillMode_t,
            n: ::std::os::raw::c_int,
            A: *mut cuComplex,
            lda: ::std::os::raw::c_int,
            B: *mut cuComplex,
            ldb: ::std::os::raw::c_int,
            vl: f32,
            vu: f32,
            il: ::std::os::raw::c_int,
            iu: ::std::os::raw::c_int,
            meig: *mut ::std::os::raw::c_int,
            W: *mut f32,
            work: *mut cuComplex,
            lwork: ::std::os::raw::c_int,
            info: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnZhegvdx: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            itype: cusolverEigType_t,
            jobz: cusolverEigMode_t,
            range: cusolverEigRange_t,
            uplo: cublasFillMode_t,
            n: ::std::os::raw::c_int,
            A: *mut cuDoubleComplex,
            lda: ::std::os::raw::c_int,
            B: *mut cuDoubleComplex,
            ldb: ::std::os::raw::c_int,
            vl: f64,
            vu: f64,
            il: ::std::os::raw::c_int,
            iu: ::std::os::raw::c_int,
            meig: *mut ::std::os::raw::c_int,
            W: *mut f64,
            work: *mut cuDoubleComplex,
            lwork: ::std::os::raw::c_int,
            info: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnSsygvd_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            itype: cusolverEigType_t,
            jobz: cusolverEigMode_t,
            uplo: cublasFillMode_t,
            n: ::std::os::raw::c_int,
            A: *const f32,
            lda: ::std::os::raw::c_int,
            B: *const f32,
            ldb: ::std::os::raw::c_int,
            W: *const f32,
            lwork: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnDsygvd_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            itype: cusolverEigType_t,
            jobz: cusolverEigMode_t,
            uplo: cublasFillMode_t,
            n: ::std::os::raw::c_int,
            A: *const f64,
            lda: ::std::os::raw::c_int,
            B: *const f64,
            ldb: ::std::os::raw::c_int,
            W: *const f64,
            lwork: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnChegvd_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            itype: cusolverEigType_t,
            jobz: cusolverEigMode_t,
            uplo: cublasFillMode_t,
            n: ::std::os::raw::c_int,
            A: *const cuComplex,
            lda: ::std::os::raw::c_int,
            B: *const cuComplex,
            ldb: ::std::os::raw::c_int,
            W: *const f32,
            lwork: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnZhegvd_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            itype: cusolverEigType_t,
            jobz: cusolverEigMode_t,
            uplo: cublasFillMode_t,
            n: ::std::os::raw::c_int,
            A: *const cuDoubleComplex,
            lda: ::std::os::raw::c_int,
            B: *const cuDoubleComplex,
            ldb: ::std::os::raw::c_int,
            W: *const f64,
            lwork: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnSsygvd: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            itype: cusolverEigType_t,
            jobz: cusolverEigMode_t,
            uplo: cublasFillMode_t,
            n: ::std::os::raw::c_int,
            A: *mut f32,
            lda: ::std::os::raw::c_int,
            B: *mut f32,
            ldb: ::std::os::raw::c_int,
            W: *mut f32,
            work: *mut f32,
            lwork: ::std::os::raw::c_int,
            info: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnDsygvd: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            itype: cusolverEigType_t,
            jobz: cusolverEigMode_t,
            uplo: cublasFillMode_t,
            n: ::std::os::raw::c_int,
            A: *mut f64,
            lda: ::std::os::raw::c_int,
            B: *mut f64,
            ldb: ::std::os::raw::c_int,
            W: *mut f64,
            work: *mut f64,
            lwork: ::std::os::raw::c_int,
            info: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnChegvd: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            itype: cusolverEigType_t,
            jobz: cusolverEigMode_t,
            uplo: cublasFillMode_t,
            n: ::std::os::raw::c_int,
            A: *mut cuComplex,
            lda: ::std::os::raw::c_int,
            B: *mut cuComplex,
            ldb: ::std::os::raw::c_int,
            W: *mut f32,
            work: *mut cuComplex,
            lwork: ::std::os::raw::c_int,
            info: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnZhegvd: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            itype: cusolverEigType_t,
            jobz: cusolverEigMode_t,
            uplo: cublasFillMode_t,
            n: ::std::os::raw::c_int,
            A: *mut cuDoubleComplex,
            lda: ::std::os::raw::c_int,
            B: *mut cuDoubleComplex,
            ldb: ::std::os::raw::c_int,
            W: *mut f64,
            work: *mut cuDoubleComplex,
            lwork: ::std::os::raw::c_int,
            info: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnXsygvd_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            params: cusolverDnParams_t,
            itype: cusolverEigType_t,
            jobz: cusolverEigMode_t,
            uplo: cublasFillMode_t,
            n: i64,
            dataTypeA: cudaDataType,
            d_A: *const ::std::os::raw::c_void,
            lda: i64,
            dataTypeB: cudaDataType,
            d_B: *const ::std::os::raw::c_void,
            ldb: i64,
            dataTypeW: cudaDataType,
            d_W: *const ::std::os::raw::c_void,
            computeType: cudaDataType,
            workspaceInBytesOnDevice: *mut usize,
            workspaceInBytesOnHost: *mut usize,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnXsygvd: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            params: cusolverDnParams_t,
            itype: cusolverEigType_t,
            jobz: cusolverEigMode_t,
            uplo: cublasFillMode_t,
            n: i64,
            dataTypeA: cudaDataType,
            d_A: *mut ::std::os::raw::c_void,
            lda: i64,
            dataTypeB: cudaDataType,
            d_B: *mut ::std::os::raw::c_void,
            ldb: i64,
            dataTypeW: cudaDataType,
            d_W: *mut ::std::os::raw::c_void,
            computeType: cudaDataType,
            bufferOnDevice: *mut ::std::os::raw::c_void,
            workspaceInBytesOnDevice: usize,
            bufferOnHost: *mut ::std::os::raw::c_void,
            workspaceInBytesOnHost: usize,
            d_info: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnXsygvdx_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            params: cusolverDnParams_t,
            itype: cusolverEigType_t,
            jobz: cusolverEigMode_t,
            uplo: cublasFillMode_t,
            n: i64,
            dataTypeA: cudaDataType,
            d_A: *const ::std::os::raw::c_void,
            lda: i64,
            dataTypeB: cudaDataType,
            d_B: *const ::std::os::raw::c_void,
            ldb: i64,
            vl: *mut ::std::os::raw::c_void,
            vu: *mut ::std::os::raw::c_void,
            il: i64,
            iu: i64,
            meig: *mut i64,
            dataTypeW: cudaDataType,
            d_W: *const ::std::os::raw::c_void,
            computeType: cudaDataType,
            workspaceInBytesOnDevice: *mut usize,
            workspaceInBytesOnHost: *mut usize,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnXsygvdx: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            params: cusolverDnParams_t,
            itype: cusolverEigType_t,
            jobz: cusolverEigMode_t,
            range: cusolverEigRange_t,
            uplo: cublasFillMode_t,
            n: i64,
            dataTypeA: cudaDataType,
            d_A: *mut ::std::os::raw::c_void,
            lda: i64,
            dataTypeB: cudaDataType,
            d_B: *mut ::std::os::raw::c_void,
            ldb: i64,
            vl: *mut ::std::os::raw::c_void,
            vu: *mut ::std::os::raw::c_void,
            il: i64,
            iu: i64,
            meig: *mut i64,
            dataTypeW: cudaDataType,
            d_W: *mut ::std::os::raw::c_void,
            computeType: cudaDataType,
            bufferOnDevice: *mut ::std::os::raw::c_void,
            workspaceInBytesOnDevice: usize,
            bufferOnHost: *mut ::std::os::raw::c_void,
            workspaceInBytesOnHost: usize,
            d_info: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnCreateSyevjInfo: Option<unsafe extern "C" fn(info: *mut syevjInfo_t) -> cusolverStatus_t>,
    pub cusolverDnDestroySyevjInfo: Option<unsafe extern "C" fn(info: syevjInfo_t) -> cusolverStatus_t>,
    pub cusolverDnXsyevjSetTolerance: Option<unsafe extern "C" fn(info: syevjInfo_t, tolerance: f64) -> cusolverStatus_t>,
    pub cusolverDnXsyevjSetMaxSweeps: Option<unsafe extern "C" fn(info: syevjInfo_t, max_sweeps: ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnXsyevjSetSortEig: Option<unsafe extern "C" fn(info: syevjInfo_t, sort_eig: ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnXsyevjGetResidual: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, info: syevjInfo_t, residual: *mut f64) -> cusolverStatus_t>,
    pub cusolverDnXsyevjGetSweeps: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, info: syevjInfo_t, executed_sweeps: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnSsyevjBatched_bufferSize: Option<
        unsafe extern "C" fn(handle: cusolverDnHandle_t, jobz: cusolverEigMode_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *const f32, lda: ::std::os::raw::c_int, W: *const f32, lwork: *mut ::std::os::raw::c_int, params: syevjInfo_t, batchSize: ::std::os::raw::c_int) -> cusolverStatus_t,
    >,
    pub cusolverDnDsyevjBatched_bufferSize: Option<
        unsafe extern "C" fn(handle: cusolverDnHandle_t, jobz: cusolverEigMode_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *const f64, lda: ::std::os::raw::c_int, W: *const f64, lwork: *mut ::std::os::raw::c_int, params: syevjInfo_t, batchSize: ::std::os::raw::c_int) -> cusolverStatus_t,
    >,
    pub cusolverDnCheevjBatched_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            jobz: cusolverEigMode_t,
            uplo: cublasFillMode_t,
            n: ::std::os::raw::c_int,
            A: *const cuComplex,
            lda: ::std::os::raw::c_int,
            W: *const f32,
            lwork: *mut ::std::os::raw::c_int,
            params: syevjInfo_t,
            batchSize: ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnZheevjBatched_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            jobz: cusolverEigMode_t,
            uplo: cublasFillMode_t,
            n: ::std::os::raw::c_int,
            A: *const cuDoubleComplex,
            lda: ::std::os::raw::c_int,
            W: *const f64,
            lwork: *mut ::std::os::raw::c_int,
            params: syevjInfo_t,
            batchSize: ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnSsyevjBatched: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            jobz: cusolverEigMode_t,
            uplo: cublasFillMode_t,
            n: ::std::os::raw::c_int,
            A: *mut f32,
            lda: ::std::os::raw::c_int,
            W: *mut f32,
            work: *mut f32,
            lwork: ::std::os::raw::c_int,
            info: *mut ::std::os::raw::c_int,
            params: syevjInfo_t,
            batchSize: ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnDsyevjBatched: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            jobz: cusolverEigMode_t,
            uplo: cublasFillMode_t,
            n: ::std::os::raw::c_int,
            A: *mut f64,
            lda: ::std::os::raw::c_int,
            W: *mut f64,
            work: *mut f64,
            lwork: ::std::os::raw::c_int,
            info: *mut ::std::os::raw::c_int,
            params: syevjInfo_t,
            batchSize: ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnCheevjBatched: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            jobz: cusolverEigMode_t,
            uplo: cublasFillMode_t,
            n: ::std::os::raw::c_int,
            A: *mut cuComplex,
            lda: ::std::os::raw::c_int,
            W: *mut f32,
            work: *mut cuComplex,
            lwork: ::std::os::raw::c_int,
            info: *mut ::std::os::raw::c_int,
            params: syevjInfo_t,
            batchSize: ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnZheevjBatched: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            jobz: cusolverEigMode_t,
            uplo: cublasFillMode_t,
            n: ::std::os::raw::c_int,
            A: *mut cuDoubleComplex,
            lda: ::std::os::raw::c_int,
            W: *mut f64,
            work: *mut cuDoubleComplex,
            lwork: ::std::os::raw::c_int,
            info: *mut ::std::os::raw::c_int,
            params: syevjInfo_t,
            batchSize: ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnSsyevj_bufferSize:
        Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, jobz: cusolverEigMode_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *const f32, lda: ::std::os::raw::c_int, W: *const f32, lwork: *mut ::std::os::raw::c_int, params: syevjInfo_t) -> cusolverStatus_t>,
    pub cusolverDnDsyevj_bufferSize:
        Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, jobz: cusolverEigMode_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *const f64, lda: ::std::os::raw::c_int, W: *const f64, lwork: *mut ::std::os::raw::c_int, params: syevjInfo_t) -> cusolverStatus_t>,
    pub cusolverDnCheevj_bufferSize:
        Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, jobz: cusolverEigMode_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *const cuComplex, lda: ::std::os::raw::c_int, W: *const f32, lwork: *mut ::std::os::raw::c_int, params: syevjInfo_t) -> cusolverStatus_t>,
    pub cusolverDnZheevj_bufferSize:
        Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, jobz: cusolverEigMode_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *const cuDoubleComplex, lda: ::std::os::raw::c_int, W: *const f64, lwork: *mut ::std::os::raw::c_int, params: syevjInfo_t) -> cusolverStatus_t>,
    pub cusolverDnSsyevj: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            jobz: cusolverEigMode_t,
            uplo: cublasFillMode_t,
            n: ::std::os::raw::c_int,
            A: *mut f32,
            lda: ::std::os::raw::c_int,
            W: *mut f32,
            work: *mut f32,
            lwork: ::std::os::raw::c_int,
            info: *mut ::std::os::raw::c_int,
            params: syevjInfo_t,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnDsyevj: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            jobz: cusolverEigMode_t,
            uplo: cublasFillMode_t,
            n: ::std::os::raw::c_int,
            A: *mut f64,
            lda: ::std::os::raw::c_int,
            W: *mut f64,
            work: *mut f64,
            lwork: ::std::os::raw::c_int,
            info: *mut ::std::os::raw::c_int,
            params: syevjInfo_t,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnCheevj: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            jobz: cusolverEigMode_t,
            uplo: cublasFillMode_t,
            n: ::std::os::raw::c_int,
            A: *mut cuComplex,
            lda: ::std::os::raw::c_int,
            W: *mut f32,
            work: *mut cuComplex,
            lwork: ::std::os::raw::c_int,
            info: *mut ::std::os::raw::c_int,
            params: syevjInfo_t,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnZheevj: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            jobz: cusolverEigMode_t,
            uplo: cublasFillMode_t,
            n: ::std::os::raw::c_int,
            A: *mut cuDoubleComplex,
            lda: ::std::os::raw::c_int,
            W: *mut f64,
            work: *mut cuDoubleComplex,
            lwork: ::std::os::raw::c_int,
            info: *mut ::std::os::raw::c_int,
            params: syevjInfo_t,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnSsygvj_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            itype: cusolverEigType_t,
            jobz: cusolverEigMode_t,
            uplo: cublasFillMode_t,
            n: ::std::os::raw::c_int,
            A: *const f32,
            lda: ::std::os::raw::c_int,
            B: *const f32,
            ldb: ::std::os::raw::c_int,
            W: *const f32,
            lwork: *mut ::std::os::raw::c_int,
            params: syevjInfo_t,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnDsygvj_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            itype: cusolverEigType_t,
            jobz: cusolverEigMode_t,
            uplo: cublasFillMode_t,
            n: ::std::os::raw::c_int,
            A: *const f64,
            lda: ::std::os::raw::c_int,
            B: *const f64,
            ldb: ::std::os::raw::c_int,
            W: *const f64,
            lwork: *mut ::std::os::raw::c_int,
            params: syevjInfo_t,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnChegvj_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            itype: cusolverEigType_t,
            jobz: cusolverEigMode_t,
            uplo: cublasFillMode_t,
            n: ::std::os::raw::c_int,
            A: *const cuComplex,
            lda: ::std::os::raw::c_int,
            B: *const cuComplex,
            ldb: ::std::os::raw::c_int,
            W: *const f32,
            lwork: *mut ::std::os::raw::c_int,
            params: syevjInfo_t,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnZhegvj_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            itype: cusolverEigType_t,
            jobz: cusolverEigMode_t,
            uplo: cublasFillMode_t,
            n: ::std::os::raw::c_int,
            A: *const cuDoubleComplex,
            lda: ::std::os::raw::c_int,
            B: *const cuDoubleComplex,
            ldb: ::std::os::raw::c_int,
            W: *const f64,
            lwork: *mut ::std::os::raw::c_int,
            params: syevjInfo_t,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnSsygvj: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            itype: cusolverEigType_t,
            jobz: cusolverEigMode_t,
            uplo: cublasFillMode_t,
            n: ::std::os::raw::c_int,
            A: *mut f32,
            lda: ::std::os::raw::c_int,
            B: *mut f32,
            ldb: ::std::os::raw::c_int,
            W: *mut f32,
            work: *mut f32,
            lwork: ::std::os::raw::c_int,
            info: *mut ::std::os::raw::c_int,
            params: syevjInfo_t,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnDsygvj: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            itype: cusolverEigType_t,
            jobz: cusolverEigMode_t,
            uplo: cublasFillMode_t,
            n: ::std::os::raw::c_int,
            A: *mut f64,
            lda: ::std::os::raw::c_int,
            B: *mut f64,
            ldb: ::std::os::raw::c_int,
            W: *mut f64,
            work: *mut f64,
            lwork: ::std::os::raw::c_int,
            info: *mut ::std::os::raw::c_int,
            params: syevjInfo_t,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnChegvj: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            itype: cusolverEigType_t,
            jobz: cusolverEigMode_t,
            uplo: cublasFillMode_t,
            n: ::std::os::raw::c_int,
            A: *mut cuComplex,
            lda: ::std::os::raw::c_int,
            B: *mut cuComplex,
            ldb: ::std::os::raw::c_int,
            W: *mut f32,
            work: *mut cuComplex,
            lwork: ::std::os::raw::c_int,
            info: *mut ::std::os::raw::c_int,
            params: syevjInfo_t,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnZhegvj: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            itype: cusolverEigType_t,
            jobz: cusolverEigMode_t,
            uplo: cublasFillMode_t,
            n: ::std::os::raw::c_int,
            A: *mut cuDoubleComplex,
            lda: ::std::os::raw::c_int,
            B: *mut cuDoubleComplex,
            ldb: ::std::os::raw::c_int,
            W: *mut f64,
            work: *mut cuDoubleComplex,
            lwork: ::std::os::raw::c_int,
            info: *mut ::std::os::raw::c_int,
            params: syevjInfo_t,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnCreateGesvdjInfo: Option<unsafe extern "C" fn(info: *mut gesvdjInfo_t) -> cusolverStatus_t>,
    pub cusolverDnDestroyGesvdjInfo: Option<unsafe extern "C" fn(info: gesvdjInfo_t) -> cusolverStatus_t>,
    pub cusolverDnXgesvdjSetTolerance: Option<unsafe extern "C" fn(info: gesvdjInfo_t, tolerance: f64) -> cusolverStatus_t>,
    pub cusolverDnXgesvdjSetMaxSweeps: Option<unsafe extern "C" fn(info: gesvdjInfo_t, max_sweeps: ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnXgesvdjSetSortEig: Option<unsafe extern "C" fn(info: gesvdjInfo_t, sort_svd: ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnXgesvdjGetResidual: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, info: gesvdjInfo_t, residual: *mut f64) -> cusolverStatus_t>,
    pub cusolverDnXgesvdjGetSweeps: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, info: gesvdjInfo_t, executed_sweeps: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnSgesvdjBatched_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            jobz: cusolverEigMode_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            A: *const f32,
            lda: ::std::os::raw::c_int,
            S: *const f32,
            U: *const f32,
            ldu: ::std::os::raw::c_int,
            V: *const f32,
            ldv: ::std::os::raw::c_int,
            lwork: *mut ::std::os::raw::c_int,
            params: gesvdjInfo_t,
            batchSize: ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnDgesvdjBatched_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            jobz: cusolverEigMode_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            A: *const f64,
            lda: ::std::os::raw::c_int,
            S: *const f64,
            U: *const f64,
            ldu: ::std::os::raw::c_int,
            V: *const f64,
            ldv: ::std::os::raw::c_int,
            lwork: *mut ::std::os::raw::c_int,
            params: gesvdjInfo_t,
            batchSize: ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnCgesvdjBatched_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            jobz: cusolverEigMode_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            A: *const cuComplex,
            lda: ::std::os::raw::c_int,
            S: *const f32,
            U: *const cuComplex,
            ldu: ::std::os::raw::c_int,
            V: *const cuComplex,
            ldv: ::std::os::raw::c_int,
            lwork: *mut ::std::os::raw::c_int,
            params: gesvdjInfo_t,
            batchSize: ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnZgesvdjBatched_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            jobz: cusolverEigMode_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            A: *const cuDoubleComplex,
            lda: ::std::os::raw::c_int,
            S: *const f64,
            U: *const cuDoubleComplex,
            ldu: ::std::os::raw::c_int,
            V: *const cuDoubleComplex,
            ldv: ::std::os::raw::c_int,
            lwork: *mut ::std::os::raw::c_int,
            params: gesvdjInfo_t,
            batchSize: ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnSgesvdjBatched: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            jobz: cusolverEigMode_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            A: *mut f32,
            lda: ::std::os::raw::c_int,
            S: *mut f32,
            U: *mut f32,
            ldu: ::std::os::raw::c_int,
            V: *mut f32,
            ldv: ::std::os::raw::c_int,
            work: *mut f32,
            lwork: ::std::os::raw::c_int,
            info: *mut ::std::os::raw::c_int,
            params: gesvdjInfo_t,
            batchSize: ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnDgesvdjBatched: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            jobz: cusolverEigMode_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            A: *mut f64,
            lda: ::std::os::raw::c_int,
            S: *mut f64,
            U: *mut f64,
            ldu: ::std::os::raw::c_int,
            V: *mut f64,
            ldv: ::std::os::raw::c_int,
            work: *mut f64,
            lwork: ::std::os::raw::c_int,
            info: *mut ::std::os::raw::c_int,
            params: gesvdjInfo_t,
            batchSize: ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnCgesvdjBatched: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            jobz: cusolverEigMode_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            A: *mut cuComplex,
            lda: ::std::os::raw::c_int,
            S: *mut f32,
            U: *mut cuComplex,
            ldu: ::std::os::raw::c_int,
            V: *mut cuComplex,
            ldv: ::std::os::raw::c_int,
            work: *mut cuComplex,
            lwork: ::std::os::raw::c_int,
            info: *mut ::std::os::raw::c_int,
            params: gesvdjInfo_t,
            batchSize: ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnZgesvdjBatched: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            jobz: cusolverEigMode_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            A: *mut cuDoubleComplex,
            lda: ::std::os::raw::c_int,
            S: *mut f64,
            U: *mut cuDoubleComplex,
            ldu: ::std::os::raw::c_int,
            V: *mut cuDoubleComplex,
            ldv: ::std::os::raw::c_int,
            work: *mut cuDoubleComplex,
            lwork: ::std::os::raw::c_int,
            info: *mut ::std::os::raw::c_int,
            params: gesvdjInfo_t,
            batchSize: ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnSgesvdj_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            jobz: cusolverEigMode_t,
            econ: ::std::os::raw::c_int,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            A: *const f32,
            lda: ::std::os::raw::c_int,
            S: *const f32,
            U: *const f32,
            ldu: ::std::os::raw::c_int,
            V: *const f32,
            ldv: ::std::os::raw::c_int,
            lwork: *mut ::std::os::raw::c_int,
            params: gesvdjInfo_t,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnDgesvdj_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            jobz: cusolverEigMode_t,
            econ: ::std::os::raw::c_int,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            A: *const f64,
            lda: ::std::os::raw::c_int,
            S: *const f64,
            U: *const f64,
            ldu: ::std::os::raw::c_int,
            V: *const f64,
            ldv: ::std::os::raw::c_int,
            lwork: *mut ::std::os::raw::c_int,
            params: gesvdjInfo_t,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnCgesvdj_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            jobz: cusolverEigMode_t,
            econ: ::std::os::raw::c_int,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            A: *const cuComplex,
            lda: ::std::os::raw::c_int,
            S: *const f32,
            U: *const cuComplex,
            ldu: ::std::os::raw::c_int,
            V: *const cuComplex,
            ldv: ::std::os::raw::c_int,
            lwork: *mut ::std::os::raw::c_int,
            params: gesvdjInfo_t,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnZgesvdj_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            jobz: cusolverEigMode_t,
            econ: ::std::os::raw::c_int,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            A: *const cuDoubleComplex,
            lda: ::std::os::raw::c_int,
            S: *const f64,
            U: *const cuDoubleComplex,
            ldu: ::std::os::raw::c_int,
            V: *const cuDoubleComplex,
            ldv: ::std::os::raw::c_int,
            lwork: *mut ::std::os::raw::c_int,
            params: gesvdjInfo_t,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnSgesvdj: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            jobz: cusolverEigMode_t,
            econ: ::std::os::raw::c_int,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            A: *mut f32,
            lda: ::std::os::raw::c_int,
            S: *mut f32,
            U: *mut f32,
            ldu: ::std::os::raw::c_int,
            V: *mut f32,
            ldv: ::std::os::raw::c_int,
            work: *mut f32,
            lwork: ::std::os::raw::c_int,
            info: *mut ::std::os::raw::c_int,
            params: gesvdjInfo_t,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnDgesvdj: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            jobz: cusolverEigMode_t,
            econ: ::std::os::raw::c_int,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            A: *mut f64,
            lda: ::std::os::raw::c_int,
            S: *mut f64,
            U: *mut f64,
            ldu: ::std::os::raw::c_int,
            V: *mut f64,
            ldv: ::std::os::raw::c_int,
            work: *mut f64,
            lwork: ::std::os::raw::c_int,
            info: *mut ::std::os::raw::c_int,
            params: gesvdjInfo_t,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnCgesvdj: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            jobz: cusolverEigMode_t,
            econ: ::std::os::raw::c_int,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            A: *mut cuComplex,
            lda: ::std::os::raw::c_int,
            S: *mut f32,
            U: *mut cuComplex,
            ldu: ::std::os::raw::c_int,
            V: *mut cuComplex,
            ldv: ::std::os::raw::c_int,
            work: *mut cuComplex,
            lwork: ::std::os::raw::c_int,
            info: *mut ::std::os::raw::c_int,
            params: gesvdjInfo_t,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnZgesvdj: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            jobz: cusolverEigMode_t,
            econ: ::std::os::raw::c_int,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            A: *mut cuDoubleComplex,
            lda: ::std::os::raw::c_int,
            S: *mut f64,
            U: *mut cuDoubleComplex,
            ldu: ::std::os::raw::c_int,
            V: *mut cuDoubleComplex,
            ldv: ::std::os::raw::c_int,
            work: *mut cuDoubleComplex,
            lwork: ::std::os::raw::c_int,
            info: *mut ::std::os::raw::c_int,
            params: gesvdjInfo_t,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnSgesvdaStridedBatched_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            jobz: cusolverEigMode_t,
            rank: ::std::os::raw::c_int,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            d_A: *const f32,
            lda: ::std::os::raw::c_int,
            strideA: ::std::os::raw::c_longlong,
            d_S: *const f32,
            strideS: ::std::os::raw::c_longlong,
            d_U: *const f32,
            ldu: ::std::os::raw::c_int,
            strideU: ::std::os::raw::c_longlong,
            d_V: *const f32,
            ldv: ::std::os::raw::c_int,
            strideV: ::std::os::raw::c_longlong,
            lwork: *mut ::std::os::raw::c_int,
            batchSize: ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnDgesvdaStridedBatched_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            jobz: cusolverEigMode_t,
            rank: ::std::os::raw::c_int,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            d_A: *const f64,
            lda: ::std::os::raw::c_int,
            strideA: ::std::os::raw::c_longlong,
            d_S: *const f64,
            strideS: ::std::os::raw::c_longlong,
            d_U: *const f64,
            ldu: ::std::os::raw::c_int,
            strideU: ::std::os::raw::c_longlong,
            d_V: *const f64,
            ldv: ::std::os::raw::c_int,
            strideV: ::std::os::raw::c_longlong,
            lwork: *mut ::std::os::raw::c_int,
            batchSize: ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnCgesvdaStridedBatched_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            jobz: cusolverEigMode_t,
            rank: ::std::os::raw::c_int,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            d_A: *const cuComplex,
            lda: ::std::os::raw::c_int,
            strideA: ::std::os::raw::c_longlong,
            d_S: *const f32,
            strideS: ::std::os::raw::c_longlong,
            d_U: *const cuComplex,
            ldu: ::std::os::raw::c_int,
            strideU: ::std::os::raw::c_longlong,
            d_V: *const cuComplex,
            ldv: ::std::os::raw::c_int,
            strideV: ::std::os::raw::c_longlong,
            lwork: *mut ::std::os::raw::c_int,
            batchSize: ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnZgesvdaStridedBatched_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            jobz: cusolverEigMode_t,
            rank: ::std::os::raw::c_int,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            d_A: *const cuDoubleComplex,
            lda: ::std::os::raw::c_int,
            strideA: ::std::os::raw::c_longlong,
            d_S: *const f64,
            strideS: ::std::os::raw::c_longlong,
            d_U: *const cuDoubleComplex,
            ldu: ::std::os::raw::c_int,
            strideU: ::std::os::raw::c_longlong,
            d_V: *const cuDoubleComplex,
            ldv: ::std::os::raw::c_int,
            strideV: ::std::os::raw::c_longlong,
            lwork: *mut ::std::os::raw::c_int,
            batchSize: ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnSgesvdaStridedBatched: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            jobz: cusolverEigMode_t,
            rank: ::std::os::raw::c_int,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            d_A: *const f32,
            lda: ::std::os::raw::c_int,
            strideA: ::std::os::raw::c_longlong,
            d_S: *mut f32,
            strideS: ::std::os::raw::c_longlong,
            d_U: *mut f32,
            ldu: ::std::os::raw::c_int,
            strideU: ::std::os::raw::c_longlong,
            d_V: *mut f32,
            ldv: ::std::os::raw::c_int,
            strideV: ::std::os::raw::c_longlong,
            d_work: *mut f32,
            lwork: ::std::os::raw::c_int,
            d_info: *mut ::std::os::raw::c_int,
            h_R_nrmF: *mut f64,
            batchSize: ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnDgesvdaStridedBatched: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            jobz: cusolverEigMode_t,
            rank: ::std::os::raw::c_int,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            d_A: *const f64,
            lda: ::std::os::raw::c_int,
            strideA: ::std::os::raw::c_longlong,
            d_S: *mut f64,
            strideS: ::std::os::raw::c_longlong,
            d_U: *mut f64,
            ldu: ::std::os::raw::c_int,
            strideU: ::std::os::raw::c_longlong,
            d_V: *mut f64,
            ldv: ::std::os::raw::c_int,
            strideV: ::std::os::raw::c_longlong,
            d_work: *mut f64,
            lwork: ::std::os::raw::c_int,
            d_info: *mut ::std::os::raw::c_int,
            h_R_nrmF: *mut f64,
            batchSize: ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnCgesvdaStridedBatched: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            jobz: cusolverEigMode_t,
            rank: ::std::os::raw::c_int,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            d_A: *const cuComplex,
            lda: ::std::os::raw::c_int,
            strideA: ::std::os::raw::c_longlong,
            d_S: *mut f32,
            strideS: ::std::os::raw::c_longlong,
            d_U: *mut cuComplex,
            ldu: ::std::os::raw::c_int,
            strideU: ::std::os::raw::c_longlong,
            d_V: *mut cuComplex,
            ldv: ::std::os::raw::c_int,
            strideV: ::std::os::raw::c_longlong,
            d_work: *mut cuComplex,
            lwork: ::std::os::raw::c_int,
            d_info: *mut ::std::os::raw::c_int,
            h_R_nrmF: *mut f64,
            batchSize: ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnZgesvdaStridedBatched: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            jobz: cusolverEigMode_t,
            rank: ::std::os::raw::c_int,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            d_A: *const cuDoubleComplex,
            lda: ::std::os::raw::c_int,
            strideA: ::std::os::raw::c_longlong,
            d_S: *mut f64,
            strideS: ::std::os::raw::c_longlong,
            d_U: *mut cuDoubleComplex,
            ldu: ::std::os::raw::c_int,
            strideU: ::std::os::raw::c_longlong,
            d_V: *mut cuDoubleComplex,
            ldv: ::std::os::raw::c_int,
            strideV: ::std::os::raw::c_longlong,
            d_work: *mut cuDoubleComplex,
            lwork: ::std::os::raw::c_int,
            d_info: *mut ::std::os::raw::c_int,
            h_R_nrmF: *mut f64,
            batchSize: ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnCreateParams: Option<unsafe extern "C" fn(params: *mut cusolverDnParams_t) -> cusolverStatus_t>,
    pub cusolverDnDestroyParams: Option<unsafe extern "C" fn(params: cusolverDnParams_t) -> cusolverStatus_t>,
    pub cusolverDnSetAdvOptions: Option<unsafe extern "C" fn(params: cusolverDnParams_t, function: cusolverDnFunction_t, algo: cusolverAlgMode_t) -> cusolverStatus_t>,
    pub cusolverDnXpotrf_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            params: cusolverDnParams_t,
            uplo: cublasFillMode_t,
            n: i64,
            dataTypeA: cudaDataType,
            A: *const ::std::os::raw::c_void,
            lda: i64,
            computeType: cudaDataType,
            workspaceInBytesOnDevice: *mut usize,
            workspaceInBytesOnHost: *mut usize,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnXpotrf: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            params: cusolverDnParams_t,
            uplo: cublasFillMode_t,
            n: i64,
            dataTypeA: cudaDataType,
            A: *mut ::std::os::raw::c_void,
            lda: i64,
            computeType: cudaDataType,
            bufferOnDevice: *mut ::std::os::raw::c_void,
            workspaceInBytesOnDevice: usize,
            bufferOnHost: *mut ::std::os::raw::c_void,
            workspaceInBytesOnHost: usize,
            info: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnXpotrs: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            params: cusolverDnParams_t,
            uplo: cublasFillMode_t,
            n: i64,
            nrhs: i64,
            dataTypeA: cudaDataType,
            A: *const ::std::os::raw::c_void,
            lda: i64,
            dataTypeB: cudaDataType,
            B: *mut ::std::os::raw::c_void,
            ldb: i64,
            info: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnXgeqrf_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            params: cusolverDnParams_t,
            m: i64,
            n: i64,
            dataTypeA: cudaDataType,
            A: *const ::std::os::raw::c_void,
            lda: i64,
            dataTypeTau: cudaDataType,
            tau: *const ::std::os::raw::c_void,
            computeType: cudaDataType,
            workspaceInBytesOnDevice: *mut usize,
            workspaceInBytesOnHost: *mut usize,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnXgeqrf: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            params: cusolverDnParams_t,
            m: i64,
            n: i64,
            dataTypeA: cudaDataType,
            A: *mut ::std::os::raw::c_void,
            lda: i64,
            dataTypeTau: cudaDataType,
            tau: *mut ::std::os::raw::c_void,
            computeType: cudaDataType,
            bufferOnDevice: *mut ::std::os::raw::c_void,
            workspaceInBytesOnDevice: usize,
            bufferOnHost: *mut ::std::os::raw::c_void,
            workspaceInBytesOnHost: usize,
            info: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnXgetrf_bufferSize:
        Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, params: cusolverDnParams_t, m: i64, n: i64, dataTypeA: cudaDataType, A: *const ::std::os::raw::c_void, lda: i64, computeType: cudaDataType, workspaceInBytesOnDevice: *mut usize, workspaceInBytesOnHost: *mut usize) -> cusolverStatus_t>,
    pub cusolverDnXgetrf: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            params: cusolverDnParams_t,
            m: i64,
            n: i64,
            dataTypeA: cudaDataType,
            A: *mut ::std::os::raw::c_void,
            lda: i64,
            ipiv: *mut i64,
            computeType: cudaDataType,
            bufferOnDevice: *mut ::std::os::raw::c_void,
            workspaceInBytesOnDevice: usize,
            bufferOnHost: *mut ::std::os::raw::c_void,
            workspaceInBytesOnHost: usize,
            info: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnXgetrs: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            params: cusolverDnParams_t,
            trans: cublasOperation_t,
            n: i64,
            nrhs: i64,
            dataTypeA: cudaDataType,
            A: *const ::std::os::raw::c_void,
            lda: i64,
            ipiv: *const i64,
            dataTypeB: cudaDataType,
            B: *mut ::std::os::raw::c_void,
            ldb: i64,
            info: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnXsyevd_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            params: cusolverDnParams_t,
            jobz: cusolverEigMode_t,
            uplo: cublasFillMode_t,
            n: i64,
            dataTypeA: cudaDataType,
            A: *const ::std::os::raw::c_void,
            lda: i64,
            dataTypeW: cudaDataType,
            W: *const ::std::os::raw::c_void,
            computeType: cudaDataType,
            workspaceInBytesOnDevice: *mut usize,
            workspaceInBytesOnHost: *mut usize,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnXsyevd: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            params: cusolverDnParams_t,
            jobz: cusolverEigMode_t,
            uplo: cublasFillMode_t,
            n: i64,
            dataTypeA: cudaDataType,
            A: *mut ::std::os::raw::c_void,
            lda: i64,
            dataTypeW: cudaDataType,
            W: *mut ::std::os::raw::c_void,
            computeType: cudaDataType,
            bufferOnDevice: *mut ::std::os::raw::c_void,
            workspaceInBytesOnDevice: usize,
            bufferOnHost: *mut ::std::os::raw::c_void,
            workspaceInBytesOnHost: usize,
            info: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnXstedc_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            params: cusolverDnParams_t,
            compz: cusolverEigComp_t,
            n: i64,
            dataTypeDE: cudaDataType,
            D: *const ::std::os::raw::c_void,
            E: *const ::std::os::raw::c_void,
            dataTypeZ: cudaDataType,
            Z: *const ::std::os::raw::c_void,
            ldz: i64,
            computeType: cudaDataType,
            workspaceInBytesOnDevice: *mut usize,
            workspaceInBytesOnHost: *mut usize,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnXstedc: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            params: cusolverDnParams_t,
            compz: cusolverEigComp_t,
            n: i64,
            dataTypeDE: cudaDataType,
            D: *mut ::std::os::raw::c_void,
            E: *mut ::std::os::raw::c_void,
            dataTypeZ: cudaDataType,
            Z: *mut ::std::os::raw::c_void,
            ldz: i64,
            computeType: cudaDataType,
            bufferOnDevice: *mut ::std::os::raw::c_void,
            workspaceInBytesOnDevice: usize,
            bufferOnHost: *mut ::std::os::raw::c_void,
            workspaceInBytesOnHost: usize,
            info: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnXsyevBatched_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            params: cusolverDnParams_t,
            jobz: cusolverEigMode_t,
            uplo: cublasFillMode_t,
            n: i64,
            dataTypeA: cudaDataType,
            A: *const ::std::os::raw::c_void,
            lda: i64,
            dataTypeW: cudaDataType,
            W: *const ::std::os::raw::c_void,
            computeType: cudaDataType,
            workspaceInBytesOnDevice: *mut usize,
            workspaceInBytesOnHost: *mut usize,
            batchSize: i64,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnXsyevBatched: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            params: cusolverDnParams_t,
            jobz: cusolverEigMode_t,
            uplo: cublasFillMode_t,
            n: i64,
            dataTypeA: cudaDataType,
            A: *mut ::std::os::raw::c_void,
            lda: i64,
            dataTypeW: cudaDataType,
            W: *mut ::std::os::raw::c_void,
            computeType: cudaDataType,
            bufferOnDevice: *mut ::std::os::raw::c_void,
            workspaceInBytesOnDevice: usize,
            bufferOnHost: *mut ::std::os::raw::c_void,
            workspaceInBytesOnHost: usize,
            info: *mut ::std::os::raw::c_int,
            batchSize: i64,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnXsyevdx_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            params: cusolverDnParams_t,
            jobz: cusolverEigMode_t,
            range: cusolverEigRange_t,
            uplo: cublasFillMode_t,
            n: i64,
            dataTypeA: cudaDataType,
            A: *const ::std::os::raw::c_void,
            lda: i64,
            vl: *mut ::std::os::raw::c_void,
            vu: *mut ::std::os::raw::c_void,
            il: i64,
            iu: i64,
            h_meig: *mut i64,
            dataTypeW: cudaDataType,
            W: *const ::std::os::raw::c_void,
            computeType: cudaDataType,
            workspaceInBytesOnDevice: *mut usize,
            workspaceInBytesOnHost: *mut usize,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnXsyevdx: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            params: cusolverDnParams_t,
            jobz: cusolverEigMode_t,
            range: cusolverEigRange_t,
            uplo: cublasFillMode_t,
            n: i64,
            dataTypeA: cudaDataType,
            A: *mut ::std::os::raw::c_void,
            lda: i64,
            vl: *mut ::std::os::raw::c_void,
            vu: *mut ::std::os::raw::c_void,
            il: i64,
            iu: i64,
            meig64: *mut i64,
            dataTypeW: cudaDataType,
            W: *mut ::std::os::raw::c_void,
            computeType: cudaDataType,
            bufferOnDevice: *mut ::std::os::raw::c_void,
            workspaceInBytesOnDevice: usize,
            bufferOnHost: *mut ::std::os::raw::c_void,
            workspaceInBytesOnHost: usize,
            info: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnXgeev_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            params: cusolverDnParams_t,
            jobvl: cusolverEigMode_t,
            jobvr: cusolverEigMode_t,
            n: i64,
            dataTypeA: cudaDataType,
            A: *const ::std::os::raw::c_void,
            lda: i64,
            dataTypeW: cudaDataType,
            W: *const ::std::os::raw::c_void,
            dataTypeVL: cudaDataType,
            VL: *const ::std::os::raw::c_void,
            ldvl: i64,
            dataTypeVR: cudaDataType,
            VR: *const ::std::os::raw::c_void,
            ldvr: i64,
            computeType: cudaDataType,
            workspaceInBytesOnDevice: *mut usize,
            workspaceInBytesOnHost: *mut usize,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnXgeev: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            params: cusolverDnParams_t,
            jobvl: cusolverEigMode_t,
            jobvr: cusolverEigMode_t,
            n: i64,
            dataTypeA: cudaDataType,
            A: *mut ::std::os::raw::c_void,
            lda: i64,
            dataTypeW: cudaDataType,
            W: *mut ::std::os::raw::c_void,
            dataTypeVL: cudaDataType,
            VL: *mut ::std::os::raw::c_void,
            ldvl: i64,
            dataTypeVR: cudaDataType,
            VR: *mut ::std::os::raw::c_void,
            ldvr: i64,
            computeType: cudaDataType,
            bufferOnDevice: *mut ::std::os::raw::c_void,
            workspaceInBytesOnDevice: usize,
            bufferOnHost: *mut ::std::os::raw::c_void,
            workspaceInBytesOnHost: usize,
            info: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnXgesvd_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            params: cusolverDnParams_t,
            jobu: ::std::os::raw::c_schar,
            jobvt: ::std::os::raw::c_schar,
            m: i64,
            n: i64,
            dataTypeA: cudaDataType,
            A: *const ::std::os::raw::c_void,
            lda: i64,
            dataTypeS: cudaDataType,
            S: *const ::std::os::raw::c_void,
            dataTypeU: cudaDataType,
            U: *const ::std::os::raw::c_void,
            ldu: i64,
            dataTypeVT: cudaDataType,
            VT: *const ::std::os::raw::c_void,
            ldvt: i64,
            computeType: cudaDataType,
            workspaceInBytesOnDevice: *mut usize,
            workspaceInBytesOnHost: *mut usize,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnXgesvd: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            params: cusolverDnParams_t,
            jobu: ::std::os::raw::c_schar,
            jobvt: ::std::os::raw::c_schar,
            m: i64,
            n: i64,
            dataTypeA: cudaDataType,
            A: *mut ::std::os::raw::c_void,
            lda: i64,
            dataTypeS: cudaDataType,
            S: *mut ::std::os::raw::c_void,
            dataTypeU: cudaDataType,
            U: *mut ::std::os::raw::c_void,
            ldu: i64,
            dataTypeVT: cudaDataType,
            VT: *mut ::std::os::raw::c_void,
            ldvt: i64,
            computeType: cudaDataType,
            bufferOnDevice: *mut ::std::os::raw::c_void,
            workspaceInBytesOnDevice: usize,
            bufferOnHost: *mut ::std::os::raw::c_void,
            workspaceInBytesOnHost: usize,
            info: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnXgesvdp_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            params: cusolverDnParams_t,
            jobz: cusolverEigMode_t,
            econ: ::std::os::raw::c_int,
            m: i64,
            n: i64,
            dataTypeA: cudaDataType,
            A: *const ::std::os::raw::c_void,
            lda: i64,
            dataTypeS: cudaDataType,
            S: *const ::std::os::raw::c_void,
            dataTypeU: cudaDataType,
            U: *const ::std::os::raw::c_void,
            ldu: i64,
            dataTypeV: cudaDataType,
            V: *const ::std::os::raw::c_void,
            ldv: i64,
            computeType: cudaDataType,
            workspaceInBytesOnDevice: *mut usize,
            workspaceInBytesOnHost: *mut usize,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnXgesvdp: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            params: cusolverDnParams_t,
            jobz: cusolverEigMode_t,
            econ: ::std::os::raw::c_int,
            m: i64,
            n: i64,
            dataTypeA: cudaDataType,
            A: *mut ::std::os::raw::c_void,
            lda: i64,
            dataTypeS: cudaDataType,
            S: *mut ::std::os::raw::c_void,
            dataTypeU: cudaDataType,
            U: *mut ::std::os::raw::c_void,
            ldu: i64,
            dataTypeV: cudaDataType,
            V: *mut ::std::os::raw::c_void,
            ldv: i64,
            computeType: cudaDataType,
            bufferOnDevice: *mut ::std::os::raw::c_void,
            workspaceInBytesOnDevice: usize,
            bufferOnHost: *mut ::std::os::raw::c_void,
            workspaceInBytesOnHost: usize,
            d_info: *mut ::std::os::raw::c_int,
            h_err_sigma: *mut f64,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnXgesvdr_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            params: cusolverDnParams_t,
            jobu: ::std::os::raw::c_schar,
            jobv: ::std::os::raw::c_schar,
            m: i64,
            n: i64,
            k: i64,
            p: i64,
            niters: i64,
            dataTypeA: cudaDataType,
            A: *const ::std::os::raw::c_void,
            lda: i64,
            dataTypeSrand: cudaDataType,
            Srand: *const ::std::os::raw::c_void,
            dataTypeUrand: cudaDataType,
            Urand: *const ::std::os::raw::c_void,
            ldUrand: i64,
            dataTypeVrand: cudaDataType,
            Vrand: *const ::std::os::raw::c_void,
            ldVrand: i64,
            computeType: cudaDataType,
            workspaceInBytesOnDevice: *mut usize,
            workspaceInBytesOnHost: *mut usize,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnXgesvdr: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            params: cusolverDnParams_t,
            jobu: ::std::os::raw::c_schar,
            jobv: ::std::os::raw::c_schar,
            m: i64,
            n: i64,
            k: i64,
            p: i64,
            niters: i64,
            dataTypeA: cudaDataType,
            A: *mut ::std::os::raw::c_void,
            lda: i64,
            dataTypeSrand: cudaDataType,
            Srand: *mut ::std::os::raw::c_void,
            dataTypeUrand: cudaDataType,
            Urand: *mut ::std::os::raw::c_void,
            ldUrand: i64,
            dataTypeVrand: cudaDataType,
            Vrand: *mut ::std::os::raw::c_void,
            ldVrand: i64,
            computeType: cudaDataType,
            bufferOnDevice: *mut ::std::os::raw::c_void,
            workspaceInBytesOnDevice: usize,
            bufferOnHost: *mut ::std::os::raw::c_void,
            workspaceInBytesOnHost: usize,
            d_info: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnXlarft_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            params: cusolverDnParams_t,
            direct: cusolverDirectMode_t,
            storev: cusolverStorevMode_t,
            n: i64,
            k: i64,
            dataTypeV: cudaDataType,
            V: *const ::std::os::raw::c_void,
            ldv: i64,
            dataTypeTau: cudaDataType,
            tau: *const ::std::os::raw::c_void,
            dataTypeT: cudaDataType,
            T: *mut ::std::os::raw::c_void,
            ldt: i64,
            computeType: cudaDataType,
            workspaceInBytesOnDevice: *mut usize,
            workspaceInBytesOnHost: *mut usize,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnXlarft: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            params: cusolverDnParams_t,
            direct: cusolverDirectMode_t,
            storev: cusolverStorevMode_t,
            n: i64,
            k: i64,
            dataTypeV: cudaDataType,
            V: *const ::std::os::raw::c_void,
            ldv: i64,
            dataTypeTau: cudaDataType,
            tau: *const ::std::os::raw::c_void,
            dataTypeT: cudaDataType,
            T: *mut ::std::os::raw::c_void,
            ldt: i64,
            computeType: cudaDataType,
            bufferOnDevice: *mut ::std::os::raw::c_void,
            workspaceInBytesOnDevice: usize,
            bufferOnHost: *mut ::std::os::raw::c_void,
            workspaceInBytesOnHost: usize,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnLoggerSetCallback: Option<unsafe extern "C" fn(callback: cusolverDnLoggerCallback_t) -> cusolverStatus_t>,
    pub cusolverDnLoggerSetFile: Option<unsafe extern "C" fn(file: *mut FILE) -> cusolverStatus_t>,
    pub cusolverDnLoggerOpenFile: Option<unsafe extern "C" fn(logFile: *const ::std::os::raw::c_char) -> cusolverStatus_t>,
    pub cusolverDnLoggerSetLevel: Option<unsafe extern "C" fn(level: ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnLoggerSetMask: Option<unsafe extern "C" fn(mask: ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverDnLoggerForceDisable: Option<unsafe extern "C" fn() -> cusolverStatus_t>,
    pub cusolverDnXpolar_bufferSize: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            params: cusolverDnParams_t,
            uplo: cublasFillMode_t,
            M: i64,
            N: i64,
            dataTypeA: cudaDataType,
            A: *const ::std::os::raw::c_void,
            lda: i64,
            dataTypeH: cudaDataType,
            H: *const ::std::os::raw::c_void,
            ldh: i64,
            computeType: cudaDataType,
            workspaceInBytesOnDevice: *mut usize,
            workspaceInBytesOnHost: *mut usize,
        ) -> cusolverStatus_t,
    >,
    pub cusolverDnXpolar: Option<
        unsafe extern "C" fn(
            handle: cusolverDnHandle_t,
            params: cusolverDnParams_t,
            uplo: cublasFillMode_t,
            M: i64,
            N: i64,
            dataTypeA: cudaDataType,
            A: *mut ::std::os::raw::c_void,
            lda: i64,
            dataTypeH: cudaDataType,
            H: *mut ::std::os::raw::c_void,
            ldh: i64,
            computeType: cudaDataType,
            bufferOnDevice: *mut ::std::os::raw::c_void,
            workspaceInBytesOnDevice: usize,
            bufferOnHost: *mut ::std::os::raw::c_void,
            workspaceInBytesOnHost: usize,
            d_res_nrm: *mut f64,
            d_A_nrmF: *mut f64,
            d_rcond: *mut f64,
            d_info: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverSpCreate: Option<unsafe extern "C" fn(handle: *mut cusolverSpHandle_t) -> cusolverStatus_t>,
    pub cusolverSpDestroy: Option<unsafe extern "C" fn(handle: cusolverSpHandle_t) -> cusolverStatus_t>,
    pub cusolverSpSetStream: Option<unsafe extern "C" fn(handle: cusolverSpHandle_t, streamId: cudaStream_t) -> cusolverStatus_t>,
    pub cusolverSpGetStream: Option<unsafe extern "C" fn(handle: cusolverSpHandle_t, streamId: *mut cudaStream_t) -> cusolverStatus_t>,
    pub cusolverSpXcsrissymHost: Option<
        unsafe extern "C" fn(
            handle: cusolverSpHandle_t,
            m: ::std::os::raw::c_int,
            nnzA: ::std::os::raw::c_int,
            descrA: cusparseMatDescr_t,
            csrRowPtrA: *const ::std::os::raw::c_int,
            csrEndPtrA: *const ::std::os::raw::c_int,
            csrColIndA: *const ::std::os::raw::c_int,
            issym: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverSpScsrlsvluHost: Option<
        unsafe extern "C" fn(
            handle: cusolverSpHandle_t,
            n: ::std::os::raw::c_int,
            nnzA: ::std::os::raw::c_int,
            descrA: cusparseMatDescr_t,
            csrValA: *const f32,
            csrRowPtrA: *const ::std::os::raw::c_int,
            csrColIndA: *const ::std::os::raw::c_int,
            b: *const f32,
            tol: f32,
            reorder: ::std::os::raw::c_int,
            x: *mut f32,
            singularity: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverSpDcsrlsvluHost: Option<
        unsafe extern "C" fn(
            handle: cusolverSpHandle_t,
            n: ::std::os::raw::c_int,
            nnzA: ::std::os::raw::c_int,
            descrA: cusparseMatDescr_t,
            csrValA: *const f64,
            csrRowPtrA: *const ::std::os::raw::c_int,
            csrColIndA: *const ::std::os::raw::c_int,
            b: *const f64,
            tol: f64,
            reorder: ::std::os::raw::c_int,
            x: *mut f64,
            singularity: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverSpCcsrlsvluHost: Option<
        unsafe extern "C" fn(
            handle: cusolverSpHandle_t,
            n: ::std::os::raw::c_int,
            nnzA: ::std::os::raw::c_int,
            descrA: cusparseMatDescr_t,
            csrValA: *const cuComplex,
            csrRowPtrA: *const ::std::os::raw::c_int,
            csrColIndA: *const ::std::os::raw::c_int,
            b: *const cuComplex,
            tol: f32,
            reorder: ::std::os::raw::c_int,
            x: *mut cuComplex,
            singularity: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverSpZcsrlsvluHost: Option<
        unsafe extern "C" fn(
            handle: cusolverSpHandle_t,
            n: ::std::os::raw::c_int,
            nnzA: ::std::os::raw::c_int,
            descrA: cusparseMatDescr_t,
            csrValA: *const cuDoubleComplex,
            csrRowPtrA: *const ::std::os::raw::c_int,
            csrColIndA: *const ::std::os::raw::c_int,
            b: *const cuDoubleComplex,
            tol: f64,
            reorder: ::std::os::raw::c_int,
            x: *mut cuDoubleComplex,
            singularity: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverSpScsrlsvqr: Option<
        unsafe extern "C" fn(
            handle: cusolverSpHandle_t,
            m: ::std::os::raw::c_int,
            nnz: ::std::os::raw::c_int,
            descrA: cusparseMatDescr_t,
            csrVal: *const f32,
            csrRowPtr: *const ::std::os::raw::c_int,
            csrColInd: *const ::std::os::raw::c_int,
            b: *const f32,
            tol: f32,
            reorder: ::std::os::raw::c_int,
            x: *mut f32,
            singularity: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverSpDcsrlsvqr: Option<
        unsafe extern "C" fn(
            handle: cusolverSpHandle_t,
            m: ::std::os::raw::c_int,
            nnz: ::std::os::raw::c_int,
            descrA: cusparseMatDescr_t,
            csrVal: *const f64,
            csrRowPtr: *const ::std::os::raw::c_int,
            csrColInd: *const ::std::os::raw::c_int,
            b: *const f64,
            tol: f64,
            reorder: ::std::os::raw::c_int,
            x: *mut f64,
            singularity: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverSpCcsrlsvqr: Option<
        unsafe extern "C" fn(
            handle: cusolverSpHandle_t,
            m: ::std::os::raw::c_int,
            nnz: ::std::os::raw::c_int,
            descrA: cusparseMatDescr_t,
            csrVal: *const cuComplex,
            csrRowPtr: *const ::std::os::raw::c_int,
            csrColInd: *const ::std::os::raw::c_int,
            b: *const cuComplex,
            tol: f32,
            reorder: ::std::os::raw::c_int,
            x: *mut cuComplex,
            singularity: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverSpZcsrlsvqr: Option<
        unsafe extern "C" fn(
            handle: cusolverSpHandle_t,
            m: ::std::os::raw::c_int,
            nnz: ::std::os::raw::c_int,
            descrA: cusparseMatDescr_t,
            csrVal: *const cuDoubleComplex,
            csrRowPtr: *const ::std::os::raw::c_int,
            csrColInd: *const ::std::os::raw::c_int,
            b: *const cuDoubleComplex,
            tol: f64,
            reorder: ::std::os::raw::c_int,
            x: *mut cuDoubleComplex,
            singularity: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverSpScsrlsvqrHost: Option<
        unsafe extern "C" fn(
            handle: cusolverSpHandle_t,
            m: ::std::os::raw::c_int,
            nnz: ::std::os::raw::c_int,
            descrA: cusparseMatDescr_t,
            csrValA: *const f32,
            csrRowPtrA: *const ::std::os::raw::c_int,
            csrColIndA: *const ::std::os::raw::c_int,
            b: *const f32,
            tol: f32,
            reorder: ::std::os::raw::c_int,
            x: *mut f32,
            singularity: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverSpDcsrlsvqrHost: Option<
        unsafe extern "C" fn(
            handle: cusolverSpHandle_t,
            m: ::std::os::raw::c_int,
            nnz: ::std::os::raw::c_int,
            descrA: cusparseMatDescr_t,
            csrValA: *const f64,
            csrRowPtrA: *const ::std::os::raw::c_int,
            csrColIndA: *const ::std::os::raw::c_int,
            b: *const f64,
            tol: f64,
            reorder: ::std::os::raw::c_int,
            x: *mut f64,
            singularity: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverSpCcsrlsvqrHost: Option<
        unsafe extern "C" fn(
            handle: cusolverSpHandle_t,
            m: ::std::os::raw::c_int,
            nnz: ::std::os::raw::c_int,
            descrA: cusparseMatDescr_t,
            csrValA: *const cuComplex,
            csrRowPtrA: *const ::std::os::raw::c_int,
            csrColIndA: *const ::std::os::raw::c_int,
            b: *const cuComplex,
            tol: f32,
            reorder: ::std::os::raw::c_int,
            x: *mut cuComplex,
            singularity: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverSpZcsrlsvqrHost: Option<
        unsafe extern "C" fn(
            handle: cusolverSpHandle_t,
            m: ::std::os::raw::c_int,
            nnz: ::std::os::raw::c_int,
            descrA: cusparseMatDescr_t,
            csrValA: *const cuDoubleComplex,
            csrRowPtrA: *const ::std::os::raw::c_int,
            csrColIndA: *const ::std::os::raw::c_int,
            b: *const cuDoubleComplex,
            tol: f64,
            reorder: ::std::os::raw::c_int,
            x: *mut cuDoubleComplex,
            singularity: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverSpScsrlsvcholHost: Option<
        unsafe extern "C" fn(
            handle: cusolverSpHandle_t,
            m: ::std::os::raw::c_int,
            nnz: ::std::os::raw::c_int,
            descrA: cusparseMatDescr_t,
            csrVal: *const f32,
            csrRowPtr: *const ::std::os::raw::c_int,
            csrColInd: *const ::std::os::raw::c_int,
            b: *const f32,
            tol: f32,
            reorder: ::std::os::raw::c_int,
            x: *mut f32,
            singularity: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverSpDcsrlsvcholHost: Option<
        unsafe extern "C" fn(
            handle: cusolverSpHandle_t,
            m: ::std::os::raw::c_int,
            nnz: ::std::os::raw::c_int,
            descrA: cusparseMatDescr_t,
            csrVal: *const f64,
            csrRowPtr: *const ::std::os::raw::c_int,
            csrColInd: *const ::std::os::raw::c_int,
            b: *const f64,
            tol: f64,
            reorder: ::std::os::raw::c_int,
            x: *mut f64,
            singularity: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverSpCcsrlsvcholHost: Option<
        unsafe extern "C" fn(
            handle: cusolverSpHandle_t,
            m: ::std::os::raw::c_int,
            nnz: ::std::os::raw::c_int,
            descrA: cusparseMatDescr_t,
            csrVal: *const cuComplex,
            csrRowPtr: *const ::std::os::raw::c_int,
            csrColInd: *const ::std::os::raw::c_int,
            b: *const cuComplex,
            tol: f32,
            reorder: ::std::os::raw::c_int,
            x: *mut cuComplex,
            singularity: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverSpZcsrlsvcholHost: Option<
        unsafe extern "C" fn(
            handle: cusolverSpHandle_t,
            m: ::std::os::raw::c_int,
            nnz: ::std::os::raw::c_int,
            descrA: cusparseMatDescr_t,
            csrVal: *const cuDoubleComplex,
            csrRowPtr: *const ::std::os::raw::c_int,
            csrColInd: *const ::std::os::raw::c_int,
            b: *const cuDoubleComplex,
            tol: f64,
            reorder: ::std::os::raw::c_int,
            x: *mut cuDoubleComplex,
            singularity: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverSpScsrlsvchol: Option<
        unsafe extern "C" fn(
            handle: cusolverSpHandle_t,
            m: ::std::os::raw::c_int,
            nnz: ::std::os::raw::c_int,
            descrA: cusparseMatDescr_t,
            csrVal: *const f32,
            csrRowPtr: *const ::std::os::raw::c_int,
            csrColInd: *const ::std::os::raw::c_int,
            b: *const f32,
            tol: f32,
            reorder: ::std::os::raw::c_int,
            x: *mut f32,
            singularity: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverSpDcsrlsvchol: Option<
        unsafe extern "C" fn(
            handle: cusolverSpHandle_t,
            m: ::std::os::raw::c_int,
            nnz: ::std::os::raw::c_int,
            descrA: cusparseMatDescr_t,
            csrVal: *const f64,
            csrRowPtr: *const ::std::os::raw::c_int,
            csrColInd: *const ::std::os::raw::c_int,
            b: *const f64,
            tol: f64,
            reorder: ::std::os::raw::c_int,
            x: *mut f64,
            singularity: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverSpCcsrlsvchol: Option<
        unsafe extern "C" fn(
            handle: cusolverSpHandle_t,
            m: ::std::os::raw::c_int,
            nnz: ::std::os::raw::c_int,
            descrA: cusparseMatDescr_t,
            csrVal: *const cuComplex,
            csrRowPtr: *const ::std::os::raw::c_int,
            csrColInd: *const ::std::os::raw::c_int,
            b: *const cuComplex,
            tol: f32,
            reorder: ::std::os::raw::c_int,
            x: *mut cuComplex,
            singularity: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverSpZcsrlsvchol: Option<
        unsafe extern "C" fn(
            handle: cusolverSpHandle_t,
            m: ::std::os::raw::c_int,
            nnz: ::std::os::raw::c_int,
            descrA: cusparseMatDescr_t,
            csrVal: *const cuDoubleComplex,
            csrRowPtr: *const ::std::os::raw::c_int,
            csrColInd: *const ::std::os::raw::c_int,
            b: *const cuDoubleComplex,
            tol: f64,
            reorder: ::std::os::raw::c_int,
            x: *mut cuDoubleComplex,
            singularity: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverSpScsrlsqvqrHost: Option<
        unsafe extern "C" fn(
            handle: cusolverSpHandle_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            nnz: ::std::os::raw::c_int,
            descrA: cusparseMatDescr_t,
            csrValA: *const f32,
            csrRowPtrA: *const ::std::os::raw::c_int,
            csrColIndA: *const ::std::os::raw::c_int,
            b: *const f32,
            tol: f32,
            rankA: *mut ::std::os::raw::c_int,
            x: *mut f32,
            p: *mut ::std::os::raw::c_int,
            min_norm: *mut f32,
        ) -> cusolverStatus_t,
    >,
    pub cusolverSpDcsrlsqvqrHost: Option<
        unsafe extern "C" fn(
            handle: cusolverSpHandle_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            nnz: ::std::os::raw::c_int,
            descrA: cusparseMatDescr_t,
            csrValA: *const f64,
            csrRowPtrA: *const ::std::os::raw::c_int,
            csrColIndA: *const ::std::os::raw::c_int,
            b: *const f64,
            tol: f64,
            rankA: *mut ::std::os::raw::c_int,
            x: *mut f64,
            p: *mut ::std::os::raw::c_int,
            min_norm: *mut f64,
        ) -> cusolverStatus_t,
    >,
    pub cusolverSpCcsrlsqvqrHost: Option<
        unsafe extern "C" fn(
            handle: cusolverSpHandle_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            nnz: ::std::os::raw::c_int,
            descrA: cusparseMatDescr_t,
            csrValA: *const cuComplex,
            csrRowPtrA: *const ::std::os::raw::c_int,
            csrColIndA: *const ::std::os::raw::c_int,
            b: *const cuComplex,
            tol: f32,
            rankA: *mut ::std::os::raw::c_int,
            x: *mut cuComplex,
            p: *mut ::std::os::raw::c_int,
            min_norm: *mut f32,
        ) -> cusolverStatus_t,
    >,
    pub cusolverSpZcsrlsqvqrHost: Option<
        unsafe extern "C" fn(
            handle: cusolverSpHandle_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            nnz: ::std::os::raw::c_int,
            descrA: cusparseMatDescr_t,
            csrValA: *const cuDoubleComplex,
            csrRowPtrA: *const ::std::os::raw::c_int,
            csrColIndA: *const ::std::os::raw::c_int,
            b: *const cuDoubleComplex,
            tol: f64,
            rankA: *mut ::std::os::raw::c_int,
            x: *mut cuDoubleComplex,
            p: *mut ::std::os::raw::c_int,
            min_norm: *mut f64,
        ) -> cusolverStatus_t,
    >,
    pub cusolverSpScsreigvsiHost: Option<
        unsafe extern "C" fn(
            handle: cusolverSpHandle_t,
            m: ::std::os::raw::c_int,
            nnz: ::std::os::raw::c_int,
            descrA: cusparseMatDescr_t,
            csrValA: *const f32,
            csrRowPtrA: *const ::std::os::raw::c_int,
            csrColIndA: *const ::std::os::raw::c_int,
            mu0: f32,
            x0: *const f32,
            maxite: ::std::os::raw::c_int,
            tol: f32,
            mu: *mut f32,
            x: *mut f32,
        ) -> cusolverStatus_t,
    >,
    pub cusolverSpDcsreigvsiHost: Option<
        unsafe extern "C" fn(
            handle: cusolverSpHandle_t,
            m: ::std::os::raw::c_int,
            nnz: ::std::os::raw::c_int,
            descrA: cusparseMatDescr_t,
            csrValA: *const f64,
            csrRowPtrA: *const ::std::os::raw::c_int,
            csrColIndA: *const ::std::os::raw::c_int,
            mu0: f64,
            x0: *const f64,
            maxite: ::std::os::raw::c_int,
            tol: f64,
            mu: *mut f64,
            x: *mut f64,
        ) -> cusolverStatus_t,
    >,
    pub cusolverSpCcsreigvsiHost: Option<
        unsafe extern "C" fn(
            handle: cusolverSpHandle_t,
            m: ::std::os::raw::c_int,
            nnz: ::std::os::raw::c_int,
            descrA: cusparseMatDescr_t,
            csrValA: *const cuComplex,
            csrRowPtrA: *const ::std::os::raw::c_int,
            csrColIndA: *const ::std::os::raw::c_int,
            mu0: cuComplex,
            x0: *const cuComplex,
            maxite: ::std::os::raw::c_int,
            tol: f32,
            mu: *mut cuComplex,
            x: *mut cuComplex,
        ) -> cusolverStatus_t,
    >,
    pub cusolverSpZcsreigvsiHost: Option<
        unsafe extern "C" fn(
            handle: cusolverSpHandle_t,
            m: ::std::os::raw::c_int,
            nnz: ::std::os::raw::c_int,
            descrA: cusparseMatDescr_t,
            csrValA: *const cuDoubleComplex,
            csrRowPtrA: *const ::std::os::raw::c_int,
            csrColIndA: *const ::std::os::raw::c_int,
            mu0: cuDoubleComplex,
            x0: *const cuDoubleComplex,
            maxite: ::std::os::raw::c_int,
            tol: f64,
            mu: *mut cuDoubleComplex,
            x: *mut cuDoubleComplex,
        ) -> cusolverStatus_t,
    >,
    pub cusolverSpScsreigvsi: Option<
        unsafe extern "C" fn(
            handle: cusolverSpHandle_t,
            m: ::std::os::raw::c_int,
            nnz: ::std::os::raw::c_int,
            descrA: cusparseMatDescr_t,
            csrValA: *const f32,
            csrRowPtrA: *const ::std::os::raw::c_int,
            csrColIndA: *const ::std::os::raw::c_int,
            mu0: f32,
            x0: *const f32,
            maxite: ::std::os::raw::c_int,
            eps: f32,
            mu: *mut f32,
            x: *mut f32,
        ) -> cusolverStatus_t,
    >,
    pub cusolverSpDcsreigvsi: Option<
        unsafe extern "C" fn(
            handle: cusolverSpHandle_t,
            m: ::std::os::raw::c_int,
            nnz: ::std::os::raw::c_int,
            descrA: cusparseMatDescr_t,
            csrValA: *const f64,
            csrRowPtrA: *const ::std::os::raw::c_int,
            csrColIndA: *const ::std::os::raw::c_int,
            mu0: f64,
            x0: *const f64,
            maxite: ::std::os::raw::c_int,
            eps: f64,
            mu: *mut f64,
            x: *mut f64,
        ) -> cusolverStatus_t,
    >,
    pub cusolverSpCcsreigvsi: Option<
        unsafe extern "C" fn(
            handle: cusolverSpHandle_t,
            m: ::std::os::raw::c_int,
            nnz: ::std::os::raw::c_int,
            descrA: cusparseMatDescr_t,
            csrValA: *const cuComplex,
            csrRowPtrA: *const ::std::os::raw::c_int,
            csrColIndA: *const ::std::os::raw::c_int,
            mu0: cuComplex,
            x0: *const cuComplex,
            maxite: ::std::os::raw::c_int,
            eps: f32,
            mu: *mut cuComplex,
            x: *mut cuComplex,
        ) -> cusolverStatus_t,
    >,
    pub cusolverSpZcsreigvsi: Option<
        unsafe extern "C" fn(
            handle: cusolverSpHandle_t,
            m: ::std::os::raw::c_int,
            nnz: ::std::os::raw::c_int,
            descrA: cusparseMatDescr_t,
            csrValA: *const cuDoubleComplex,
            csrRowPtrA: *const ::std::os::raw::c_int,
            csrColIndA: *const ::std::os::raw::c_int,
            mu0: cuDoubleComplex,
            x0: *const cuDoubleComplex,
            maxite: ::std::os::raw::c_int,
            eps: f64,
            mu: *mut cuDoubleComplex,
            x: *mut cuDoubleComplex,
        ) -> cusolverStatus_t,
    >,
    pub cusolverSpScsreigsHost: Option<
        unsafe extern "C" fn(
            handle: cusolverSpHandle_t,
            m: ::std::os::raw::c_int,
            nnz: ::std::os::raw::c_int,
            descrA: cusparseMatDescr_t,
            csrValA: *const f32,
            csrRowPtrA: *const ::std::os::raw::c_int,
            csrColIndA: *const ::std::os::raw::c_int,
            left_bottom_corner: cuComplex,
            right_upper_corner: cuComplex,
            num_eigs: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverSpDcsreigsHost: Option<
        unsafe extern "C" fn(
            handle: cusolverSpHandle_t,
            m: ::std::os::raw::c_int,
            nnz: ::std::os::raw::c_int,
            descrA: cusparseMatDescr_t,
            csrValA: *const f64,
            csrRowPtrA: *const ::std::os::raw::c_int,
            csrColIndA: *const ::std::os::raw::c_int,
            left_bottom_corner: cuDoubleComplex,
            right_upper_corner: cuDoubleComplex,
            num_eigs: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverSpCcsreigsHost: Option<
        unsafe extern "C" fn(
            handle: cusolverSpHandle_t,
            m: ::std::os::raw::c_int,
            nnz: ::std::os::raw::c_int,
            descrA: cusparseMatDescr_t,
            csrValA: *const cuComplex,
            csrRowPtrA: *const ::std::os::raw::c_int,
            csrColIndA: *const ::std::os::raw::c_int,
            left_bottom_corner: cuComplex,
            right_upper_corner: cuComplex,
            num_eigs: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverSpZcsreigsHost: Option<
        unsafe extern "C" fn(
            handle: cusolverSpHandle_t,
            m: ::std::os::raw::c_int,
            nnz: ::std::os::raw::c_int,
            descrA: cusparseMatDescr_t,
            csrValA: *const cuDoubleComplex,
            csrRowPtrA: *const ::std::os::raw::c_int,
            csrColIndA: *const ::std::os::raw::c_int,
            left_bottom_corner: cuDoubleComplex,
            right_upper_corner: cuDoubleComplex,
            num_eigs: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverSpXcsrsymrcmHost:
        Option<unsafe extern "C" fn(handle: cusolverSpHandle_t, n: ::std::os::raw::c_int, nnzA: ::std::os::raw::c_int, descrA: cusparseMatDescr_t, csrRowPtrA: *const ::std::os::raw::c_int, csrColIndA: *const ::std::os::raw::c_int, p: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverSpXcsrsymmdqHost:
        Option<unsafe extern "C" fn(handle: cusolverSpHandle_t, n: ::std::os::raw::c_int, nnzA: ::std::os::raw::c_int, descrA: cusparseMatDescr_t, csrRowPtrA: *const ::std::os::raw::c_int, csrColIndA: *const ::std::os::raw::c_int, p: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverSpXcsrsymamdHost:
        Option<unsafe extern "C" fn(handle: cusolverSpHandle_t, n: ::std::os::raw::c_int, nnzA: ::std::os::raw::c_int, descrA: cusparseMatDescr_t, csrRowPtrA: *const ::std::os::raw::c_int, csrColIndA: *const ::std::os::raw::c_int, p: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    pub cusolverSpXcsrmetisndHost: Option<
        unsafe extern "C" fn(handle: cusolverSpHandle_t, n: ::std::os::raw::c_int, nnzA: ::std::os::raw::c_int, descrA: cusparseMatDescr_t, csrRowPtrA: *const ::std::os::raw::c_int, csrColIndA: *const ::std::os::raw::c_int, options: *const i64, p: *mut ::std::os::raw::c_int) -> cusolverStatus_t,
    >,
    pub cusolverSpScsrzfdHost: Option<
        unsafe extern "C" fn(
            handle: cusolverSpHandle_t,
            n: ::std::os::raw::c_int,
            nnz: ::std::os::raw::c_int,
            descrA: cusparseMatDescr_t,
            csrValA: *const f32,
            csrRowPtrA: *const ::std::os::raw::c_int,
            csrColIndA: *const ::std::os::raw::c_int,
            P: *mut ::std::os::raw::c_int,
            numnz: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverSpDcsrzfdHost: Option<
        unsafe extern "C" fn(
            handle: cusolverSpHandle_t,
            n: ::std::os::raw::c_int,
            nnz: ::std::os::raw::c_int,
            descrA: cusparseMatDescr_t,
            csrValA: *const f64,
            csrRowPtrA: *const ::std::os::raw::c_int,
            csrColIndA: *const ::std::os::raw::c_int,
            P: *mut ::std::os::raw::c_int,
            numnz: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverSpCcsrzfdHost: Option<
        unsafe extern "C" fn(
            handle: cusolverSpHandle_t,
            n: ::std::os::raw::c_int,
            nnz: ::std::os::raw::c_int,
            descrA: cusparseMatDescr_t,
            csrValA: *const cuComplex,
            csrRowPtrA: *const ::std::os::raw::c_int,
            csrColIndA: *const ::std::os::raw::c_int,
            P: *mut ::std::os::raw::c_int,
            numnz: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverSpZcsrzfdHost: Option<
        unsafe extern "C" fn(
            handle: cusolverSpHandle_t,
            n: ::std::os::raw::c_int,
            nnz: ::std::os::raw::c_int,
            descrA: cusparseMatDescr_t,
            csrValA: *const cuDoubleComplex,
            csrRowPtrA: *const ::std::os::raw::c_int,
            csrColIndA: *const ::std::os::raw::c_int,
            P: *mut ::std::os::raw::c_int,
            numnz: *mut ::std::os::raw::c_int,
        ) -> cusolverStatus_t,
    >,
    pub cusolverSpXcsrperm_bufferSizeHost: Option<
        unsafe extern "C" fn(
            handle: cusolverSpHandle_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            nnzA: ::std::os::raw::c_int,
            descrA: cusparseMatDescr_t,
            csrRowPtrA: *const ::std::os::raw::c_int,
            csrColIndA: *const ::std::os::raw::c_int,
            p: *const ::std::os::raw::c_int,
            q: *const ::std::os::raw::c_int,
            bufferSizeInBytes: *mut usize,
        ) -> cusolverStatus_t,
    >,
    pub cusolverSpXcsrpermHost: Option<
        unsafe extern "C" fn(
            handle: cusolverSpHandle_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            nnzA: ::std::os::raw::c_int,
            descrA: cusparseMatDescr_t,
            csrRowPtrA: *mut ::std::os::raw::c_int,
            csrColIndA: *mut ::std::os::raw::c_int,
            p: *const ::std::os::raw::c_int,
            q: *const ::std::os::raw::c_int,
            map: *mut ::std::os::raw::c_int,
            pBuffer: *mut ::std::os::raw::c_void,
        ) -> cusolverStatus_t,
    >,
    pub cusolverSpCreateCsrqrInfo: Option<unsafe extern "C" fn(info: *mut csrqrInfo_t) -> cusolverStatus_t>,
    pub cusolverSpDestroyCsrqrInfo: Option<unsafe extern "C" fn(info: csrqrInfo_t) -> cusolverStatus_t>,
    pub cusolverSpXcsrqrAnalysisBatched:
        Option<unsafe extern "C" fn(handle: cusolverSpHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, nnzA: ::std::os::raw::c_int, descrA: cusparseMatDescr_t, csrRowPtrA: *const ::std::os::raw::c_int, csrColIndA: *const ::std::os::raw::c_int, info: csrqrInfo_t) -> cusolverStatus_t>,
    pub cusolverSpScsrqrBufferInfoBatched: Option<
        unsafe extern "C" fn(
            handle: cusolverSpHandle_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            nnz: ::std::os::raw::c_int,
            descrA: cusparseMatDescr_t,
            csrVal: *const f32,
            csrRowPtr: *const ::std::os::raw::c_int,
            csrColInd: *const ::std::os::raw::c_int,
            batchSize: ::std::os::raw::c_int,
            info: csrqrInfo_t,
            internalDataInBytes: *mut usize,
            workspaceInBytes: *mut usize,
        ) -> cusolverStatus_t,
    >,
    pub cusolverSpDcsrqrBufferInfoBatched: Option<
        unsafe extern "C" fn(
            handle: cusolverSpHandle_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            nnz: ::std::os::raw::c_int,
            descrA: cusparseMatDescr_t,
            csrVal: *const f64,
            csrRowPtr: *const ::std::os::raw::c_int,
            csrColInd: *const ::std::os::raw::c_int,
            batchSize: ::std::os::raw::c_int,
            info: csrqrInfo_t,
            internalDataInBytes: *mut usize,
            workspaceInBytes: *mut usize,
        ) -> cusolverStatus_t,
    >,
    pub cusolverSpCcsrqrBufferInfoBatched: Option<
        unsafe extern "C" fn(
            handle: cusolverSpHandle_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            nnz: ::std::os::raw::c_int,
            descrA: cusparseMatDescr_t,
            csrVal: *const cuComplex,
            csrRowPtr: *const ::std::os::raw::c_int,
            csrColInd: *const ::std::os::raw::c_int,
            batchSize: ::std::os::raw::c_int,
            info: csrqrInfo_t,
            internalDataInBytes: *mut usize,
            workspaceInBytes: *mut usize,
        ) -> cusolverStatus_t,
    >,
    pub cusolverSpZcsrqrBufferInfoBatched: Option<
        unsafe extern "C" fn(
            handle: cusolverSpHandle_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            nnz: ::std::os::raw::c_int,
            descrA: cusparseMatDescr_t,
            csrVal: *const cuDoubleComplex,
            csrRowPtr: *const ::std::os::raw::c_int,
            csrColInd: *const ::std::os::raw::c_int,
            batchSize: ::std::os::raw::c_int,
            info: csrqrInfo_t,
            internalDataInBytes: *mut usize,
            workspaceInBytes: *mut usize,
        ) -> cusolverStatus_t,
    >,
    pub cusolverSpScsrqrsvBatched: Option<
        unsafe extern "C" fn(
            handle: cusolverSpHandle_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            nnz: ::std::os::raw::c_int,
            descrA: cusparseMatDescr_t,
            csrValA: *const f32,
            csrRowPtrA: *const ::std::os::raw::c_int,
            csrColIndA: *const ::std::os::raw::c_int,
            b: *const f32,
            x: *mut f32,
            batchSize: ::std::os::raw::c_int,
            info: csrqrInfo_t,
            pBuffer: *mut ::std::os::raw::c_void,
        ) -> cusolverStatus_t,
    >,
    pub cusolverSpDcsrqrsvBatched: Option<
        unsafe extern "C" fn(
            handle: cusolverSpHandle_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            nnz: ::std::os::raw::c_int,
            descrA: cusparseMatDescr_t,
            csrValA: *const f64,
            csrRowPtrA: *const ::std::os::raw::c_int,
            csrColIndA: *const ::std::os::raw::c_int,
            b: *const f64,
            x: *mut f64,
            batchSize: ::std::os::raw::c_int,
            info: csrqrInfo_t,
            pBuffer: *mut ::std::os::raw::c_void,
        ) -> cusolverStatus_t,
    >,
    pub cusolverSpCcsrqrsvBatched: Option<
        unsafe extern "C" fn(
            handle: cusolverSpHandle_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            nnz: ::std::os::raw::c_int,
            descrA: cusparseMatDescr_t,
            csrValA: *const cuComplex,
            csrRowPtrA: *const ::std::os::raw::c_int,
            csrColIndA: *const ::std::os::raw::c_int,
            b: *const cuComplex,
            x: *mut cuComplex,
            batchSize: ::std::os::raw::c_int,
            info: csrqrInfo_t,
            pBuffer: *mut ::std::os::raw::c_void,
        ) -> cusolverStatus_t,
    >,
    pub cusolverSpZcsrqrsvBatched: Option<
        unsafe extern "C" fn(
            handle: cusolverSpHandle_t,
            m: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            nnz: ::std::os::raw::c_int,
            descrA: cusparseMatDescr_t,
            csrValA: *const cuDoubleComplex,
            csrRowPtrA: *const ::std::os::raw::c_int,
            csrColIndA: *const ::std::os::raw::c_int,
            b: *const cuDoubleComplex,
            x: *mut cuDoubleComplex,
            batchSize: ::std::os::raw::c_int,
            info: csrqrInfo_t,
            pBuffer: *mut ::std::os::raw::c_void,
        ) -> cusolverStatus_t,
    >,
}
#[cfg(feature = "runtime-link")]
pub static DYNAMIC_BINDINGS: std::sync::OnceLock<Box<DynamicBindings>> = std::sync::OnceLock::new();
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverGetProperty(type_: libraryPropertyType, value: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverGetProperty {
        Some(____func) => unsafe { ____func(type_, value) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverGetProperty"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverGetVersion(version: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverGetVersion {
        Some(____func) => unsafe { ____func(version) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverGetVersion"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnCreate(handle: *mut cusolverDnHandle_t) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnCreate {
        Some(____func) => unsafe { ____func(handle) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnCreate"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDestroy(handle: cusolverDnHandle_t) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDestroy {
        Some(____func) => unsafe { ____func(handle) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnDestroy"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSetStream(handle: cusolverDnHandle_t, streamId: cudaStream_t) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSetStream {
        Some(____func) => unsafe { ____func(handle, streamId) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnSetStream"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnGetStream(handle: cusolverDnHandle_t, streamId: *mut cudaStream_t) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnGetStream {
        Some(____func) => unsafe { ____func(handle, streamId) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnGetStream"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSetDeterministicMode(handle: cusolverDnHandle_t, mode: cusolverDeterministicMode_t) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSetDeterministicMode {
        Some(____func) => unsafe { ____func(handle, mode) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnSetDeterministicMode"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnGetDeterministicMode(handle: cusolverDnHandle_t, mode: *mut cusolverDeterministicMode_t) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnGetDeterministicMode {
        Some(____func) => unsafe { ____func(handle, mode) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnGetDeterministicMode"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSetMathMode(handle: cusolverDnHandle_t, mode: cusolverMathMode_t) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSetMathMode {
        Some(____func) => unsafe { ____func(handle, mode) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnSetMathMode"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnGetMathMode(handle: cusolverDnHandle_t, mode: *mut cusolverMathMode_t) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnGetMathMode {
        Some(____func) => unsafe { ____func(handle, mode) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnGetMathMode"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSetEmulationStrategy(handle: cusolverDnHandle_t, strategy: cudaEmulationStrategy_t) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSetEmulationStrategy {
        Some(____func) => unsafe { ____func(handle, strategy) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnSetEmulationStrategy"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnGetEmulationStrategy(handle: cusolverDnHandle_t, strategy: *mut cudaEmulationStrategy_t) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnGetEmulationStrategy {
        Some(____func) => unsafe { ____func(handle, strategy) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnGetEmulationStrategy"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSetFixedPointEmulationMantissaControl(handle: cusolverDnHandle_t, control: cudaEmulationMantissaControl_t) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSetFixedPointEmulationMantissaControl {
        Some(____func) => unsafe { ____func(handle, control) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnSetFixedPointEmulationMantissaControl"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnGetFixedPointEmulationMantissaControl(handle: cusolverDnHandle_t, control: *mut cudaEmulationMantissaControl_t) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnGetFixedPointEmulationMantissaControl {
        Some(____func) => unsafe { ____func(handle, control) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnGetFixedPointEmulationMantissaControl"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSetFixedPointEmulationMaxMantissaBitCount(handle: cusolverDnHandle_t, mantissaBitCount: ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSetFixedPointEmulationMaxMantissaBitCount {
        Some(____func) => unsafe { ____func(handle, mantissaBitCount) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnSetFixedPointEmulationMaxMantissaBitCount"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnGetFixedPointEmulationMaxMantissaBitCount(handle: cusolverDnHandle_t, mantissaBitCount: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnGetFixedPointEmulationMaxMantissaBitCount {
        Some(____func) => unsafe { ____func(handle, mantissaBitCount) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnGetFixedPointEmulationMaxMantissaBitCount"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSetFixedPointEmulationMantissaBitOffset(handle: cusolverDnHandle_t, mantissaBitOffset: ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSetFixedPointEmulationMantissaBitOffset {
        Some(____func) => unsafe { ____func(handle, mantissaBitOffset) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnSetFixedPointEmulationMantissaBitOffset"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnGetFixedPointEmulationMantissaBitOffset(handle: cusolverDnHandle_t, mantissaBitOffset: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnGetFixedPointEmulationMantissaBitOffset {
        Some(____func) => unsafe { ____func(handle, mantissaBitOffset) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnGetFixedPointEmulationMantissaBitOffset"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSetEmulationSpecialValuesSupport(handle: cusolverDnHandle_t, mask: cudaEmulationSpecialValuesSupport_t) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSetEmulationSpecialValuesSupport {
        Some(____func) => unsafe { ____func(handle, mask) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnSetEmulationSpecialValuesSupport"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnGetEmulationSpecialValuesSupport(handle: cusolverDnHandle_t, mask: *mut cudaEmulationSpecialValuesSupport_t) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnGetEmulationSpecialValuesSupport {
        Some(____func) => unsafe { ____func(handle, mask) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnGetEmulationSpecialValuesSupport"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnIRSParamsCreate(params_ptr: *mut cusolverDnIRSParams_t) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnIRSParamsCreate {
        Some(____func) => unsafe { ____func(params_ptr) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnIRSParamsCreate"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnIRSParamsDestroy(params: cusolverDnIRSParams_t) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnIRSParamsDestroy {
        Some(____func) => unsafe { ____func(params) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnIRSParamsDestroy"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnIRSParamsSetRefinementSolver(params: cusolverDnIRSParams_t, refinement_solver: cusolverIRSRefinement_t) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnIRSParamsSetRefinementSolver {
        Some(____func) => unsafe { ____func(params, refinement_solver) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnIRSParamsSetRefinementSolver"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnIRSParamsSetSolverMainPrecision(params: cusolverDnIRSParams_t, solver_main_precision: cusolverPrecType_t) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnIRSParamsSetSolverMainPrecision {
        Some(____func) => unsafe { ____func(params, solver_main_precision) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnIRSParamsSetSolverMainPrecision"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnIRSParamsSetSolverLowestPrecision(params: cusolverDnIRSParams_t, solver_lowest_precision: cusolverPrecType_t) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnIRSParamsSetSolverLowestPrecision {
        Some(____func) => unsafe { ____func(params, solver_lowest_precision) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnIRSParamsSetSolverLowestPrecision"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnIRSParamsSetSolverPrecisions(params: cusolverDnIRSParams_t, solver_main_precision: cusolverPrecType_t, solver_lowest_precision: cusolverPrecType_t) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnIRSParamsSetSolverPrecisions {
        Some(____func) => unsafe { ____func(params, solver_main_precision, solver_lowest_precision) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnIRSParamsSetSolverPrecisions"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnIRSParamsSetTol(params: cusolverDnIRSParams_t, val: f64) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnIRSParamsSetTol {
        Some(____func) => unsafe { ____func(params, val) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnIRSParamsSetTol"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnIRSParamsSetTolInner(params: cusolverDnIRSParams_t, val: f64) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnIRSParamsSetTolInner {
        Some(____func) => unsafe { ____func(params, val) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnIRSParamsSetTolInner"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnIRSParamsSetMaxIters(params: cusolverDnIRSParams_t, maxiters: cusolver_int_t) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnIRSParamsSetMaxIters {
        Some(____func) => unsafe { ____func(params, maxiters) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnIRSParamsSetMaxIters"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnIRSParamsSetMaxItersInner(params: cusolverDnIRSParams_t, maxiters_inner: cusolver_int_t) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnIRSParamsSetMaxItersInner {
        Some(____func) => unsafe { ____func(params, maxiters_inner) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnIRSParamsSetMaxItersInner"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnIRSParamsGetMaxIters(params: cusolverDnIRSParams_t, maxiters: *mut cusolver_int_t) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnIRSParamsGetMaxIters {
        Some(____func) => unsafe { ____func(params, maxiters) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnIRSParamsGetMaxIters"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnIRSParamsEnableFallback(params: cusolverDnIRSParams_t) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnIRSParamsEnableFallback {
        Some(____func) => unsafe { ____func(params) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnIRSParamsEnableFallback"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnIRSParamsDisableFallback(params: cusolverDnIRSParams_t) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnIRSParamsDisableFallback {
        Some(____func) => unsafe { ____func(params) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnIRSParamsDisableFallback"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnIRSInfosDestroy(infos: cusolverDnIRSInfos_t) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnIRSInfosDestroy {
        Some(____func) => unsafe { ____func(infos) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnIRSInfosDestroy"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnIRSInfosCreate(infos_ptr: *mut cusolverDnIRSInfos_t) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnIRSInfosCreate {
        Some(____func) => unsafe { ____func(infos_ptr) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnIRSInfosCreate"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnIRSInfosGetNiters(infos: cusolverDnIRSInfos_t, niters: *mut cusolver_int_t) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnIRSInfosGetNiters {
        Some(____func) => unsafe { ____func(infos, niters) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnIRSInfosGetNiters"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnIRSInfosGetOuterNiters(infos: cusolverDnIRSInfos_t, outer_niters: *mut cusolver_int_t) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnIRSInfosGetOuterNiters {
        Some(____func) => unsafe { ____func(infos, outer_niters) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnIRSInfosGetOuterNiters"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnIRSInfosRequestResidual(infos: cusolverDnIRSInfos_t) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnIRSInfosRequestResidual {
        Some(____func) => unsafe { ____func(infos) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnIRSInfosRequestResidual"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnIRSInfosGetResidualHistory(infos: cusolverDnIRSInfos_t, residual_history: *mut *mut ::std::os::raw::c_void) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnIRSInfosGetResidualHistory {
        Some(____func) => unsafe { ____func(infos, residual_history) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnIRSInfosGetResidualHistory"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnIRSInfosGetMaxIters(infos: cusolverDnIRSInfos_t, maxiters: *mut cusolver_int_t) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnIRSInfosGetMaxIters {
        Some(____func) => unsafe { ____func(infos, maxiters) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnIRSInfosGetMaxIters"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZZgesv(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuDoubleComplex,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut cuDoubleComplex,
    lddb: cusolver_int_t,
    dX: *mut cuDoubleComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZZgesv {
        Some(____func) => unsafe { ____func(handle, n, nrhs, dA, ldda, dipiv, dB, lddb, dX, lddx, dWorkspace, lwork_bytes, iter, d_info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnZZgesv"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZCgesv(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuDoubleComplex,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut cuDoubleComplex,
    lddb: cusolver_int_t,
    dX: *mut cuDoubleComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZCgesv {
        Some(____func) => unsafe { ____func(handle, n, nrhs, dA, ldda, dipiv, dB, lddb, dX, lddx, dWorkspace, lwork_bytes, iter, d_info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnZCgesv"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZKgesv(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuDoubleComplex,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut cuDoubleComplex,
    lddb: cusolver_int_t,
    dX: *mut cuDoubleComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZKgesv {
        Some(____func) => unsafe { ____func(handle, n, nrhs, dA, ldda, dipiv, dB, lddb, dX, lddx, dWorkspace, lwork_bytes, iter, d_info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnZKgesv"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZEgesv(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuDoubleComplex,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut cuDoubleComplex,
    lddb: cusolver_int_t,
    dX: *mut cuDoubleComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZEgesv {
        Some(____func) => unsafe { ____func(handle, n, nrhs, dA, ldda, dipiv, dB, lddb, dX, lddx, dWorkspace, lwork_bytes, iter, d_info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnZEgesv"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZYgesv(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuDoubleComplex,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut cuDoubleComplex,
    lddb: cusolver_int_t,
    dX: *mut cuDoubleComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZYgesv {
        Some(____func) => unsafe { ____func(handle, n, nrhs, dA, ldda, dipiv, dB, lddb, dX, lddx, dWorkspace, lwork_bytes, iter, d_info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnZYgesv"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnCCgesv(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuComplex,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut cuComplex,
    lddb: cusolver_int_t,
    dX: *mut cuComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnCCgesv {
        Some(____func) => unsafe { ____func(handle, n, nrhs, dA, ldda, dipiv, dB, lddb, dX, lddx, dWorkspace, lwork_bytes, iter, d_info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnCCgesv"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnCEgesv(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuComplex,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut cuComplex,
    lddb: cusolver_int_t,
    dX: *mut cuComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnCEgesv {
        Some(____func) => unsafe { ____func(handle, n, nrhs, dA, ldda, dipiv, dB, lddb, dX, lddx, dWorkspace, lwork_bytes, iter, d_info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnCEgesv"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnCKgesv(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuComplex,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut cuComplex,
    lddb: cusolver_int_t,
    dX: *mut cuComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnCKgesv {
        Some(____func) => unsafe { ____func(handle, n, nrhs, dA, ldda, dipiv, dB, lddb, dX, lddx, dWorkspace, lwork_bytes, iter, d_info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnCKgesv"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnCYgesv(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuComplex,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut cuComplex,
    lddb: cusolver_int_t,
    dX: *mut cuComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnCYgesv {
        Some(____func) => unsafe { ____func(handle, n, nrhs, dA, ldda, dipiv, dB, lddb, dX, lddx, dWorkspace, lwork_bytes, iter, d_info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnCYgesv"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDDgesv(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f64,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut f64,
    lddb: cusolver_int_t,
    dX: *mut f64,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDDgesv {
        Some(____func) => unsafe { ____func(handle, n, nrhs, dA, ldda, dipiv, dB, lddb, dX, lddx, dWorkspace, lwork_bytes, iter, d_info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnDDgesv"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDSgesv(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f64,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut f64,
    lddb: cusolver_int_t,
    dX: *mut f64,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDSgesv {
        Some(____func) => unsafe { ____func(handle, n, nrhs, dA, ldda, dipiv, dB, lddb, dX, lddx, dWorkspace, lwork_bytes, iter, d_info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnDSgesv"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDHgesv(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f64,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut f64,
    lddb: cusolver_int_t,
    dX: *mut f64,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDHgesv {
        Some(____func) => unsafe { ____func(handle, n, nrhs, dA, ldda, dipiv, dB, lddb, dX, lddx, dWorkspace, lwork_bytes, iter, d_info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnDHgesv"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDBgesv(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f64,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut f64,
    lddb: cusolver_int_t,
    dX: *mut f64,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDBgesv {
        Some(____func) => unsafe { ____func(handle, n, nrhs, dA, ldda, dipiv, dB, lddb, dX, lddx, dWorkspace, lwork_bytes, iter, d_info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnDBgesv"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDXgesv(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f64,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut f64,
    lddb: cusolver_int_t,
    dX: *mut f64,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDXgesv {
        Some(____func) => unsafe { ____func(handle, n, nrhs, dA, ldda, dipiv, dB, lddb, dX, lddx, dWorkspace, lwork_bytes, iter, d_info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnDXgesv"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSSgesv(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f32,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut f32,
    lddb: cusolver_int_t,
    dX: *mut f32,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSSgesv {
        Some(____func) => unsafe { ____func(handle, n, nrhs, dA, ldda, dipiv, dB, lddb, dX, lddx, dWorkspace, lwork_bytes, iter, d_info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnSSgesv"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSHgesv(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f32,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut f32,
    lddb: cusolver_int_t,
    dX: *mut f32,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSHgesv {
        Some(____func) => unsafe { ____func(handle, n, nrhs, dA, ldda, dipiv, dB, lddb, dX, lddx, dWorkspace, lwork_bytes, iter, d_info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnSHgesv"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSBgesv(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f32,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut f32,
    lddb: cusolver_int_t,
    dX: *mut f32,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSBgesv {
        Some(____func) => unsafe { ____func(handle, n, nrhs, dA, ldda, dipiv, dB, lddb, dX, lddx, dWorkspace, lwork_bytes, iter, d_info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnSBgesv"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSXgesv(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f32,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut f32,
    lddb: cusolver_int_t,
    dX: *mut f32,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSXgesv {
        Some(____func) => unsafe { ____func(handle, n, nrhs, dA, ldda, dipiv, dB, lddb, dX, lddx, dWorkspace, lwork_bytes, iter, d_info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnSXgesv"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZZgesv_bufferSize(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuDoubleComplex,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut cuDoubleComplex,
    lddb: cusolver_int_t,
    dX: *mut cuDoubleComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZZgesv_bufferSize {
        Some(____func) => unsafe { ____func(handle, n, nrhs, dA, ldda, dipiv, dB, lddb, dX, lddx, dWorkspace, lwork_bytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnZZgesv_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZCgesv_bufferSize(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuDoubleComplex,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut cuDoubleComplex,
    lddb: cusolver_int_t,
    dX: *mut cuDoubleComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZCgesv_bufferSize {
        Some(____func) => unsafe { ____func(handle, n, nrhs, dA, ldda, dipiv, dB, lddb, dX, lddx, dWorkspace, lwork_bytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnZCgesv_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZKgesv_bufferSize(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuDoubleComplex,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut cuDoubleComplex,
    lddb: cusolver_int_t,
    dX: *mut cuDoubleComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZKgesv_bufferSize {
        Some(____func) => unsafe { ____func(handle, n, nrhs, dA, ldda, dipiv, dB, lddb, dX, lddx, dWorkspace, lwork_bytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnZKgesv_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZEgesv_bufferSize(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuDoubleComplex,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut cuDoubleComplex,
    lddb: cusolver_int_t,
    dX: *mut cuDoubleComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZEgesv_bufferSize {
        Some(____func) => unsafe { ____func(handle, n, nrhs, dA, ldda, dipiv, dB, lddb, dX, lddx, dWorkspace, lwork_bytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnZEgesv_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZYgesv_bufferSize(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuDoubleComplex,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut cuDoubleComplex,
    lddb: cusolver_int_t,
    dX: *mut cuDoubleComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZYgesv_bufferSize {
        Some(____func) => unsafe { ____func(handle, n, nrhs, dA, ldda, dipiv, dB, lddb, dX, lddx, dWorkspace, lwork_bytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnZYgesv_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnCCgesv_bufferSize(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuComplex,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut cuComplex,
    lddb: cusolver_int_t,
    dX: *mut cuComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnCCgesv_bufferSize {
        Some(____func) => unsafe { ____func(handle, n, nrhs, dA, ldda, dipiv, dB, lddb, dX, lddx, dWorkspace, lwork_bytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnCCgesv_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnCKgesv_bufferSize(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuComplex,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut cuComplex,
    lddb: cusolver_int_t,
    dX: *mut cuComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnCKgesv_bufferSize {
        Some(____func) => unsafe { ____func(handle, n, nrhs, dA, ldda, dipiv, dB, lddb, dX, lddx, dWorkspace, lwork_bytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnCKgesv_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnCEgesv_bufferSize(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuComplex,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut cuComplex,
    lddb: cusolver_int_t,
    dX: *mut cuComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnCEgesv_bufferSize {
        Some(____func) => unsafe { ____func(handle, n, nrhs, dA, ldda, dipiv, dB, lddb, dX, lddx, dWorkspace, lwork_bytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnCEgesv_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnCYgesv_bufferSize(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuComplex,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut cuComplex,
    lddb: cusolver_int_t,
    dX: *mut cuComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnCYgesv_bufferSize {
        Some(____func) => unsafe { ____func(handle, n, nrhs, dA, ldda, dipiv, dB, lddb, dX, lddx, dWorkspace, lwork_bytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnCYgesv_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDDgesv_bufferSize(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f64,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut f64,
    lddb: cusolver_int_t,
    dX: *mut f64,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDDgesv_bufferSize {
        Some(____func) => unsafe { ____func(handle, n, nrhs, dA, ldda, dipiv, dB, lddb, dX, lddx, dWorkspace, lwork_bytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnDDgesv_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDSgesv_bufferSize(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f64,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut f64,
    lddb: cusolver_int_t,
    dX: *mut f64,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDSgesv_bufferSize {
        Some(____func) => unsafe { ____func(handle, n, nrhs, dA, ldda, dipiv, dB, lddb, dX, lddx, dWorkspace, lwork_bytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnDSgesv_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDHgesv_bufferSize(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f64,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut f64,
    lddb: cusolver_int_t,
    dX: *mut f64,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDHgesv_bufferSize {
        Some(____func) => unsafe { ____func(handle, n, nrhs, dA, ldda, dipiv, dB, lddb, dX, lddx, dWorkspace, lwork_bytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnDHgesv_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDBgesv_bufferSize(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f64,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut f64,
    lddb: cusolver_int_t,
    dX: *mut f64,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDBgesv_bufferSize {
        Some(____func) => unsafe { ____func(handle, n, nrhs, dA, ldda, dipiv, dB, lddb, dX, lddx, dWorkspace, lwork_bytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnDBgesv_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDXgesv_bufferSize(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f64,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut f64,
    lddb: cusolver_int_t,
    dX: *mut f64,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDXgesv_bufferSize {
        Some(____func) => unsafe { ____func(handle, n, nrhs, dA, ldda, dipiv, dB, lddb, dX, lddx, dWorkspace, lwork_bytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnDXgesv_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSSgesv_bufferSize(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f32,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut f32,
    lddb: cusolver_int_t,
    dX: *mut f32,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSSgesv_bufferSize {
        Some(____func) => unsafe { ____func(handle, n, nrhs, dA, ldda, dipiv, dB, lddb, dX, lddx, dWorkspace, lwork_bytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnSSgesv_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSHgesv_bufferSize(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f32,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut f32,
    lddb: cusolver_int_t,
    dX: *mut f32,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSHgesv_bufferSize {
        Some(____func) => unsafe { ____func(handle, n, nrhs, dA, ldda, dipiv, dB, lddb, dX, lddx, dWorkspace, lwork_bytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnSHgesv_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSBgesv_bufferSize(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f32,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut f32,
    lddb: cusolver_int_t,
    dX: *mut f32,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSBgesv_bufferSize {
        Some(____func) => unsafe { ____func(handle, n, nrhs, dA, ldda, dipiv, dB, lddb, dX, lddx, dWorkspace, lwork_bytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnSBgesv_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSXgesv_bufferSize(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f32,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut f32,
    lddb: cusolver_int_t,
    dX: *mut f32,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSXgesv_bufferSize {
        Some(____func) => unsafe { ____func(handle, n, nrhs, dA, ldda, dipiv, dB, lddb, dX, lddx, dWorkspace, lwork_bytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnSXgesv_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZZgels(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuDoubleComplex,
    ldda: cusolver_int_t,
    dB: *mut cuDoubleComplex,
    lddb: cusolver_int_t,
    dX: *mut cuDoubleComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZZgels {
        Some(____func) => unsafe { ____func(handle, m, n, nrhs, dA, ldda, dB, lddb, dX, lddx, dWorkspace, lwork_bytes, iter, d_info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnZZgels"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZCgels(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuDoubleComplex,
    ldda: cusolver_int_t,
    dB: *mut cuDoubleComplex,
    lddb: cusolver_int_t,
    dX: *mut cuDoubleComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZCgels {
        Some(____func) => unsafe { ____func(handle, m, n, nrhs, dA, ldda, dB, lddb, dX, lddx, dWorkspace, lwork_bytes, iter, d_info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnZCgels"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZKgels(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuDoubleComplex,
    ldda: cusolver_int_t,
    dB: *mut cuDoubleComplex,
    lddb: cusolver_int_t,
    dX: *mut cuDoubleComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZKgels {
        Some(____func) => unsafe { ____func(handle, m, n, nrhs, dA, ldda, dB, lddb, dX, lddx, dWorkspace, lwork_bytes, iter, d_info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnZKgels"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZEgels(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuDoubleComplex,
    ldda: cusolver_int_t,
    dB: *mut cuDoubleComplex,
    lddb: cusolver_int_t,
    dX: *mut cuDoubleComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZEgels {
        Some(____func) => unsafe { ____func(handle, m, n, nrhs, dA, ldda, dB, lddb, dX, lddx, dWorkspace, lwork_bytes, iter, d_info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnZEgels"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZYgels(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuDoubleComplex,
    ldda: cusolver_int_t,
    dB: *mut cuDoubleComplex,
    lddb: cusolver_int_t,
    dX: *mut cuDoubleComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZYgels {
        Some(____func) => unsafe { ____func(handle, m, n, nrhs, dA, ldda, dB, lddb, dX, lddx, dWorkspace, lwork_bytes, iter, d_info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnZYgels"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnCCgels(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuComplex,
    ldda: cusolver_int_t,
    dB: *mut cuComplex,
    lddb: cusolver_int_t,
    dX: *mut cuComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnCCgels {
        Some(____func) => unsafe { ____func(handle, m, n, nrhs, dA, ldda, dB, lddb, dX, lddx, dWorkspace, lwork_bytes, iter, d_info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnCCgels"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnCKgels(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuComplex,
    ldda: cusolver_int_t,
    dB: *mut cuComplex,
    lddb: cusolver_int_t,
    dX: *mut cuComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnCKgels {
        Some(____func) => unsafe { ____func(handle, m, n, nrhs, dA, ldda, dB, lddb, dX, lddx, dWorkspace, lwork_bytes, iter, d_info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnCKgels"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnCEgels(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuComplex,
    ldda: cusolver_int_t,
    dB: *mut cuComplex,
    lddb: cusolver_int_t,
    dX: *mut cuComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnCEgels {
        Some(____func) => unsafe { ____func(handle, m, n, nrhs, dA, ldda, dB, lddb, dX, lddx, dWorkspace, lwork_bytes, iter, d_info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnCEgels"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnCYgels(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuComplex,
    ldda: cusolver_int_t,
    dB: *mut cuComplex,
    lddb: cusolver_int_t,
    dX: *mut cuComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnCYgels {
        Some(____func) => unsafe { ____func(handle, m, n, nrhs, dA, ldda, dB, lddb, dX, lddx, dWorkspace, lwork_bytes, iter, d_info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnCYgels"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDDgels(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f64,
    ldda: cusolver_int_t,
    dB: *mut f64,
    lddb: cusolver_int_t,
    dX: *mut f64,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDDgels {
        Some(____func) => unsafe { ____func(handle, m, n, nrhs, dA, ldda, dB, lddb, dX, lddx, dWorkspace, lwork_bytes, iter, d_info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnDDgels"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDSgels(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f64,
    ldda: cusolver_int_t,
    dB: *mut f64,
    lddb: cusolver_int_t,
    dX: *mut f64,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDSgels {
        Some(____func) => unsafe { ____func(handle, m, n, nrhs, dA, ldda, dB, lddb, dX, lddx, dWorkspace, lwork_bytes, iter, d_info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnDSgels"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDHgels(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f64,
    ldda: cusolver_int_t,
    dB: *mut f64,
    lddb: cusolver_int_t,
    dX: *mut f64,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDHgels {
        Some(____func) => unsafe { ____func(handle, m, n, nrhs, dA, ldda, dB, lddb, dX, lddx, dWorkspace, lwork_bytes, iter, d_info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnDHgels"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDBgels(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f64,
    ldda: cusolver_int_t,
    dB: *mut f64,
    lddb: cusolver_int_t,
    dX: *mut f64,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDBgels {
        Some(____func) => unsafe { ____func(handle, m, n, nrhs, dA, ldda, dB, lddb, dX, lddx, dWorkspace, lwork_bytes, iter, d_info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnDBgels"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDXgels(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f64,
    ldda: cusolver_int_t,
    dB: *mut f64,
    lddb: cusolver_int_t,
    dX: *mut f64,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDXgels {
        Some(____func) => unsafe { ____func(handle, m, n, nrhs, dA, ldda, dB, lddb, dX, lddx, dWorkspace, lwork_bytes, iter, d_info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnDXgels"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSSgels(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f32,
    ldda: cusolver_int_t,
    dB: *mut f32,
    lddb: cusolver_int_t,
    dX: *mut f32,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSSgels {
        Some(____func) => unsafe { ____func(handle, m, n, nrhs, dA, ldda, dB, lddb, dX, lddx, dWorkspace, lwork_bytes, iter, d_info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnSSgels"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSHgels(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f32,
    ldda: cusolver_int_t,
    dB: *mut f32,
    lddb: cusolver_int_t,
    dX: *mut f32,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSHgels {
        Some(____func) => unsafe { ____func(handle, m, n, nrhs, dA, ldda, dB, lddb, dX, lddx, dWorkspace, lwork_bytes, iter, d_info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnSHgels"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSBgels(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f32,
    ldda: cusolver_int_t,
    dB: *mut f32,
    lddb: cusolver_int_t,
    dX: *mut f32,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSBgels {
        Some(____func) => unsafe { ____func(handle, m, n, nrhs, dA, ldda, dB, lddb, dX, lddx, dWorkspace, lwork_bytes, iter, d_info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnSBgels"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSXgels(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f32,
    ldda: cusolver_int_t,
    dB: *mut f32,
    lddb: cusolver_int_t,
    dX: *mut f32,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSXgels {
        Some(____func) => unsafe { ____func(handle, m, n, nrhs, dA, ldda, dB, lddb, dX, lddx, dWorkspace, lwork_bytes, iter, d_info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnSXgels"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZZgels_bufferSize(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuDoubleComplex,
    ldda: cusolver_int_t,
    dB: *mut cuDoubleComplex,
    lddb: cusolver_int_t,
    dX: *mut cuDoubleComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZZgels_bufferSize {
        Some(____func) => unsafe { ____func(handle, m, n, nrhs, dA, ldda, dB, lddb, dX, lddx, dWorkspace, lwork_bytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnZZgels_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZCgels_bufferSize(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuDoubleComplex,
    ldda: cusolver_int_t,
    dB: *mut cuDoubleComplex,
    lddb: cusolver_int_t,
    dX: *mut cuDoubleComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZCgels_bufferSize {
        Some(____func) => unsafe { ____func(handle, m, n, nrhs, dA, ldda, dB, lddb, dX, lddx, dWorkspace, lwork_bytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnZCgels_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZKgels_bufferSize(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuDoubleComplex,
    ldda: cusolver_int_t,
    dB: *mut cuDoubleComplex,
    lddb: cusolver_int_t,
    dX: *mut cuDoubleComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZKgels_bufferSize {
        Some(____func) => unsafe { ____func(handle, m, n, nrhs, dA, ldda, dB, lddb, dX, lddx, dWorkspace, lwork_bytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnZKgels_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZEgels_bufferSize(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuDoubleComplex,
    ldda: cusolver_int_t,
    dB: *mut cuDoubleComplex,
    lddb: cusolver_int_t,
    dX: *mut cuDoubleComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZEgels_bufferSize {
        Some(____func) => unsafe { ____func(handle, m, n, nrhs, dA, ldda, dB, lddb, dX, lddx, dWorkspace, lwork_bytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnZEgels_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZYgels_bufferSize(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuDoubleComplex,
    ldda: cusolver_int_t,
    dB: *mut cuDoubleComplex,
    lddb: cusolver_int_t,
    dX: *mut cuDoubleComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZYgels_bufferSize {
        Some(____func) => unsafe { ____func(handle, m, n, nrhs, dA, ldda, dB, lddb, dX, lddx, dWorkspace, lwork_bytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnZYgels_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnCCgels_bufferSize(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuComplex,
    ldda: cusolver_int_t,
    dB: *mut cuComplex,
    lddb: cusolver_int_t,
    dX: *mut cuComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnCCgels_bufferSize {
        Some(____func) => unsafe { ____func(handle, m, n, nrhs, dA, ldda, dB, lddb, dX, lddx, dWorkspace, lwork_bytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnCCgels_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnCKgels_bufferSize(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuComplex,
    ldda: cusolver_int_t,
    dB: *mut cuComplex,
    lddb: cusolver_int_t,
    dX: *mut cuComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnCKgels_bufferSize {
        Some(____func) => unsafe { ____func(handle, m, n, nrhs, dA, ldda, dB, lddb, dX, lddx, dWorkspace, lwork_bytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnCKgels_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnCEgels_bufferSize(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuComplex,
    ldda: cusolver_int_t,
    dB: *mut cuComplex,
    lddb: cusolver_int_t,
    dX: *mut cuComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnCEgels_bufferSize {
        Some(____func) => unsafe { ____func(handle, m, n, nrhs, dA, ldda, dB, lddb, dX, lddx, dWorkspace, lwork_bytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnCEgels_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnCYgels_bufferSize(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuComplex,
    ldda: cusolver_int_t,
    dB: *mut cuComplex,
    lddb: cusolver_int_t,
    dX: *mut cuComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnCYgels_bufferSize {
        Some(____func) => unsafe { ____func(handle, m, n, nrhs, dA, ldda, dB, lddb, dX, lddx, dWorkspace, lwork_bytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnCYgels_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDDgels_bufferSize(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f64,
    ldda: cusolver_int_t,
    dB: *mut f64,
    lddb: cusolver_int_t,
    dX: *mut f64,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDDgels_bufferSize {
        Some(____func) => unsafe { ____func(handle, m, n, nrhs, dA, ldda, dB, lddb, dX, lddx, dWorkspace, lwork_bytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnDDgels_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDSgels_bufferSize(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f64,
    ldda: cusolver_int_t,
    dB: *mut f64,
    lddb: cusolver_int_t,
    dX: *mut f64,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDSgels_bufferSize {
        Some(____func) => unsafe { ____func(handle, m, n, nrhs, dA, ldda, dB, lddb, dX, lddx, dWorkspace, lwork_bytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnDSgels_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDHgels_bufferSize(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f64,
    ldda: cusolver_int_t,
    dB: *mut f64,
    lddb: cusolver_int_t,
    dX: *mut f64,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDHgels_bufferSize {
        Some(____func) => unsafe { ____func(handle, m, n, nrhs, dA, ldda, dB, lddb, dX, lddx, dWorkspace, lwork_bytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnDHgels_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDBgels_bufferSize(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f64,
    ldda: cusolver_int_t,
    dB: *mut f64,
    lddb: cusolver_int_t,
    dX: *mut f64,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDBgels_bufferSize {
        Some(____func) => unsafe { ____func(handle, m, n, nrhs, dA, ldda, dB, lddb, dX, lddx, dWorkspace, lwork_bytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnDBgels_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDXgels_bufferSize(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f64,
    ldda: cusolver_int_t,
    dB: *mut f64,
    lddb: cusolver_int_t,
    dX: *mut f64,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDXgels_bufferSize {
        Some(____func) => unsafe { ____func(handle, m, n, nrhs, dA, ldda, dB, lddb, dX, lddx, dWorkspace, lwork_bytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnDXgels_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSSgels_bufferSize(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f32,
    ldda: cusolver_int_t,
    dB: *mut f32,
    lddb: cusolver_int_t,
    dX: *mut f32,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSSgels_bufferSize {
        Some(____func) => unsafe { ____func(handle, m, n, nrhs, dA, ldda, dB, lddb, dX, lddx, dWorkspace, lwork_bytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnSSgels_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSHgels_bufferSize(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f32,
    ldda: cusolver_int_t,
    dB: *mut f32,
    lddb: cusolver_int_t,
    dX: *mut f32,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSHgels_bufferSize {
        Some(____func) => unsafe { ____func(handle, m, n, nrhs, dA, ldda, dB, lddb, dX, lddx, dWorkspace, lwork_bytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnSHgels_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSBgels_bufferSize(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f32,
    ldda: cusolver_int_t,
    dB: *mut f32,
    lddb: cusolver_int_t,
    dX: *mut f32,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSBgels_bufferSize {
        Some(____func) => unsafe { ____func(handle, m, n, nrhs, dA, ldda, dB, lddb, dX, lddx, dWorkspace, lwork_bytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnSBgels_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSXgels_bufferSize(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f32,
    ldda: cusolver_int_t,
    dB: *mut f32,
    lddb: cusolver_int_t,
    dX: *mut f32,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSXgels_bufferSize {
        Some(____func) => unsafe { ____func(handle, m, n, nrhs, dA, ldda, dB, lddb, dX, lddx, dWorkspace, lwork_bytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnSXgels_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnIRSXgesv(
    handle: cusolverDnHandle_t,
    gesv_irs_params: cusolverDnIRSParams_t,
    gesv_irs_infos: cusolverDnIRSInfos_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut ::std::os::raw::c_void,
    ldda: cusolver_int_t,
    dB: *mut ::std::os::raw::c_void,
    lddb: cusolver_int_t,
    dX: *mut ::std::os::raw::c_void,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    niters: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnIRSXgesv {
        Some(____func) => unsafe { ____func(handle, gesv_irs_params, gesv_irs_infos, n, nrhs, dA, ldda, dB, lddb, dX, lddx, dWorkspace, lwork_bytes, niters, d_info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnIRSXgesv"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnIRSXgesv_bufferSize(handle: cusolverDnHandle_t, params: cusolverDnIRSParams_t, n: cusolver_int_t, nrhs: cusolver_int_t, lwork_bytes: *mut usize) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnIRSXgesv_bufferSize {
        Some(____func) => unsafe { ____func(handle, params, n, nrhs, lwork_bytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnIRSXgesv_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnIRSXgels(
    handle: cusolverDnHandle_t,
    gels_irs_params: cusolverDnIRSParams_t,
    gels_irs_infos: cusolverDnIRSInfos_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut ::std::os::raw::c_void,
    ldda: cusolver_int_t,
    dB: *mut ::std::os::raw::c_void,
    lddb: cusolver_int_t,
    dX: *mut ::std::os::raw::c_void,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    niters: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnIRSXgels {
        Some(____func) => unsafe { ____func(handle, gels_irs_params, gels_irs_infos, m, n, nrhs, dA, ldda, dB, lddb, dX, lddx, dWorkspace, lwork_bytes, niters, d_info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnIRSXgels"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnIRSXgels_bufferSize(handle: cusolverDnHandle_t, params: cusolverDnIRSParams_t, m: cusolver_int_t, n: cusolver_int_t, nrhs: cusolver_int_t, lwork_bytes: *mut usize) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnIRSXgels_bufferSize {
        Some(____func) => unsafe { ____func(handle, params, m, n, nrhs, lwork_bytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnIRSXgels_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSpotrf_bufferSize(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f32, lda: ::std::os::raw::c_int, Lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSpotrf_bufferSize {
        Some(____func) => unsafe { ____func(handle, uplo, n, A, lda, Lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnSpotrf_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDpotrf_bufferSize(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f64, lda: ::std::os::raw::c_int, Lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDpotrf_bufferSize {
        Some(____func) => unsafe { ____func(handle, uplo, n, A, lda, Lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnDpotrf_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnCpotrf_bufferSize(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut cuComplex, lda: ::std::os::raw::c_int, Lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnCpotrf_bufferSize {
        Some(____func) => unsafe { ____func(handle, uplo, n, A, lda, Lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnCpotrf_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZpotrf_bufferSize(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut cuDoubleComplex, lda: ::std::os::raw::c_int, Lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZpotrf_bufferSize {
        Some(____func) => unsafe { ____func(handle, uplo, n, A, lda, Lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnZpotrf_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSpotrf(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f32, lda: ::std::os::raw::c_int, Workspace: *mut f32, Lwork: ::std::os::raw::c_int, devInfo: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSpotrf {
        Some(____func) => unsafe { ____func(handle, uplo, n, A, lda, Workspace, Lwork, devInfo) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnSpotrf"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDpotrf(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f64, lda: ::std::os::raw::c_int, Workspace: *mut f64, Lwork: ::std::os::raw::c_int, devInfo: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDpotrf {
        Some(____func) => unsafe { ____func(handle, uplo, n, A, lda, Workspace, Lwork, devInfo) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnDpotrf"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnCpotrf(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut cuComplex, lda: ::std::os::raw::c_int, Workspace: *mut cuComplex, Lwork: ::std::os::raw::c_int, devInfo: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnCpotrf {
        Some(____func) => unsafe { ____func(handle, uplo, n, A, lda, Workspace, Lwork, devInfo) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnCpotrf"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZpotrf(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut cuDoubleComplex, lda: ::std::os::raw::c_int, Workspace: *mut cuDoubleComplex, Lwork: ::std::os::raw::c_int, devInfo: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZpotrf {
        Some(____func) => unsafe { ____func(handle, uplo, n, A, lda, Workspace, Lwork, devInfo) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnZpotrf"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSpotrs(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, nrhs: ::std::os::raw::c_int, A: *const f32, lda: ::std::os::raw::c_int, B: *mut f32, ldb: ::std::os::raw::c_int, devInfo: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSpotrs {
        Some(____func) => unsafe { ____func(handle, uplo, n, nrhs, A, lda, B, ldb, devInfo) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnSpotrs"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDpotrs(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, nrhs: ::std::os::raw::c_int, A: *const f64, lda: ::std::os::raw::c_int, B: *mut f64, ldb: ::std::os::raw::c_int, devInfo: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDpotrs {
        Some(____func) => unsafe { ____func(handle, uplo, n, nrhs, A, lda, B, ldb, devInfo) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnDpotrs"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnCpotrs(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    nrhs: ::std::os::raw::c_int,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    B: *mut cuComplex,
    ldb: ::std::os::raw::c_int,
    devInfo: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnCpotrs {
        Some(____func) => unsafe { ____func(handle, uplo, n, nrhs, A, lda, B, ldb, devInfo) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnCpotrs"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZpotrs(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    nrhs: ::std::os::raw::c_int,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    B: *mut cuDoubleComplex,
    ldb: ::std::os::raw::c_int,
    devInfo: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZpotrs {
        Some(____func) => unsafe { ____func(handle, uplo, n, nrhs, A, lda, B, ldb, devInfo) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnZpotrs"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSpotrfBatched(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, Aarray: *mut *mut f32, lda: ::std::os::raw::c_int, infoArray: *mut ::std::os::raw::c_int, batchSize: ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSpotrfBatched {
        Some(____func) => unsafe { ____func(handle, uplo, n, Aarray, lda, infoArray, batchSize) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnSpotrfBatched"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDpotrfBatched(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, Aarray: *mut *mut f64, lda: ::std::os::raw::c_int, infoArray: *mut ::std::os::raw::c_int, batchSize: ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDpotrfBatched {
        Some(____func) => unsafe { ____func(handle, uplo, n, Aarray, lda, infoArray, batchSize) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnDpotrfBatched"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnCpotrfBatched(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, Aarray: *mut *mut cuComplex, lda: ::std::os::raw::c_int, infoArray: *mut ::std::os::raw::c_int, batchSize: ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnCpotrfBatched {
        Some(____func) => unsafe { ____func(handle, uplo, n, Aarray, lda, infoArray, batchSize) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnCpotrfBatched"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZpotrfBatched(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, Aarray: *mut *mut cuDoubleComplex, lda: ::std::os::raw::c_int, infoArray: *mut ::std::os::raw::c_int, batchSize: ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZpotrfBatched {
        Some(____func) => unsafe { ____func(handle, uplo, n, Aarray, lda, infoArray, batchSize) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnZpotrfBatched"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSpotrsBatched(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    nrhs: ::std::os::raw::c_int,
    A: *mut *mut f32,
    lda: ::std::os::raw::c_int,
    B: *mut *mut f32,
    ldb: ::std::os::raw::c_int,
    d_info: *mut ::std::os::raw::c_int,
    batchSize: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSpotrsBatched {
        Some(____func) => unsafe { ____func(handle, uplo, n, nrhs, A, lda, B, ldb, d_info, batchSize) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnSpotrsBatched"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDpotrsBatched(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    nrhs: ::std::os::raw::c_int,
    A: *mut *mut f64,
    lda: ::std::os::raw::c_int,
    B: *mut *mut f64,
    ldb: ::std::os::raw::c_int,
    d_info: *mut ::std::os::raw::c_int,
    batchSize: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDpotrsBatched {
        Some(____func) => unsafe { ____func(handle, uplo, n, nrhs, A, lda, B, ldb, d_info, batchSize) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnDpotrsBatched"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnCpotrsBatched(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    nrhs: ::std::os::raw::c_int,
    A: *mut *mut cuComplex,
    lda: ::std::os::raw::c_int,
    B: *mut *mut cuComplex,
    ldb: ::std::os::raw::c_int,
    d_info: *mut ::std::os::raw::c_int,
    batchSize: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnCpotrsBatched {
        Some(____func) => unsafe { ____func(handle, uplo, n, nrhs, A, lda, B, ldb, d_info, batchSize) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnCpotrsBatched"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZpotrsBatched(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    nrhs: ::std::os::raw::c_int,
    A: *mut *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    B: *mut *mut cuDoubleComplex,
    ldb: ::std::os::raw::c_int,
    d_info: *mut ::std::os::raw::c_int,
    batchSize: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZpotrsBatched {
        Some(____func) => unsafe { ____func(handle, uplo, n, nrhs, A, lda, B, ldb, d_info, batchSize) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnZpotrsBatched"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSpotri_bufferSize(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f32, lda: ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSpotri_bufferSize {
        Some(____func) => unsafe { ____func(handle, uplo, n, A, lda, lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnSpotri_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDpotri_bufferSize(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f64, lda: ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDpotri_bufferSize {
        Some(____func) => unsafe { ____func(handle, uplo, n, A, lda, lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnDpotri_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnCpotri_bufferSize(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut cuComplex, lda: ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnCpotri_bufferSize {
        Some(____func) => unsafe { ____func(handle, uplo, n, A, lda, lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnCpotri_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZpotri_bufferSize(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut cuDoubleComplex, lda: ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZpotri_bufferSize {
        Some(____func) => unsafe { ____func(handle, uplo, n, A, lda, lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnZpotri_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSpotri(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f32, lda: ::std::os::raw::c_int, work: *mut f32, lwork: ::std::os::raw::c_int, devInfo: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSpotri {
        Some(____func) => unsafe { ____func(handle, uplo, n, A, lda, work, lwork, devInfo) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnSpotri"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDpotri(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f64, lda: ::std::os::raw::c_int, work: *mut f64, lwork: ::std::os::raw::c_int, devInfo: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDpotri {
        Some(____func) => unsafe { ____func(handle, uplo, n, A, lda, work, lwork, devInfo) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnDpotri"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnCpotri(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut cuComplex, lda: ::std::os::raw::c_int, work: *mut cuComplex, lwork: ::std::os::raw::c_int, devInfo: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnCpotri {
        Some(____func) => unsafe { ____func(handle, uplo, n, A, lda, work, lwork, devInfo) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnCpotri"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZpotri(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut cuDoubleComplex, lda: ::std::os::raw::c_int, work: *mut cuDoubleComplex, lwork: ::std::os::raw::c_int, devInfo: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZpotri {
        Some(____func) => unsafe { ____func(handle, uplo, n, A, lda, work, lwork, devInfo) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnZpotri"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnXtrtri_bufferSize(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, diag: cublasDiagType_t, n: i64, dataTypeA: cudaDataType, A: *mut ::std::os::raw::c_void, lda: i64, workspaceInBytesOnDevice: *mut usize, workspaceInBytesOnHost: *mut usize) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnXtrtri_bufferSize {
        Some(____func) => unsafe { ____func(handle, uplo, diag, n, dataTypeA, A, lda, workspaceInBytesOnDevice, workspaceInBytesOnHost) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnXtrtri_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnXtrtri(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    diag: cublasDiagType_t,
    n: i64,
    dataTypeA: cudaDataType,
    A: *mut ::std::os::raw::c_void,
    lda: i64,
    bufferOnDevice: *mut ::std::os::raw::c_void,
    workspaceInBytesOnDevice: usize,
    bufferOnHost: *mut ::std::os::raw::c_void,
    workspaceInBytesOnHost: usize,
    devInfo: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnXtrtri {
        Some(____func) => unsafe { ____func(handle, uplo, diag, n, dataTypeA, A, lda, bufferOnDevice, workspaceInBytesOnDevice, bufferOnHost, workspaceInBytesOnHost, devInfo) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnXtrtri"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSlauum_bufferSize(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f32, lda: ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSlauum_bufferSize {
        Some(____func) => unsafe { ____func(handle, uplo, n, A, lda, lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnSlauum_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDlauum_bufferSize(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f64, lda: ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDlauum_bufferSize {
        Some(____func) => unsafe { ____func(handle, uplo, n, A, lda, lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnDlauum_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnClauum_bufferSize(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut cuComplex, lda: ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnClauum_bufferSize {
        Some(____func) => unsafe { ____func(handle, uplo, n, A, lda, lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnClauum_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZlauum_bufferSize(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut cuDoubleComplex, lda: ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZlauum_bufferSize {
        Some(____func) => unsafe { ____func(handle, uplo, n, A, lda, lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnZlauum_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSlauum(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f32, lda: ::std::os::raw::c_int, work: *mut f32, lwork: ::std::os::raw::c_int, devInfo: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSlauum {
        Some(____func) => unsafe { ____func(handle, uplo, n, A, lda, work, lwork, devInfo) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnSlauum"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDlauum(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f64, lda: ::std::os::raw::c_int, work: *mut f64, lwork: ::std::os::raw::c_int, devInfo: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDlauum {
        Some(____func) => unsafe { ____func(handle, uplo, n, A, lda, work, lwork, devInfo) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnDlauum"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnClauum(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut cuComplex, lda: ::std::os::raw::c_int, work: *mut cuComplex, lwork: ::std::os::raw::c_int, devInfo: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnClauum {
        Some(____func) => unsafe { ____func(handle, uplo, n, A, lda, work, lwork, devInfo) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnClauum"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZlauum(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut cuDoubleComplex, lda: ::std::os::raw::c_int, work: *mut cuDoubleComplex, lwork: ::std::os::raw::c_int, devInfo: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZlauum {
        Some(____func) => unsafe { ____func(handle, uplo, n, A, lda, work, lwork, devInfo) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnZlauum"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSgetrf_bufferSize(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, A: *mut f32, lda: ::std::os::raw::c_int, Lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSgetrf_bufferSize {
        Some(____func) => unsafe { ____func(handle, m, n, A, lda, Lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnSgetrf_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDgetrf_bufferSize(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, A: *mut f64, lda: ::std::os::raw::c_int, Lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDgetrf_bufferSize {
        Some(____func) => unsafe { ____func(handle, m, n, A, lda, Lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnDgetrf_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnCgetrf_bufferSize(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, A: *mut cuComplex, lda: ::std::os::raw::c_int, Lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnCgetrf_bufferSize {
        Some(____func) => unsafe { ____func(handle, m, n, A, lda, Lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnCgetrf_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZgetrf_bufferSize(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, A: *mut cuDoubleComplex, lda: ::std::os::raw::c_int, Lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZgetrf_bufferSize {
        Some(____func) => unsafe { ____func(handle, m, n, A, lda, Lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnZgetrf_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSgetrf(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, A: *mut f32, lda: ::std::os::raw::c_int, Workspace: *mut f32, devIpiv: *mut ::std::os::raw::c_int, devInfo: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSgetrf {
        Some(____func) => unsafe { ____func(handle, m, n, A, lda, Workspace, devIpiv, devInfo) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnSgetrf"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDgetrf(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, A: *mut f64, lda: ::std::os::raw::c_int, Workspace: *mut f64, devIpiv: *mut ::std::os::raw::c_int, devInfo: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDgetrf {
        Some(____func) => unsafe { ____func(handle, m, n, A, lda, Workspace, devIpiv, devInfo) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnDgetrf"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnCgetrf(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, A: *mut cuComplex, lda: ::std::os::raw::c_int, Workspace: *mut cuComplex, devIpiv: *mut ::std::os::raw::c_int, devInfo: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnCgetrf {
        Some(____func) => unsafe { ____func(handle, m, n, A, lda, Workspace, devIpiv, devInfo) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnCgetrf"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZgetrf(
    handle: cusolverDnHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    Workspace: *mut cuDoubleComplex,
    devIpiv: *mut ::std::os::raw::c_int,
    devInfo: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZgetrf {
        Some(____func) => unsafe { ____func(handle, m, n, A, lda, Workspace, devIpiv, devInfo) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnZgetrf"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSlaswp(handle: cusolverDnHandle_t, n: ::std::os::raw::c_int, A: *mut f32, lda: ::std::os::raw::c_int, k1: ::std::os::raw::c_int, k2: ::std::os::raw::c_int, devIpiv: *const ::std::os::raw::c_int, incx: ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSlaswp {
        Some(____func) => unsafe { ____func(handle, n, A, lda, k1, k2, devIpiv, incx) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnSlaswp"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDlaswp(handle: cusolverDnHandle_t, n: ::std::os::raw::c_int, A: *mut f64, lda: ::std::os::raw::c_int, k1: ::std::os::raw::c_int, k2: ::std::os::raw::c_int, devIpiv: *const ::std::os::raw::c_int, incx: ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDlaswp {
        Some(____func) => unsafe { ____func(handle, n, A, lda, k1, k2, devIpiv, incx) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnDlaswp"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnClaswp(handle: cusolverDnHandle_t, n: ::std::os::raw::c_int, A: *mut cuComplex, lda: ::std::os::raw::c_int, k1: ::std::os::raw::c_int, k2: ::std::os::raw::c_int, devIpiv: *const ::std::os::raw::c_int, incx: ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnClaswp {
        Some(____func) => unsafe { ____func(handle, n, A, lda, k1, k2, devIpiv, incx) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnClaswp"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZlaswp(handle: cusolverDnHandle_t, n: ::std::os::raw::c_int, A: *mut cuDoubleComplex, lda: ::std::os::raw::c_int, k1: ::std::os::raw::c_int, k2: ::std::os::raw::c_int, devIpiv: *const ::std::os::raw::c_int, incx: ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZlaswp {
        Some(____func) => unsafe { ____func(handle, n, A, lda, k1, k2, devIpiv, incx) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnZlaswp"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSgetrs(
    handle: cusolverDnHandle_t,
    trans: cublasOperation_t,
    n: ::std::os::raw::c_int,
    nrhs: ::std::os::raw::c_int,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    devIpiv: *const ::std::os::raw::c_int,
    B: *mut f32,
    ldb: ::std::os::raw::c_int,
    devInfo: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSgetrs {
        Some(____func) => unsafe { ____func(handle, trans, n, nrhs, A, lda, devIpiv, B, ldb, devInfo) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnSgetrs"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDgetrs(
    handle: cusolverDnHandle_t,
    trans: cublasOperation_t,
    n: ::std::os::raw::c_int,
    nrhs: ::std::os::raw::c_int,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    devIpiv: *const ::std::os::raw::c_int,
    B: *mut f64,
    ldb: ::std::os::raw::c_int,
    devInfo: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDgetrs {
        Some(____func) => unsafe { ____func(handle, trans, n, nrhs, A, lda, devIpiv, B, ldb, devInfo) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnDgetrs"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnCgetrs(
    handle: cusolverDnHandle_t,
    trans: cublasOperation_t,
    n: ::std::os::raw::c_int,
    nrhs: ::std::os::raw::c_int,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    devIpiv: *const ::std::os::raw::c_int,
    B: *mut cuComplex,
    ldb: ::std::os::raw::c_int,
    devInfo: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnCgetrs {
        Some(____func) => unsafe { ____func(handle, trans, n, nrhs, A, lda, devIpiv, B, ldb, devInfo) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnCgetrs"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZgetrs(
    handle: cusolverDnHandle_t,
    trans: cublasOperation_t,
    n: ::std::os::raw::c_int,
    nrhs: ::std::os::raw::c_int,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    devIpiv: *const ::std::os::raw::c_int,
    B: *mut cuDoubleComplex,
    ldb: ::std::os::raw::c_int,
    devInfo: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZgetrs {
        Some(____func) => unsafe { ____func(handle, trans, n, nrhs, A, lda, devIpiv, B, ldb, devInfo) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnZgetrs"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSgeqrf_bufferSize(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, A: *mut f32, lda: ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSgeqrf_bufferSize {
        Some(____func) => unsafe { ____func(handle, m, n, A, lda, lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnSgeqrf_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDgeqrf_bufferSize(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, A: *mut f64, lda: ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDgeqrf_bufferSize {
        Some(____func) => unsafe { ____func(handle, m, n, A, lda, lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnDgeqrf_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnCgeqrf_bufferSize(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, A: *mut cuComplex, lda: ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnCgeqrf_bufferSize {
        Some(____func) => unsafe { ____func(handle, m, n, A, lda, lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnCgeqrf_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZgeqrf_bufferSize(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, A: *mut cuDoubleComplex, lda: ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZgeqrf_bufferSize {
        Some(____func) => unsafe { ____func(handle, m, n, A, lda, lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnZgeqrf_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSgeqrf(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, A: *mut f32, lda: ::std::os::raw::c_int, TAU: *mut f32, Workspace: *mut f32, Lwork: ::std::os::raw::c_int, devInfo: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSgeqrf {
        Some(____func) => unsafe { ____func(handle, m, n, A, lda, TAU, Workspace, Lwork, devInfo) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnSgeqrf"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDgeqrf(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, A: *mut f64, lda: ::std::os::raw::c_int, TAU: *mut f64, Workspace: *mut f64, Lwork: ::std::os::raw::c_int, devInfo: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDgeqrf {
        Some(____func) => unsafe { ____func(handle, m, n, A, lda, TAU, Workspace, Lwork, devInfo) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnDgeqrf"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnCgeqrf(
    handle: cusolverDnHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *mut cuComplex,
    lda: ::std::os::raw::c_int,
    TAU: *mut cuComplex,
    Workspace: *mut cuComplex,
    Lwork: ::std::os::raw::c_int,
    devInfo: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnCgeqrf {
        Some(____func) => unsafe { ____func(handle, m, n, A, lda, TAU, Workspace, Lwork, devInfo) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnCgeqrf"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZgeqrf(
    handle: cusolverDnHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    TAU: *mut cuDoubleComplex,
    Workspace: *mut cuDoubleComplex,
    Lwork: ::std::os::raw::c_int,
    devInfo: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZgeqrf {
        Some(____func) => unsafe { ____func(handle, m, n, A, lda, TAU, Workspace, Lwork, devInfo) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnZgeqrf"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSorgqr_bufferSize(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, k: ::std::os::raw::c_int, A: *const f32, lda: ::std::os::raw::c_int, tau: *const f32, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSorgqr_bufferSize {
        Some(____func) => unsafe { ____func(handle, m, n, k, A, lda, tau, lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnSorgqr_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDorgqr_bufferSize(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, k: ::std::os::raw::c_int, A: *const f64, lda: ::std::os::raw::c_int, tau: *const f64, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDorgqr_bufferSize {
        Some(____func) => unsafe { ____func(handle, m, n, k, A, lda, tau, lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnDorgqr_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnCungqr_bufferSize(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, k: ::std::os::raw::c_int, A: *const cuComplex, lda: ::std::os::raw::c_int, tau: *const cuComplex, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnCungqr_bufferSize {
        Some(____func) => unsafe { ____func(handle, m, n, k, A, lda, tau, lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnCungqr_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZungqr_bufferSize(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, k: ::std::os::raw::c_int, A: *const cuDoubleComplex, lda: ::std::os::raw::c_int, tau: *const cuDoubleComplex, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZungqr_bufferSize {
        Some(____func) => unsafe { ____func(handle, m, n, k, A, lda, tau, lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnZungqr_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSorgqr(
    handle: cusolverDnHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    A: *mut f32,
    lda: ::std::os::raw::c_int,
    tau: *const f32,
    work: *mut f32,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSorgqr {
        Some(____func) => unsafe { ____func(handle, m, n, k, A, lda, tau, work, lwork, info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnSorgqr"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDorgqr(
    handle: cusolverDnHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    A: *mut f64,
    lda: ::std::os::raw::c_int,
    tau: *const f64,
    work: *mut f64,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDorgqr {
        Some(____func) => unsafe { ____func(handle, m, n, k, A, lda, tau, work, lwork, info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnDorgqr"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnCungqr(
    handle: cusolverDnHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    A: *mut cuComplex,
    lda: ::std::os::raw::c_int,
    tau: *const cuComplex,
    work: *mut cuComplex,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnCungqr {
        Some(____func) => unsafe { ____func(handle, m, n, k, A, lda, tau, work, lwork, info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnCungqr"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZungqr(
    handle: cusolverDnHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    A: *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    tau: *const cuDoubleComplex,
    work: *mut cuDoubleComplex,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZungqr {
        Some(____func) => unsafe { ____func(handle, m, n, k, A, lda, tau, work, lwork, info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnZungqr"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSormqr_bufferSize(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    tau: *const f32,
    C: *const f32,
    ldc: ::std::os::raw::c_int,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSormqr_bufferSize {
        Some(____func) => unsafe { ____func(handle, side, trans, m, n, k, A, lda, tau, C, ldc, lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnSormqr_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDormqr_bufferSize(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    tau: *const f64,
    C: *const f64,
    ldc: ::std::os::raw::c_int,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDormqr_bufferSize {
        Some(____func) => unsafe { ____func(handle, side, trans, m, n, k, A, lda, tau, C, ldc, lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnDormqr_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnCunmqr_bufferSize(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    tau: *const cuComplex,
    C: *const cuComplex,
    ldc: ::std::os::raw::c_int,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnCunmqr_bufferSize {
        Some(____func) => unsafe { ____func(handle, side, trans, m, n, k, A, lda, tau, C, ldc, lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnCunmqr_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZunmqr_bufferSize(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    tau: *const cuDoubleComplex,
    C: *const cuDoubleComplex,
    ldc: ::std::os::raw::c_int,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZunmqr_bufferSize {
        Some(____func) => unsafe { ____func(handle, side, trans, m, n, k, A, lda, tau, C, ldc, lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnZunmqr_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSormqr(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    tau: *const f32,
    C: *mut f32,
    ldc: ::std::os::raw::c_int,
    work: *mut f32,
    lwork: ::std::os::raw::c_int,
    devInfo: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSormqr {
        Some(____func) => unsafe { ____func(handle, side, trans, m, n, k, A, lda, tau, C, ldc, work, lwork, devInfo) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnSormqr"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDormqr(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    tau: *const f64,
    C: *mut f64,
    ldc: ::std::os::raw::c_int,
    work: *mut f64,
    lwork: ::std::os::raw::c_int,
    devInfo: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDormqr {
        Some(____func) => unsafe { ____func(handle, side, trans, m, n, k, A, lda, tau, C, ldc, work, lwork, devInfo) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnDormqr"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnCunmqr(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    tau: *const cuComplex,
    C: *mut cuComplex,
    ldc: ::std::os::raw::c_int,
    work: *mut cuComplex,
    lwork: ::std::os::raw::c_int,
    devInfo: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnCunmqr {
        Some(____func) => unsafe { ____func(handle, side, trans, m, n, k, A, lda, tau, C, ldc, work, lwork, devInfo) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnCunmqr"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZunmqr(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    tau: *const cuDoubleComplex,
    C: *mut cuDoubleComplex,
    ldc: ::std::os::raw::c_int,
    work: *mut cuDoubleComplex,
    lwork: ::std::os::raw::c_int,
    devInfo: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZunmqr {
        Some(____func) => unsafe { ____func(handle, side, trans, m, n, k, A, lda, tau, C, ldc, work, lwork, devInfo) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnZunmqr"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSsytrf_bufferSize(handle: cusolverDnHandle_t, n: ::std::os::raw::c_int, A: *mut f32, lda: ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSsytrf_bufferSize {
        Some(____func) => unsafe { ____func(handle, n, A, lda, lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnSsytrf_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDsytrf_bufferSize(handle: cusolverDnHandle_t, n: ::std::os::raw::c_int, A: *mut f64, lda: ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDsytrf_bufferSize {
        Some(____func) => unsafe { ____func(handle, n, A, lda, lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnDsytrf_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnCsytrf_bufferSize(handle: cusolverDnHandle_t, n: ::std::os::raw::c_int, A: *mut cuComplex, lda: ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnCsytrf_bufferSize {
        Some(____func) => unsafe { ____func(handle, n, A, lda, lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnCsytrf_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZsytrf_bufferSize(handle: cusolverDnHandle_t, n: ::std::os::raw::c_int, A: *mut cuDoubleComplex, lda: ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZsytrf_bufferSize {
        Some(____func) => unsafe { ____func(handle, n, A, lda, lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnZsytrf_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSsytrf(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f32, lda: ::std::os::raw::c_int, ipiv: *mut ::std::os::raw::c_int, work: *mut f32, lwork: ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSsytrf {
        Some(____func) => unsafe { ____func(handle, uplo, n, A, lda, ipiv, work, lwork, info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnSsytrf"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDsytrf(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f64, lda: ::std::os::raw::c_int, ipiv: *mut ::std::os::raw::c_int, work: *mut f64, lwork: ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDsytrf {
        Some(____func) => unsafe { ____func(handle, uplo, n, A, lda, ipiv, work, lwork, info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnDsytrf"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnCsytrf(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut cuComplex,
    lda: ::std::os::raw::c_int,
    ipiv: *mut ::std::os::raw::c_int,
    work: *mut cuComplex,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnCsytrf {
        Some(____func) => unsafe { ____func(handle, uplo, n, A, lda, ipiv, work, lwork, info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnCsytrf"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZsytrf(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    ipiv: *mut ::std::os::raw::c_int,
    work: *mut cuDoubleComplex,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZsytrf {
        Some(____func) => unsafe { ____func(handle, uplo, n, A, lda, ipiv, work, lwork, info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnZsytrf"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnXsytrs_bufferSize(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i64,
    nrhs: i64,
    dataTypeA: cudaDataType,
    A: *const ::std::os::raw::c_void,
    lda: i64,
    ipiv: *const i64,
    dataTypeB: cudaDataType,
    B: *mut ::std::os::raw::c_void,
    ldb: i64,
    workspaceInBytesOnDevice: *mut usize,
    workspaceInBytesOnHost: *mut usize,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnXsytrs_bufferSize {
        Some(____func) => unsafe { ____func(handle, uplo, n, nrhs, dataTypeA, A, lda, ipiv, dataTypeB, B, ldb, workspaceInBytesOnDevice, workspaceInBytesOnHost) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnXsytrs_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnXsytrs(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i64,
    nrhs: i64,
    dataTypeA: cudaDataType,
    A: *const ::std::os::raw::c_void,
    lda: i64,
    ipiv: *const i64,
    dataTypeB: cudaDataType,
    B: *mut ::std::os::raw::c_void,
    ldb: i64,
    bufferOnDevice: *mut ::std::os::raw::c_void,
    workspaceInBytesOnDevice: usize,
    bufferOnHost: *mut ::std::os::raw::c_void,
    workspaceInBytesOnHost: usize,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnXsytrs {
        Some(____func) => unsafe { ____func(handle, uplo, n, nrhs, dataTypeA, A, lda, ipiv, dataTypeB, B, ldb, bufferOnDevice, workspaceInBytesOnDevice, bufferOnHost, workspaceInBytesOnHost, info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnXsytrs"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSsytri_bufferSize(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f32, lda: ::std::os::raw::c_int, ipiv: *const ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSsytri_bufferSize {
        Some(____func) => unsafe { ____func(handle, uplo, n, A, lda, ipiv, lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnSsytri_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDsytri_bufferSize(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f64, lda: ::std::os::raw::c_int, ipiv: *const ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDsytri_bufferSize {
        Some(____func) => unsafe { ____func(handle, uplo, n, A, lda, ipiv, lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnDsytri_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnCsytri_bufferSize(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut cuComplex, lda: ::std::os::raw::c_int, ipiv: *const ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnCsytri_bufferSize {
        Some(____func) => unsafe { ____func(handle, uplo, n, A, lda, ipiv, lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnCsytri_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZsytri_bufferSize(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut cuDoubleComplex, lda: ::std::os::raw::c_int, ipiv: *const ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZsytri_bufferSize {
        Some(____func) => unsafe { ____func(handle, uplo, n, A, lda, ipiv, lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnZsytri_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSsytri(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f32, lda: ::std::os::raw::c_int, ipiv: *const ::std::os::raw::c_int, work: *mut f32, lwork: ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSsytri {
        Some(____func) => unsafe { ____func(handle, uplo, n, A, lda, ipiv, work, lwork, info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnSsytri"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDsytri(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f64, lda: ::std::os::raw::c_int, ipiv: *const ::std::os::raw::c_int, work: *mut f64, lwork: ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDsytri {
        Some(____func) => unsafe { ____func(handle, uplo, n, A, lda, ipiv, work, lwork, info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnDsytri"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnCsytri(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut cuComplex,
    lda: ::std::os::raw::c_int,
    ipiv: *const ::std::os::raw::c_int,
    work: *mut cuComplex,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnCsytri {
        Some(____func) => unsafe { ____func(handle, uplo, n, A, lda, ipiv, work, lwork, info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnCsytri"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZsytri(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    ipiv: *const ::std::os::raw::c_int,
    work: *mut cuDoubleComplex,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZsytri {
        Some(____func) => unsafe { ____func(handle, uplo, n, A, lda, ipiv, work, lwork, info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnZsytri"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSgebrd_bufferSize(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, Lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSgebrd_bufferSize {
        Some(____func) => unsafe { ____func(handle, m, n, Lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnSgebrd_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDgebrd_bufferSize(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, Lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDgebrd_bufferSize {
        Some(____func) => unsafe { ____func(handle, m, n, Lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnDgebrd_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnCgebrd_bufferSize(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, Lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnCgebrd_bufferSize {
        Some(____func) => unsafe { ____func(handle, m, n, Lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnCgebrd_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZgebrd_bufferSize(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, Lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZgebrd_bufferSize {
        Some(____func) => unsafe { ____func(handle, m, n, Lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnZgebrd_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSgebrd(
    handle: cusolverDnHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *mut f32,
    lda: ::std::os::raw::c_int,
    D: *mut f32,
    E: *mut f32,
    TAUQ: *mut f32,
    TAUP: *mut f32,
    Work: *mut f32,
    Lwork: ::std::os::raw::c_int,
    devInfo: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSgebrd {
        Some(____func) => unsafe { ____func(handle, m, n, A, lda, D, E, TAUQ, TAUP, Work, Lwork, devInfo) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnSgebrd"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDgebrd(
    handle: cusolverDnHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *mut f64,
    lda: ::std::os::raw::c_int,
    D: *mut f64,
    E: *mut f64,
    TAUQ: *mut f64,
    TAUP: *mut f64,
    Work: *mut f64,
    Lwork: ::std::os::raw::c_int,
    devInfo: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDgebrd {
        Some(____func) => unsafe { ____func(handle, m, n, A, lda, D, E, TAUQ, TAUP, Work, Lwork, devInfo) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnDgebrd"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnCgebrd(
    handle: cusolverDnHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *mut cuComplex,
    lda: ::std::os::raw::c_int,
    D: *mut f32,
    E: *mut f32,
    TAUQ: *mut cuComplex,
    TAUP: *mut cuComplex,
    Work: *mut cuComplex,
    Lwork: ::std::os::raw::c_int,
    devInfo: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnCgebrd {
        Some(____func) => unsafe { ____func(handle, m, n, A, lda, D, E, TAUQ, TAUP, Work, Lwork, devInfo) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnCgebrd"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZgebrd(
    handle: cusolverDnHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    D: *mut f64,
    E: *mut f64,
    TAUQ: *mut cuDoubleComplex,
    TAUP: *mut cuDoubleComplex,
    Work: *mut cuDoubleComplex,
    Lwork: ::std::os::raw::c_int,
    devInfo: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZgebrd {
        Some(____func) => unsafe { ____func(handle, m, n, A, lda, D, E, TAUQ, TAUP, Work, Lwork, devInfo) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnZgebrd"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSorgbr_bufferSize(handle: cusolverDnHandle_t, side: cublasSideMode_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, k: ::std::os::raw::c_int, A: *const f32, lda: ::std::os::raw::c_int, tau: *const f32, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSorgbr_bufferSize {
        Some(____func) => unsafe { ____func(handle, side, m, n, k, A, lda, tau, lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnSorgbr_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDorgbr_bufferSize(handle: cusolverDnHandle_t, side: cublasSideMode_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, k: ::std::os::raw::c_int, A: *const f64, lda: ::std::os::raw::c_int, tau: *const f64, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDorgbr_bufferSize {
        Some(____func) => unsafe { ____func(handle, side, m, n, k, A, lda, tau, lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnDorgbr_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnCungbr_bufferSize(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    tau: *const cuComplex,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnCungbr_bufferSize {
        Some(____func) => unsafe { ____func(handle, side, m, n, k, A, lda, tau, lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnCungbr_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZungbr_bufferSize(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    tau: *const cuDoubleComplex,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZungbr_bufferSize {
        Some(____func) => unsafe { ____func(handle, side, m, n, k, A, lda, tau, lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnZungbr_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSorgbr(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    A: *mut f32,
    lda: ::std::os::raw::c_int,
    tau: *const f32,
    work: *mut f32,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSorgbr {
        Some(____func) => unsafe { ____func(handle, side, m, n, k, A, lda, tau, work, lwork, info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnSorgbr"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDorgbr(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    A: *mut f64,
    lda: ::std::os::raw::c_int,
    tau: *const f64,
    work: *mut f64,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDorgbr {
        Some(____func) => unsafe { ____func(handle, side, m, n, k, A, lda, tau, work, lwork, info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnDorgbr"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnCungbr(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    A: *mut cuComplex,
    lda: ::std::os::raw::c_int,
    tau: *const cuComplex,
    work: *mut cuComplex,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnCungbr {
        Some(____func) => unsafe { ____func(handle, side, m, n, k, A, lda, tau, work, lwork, info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnCungbr"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZungbr(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    A: *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    tau: *const cuDoubleComplex,
    work: *mut cuDoubleComplex,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZungbr {
        Some(____func) => unsafe { ____func(handle, side, m, n, k, A, lda, tau, work, lwork, info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnZungbr"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSsytrd_bufferSize(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *const f32, lda: ::std::os::raw::c_int, d: *const f32, e: *const f32, tau: *const f32, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSsytrd_bufferSize {
        Some(____func) => unsafe { ____func(handle, uplo, n, A, lda, d, e, tau, lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnSsytrd_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDsytrd_bufferSize(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *const f64, lda: ::std::os::raw::c_int, d: *const f64, e: *const f64, tau: *const f64, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDsytrd_bufferSize {
        Some(____func) => unsafe { ____func(handle, uplo, n, A, lda, d, e, tau, lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnDsytrd_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnChetrd_bufferSize(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *const cuComplex, lda: ::std::os::raw::c_int, d: *const f32, e: *const f32, tau: *const cuComplex, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnChetrd_bufferSize {
        Some(____func) => unsafe { ____func(handle, uplo, n, A, lda, d, e, tau, lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnChetrd_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZhetrd_bufferSize(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    d: *const f64,
    e: *const f64,
    tau: *const cuDoubleComplex,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZhetrd_bufferSize {
        Some(____func) => unsafe { ____func(handle, uplo, n, A, lda, d, e, tau, lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnZhetrd_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSsytrd(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut f32,
    lda: ::std::os::raw::c_int,
    d: *mut f32,
    e: *mut f32,
    tau: *mut f32,
    work: *mut f32,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSsytrd {
        Some(____func) => unsafe { ____func(handle, uplo, n, A, lda, d, e, tau, work, lwork, info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnSsytrd"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDsytrd(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut f64,
    lda: ::std::os::raw::c_int,
    d: *mut f64,
    e: *mut f64,
    tau: *mut f64,
    work: *mut f64,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDsytrd {
        Some(____func) => unsafe { ____func(handle, uplo, n, A, lda, d, e, tau, work, lwork, info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnDsytrd"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnChetrd(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut cuComplex,
    lda: ::std::os::raw::c_int,
    d: *mut f32,
    e: *mut f32,
    tau: *mut cuComplex,
    work: *mut cuComplex,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnChetrd {
        Some(____func) => unsafe { ____func(handle, uplo, n, A, lda, d, e, tau, work, lwork, info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnChetrd"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZhetrd(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    d: *mut f64,
    e: *mut f64,
    tau: *mut cuDoubleComplex,
    work: *mut cuDoubleComplex,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZhetrd {
        Some(____func) => unsafe { ____func(handle, uplo, n, A, lda, d, e, tau, work, lwork, info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnZhetrd"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSorgtr_bufferSize(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *const f32, lda: ::std::os::raw::c_int, tau: *const f32, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSorgtr_bufferSize {
        Some(____func) => unsafe { ____func(handle, uplo, n, A, lda, tau, lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnSorgtr_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDorgtr_bufferSize(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *const f64, lda: ::std::os::raw::c_int, tau: *const f64, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDorgtr_bufferSize {
        Some(____func) => unsafe { ____func(handle, uplo, n, A, lda, tau, lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnDorgtr_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnCungtr_bufferSize(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *const cuComplex, lda: ::std::os::raw::c_int, tau: *const cuComplex, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnCungtr_bufferSize {
        Some(____func) => unsafe { ____func(handle, uplo, n, A, lda, tau, lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnCungtr_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZungtr_bufferSize(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *const cuDoubleComplex, lda: ::std::os::raw::c_int, tau: *const cuDoubleComplex, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZungtr_bufferSize {
        Some(____func) => unsafe { ____func(handle, uplo, n, A, lda, tau, lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnZungtr_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSorgtr(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f32, lda: ::std::os::raw::c_int, tau: *const f32, work: *mut f32, lwork: ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSorgtr {
        Some(____func) => unsafe { ____func(handle, uplo, n, A, lda, tau, work, lwork, info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnSorgtr"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDorgtr(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f64, lda: ::std::os::raw::c_int, tau: *const f64, work: *mut f64, lwork: ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDorgtr {
        Some(____func) => unsafe { ____func(handle, uplo, n, A, lda, tau, work, lwork, info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnDorgtr"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnCungtr(handle: cusolverDnHandle_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut cuComplex, lda: ::std::os::raw::c_int, tau: *const cuComplex, work: *mut cuComplex, lwork: ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnCungtr {
        Some(____func) => unsafe { ____func(handle, uplo, n, A, lda, tau, work, lwork, info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnCungtr"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZungtr(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    tau: *const cuDoubleComplex,
    work: *mut cuDoubleComplex,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZungtr {
        Some(____func) => unsafe { ____func(handle, uplo, n, A, lda, tau, work, lwork, info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnZungtr"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSormtr_bufferSize(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    tau: *const f32,
    C: *const f32,
    ldc: ::std::os::raw::c_int,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSormtr_bufferSize {
        Some(____func) => unsafe { ____func(handle, side, uplo, trans, m, n, A, lda, tau, C, ldc, lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnSormtr_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDormtr_bufferSize(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    tau: *const f64,
    C: *const f64,
    ldc: ::std::os::raw::c_int,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDormtr_bufferSize {
        Some(____func) => unsafe { ____func(handle, side, uplo, trans, m, n, A, lda, tau, C, ldc, lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnDormtr_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnCunmtr_bufferSize(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    tau: *const cuComplex,
    C: *const cuComplex,
    ldc: ::std::os::raw::c_int,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnCunmtr_bufferSize {
        Some(____func) => unsafe { ____func(handle, side, uplo, trans, m, n, A, lda, tau, C, ldc, lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnCunmtr_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZunmtr_bufferSize(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    tau: *const cuDoubleComplex,
    C: *const cuDoubleComplex,
    ldc: ::std::os::raw::c_int,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZunmtr_bufferSize {
        Some(____func) => unsafe { ____func(handle, side, uplo, trans, m, n, A, lda, tau, C, ldc, lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnZunmtr_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSormtr(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *mut f32,
    lda: ::std::os::raw::c_int,
    tau: *mut f32,
    C: *mut f32,
    ldc: ::std::os::raw::c_int,
    work: *mut f32,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSormtr {
        Some(____func) => unsafe { ____func(handle, side, uplo, trans, m, n, A, lda, tau, C, ldc, work, lwork, info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnSormtr"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDormtr(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *mut f64,
    lda: ::std::os::raw::c_int,
    tau: *mut f64,
    C: *mut f64,
    ldc: ::std::os::raw::c_int,
    work: *mut f64,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDormtr {
        Some(____func) => unsafe { ____func(handle, side, uplo, trans, m, n, A, lda, tau, C, ldc, work, lwork, info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnDormtr"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnCunmtr(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *mut cuComplex,
    lda: ::std::os::raw::c_int,
    tau: *mut cuComplex,
    C: *mut cuComplex,
    ldc: ::std::os::raw::c_int,
    work: *mut cuComplex,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnCunmtr {
        Some(____func) => unsafe { ____func(handle, side, uplo, trans, m, n, A, lda, tau, C, ldc, work, lwork, info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnCunmtr"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZunmtr(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    tau: *mut cuDoubleComplex,
    C: *mut cuDoubleComplex,
    ldc: ::std::os::raw::c_int,
    work: *mut cuDoubleComplex,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZunmtr {
        Some(____func) => unsafe { ____func(handle, side, uplo, trans, m, n, A, lda, tau, C, ldc, work, lwork, info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnZunmtr"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSgesvd_bufferSize(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSgesvd_bufferSize {
        Some(____func) => unsafe { ____func(handle, m, n, lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnSgesvd_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDgesvd_bufferSize(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDgesvd_bufferSize {
        Some(____func) => unsafe { ____func(handle, m, n, lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnDgesvd_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnCgesvd_bufferSize(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnCgesvd_bufferSize {
        Some(____func) => unsafe { ____func(handle, m, n, lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnCgesvd_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZgesvd_bufferSize(handle: cusolverDnHandle_t, m: ::std::os::raw::c_int, n: ::std::os::raw::c_int, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZgesvd_bufferSize {
        Some(____func) => unsafe { ____func(handle, m, n, lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnZgesvd_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSgesvd(
    handle: cusolverDnHandle_t,
    jobu: ::std::os::raw::c_schar,
    jobvt: ::std::os::raw::c_schar,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *mut f32,
    lda: ::std::os::raw::c_int,
    S: *mut f32,
    U: *mut f32,
    ldu: ::std::os::raw::c_int,
    VT: *mut f32,
    ldvt: ::std::os::raw::c_int,
    work: *mut f32,
    lwork: ::std::os::raw::c_int,
    rwork: *mut f32,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSgesvd {
        Some(____func) => unsafe { ____func(handle, jobu, jobvt, m, n, A, lda, S, U, ldu, VT, ldvt, work, lwork, rwork, info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnSgesvd"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDgesvd(
    handle: cusolverDnHandle_t,
    jobu: ::std::os::raw::c_schar,
    jobvt: ::std::os::raw::c_schar,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *mut f64,
    lda: ::std::os::raw::c_int,
    S: *mut f64,
    U: *mut f64,
    ldu: ::std::os::raw::c_int,
    VT: *mut f64,
    ldvt: ::std::os::raw::c_int,
    work: *mut f64,
    lwork: ::std::os::raw::c_int,
    rwork: *mut f64,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDgesvd {
        Some(____func) => unsafe { ____func(handle, jobu, jobvt, m, n, A, lda, S, U, ldu, VT, ldvt, work, lwork, rwork, info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnDgesvd"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnCgesvd(
    handle: cusolverDnHandle_t,
    jobu: ::std::os::raw::c_schar,
    jobvt: ::std::os::raw::c_schar,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *mut cuComplex,
    lda: ::std::os::raw::c_int,
    S: *mut f32,
    U: *mut cuComplex,
    ldu: ::std::os::raw::c_int,
    VT: *mut cuComplex,
    ldvt: ::std::os::raw::c_int,
    work: *mut cuComplex,
    lwork: ::std::os::raw::c_int,
    rwork: *mut f32,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnCgesvd {
        Some(____func) => unsafe { ____func(handle, jobu, jobvt, m, n, A, lda, S, U, ldu, VT, ldvt, work, lwork, rwork, info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnCgesvd"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZgesvd(
    handle: cusolverDnHandle_t,
    jobu: ::std::os::raw::c_schar,
    jobvt: ::std::os::raw::c_schar,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    S: *mut f64,
    U: *mut cuDoubleComplex,
    ldu: ::std::os::raw::c_int,
    VT: *mut cuDoubleComplex,
    ldvt: ::std::os::raw::c_int,
    work: *mut cuDoubleComplex,
    lwork: ::std::os::raw::c_int,
    rwork: *mut f64,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZgesvd {
        Some(____func) => unsafe { ____func(handle, jobu, jobvt, m, n, A, lda, S, U, ldu, VT, ldvt, work, lwork, rwork, info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnZgesvd"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSsyevd_bufferSize(handle: cusolverDnHandle_t, jobz: cusolverEigMode_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *const f32, lda: ::std::os::raw::c_int, W: *const f32, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSsyevd_bufferSize {
        Some(____func) => unsafe { ____func(handle, jobz, uplo, n, A, lda, W, lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnSsyevd_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDsyevd_bufferSize(handle: cusolverDnHandle_t, jobz: cusolverEigMode_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *const f64, lda: ::std::os::raw::c_int, W: *const f64, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDsyevd_bufferSize {
        Some(____func) => unsafe { ____func(handle, jobz, uplo, n, A, lda, W, lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnDsyevd_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnCheevd_bufferSize(handle: cusolverDnHandle_t, jobz: cusolverEigMode_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *const cuComplex, lda: ::std::os::raw::c_int, W: *const f32, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnCheevd_bufferSize {
        Some(____func) => unsafe { ____func(handle, jobz, uplo, n, A, lda, W, lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnCheevd_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZheevd_bufferSize(handle: cusolverDnHandle_t, jobz: cusolverEigMode_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *const cuDoubleComplex, lda: ::std::os::raw::c_int, W: *const f64, lwork: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZheevd_bufferSize {
        Some(____func) => unsafe { ____func(handle, jobz, uplo, n, A, lda, W, lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnZheevd_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSsyevd(handle: cusolverDnHandle_t, jobz: cusolverEigMode_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f32, lda: ::std::os::raw::c_int, W: *mut f32, work: *mut f32, lwork: ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSsyevd {
        Some(____func) => unsafe { ____func(handle, jobz, uplo, n, A, lda, W, work, lwork, info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnSsyevd"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDsyevd(handle: cusolverDnHandle_t, jobz: cusolverEigMode_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *mut f64, lda: ::std::os::raw::c_int, W: *mut f64, work: *mut f64, lwork: ::std::os::raw::c_int, info: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDsyevd {
        Some(____func) => unsafe { ____func(handle, jobz, uplo, n, A, lda, W, work, lwork, info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnDsyevd"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnCheevd(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut cuComplex,
    lda: ::std::os::raw::c_int,
    W: *mut f32,
    work: *mut cuComplex,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnCheevd {
        Some(____func) => unsafe { ____func(handle, jobz, uplo, n, A, lda, W, work, lwork, info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnCheevd"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZheevd(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    W: *mut f64,
    work: *mut cuDoubleComplex,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZheevd {
        Some(____func) => unsafe { ____func(handle, jobz, uplo, n, A, lda, W, work, lwork, info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnZheevd"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSsyevdx_bufferSize(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    vl: f32,
    vu: f32,
    il: ::std::os::raw::c_int,
    iu: ::std::os::raw::c_int,
    meig: *mut ::std::os::raw::c_int,
    W: *const f32,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSsyevdx_bufferSize {
        Some(____func) => unsafe { ____func(handle, jobz, range, uplo, n, A, lda, vl, vu, il, iu, meig, W, lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnSsyevdx_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDsyevdx_bufferSize(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    vl: f64,
    vu: f64,
    il: ::std::os::raw::c_int,
    iu: ::std::os::raw::c_int,
    meig: *mut ::std::os::raw::c_int,
    W: *const f64,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDsyevdx_bufferSize {
        Some(____func) => unsafe { ____func(handle, jobz, range, uplo, n, A, lda, vl, vu, il, iu, meig, W, lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnDsyevdx_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnCheevdx_bufferSize(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    vl: f32,
    vu: f32,
    il: ::std::os::raw::c_int,
    iu: ::std::os::raw::c_int,
    meig: *mut ::std::os::raw::c_int,
    W: *const f32,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnCheevdx_bufferSize {
        Some(____func) => unsafe { ____func(handle, jobz, range, uplo, n, A, lda, vl, vu, il, iu, meig, W, lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnCheevdx_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZheevdx_bufferSize(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    vl: f64,
    vu: f64,
    il: ::std::os::raw::c_int,
    iu: ::std::os::raw::c_int,
    meig: *mut ::std::os::raw::c_int,
    W: *const f64,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZheevdx_bufferSize {
        Some(____func) => unsafe { ____func(handle, jobz, range, uplo, n, A, lda, vl, vu, il, iu, meig, W, lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnZheevdx_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSsyevdx(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut f32,
    lda: ::std::os::raw::c_int,
    vl: f32,
    vu: f32,
    il: ::std::os::raw::c_int,
    iu: ::std::os::raw::c_int,
    meig: *mut ::std::os::raw::c_int,
    W: *mut f32,
    work: *mut f32,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSsyevdx {
        Some(____func) => unsafe { ____func(handle, jobz, range, uplo, n, A, lda, vl, vu, il, iu, meig, W, work, lwork, info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnSsyevdx"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDsyevdx(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut f64,
    lda: ::std::os::raw::c_int,
    vl: f64,
    vu: f64,
    il: ::std::os::raw::c_int,
    iu: ::std::os::raw::c_int,
    meig: *mut ::std::os::raw::c_int,
    W: *mut f64,
    work: *mut f64,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDsyevdx {
        Some(____func) => unsafe { ____func(handle, jobz, range, uplo, n, A, lda, vl, vu, il, iu, meig, W, work, lwork, info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnDsyevdx"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnCheevdx(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut cuComplex,
    lda: ::std::os::raw::c_int,
    vl: f32,
    vu: f32,
    il: ::std::os::raw::c_int,
    iu: ::std::os::raw::c_int,
    meig: *mut ::std::os::raw::c_int,
    W: *mut f32,
    work: *mut cuComplex,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnCheevdx {
        Some(____func) => unsafe { ____func(handle, jobz, range, uplo, n, A, lda, vl, vu, il, iu, meig, W, work, lwork, info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnCheevdx"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZheevdx(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    vl: f64,
    vu: f64,
    il: ::std::os::raw::c_int,
    iu: ::std::os::raw::c_int,
    meig: *mut ::std::os::raw::c_int,
    W: *mut f64,
    work: *mut cuDoubleComplex,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZheevdx {
        Some(____func) => unsafe { ____func(handle, jobz, range, uplo, n, A, lda, vl, vu, il, iu, meig, W, work, lwork, info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnZheevdx"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSsygvdx_bufferSize(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    B: *const f32,
    ldb: ::std::os::raw::c_int,
    vl: f32,
    vu: f32,
    il: ::std::os::raw::c_int,
    iu: ::std::os::raw::c_int,
    meig: *mut ::std::os::raw::c_int,
    W: *const f32,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSsygvdx_bufferSize {
        Some(____func) => unsafe { ____func(handle, itype, jobz, range, uplo, n, A, lda, B, ldb, vl, vu, il, iu, meig, W, lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnSsygvdx_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDsygvdx_bufferSize(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    B: *const f64,
    ldb: ::std::os::raw::c_int,
    vl: f64,
    vu: f64,
    il: ::std::os::raw::c_int,
    iu: ::std::os::raw::c_int,
    meig: *mut ::std::os::raw::c_int,
    W: *const f64,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDsygvdx_bufferSize {
        Some(____func) => unsafe { ____func(handle, itype, jobz, range, uplo, n, A, lda, B, ldb, vl, vu, il, iu, meig, W, lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnDsygvdx_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnChegvdx_bufferSize(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    B: *const cuComplex,
    ldb: ::std::os::raw::c_int,
    vl: f32,
    vu: f32,
    il: ::std::os::raw::c_int,
    iu: ::std::os::raw::c_int,
    meig: *mut ::std::os::raw::c_int,
    W: *const f32,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnChegvdx_bufferSize {
        Some(____func) => unsafe { ____func(handle, itype, jobz, range, uplo, n, A, lda, B, ldb, vl, vu, il, iu, meig, W, lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnChegvdx_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZhegvdx_bufferSize(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    B: *const cuDoubleComplex,
    ldb: ::std::os::raw::c_int,
    vl: f64,
    vu: f64,
    il: ::std::os::raw::c_int,
    iu: ::std::os::raw::c_int,
    meig: *mut ::std::os::raw::c_int,
    W: *const f64,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZhegvdx_bufferSize {
        Some(____func) => unsafe { ____func(handle, itype, jobz, range, uplo, n, A, lda, B, ldb, vl, vu, il, iu, meig, W, lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnZhegvdx_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSsygvdx(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut f32,
    lda: ::std::os::raw::c_int,
    B: *mut f32,
    ldb: ::std::os::raw::c_int,
    vl: f32,
    vu: f32,
    il: ::std::os::raw::c_int,
    iu: ::std::os::raw::c_int,
    meig: *mut ::std::os::raw::c_int,
    W: *mut f32,
    work: *mut f32,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSsygvdx {
        Some(____func) => unsafe { ____func(handle, itype, jobz, range, uplo, n, A, lda, B, ldb, vl, vu, il, iu, meig, W, work, lwork, info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnSsygvdx"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDsygvdx(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut f64,
    lda: ::std::os::raw::c_int,
    B: *mut f64,
    ldb: ::std::os::raw::c_int,
    vl: f64,
    vu: f64,
    il: ::std::os::raw::c_int,
    iu: ::std::os::raw::c_int,
    meig: *mut ::std::os::raw::c_int,
    W: *mut f64,
    work: *mut f64,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDsygvdx {
        Some(____func) => unsafe { ____func(handle, itype, jobz, range, uplo, n, A, lda, B, ldb, vl, vu, il, iu, meig, W, work, lwork, info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnDsygvdx"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnChegvdx(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut cuComplex,
    lda: ::std::os::raw::c_int,
    B: *mut cuComplex,
    ldb: ::std::os::raw::c_int,
    vl: f32,
    vu: f32,
    il: ::std::os::raw::c_int,
    iu: ::std::os::raw::c_int,
    meig: *mut ::std::os::raw::c_int,
    W: *mut f32,
    work: *mut cuComplex,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnChegvdx {
        Some(____func) => unsafe { ____func(handle, itype, jobz, range, uplo, n, A, lda, B, ldb, vl, vu, il, iu, meig, W, work, lwork, info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnChegvdx"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZhegvdx(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    B: *mut cuDoubleComplex,
    ldb: ::std::os::raw::c_int,
    vl: f64,
    vu: f64,
    il: ::std::os::raw::c_int,
    iu: ::std::os::raw::c_int,
    meig: *mut ::std::os::raw::c_int,
    W: *mut f64,
    work: *mut cuDoubleComplex,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZhegvdx {
        Some(____func) => unsafe { ____func(handle, itype, jobz, range, uplo, n, A, lda, B, ldb, vl, vu, il, iu, meig, W, work, lwork, info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnZhegvdx"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSsygvd_bufferSize(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    B: *const f32,
    ldb: ::std::os::raw::c_int,
    W: *const f32,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSsygvd_bufferSize {
        Some(____func) => unsafe { ____func(handle, itype, jobz, uplo, n, A, lda, B, ldb, W, lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnSsygvd_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDsygvd_bufferSize(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    B: *const f64,
    ldb: ::std::os::raw::c_int,
    W: *const f64,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDsygvd_bufferSize {
        Some(____func) => unsafe { ____func(handle, itype, jobz, uplo, n, A, lda, B, ldb, W, lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnDsygvd_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnChegvd_bufferSize(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    B: *const cuComplex,
    ldb: ::std::os::raw::c_int,
    W: *const f32,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnChegvd_bufferSize {
        Some(____func) => unsafe { ____func(handle, itype, jobz, uplo, n, A, lda, B, ldb, W, lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnChegvd_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZhegvd_bufferSize(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    B: *const cuDoubleComplex,
    ldb: ::std::os::raw::c_int,
    W: *const f64,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZhegvd_bufferSize {
        Some(____func) => unsafe { ____func(handle, itype, jobz, uplo, n, A, lda, B, ldb, W, lwork) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnZhegvd_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSsygvd(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut f32,
    lda: ::std::os::raw::c_int,
    B: *mut f32,
    ldb: ::std::os::raw::c_int,
    W: *mut f32,
    work: *mut f32,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSsygvd {
        Some(____func) => unsafe { ____func(handle, itype, jobz, uplo, n, A, lda, B, ldb, W, work, lwork, info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnSsygvd"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDsygvd(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut f64,
    lda: ::std::os::raw::c_int,
    B: *mut f64,
    ldb: ::std::os::raw::c_int,
    W: *mut f64,
    work: *mut f64,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDsygvd {
        Some(____func) => unsafe { ____func(handle, itype, jobz, uplo, n, A, lda, B, ldb, W, work, lwork, info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnDsygvd"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnChegvd(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut cuComplex,
    lda: ::std::os::raw::c_int,
    B: *mut cuComplex,
    ldb: ::std::os::raw::c_int,
    W: *mut f32,
    work: *mut cuComplex,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnChegvd {
        Some(____func) => unsafe { ____func(handle, itype, jobz, uplo, n, A, lda, B, ldb, W, work, lwork, info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnChegvd"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZhegvd(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    B: *mut cuDoubleComplex,
    ldb: ::std::os::raw::c_int,
    W: *mut f64,
    work: *mut cuDoubleComplex,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZhegvd {
        Some(____func) => unsafe { ____func(handle, itype, jobz, uplo, n, A, lda, B, ldb, W, work, lwork, info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnZhegvd"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnXsygvd_bufferSize(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i64,
    dataTypeA: cudaDataType,
    d_A: *const ::std::os::raw::c_void,
    lda: i64,
    dataTypeB: cudaDataType,
    d_B: *const ::std::os::raw::c_void,
    ldb: i64,
    dataTypeW: cudaDataType,
    d_W: *const ::std::os::raw::c_void,
    computeType: cudaDataType,
    workspaceInBytesOnDevice: *mut usize,
    workspaceInBytesOnHost: *mut usize,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnXsygvd_bufferSize {
        Some(____func) => unsafe { ____func(handle, params, itype, jobz, uplo, n, dataTypeA, d_A, lda, dataTypeB, d_B, ldb, dataTypeW, d_W, computeType, workspaceInBytesOnDevice, workspaceInBytesOnHost) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnXsygvd_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnXsygvd(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i64,
    dataTypeA: cudaDataType,
    d_A: *mut ::std::os::raw::c_void,
    lda: i64,
    dataTypeB: cudaDataType,
    d_B: *mut ::std::os::raw::c_void,
    ldb: i64,
    dataTypeW: cudaDataType,
    d_W: *mut ::std::os::raw::c_void,
    computeType: cudaDataType,
    bufferOnDevice: *mut ::std::os::raw::c_void,
    workspaceInBytesOnDevice: usize,
    bufferOnHost: *mut ::std::os::raw::c_void,
    workspaceInBytesOnHost: usize,
    d_info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnXsygvd {
        Some(____func) => unsafe {
            ____func(
                handle,
                params,
                itype,
                jobz,
                uplo,
                n,
                dataTypeA,
                d_A,
                lda,
                dataTypeB,
                d_B,
                ldb,
                dataTypeW,
                d_W,
                computeType,
                bufferOnDevice,
                workspaceInBytesOnDevice,
                bufferOnHost,
                workspaceInBytesOnHost,
                d_info,
            )
        },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnXsygvd"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnXsygvdx_bufferSize(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i64,
    dataTypeA: cudaDataType,
    d_A: *const ::std::os::raw::c_void,
    lda: i64,
    dataTypeB: cudaDataType,
    d_B: *const ::std::os::raw::c_void,
    ldb: i64,
    vl: *mut ::std::os::raw::c_void,
    vu: *mut ::std::os::raw::c_void,
    il: i64,
    iu: i64,
    meig: *mut i64,
    dataTypeW: cudaDataType,
    d_W: *const ::std::os::raw::c_void,
    computeType: cudaDataType,
    workspaceInBytesOnDevice: *mut usize,
    workspaceInBytesOnHost: *mut usize,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnXsygvdx_bufferSize {
        Some(____func) => unsafe { ____func(handle, params, itype, jobz, uplo, n, dataTypeA, d_A, lda, dataTypeB, d_B, ldb, vl, vu, il, iu, meig, dataTypeW, d_W, computeType, workspaceInBytesOnDevice, workspaceInBytesOnHost) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnXsygvdx_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnXsygvdx(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: i64,
    dataTypeA: cudaDataType,
    d_A: *mut ::std::os::raw::c_void,
    lda: i64,
    dataTypeB: cudaDataType,
    d_B: *mut ::std::os::raw::c_void,
    ldb: i64,
    vl: *mut ::std::os::raw::c_void,
    vu: *mut ::std::os::raw::c_void,
    il: i64,
    iu: i64,
    meig: *mut i64,
    dataTypeW: cudaDataType,
    d_W: *mut ::std::os::raw::c_void,
    computeType: cudaDataType,
    bufferOnDevice: *mut ::std::os::raw::c_void,
    workspaceInBytesOnDevice: usize,
    bufferOnHost: *mut ::std::os::raw::c_void,
    workspaceInBytesOnHost: usize,
    d_info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnXsygvdx {
        Some(____func) => unsafe {
            ____func(
                handle,
                params,
                itype,
                jobz,
                range,
                uplo,
                n,
                dataTypeA,
                d_A,
                lda,
                dataTypeB,
                d_B,
                ldb,
                vl,
                vu,
                il,
                iu,
                meig,
                dataTypeW,
                d_W,
                computeType,
                bufferOnDevice,
                workspaceInBytesOnDevice,
                bufferOnHost,
                workspaceInBytesOnHost,
                d_info,
            )
        },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnXsygvdx"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnCreateSyevjInfo(info: *mut syevjInfo_t) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnCreateSyevjInfo {
        Some(____func) => unsafe { ____func(info) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnCreateSyevjInfo"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDestroySyevjInfo(info: syevjInfo_t) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDestroySyevjInfo {
        Some(____func) => unsafe { ____func(info) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnDestroySyevjInfo"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnXsyevjSetTolerance(info: syevjInfo_t, tolerance: f64) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnXsyevjSetTolerance {
        Some(____func) => unsafe { ____func(info, tolerance) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnXsyevjSetTolerance"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnXsyevjSetMaxSweeps(info: syevjInfo_t, max_sweeps: ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnXsyevjSetMaxSweeps {
        Some(____func) => unsafe { ____func(info, max_sweeps) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnXsyevjSetMaxSweeps"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnXsyevjSetSortEig(info: syevjInfo_t, sort_eig: ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnXsyevjSetSortEig {
        Some(____func) => unsafe { ____func(info, sort_eig) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnXsyevjSetSortEig"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnXsyevjGetResidual(handle: cusolverDnHandle_t, info: syevjInfo_t, residual: *mut f64) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnXsyevjGetResidual {
        Some(____func) => unsafe { ____func(handle, info, residual) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnXsyevjGetResidual"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnXsyevjGetSweeps(handle: cusolverDnHandle_t, info: syevjInfo_t, executed_sweeps: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnXsyevjGetSweeps {
        Some(____func) => unsafe { ____func(handle, info, executed_sweeps) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnXsyevjGetSweeps"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSsyevjBatched_bufferSize(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    W: *const f32,
    lwork: *mut ::std::os::raw::c_int,
    params: syevjInfo_t,
    batchSize: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSsyevjBatched_bufferSize {
        Some(____func) => unsafe { ____func(handle, jobz, uplo, n, A, lda, W, lwork, params, batchSize) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnSsyevjBatched_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDsyevjBatched_bufferSize(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    W: *const f64,
    lwork: *mut ::std::os::raw::c_int,
    params: syevjInfo_t,
    batchSize: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDsyevjBatched_bufferSize {
        Some(____func) => unsafe { ____func(handle, jobz, uplo, n, A, lda, W, lwork, params, batchSize) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnDsyevjBatched_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnCheevjBatched_bufferSize(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    W: *const f32,
    lwork: *mut ::std::os::raw::c_int,
    params: syevjInfo_t,
    batchSize: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnCheevjBatched_bufferSize {
        Some(____func) => unsafe { ____func(handle, jobz, uplo, n, A, lda, W, lwork, params, batchSize) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnCheevjBatched_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZheevjBatched_bufferSize(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    W: *const f64,
    lwork: *mut ::std::os::raw::c_int,
    params: syevjInfo_t,
    batchSize: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZheevjBatched_bufferSize {
        Some(____func) => unsafe { ____func(handle, jobz, uplo, n, A, lda, W, lwork, params, batchSize) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnZheevjBatched_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSsyevjBatched(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut f32,
    lda: ::std::os::raw::c_int,
    W: *mut f32,
    work: *mut f32,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    params: syevjInfo_t,
    batchSize: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSsyevjBatched {
        Some(____func) => unsafe { ____func(handle, jobz, uplo, n, A, lda, W, work, lwork, info, params, batchSize) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnSsyevjBatched"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDsyevjBatched(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut f64,
    lda: ::std::os::raw::c_int,
    W: *mut f64,
    work: *mut f64,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    params: syevjInfo_t,
    batchSize: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDsyevjBatched {
        Some(____func) => unsafe { ____func(handle, jobz, uplo, n, A, lda, W, work, lwork, info, params, batchSize) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnDsyevjBatched"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnCheevjBatched(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut cuComplex,
    lda: ::std::os::raw::c_int,
    W: *mut f32,
    work: *mut cuComplex,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    params: syevjInfo_t,
    batchSize: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnCheevjBatched {
        Some(____func) => unsafe { ____func(handle, jobz, uplo, n, A, lda, W, work, lwork, info, params, batchSize) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnCheevjBatched"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZheevjBatched(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    W: *mut f64,
    work: *mut cuDoubleComplex,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    params: syevjInfo_t,
    batchSize: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZheevjBatched {
        Some(____func) => unsafe { ____func(handle, jobz, uplo, n, A, lda, W, work, lwork, info, params, batchSize) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnZheevjBatched"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSsyevj_bufferSize(handle: cusolverDnHandle_t, jobz: cusolverEigMode_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *const f32, lda: ::std::os::raw::c_int, W: *const f32, lwork: *mut ::std::os::raw::c_int, params: syevjInfo_t) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSsyevj_bufferSize {
        Some(____func) => unsafe { ____func(handle, jobz, uplo, n, A, lda, W, lwork, params) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnSsyevj_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDsyevj_bufferSize(handle: cusolverDnHandle_t, jobz: cusolverEigMode_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *const f64, lda: ::std::os::raw::c_int, W: *const f64, lwork: *mut ::std::os::raw::c_int, params: syevjInfo_t) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDsyevj_bufferSize {
        Some(____func) => unsafe { ____func(handle, jobz, uplo, n, A, lda, W, lwork, params) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnDsyevj_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnCheevj_bufferSize(handle: cusolverDnHandle_t, jobz: cusolverEigMode_t, uplo: cublasFillMode_t, n: ::std::os::raw::c_int, A: *const cuComplex, lda: ::std::os::raw::c_int, W: *const f32, lwork: *mut ::std::os::raw::c_int, params: syevjInfo_t) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnCheevj_bufferSize {
        Some(____func) => unsafe { ____func(handle, jobz, uplo, n, A, lda, W, lwork, params) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnCheevj_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZheevj_bufferSize(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    W: *const f64,
    lwork: *mut ::std::os::raw::c_int,
    params: syevjInfo_t,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZheevj_bufferSize {
        Some(____func) => unsafe { ____func(handle, jobz, uplo, n, A, lda, W, lwork, params) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnZheevj_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSsyevj(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut f32,
    lda: ::std::os::raw::c_int,
    W: *mut f32,
    work: *mut f32,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    params: syevjInfo_t,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSsyevj {
        Some(____func) => unsafe { ____func(handle, jobz, uplo, n, A, lda, W, work, lwork, info, params) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnSsyevj"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDsyevj(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut f64,
    lda: ::std::os::raw::c_int,
    W: *mut f64,
    work: *mut f64,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    params: syevjInfo_t,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDsyevj {
        Some(____func) => unsafe { ____func(handle, jobz, uplo, n, A, lda, W, work, lwork, info, params) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnDsyevj"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnCheevj(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut cuComplex,
    lda: ::std::os::raw::c_int,
    W: *mut f32,
    work: *mut cuComplex,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    params: syevjInfo_t,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnCheevj {
        Some(____func) => unsafe { ____func(handle, jobz, uplo, n, A, lda, W, work, lwork, info, params) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnCheevj"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZheevj(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    W: *mut f64,
    work: *mut cuDoubleComplex,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    params: syevjInfo_t,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZheevj {
        Some(____func) => unsafe { ____func(handle, jobz, uplo, n, A, lda, W, work, lwork, info, params) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnZheevj"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSsygvj_bufferSize(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    B: *const f32,
    ldb: ::std::os::raw::c_int,
    W: *const f32,
    lwork: *mut ::std::os::raw::c_int,
    params: syevjInfo_t,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSsygvj_bufferSize {
        Some(____func) => unsafe { ____func(handle, itype, jobz, uplo, n, A, lda, B, ldb, W, lwork, params) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnSsygvj_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDsygvj_bufferSize(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    B: *const f64,
    ldb: ::std::os::raw::c_int,
    W: *const f64,
    lwork: *mut ::std::os::raw::c_int,
    params: syevjInfo_t,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDsygvj_bufferSize {
        Some(____func) => unsafe { ____func(handle, itype, jobz, uplo, n, A, lda, B, ldb, W, lwork, params) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnDsygvj_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnChegvj_bufferSize(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    B: *const cuComplex,
    ldb: ::std::os::raw::c_int,
    W: *const f32,
    lwork: *mut ::std::os::raw::c_int,
    params: syevjInfo_t,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnChegvj_bufferSize {
        Some(____func) => unsafe { ____func(handle, itype, jobz, uplo, n, A, lda, B, ldb, W, lwork, params) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnChegvj_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZhegvj_bufferSize(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    B: *const cuDoubleComplex,
    ldb: ::std::os::raw::c_int,
    W: *const f64,
    lwork: *mut ::std::os::raw::c_int,
    params: syevjInfo_t,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZhegvj_bufferSize {
        Some(____func) => unsafe { ____func(handle, itype, jobz, uplo, n, A, lda, B, ldb, W, lwork, params) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnZhegvj_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSsygvj(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut f32,
    lda: ::std::os::raw::c_int,
    B: *mut f32,
    ldb: ::std::os::raw::c_int,
    W: *mut f32,
    work: *mut f32,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    params: syevjInfo_t,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSsygvj {
        Some(____func) => unsafe { ____func(handle, itype, jobz, uplo, n, A, lda, B, ldb, W, work, lwork, info, params) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnSsygvj"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDsygvj(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut f64,
    lda: ::std::os::raw::c_int,
    B: *mut f64,
    ldb: ::std::os::raw::c_int,
    W: *mut f64,
    work: *mut f64,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    params: syevjInfo_t,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDsygvj {
        Some(____func) => unsafe { ____func(handle, itype, jobz, uplo, n, A, lda, B, ldb, W, work, lwork, info, params) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnDsygvj"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnChegvj(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut cuComplex,
    lda: ::std::os::raw::c_int,
    B: *mut cuComplex,
    ldb: ::std::os::raw::c_int,
    W: *mut f32,
    work: *mut cuComplex,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    params: syevjInfo_t,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnChegvj {
        Some(____func) => unsafe { ____func(handle, itype, jobz, uplo, n, A, lda, B, ldb, W, work, lwork, info, params) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnChegvj"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZhegvj(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    B: *mut cuDoubleComplex,
    ldb: ::std::os::raw::c_int,
    W: *mut f64,
    work: *mut cuDoubleComplex,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    params: syevjInfo_t,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZhegvj {
        Some(____func) => unsafe { ____func(handle, itype, jobz, uplo, n, A, lda, B, ldb, W, work, lwork, info, params) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnZhegvj"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnCreateGesvdjInfo(info: *mut gesvdjInfo_t) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnCreateGesvdjInfo {
        Some(____func) => unsafe { ____func(info) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnCreateGesvdjInfo"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDestroyGesvdjInfo(info: gesvdjInfo_t) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDestroyGesvdjInfo {
        Some(____func) => unsafe { ____func(info) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnDestroyGesvdjInfo"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnXgesvdjSetTolerance(info: gesvdjInfo_t, tolerance: f64) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnXgesvdjSetTolerance {
        Some(____func) => unsafe { ____func(info, tolerance) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnXgesvdjSetTolerance"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnXgesvdjSetMaxSweeps(info: gesvdjInfo_t, max_sweeps: ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnXgesvdjSetMaxSweeps {
        Some(____func) => unsafe { ____func(info, max_sweeps) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnXgesvdjSetMaxSweeps"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnXgesvdjSetSortEig(info: gesvdjInfo_t, sort_svd: ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnXgesvdjSetSortEig {
        Some(____func) => unsafe { ____func(info, sort_svd) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnXgesvdjSetSortEig"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnXgesvdjGetResidual(handle: cusolverDnHandle_t, info: gesvdjInfo_t, residual: *mut f64) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnXgesvdjGetResidual {
        Some(____func) => unsafe { ____func(handle, info, residual) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnXgesvdjGetResidual"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnXgesvdjGetSweeps(handle: cusolverDnHandle_t, info: gesvdjInfo_t, executed_sweeps: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnXgesvdjGetSweeps {
        Some(____func) => unsafe { ____func(handle, info, executed_sweeps) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnXgesvdjGetSweeps"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSgesvdjBatched_bufferSize(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    S: *const f32,
    U: *const f32,
    ldu: ::std::os::raw::c_int,
    V: *const f32,
    ldv: ::std::os::raw::c_int,
    lwork: *mut ::std::os::raw::c_int,
    params: gesvdjInfo_t,
    batchSize: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSgesvdjBatched_bufferSize {
        Some(____func) => unsafe { ____func(handle, jobz, m, n, A, lda, S, U, ldu, V, ldv, lwork, params, batchSize) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnSgesvdjBatched_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDgesvdjBatched_bufferSize(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    S: *const f64,
    U: *const f64,
    ldu: ::std::os::raw::c_int,
    V: *const f64,
    ldv: ::std::os::raw::c_int,
    lwork: *mut ::std::os::raw::c_int,
    params: gesvdjInfo_t,
    batchSize: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDgesvdjBatched_bufferSize {
        Some(____func) => unsafe { ____func(handle, jobz, m, n, A, lda, S, U, ldu, V, ldv, lwork, params, batchSize) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnDgesvdjBatched_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnCgesvdjBatched_bufferSize(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    S: *const f32,
    U: *const cuComplex,
    ldu: ::std::os::raw::c_int,
    V: *const cuComplex,
    ldv: ::std::os::raw::c_int,
    lwork: *mut ::std::os::raw::c_int,
    params: gesvdjInfo_t,
    batchSize: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnCgesvdjBatched_bufferSize {
        Some(____func) => unsafe { ____func(handle, jobz, m, n, A, lda, S, U, ldu, V, ldv, lwork, params, batchSize) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnCgesvdjBatched_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZgesvdjBatched_bufferSize(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    S: *const f64,
    U: *const cuDoubleComplex,
    ldu: ::std::os::raw::c_int,
    V: *const cuDoubleComplex,
    ldv: ::std::os::raw::c_int,
    lwork: *mut ::std::os::raw::c_int,
    params: gesvdjInfo_t,
    batchSize: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZgesvdjBatched_bufferSize {
        Some(____func) => unsafe { ____func(handle, jobz, m, n, A, lda, S, U, ldu, V, ldv, lwork, params, batchSize) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnZgesvdjBatched_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSgesvdjBatched(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *mut f32,
    lda: ::std::os::raw::c_int,
    S: *mut f32,
    U: *mut f32,
    ldu: ::std::os::raw::c_int,
    V: *mut f32,
    ldv: ::std::os::raw::c_int,
    work: *mut f32,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    params: gesvdjInfo_t,
    batchSize: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSgesvdjBatched {
        Some(____func) => unsafe { ____func(handle, jobz, m, n, A, lda, S, U, ldu, V, ldv, work, lwork, info, params, batchSize) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnSgesvdjBatched"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDgesvdjBatched(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *mut f64,
    lda: ::std::os::raw::c_int,
    S: *mut f64,
    U: *mut f64,
    ldu: ::std::os::raw::c_int,
    V: *mut f64,
    ldv: ::std::os::raw::c_int,
    work: *mut f64,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    params: gesvdjInfo_t,
    batchSize: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDgesvdjBatched {
        Some(____func) => unsafe { ____func(handle, jobz, m, n, A, lda, S, U, ldu, V, ldv, work, lwork, info, params, batchSize) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnDgesvdjBatched"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnCgesvdjBatched(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *mut cuComplex,
    lda: ::std::os::raw::c_int,
    S: *mut f32,
    U: *mut cuComplex,
    ldu: ::std::os::raw::c_int,
    V: *mut cuComplex,
    ldv: ::std::os::raw::c_int,
    work: *mut cuComplex,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    params: gesvdjInfo_t,
    batchSize: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnCgesvdjBatched {
        Some(____func) => unsafe { ____func(handle, jobz, m, n, A, lda, S, U, ldu, V, ldv, work, lwork, info, params, batchSize) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnCgesvdjBatched"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZgesvdjBatched(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    S: *mut f64,
    U: *mut cuDoubleComplex,
    ldu: ::std::os::raw::c_int,
    V: *mut cuDoubleComplex,
    ldv: ::std::os::raw::c_int,
    work: *mut cuDoubleComplex,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    params: gesvdjInfo_t,
    batchSize: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZgesvdjBatched {
        Some(____func) => unsafe { ____func(handle, jobz, m, n, A, lda, S, U, ldu, V, ldv, work, lwork, info, params, batchSize) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnZgesvdjBatched"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSgesvdj_bufferSize(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    econ: ::std::os::raw::c_int,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    S: *const f32,
    U: *const f32,
    ldu: ::std::os::raw::c_int,
    V: *const f32,
    ldv: ::std::os::raw::c_int,
    lwork: *mut ::std::os::raw::c_int,
    params: gesvdjInfo_t,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSgesvdj_bufferSize {
        Some(____func) => unsafe { ____func(handle, jobz, econ, m, n, A, lda, S, U, ldu, V, ldv, lwork, params) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnSgesvdj_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDgesvdj_bufferSize(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    econ: ::std::os::raw::c_int,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    S: *const f64,
    U: *const f64,
    ldu: ::std::os::raw::c_int,
    V: *const f64,
    ldv: ::std::os::raw::c_int,
    lwork: *mut ::std::os::raw::c_int,
    params: gesvdjInfo_t,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDgesvdj_bufferSize {
        Some(____func) => unsafe { ____func(handle, jobz, econ, m, n, A, lda, S, U, ldu, V, ldv, lwork, params) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnDgesvdj_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnCgesvdj_bufferSize(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    econ: ::std::os::raw::c_int,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    S: *const f32,
    U: *const cuComplex,
    ldu: ::std::os::raw::c_int,
    V: *const cuComplex,
    ldv: ::std::os::raw::c_int,
    lwork: *mut ::std::os::raw::c_int,
    params: gesvdjInfo_t,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnCgesvdj_bufferSize {
        Some(____func) => unsafe { ____func(handle, jobz, econ, m, n, A, lda, S, U, ldu, V, ldv, lwork, params) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnCgesvdj_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZgesvdj_bufferSize(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    econ: ::std::os::raw::c_int,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    S: *const f64,
    U: *const cuDoubleComplex,
    ldu: ::std::os::raw::c_int,
    V: *const cuDoubleComplex,
    ldv: ::std::os::raw::c_int,
    lwork: *mut ::std::os::raw::c_int,
    params: gesvdjInfo_t,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZgesvdj_bufferSize {
        Some(____func) => unsafe { ____func(handle, jobz, econ, m, n, A, lda, S, U, ldu, V, ldv, lwork, params) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnZgesvdj_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSgesvdj(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    econ: ::std::os::raw::c_int,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *mut f32,
    lda: ::std::os::raw::c_int,
    S: *mut f32,
    U: *mut f32,
    ldu: ::std::os::raw::c_int,
    V: *mut f32,
    ldv: ::std::os::raw::c_int,
    work: *mut f32,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    params: gesvdjInfo_t,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSgesvdj {
        Some(____func) => unsafe { ____func(handle, jobz, econ, m, n, A, lda, S, U, ldu, V, ldv, work, lwork, info, params) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnSgesvdj"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDgesvdj(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    econ: ::std::os::raw::c_int,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *mut f64,
    lda: ::std::os::raw::c_int,
    S: *mut f64,
    U: *mut f64,
    ldu: ::std::os::raw::c_int,
    V: *mut f64,
    ldv: ::std::os::raw::c_int,
    work: *mut f64,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    params: gesvdjInfo_t,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDgesvdj {
        Some(____func) => unsafe { ____func(handle, jobz, econ, m, n, A, lda, S, U, ldu, V, ldv, work, lwork, info, params) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnDgesvdj"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnCgesvdj(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    econ: ::std::os::raw::c_int,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *mut cuComplex,
    lda: ::std::os::raw::c_int,
    S: *mut f32,
    U: *mut cuComplex,
    ldu: ::std::os::raw::c_int,
    V: *mut cuComplex,
    ldv: ::std::os::raw::c_int,
    work: *mut cuComplex,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    params: gesvdjInfo_t,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnCgesvdj {
        Some(____func) => unsafe { ____func(handle, jobz, econ, m, n, A, lda, S, U, ldu, V, ldv, work, lwork, info, params) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnCgesvdj"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZgesvdj(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    econ: ::std::os::raw::c_int,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    S: *mut f64,
    U: *mut cuDoubleComplex,
    ldu: ::std::os::raw::c_int,
    V: *mut cuDoubleComplex,
    ldv: ::std::os::raw::c_int,
    work: *mut cuDoubleComplex,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    params: gesvdjInfo_t,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZgesvdj {
        Some(____func) => unsafe { ____func(handle, jobz, econ, m, n, A, lda, S, U, ldu, V, ldv, work, lwork, info, params) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnZgesvdj"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSgesvdaStridedBatched_bufferSize(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    rank: ::std::os::raw::c_int,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    d_A: *const f32,
    lda: ::std::os::raw::c_int,
    strideA: ::std::os::raw::c_longlong,
    d_S: *const f32,
    strideS: ::std::os::raw::c_longlong,
    d_U: *const f32,
    ldu: ::std::os::raw::c_int,
    strideU: ::std::os::raw::c_longlong,
    d_V: *const f32,
    ldv: ::std::os::raw::c_int,
    strideV: ::std::os::raw::c_longlong,
    lwork: *mut ::std::os::raw::c_int,
    batchSize: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSgesvdaStridedBatched_bufferSize {
        Some(____func) => unsafe { ____func(handle, jobz, rank, m, n, d_A, lda, strideA, d_S, strideS, d_U, ldu, strideU, d_V, ldv, strideV, lwork, batchSize) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnSgesvdaStridedBatched_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDgesvdaStridedBatched_bufferSize(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    rank: ::std::os::raw::c_int,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    d_A: *const f64,
    lda: ::std::os::raw::c_int,
    strideA: ::std::os::raw::c_longlong,
    d_S: *const f64,
    strideS: ::std::os::raw::c_longlong,
    d_U: *const f64,
    ldu: ::std::os::raw::c_int,
    strideU: ::std::os::raw::c_longlong,
    d_V: *const f64,
    ldv: ::std::os::raw::c_int,
    strideV: ::std::os::raw::c_longlong,
    lwork: *mut ::std::os::raw::c_int,
    batchSize: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDgesvdaStridedBatched_bufferSize {
        Some(____func) => unsafe { ____func(handle, jobz, rank, m, n, d_A, lda, strideA, d_S, strideS, d_U, ldu, strideU, d_V, ldv, strideV, lwork, batchSize) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnDgesvdaStridedBatched_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnCgesvdaStridedBatched_bufferSize(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    rank: ::std::os::raw::c_int,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    d_A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    strideA: ::std::os::raw::c_longlong,
    d_S: *const f32,
    strideS: ::std::os::raw::c_longlong,
    d_U: *const cuComplex,
    ldu: ::std::os::raw::c_int,
    strideU: ::std::os::raw::c_longlong,
    d_V: *const cuComplex,
    ldv: ::std::os::raw::c_int,
    strideV: ::std::os::raw::c_longlong,
    lwork: *mut ::std::os::raw::c_int,
    batchSize: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnCgesvdaStridedBatched_bufferSize {
        Some(____func) => unsafe { ____func(handle, jobz, rank, m, n, d_A, lda, strideA, d_S, strideS, d_U, ldu, strideU, d_V, ldv, strideV, lwork, batchSize) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnCgesvdaStridedBatched_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZgesvdaStridedBatched_bufferSize(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    rank: ::std::os::raw::c_int,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    d_A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    strideA: ::std::os::raw::c_longlong,
    d_S: *const f64,
    strideS: ::std::os::raw::c_longlong,
    d_U: *const cuDoubleComplex,
    ldu: ::std::os::raw::c_int,
    strideU: ::std::os::raw::c_longlong,
    d_V: *const cuDoubleComplex,
    ldv: ::std::os::raw::c_int,
    strideV: ::std::os::raw::c_longlong,
    lwork: *mut ::std::os::raw::c_int,
    batchSize: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZgesvdaStridedBatched_bufferSize {
        Some(____func) => unsafe { ____func(handle, jobz, rank, m, n, d_A, lda, strideA, d_S, strideS, d_U, ldu, strideU, d_V, ldv, strideV, lwork, batchSize) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnZgesvdaStridedBatched_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSgesvdaStridedBatched(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    rank: ::std::os::raw::c_int,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    d_A: *const f32,
    lda: ::std::os::raw::c_int,
    strideA: ::std::os::raw::c_longlong,
    d_S: *mut f32,
    strideS: ::std::os::raw::c_longlong,
    d_U: *mut f32,
    ldu: ::std::os::raw::c_int,
    strideU: ::std::os::raw::c_longlong,
    d_V: *mut f32,
    ldv: ::std::os::raw::c_int,
    strideV: ::std::os::raw::c_longlong,
    d_work: *mut f32,
    lwork: ::std::os::raw::c_int,
    d_info: *mut ::std::os::raw::c_int,
    h_R_nrmF: *mut f64,
    batchSize: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSgesvdaStridedBatched {
        Some(____func) => unsafe { ____func(handle, jobz, rank, m, n, d_A, lda, strideA, d_S, strideS, d_U, ldu, strideU, d_V, ldv, strideV, d_work, lwork, d_info, h_R_nrmF, batchSize) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnSgesvdaStridedBatched"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDgesvdaStridedBatched(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    rank: ::std::os::raw::c_int,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    d_A: *const f64,
    lda: ::std::os::raw::c_int,
    strideA: ::std::os::raw::c_longlong,
    d_S: *mut f64,
    strideS: ::std::os::raw::c_longlong,
    d_U: *mut f64,
    ldu: ::std::os::raw::c_int,
    strideU: ::std::os::raw::c_longlong,
    d_V: *mut f64,
    ldv: ::std::os::raw::c_int,
    strideV: ::std::os::raw::c_longlong,
    d_work: *mut f64,
    lwork: ::std::os::raw::c_int,
    d_info: *mut ::std::os::raw::c_int,
    h_R_nrmF: *mut f64,
    batchSize: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDgesvdaStridedBatched {
        Some(____func) => unsafe { ____func(handle, jobz, rank, m, n, d_A, lda, strideA, d_S, strideS, d_U, ldu, strideU, d_V, ldv, strideV, d_work, lwork, d_info, h_R_nrmF, batchSize) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnDgesvdaStridedBatched"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnCgesvdaStridedBatched(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    rank: ::std::os::raw::c_int,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    d_A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    strideA: ::std::os::raw::c_longlong,
    d_S: *mut f32,
    strideS: ::std::os::raw::c_longlong,
    d_U: *mut cuComplex,
    ldu: ::std::os::raw::c_int,
    strideU: ::std::os::raw::c_longlong,
    d_V: *mut cuComplex,
    ldv: ::std::os::raw::c_int,
    strideV: ::std::os::raw::c_longlong,
    d_work: *mut cuComplex,
    lwork: ::std::os::raw::c_int,
    d_info: *mut ::std::os::raw::c_int,
    h_R_nrmF: *mut f64,
    batchSize: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnCgesvdaStridedBatched {
        Some(____func) => unsafe { ____func(handle, jobz, rank, m, n, d_A, lda, strideA, d_S, strideS, d_U, ldu, strideU, d_V, ldv, strideV, d_work, lwork, d_info, h_R_nrmF, batchSize) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnCgesvdaStridedBatched"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnZgesvdaStridedBatched(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    rank: ::std::os::raw::c_int,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    d_A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    strideA: ::std::os::raw::c_longlong,
    d_S: *mut f64,
    strideS: ::std::os::raw::c_longlong,
    d_U: *mut cuDoubleComplex,
    ldu: ::std::os::raw::c_int,
    strideU: ::std::os::raw::c_longlong,
    d_V: *mut cuDoubleComplex,
    ldv: ::std::os::raw::c_int,
    strideV: ::std::os::raw::c_longlong,
    d_work: *mut cuDoubleComplex,
    lwork: ::std::os::raw::c_int,
    d_info: *mut ::std::os::raw::c_int,
    h_R_nrmF: *mut f64,
    batchSize: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnZgesvdaStridedBatched {
        Some(____func) => unsafe { ____func(handle, jobz, rank, m, n, d_A, lda, strideA, d_S, strideS, d_U, ldu, strideU, d_V, ldv, strideV, d_work, lwork, d_info, h_R_nrmF, batchSize) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnZgesvdaStridedBatched"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnCreateParams(params: *mut cusolverDnParams_t) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnCreateParams {
        Some(____func) => unsafe { ____func(params) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnCreateParams"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnDestroyParams(params: cusolverDnParams_t) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnDestroyParams {
        Some(____func) => unsafe { ____func(params) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnDestroyParams"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnSetAdvOptions(params: cusolverDnParams_t, function: cusolverDnFunction_t, algo: cusolverAlgMode_t) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnSetAdvOptions {
        Some(____func) => unsafe { ____func(params, function, algo) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnSetAdvOptions"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnXpotrf_bufferSize(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    uplo: cublasFillMode_t,
    n: i64,
    dataTypeA: cudaDataType,
    A: *const ::std::os::raw::c_void,
    lda: i64,
    computeType: cudaDataType,
    workspaceInBytesOnDevice: *mut usize,
    workspaceInBytesOnHost: *mut usize,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnXpotrf_bufferSize {
        Some(____func) => unsafe { ____func(handle, params, uplo, n, dataTypeA, A, lda, computeType, workspaceInBytesOnDevice, workspaceInBytesOnHost) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnXpotrf_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnXpotrf(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    uplo: cublasFillMode_t,
    n: i64,
    dataTypeA: cudaDataType,
    A: *mut ::std::os::raw::c_void,
    lda: i64,
    computeType: cudaDataType,
    bufferOnDevice: *mut ::std::os::raw::c_void,
    workspaceInBytesOnDevice: usize,
    bufferOnHost: *mut ::std::os::raw::c_void,
    workspaceInBytesOnHost: usize,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnXpotrf {
        Some(____func) => unsafe { ____func(handle, params, uplo, n, dataTypeA, A, lda, computeType, bufferOnDevice, workspaceInBytesOnDevice, bufferOnHost, workspaceInBytesOnHost, info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnXpotrf"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnXpotrs(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    uplo: cublasFillMode_t,
    n: i64,
    nrhs: i64,
    dataTypeA: cudaDataType,
    A: *const ::std::os::raw::c_void,
    lda: i64,
    dataTypeB: cudaDataType,
    B: *mut ::std::os::raw::c_void,
    ldb: i64,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnXpotrs {
        Some(____func) => unsafe { ____func(handle, params, uplo, n, nrhs, dataTypeA, A, lda, dataTypeB, B, ldb, info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnXpotrs"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnXgeqrf_bufferSize(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    m: i64,
    n: i64,
    dataTypeA: cudaDataType,
    A: *const ::std::os::raw::c_void,
    lda: i64,
    dataTypeTau: cudaDataType,
    tau: *const ::std::os::raw::c_void,
    computeType: cudaDataType,
    workspaceInBytesOnDevice: *mut usize,
    workspaceInBytesOnHost: *mut usize,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnXgeqrf_bufferSize {
        Some(____func) => unsafe { ____func(handle, params, m, n, dataTypeA, A, lda, dataTypeTau, tau, computeType, workspaceInBytesOnDevice, workspaceInBytesOnHost) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnXgeqrf_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnXgeqrf(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    m: i64,
    n: i64,
    dataTypeA: cudaDataType,
    A: *mut ::std::os::raw::c_void,
    lda: i64,
    dataTypeTau: cudaDataType,
    tau: *mut ::std::os::raw::c_void,
    computeType: cudaDataType,
    bufferOnDevice: *mut ::std::os::raw::c_void,
    workspaceInBytesOnDevice: usize,
    bufferOnHost: *mut ::std::os::raw::c_void,
    workspaceInBytesOnHost: usize,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnXgeqrf {
        Some(____func) => unsafe { ____func(handle, params, m, n, dataTypeA, A, lda, dataTypeTau, tau, computeType, bufferOnDevice, workspaceInBytesOnDevice, bufferOnHost, workspaceInBytesOnHost, info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnXgeqrf"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnXgetrf_bufferSize(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    m: i64,
    n: i64,
    dataTypeA: cudaDataType,
    A: *const ::std::os::raw::c_void,
    lda: i64,
    computeType: cudaDataType,
    workspaceInBytesOnDevice: *mut usize,
    workspaceInBytesOnHost: *mut usize,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnXgetrf_bufferSize {
        Some(____func) => unsafe { ____func(handle, params, m, n, dataTypeA, A, lda, computeType, workspaceInBytesOnDevice, workspaceInBytesOnHost) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnXgetrf_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnXgetrf(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    m: i64,
    n: i64,
    dataTypeA: cudaDataType,
    A: *mut ::std::os::raw::c_void,
    lda: i64,
    ipiv: *mut i64,
    computeType: cudaDataType,
    bufferOnDevice: *mut ::std::os::raw::c_void,
    workspaceInBytesOnDevice: usize,
    bufferOnHost: *mut ::std::os::raw::c_void,
    workspaceInBytesOnHost: usize,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnXgetrf {
        Some(____func) => unsafe { ____func(handle, params, m, n, dataTypeA, A, lda, ipiv, computeType, bufferOnDevice, workspaceInBytesOnDevice, bufferOnHost, workspaceInBytesOnHost, info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnXgetrf"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnXgetrs(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    trans: cublasOperation_t,
    n: i64,
    nrhs: i64,
    dataTypeA: cudaDataType,
    A: *const ::std::os::raw::c_void,
    lda: i64,
    ipiv: *const i64,
    dataTypeB: cudaDataType,
    B: *mut ::std::os::raw::c_void,
    ldb: i64,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnXgetrs {
        Some(____func) => unsafe { ____func(handle, params, trans, n, nrhs, dataTypeA, A, lda, ipiv, dataTypeB, B, ldb, info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnXgetrs"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnXsyevd_bufferSize(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i64,
    dataTypeA: cudaDataType,
    A: *const ::std::os::raw::c_void,
    lda: i64,
    dataTypeW: cudaDataType,
    W: *const ::std::os::raw::c_void,
    computeType: cudaDataType,
    workspaceInBytesOnDevice: *mut usize,
    workspaceInBytesOnHost: *mut usize,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnXsyevd_bufferSize {
        Some(____func) => unsafe { ____func(handle, params, jobz, uplo, n, dataTypeA, A, lda, dataTypeW, W, computeType, workspaceInBytesOnDevice, workspaceInBytesOnHost) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnXsyevd_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnXsyevd(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i64,
    dataTypeA: cudaDataType,
    A: *mut ::std::os::raw::c_void,
    lda: i64,
    dataTypeW: cudaDataType,
    W: *mut ::std::os::raw::c_void,
    computeType: cudaDataType,
    bufferOnDevice: *mut ::std::os::raw::c_void,
    workspaceInBytesOnDevice: usize,
    bufferOnHost: *mut ::std::os::raw::c_void,
    workspaceInBytesOnHost: usize,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnXsyevd {
        Some(____func) => unsafe { ____func(handle, params, jobz, uplo, n, dataTypeA, A, lda, dataTypeW, W, computeType, bufferOnDevice, workspaceInBytesOnDevice, bufferOnHost, workspaceInBytesOnHost, info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnXsyevd"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnXstedc_bufferSize(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    compz: cusolverEigComp_t,
    n: i64,
    dataTypeDE: cudaDataType,
    D: *const ::std::os::raw::c_void,
    E: *const ::std::os::raw::c_void,
    dataTypeZ: cudaDataType,
    Z: *const ::std::os::raw::c_void,
    ldz: i64,
    computeType: cudaDataType,
    workspaceInBytesOnDevice: *mut usize,
    workspaceInBytesOnHost: *mut usize,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnXstedc_bufferSize {
        Some(____func) => unsafe { ____func(handle, params, compz, n, dataTypeDE, D, E, dataTypeZ, Z, ldz, computeType, workspaceInBytesOnDevice, workspaceInBytesOnHost) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnXstedc_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnXstedc(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    compz: cusolverEigComp_t,
    n: i64,
    dataTypeDE: cudaDataType,
    D: *mut ::std::os::raw::c_void,
    E: *mut ::std::os::raw::c_void,
    dataTypeZ: cudaDataType,
    Z: *mut ::std::os::raw::c_void,
    ldz: i64,
    computeType: cudaDataType,
    bufferOnDevice: *mut ::std::os::raw::c_void,
    workspaceInBytesOnDevice: usize,
    bufferOnHost: *mut ::std::os::raw::c_void,
    workspaceInBytesOnHost: usize,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnXstedc {
        Some(____func) => unsafe { ____func(handle, params, compz, n, dataTypeDE, D, E, dataTypeZ, Z, ldz, computeType, bufferOnDevice, workspaceInBytesOnDevice, bufferOnHost, workspaceInBytesOnHost, info) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnXstedc"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnXsyevBatched_bufferSize(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i64,
    dataTypeA: cudaDataType,
    A: *const ::std::os::raw::c_void,
    lda: i64,
    dataTypeW: cudaDataType,
    W: *const ::std::os::raw::c_void,
    computeType: cudaDataType,
    workspaceInBytesOnDevice: *mut usize,
    workspaceInBytesOnHost: *mut usize,
    batchSize: i64,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnXsyevBatched_bufferSize {
        Some(____func) => unsafe { ____func(handle, params, jobz, uplo, n, dataTypeA, A, lda, dataTypeW, W, computeType, workspaceInBytesOnDevice, workspaceInBytesOnHost, batchSize) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnXsyevBatched_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnXsyevBatched(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i64,
    dataTypeA: cudaDataType,
    A: *mut ::std::os::raw::c_void,
    lda: i64,
    dataTypeW: cudaDataType,
    W: *mut ::std::os::raw::c_void,
    computeType: cudaDataType,
    bufferOnDevice: *mut ::std::os::raw::c_void,
    workspaceInBytesOnDevice: usize,
    bufferOnHost: *mut ::std::os::raw::c_void,
    workspaceInBytesOnHost: usize,
    info: *mut ::std::os::raw::c_int,
    batchSize: i64,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnXsyevBatched {
        Some(____func) => unsafe { ____func(handle, params, jobz, uplo, n, dataTypeA, A, lda, dataTypeW, W, computeType, bufferOnDevice, workspaceInBytesOnDevice, bufferOnHost, workspaceInBytesOnHost, info, batchSize) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnXsyevBatched"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnXsyevdx_bufferSize(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: i64,
    dataTypeA: cudaDataType,
    A: *const ::std::os::raw::c_void,
    lda: i64,
    vl: *mut ::std::os::raw::c_void,
    vu: *mut ::std::os::raw::c_void,
    il: i64,
    iu: i64,
    h_meig: *mut i64,
    dataTypeW: cudaDataType,
    W: *const ::std::os::raw::c_void,
    computeType: cudaDataType,
    workspaceInBytesOnDevice: *mut usize,
    workspaceInBytesOnHost: *mut usize,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnXsyevdx_bufferSize {
        Some(____func) => unsafe { ____func(handle, params, jobz, range, uplo, n, dataTypeA, A, lda, vl, vu, il, iu, h_meig, dataTypeW, W, computeType, workspaceInBytesOnDevice, workspaceInBytesOnHost) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnXsyevdx_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnXsyevdx(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: i64,
    dataTypeA: cudaDataType,
    A: *mut ::std::os::raw::c_void,
    lda: i64,
    vl: *mut ::std::os::raw::c_void,
    vu: *mut ::std::os::raw::c_void,
    il: i64,
    iu: i64,
    meig64: *mut i64,
    dataTypeW: cudaDataType,
    W: *mut ::std::os::raw::c_void,
    computeType: cudaDataType,
    bufferOnDevice: *mut ::std::os::raw::c_void,
    workspaceInBytesOnDevice: usize,
    bufferOnHost: *mut ::std::os::raw::c_void,
    workspaceInBytesOnHost: usize,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnXsyevdx {
        Some(____func) => unsafe {
            ____func(
                handle,
                params,
                jobz,
                range,
                uplo,
                n,
                dataTypeA,
                A,
                lda,
                vl,
                vu,
                il,
                iu,
                meig64,
                dataTypeW,
                W,
                computeType,
                bufferOnDevice,
                workspaceInBytesOnDevice,
                bufferOnHost,
                workspaceInBytesOnHost,
                info,
            )
        },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnXsyevdx"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnXgeev_bufferSize(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    jobvl: cusolverEigMode_t,
    jobvr: cusolverEigMode_t,
    n: i64,
    dataTypeA: cudaDataType,
    A: *const ::std::os::raw::c_void,
    lda: i64,
    dataTypeW: cudaDataType,
    W: *const ::std::os::raw::c_void,
    dataTypeVL: cudaDataType,
    VL: *const ::std::os::raw::c_void,
    ldvl: i64,
    dataTypeVR: cudaDataType,
    VR: *const ::std::os::raw::c_void,
    ldvr: i64,
    computeType: cudaDataType,
    workspaceInBytesOnDevice: *mut usize,
    workspaceInBytesOnHost: *mut usize,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnXgeev_bufferSize {
        Some(____func) => unsafe { ____func(handle, params, jobvl, jobvr, n, dataTypeA, A, lda, dataTypeW, W, dataTypeVL, VL, ldvl, dataTypeVR, VR, ldvr, computeType, workspaceInBytesOnDevice, workspaceInBytesOnHost) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnXgeev_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnXgeev(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    jobvl: cusolverEigMode_t,
    jobvr: cusolverEigMode_t,
    n: i64,
    dataTypeA: cudaDataType,
    A: *mut ::std::os::raw::c_void,
    lda: i64,
    dataTypeW: cudaDataType,
    W: *mut ::std::os::raw::c_void,
    dataTypeVL: cudaDataType,
    VL: *mut ::std::os::raw::c_void,
    ldvl: i64,
    dataTypeVR: cudaDataType,
    VR: *mut ::std::os::raw::c_void,
    ldvr: i64,
    computeType: cudaDataType,
    bufferOnDevice: *mut ::std::os::raw::c_void,
    workspaceInBytesOnDevice: usize,
    bufferOnHost: *mut ::std::os::raw::c_void,
    workspaceInBytesOnHost: usize,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnXgeev {
        Some(____func) => unsafe {
            ____func(
                handle,
                params,
                jobvl,
                jobvr,
                n,
                dataTypeA,
                A,
                lda,
                dataTypeW,
                W,
                dataTypeVL,
                VL,
                ldvl,
                dataTypeVR,
                VR,
                ldvr,
                computeType,
                bufferOnDevice,
                workspaceInBytesOnDevice,
                bufferOnHost,
                workspaceInBytesOnHost,
                info,
            )
        },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnXgeev"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnXgesvd_bufferSize(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    jobu: ::std::os::raw::c_schar,
    jobvt: ::std::os::raw::c_schar,
    m: i64,
    n: i64,
    dataTypeA: cudaDataType,
    A: *const ::std::os::raw::c_void,
    lda: i64,
    dataTypeS: cudaDataType,
    S: *const ::std::os::raw::c_void,
    dataTypeU: cudaDataType,
    U: *const ::std::os::raw::c_void,
    ldu: i64,
    dataTypeVT: cudaDataType,
    VT: *const ::std::os::raw::c_void,
    ldvt: i64,
    computeType: cudaDataType,
    workspaceInBytesOnDevice: *mut usize,
    workspaceInBytesOnHost: *mut usize,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnXgesvd_bufferSize {
        Some(____func) => unsafe { ____func(handle, params, jobu, jobvt, m, n, dataTypeA, A, lda, dataTypeS, S, dataTypeU, U, ldu, dataTypeVT, VT, ldvt, computeType, workspaceInBytesOnDevice, workspaceInBytesOnHost) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnXgesvd_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnXgesvd(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    jobu: ::std::os::raw::c_schar,
    jobvt: ::std::os::raw::c_schar,
    m: i64,
    n: i64,
    dataTypeA: cudaDataType,
    A: *mut ::std::os::raw::c_void,
    lda: i64,
    dataTypeS: cudaDataType,
    S: *mut ::std::os::raw::c_void,
    dataTypeU: cudaDataType,
    U: *mut ::std::os::raw::c_void,
    ldu: i64,
    dataTypeVT: cudaDataType,
    VT: *mut ::std::os::raw::c_void,
    ldvt: i64,
    computeType: cudaDataType,
    bufferOnDevice: *mut ::std::os::raw::c_void,
    workspaceInBytesOnDevice: usize,
    bufferOnHost: *mut ::std::os::raw::c_void,
    workspaceInBytesOnHost: usize,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnXgesvd {
        Some(____func) => unsafe {
            ____func(
                handle,
                params,
                jobu,
                jobvt,
                m,
                n,
                dataTypeA,
                A,
                lda,
                dataTypeS,
                S,
                dataTypeU,
                U,
                ldu,
                dataTypeVT,
                VT,
                ldvt,
                computeType,
                bufferOnDevice,
                workspaceInBytesOnDevice,
                bufferOnHost,
                workspaceInBytesOnHost,
                info,
            )
        },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnXgesvd"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnXgesvdp_bufferSize(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    jobz: cusolverEigMode_t,
    econ: ::std::os::raw::c_int,
    m: i64,
    n: i64,
    dataTypeA: cudaDataType,
    A: *const ::std::os::raw::c_void,
    lda: i64,
    dataTypeS: cudaDataType,
    S: *const ::std::os::raw::c_void,
    dataTypeU: cudaDataType,
    U: *const ::std::os::raw::c_void,
    ldu: i64,
    dataTypeV: cudaDataType,
    V: *const ::std::os::raw::c_void,
    ldv: i64,
    computeType: cudaDataType,
    workspaceInBytesOnDevice: *mut usize,
    workspaceInBytesOnHost: *mut usize,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnXgesvdp_bufferSize {
        Some(____func) => unsafe { ____func(handle, params, jobz, econ, m, n, dataTypeA, A, lda, dataTypeS, S, dataTypeU, U, ldu, dataTypeV, V, ldv, computeType, workspaceInBytesOnDevice, workspaceInBytesOnHost) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnXgesvdp_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnXgesvdp(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    jobz: cusolverEigMode_t,
    econ: ::std::os::raw::c_int,
    m: i64,
    n: i64,
    dataTypeA: cudaDataType,
    A: *mut ::std::os::raw::c_void,
    lda: i64,
    dataTypeS: cudaDataType,
    S: *mut ::std::os::raw::c_void,
    dataTypeU: cudaDataType,
    U: *mut ::std::os::raw::c_void,
    ldu: i64,
    dataTypeV: cudaDataType,
    V: *mut ::std::os::raw::c_void,
    ldv: i64,
    computeType: cudaDataType,
    bufferOnDevice: *mut ::std::os::raw::c_void,
    workspaceInBytesOnDevice: usize,
    bufferOnHost: *mut ::std::os::raw::c_void,
    workspaceInBytesOnHost: usize,
    d_info: *mut ::std::os::raw::c_int,
    h_err_sigma: *mut f64,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnXgesvdp {
        Some(____func) => unsafe {
            ____func(
                handle,
                params,
                jobz,
                econ,
                m,
                n,
                dataTypeA,
                A,
                lda,
                dataTypeS,
                S,
                dataTypeU,
                U,
                ldu,
                dataTypeV,
                V,
                ldv,
                computeType,
                bufferOnDevice,
                workspaceInBytesOnDevice,
                bufferOnHost,
                workspaceInBytesOnHost,
                d_info,
                h_err_sigma,
            )
        },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnXgesvdp"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnXgesvdr_bufferSize(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    jobu: ::std::os::raw::c_schar,
    jobv: ::std::os::raw::c_schar,
    m: i64,
    n: i64,
    k: i64,
    p: i64,
    niters: i64,
    dataTypeA: cudaDataType,
    A: *const ::std::os::raw::c_void,
    lda: i64,
    dataTypeSrand: cudaDataType,
    Srand: *const ::std::os::raw::c_void,
    dataTypeUrand: cudaDataType,
    Urand: *const ::std::os::raw::c_void,
    ldUrand: i64,
    dataTypeVrand: cudaDataType,
    Vrand: *const ::std::os::raw::c_void,
    ldVrand: i64,
    computeType: cudaDataType,
    workspaceInBytesOnDevice: *mut usize,
    workspaceInBytesOnHost: *mut usize,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnXgesvdr_bufferSize {
        Some(____func) => unsafe {
            ____func(
                handle,
                params,
                jobu,
                jobv,
                m,
                n,
                k,
                p,
                niters,
                dataTypeA,
                A,
                lda,
                dataTypeSrand,
                Srand,
                dataTypeUrand,
                Urand,
                ldUrand,
                dataTypeVrand,
                Vrand,
                ldVrand,
                computeType,
                workspaceInBytesOnDevice,
                workspaceInBytesOnHost,
            )
        },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnXgesvdr_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnXgesvdr(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    jobu: ::std::os::raw::c_schar,
    jobv: ::std::os::raw::c_schar,
    m: i64,
    n: i64,
    k: i64,
    p: i64,
    niters: i64,
    dataTypeA: cudaDataType,
    A: *mut ::std::os::raw::c_void,
    lda: i64,
    dataTypeSrand: cudaDataType,
    Srand: *mut ::std::os::raw::c_void,
    dataTypeUrand: cudaDataType,
    Urand: *mut ::std::os::raw::c_void,
    ldUrand: i64,
    dataTypeVrand: cudaDataType,
    Vrand: *mut ::std::os::raw::c_void,
    ldVrand: i64,
    computeType: cudaDataType,
    bufferOnDevice: *mut ::std::os::raw::c_void,
    workspaceInBytesOnDevice: usize,
    bufferOnHost: *mut ::std::os::raw::c_void,
    workspaceInBytesOnHost: usize,
    d_info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnXgesvdr {
        Some(____func) => unsafe {
            ____func(
                handle,
                params,
                jobu,
                jobv,
                m,
                n,
                k,
                p,
                niters,
                dataTypeA,
                A,
                lda,
                dataTypeSrand,
                Srand,
                dataTypeUrand,
                Urand,
                ldUrand,
                dataTypeVrand,
                Vrand,
                ldVrand,
                computeType,
                bufferOnDevice,
                workspaceInBytesOnDevice,
                bufferOnHost,
                workspaceInBytesOnHost,
                d_info,
            )
        },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnXgesvdr"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnXlarft_bufferSize(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    direct: cusolverDirectMode_t,
    storev: cusolverStorevMode_t,
    n: i64,
    k: i64,
    dataTypeV: cudaDataType,
    V: *const ::std::os::raw::c_void,
    ldv: i64,
    dataTypeTau: cudaDataType,
    tau: *const ::std::os::raw::c_void,
    dataTypeT: cudaDataType,
    T: *mut ::std::os::raw::c_void,
    ldt: i64,
    computeType: cudaDataType,
    workspaceInBytesOnDevice: *mut usize,
    workspaceInBytesOnHost: *mut usize,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnXlarft_bufferSize {
        Some(____func) => unsafe { ____func(handle, params, direct, storev, n, k, dataTypeV, V, ldv, dataTypeTau, tau, dataTypeT, T, ldt, computeType, workspaceInBytesOnDevice, workspaceInBytesOnHost) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnXlarft_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnXlarft(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    direct: cusolverDirectMode_t,
    storev: cusolverStorevMode_t,
    n: i64,
    k: i64,
    dataTypeV: cudaDataType,
    V: *const ::std::os::raw::c_void,
    ldv: i64,
    dataTypeTau: cudaDataType,
    tau: *const ::std::os::raw::c_void,
    dataTypeT: cudaDataType,
    T: *mut ::std::os::raw::c_void,
    ldt: i64,
    computeType: cudaDataType,
    bufferOnDevice: *mut ::std::os::raw::c_void,
    workspaceInBytesOnDevice: usize,
    bufferOnHost: *mut ::std::os::raw::c_void,
    workspaceInBytesOnHost: usize,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnXlarft {
        Some(____func) => unsafe {
            ____func(
                handle,
                params,
                direct,
                storev,
                n,
                k,
                dataTypeV,
                V,
                ldv,
                dataTypeTau,
                tau,
                dataTypeT,
                T,
                ldt,
                computeType,
                bufferOnDevice,
                workspaceInBytesOnDevice,
                bufferOnHost,
                workspaceInBytesOnHost,
            )
        },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnXlarft"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnLoggerSetCallback(callback: cusolverDnLoggerCallback_t) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnLoggerSetCallback {
        Some(____func) => unsafe { ____func(callback) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnLoggerSetCallback"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnLoggerSetFile(file: *mut FILE) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnLoggerSetFile {
        Some(____func) => unsafe { ____func(file) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnLoggerSetFile"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnLoggerOpenFile(logFile: *const ::std::os::raw::c_char) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnLoggerOpenFile {
        Some(____func) => unsafe { ____func(logFile) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnLoggerOpenFile"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnLoggerSetLevel(level: ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnLoggerSetLevel {
        Some(____func) => unsafe { ____func(level) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnLoggerSetLevel"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnLoggerSetMask(mask: ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnLoggerSetMask {
        Some(____func) => unsafe { ____func(mask) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnLoggerSetMask"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnLoggerForceDisable() -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnLoggerForceDisable {
        Some(____func) => unsafe { ____func() },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnLoggerForceDisable"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnXpolar_bufferSize(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    uplo: cublasFillMode_t,
    M: i64,
    N: i64,
    dataTypeA: cudaDataType,
    A: *const ::std::os::raw::c_void,
    lda: i64,
    dataTypeH: cudaDataType,
    H: *const ::std::os::raw::c_void,
    ldh: i64,
    computeType: cudaDataType,
    workspaceInBytesOnDevice: *mut usize,
    workspaceInBytesOnHost: *mut usize,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnXpolar_bufferSize {
        Some(____func) => unsafe { ____func(handle, params, uplo, M, N, dataTypeA, A, lda, dataTypeH, H, ldh, computeType, workspaceInBytesOnDevice, workspaceInBytesOnHost) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverDnXpolar_bufferSize"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverDnXpolar(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    uplo: cublasFillMode_t,
    M: i64,
    N: i64,
    dataTypeA: cudaDataType,
    A: *mut ::std::os::raw::c_void,
    lda: i64,
    dataTypeH: cudaDataType,
    H: *mut ::std::os::raw::c_void,
    ldh: i64,
    computeType: cudaDataType,
    bufferOnDevice: *mut ::std::os::raw::c_void,
    workspaceInBytesOnDevice: usize,
    bufferOnHost: *mut ::std::os::raw::c_void,
    workspaceInBytesOnHost: usize,
    d_res_nrm: *mut f64,
    d_A_nrmF: *mut f64,
    d_rcond: *mut f64,
    d_info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverDnXpolar {
        Some(____func) => unsafe {
            ____func(
                handle,
                params,
                uplo,
                M,
                N,
                dataTypeA,
                A,
                lda,
                dataTypeH,
                H,
                ldh,
                computeType,
                bufferOnDevice,
                workspaceInBytesOnDevice,
                bufferOnHost,
                workspaceInBytesOnHost,
                d_res_nrm,
                d_A_nrmF,
                d_rcond,
                d_info,
            )
        },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverDnXpolar"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverSpCreate(handle: *mut cusolverSpHandle_t) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverSpCreate {
        Some(____func) => unsafe { ____func(handle) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverSpCreate"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverSpDestroy(handle: cusolverSpHandle_t) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverSpDestroy {
        Some(____func) => unsafe { ____func(handle) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverSpDestroy"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverSpSetStream(handle: cusolverSpHandle_t, streamId: cudaStream_t) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverSpSetStream {
        Some(____func) => unsafe { ____func(handle, streamId) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverSpSetStream"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverSpGetStream(handle: cusolverSpHandle_t, streamId: *mut cudaStream_t) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverSpGetStream {
        Some(____func) => unsafe { ____func(handle, streamId) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverSpGetStream"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverSpXcsrissymHost(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnzA: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrEndPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    issym: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverSpXcsrissymHost {
        Some(____func) => unsafe { ____func(handle, m, nnzA, descrA, csrRowPtrA, csrEndPtrA, csrColIndA, issym) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverSpXcsrissymHost"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverSpScsrlsvluHost(
    handle: cusolverSpHandle_t,
    n: ::std::os::raw::c_int,
    nnzA: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: *const f32,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    b: *const f32,
    tol: f32,
    reorder: ::std::os::raw::c_int,
    x: *mut f32,
    singularity: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverSpScsrlsvluHost {
        Some(____func) => unsafe { ____func(handle, n, nnzA, descrA, csrValA, csrRowPtrA, csrColIndA, b, tol, reorder, x, singularity) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverSpScsrlsvluHost"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverSpDcsrlsvluHost(
    handle: cusolverSpHandle_t,
    n: ::std::os::raw::c_int,
    nnzA: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: *const f64,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    b: *const f64,
    tol: f64,
    reorder: ::std::os::raw::c_int,
    x: *mut f64,
    singularity: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverSpDcsrlsvluHost {
        Some(____func) => unsafe { ____func(handle, n, nnzA, descrA, csrValA, csrRowPtrA, csrColIndA, b, tol, reorder, x, singularity) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverSpDcsrlsvluHost"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverSpCcsrlsvluHost(
    handle: cusolverSpHandle_t,
    n: ::std::os::raw::c_int,
    nnzA: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: *const cuComplex,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    b: *const cuComplex,
    tol: f32,
    reorder: ::std::os::raw::c_int,
    x: *mut cuComplex,
    singularity: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverSpCcsrlsvluHost {
        Some(____func) => unsafe { ____func(handle, n, nnzA, descrA, csrValA, csrRowPtrA, csrColIndA, b, tol, reorder, x, singularity) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverSpCcsrlsvluHost"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverSpZcsrlsvluHost(
    handle: cusolverSpHandle_t,
    n: ::std::os::raw::c_int,
    nnzA: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: *const cuDoubleComplex,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    b: *const cuDoubleComplex,
    tol: f64,
    reorder: ::std::os::raw::c_int,
    x: *mut cuDoubleComplex,
    singularity: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverSpZcsrlsvluHost {
        Some(____func) => unsafe { ____func(handle, n, nnzA, descrA, csrValA, csrRowPtrA, csrColIndA, b, tol, reorder, x, singularity) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverSpZcsrlsvluHost"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverSpScsrlsvqr(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrVal: *const f32,
    csrRowPtr: *const ::std::os::raw::c_int,
    csrColInd: *const ::std::os::raw::c_int,
    b: *const f32,
    tol: f32,
    reorder: ::std::os::raw::c_int,
    x: *mut f32,
    singularity: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverSpScsrlsvqr {
        Some(____func) => unsafe { ____func(handle, m, nnz, descrA, csrVal, csrRowPtr, csrColInd, b, tol, reorder, x, singularity) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverSpScsrlsvqr"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverSpDcsrlsvqr(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrVal: *const f64,
    csrRowPtr: *const ::std::os::raw::c_int,
    csrColInd: *const ::std::os::raw::c_int,
    b: *const f64,
    tol: f64,
    reorder: ::std::os::raw::c_int,
    x: *mut f64,
    singularity: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverSpDcsrlsvqr {
        Some(____func) => unsafe { ____func(handle, m, nnz, descrA, csrVal, csrRowPtr, csrColInd, b, tol, reorder, x, singularity) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverSpDcsrlsvqr"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverSpCcsrlsvqr(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrVal: *const cuComplex,
    csrRowPtr: *const ::std::os::raw::c_int,
    csrColInd: *const ::std::os::raw::c_int,
    b: *const cuComplex,
    tol: f32,
    reorder: ::std::os::raw::c_int,
    x: *mut cuComplex,
    singularity: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverSpCcsrlsvqr {
        Some(____func) => unsafe { ____func(handle, m, nnz, descrA, csrVal, csrRowPtr, csrColInd, b, tol, reorder, x, singularity) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverSpCcsrlsvqr"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverSpZcsrlsvqr(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrVal: *const cuDoubleComplex,
    csrRowPtr: *const ::std::os::raw::c_int,
    csrColInd: *const ::std::os::raw::c_int,
    b: *const cuDoubleComplex,
    tol: f64,
    reorder: ::std::os::raw::c_int,
    x: *mut cuDoubleComplex,
    singularity: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverSpZcsrlsvqr {
        Some(____func) => unsafe { ____func(handle, m, nnz, descrA, csrVal, csrRowPtr, csrColInd, b, tol, reorder, x, singularity) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverSpZcsrlsvqr"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverSpScsrlsvqrHost(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: *const f32,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    b: *const f32,
    tol: f32,
    reorder: ::std::os::raw::c_int,
    x: *mut f32,
    singularity: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverSpScsrlsvqrHost {
        Some(____func) => unsafe { ____func(handle, m, nnz, descrA, csrValA, csrRowPtrA, csrColIndA, b, tol, reorder, x, singularity) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverSpScsrlsvqrHost"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverSpDcsrlsvqrHost(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: *const f64,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    b: *const f64,
    tol: f64,
    reorder: ::std::os::raw::c_int,
    x: *mut f64,
    singularity: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverSpDcsrlsvqrHost {
        Some(____func) => unsafe { ____func(handle, m, nnz, descrA, csrValA, csrRowPtrA, csrColIndA, b, tol, reorder, x, singularity) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverSpDcsrlsvqrHost"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverSpCcsrlsvqrHost(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: *const cuComplex,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    b: *const cuComplex,
    tol: f32,
    reorder: ::std::os::raw::c_int,
    x: *mut cuComplex,
    singularity: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverSpCcsrlsvqrHost {
        Some(____func) => unsafe { ____func(handle, m, nnz, descrA, csrValA, csrRowPtrA, csrColIndA, b, tol, reorder, x, singularity) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverSpCcsrlsvqrHost"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverSpZcsrlsvqrHost(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: *const cuDoubleComplex,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    b: *const cuDoubleComplex,
    tol: f64,
    reorder: ::std::os::raw::c_int,
    x: *mut cuDoubleComplex,
    singularity: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverSpZcsrlsvqrHost {
        Some(____func) => unsafe { ____func(handle, m, nnz, descrA, csrValA, csrRowPtrA, csrColIndA, b, tol, reorder, x, singularity) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverSpZcsrlsvqrHost"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverSpScsrlsvcholHost(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrVal: *const f32,
    csrRowPtr: *const ::std::os::raw::c_int,
    csrColInd: *const ::std::os::raw::c_int,
    b: *const f32,
    tol: f32,
    reorder: ::std::os::raw::c_int,
    x: *mut f32,
    singularity: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverSpScsrlsvcholHost {
        Some(____func) => unsafe { ____func(handle, m, nnz, descrA, csrVal, csrRowPtr, csrColInd, b, tol, reorder, x, singularity) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverSpScsrlsvcholHost"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverSpDcsrlsvcholHost(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrVal: *const f64,
    csrRowPtr: *const ::std::os::raw::c_int,
    csrColInd: *const ::std::os::raw::c_int,
    b: *const f64,
    tol: f64,
    reorder: ::std::os::raw::c_int,
    x: *mut f64,
    singularity: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverSpDcsrlsvcholHost {
        Some(____func) => unsafe { ____func(handle, m, nnz, descrA, csrVal, csrRowPtr, csrColInd, b, tol, reorder, x, singularity) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverSpDcsrlsvcholHost"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverSpCcsrlsvcholHost(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrVal: *const cuComplex,
    csrRowPtr: *const ::std::os::raw::c_int,
    csrColInd: *const ::std::os::raw::c_int,
    b: *const cuComplex,
    tol: f32,
    reorder: ::std::os::raw::c_int,
    x: *mut cuComplex,
    singularity: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverSpCcsrlsvcholHost {
        Some(____func) => unsafe { ____func(handle, m, nnz, descrA, csrVal, csrRowPtr, csrColInd, b, tol, reorder, x, singularity) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverSpCcsrlsvcholHost"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverSpZcsrlsvcholHost(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrVal: *const cuDoubleComplex,
    csrRowPtr: *const ::std::os::raw::c_int,
    csrColInd: *const ::std::os::raw::c_int,
    b: *const cuDoubleComplex,
    tol: f64,
    reorder: ::std::os::raw::c_int,
    x: *mut cuDoubleComplex,
    singularity: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverSpZcsrlsvcholHost {
        Some(____func) => unsafe { ____func(handle, m, nnz, descrA, csrVal, csrRowPtr, csrColInd, b, tol, reorder, x, singularity) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverSpZcsrlsvcholHost"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverSpScsrlsvchol(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrVal: *const f32,
    csrRowPtr: *const ::std::os::raw::c_int,
    csrColInd: *const ::std::os::raw::c_int,
    b: *const f32,
    tol: f32,
    reorder: ::std::os::raw::c_int,
    x: *mut f32,
    singularity: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverSpScsrlsvchol {
        Some(____func) => unsafe { ____func(handle, m, nnz, descrA, csrVal, csrRowPtr, csrColInd, b, tol, reorder, x, singularity) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverSpScsrlsvchol"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverSpDcsrlsvchol(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrVal: *const f64,
    csrRowPtr: *const ::std::os::raw::c_int,
    csrColInd: *const ::std::os::raw::c_int,
    b: *const f64,
    tol: f64,
    reorder: ::std::os::raw::c_int,
    x: *mut f64,
    singularity: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverSpDcsrlsvchol {
        Some(____func) => unsafe { ____func(handle, m, nnz, descrA, csrVal, csrRowPtr, csrColInd, b, tol, reorder, x, singularity) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverSpDcsrlsvchol"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverSpCcsrlsvchol(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrVal: *const cuComplex,
    csrRowPtr: *const ::std::os::raw::c_int,
    csrColInd: *const ::std::os::raw::c_int,
    b: *const cuComplex,
    tol: f32,
    reorder: ::std::os::raw::c_int,
    x: *mut cuComplex,
    singularity: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverSpCcsrlsvchol {
        Some(____func) => unsafe { ____func(handle, m, nnz, descrA, csrVal, csrRowPtr, csrColInd, b, tol, reorder, x, singularity) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverSpCcsrlsvchol"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverSpZcsrlsvchol(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrVal: *const cuDoubleComplex,
    csrRowPtr: *const ::std::os::raw::c_int,
    csrColInd: *const ::std::os::raw::c_int,
    b: *const cuDoubleComplex,
    tol: f64,
    reorder: ::std::os::raw::c_int,
    x: *mut cuDoubleComplex,
    singularity: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverSpZcsrlsvchol {
        Some(____func) => unsafe { ____func(handle, m, nnz, descrA, csrVal, csrRowPtr, csrColInd, b, tol, reorder, x, singularity) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverSpZcsrlsvchol"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverSpScsrlsqvqrHost(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: *const f32,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    b: *const f32,
    tol: f32,
    rankA: *mut ::std::os::raw::c_int,
    x: *mut f32,
    p: *mut ::std::os::raw::c_int,
    min_norm: *mut f32,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverSpScsrlsqvqrHost {
        Some(____func) => unsafe { ____func(handle, m, n, nnz, descrA, csrValA, csrRowPtrA, csrColIndA, b, tol, rankA, x, p, min_norm) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverSpScsrlsqvqrHost"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverSpDcsrlsqvqrHost(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: *const f64,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    b: *const f64,
    tol: f64,
    rankA: *mut ::std::os::raw::c_int,
    x: *mut f64,
    p: *mut ::std::os::raw::c_int,
    min_norm: *mut f64,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverSpDcsrlsqvqrHost {
        Some(____func) => unsafe { ____func(handle, m, n, nnz, descrA, csrValA, csrRowPtrA, csrColIndA, b, tol, rankA, x, p, min_norm) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverSpDcsrlsqvqrHost"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverSpCcsrlsqvqrHost(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: *const cuComplex,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    b: *const cuComplex,
    tol: f32,
    rankA: *mut ::std::os::raw::c_int,
    x: *mut cuComplex,
    p: *mut ::std::os::raw::c_int,
    min_norm: *mut f32,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverSpCcsrlsqvqrHost {
        Some(____func) => unsafe { ____func(handle, m, n, nnz, descrA, csrValA, csrRowPtrA, csrColIndA, b, tol, rankA, x, p, min_norm) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverSpCcsrlsqvqrHost"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverSpZcsrlsqvqrHost(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: *const cuDoubleComplex,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    b: *const cuDoubleComplex,
    tol: f64,
    rankA: *mut ::std::os::raw::c_int,
    x: *mut cuDoubleComplex,
    p: *mut ::std::os::raw::c_int,
    min_norm: *mut f64,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverSpZcsrlsqvqrHost {
        Some(____func) => unsafe { ____func(handle, m, n, nnz, descrA, csrValA, csrRowPtrA, csrColIndA, b, tol, rankA, x, p, min_norm) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverSpZcsrlsqvqrHost"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverSpScsreigvsiHost(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: *const f32,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    mu0: f32,
    x0: *const f32,
    maxite: ::std::os::raw::c_int,
    tol: f32,
    mu: *mut f32,
    x: *mut f32,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverSpScsreigvsiHost {
        Some(____func) => unsafe { ____func(handle, m, nnz, descrA, csrValA, csrRowPtrA, csrColIndA, mu0, x0, maxite, tol, mu, x) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverSpScsreigvsiHost"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverSpDcsreigvsiHost(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: *const f64,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    mu0: f64,
    x0: *const f64,
    maxite: ::std::os::raw::c_int,
    tol: f64,
    mu: *mut f64,
    x: *mut f64,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverSpDcsreigvsiHost {
        Some(____func) => unsafe { ____func(handle, m, nnz, descrA, csrValA, csrRowPtrA, csrColIndA, mu0, x0, maxite, tol, mu, x) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverSpDcsreigvsiHost"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverSpCcsreigvsiHost(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: *const cuComplex,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    mu0: cuComplex,
    x0: *const cuComplex,
    maxite: ::std::os::raw::c_int,
    tol: f32,
    mu: *mut cuComplex,
    x: *mut cuComplex,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverSpCcsreigvsiHost {
        Some(____func) => unsafe { ____func(handle, m, nnz, descrA, csrValA, csrRowPtrA, csrColIndA, mu0, x0, maxite, tol, mu, x) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverSpCcsreigvsiHost"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverSpZcsreigvsiHost(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: *const cuDoubleComplex,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    mu0: cuDoubleComplex,
    x0: *const cuDoubleComplex,
    maxite: ::std::os::raw::c_int,
    tol: f64,
    mu: *mut cuDoubleComplex,
    x: *mut cuDoubleComplex,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverSpZcsreigvsiHost {
        Some(____func) => unsafe { ____func(handle, m, nnz, descrA, csrValA, csrRowPtrA, csrColIndA, mu0, x0, maxite, tol, mu, x) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverSpZcsreigvsiHost"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverSpScsreigvsi(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: *const f32,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    mu0: f32,
    x0: *const f32,
    maxite: ::std::os::raw::c_int,
    eps: f32,
    mu: *mut f32,
    x: *mut f32,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverSpScsreigvsi {
        Some(____func) => unsafe { ____func(handle, m, nnz, descrA, csrValA, csrRowPtrA, csrColIndA, mu0, x0, maxite, eps, mu, x) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverSpScsreigvsi"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverSpDcsreigvsi(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: *const f64,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    mu0: f64,
    x0: *const f64,
    maxite: ::std::os::raw::c_int,
    eps: f64,
    mu: *mut f64,
    x: *mut f64,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverSpDcsreigvsi {
        Some(____func) => unsafe { ____func(handle, m, nnz, descrA, csrValA, csrRowPtrA, csrColIndA, mu0, x0, maxite, eps, mu, x) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverSpDcsreigvsi"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverSpCcsreigvsi(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: *const cuComplex,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    mu0: cuComplex,
    x0: *const cuComplex,
    maxite: ::std::os::raw::c_int,
    eps: f32,
    mu: *mut cuComplex,
    x: *mut cuComplex,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverSpCcsreigvsi {
        Some(____func) => unsafe { ____func(handle, m, nnz, descrA, csrValA, csrRowPtrA, csrColIndA, mu0, x0, maxite, eps, mu, x) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverSpCcsreigvsi"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverSpZcsreigvsi(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: *const cuDoubleComplex,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    mu0: cuDoubleComplex,
    x0: *const cuDoubleComplex,
    maxite: ::std::os::raw::c_int,
    eps: f64,
    mu: *mut cuDoubleComplex,
    x: *mut cuDoubleComplex,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverSpZcsreigvsi {
        Some(____func) => unsafe { ____func(handle, m, nnz, descrA, csrValA, csrRowPtrA, csrColIndA, mu0, x0, maxite, eps, mu, x) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverSpZcsreigvsi"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverSpScsreigsHost(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: *const f32,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    left_bottom_corner: cuComplex,
    right_upper_corner: cuComplex,
    num_eigs: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverSpScsreigsHost {
        Some(____func) => unsafe { ____func(handle, m, nnz, descrA, csrValA, csrRowPtrA, csrColIndA, left_bottom_corner, right_upper_corner, num_eigs) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverSpScsreigsHost"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverSpDcsreigsHost(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: *const f64,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    left_bottom_corner: cuDoubleComplex,
    right_upper_corner: cuDoubleComplex,
    num_eigs: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverSpDcsreigsHost {
        Some(____func) => unsafe { ____func(handle, m, nnz, descrA, csrValA, csrRowPtrA, csrColIndA, left_bottom_corner, right_upper_corner, num_eigs) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverSpDcsreigsHost"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverSpCcsreigsHost(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: *const cuComplex,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    left_bottom_corner: cuComplex,
    right_upper_corner: cuComplex,
    num_eigs: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverSpCcsreigsHost {
        Some(____func) => unsafe { ____func(handle, m, nnz, descrA, csrValA, csrRowPtrA, csrColIndA, left_bottom_corner, right_upper_corner, num_eigs) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverSpCcsreigsHost"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverSpZcsreigsHost(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: *const cuDoubleComplex,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    left_bottom_corner: cuDoubleComplex,
    right_upper_corner: cuDoubleComplex,
    num_eigs: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverSpZcsreigsHost {
        Some(____func) => unsafe { ____func(handle, m, nnz, descrA, csrValA, csrRowPtrA, csrColIndA, left_bottom_corner, right_upper_corner, num_eigs) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverSpZcsreigsHost"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverSpXcsrsymrcmHost(handle: cusolverSpHandle_t, n: ::std::os::raw::c_int, nnzA: ::std::os::raw::c_int, descrA: cusparseMatDescr_t, csrRowPtrA: *const ::std::os::raw::c_int, csrColIndA: *const ::std::os::raw::c_int, p: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverSpXcsrsymrcmHost {
        Some(____func) => unsafe { ____func(handle, n, nnzA, descrA, csrRowPtrA, csrColIndA, p) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverSpXcsrsymrcmHost"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverSpXcsrsymmdqHost(handle: cusolverSpHandle_t, n: ::std::os::raw::c_int, nnzA: ::std::os::raw::c_int, descrA: cusparseMatDescr_t, csrRowPtrA: *const ::std::os::raw::c_int, csrColIndA: *const ::std::os::raw::c_int, p: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverSpXcsrsymmdqHost {
        Some(____func) => unsafe { ____func(handle, n, nnzA, descrA, csrRowPtrA, csrColIndA, p) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverSpXcsrsymmdqHost"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverSpXcsrsymamdHost(handle: cusolverSpHandle_t, n: ::std::os::raw::c_int, nnzA: ::std::os::raw::c_int, descrA: cusparseMatDescr_t, csrRowPtrA: *const ::std::os::raw::c_int, csrColIndA: *const ::std::os::raw::c_int, p: *mut ::std::os::raw::c_int) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverSpXcsrsymamdHost {
        Some(____func) => unsafe { ____func(handle, n, nnzA, descrA, csrRowPtrA, csrColIndA, p) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverSpXcsrsymamdHost"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverSpXcsrmetisndHost(
    handle: cusolverSpHandle_t,
    n: ::std::os::raw::c_int,
    nnzA: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    options: *const i64,
    p: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverSpXcsrmetisndHost {
        Some(____func) => unsafe { ____func(handle, n, nnzA, descrA, csrRowPtrA, csrColIndA, options, p) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverSpXcsrmetisndHost"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverSpScsrzfdHost(
    handle: cusolverSpHandle_t,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: *const f32,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    P: *mut ::std::os::raw::c_int,
    numnz: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverSpScsrzfdHost {
        Some(____func) => unsafe { ____func(handle, n, nnz, descrA, csrValA, csrRowPtrA, csrColIndA, P, numnz) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverSpScsrzfdHost"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverSpDcsrzfdHost(
    handle: cusolverSpHandle_t,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: *const f64,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    P: *mut ::std::os::raw::c_int,
    numnz: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverSpDcsrzfdHost {
        Some(____func) => unsafe { ____func(handle, n, nnz, descrA, csrValA, csrRowPtrA, csrColIndA, P, numnz) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverSpDcsrzfdHost"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverSpCcsrzfdHost(
    handle: cusolverSpHandle_t,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: *const cuComplex,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    P: *mut ::std::os::raw::c_int,
    numnz: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverSpCcsrzfdHost {
        Some(____func) => unsafe { ____func(handle, n, nnz, descrA, csrValA, csrRowPtrA, csrColIndA, P, numnz) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverSpCcsrzfdHost"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverSpZcsrzfdHost(
    handle: cusolverSpHandle_t,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: *const cuDoubleComplex,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    P: *mut ::std::os::raw::c_int,
    numnz: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverSpZcsrzfdHost {
        Some(____func) => unsafe { ____func(handle, n, nnz, descrA, csrValA, csrRowPtrA, csrColIndA, P, numnz) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverSpZcsrzfdHost"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverSpXcsrperm_bufferSizeHost(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnzA: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    p: *const ::std::os::raw::c_int,
    q: *const ::std::os::raw::c_int,
    bufferSizeInBytes: *mut usize,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverSpXcsrperm_bufferSizeHost {
        Some(____func) => unsafe { ____func(handle, m, n, nnzA, descrA, csrRowPtrA, csrColIndA, p, q, bufferSizeInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverSpXcsrperm_bufferSizeHost"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverSpXcsrpermHost(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnzA: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrRowPtrA: *mut ::std::os::raw::c_int,
    csrColIndA: *mut ::std::os::raw::c_int,
    p: *const ::std::os::raw::c_int,
    q: *const ::std::os::raw::c_int,
    map: *mut ::std::os::raw::c_int,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverSpXcsrpermHost {
        Some(____func) => unsafe { ____func(handle, m, n, nnzA, descrA, csrRowPtrA, csrColIndA, p, q, map, pBuffer) },
        None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", "cusolverSpXcsrpermHost"),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverSpCreateCsrqrInfo(info: *mut csrqrInfo_t) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverSpCreateCsrqrInfo {
        Some(____func) => unsafe { ____func(info) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverSpCreateCsrqrInfo"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverSpDestroyCsrqrInfo(info: csrqrInfo_t) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverSpDestroyCsrqrInfo {
        Some(____func) => unsafe { ____func(info) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverSpDestroyCsrqrInfo"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverSpXcsrqrAnalysisBatched(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnzA: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    info: csrqrInfo_t,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverSpXcsrqrAnalysisBatched {
        Some(____func) => unsafe { ____func(handle, m, n, nnzA, descrA, csrRowPtrA, csrColIndA, info) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverSpXcsrqrAnalysisBatched"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverSpScsrqrBufferInfoBatched(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrVal: *const f32,
    csrRowPtr: *const ::std::os::raw::c_int,
    csrColInd: *const ::std::os::raw::c_int,
    batchSize: ::std::os::raw::c_int,
    info: csrqrInfo_t,
    internalDataInBytes: *mut usize,
    workspaceInBytes: *mut usize,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverSpScsrqrBufferInfoBatched {
        Some(____func) => unsafe { ____func(handle, m, n, nnz, descrA, csrVal, csrRowPtr, csrColInd, batchSize, info, internalDataInBytes, workspaceInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverSpScsrqrBufferInfoBatched"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverSpDcsrqrBufferInfoBatched(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrVal: *const f64,
    csrRowPtr: *const ::std::os::raw::c_int,
    csrColInd: *const ::std::os::raw::c_int,
    batchSize: ::std::os::raw::c_int,
    info: csrqrInfo_t,
    internalDataInBytes: *mut usize,
    workspaceInBytes: *mut usize,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverSpDcsrqrBufferInfoBatched {
        Some(____func) => unsafe { ____func(handle, m, n, nnz, descrA, csrVal, csrRowPtr, csrColInd, batchSize, info, internalDataInBytes, workspaceInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverSpDcsrqrBufferInfoBatched"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverSpCcsrqrBufferInfoBatched(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrVal: *const cuComplex,
    csrRowPtr: *const ::std::os::raw::c_int,
    csrColInd: *const ::std::os::raw::c_int,
    batchSize: ::std::os::raw::c_int,
    info: csrqrInfo_t,
    internalDataInBytes: *mut usize,
    workspaceInBytes: *mut usize,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverSpCcsrqrBufferInfoBatched {
        Some(____func) => unsafe { ____func(handle, m, n, nnz, descrA, csrVal, csrRowPtr, csrColInd, batchSize, info, internalDataInBytes, workspaceInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverSpCcsrqrBufferInfoBatched"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverSpZcsrqrBufferInfoBatched(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrVal: *const cuDoubleComplex,
    csrRowPtr: *const ::std::os::raw::c_int,
    csrColInd: *const ::std::os::raw::c_int,
    batchSize: ::std::os::raw::c_int,
    info: csrqrInfo_t,
    internalDataInBytes: *mut usize,
    workspaceInBytes: *mut usize,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverSpZcsrqrBufferInfoBatched {
        Some(____func) => unsafe { ____func(handle, m, n, nnz, descrA, csrVal, csrRowPtr, csrColInd, batchSize, info, internalDataInBytes, workspaceInBytes) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverSpZcsrqrBufferInfoBatched"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverSpScsrqrsvBatched(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: *const f32,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    b: *const f32,
    x: *mut f32,
    batchSize: ::std::os::raw::c_int,
    info: csrqrInfo_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverSpScsrqrsvBatched {
        Some(____func) => unsafe { ____func(handle, m, n, nnz, descrA, csrValA, csrRowPtrA, csrColIndA, b, x, batchSize, info, pBuffer) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverSpScsrqrsvBatched"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverSpDcsrqrsvBatched(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: *const f64,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    b: *const f64,
    x: *mut f64,
    batchSize: ::std::os::raw::c_int,
    info: csrqrInfo_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverSpDcsrqrsvBatched {
        Some(____func) => unsafe { ____func(handle, m, n, nnz, descrA, csrValA, csrRowPtrA, csrColIndA, b, x, batchSize, info, pBuffer) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverSpDcsrqrsvBatched"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverSpCcsrqrsvBatched(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: *const cuComplex,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    b: *const cuComplex,
    x: *mut cuComplex,
    batchSize: ::std::os::raw::c_int,
    info: csrqrInfo_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverSpCcsrqrsvBatched {
        Some(____func) => unsafe { ____func(handle, m, n, nnz, descrA, csrValA, csrRowPtrA, csrColIndA, b, x, batchSize, info, pBuffer) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverSpCcsrqrsvBatched"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn cusolverSpZcsrqrsvBatched(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: *const cuDoubleComplex,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    b: *const cuDoubleComplex,
    x: *mut cuDoubleComplex,
    batchSize: ::std::os::raw::c_int,
    info: csrqrInfo_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusolverStatus_t {
    match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").cusolverSpZcsrqrsvBatched {
        Some(____func) => unsafe { ____func(handle, m, n, nnz, descrA, csrValA, csrRowPtrA, csrColIndA, b, x, batchSize, info, pBuffer) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "cusolverSpZcsrqrsvBatched"
        ),
    }
}
#[cfg(feature = "runtime-link")]
pub unsafe fn load_dynamic_bindings(lib: *mut std::ffi::c_void, get_proc_addr: unsafe fn(*mut std::ffi::c_void, *const u8) -> *mut std::ffi::c_void) {
    let bindings = unsafe {
        Box::new(DynamicBindings {
            cusolverGetProperty: {
                let p = get_proc_addr(lib, b"cusolverGetProperty\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverGetVersion: {
                let p = get_proc_addr(lib, b"cusolverGetVersion\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnCreate: {
                let p = get_proc_addr(lib, b"cusolverDnCreate\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDestroy: {
                let p = get_proc_addr(lib, b"cusolverDnDestroy\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSetStream: {
                let p = get_proc_addr(lib, b"cusolverDnSetStream\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnGetStream: {
                let p = get_proc_addr(lib, b"cusolverDnGetStream\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSetDeterministicMode: {
                let p = get_proc_addr(lib, b"cusolverDnSetDeterministicMode\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnGetDeterministicMode: {
                let p = get_proc_addr(lib, b"cusolverDnGetDeterministicMode\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSetMathMode: {
                let p = get_proc_addr(lib, b"cusolverDnSetMathMode\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnGetMathMode: {
                let p = get_proc_addr(lib, b"cusolverDnGetMathMode\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSetEmulationStrategy: {
                let p = get_proc_addr(lib, b"cusolverDnSetEmulationStrategy\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnGetEmulationStrategy: {
                let p = get_proc_addr(lib, b"cusolverDnGetEmulationStrategy\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSetFixedPointEmulationMantissaControl: {
                let p = get_proc_addr(lib, b"cusolverDnSetFixedPointEmulationMantissaControl\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnGetFixedPointEmulationMantissaControl: {
                let p = get_proc_addr(lib, b"cusolverDnGetFixedPointEmulationMantissaControl\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSetFixedPointEmulationMaxMantissaBitCount: {
                let p = get_proc_addr(lib, b"cusolverDnSetFixedPointEmulationMaxMantissaBitCount\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnGetFixedPointEmulationMaxMantissaBitCount: {
                let p = get_proc_addr(lib, b"cusolverDnGetFixedPointEmulationMaxMantissaBitCount\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSetFixedPointEmulationMantissaBitOffset: {
                let p = get_proc_addr(lib, b"cusolverDnSetFixedPointEmulationMantissaBitOffset\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnGetFixedPointEmulationMantissaBitOffset: {
                let p = get_proc_addr(lib, b"cusolverDnGetFixedPointEmulationMantissaBitOffset\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSetEmulationSpecialValuesSupport: {
                let p = get_proc_addr(lib, b"cusolverDnSetEmulationSpecialValuesSupport\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnGetEmulationSpecialValuesSupport: {
                let p = get_proc_addr(lib, b"cusolverDnGetEmulationSpecialValuesSupport\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnIRSParamsCreate: {
                let p = get_proc_addr(lib, b"cusolverDnIRSParamsCreate\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnIRSParamsDestroy: {
                let p = get_proc_addr(lib, b"cusolverDnIRSParamsDestroy\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnIRSParamsSetRefinementSolver: {
                let p = get_proc_addr(lib, b"cusolverDnIRSParamsSetRefinementSolver\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnIRSParamsSetSolverMainPrecision: {
                let p = get_proc_addr(lib, b"cusolverDnIRSParamsSetSolverMainPrecision\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnIRSParamsSetSolverLowestPrecision: {
                let p = get_proc_addr(lib, b"cusolverDnIRSParamsSetSolverLowestPrecision\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnIRSParamsSetSolverPrecisions: {
                let p = get_proc_addr(lib, b"cusolverDnIRSParamsSetSolverPrecisions\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnIRSParamsSetTol: {
                let p = get_proc_addr(lib, b"cusolverDnIRSParamsSetTol\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnIRSParamsSetTolInner: {
                let p = get_proc_addr(lib, b"cusolverDnIRSParamsSetTolInner\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnIRSParamsSetMaxIters: {
                let p = get_proc_addr(lib, b"cusolverDnIRSParamsSetMaxIters\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnIRSParamsSetMaxItersInner: {
                let p = get_proc_addr(lib, b"cusolverDnIRSParamsSetMaxItersInner\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnIRSParamsGetMaxIters: {
                let p = get_proc_addr(lib, b"cusolverDnIRSParamsGetMaxIters\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnIRSParamsEnableFallback: {
                let p = get_proc_addr(lib, b"cusolverDnIRSParamsEnableFallback\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnIRSParamsDisableFallback: {
                let p = get_proc_addr(lib, b"cusolverDnIRSParamsDisableFallback\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnIRSInfosDestroy: {
                let p = get_proc_addr(lib, b"cusolverDnIRSInfosDestroy\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnIRSInfosCreate: {
                let p = get_proc_addr(lib, b"cusolverDnIRSInfosCreate\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnIRSInfosGetNiters: {
                let p = get_proc_addr(lib, b"cusolverDnIRSInfosGetNiters\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnIRSInfosGetOuterNiters: {
                let p = get_proc_addr(lib, b"cusolverDnIRSInfosGetOuterNiters\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnIRSInfosRequestResidual: {
                let p = get_proc_addr(lib, b"cusolverDnIRSInfosRequestResidual\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnIRSInfosGetResidualHistory: {
                let p = get_proc_addr(lib, b"cusolverDnIRSInfosGetResidualHistory\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnIRSInfosGetMaxIters: {
                let p = get_proc_addr(lib, b"cusolverDnIRSInfosGetMaxIters\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZZgesv: {
                let p = get_proc_addr(lib, b"cusolverDnZZgesv\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZCgesv: {
                let p = get_proc_addr(lib, b"cusolverDnZCgesv\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZKgesv: {
                let p = get_proc_addr(lib, b"cusolverDnZKgesv\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZEgesv: {
                let p = get_proc_addr(lib, b"cusolverDnZEgesv\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZYgesv: {
                let p = get_proc_addr(lib, b"cusolverDnZYgesv\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnCCgesv: {
                let p = get_proc_addr(lib, b"cusolverDnCCgesv\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnCEgesv: {
                let p = get_proc_addr(lib, b"cusolverDnCEgesv\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnCKgesv: {
                let p = get_proc_addr(lib, b"cusolverDnCKgesv\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnCYgesv: {
                let p = get_proc_addr(lib, b"cusolverDnCYgesv\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDDgesv: {
                let p = get_proc_addr(lib, b"cusolverDnDDgesv\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDSgesv: {
                let p = get_proc_addr(lib, b"cusolverDnDSgesv\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDHgesv: {
                let p = get_proc_addr(lib, b"cusolverDnDHgesv\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDBgesv: {
                let p = get_proc_addr(lib, b"cusolverDnDBgesv\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDXgesv: {
                let p = get_proc_addr(lib, b"cusolverDnDXgesv\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSSgesv: {
                let p = get_proc_addr(lib, b"cusolverDnSSgesv\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSHgesv: {
                let p = get_proc_addr(lib, b"cusolverDnSHgesv\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSBgesv: {
                let p = get_proc_addr(lib, b"cusolverDnSBgesv\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSXgesv: {
                let p = get_proc_addr(lib, b"cusolverDnSXgesv\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZZgesv_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnZZgesv_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZCgesv_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnZCgesv_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZKgesv_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnZKgesv_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZEgesv_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnZEgesv_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZYgesv_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnZYgesv_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnCCgesv_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnCCgesv_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnCKgesv_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnCKgesv_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnCEgesv_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnCEgesv_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnCYgesv_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnCYgesv_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDDgesv_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnDDgesv_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDSgesv_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnDSgesv_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDHgesv_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnDHgesv_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDBgesv_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnDBgesv_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDXgesv_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnDXgesv_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSSgesv_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnSSgesv_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSHgesv_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnSHgesv_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSBgesv_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnSBgesv_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSXgesv_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnSXgesv_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZZgels: {
                let p = get_proc_addr(lib, b"cusolverDnZZgels\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZCgels: {
                let p = get_proc_addr(lib, b"cusolverDnZCgels\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZKgels: {
                let p = get_proc_addr(lib, b"cusolverDnZKgels\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZEgels: {
                let p = get_proc_addr(lib, b"cusolverDnZEgels\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZYgels: {
                let p = get_proc_addr(lib, b"cusolverDnZYgels\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnCCgels: {
                let p = get_proc_addr(lib, b"cusolverDnCCgels\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnCKgels: {
                let p = get_proc_addr(lib, b"cusolverDnCKgels\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnCEgels: {
                let p = get_proc_addr(lib, b"cusolverDnCEgels\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnCYgels: {
                let p = get_proc_addr(lib, b"cusolverDnCYgels\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDDgels: {
                let p = get_proc_addr(lib, b"cusolverDnDDgels\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDSgels: {
                let p = get_proc_addr(lib, b"cusolverDnDSgels\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDHgels: {
                let p = get_proc_addr(lib, b"cusolverDnDHgels\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDBgels: {
                let p = get_proc_addr(lib, b"cusolverDnDBgels\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDXgels: {
                let p = get_proc_addr(lib, b"cusolverDnDXgels\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSSgels: {
                let p = get_proc_addr(lib, b"cusolverDnSSgels\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSHgels: {
                let p = get_proc_addr(lib, b"cusolverDnSHgels\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSBgels: {
                let p = get_proc_addr(lib, b"cusolverDnSBgels\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSXgels: {
                let p = get_proc_addr(lib, b"cusolverDnSXgels\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZZgels_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnZZgels_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZCgels_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnZCgels_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZKgels_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnZKgels_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZEgels_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnZEgels_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZYgels_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnZYgels_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnCCgels_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnCCgels_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnCKgels_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnCKgels_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnCEgels_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnCEgels_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnCYgels_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnCYgels_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDDgels_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnDDgels_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDSgels_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnDSgels_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDHgels_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnDHgels_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDBgels_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnDBgels_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDXgels_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnDXgels_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSSgels_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnSSgels_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSHgels_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnSHgels_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSBgels_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnSBgels_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSXgels_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnSXgels_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnIRSXgesv: {
                let p = get_proc_addr(lib, b"cusolverDnIRSXgesv\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnIRSXgesv_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnIRSXgesv_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnIRSXgels: {
                let p = get_proc_addr(lib, b"cusolverDnIRSXgels\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnIRSXgels_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnIRSXgels_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSpotrf_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnSpotrf_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDpotrf_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnDpotrf_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnCpotrf_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnCpotrf_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZpotrf_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnZpotrf_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSpotrf: {
                let p = get_proc_addr(lib, b"cusolverDnSpotrf\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDpotrf: {
                let p = get_proc_addr(lib, b"cusolverDnDpotrf\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnCpotrf: {
                let p = get_proc_addr(lib, b"cusolverDnCpotrf\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZpotrf: {
                let p = get_proc_addr(lib, b"cusolverDnZpotrf\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSpotrs: {
                let p = get_proc_addr(lib, b"cusolverDnSpotrs\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDpotrs: {
                let p = get_proc_addr(lib, b"cusolverDnDpotrs\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnCpotrs: {
                let p = get_proc_addr(lib, b"cusolverDnCpotrs\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZpotrs: {
                let p = get_proc_addr(lib, b"cusolverDnZpotrs\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSpotrfBatched: {
                let p = get_proc_addr(lib, b"cusolverDnSpotrfBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDpotrfBatched: {
                let p = get_proc_addr(lib, b"cusolverDnDpotrfBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnCpotrfBatched: {
                let p = get_proc_addr(lib, b"cusolverDnCpotrfBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZpotrfBatched: {
                let p = get_proc_addr(lib, b"cusolverDnZpotrfBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSpotrsBatched: {
                let p = get_proc_addr(lib, b"cusolverDnSpotrsBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDpotrsBatched: {
                let p = get_proc_addr(lib, b"cusolverDnDpotrsBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnCpotrsBatched: {
                let p = get_proc_addr(lib, b"cusolverDnCpotrsBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZpotrsBatched: {
                let p = get_proc_addr(lib, b"cusolverDnZpotrsBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSpotri_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnSpotri_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDpotri_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnDpotri_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnCpotri_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnCpotri_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZpotri_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnZpotri_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSpotri: {
                let p = get_proc_addr(lib, b"cusolverDnSpotri\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDpotri: {
                let p = get_proc_addr(lib, b"cusolverDnDpotri\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnCpotri: {
                let p = get_proc_addr(lib, b"cusolverDnCpotri\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZpotri: {
                let p = get_proc_addr(lib, b"cusolverDnZpotri\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnXtrtri_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnXtrtri_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnXtrtri: {
                let p = get_proc_addr(lib, b"cusolverDnXtrtri\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSlauum_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnSlauum_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDlauum_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnDlauum_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnClauum_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnClauum_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZlauum_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnZlauum_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSlauum: {
                let p = get_proc_addr(lib, b"cusolverDnSlauum\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDlauum: {
                let p = get_proc_addr(lib, b"cusolverDnDlauum\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnClauum: {
                let p = get_proc_addr(lib, b"cusolverDnClauum\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZlauum: {
                let p = get_proc_addr(lib, b"cusolverDnZlauum\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSgetrf_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnSgetrf_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDgetrf_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnDgetrf_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnCgetrf_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnCgetrf_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZgetrf_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnZgetrf_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSgetrf: {
                let p = get_proc_addr(lib, b"cusolverDnSgetrf\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDgetrf: {
                let p = get_proc_addr(lib, b"cusolverDnDgetrf\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnCgetrf: {
                let p = get_proc_addr(lib, b"cusolverDnCgetrf\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZgetrf: {
                let p = get_proc_addr(lib, b"cusolverDnZgetrf\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSlaswp: {
                let p = get_proc_addr(lib, b"cusolverDnSlaswp\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDlaswp: {
                let p = get_proc_addr(lib, b"cusolverDnDlaswp\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnClaswp: {
                let p = get_proc_addr(lib, b"cusolverDnClaswp\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZlaswp: {
                let p = get_proc_addr(lib, b"cusolverDnZlaswp\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSgetrs: {
                let p = get_proc_addr(lib, b"cusolverDnSgetrs\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDgetrs: {
                let p = get_proc_addr(lib, b"cusolverDnDgetrs\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnCgetrs: {
                let p = get_proc_addr(lib, b"cusolverDnCgetrs\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZgetrs: {
                let p = get_proc_addr(lib, b"cusolverDnZgetrs\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSgeqrf_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnSgeqrf_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDgeqrf_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnDgeqrf_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnCgeqrf_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnCgeqrf_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZgeqrf_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnZgeqrf_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSgeqrf: {
                let p = get_proc_addr(lib, b"cusolverDnSgeqrf\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDgeqrf: {
                let p = get_proc_addr(lib, b"cusolverDnDgeqrf\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnCgeqrf: {
                let p = get_proc_addr(lib, b"cusolverDnCgeqrf\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZgeqrf: {
                let p = get_proc_addr(lib, b"cusolverDnZgeqrf\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSorgqr_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnSorgqr_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDorgqr_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnDorgqr_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnCungqr_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnCungqr_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZungqr_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnZungqr_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSorgqr: {
                let p = get_proc_addr(lib, b"cusolverDnSorgqr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDorgqr: {
                let p = get_proc_addr(lib, b"cusolverDnDorgqr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnCungqr: {
                let p = get_proc_addr(lib, b"cusolverDnCungqr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZungqr: {
                let p = get_proc_addr(lib, b"cusolverDnZungqr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSormqr_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnSormqr_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDormqr_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnDormqr_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnCunmqr_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnCunmqr_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZunmqr_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnZunmqr_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSormqr: {
                let p = get_proc_addr(lib, b"cusolverDnSormqr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDormqr: {
                let p = get_proc_addr(lib, b"cusolverDnDormqr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnCunmqr: {
                let p = get_proc_addr(lib, b"cusolverDnCunmqr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZunmqr: {
                let p = get_proc_addr(lib, b"cusolverDnZunmqr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSsytrf_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnSsytrf_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDsytrf_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnDsytrf_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnCsytrf_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnCsytrf_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZsytrf_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnZsytrf_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSsytrf: {
                let p = get_proc_addr(lib, b"cusolverDnSsytrf\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDsytrf: {
                let p = get_proc_addr(lib, b"cusolverDnDsytrf\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnCsytrf: {
                let p = get_proc_addr(lib, b"cusolverDnCsytrf\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZsytrf: {
                let p = get_proc_addr(lib, b"cusolverDnZsytrf\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnXsytrs_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnXsytrs_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnXsytrs: {
                let p = get_proc_addr(lib, b"cusolverDnXsytrs\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSsytri_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnSsytri_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDsytri_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnDsytri_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnCsytri_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnCsytri_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZsytri_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnZsytri_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSsytri: {
                let p = get_proc_addr(lib, b"cusolverDnSsytri\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDsytri: {
                let p = get_proc_addr(lib, b"cusolverDnDsytri\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnCsytri: {
                let p = get_proc_addr(lib, b"cusolverDnCsytri\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZsytri: {
                let p = get_proc_addr(lib, b"cusolverDnZsytri\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSgebrd_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnSgebrd_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDgebrd_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnDgebrd_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnCgebrd_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnCgebrd_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZgebrd_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnZgebrd_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSgebrd: {
                let p = get_proc_addr(lib, b"cusolverDnSgebrd\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDgebrd: {
                let p = get_proc_addr(lib, b"cusolverDnDgebrd\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnCgebrd: {
                let p = get_proc_addr(lib, b"cusolverDnCgebrd\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZgebrd: {
                let p = get_proc_addr(lib, b"cusolverDnZgebrd\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSorgbr_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnSorgbr_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDorgbr_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnDorgbr_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnCungbr_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnCungbr_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZungbr_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnZungbr_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSorgbr: {
                let p = get_proc_addr(lib, b"cusolverDnSorgbr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDorgbr: {
                let p = get_proc_addr(lib, b"cusolverDnDorgbr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnCungbr: {
                let p = get_proc_addr(lib, b"cusolverDnCungbr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZungbr: {
                let p = get_proc_addr(lib, b"cusolverDnZungbr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSsytrd_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnSsytrd_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDsytrd_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnDsytrd_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnChetrd_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnChetrd_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZhetrd_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnZhetrd_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSsytrd: {
                let p = get_proc_addr(lib, b"cusolverDnSsytrd\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDsytrd: {
                let p = get_proc_addr(lib, b"cusolverDnDsytrd\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnChetrd: {
                let p = get_proc_addr(lib, b"cusolverDnChetrd\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZhetrd: {
                let p = get_proc_addr(lib, b"cusolverDnZhetrd\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSorgtr_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnSorgtr_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDorgtr_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnDorgtr_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnCungtr_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnCungtr_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZungtr_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnZungtr_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSorgtr: {
                let p = get_proc_addr(lib, b"cusolverDnSorgtr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDorgtr: {
                let p = get_proc_addr(lib, b"cusolverDnDorgtr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnCungtr: {
                let p = get_proc_addr(lib, b"cusolverDnCungtr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZungtr: {
                let p = get_proc_addr(lib, b"cusolverDnZungtr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSormtr_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnSormtr_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDormtr_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnDormtr_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnCunmtr_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnCunmtr_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZunmtr_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnZunmtr_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSormtr: {
                let p = get_proc_addr(lib, b"cusolverDnSormtr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDormtr: {
                let p = get_proc_addr(lib, b"cusolverDnDormtr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnCunmtr: {
                let p = get_proc_addr(lib, b"cusolverDnCunmtr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZunmtr: {
                let p = get_proc_addr(lib, b"cusolverDnZunmtr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSgesvd_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnSgesvd_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDgesvd_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnDgesvd_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnCgesvd_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnCgesvd_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZgesvd_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnZgesvd_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSgesvd: {
                let p = get_proc_addr(lib, b"cusolverDnSgesvd\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDgesvd: {
                let p = get_proc_addr(lib, b"cusolverDnDgesvd\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnCgesvd: {
                let p = get_proc_addr(lib, b"cusolverDnCgesvd\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZgesvd: {
                let p = get_proc_addr(lib, b"cusolverDnZgesvd\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSsyevd_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnSsyevd_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDsyevd_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnDsyevd_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnCheevd_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnCheevd_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZheevd_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnZheevd_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSsyevd: {
                let p = get_proc_addr(lib, b"cusolverDnSsyevd\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDsyevd: {
                let p = get_proc_addr(lib, b"cusolverDnDsyevd\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnCheevd: {
                let p = get_proc_addr(lib, b"cusolverDnCheevd\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZheevd: {
                let p = get_proc_addr(lib, b"cusolverDnZheevd\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSsyevdx_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnSsyevdx_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDsyevdx_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnDsyevdx_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnCheevdx_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnCheevdx_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZheevdx_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnZheevdx_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSsyevdx: {
                let p = get_proc_addr(lib, b"cusolverDnSsyevdx\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDsyevdx: {
                let p = get_proc_addr(lib, b"cusolverDnDsyevdx\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnCheevdx: {
                let p = get_proc_addr(lib, b"cusolverDnCheevdx\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZheevdx: {
                let p = get_proc_addr(lib, b"cusolverDnZheevdx\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSsygvdx_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnSsygvdx_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDsygvdx_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnDsygvdx_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnChegvdx_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnChegvdx_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZhegvdx_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnZhegvdx_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSsygvdx: {
                let p = get_proc_addr(lib, b"cusolverDnSsygvdx\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDsygvdx: {
                let p = get_proc_addr(lib, b"cusolverDnDsygvdx\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnChegvdx: {
                let p = get_proc_addr(lib, b"cusolverDnChegvdx\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZhegvdx: {
                let p = get_proc_addr(lib, b"cusolverDnZhegvdx\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSsygvd_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnSsygvd_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDsygvd_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnDsygvd_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnChegvd_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnChegvd_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZhegvd_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnZhegvd_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSsygvd: {
                let p = get_proc_addr(lib, b"cusolverDnSsygvd\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDsygvd: {
                let p = get_proc_addr(lib, b"cusolverDnDsygvd\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnChegvd: {
                let p = get_proc_addr(lib, b"cusolverDnChegvd\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZhegvd: {
                let p = get_proc_addr(lib, b"cusolverDnZhegvd\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnXsygvd_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnXsygvd_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnXsygvd: {
                let p = get_proc_addr(lib, b"cusolverDnXsygvd\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnXsygvdx_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnXsygvdx_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnXsygvdx: {
                let p = get_proc_addr(lib, b"cusolverDnXsygvdx\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnCreateSyevjInfo: {
                let p = get_proc_addr(lib, b"cusolverDnCreateSyevjInfo\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDestroySyevjInfo: {
                let p = get_proc_addr(lib, b"cusolverDnDestroySyevjInfo\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnXsyevjSetTolerance: {
                let p = get_proc_addr(lib, b"cusolverDnXsyevjSetTolerance\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnXsyevjSetMaxSweeps: {
                let p = get_proc_addr(lib, b"cusolverDnXsyevjSetMaxSweeps\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnXsyevjSetSortEig: {
                let p = get_proc_addr(lib, b"cusolverDnXsyevjSetSortEig\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnXsyevjGetResidual: {
                let p = get_proc_addr(lib, b"cusolverDnXsyevjGetResidual\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnXsyevjGetSweeps: {
                let p = get_proc_addr(lib, b"cusolverDnXsyevjGetSweeps\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSsyevjBatched_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnSsyevjBatched_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDsyevjBatched_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnDsyevjBatched_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnCheevjBatched_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnCheevjBatched_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZheevjBatched_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnZheevjBatched_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSsyevjBatched: {
                let p = get_proc_addr(lib, b"cusolverDnSsyevjBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDsyevjBatched: {
                let p = get_proc_addr(lib, b"cusolverDnDsyevjBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnCheevjBatched: {
                let p = get_proc_addr(lib, b"cusolverDnCheevjBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZheevjBatched: {
                let p = get_proc_addr(lib, b"cusolverDnZheevjBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSsyevj_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnSsyevj_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDsyevj_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnDsyevj_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnCheevj_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnCheevj_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZheevj_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnZheevj_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSsyevj: {
                let p = get_proc_addr(lib, b"cusolverDnSsyevj\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDsyevj: {
                let p = get_proc_addr(lib, b"cusolverDnDsyevj\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnCheevj: {
                let p = get_proc_addr(lib, b"cusolverDnCheevj\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZheevj: {
                let p = get_proc_addr(lib, b"cusolverDnZheevj\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSsygvj_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnSsygvj_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDsygvj_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnDsygvj_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnChegvj_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnChegvj_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZhegvj_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnZhegvj_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSsygvj: {
                let p = get_proc_addr(lib, b"cusolverDnSsygvj\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDsygvj: {
                let p = get_proc_addr(lib, b"cusolverDnDsygvj\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnChegvj: {
                let p = get_proc_addr(lib, b"cusolverDnChegvj\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZhegvj: {
                let p = get_proc_addr(lib, b"cusolverDnZhegvj\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnCreateGesvdjInfo: {
                let p = get_proc_addr(lib, b"cusolverDnCreateGesvdjInfo\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDestroyGesvdjInfo: {
                let p = get_proc_addr(lib, b"cusolverDnDestroyGesvdjInfo\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnXgesvdjSetTolerance: {
                let p = get_proc_addr(lib, b"cusolverDnXgesvdjSetTolerance\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnXgesvdjSetMaxSweeps: {
                let p = get_proc_addr(lib, b"cusolverDnXgesvdjSetMaxSweeps\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnXgesvdjSetSortEig: {
                let p = get_proc_addr(lib, b"cusolverDnXgesvdjSetSortEig\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnXgesvdjGetResidual: {
                let p = get_proc_addr(lib, b"cusolverDnXgesvdjGetResidual\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnXgesvdjGetSweeps: {
                let p = get_proc_addr(lib, b"cusolverDnXgesvdjGetSweeps\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSgesvdjBatched_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnSgesvdjBatched_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDgesvdjBatched_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnDgesvdjBatched_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnCgesvdjBatched_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnCgesvdjBatched_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZgesvdjBatched_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnZgesvdjBatched_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSgesvdjBatched: {
                let p = get_proc_addr(lib, b"cusolverDnSgesvdjBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDgesvdjBatched: {
                let p = get_proc_addr(lib, b"cusolverDnDgesvdjBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnCgesvdjBatched: {
                let p = get_proc_addr(lib, b"cusolverDnCgesvdjBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZgesvdjBatched: {
                let p = get_proc_addr(lib, b"cusolverDnZgesvdjBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSgesvdj_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnSgesvdj_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDgesvdj_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnDgesvdj_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnCgesvdj_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnCgesvdj_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZgesvdj_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnZgesvdj_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSgesvdj: {
                let p = get_proc_addr(lib, b"cusolverDnSgesvdj\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDgesvdj: {
                let p = get_proc_addr(lib, b"cusolverDnDgesvdj\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnCgesvdj: {
                let p = get_proc_addr(lib, b"cusolverDnCgesvdj\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZgesvdj: {
                let p = get_proc_addr(lib, b"cusolverDnZgesvdj\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSgesvdaStridedBatched_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnSgesvdaStridedBatched_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDgesvdaStridedBatched_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnDgesvdaStridedBatched_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnCgesvdaStridedBatched_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnCgesvdaStridedBatched_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZgesvdaStridedBatched_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnZgesvdaStridedBatched_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSgesvdaStridedBatched: {
                let p = get_proc_addr(lib, b"cusolverDnSgesvdaStridedBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDgesvdaStridedBatched: {
                let p = get_proc_addr(lib, b"cusolverDnDgesvdaStridedBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnCgesvdaStridedBatched: {
                let p = get_proc_addr(lib, b"cusolverDnCgesvdaStridedBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnZgesvdaStridedBatched: {
                let p = get_proc_addr(lib, b"cusolverDnZgesvdaStridedBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnCreateParams: {
                let p = get_proc_addr(lib, b"cusolverDnCreateParams\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnDestroyParams: {
                let p = get_proc_addr(lib, b"cusolverDnDestroyParams\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnSetAdvOptions: {
                let p = get_proc_addr(lib, b"cusolverDnSetAdvOptions\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnXpotrf_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnXpotrf_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnXpotrf: {
                let p = get_proc_addr(lib, b"cusolverDnXpotrf\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnXpotrs: {
                let p = get_proc_addr(lib, b"cusolverDnXpotrs\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnXgeqrf_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnXgeqrf_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnXgeqrf: {
                let p = get_proc_addr(lib, b"cusolverDnXgeqrf\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnXgetrf_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnXgetrf_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnXgetrf: {
                let p = get_proc_addr(lib, b"cusolverDnXgetrf\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnXgetrs: {
                let p = get_proc_addr(lib, b"cusolverDnXgetrs\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnXsyevd_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnXsyevd_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnXsyevd: {
                let p = get_proc_addr(lib, b"cusolverDnXsyevd\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnXstedc_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnXstedc_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnXstedc: {
                let p = get_proc_addr(lib, b"cusolverDnXstedc\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnXsyevBatched_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnXsyevBatched_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnXsyevBatched: {
                let p = get_proc_addr(lib, b"cusolverDnXsyevBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnXsyevdx_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnXsyevdx_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnXsyevdx: {
                let p = get_proc_addr(lib, b"cusolverDnXsyevdx\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnXgeev_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnXgeev_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnXgeev: {
                let p = get_proc_addr(lib, b"cusolverDnXgeev\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnXgesvd_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnXgesvd_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnXgesvd: {
                let p = get_proc_addr(lib, b"cusolverDnXgesvd\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnXgesvdp_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnXgesvdp_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnXgesvdp: {
                let p = get_proc_addr(lib, b"cusolverDnXgesvdp\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnXgesvdr_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnXgesvdr_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnXgesvdr: {
                let p = get_proc_addr(lib, b"cusolverDnXgesvdr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnXlarft_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnXlarft_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnXlarft: {
                let p = get_proc_addr(lib, b"cusolverDnXlarft\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnLoggerSetCallback: {
                let p = get_proc_addr(lib, b"cusolverDnLoggerSetCallback\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnLoggerSetFile: {
                let p = get_proc_addr(lib, b"cusolverDnLoggerSetFile\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnLoggerOpenFile: {
                let p = get_proc_addr(lib, b"cusolverDnLoggerOpenFile\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnLoggerSetLevel: {
                let p = get_proc_addr(lib, b"cusolverDnLoggerSetLevel\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnLoggerSetMask: {
                let p = get_proc_addr(lib, b"cusolverDnLoggerSetMask\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnLoggerForceDisable: {
                let p = get_proc_addr(lib, b"cusolverDnLoggerForceDisable\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnXpolar_bufferSize: {
                let p = get_proc_addr(lib, b"cusolverDnXpolar_bufferSize\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverDnXpolar: {
                let p = get_proc_addr(lib, b"cusolverDnXpolar\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverSpCreate: {
                let p = get_proc_addr(lib, b"cusolverSpCreate\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverSpDestroy: {
                let p = get_proc_addr(lib, b"cusolverSpDestroy\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverSpSetStream: {
                let p = get_proc_addr(lib, b"cusolverSpSetStream\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverSpGetStream: {
                let p = get_proc_addr(lib, b"cusolverSpGetStream\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverSpXcsrissymHost: {
                let p = get_proc_addr(lib, b"cusolverSpXcsrissymHost\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverSpScsrlsvluHost: {
                let p = get_proc_addr(lib, b"cusolverSpScsrlsvluHost\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverSpDcsrlsvluHost: {
                let p = get_proc_addr(lib, b"cusolverSpDcsrlsvluHost\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverSpCcsrlsvluHost: {
                let p = get_proc_addr(lib, b"cusolverSpCcsrlsvluHost\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverSpZcsrlsvluHost: {
                let p = get_proc_addr(lib, b"cusolverSpZcsrlsvluHost\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverSpScsrlsvqr: {
                let p = get_proc_addr(lib, b"cusolverSpScsrlsvqr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverSpDcsrlsvqr: {
                let p = get_proc_addr(lib, b"cusolverSpDcsrlsvqr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverSpCcsrlsvqr: {
                let p = get_proc_addr(lib, b"cusolverSpCcsrlsvqr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverSpZcsrlsvqr: {
                let p = get_proc_addr(lib, b"cusolverSpZcsrlsvqr\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverSpScsrlsvqrHost: {
                let p = get_proc_addr(lib, b"cusolverSpScsrlsvqrHost\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverSpDcsrlsvqrHost: {
                let p = get_proc_addr(lib, b"cusolverSpDcsrlsvqrHost\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverSpCcsrlsvqrHost: {
                let p = get_proc_addr(lib, b"cusolverSpCcsrlsvqrHost\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverSpZcsrlsvqrHost: {
                let p = get_proc_addr(lib, b"cusolverSpZcsrlsvqrHost\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverSpScsrlsvcholHost: {
                let p = get_proc_addr(lib, b"cusolverSpScsrlsvcholHost\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverSpDcsrlsvcholHost: {
                let p = get_proc_addr(lib, b"cusolverSpDcsrlsvcholHost\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverSpCcsrlsvcholHost: {
                let p = get_proc_addr(lib, b"cusolverSpCcsrlsvcholHost\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverSpZcsrlsvcholHost: {
                let p = get_proc_addr(lib, b"cusolverSpZcsrlsvcholHost\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverSpScsrlsvchol: {
                let p = get_proc_addr(lib, b"cusolverSpScsrlsvchol\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverSpDcsrlsvchol: {
                let p = get_proc_addr(lib, b"cusolverSpDcsrlsvchol\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverSpCcsrlsvchol: {
                let p = get_proc_addr(lib, b"cusolverSpCcsrlsvchol\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverSpZcsrlsvchol: {
                let p = get_proc_addr(lib, b"cusolverSpZcsrlsvchol\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverSpScsrlsqvqrHost: {
                let p = get_proc_addr(lib, b"cusolverSpScsrlsqvqrHost\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverSpDcsrlsqvqrHost: {
                let p = get_proc_addr(lib, b"cusolverSpDcsrlsqvqrHost\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverSpCcsrlsqvqrHost: {
                let p = get_proc_addr(lib, b"cusolverSpCcsrlsqvqrHost\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverSpZcsrlsqvqrHost: {
                let p = get_proc_addr(lib, b"cusolverSpZcsrlsqvqrHost\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverSpScsreigvsiHost: {
                let p = get_proc_addr(lib, b"cusolverSpScsreigvsiHost\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverSpDcsreigvsiHost: {
                let p = get_proc_addr(lib, b"cusolverSpDcsreigvsiHost\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverSpCcsreigvsiHost: {
                let p = get_proc_addr(lib, b"cusolverSpCcsreigvsiHost\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverSpZcsreigvsiHost: {
                let p = get_proc_addr(lib, b"cusolverSpZcsreigvsiHost\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverSpScsreigvsi: {
                let p = get_proc_addr(lib, b"cusolverSpScsreigvsi\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverSpDcsreigvsi: {
                let p = get_proc_addr(lib, b"cusolverSpDcsreigvsi\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverSpCcsreigvsi: {
                let p = get_proc_addr(lib, b"cusolverSpCcsreigvsi\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverSpZcsreigvsi: {
                let p = get_proc_addr(lib, b"cusolverSpZcsreigvsi\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverSpScsreigsHost: {
                let p = get_proc_addr(lib, b"cusolverSpScsreigsHost\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverSpDcsreigsHost: {
                let p = get_proc_addr(lib, b"cusolverSpDcsreigsHost\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverSpCcsreigsHost: {
                let p = get_proc_addr(lib, b"cusolverSpCcsreigsHost\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverSpZcsreigsHost: {
                let p = get_proc_addr(lib, b"cusolverSpZcsreigsHost\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverSpXcsrsymrcmHost: {
                let p = get_proc_addr(lib, b"cusolverSpXcsrsymrcmHost\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverSpXcsrsymmdqHost: {
                let p = get_proc_addr(lib, b"cusolverSpXcsrsymmdqHost\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverSpXcsrsymamdHost: {
                let p = get_proc_addr(lib, b"cusolverSpXcsrsymamdHost\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverSpXcsrmetisndHost: {
                let p = get_proc_addr(lib, b"cusolverSpXcsrmetisndHost\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverSpScsrzfdHost: {
                let p = get_proc_addr(lib, b"cusolverSpScsrzfdHost\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverSpDcsrzfdHost: {
                let p = get_proc_addr(lib, b"cusolverSpDcsrzfdHost\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverSpCcsrzfdHost: {
                let p = get_proc_addr(lib, b"cusolverSpCcsrzfdHost\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverSpZcsrzfdHost: {
                let p = get_proc_addr(lib, b"cusolverSpZcsrzfdHost\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverSpXcsrperm_bufferSizeHost: {
                let p = get_proc_addr(lib, b"cusolverSpXcsrperm_bufferSizeHost\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverSpXcsrpermHost: {
                let p = get_proc_addr(lib, b"cusolverSpXcsrpermHost\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverSpCreateCsrqrInfo: {
                let p = get_proc_addr(lib, b"cusolverSpCreateCsrqrInfo\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverSpDestroyCsrqrInfo: {
                let p = get_proc_addr(lib, b"cusolverSpDestroyCsrqrInfo\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverSpXcsrqrAnalysisBatched: {
                let p = get_proc_addr(lib, b"cusolverSpXcsrqrAnalysisBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverSpScsrqrBufferInfoBatched: {
                let p = get_proc_addr(lib, b"cusolverSpScsrqrBufferInfoBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverSpDcsrqrBufferInfoBatched: {
                let p = get_proc_addr(lib, b"cusolverSpDcsrqrBufferInfoBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverSpCcsrqrBufferInfoBatched: {
                let p = get_proc_addr(lib, b"cusolverSpCcsrqrBufferInfoBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverSpZcsrqrBufferInfoBatched: {
                let p = get_proc_addr(lib, b"cusolverSpZcsrqrBufferInfoBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverSpScsrqrsvBatched: {
                let p = get_proc_addr(lib, b"cusolverSpScsrqrsvBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverSpDcsrqrsvBatched: {
                let p = get_proc_addr(lib, b"cusolverSpDcsrqrsvBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverSpCcsrqrsvBatched: {
                let p = get_proc_addr(lib, b"cusolverSpCcsrqrsvBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
            cusolverSpZcsrqrsvBatched: {
                let p = get_proc_addr(lib, b"cusolverSpZcsrqrsvBatched\0".as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute(p)) }
            },
        })
    };
    DYNAMIC_BINDINGS.set(bindings).ok();
}
