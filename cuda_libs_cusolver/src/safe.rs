pub use crate::sys::cusolverStatus_t as CudaTargetStatus;
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
    pub fn cusolverGetProperty(
        mut self,
        val: Option<
            unsafe extern "C" fn(type_: libraryPropertyType, value: *mut ::std::os::raw::c_int) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverGetProperty = val;
        self
    }
    pub fn cusolverGetVersion(
        mut self,
        val: Option<unsafe extern "C" fn(version: *mut ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverGetVersion = val;
        self
    }
    pub fn cusolverDnCreate(
        mut self,
        val: Option<unsafe extern "C" fn(handle: *mut cusolverDnHandle_t) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnCreate = val;
        self
    }
    pub fn cusolverDnDestroy(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnDestroy = val;
        self
    }
    pub fn cusolverDnSetStream(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, streamId: cudaStream_t) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnSetStream = val;
        self
    }
    pub fn cusolverDnGetStream(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, streamId: *mut cudaStream_t) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnGetStream = val;
        self
    }
    pub fn cusolverDnSetDeterministicMode(
        mut self,
        val: Option<
            unsafe extern "C" fn(handle: cusolverDnHandle_t, mode: cusolverDeterministicMode_t) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnSetDeterministicMode = val;
        self
    }
    pub fn cusolverDnGetDeterministicMode(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                mode: *mut cusolverDeterministicMode_t,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnGetDeterministicMode = val;
        self
    }
    pub fn cusolverDnSetMathMode(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cusolverDnHandle_t, mode: cusolverMathMode_t) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnSetMathMode = val;
        self
    }
    pub fn cusolverDnGetMathMode(
        mut self,
        val: Option<
            unsafe extern "C" fn(handle: cusolverDnHandle_t, mode: *mut cusolverMathMode_t) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnGetMathMode = val;
        self
    }
    pub fn cusolverDnSetEmulationStrategy(
        mut self,
        val: Option<
            unsafe extern "C" fn(handle: cusolverDnHandle_t, strategy: cudaEmulationStrategy_t) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnSetEmulationStrategy = val;
        self
    }
    pub fn cusolverDnGetEmulationStrategy(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                strategy: *mut cudaEmulationStrategy_t,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnGetEmulationStrategy = val;
        self
    }
    pub fn cusolverDnSetFixedPointEmulationMantissaControl(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                control: cudaEmulationMantissaControl_t,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnSetFixedPointEmulationMantissaControl = val;
        self
    }
    pub fn cusolverDnGetFixedPointEmulationMantissaControl(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                control: *mut cudaEmulationMantissaControl_t,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnGetFixedPointEmulationMantissaControl = val;
        self
    }
    pub fn cusolverDnSetFixedPointEmulationMaxMantissaBitCount(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                mantissaBitCount: ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnSetFixedPointEmulationMaxMantissaBitCount = val;
        self
    }
    pub fn cusolverDnGetFixedPointEmulationMaxMantissaBitCount(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                mantissaBitCount: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnGetFixedPointEmulationMaxMantissaBitCount = val;
        self
    }
    pub fn cusolverDnSetFixedPointEmulationMantissaBitOffset(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                mantissaBitOffset: ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnSetFixedPointEmulationMantissaBitOffset = val;
        self
    }
    pub fn cusolverDnGetFixedPointEmulationMantissaBitOffset(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                mantissaBitOffset: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnGetFixedPointEmulationMantissaBitOffset = val;
        self
    }
    pub fn cusolverDnSetEmulationSpecialValuesSupport(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                mask: cudaEmulationSpecialValuesSupport_t,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnSetEmulationSpecialValuesSupport = val;
        self
    }
    pub fn cusolverDnGetEmulationSpecialValuesSupport(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                mask: *mut cudaEmulationSpecialValuesSupport_t,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnGetEmulationSpecialValuesSupport = val;
        self
    }
    pub fn cusolverDnIRSParamsCreate(
        mut self,
        val: Option<unsafe extern "C" fn(params_ptr: *mut cusolverDnIRSParams_t) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnIRSParamsCreate = val;
        self
    }
    pub fn cusolverDnIRSParamsDestroy(
        mut self,
        val: Option<unsafe extern "C" fn(params: cusolverDnIRSParams_t) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnIRSParamsDestroy = val;
        self
    }
    pub fn cusolverDnIRSParamsSetRefinementSolver(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                params: cusolverDnIRSParams_t,
                refinement_solver: cusolverIRSRefinement_t,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnIRSParamsSetRefinementSolver = val;
        self
    }
    pub fn cusolverDnIRSParamsSetSolverMainPrecision(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                params: cusolverDnIRSParams_t,
                solver_main_precision: cusolverPrecType_t,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnIRSParamsSetSolverMainPrecision = val;
        self
    }
    pub fn cusolverDnIRSParamsSetSolverLowestPrecision(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                params: cusolverDnIRSParams_t,
                solver_lowest_precision: cusolverPrecType_t,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnIRSParamsSetSolverLowestPrecision = val;
        self
    }
    pub fn cusolverDnIRSParamsSetSolverPrecisions(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                params: cusolverDnIRSParams_t,
                solver_main_precision: cusolverPrecType_t,
                solver_lowest_precision: cusolverPrecType_t,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnIRSParamsSetSolverPrecisions = val;
        self
    }
    pub fn cusolverDnIRSParamsSetTol(
        mut self,
        val: Option<unsafe extern "C" fn(params: cusolverDnIRSParams_t, val: f64) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnIRSParamsSetTol = val;
        self
    }
    pub fn cusolverDnIRSParamsSetTolInner(
        mut self,
        val: Option<unsafe extern "C" fn(params: cusolverDnIRSParams_t, val: f64) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnIRSParamsSetTolInner = val;
        self
    }
    pub fn cusolverDnIRSParamsSetMaxIters(
        mut self,
        val: Option<unsafe extern "C" fn(params: cusolverDnIRSParams_t, maxiters: cusolver_int_t) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnIRSParamsSetMaxIters = val;
        self
    }
    pub fn cusolverDnIRSParamsSetMaxItersInner(
        mut self,
        val: Option<
            unsafe extern "C" fn(params: cusolverDnIRSParams_t, maxiters_inner: cusolver_int_t) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnIRSParamsSetMaxItersInner = val;
        self
    }
    pub fn cusolverDnIRSParamsGetMaxIters(
        mut self,
        val: Option<
            unsafe extern "C" fn(params: cusolverDnIRSParams_t, maxiters: *mut cusolver_int_t) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnIRSParamsGetMaxIters = val;
        self
    }
    pub fn cusolverDnIRSParamsEnableFallback(
        mut self,
        val: Option<unsafe extern "C" fn(params: cusolverDnIRSParams_t) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnIRSParamsEnableFallback = val;
        self
    }
    pub fn cusolverDnIRSParamsDisableFallback(
        mut self,
        val: Option<unsafe extern "C" fn(params: cusolverDnIRSParams_t) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnIRSParamsDisableFallback = val;
        self
    }
    pub fn cusolverDnIRSInfosDestroy(
        mut self,
        val: Option<unsafe extern "C" fn(infos: cusolverDnIRSInfos_t) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnIRSInfosDestroy = val;
        self
    }
    pub fn cusolverDnIRSInfosCreate(
        mut self,
        val: Option<unsafe extern "C" fn(infos_ptr: *mut cusolverDnIRSInfos_t) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnIRSInfosCreate = val;
        self
    }
    pub fn cusolverDnIRSInfosGetNiters(
        mut self,
        val: Option<unsafe extern "C" fn(infos: cusolverDnIRSInfos_t, niters: *mut cusolver_int_t) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnIRSInfosGetNiters = val;
        self
    }
    pub fn cusolverDnIRSInfosGetOuterNiters(
        mut self,
        val: Option<
            unsafe extern "C" fn(infos: cusolverDnIRSInfos_t, outer_niters: *mut cusolver_int_t) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnIRSInfosGetOuterNiters = val;
        self
    }
    pub fn cusolverDnIRSInfosRequestResidual(
        mut self,
        val: Option<unsafe extern "C" fn(infos: cusolverDnIRSInfos_t) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnIRSInfosRequestResidual = val;
        self
    }
    pub fn cusolverDnIRSInfosGetResidualHistory(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                infos: cusolverDnIRSInfos_t,
                residual_history: *mut *mut ::std::os::raw::c_void,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnIRSInfosGetResidualHistory = val;
        self
    }
    pub fn cusolverDnIRSInfosGetMaxIters(
        mut self,
        val: Option<
            unsafe extern "C" fn(infos: cusolverDnIRSInfos_t, maxiters: *mut cusolver_int_t) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnIRSInfosGetMaxIters = val;
        self
    }
    pub fn cusolverDnZZgesv(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnZZgesv = val;
        self
    }
    pub fn cusolverDnZCgesv(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnZCgesv = val;
        self
    }
    pub fn cusolverDnZKgesv(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnZKgesv = val;
        self
    }
    pub fn cusolverDnZEgesv(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnZEgesv = val;
        self
    }
    pub fn cusolverDnZYgesv(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnZYgesv = val;
        self
    }
    pub fn cusolverDnCCgesv(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnCCgesv = val;
        self
    }
    pub fn cusolverDnCEgesv(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnCEgesv = val;
        self
    }
    pub fn cusolverDnCKgesv(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnCKgesv = val;
        self
    }
    pub fn cusolverDnCYgesv(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnCYgesv = val;
        self
    }
    pub fn cusolverDnDDgesv(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnDDgesv = val;
        self
    }
    pub fn cusolverDnDSgesv(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnDSgesv = val;
        self
    }
    pub fn cusolverDnDHgesv(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnDHgesv = val;
        self
    }
    pub fn cusolverDnDBgesv(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnDBgesv = val;
        self
    }
    pub fn cusolverDnDXgesv(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnDXgesv = val;
        self
    }
    pub fn cusolverDnSSgesv(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnSSgesv = val;
        self
    }
    pub fn cusolverDnSHgesv(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnSHgesv = val;
        self
    }
    pub fn cusolverDnSBgesv(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnSBgesv = val;
        self
    }
    pub fn cusolverDnSXgesv(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnSXgesv = val;
        self
    }
    pub fn cusolverDnZZgesv_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnZZgesv_bufferSize = val;
        self
    }
    pub fn cusolverDnZCgesv_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnZCgesv_bufferSize = val;
        self
    }
    pub fn cusolverDnZKgesv_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnZKgesv_bufferSize = val;
        self
    }
    pub fn cusolverDnZEgesv_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnZEgesv_bufferSize = val;
        self
    }
    pub fn cusolverDnZYgesv_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnZYgesv_bufferSize = val;
        self
    }
    pub fn cusolverDnCCgesv_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnCCgesv_bufferSize = val;
        self
    }
    pub fn cusolverDnCKgesv_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnCKgesv_bufferSize = val;
        self
    }
    pub fn cusolverDnCEgesv_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnCEgesv_bufferSize = val;
        self
    }
    pub fn cusolverDnCYgesv_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnCYgesv_bufferSize = val;
        self
    }
    pub fn cusolverDnDDgesv_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnDDgesv_bufferSize = val;
        self
    }
    pub fn cusolverDnDSgesv_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnDSgesv_bufferSize = val;
        self
    }
    pub fn cusolverDnDHgesv_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnDHgesv_bufferSize = val;
        self
    }
    pub fn cusolverDnDBgesv_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnDBgesv_bufferSize = val;
        self
    }
    pub fn cusolverDnDXgesv_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnDXgesv_bufferSize = val;
        self
    }
    pub fn cusolverDnSSgesv_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnSSgesv_bufferSize = val;
        self
    }
    pub fn cusolverDnSHgesv_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnSHgesv_bufferSize = val;
        self
    }
    pub fn cusolverDnSBgesv_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnSBgesv_bufferSize = val;
        self
    }
    pub fn cusolverDnSXgesv_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnSXgesv_bufferSize = val;
        self
    }
    pub fn cusolverDnZZgels(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnZZgels = val;
        self
    }
    pub fn cusolverDnZCgels(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnZCgels = val;
        self
    }
    pub fn cusolverDnZKgels(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnZKgels = val;
        self
    }
    pub fn cusolverDnZEgels(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnZEgels = val;
        self
    }
    pub fn cusolverDnZYgels(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnZYgels = val;
        self
    }
    pub fn cusolverDnCCgels(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnCCgels = val;
        self
    }
    pub fn cusolverDnCKgels(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnCKgels = val;
        self
    }
    pub fn cusolverDnCEgels(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnCEgels = val;
        self
    }
    pub fn cusolverDnCYgels(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnCYgels = val;
        self
    }
    pub fn cusolverDnDDgels(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnDDgels = val;
        self
    }
    pub fn cusolverDnDSgels(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnDSgels = val;
        self
    }
    pub fn cusolverDnDHgels(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnDHgels = val;
        self
    }
    pub fn cusolverDnDBgels(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnDBgels = val;
        self
    }
    pub fn cusolverDnDXgels(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnDXgels = val;
        self
    }
    pub fn cusolverDnSSgels(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnSSgels = val;
        self
    }
    pub fn cusolverDnSHgels(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnSHgels = val;
        self
    }
    pub fn cusolverDnSBgels(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnSBgels = val;
        self
    }
    pub fn cusolverDnSXgels(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnSXgels = val;
        self
    }
    pub fn cusolverDnZZgels_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnZZgels_bufferSize = val;
        self
    }
    pub fn cusolverDnZCgels_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnZCgels_bufferSize = val;
        self
    }
    pub fn cusolverDnZKgels_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnZKgels_bufferSize = val;
        self
    }
    pub fn cusolverDnZEgels_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnZEgels_bufferSize = val;
        self
    }
    pub fn cusolverDnZYgels_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnZYgels_bufferSize = val;
        self
    }
    pub fn cusolverDnCCgels_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnCCgels_bufferSize = val;
        self
    }
    pub fn cusolverDnCKgels_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnCKgels_bufferSize = val;
        self
    }
    pub fn cusolverDnCEgels_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnCEgels_bufferSize = val;
        self
    }
    pub fn cusolverDnCYgels_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnCYgels_bufferSize = val;
        self
    }
    pub fn cusolverDnDDgels_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnDDgels_bufferSize = val;
        self
    }
    pub fn cusolverDnDSgels_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnDSgels_bufferSize = val;
        self
    }
    pub fn cusolverDnDHgels_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnDHgels_bufferSize = val;
        self
    }
    pub fn cusolverDnDBgels_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnDBgels_bufferSize = val;
        self
    }
    pub fn cusolverDnDXgels_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnDXgels_bufferSize = val;
        self
    }
    pub fn cusolverDnSSgels_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnSSgels_bufferSize = val;
        self
    }
    pub fn cusolverDnSHgels_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnSHgels_bufferSize = val;
        self
    }
    pub fn cusolverDnSBgels_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnSBgels_bufferSize = val;
        self
    }
    pub fn cusolverDnSXgels_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnSXgels_bufferSize = val;
        self
    }
    pub fn cusolverDnIRSXgesv(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnIRSXgesv = val;
        self
    }
    pub fn cusolverDnIRSXgesv_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                params: cusolverDnIRSParams_t,
                n: cusolver_int_t,
                nrhs: cusolver_int_t,
                lwork_bytes: *mut usize,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnIRSXgesv_bufferSize = val;
        self
    }
    pub fn cusolverDnIRSXgels(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnIRSXgels = val;
        self
    }
    pub fn cusolverDnIRSXgels_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                params: cusolverDnIRSParams_t,
                m: cusolver_int_t,
                n: cusolver_int_t,
                nrhs: cusolver_int_t,
                lwork_bytes: *mut usize,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnIRSXgels_bufferSize = val;
        self
    }
    pub fn cusolverDnSpotrf_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                uplo: cublasFillMode_t,
                n: ::std::os::raw::c_int,
                A: *mut f32,
                lda: ::std::os::raw::c_int,
                Lwork: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnSpotrf_bufferSize = val;
        self
    }
    pub fn cusolverDnDpotrf_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                uplo: cublasFillMode_t,
                n: ::std::os::raw::c_int,
                A: *mut f64,
                lda: ::std::os::raw::c_int,
                Lwork: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnDpotrf_bufferSize = val;
        self
    }
    pub fn cusolverDnCpotrf_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                uplo: cublasFillMode_t,
                n: ::std::os::raw::c_int,
                A: *mut cuComplex,
                lda: ::std::os::raw::c_int,
                Lwork: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnCpotrf_bufferSize = val;
        self
    }
    pub fn cusolverDnZpotrf_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                uplo: cublasFillMode_t,
                n: ::std::os::raw::c_int,
                A: *mut cuDoubleComplex,
                lda: ::std::os::raw::c_int,
                Lwork: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnZpotrf_bufferSize = val;
        self
    }
    pub fn cusolverDnSpotrf(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                uplo: cublasFillMode_t,
                n: ::std::os::raw::c_int,
                A: *mut f32,
                lda: ::std::os::raw::c_int,
                Workspace: *mut f32,
                Lwork: ::std::os::raw::c_int,
                devInfo: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnSpotrf = val;
        self
    }
    pub fn cusolverDnDpotrf(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                uplo: cublasFillMode_t,
                n: ::std::os::raw::c_int,
                A: *mut f64,
                lda: ::std::os::raw::c_int,
                Workspace: *mut f64,
                Lwork: ::std::os::raw::c_int,
                devInfo: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnDpotrf = val;
        self
    }
    pub fn cusolverDnCpotrf(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                uplo: cublasFillMode_t,
                n: ::std::os::raw::c_int,
                A: *mut cuComplex,
                lda: ::std::os::raw::c_int,
                Workspace: *mut cuComplex,
                Lwork: ::std::os::raw::c_int,
                devInfo: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnCpotrf = val;
        self
    }
    pub fn cusolverDnZpotrf(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                uplo: cublasFillMode_t,
                n: ::std::os::raw::c_int,
                A: *mut cuDoubleComplex,
                lda: ::std::os::raw::c_int,
                Workspace: *mut cuDoubleComplex,
                Lwork: ::std::os::raw::c_int,
                devInfo: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnZpotrf = val;
        self
    }
    pub fn cusolverDnSpotrs(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                uplo: cublasFillMode_t,
                n: ::std::os::raw::c_int,
                nrhs: ::std::os::raw::c_int,
                A: *const f32,
                lda: ::std::os::raw::c_int,
                B: *mut f32,
                ldb: ::std::os::raw::c_int,
                devInfo: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnSpotrs = val;
        self
    }
    pub fn cusolverDnDpotrs(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                uplo: cublasFillMode_t,
                n: ::std::os::raw::c_int,
                nrhs: ::std::os::raw::c_int,
                A: *const f64,
                lda: ::std::os::raw::c_int,
                B: *mut f64,
                ldb: ::std::os::raw::c_int,
                devInfo: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnDpotrs = val;
        self
    }
    pub fn cusolverDnCpotrs(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                uplo: cublasFillMode_t,
                n: ::std::os::raw::c_int,
                nrhs: ::std::os::raw::c_int,
                A: *const cuComplex,
                lda: ::std::os::raw::c_int,
                B: *mut cuComplex,
                ldb: ::std::os::raw::c_int,
                devInfo: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnCpotrs = val;
        self
    }
    pub fn cusolverDnZpotrs(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnZpotrs = val;
        self
    }
    pub fn cusolverDnSpotrfBatched(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                uplo: cublasFillMode_t,
                n: ::std::os::raw::c_int,
                Aarray: *mut *mut f32,
                lda: ::std::os::raw::c_int,
                infoArray: *mut ::std::os::raw::c_int,
                batchSize: ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnSpotrfBatched = val;
        self
    }
    pub fn cusolverDnDpotrfBatched(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                uplo: cublasFillMode_t,
                n: ::std::os::raw::c_int,
                Aarray: *mut *mut f64,
                lda: ::std::os::raw::c_int,
                infoArray: *mut ::std::os::raw::c_int,
                batchSize: ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnDpotrfBatched = val;
        self
    }
    pub fn cusolverDnCpotrfBatched(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                uplo: cublasFillMode_t,
                n: ::std::os::raw::c_int,
                Aarray: *mut *mut cuComplex,
                lda: ::std::os::raw::c_int,
                infoArray: *mut ::std::os::raw::c_int,
                batchSize: ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnCpotrfBatched = val;
        self
    }
    pub fn cusolverDnZpotrfBatched(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                uplo: cublasFillMode_t,
                n: ::std::os::raw::c_int,
                Aarray: *mut *mut cuDoubleComplex,
                lda: ::std::os::raw::c_int,
                infoArray: *mut ::std::os::raw::c_int,
                batchSize: ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnZpotrfBatched = val;
        self
    }
    pub fn cusolverDnSpotrsBatched(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnSpotrsBatched = val;
        self
    }
    pub fn cusolverDnDpotrsBatched(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnDpotrsBatched = val;
        self
    }
    pub fn cusolverDnCpotrsBatched(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnCpotrsBatched = val;
        self
    }
    pub fn cusolverDnZpotrsBatched(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnZpotrsBatched = val;
        self
    }
    pub fn cusolverDnSpotri_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                uplo: cublasFillMode_t,
                n: ::std::os::raw::c_int,
                A: *mut f32,
                lda: ::std::os::raw::c_int,
                lwork: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnSpotri_bufferSize = val;
        self
    }
    pub fn cusolverDnDpotri_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                uplo: cublasFillMode_t,
                n: ::std::os::raw::c_int,
                A: *mut f64,
                lda: ::std::os::raw::c_int,
                lwork: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnDpotri_bufferSize = val;
        self
    }
    pub fn cusolverDnCpotri_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                uplo: cublasFillMode_t,
                n: ::std::os::raw::c_int,
                A: *mut cuComplex,
                lda: ::std::os::raw::c_int,
                lwork: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnCpotri_bufferSize = val;
        self
    }
    pub fn cusolverDnZpotri_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                uplo: cublasFillMode_t,
                n: ::std::os::raw::c_int,
                A: *mut cuDoubleComplex,
                lda: ::std::os::raw::c_int,
                lwork: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnZpotri_bufferSize = val;
        self
    }
    pub fn cusolverDnSpotri(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                uplo: cublasFillMode_t,
                n: ::std::os::raw::c_int,
                A: *mut f32,
                lda: ::std::os::raw::c_int,
                work: *mut f32,
                lwork: ::std::os::raw::c_int,
                devInfo: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnSpotri = val;
        self
    }
    pub fn cusolverDnDpotri(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                uplo: cublasFillMode_t,
                n: ::std::os::raw::c_int,
                A: *mut f64,
                lda: ::std::os::raw::c_int,
                work: *mut f64,
                lwork: ::std::os::raw::c_int,
                devInfo: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnDpotri = val;
        self
    }
    pub fn cusolverDnCpotri(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                uplo: cublasFillMode_t,
                n: ::std::os::raw::c_int,
                A: *mut cuComplex,
                lda: ::std::os::raw::c_int,
                work: *mut cuComplex,
                lwork: ::std::os::raw::c_int,
                devInfo: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnCpotri = val;
        self
    }
    pub fn cusolverDnZpotri(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                uplo: cublasFillMode_t,
                n: ::std::os::raw::c_int,
                A: *mut cuDoubleComplex,
                lda: ::std::os::raw::c_int,
                work: *mut cuDoubleComplex,
                lwork: ::std::os::raw::c_int,
                devInfo: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnZpotri = val;
        self
    }
    pub fn cusolverDnXtrtri_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                uplo: cublasFillMode_t,
                diag: cublasDiagType_t,
                n: i64,
                dataTypeA: cudaDataType,
                A: *mut ::std::os::raw::c_void,
                lda: i64,
                workspaceInBytesOnDevice: *mut usize,
                workspaceInBytesOnHost: *mut usize,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnXtrtri_bufferSize = val;
        self
    }
    pub fn cusolverDnXtrtri(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnXtrtri = val;
        self
    }
    pub fn cusolverDnSlauum_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                uplo: cublasFillMode_t,
                n: ::std::os::raw::c_int,
                A: *mut f32,
                lda: ::std::os::raw::c_int,
                lwork: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnSlauum_bufferSize = val;
        self
    }
    pub fn cusolverDnDlauum_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                uplo: cublasFillMode_t,
                n: ::std::os::raw::c_int,
                A: *mut f64,
                lda: ::std::os::raw::c_int,
                lwork: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnDlauum_bufferSize = val;
        self
    }
    pub fn cusolverDnClauum_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                uplo: cublasFillMode_t,
                n: ::std::os::raw::c_int,
                A: *mut cuComplex,
                lda: ::std::os::raw::c_int,
                lwork: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnClauum_bufferSize = val;
        self
    }
    pub fn cusolverDnZlauum_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                uplo: cublasFillMode_t,
                n: ::std::os::raw::c_int,
                A: *mut cuDoubleComplex,
                lda: ::std::os::raw::c_int,
                lwork: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnZlauum_bufferSize = val;
        self
    }
    pub fn cusolverDnSlauum(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                uplo: cublasFillMode_t,
                n: ::std::os::raw::c_int,
                A: *mut f32,
                lda: ::std::os::raw::c_int,
                work: *mut f32,
                lwork: ::std::os::raw::c_int,
                devInfo: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnSlauum = val;
        self
    }
    pub fn cusolverDnDlauum(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                uplo: cublasFillMode_t,
                n: ::std::os::raw::c_int,
                A: *mut f64,
                lda: ::std::os::raw::c_int,
                work: *mut f64,
                lwork: ::std::os::raw::c_int,
                devInfo: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnDlauum = val;
        self
    }
    pub fn cusolverDnClauum(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                uplo: cublasFillMode_t,
                n: ::std::os::raw::c_int,
                A: *mut cuComplex,
                lda: ::std::os::raw::c_int,
                work: *mut cuComplex,
                lwork: ::std::os::raw::c_int,
                devInfo: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnClauum = val;
        self
    }
    pub fn cusolverDnZlauum(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                uplo: cublasFillMode_t,
                n: ::std::os::raw::c_int,
                A: *mut cuDoubleComplex,
                lda: ::std::os::raw::c_int,
                work: *mut cuDoubleComplex,
                lwork: ::std::os::raw::c_int,
                devInfo: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnZlauum = val;
        self
    }
    pub fn cusolverDnSgetrf_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                A: *mut f32,
                lda: ::std::os::raw::c_int,
                Lwork: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnSgetrf_bufferSize = val;
        self
    }
    pub fn cusolverDnDgetrf_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                A: *mut f64,
                lda: ::std::os::raw::c_int,
                Lwork: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnDgetrf_bufferSize = val;
        self
    }
    pub fn cusolverDnCgetrf_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                A: *mut cuComplex,
                lda: ::std::os::raw::c_int,
                Lwork: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnCgetrf_bufferSize = val;
        self
    }
    pub fn cusolverDnZgetrf_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                A: *mut cuDoubleComplex,
                lda: ::std::os::raw::c_int,
                Lwork: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnZgetrf_bufferSize = val;
        self
    }
    pub fn cusolverDnSgetrf(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                A: *mut f32,
                lda: ::std::os::raw::c_int,
                Workspace: *mut f32,
                devIpiv: *mut ::std::os::raw::c_int,
                devInfo: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnSgetrf = val;
        self
    }
    pub fn cusolverDnDgetrf(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                A: *mut f64,
                lda: ::std::os::raw::c_int,
                Workspace: *mut f64,
                devIpiv: *mut ::std::os::raw::c_int,
                devInfo: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnDgetrf = val;
        self
    }
    pub fn cusolverDnCgetrf(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                A: *mut cuComplex,
                lda: ::std::os::raw::c_int,
                Workspace: *mut cuComplex,
                devIpiv: *mut ::std::os::raw::c_int,
                devInfo: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnCgetrf = val;
        self
    }
    pub fn cusolverDnZgetrf(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                A: *mut cuDoubleComplex,
                lda: ::std::os::raw::c_int,
                Workspace: *mut cuDoubleComplex,
                devIpiv: *mut ::std::os::raw::c_int,
                devInfo: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnZgetrf = val;
        self
    }
    pub fn cusolverDnSlaswp(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                n: ::std::os::raw::c_int,
                A: *mut f32,
                lda: ::std::os::raw::c_int,
                k1: ::std::os::raw::c_int,
                k2: ::std::os::raw::c_int,
                devIpiv: *const ::std::os::raw::c_int,
                incx: ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnSlaswp = val;
        self
    }
    pub fn cusolverDnDlaswp(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                n: ::std::os::raw::c_int,
                A: *mut f64,
                lda: ::std::os::raw::c_int,
                k1: ::std::os::raw::c_int,
                k2: ::std::os::raw::c_int,
                devIpiv: *const ::std::os::raw::c_int,
                incx: ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnDlaswp = val;
        self
    }
    pub fn cusolverDnClaswp(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                n: ::std::os::raw::c_int,
                A: *mut cuComplex,
                lda: ::std::os::raw::c_int,
                k1: ::std::os::raw::c_int,
                k2: ::std::os::raw::c_int,
                devIpiv: *const ::std::os::raw::c_int,
                incx: ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnClaswp = val;
        self
    }
    pub fn cusolverDnZlaswp(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                n: ::std::os::raw::c_int,
                A: *mut cuDoubleComplex,
                lda: ::std::os::raw::c_int,
                k1: ::std::os::raw::c_int,
                k2: ::std::os::raw::c_int,
                devIpiv: *const ::std::os::raw::c_int,
                incx: ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnZlaswp = val;
        self
    }
    pub fn cusolverDnSgetrs(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnSgetrs = val;
        self
    }
    pub fn cusolverDnDgetrs(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnDgetrs = val;
        self
    }
    pub fn cusolverDnCgetrs(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnCgetrs = val;
        self
    }
    pub fn cusolverDnZgetrs(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnZgetrs = val;
        self
    }
    pub fn cusolverDnSgeqrf_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                A: *mut f32,
                lda: ::std::os::raw::c_int,
                lwork: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnSgeqrf_bufferSize = val;
        self
    }
    pub fn cusolverDnDgeqrf_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                A: *mut f64,
                lda: ::std::os::raw::c_int,
                lwork: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnDgeqrf_bufferSize = val;
        self
    }
    pub fn cusolverDnCgeqrf_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                A: *mut cuComplex,
                lda: ::std::os::raw::c_int,
                lwork: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnCgeqrf_bufferSize = val;
        self
    }
    pub fn cusolverDnZgeqrf_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                A: *mut cuDoubleComplex,
                lda: ::std::os::raw::c_int,
                lwork: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnZgeqrf_bufferSize = val;
        self
    }
    pub fn cusolverDnSgeqrf(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                A: *mut f32,
                lda: ::std::os::raw::c_int,
                TAU: *mut f32,
                Workspace: *mut f32,
                Lwork: ::std::os::raw::c_int,
                devInfo: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnSgeqrf = val;
        self
    }
    pub fn cusolverDnDgeqrf(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                A: *mut f64,
                lda: ::std::os::raw::c_int,
                TAU: *mut f64,
                Workspace: *mut f64,
                Lwork: ::std::os::raw::c_int,
                devInfo: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnDgeqrf = val;
        self
    }
    pub fn cusolverDnCgeqrf(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                A: *mut cuComplex,
                lda: ::std::os::raw::c_int,
                TAU: *mut cuComplex,
                Workspace: *mut cuComplex,
                Lwork: ::std::os::raw::c_int,
                devInfo: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnCgeqrf = val;
        self
    }
    pub fn cusolverDnZgeqrf(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnZgeqrf = val;
        self
    }
    pub fn cusolverDnSorgqr_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                k: ::std::os::raw::c_int,
                A: *const f32,
                lda: ::std::os::raw::c_int,
                tau: *const f32,
                lwork: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnSorgqr_bufferSize = val;
        self
    }
    pub fn cusolverDnDorgqr_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                k: ::std::os::raw::c_int,
                A: *const f64,
                lda: ::std::os::raw::c_int,
                tau: *const f64,
                lwork: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnDorgqr_bufferSize = val;
        self
    }
    pub fn cusolverDnCungqr_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                k: ::std::os::raw::c_int,
                A: *const cuComplex,
                lda: ::std::os::raw::c_int,
                tau: *const cuComplex,
                lwork: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnCungqr_bufferSize = val;
        self
    }
    pub fn cusolverDnZungqr_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                k: ::std::os::raw::c_int,
                A: *const cuDoubleComplex,
                lda: ::std::os::raw::c_int,
                tau: *const cuDoubleComplex,
                lwork: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnZungqr_bufferSize = val;
        self
    }
    pub fn cusolverDnSorgqr(
        mut self,
        val: Option<
            unsafe extern "C" fn(
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
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnSorgqr = val;
        self
    }
    pub fn cusolverDnDorgqr(
        mut self,
        val: Option<
            unsafe extern "C" fn(
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
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnDorgqr = val;
        self
    }
    pub fn cusolverDnCungqr(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnCungqr = val;
        self
    }
    pub fn cusolverDnZungqr(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnZungqr = val;
        self
    }
    pub fn cusolverDnSormqr_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnSormqr_bufferSize = val;
        self
    }
    pub fn cusolverDnDormqr_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnDormqr_bufferSize = val;
        self
    }
    pub fn cusolverDnCunmqr_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnCunmqr_bufferSize = val;
        self
    }
    pub fn cusolverDnZunmqr_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnZunmqr_bufferSize = val;
        self
    }
    pub fn cusolverDnSormqr(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnSormqr = val;
        self
    }
    pub fn cusolverDnDormqr(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnDormqr = val;
        self
    }
    pub fn cusolverDnCunmqr(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnCunmqr = val;
        self
    }
    pub fn cusolverDnZunmqr(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnZunmqr = val;
        self
    }
    pub fn cusolverDnSsytrf_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                n: ::std::os::raw::c_int,
                A: *mut f32,
                lda: ::std::os::raw::c_int,
                lwork: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnSsytrf_bufferSize = val;
        self
    }
    pub fn cusolverDnDsytrf_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                n: ::std::os::raw::c_int,
                A: *mut f64,
                lda: ::std::os::raw::c_int,
                lwork: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnDsytrf_bufferSize = val;
        self
    }
    pub fn cusolverDnCsytrf_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                n: ::std::os::raw::c_int,
                A: *mut cuComplex,
                lda: ::std::os::raw::c_int,
                lwork: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnCsytrf_bufferSize = val;
        self
    }
    pub fn cusolverDnZsytrf_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                n: ::std::os::raw::c_int,
                A: *mut cuDoubleComplex,
                lda: ::std::os::raw::c_int,
                lwork: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnZsytrf_bufferSize = val;
        self
    }
    pub fn cusolverDnSsytrf(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                uplo: cublasFillMode_t,
                n: ::std::os::raw::c_int,
                A: *mut f32,
                lda: ::std::os::raw::c_int,
                ipiv: *mut ::std::os::raw::c_int,
                work: *mut f32,
                lwork: ::std::os::raw::c_int,
                info: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnSsytrf = val;
        self
    }
    pub fn cusolverDnDsytrf(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                uplo: cublasFillMode_t,
                n: ::std::os::raw::c_int,
                A: *mut f64,
                lda: ::std::os::raw::c_int,
                ipiv: *mut ::std::os::raw::c_int,
                work: *mut f64,
                lwork: ::std::os::raw::c_int,
                info: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnDsytrf = val;
        self
    }
    pub fn cusolverDnCsytrf(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                uplo: cublasFillMode_t,
                n: ::std::os::raw::c_int,
                A: *mut cuComplex,
                lda: ::std::os::raw::c_int,
                ipiv: *mut ::std::os::raw::c_int,
                work: *mut cuComplex,
                lwork: ::std::os::raw::c_int,
                info: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnCsytrf = val;
        self
    }
    pub fn cusolverDnZsytrf(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnZsytrf = val;
        self
    }
    pub fn cusolverDnXsytrs_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnXsytrs_bufferSize = val;
        self
    }
    pub fn cusolverDnXsytrs(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnXsytrs = val;
        self
    }
    pub fn cusolverDnSsytri_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                uplo: cublasFillMode_t,
                n: ::std::os::raw::c_int,
                A: *mut f32,
                lda: ::std::os::raw::c_int,
                ipiv: *const ::std::os::raw::c_int,
                lwork: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnSsytri_bufferSize = val;
        self
    }
    pub fn cusolverDnDsytri_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                uplo: cublasFillMode_t,
                n: ::std::os::raw::c_int,
                A: *mut f64,
                lda: ::std::os::raw::c_int,
                ipiv: *const ::std::os::raw::c_int,
                lwork: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnDsytri_bufferSize = val;
        self
    }
    pub fn cusolverDnCsytri_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                uplo: cublasFillMode_t,
                n: ::std::os::raw::c_int,
                A: *mut cuComplex,
                lda: ::std::os::raw::c_int,
                ipiv: *const ::std::os::raw::c_int,
                lwork: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnCsytri_bufferSize = val;
        self
    }
    pub fn cusolverDnZsytri_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                uplo: cublasFillMode_t,
                n: ::std::os::raw::c_int,
                A: *mut cuDoubleComplex,
                lda: ::std::os::raw::c_int,
                ipiv: *const ::std::os::raw::c_int,
                lwork: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnZsytri_bufferSize = val;
        self
    }
    pub fn cusolverDnSsytri(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                uplo: cublasFillMode_t,
                n: ::std::os::raw::c_int,
                A: *mut f32,
                lda: ::std::os::raw::c_int,
                ipiv: *const ::std::os::raw::c_int,
                work: *mut f32,
                lwork: ::std::os::raw::c_int,
                info: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnSsytri = val;
        self
    }
    pub fn cusolverDnDsytri(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                uplo: cublasFillMode_t,
                n: ::std::os::raw::c_int,
                A: *mut f64,
                lda: ::std::os::raw::c_int,
                ipiv: *const ::std::os::raw::c_int,
                work: *mut f64,
                lwork: ::std::os::raw::c_int,
                info: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnDsytri = val;
        self
    }
    pub fn cusolverDnCsytri(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                uplo: cublasFillMode_t,
                n: ::std::os::raw::c_int,
                A: *mut cuComplex,
                lda: ::std::os::raw::c_int,
                ipiv: *const ::std::os::raw::c_int,
                work: *mut cuComplex,
                lwork: ::std::os::raw::c_int,
                info: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnCsytri = val;
        self
    }
    pub fn cusolverDnZsytri(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnZsytri = val;
        self
    }
    pub fn cusolverDnSgebrd_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                Lwork: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnSgebrd_bufferSize = val;
        self
    }
    pub fn cusolverDnDgebrd_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                Lwork: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnDgebrd_bufferSize = val;
        self
    }
    pub fn cusolverDnCgebrd_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                Lwork: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnCgebrd_bufferSize = val;
        self
    }
    pub fn cusolverDnZgebrd_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                Lwork: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnZgebrd_bufferSize = val;
        self
    }
    pub fn cusolverDnSgebrd(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnSgebrd = val;
        self
    }
    pub fn cusolverDnDgebrd(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnDgebrd = val;
        self
    }
    pub fn cusolverDnCgebrd(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnCgebrd = val;
        self
    }
    pub fn cusolverDnZgebrd(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnZgebrd = val;
        self
    }
    pub fn cusolverDnSorgbr_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                side: cublasSideMode_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                k: ::std::os::raw::c_int,
                A: *const f32,
                lda: ::std::os::raw::c_int,
                tau: *const f32,
                lwork: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnSorgbr_bufferSize = val;
        self
    }
    pub fn cusolverDnDorgbr_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                side: cublasSideMode_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                k: ::std::os::raw::c_int,
                A: *const f64,
                lda: ::std::os::raw::c_int,
                tau: *const f64,
                lwork: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnDorgbr_bufferSize = val;
        self
    }
    pub fn cusolverDnCungbr_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                side: cublasSideMode_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                k: ::std::os::raw::c_int,
                A: *const cuComplex,
                lda: ::std::os::raw::c_int,
                tau: *const cuComplex,
                lwork: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnCungbr_bufferSize = val;
        self
    }
    pub fn cusolverDnZungbr_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                side: cublasSideMode_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                k: ::std::os::raw::c_int,
                A: *const cuDoubleComplex,
                lda: ::std::os::raw::c_int,
                tau: *const cuDoubleComplex,
                lwork: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnZungbr_bufferSize = val;
        self
    }
    pub fn cusolverDnSorgbr(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnSorgbr = val;
        self
    }
    pub fn cusolverDnDorgbr(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnDorgbr = val;
        self
    }
    pub fn cusolverDnCungbr(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnCungbr = val;
        self
    }
    pub fn cusolverDnZungbr(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnZungbr = val;
        self
    }
    pub fn cusolverDnSsytrd_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                uplo: cublasFillMode_t,
                n: ::std::os::raw::c_int,
                A: *const f32,
                lda: ::std::os::raw::c_int,
                d: *const f32,
                e: *const f32,
                tau: *const f32,
                lwork: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnSsytrd_bufferSize = val;
        self
    }
    pub fn cusolverDnDsytrd_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                uplo: cublasFillMode_t,
                n: ::std::os::raw::c_int,
                A: *const f64,
                lda: ::std::os::raw::c_int,
                d: *const f64,
                e: *const f64,
                tau: *const f64,
                lwork: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnDsytrd_bufferSize = val;
        self
    }
    pub fn cusolverDnChetrd_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                uplo: cublasFillMode_t,
                n: ::std::os::raw::c_int,
                A: *const cuComplex,
                lda: ::std::os::raw::c_int,
                d: *const f32,
                e: *const f32,
                tau: *const cuComplex,
                lwork: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnChetrd_bufferSize = val;
        self
    }
    pub fn cusolverDnZhetrd_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                uplo: cublasFillMode_t,
                n: ::std::os::raw::c_int,
                A: *const cuDoubleComplex,
                lda: ::std::os::raw::c_int,
                d: *const f64,
                e: *const f64,
                tau: *const cuDoubleComplex,
                lwork: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnZhetrd_bufferSize = val;
        self
    }
    pub fn cusolverDnSsytrd(
        mut self,
        val: Option<
            unsafe extern "C" fn(
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
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnSsytrd = val;
        self
    }
    pub fn cusolverDnDsytrd(
        mut self,
        val: Option<
            unsafe extern "C" fn(
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
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnDsytrd = val;
        self
    }
    pub fn cusolverDnChetrd(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnChetrd = val;
        self
    }
    pub fn cusolverDnZhetrd(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnZhetrd = val;
        self
    }
    pub fn cusolverDnSorgtr_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                uplo: cublasFillMode_t,
                n: ::std::os::raw::c_int,
                A: *const f32,
                lda: ::std::os::raw::c_int,
                tau: *const f32,
                lwork: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnSorgtr_bufferSize = val;
        self
    }
    pub fn cusolverDnDorgtr_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                uplo: cublasFillMode_t,
                n: ::std::os::raw::c_int,
                A: *const f64,
                lda: ::std::os::raw::c_int,
                tau: *const f64,
                lwork: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnDorgtr_bufferSize = val;
        self
    }
    pub fn cusolverDnCungtr_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                uplo: cublasFillMode_t,
                n: ::std::os::raw::c_int,
                A: *const cuComplex,
                lda: ::std::os::raw::c_int,
                tau: *const cuComplex,
                lwork: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnCungtr_bufferSize = val;
        self
    }
    pub fn cusolverDnZungtr_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                uplo: cublasFillMode_t,
                n: ::std::os::raw::c_int,
                A: *const cuDoubleComplex,
                lda: ::std::os::raw::c_int,
                tau: *const cuDoubleComplex,
                lwork: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnZungtr_bufferSize = val;
        self
    }
    pub fn cusolverDnSorgtr(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                uplo: cublasFillMode_t,
                n: ::std::os::raw::c_int,
                A: *mut f32,
                lda: ::std::os::raw::c_int,
                tau: *const f32,
                work: *mut f32,
                lwork: ::std::os::raw::c_int,
                info: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnSorgtr = val;
        self
    }
    pub fn cusolverDnDorgtr(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                uplo: cublasFillMode_t,
                n: ::std::os::raw::c_int,
                A: *mut f64,
                lda: ::std::os::raw::c_int,
                tau: *const f64,
                work: *mut f64,
                lwork: ::std::os::raw::c_int,
                info: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnDorgtr = val;
        self
    }
    pub fn cusolverDnCungtr(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                uplo: cublasFillMode_t,
                n: ::std::os::raw::c_int,
                A: *mut cuComplex,
                lda: ::std::os::raw::c_int,
                tau: *const cuComplex,
                work: *mut cuComplex,
                lwork: ::std::os::raw::c_int,
                info: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnCungtr = val;
        self
    }
    pub fn cusolverDnZungtr(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnZungtr = val;
        self
    }
    pub fn cusolverDnSormtr_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnSormtr_bufferSize = val;
        self
    }
    pub fn cusolverDnDormtr_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnDormtr_bufferSize = val;
        self
    }
    pub fn cusolverDnCunmtr_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnCunmtr_bufferSize = val;
        self
    }
    pub fn cusolverDnZunmtr_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnZunmtr_bufferSize = val;
        self
    }
    pub fn cusolverDnSormtr(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnSormtr = val;
        self
    }
    pub fn cusolverDnDormtr(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnDormtr = val;
        self
    }
    pub fn cusolverDnCunmtr(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnCunmtr = val;
        self
    }
    pub fn cusolverDnZunmtr(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnZunmtr = val;
        self
    }
    pub fn cusolverDnSgesvd_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                lwork: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnSgesvd_bufferSize = val;
        self
    }
    pub fn cusolverDnDgesvd_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                lwork: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnDgesvd_bufferSize = val;
        self
    }
    pub fn cusolverDnCgesvd_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                lwork: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnCgesvd_bufferSize = val;
        self
    }
    pub fn cusolverDnZgesvd_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                lwork: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnZgesvd_bufferSize = val;
        self
    }
    pub fn cusolverDnSgesvd(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnSgesvd = val;
        self
    }
    pub fn cusolverDnDgesvd(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnDgesvd = val;
        self
    }
    pub fn cusolverDnCgesvd(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnCgesvd = val;
        self
    }
    pub fn cusolverDnZgesvd(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnZgesvd = val;
        self
    }
    pub fn cusolverDnSsyevd_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                jobz: cusolverEigMode_t,
                uplo: cublasFillMode_t,
                n: ::std::os::raw::c_int,
                A: *const f32,
                lda: ::std::os::raw::c_int,
                W: *const f32,
                lwork: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnSsyevd_bufferSize = val;
        self
    }
    pub fn cusolverDnDsyevd_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                jobz: cusolverEigMode_t,
                uplo: cublasFillMode_t,
                n: ::std::os::raw::c_int,
                A: *const f64,
                lda: ::std::os::raw::c_int,
                W: *const f64,
                lwork: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnDsyevd_bufferSize = val;
        self
    }
    pub fn cusolverDnCheevd_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                jobz: cusolverEigMode_t,
                uplo: cublasFillMode_t,
                n: ::std::os::raw::c_int,
                A: *const cuComplex,
                lda: ::std::os::raw::c_int,
                W: *const f32,
                lwork: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnCheevd_bufferSize = val;
        self
    }
    pub fn cusolverDnZheevd_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                jobz: cusolverEigMode_t,
                uplo: cublasFillMode_t,
                n: ::std::os::raw::c_int,
                A: *const cuDoubleComplex,
                lda: ::std::os::raw::c_int,
                W: *const f64,
                lwork: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnZheevd_bufferSize = val;
        self
    }
    pub fn cusolverDnSsyevd(
        mut self,
        val: Option<
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
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnSsyevd = val;
        self
    }
    pub fn cusolverDnDsyevd(
        mut self,
        val: Option<
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
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnDsyevd = val;
        self
    }
    pub fn cusolverDnCheevd(
        mut self,
        val: Option<
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
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnCheevd = val;
        self
    }
    pub fn cusolverDnZheevd(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnZheevd = val;
        self
    }
    pub fn cusolverDnSsyevdx_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnSsyevdx_bufferSize = val;
        self
    }
    pub fn cusolverDnDsyevdx_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnDsyevdx_bufferSize = val;
        self
    }
    pub fn cusolverDnCheevdx_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnCheevdx_bufferSize = val;
        self
    }
    pub fn cusolverDnZheevdx_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnZheevdx_bufferSize = val;
        self
    }
    pub fn cusolverDnSsyevdx(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnSsyevdx = val;
        self
    }
    pub fn cusolverDnDsyevdx(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnDsyevdx = val;
        self
    }
    pub fn cusolverDnCheevdx(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnCheevdx = val;
        self
    }
    pub fn cusolverDnZheevdx(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnZheevdx = val;
        self
    }
    pub fn cusolverDnSsygvdx_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnSsygvdx_bufferSize = val;
        self
    }
    pub fn cusolverDnDsygvdx_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnDsygvdx_bufferSize = val;
        self
    }
    pub fn cusolverDnChegvdx_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnChegvdx_bufferSize = val;
        self
    }
    pub fn cusolverDnZhegvdx_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnZhegvdx_bufferSize = val;
        self
    }
    pub fn cusolverDnSsygvdx(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnSsygvdx = val;
        self
    }
    pub fn cusolverDnDsygvdx(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnDsygvdx = val;
        self
    }
    pub fn cusolverDnChegvdx(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnChegvdx = val;
        self
    }
    pub fn cusolverDnZhegvdx(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnZhegvdx = val;
        self
    }
    pub fn cusolverDnSsygvd_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnSsygvd_bufferSize = val;
        self
    }
    pub fn cusolverDnDsygvd_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnDsygvd_bufferSize = val;
        self
    }
    pub fn cusolverDnChegvd_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnChegvd_bufferSize = val;
        self
    }
    pub fn cusolverDnZhegvd_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnZhegvd_bufferSize = val;
        self
    }
    pub fn cusolverDnSsygvd(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnSsygvd = val;
        self
    }
    pub fn cusolverDnDsygvd(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnDsygvd = val;
        self
    }
    pub fn cusolverDnChegvd(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnChegvd = val;
        self
    }
    pub fn cusolverDnZhegvd(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnZhegvd = val;
        self
    }
    pub fn cusolverDnXsygvd_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnXsygvd_bufferSize = val;
        self
    }
    pub fn cusolverDnXsygvd(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnXsygvd = val;
        self
    }
    pub fn cusolverDnXsygvdx_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnXsygvdx_bufferSize = val;
        self
    }
    pub fn cusolverDnXsygvdx(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnXsygvdx = val;
        self
    }
    pub fn cusolverDnCreateSyevjInfo(
        mut self,
        val: Option<unsafe extern "C" fn(info: *mut syevjInfo_t) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnCreateSyevjInfo = val;
        self
    }
    pub fn cusolverDnDestroySyevjInfo(
        mut self,
        val: Option<unsafe extern "C" fn(info: syevjInfo_t) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnDestroySyevjInfo = val;
        self
    }
    pub fn cusolverDnXsyevjSetTolerance(
        mut self,
        val: Option<unsafe extern "C" fn(info: syevjInfo_t, tolerance: f64) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnXsyevjSetTolerance = val;
        self
    }
    pub fn cusolverDnXsyevjSetMaxSweeps(
        mut self,
        val: Option<unsafe extern "C" fn(info: syevjInfo_t, max_sweeps: ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnXsyevjSetMaxSweeps = val;
        self
    }
    pub fn cusolverDnXsyevjSetSortEig(
        mut self,
        val: Option<unsafe extern "C" fn(info: syevjInfo_t, sort_eig: ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnXsyevjSetSortEig = val;
        self
    }
    pub fn cusolverDnXsyevjGetResidual(
        mut self,
        val: Option<
            unsafe extern "C" fn(handle: cusolverDnHandle_t, info: syevjInfo_t, residual: *mut f64) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnXsyevjGetResidual = val;
        self
    }
    pub fn cusolverDnXsyevjGetSweeps(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                info: syevjInfo_t,
                executed_sweeps: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnXsyevjGetSweeps = val;
        self
    }
    pub fn cusolverDnSsyevjBatched_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
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
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnSsyevjBatched_bufferSize = val;
        self
    }
    pub fn cusolverDnDsyevjBatched_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
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
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnDsyevjBatched_bufferSize = val;
        self
    }
    pub fn cusolverDnCheevjBatched_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnCheevjBatched_bufferSize = val;
        self
    }
    pub fn cusolverDnZheevjBatched_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnZheevjBatched_bufferSize = val;
        self
    }
    pub fn cusolverDnSsyevjBatched(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnSsyevjBatched = val;
        self
    }
    pub fn cusolverDnDsyevjBatched(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnDsyevjBatched = val;
        self
    }
    pub fn cusolverDnCheevjBatched(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnCheevjBatched = val;
        self
    }
    pub fn cusolverDnZheevjBatched(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnZheevjBatched = val;
        self
    }
    pub fn cusolverDnSsyevj_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                jobz: cusolverEigMode_t,
                uplo: cublasFillMode_t,
                n: ::std::os::raw::c_int,
                A: *const f32,
                lda: ::std::os::raw::c_int,
                W: *const f32,
                lwork: *mut ::std::os::raw::c_int,
                params: syevjInfo_t,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnSsyevj_bufferSize = val;
        self
    }
    pub fn cusolverDnDsyevj_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                jobz: cusolverEigMode_t,
                uplo: cublasFillMode_t,
                n: ::std::os::raw::c_int,
                A: *const f64,
                lda: ::std::os::raw::c_int,
                W: *const f64,
                lwork: *mut ::std::os::raw::c_int,
                params: syevjInfo_t,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnDsyevj_bufferSize = val;
        self
    }
    pub fn cusolverDnCheevj_bufferSize(
        mut self,
        val: Option<
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
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnCheevj_bufferSize = val;
        self
    }
    pub fn cusolverDnZheevj_bufferSize(
        mut self,
        val: Option<
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
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnZheevj_bufferSize = val;
        self
    }
    pub fn cusolverDnSsyevj(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnSsyevj = val;
        self
    }
    pub fn cusolverDnDsyevj(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnDsyevj = val;
        self
    }
    pub fn cusolverDnCheevj(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnCheevj = val;
        self
    }
    pub fn cusolverDnZheevj(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnZheevj = val;
        self
    }
    pub fn cusolverDnSsygvj_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnSsygvj_bufferSize = val;
        self
    }
    pub fn cusolverDnDsygvj_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnDsygvj_bufferSize = val;
        self
    }
    pub fn cusolverDnChegvj_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnChegvj_bufferSize = val;
        self
    }
    pub fn cusolverDnZhegvj_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnZhegvj_bufferSize = val;
        self
    }
    pub fn cusolverDnSsygvj(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnSsygvj = val;
        self
    }
    pub fn cusolverDnDsygvj(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnDsygvj = val;
        self
    }
    pub fn cusolverDnChegvj(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnChegvj = val;
        self
    }
    pub fn cusolverDnZhegvj(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnZhegvj = val;
        self
    }
    pub fn cusolverDnCreateGesvdjInfo(
        mut self,
        val: Option<unsafe extern "C" fn(info: *mut gesvdjInfo_t) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnCreateGesvdjInfo = val;
        self
    }
    pub fn cusolverDnDestroyGesvdjInfo(
        mut self,
        val: Option<unsafe extern "C" fn(info: gesvdjInfo_t) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnDestroyGesvdjInfo = val;
        self
    }
    pub fn cusolverDnXgesvdjSetTolerance(
        mut self,
        val: Option<unsafe extern "C" fn(info: gesvdjInfo_t, tolerance: f64) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnXgesvdjSetTolerance = val;
        self
    }
    pub fn cusolverDnXgesvdjSetMaxSweeps(
        mut self,
        val: Option<unsafe extern "C" fn(info: gesvdjInfo_t, max_sweeps: ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnXgesvdjSetMaxSweeps = val;
        self
    }
    pub fn cusolverDnXgesvdjSetSortEig(
        mut self,
        val: Option<unsafe extern "C" fn(info: gesvdjInfo_t, sort_svd: ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnXgesvdjSetSortEig = val;
        self
    }
    pub fn cusolverDnXgesvdjGetResidual(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                info: gesvdjInfo_t,
                residual: *mut f64,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnXgesvdjGetResidual = val;
        self
    }
    pub fn cusolverDnXgesvdjGetSweeps(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverDnHandle_t,
                info: gesvdjInfo_t,
                executed_sweeps: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnXgesvdjGetSweeps = val;
        self
    }
    pub fn cusolverDnSgesvdjBatched_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnSgesvdjBatched_bufferSize = val;
        self
    }
    pub fn cusolverDnDgesvdjBatched_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnDgesvdjBatched_bufferSize = val;
        self
    }
    pub fn cusolverDnCgesvdjBatched_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnCgesvdjBatched_bufferSize = val;
        self
    }
    pub fn cusolverDnZgesvdjBatched_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnZgesvdjBatched_bufferSize = val;
        self
    }
    pub fn cusolverDnSgesvdjBatched(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnSgesvdjBatched = val;
        self
    }
    pub fn cusolverDnDgesvdjBatched(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnDgesvdjBatched = val;
        self
    }
    pub fn cusolverDnCgesvdjBatched(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnCgesvdjBatched = val;
        self
    }
    pub fn cusolverDnZgesvdjBatched(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnZgesvdjBatched = val;
        self
    }
    pub fn cusolverDnSgesvdj_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnSgesvdj_bufferSize = val;
        self
    }
    pub fn cusolverDnDgesvdj_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnDgesvdj_bufferSize = val;
        self
    }
    pub fn cusolverDnCgesvdj_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnCgesvdj_bufferSize = val;
        self
    }
    pub fn cusolverDnZgesvdj_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnZgesvdj_bufferSize = val;
        self
    }
    pub fn cusolverDnSgesvdj(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnSgesvdj = val;
        self
    }
    pub fn cusolverDnDgesvdj(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnDgesvdj = val;
        self
    }
    pub fn cusolverDnCgesvdj(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnCgesvdj = val;
        self
    }
    pub fn cusolverDnZgesvdj(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnZgesvdj = val;
        self
    }
    pub fn cusolverDnSgesvdaStridedBatched_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnSgesvdaStridedBatched_bufferSize = val;
        self
    }
    pub fn cusolverDnDgesvdaStridedBatched_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnDgesvdaStridedBatched_bufferSize = val;
        self
    }
    pub fn cusolverDnCgesvdaStridedBatched_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnCgesvdaStridedBatched_bufferSize = val;
        self
    }
    pub fn cusolverDnZgesvdaStridedBatched_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnZgesvdaStridedBatched_bufferSize = val;
        self
    }
    pub fn cusolverDnSgesvdaStridedBatched(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnSgesvdaStridedBatched = val;
        self
    }
    pub fn cusolverDnDgesvdaStridedBatched(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnDgesvdaStridedBatched = val;
        self
    }
    pub fn cusolverDnCgesvdaStridedBatched(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnCgesvdaStridedBatched = val;
        self
    }
    pub fn cusolverDnZgesvdaStridedBatched(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnZgesvdaStridedBatched = val;
        self
    }
    pub fn cusolverDnCreateParams(
        mut self,
        val: Option<unsafe extern "C" fn(params: *mut cusolverDnParams_t) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnCreateParams = val;
        self
    }
    pub fn cusolverDnDestroyParams(
        mut self,
        val: Option<unsafe extern "C" fn(params: cusolverDnParams_t) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnDestroyParams = val;
        self
    }
    pub fn cusolverDnSetAdvOptions(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                params: cusolverDnParams_t,
                function: cusolverDnFunction_t,
                algo: cusolverAlgMode_t,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnSetAdvOptions = val;
        self
    }
    pub fn cusolverDnXpotrf_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnXpotrf_bufferSize = val;
        self
    }
    pub fn cusolverDnXpotrf(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnXpotrf = val;
        self
    }
    pub fn cusolverDnXpotrs(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnXpotrs = val;
        self
    }
    pub fn cusolverDnXgeqrf_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnXgeqrf_bufferSize = val;
        self
    }
    pub fn cusolverDnXgeqrf(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnXgeqrf = val;
        self
    }
    pub fn cusolverDnXgetrf_bufferSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
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
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverDnXgetrf_bufferSize = val;
        self
    }
    pub fn cusolverDnXgetrf(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnXgetrf = val;
        self
    }
    pub fn cusolverDnXgetrs(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnXgetrs = val;
        self
    }
    pub fn cusolverDnXsyevd_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnXsyevd_bufferSize = val;
        self
    }
    pub fn cusolverDnXsyevd(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnXsyevd = val;
        self
    }
    pub fn cusolverDnXstedc_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnXstedc_bufferSize = val;
        self
    }
    pub fn cusolverDnXstedc(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnXstedc = val;
        self
    }
    pub fn cusolverDnXsyevBatched_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnXsyevBatched_bufferSize = val;
        self
    }
    pub fn cusolverDnXsyevBatched(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnXsyevBatched = val;
        self
    }
    pub fn cusolverDnXsyevdx_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnXsyevdx_bufferSize = val;
        self
    }
    pub fn cusolverDnXsyevdx(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnXsyevdx = val;
        self
    }
    pub fn cusolverDnXgeev_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnXgeev_bufferSize = val;
        self
    }
    pub fn cusolverDnXgeev(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnXgeev = val;
        self
    }
    pub fn cusolverDnXgesvd_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnXgesvd_bufferSize = val;
        self
    }
    pub fn cusolverDnXgesvd(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnXgesvd = val;
        self
    }
    pub fn cusolverDnXgesvdp_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnXgesvdp_bufferSize = val;
        self
    }
    pub fn cusolverDnXgesvdp(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnXgesvdp = val;
        self
    }
    pub fn cusolverDnXgesvdr_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnXgesvdr_bufferSize = val;
        self
    }
    pub fn cusolverDnXgesvdr(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnXgesvdr = val;
        self
    }
    pub fn cusolverDnXlarft_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnXlarft_bufferSize = val;
        self
    }
    pub fn cusolverDnXlarft(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnXlarft = val;
        self
    }
    pub fn cusolverDnLoggerSetCallback(
        mut self,
        val: Option<unsafe extern "C" fn(callback: cusolverDnLoggerCallback_t) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnLoggerSetCallback = val;
        self
    }
    pub fn cusolverDnLoggerSetFile(
        mut self,
        val: Option<unsafe extern "C" fn(file: *mut FILE) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnLoggerSetFile = val;
        self
    }
    pub fn cusolverDnLoggerOpenFile(
        mut self,
        val: Option<unsafe extern "C" fn(logFile: *const ::std::os::raw::c_char) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnLoggerOpenFile = val;
        self
    }
    pub fn cusolverDnLoggerSetLevel(
        mut self,
        val: Option<unsafe extern "C" fn(level: ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnLoggerSetLevel = val;
        self
    }
    pub fn cusolverDnLoggerSetMask(
        mut self,
        val: Option<unsafe extern "C" fn(mask: ::std::os::raw::c_int) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverDnLoggerSetMask = val;
        self
    }
    pub fn cusolverDnLoggerForceDisable(mut self, val: Option<unsafe extern "C" fn() -> cusolverStatus_t>) -> Self {
        self.cusolverDnLoggerForceDisable = val;
        self
    }
    pub fn cusolverDnXpolar_bufferSize(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnXpolar_bufferSize = val;
        self
    }
    pub fn cusolverDnXpolar(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverDnXpolar = val;
        self
    }
    pub fn cusolverSpCreate(
        mut self,
        val: Option<unsafe extern "C" fn(handle: *mut cusolverSpHandle_t) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverSpCreate = val;
        self
    }
    pub fn cusolverSpDestroy(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cusolverSpHandle_t) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverSpDestroy = val;
        self
    }
    pub fn cusolverSpSetStream(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cusolverSpHandle_t, streamId: cudaStream_t) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverSpSetStream = val;
        self
    }
    pub fn cusolverSpGetStream(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cusolverSpHandle_t, streamId: *mut cudaStream_t) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverSpGetStream = val;
        self
    }
    pub fn cusolverSpXcsrissymHost(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverSpXcsrissymHost = val;
        self
    }
    pub fn cusolverSpScsrlsvluHost(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverSpScsrlsvluHost = val;
        self
    }
    pub fn cusolverSpDcsrlsvluHost(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverSpDcsrlsvluHost = val;
        self
    }
    pub fn cusolverSpCcsrlsvluHost(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverSpCcsrlsvluHost = val;
        self
    }
    pub fn cusolverSpZcsrlsvluHost(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverSpZcsrlsvluHost = val;
        self
    }
    pub fn cusolverSpScsrlsvqr(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverSpScsrlsvqr = val;
        self
    }
    pub fn cusolverSpDcsrlsvqr(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverSpDcsrlsvqr = val;
        self
    }
    pub fn cusolverSpCcsrlsvqr(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverSpCcsrlsvqr = val;
        self
    }
    pub fn cusolverSpZcsrlsvqr(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverSpZcsrlsvqr = val;
        self
    }
    pub fn cusolverSpScsrlsvqrHost(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverSpScsrlsvqrHost = val;
        self
    }
    pub fn cusolverSpDcsrlsvqrHost(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverSpDcsrlsvqrHost = val;
        self
    }
    pub fn cusolverSpCcsrlsvqrHost(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverSpCcsrlsvqrHost = val;
        self
    }
    pub fn cusolverSpZcsrlsvqrHost(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverSpZcsrlsvqrHost = val;
        self
    }
    pub fn cusolverSpScsrlsvcholHost(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverSpScsrlsvcholHost = val;
        self
    }
    pub fn cusolverSpDcsrlsvcholHost(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverSpDcsrlsvcholHost = val;
        self
    }
    pub fn cusolverSpCcsrlsvcholHost(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverSpCcsrlsvcholHost = val;
        self
    }
    pub fn cusolverSpZcsrlsvcholHost(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverSpZcsrlsvcholHost = val;
        self
    }
    pub fn cusolverSpScsrlsvchol(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverSpScsrlsvchol = val;
        self
    }
    pub fn cusolverSpDcsrlsvchol(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverSpDcsrlsvchol = val;
        self
    }
    pub fn cusolverSpCcsrlsvchol(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverSpCcsrlsvchol = val;
        self
    }
    pub fn cusolverSpZcsrlsvchol(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverSpZcsrlsvchol = val;
        self
    }
    pub fn cusolverSpScsrlsqvqrHost(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverSpScsrlsqvqrHost = val;
        self
    }
    pub fn cusolverSpDcsrlsqvqrHost(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverSpDcsrlsqvqrHost = val;
        self
    }
    pub fn cusolverSpCcsrlsqvqrHost(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverSpCcsrlsqvqrHost = val;
        self
    }
    pub fn cusolverSpZcsrlsqvqrHost(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverSpZcsrlsqvqrHost = val;
        self
    }
    pub fn cusolverSpScsreigvsiHost(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverSpScsreigvsiHost = val;
        self
    }
    pub fn cusolverSpDcsreigvsiHost(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverSpDcsreigvsiHost = val;
        self
    }
    pub fn cusolverSpCcsreigvsiHost(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverSpCcsreigvsiHost = val;
        self
    }
    pub fn cusolverSpZcsreigvsiHost(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverSpZcsreigvsiHost = val;
        self
    }
    pub fn cusolverSpScsreigvsi(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverSpScsreigvsi = val;
        self
    }
    pub fn cusolverSpDcsreigvsi(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverSpDcsreigvsi = val;
        self
    }
    pub fn cusolverSpCcsreigvsi(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverSpCcsreigvsi = val;
        self
    }
    pub fn cusolverSpZcsreigvsi(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverSpZcsreigvsi = val;
        self
    }
    pub fn cusolverSpScsreigsHost(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverSpScsreigsHost = val;
        self
    }
    pub fn cusolverSpDcsreigsHost(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverSpDcsreigsHost = val;
        self
    }
    pub fn cusolverSpCcsreigsHost(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverSpCcsreigsHost = val;
        self
    }
    pub fn cusolverSpZcsreigsHost(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverSpZcsreigsHost = val;
        self
    }
    pub fn cusolverSpXcsrsymrcmHost(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverSpHandle_t,
                n: ::std::os::raw::c_int,
                nnzA: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrRowPtrA: *const ::std::os::raw::c_int,
                csrColIndA: *const ::std::os::raw::c_int,
                p: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverSpXcsrsymrcmHost = val;
        self
    }
    pub fn cusolverSpXcsrsymmdqHost(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverSpHandle_t,
                n: ::std::os::raw::c_int,
                nnzA: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrRowPtrA: *const ::std::os::raw::c_int,
                csrColIndA: *const ::std::os::raw::c_int,
                p: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverSpXcsrsymmdqHost = val;
        self
    }
    pub fn cusolverSpXcsrsymamdHost(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverSpHandle_t,
                n: ::std::os::raw::c_int,
                nnzA: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrRowPtrA: *const ::std::os::raw::c_int,
                csrColIndA: *const ::std::os::raw::c_int,
                p: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverSpXcsrsymamdHost = val;
        self
    }
    pub fn cusolverSpXcsrmetisndHost(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverSpHandle_t,
                n: ::std::os::raw::c_int,
                nnzA: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrRowPtrA: *const ::std::os::raw::c_int,
                csrColIndA: *const ::std::os::raw::c_int,
                options: *const i64,
                p: *mut ::std::os::raw::c_int,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverSpXcsrmetisndHost = val;
        self
    }
    pub fn cusolverSpScsrzfdHost(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverSpScsrzfdHost = val;
        self
    }
    pub fn cusolverSpDcsrzfdHost(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverSpDcsrzfdHost = val;
        self
    }
    pub fn cusolverSpCcsrzfdHost(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverSpCcsrzfdHost = val;
        self
    }
    pub fn cusolverSpZcsrzfdHost(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverSpZcsrzfdHost = val;
        self
    }
    pub fn cusolverSpXcsrperm_bufferSizeHost(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverSpXcsrperm_bufferSizeHost = val;
        self
    }
    pub fn cusolverSpXcsrpermHost(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverSpXcsrpermHost = val;
        self
    }
    pub fn cusolverSpCreateCsrqrInfo(
        mut self,
        val: Option<unsafe extern "C" fn(info: *mut csrqrInfo_t) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverSpCreateCsrqrInfo = val;
        self
    }
    pub fn cusolverSpDestroyCsrqrInfo(
        mut self,
        val: Option<unsafe extern "C" fn(info: csrqrInfo_t) -> cusolverStatus_t>,
    ) -> Self {
        self.cusolverSpDestroyCsrqrInfo = val;
        self
    }
    pub fn cusolverSpXcsrqrAnalysisBatched(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cusolverSpHandle_t,
                m: ::std::os::raw::c_int,
                n: ::std::os::raw::c_int,
                nnzA: ::std::os::raw::c_int,
                descrA: cusparseMatDescr_t,
                csrRowPtrA: *const ::std::os::raw::c_int,
                csrColIndA: *const ::std::os::raw::c_int,
                info: csrqrInfo_t,
            ) -> cusolverStatus_t,
        >,
    ) -> Self {
        self.cusolverSpXcsrqrAnalysisBatched = val;
        self
    }
    pub fn cusolverSpScsrqrBufferInfoBatched(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverSpScsrqrBufferInfoBatched = val;
        self
    }
    pub fn cusolverSpDcsrqrBufferInfoBatched(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverSpDcsrqrBufferInfoBatched = val;
        self
    }
    pub fn cusolverSpCcsrqrBufferInfoBatched(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverSpCcsrqrBufferInfoBatched = val;
        self
    }
    pub fn cusolverSpZcsrqrBufferInfoBatched(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverSpZcsrqrBufferInfoBatched = val;
        self
    }
    pub fn cusolverSpScsrqrsvBatched(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverSpScsrqrsvBatched = val;
        self
    }
    pub fn cusolverSpDcsrqrsvBatched(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverSpDcsrqrsvBatched = val;
        self
    }
    pub fn cusolverSpCcsrqrsvBatched(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverSpCcsrqrsvBatched = val;
        self
    }
    pub fn cusolverSpZcsrqrsvBatched(
        mut self,
        val: Option<
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
    ) -> Self {
        self.cusolverSpZcsrqrsvBatched = val;
        self
    }
}
pub unsafe fn cusolverGetProperty(type_: libraryPropertyType) -> Result<i32, crate::sys::cusolverStatus_t> {
    let mut out_1: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cusolverGetProperty(type_, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as i32) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusolverGetVersion() -> Result<i32, crate::sys::cusolverStatus_t> {
    let mut out_0: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cusolverGetVersion(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as i32) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusolverDnCreate() -> Result<cusolverDnHandle_t, crate::sys::cusolverStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cusolverDnHandle_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cusolverDnCreate(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cusolverDnHandle_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusolverDnDestroy(handle: cusolverDnHandle_t) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnDestroy(handle) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnSetStream(
    handle: cusolverDnHandle_t,
    streamId: cudaStream_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnSetStream(handle, streamId) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnGetStream(handle: cusolverDnHandle_t) -> Result<cudaStream_t, crate::sys::cusolverStatus_t> {
    let mut out_1: std::mem::MaybeUninit<cudaStream_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cusolverDnGetStream(handle, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as cudaStream_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusolverDnSetDeterministicMode(
    handle: cusolverDnHandle_t,
    mode: cusolverDeterministicMode_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnSetDeterministicMode(handle, mode) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnGetDeterministicMode(
    handle: cusolverDnHandle_t,
) -> Result<cusolverDeterministicMode_t, crate::sys::cusolverStatus_t> {
    let mut out_1: std::mem::MaybeUninit<cusolverDeterministicMode_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cusolverDnGetDeterministicMode(handle, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as cusolverDeterministicMode_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusolverDnSetMathMode(
    handle: cusolverDnHandle_t,
    mode: cusolverMathMode_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnSetMathMode(handle, mode) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnGetMathMode(
    handle: cusolverDnHandle_t,
) -> Result<cusolverMathMode_t, crate::sys::cusolverStatus_t> {
    let mut out_1: std::mem::MaybeUninit<cusolverMathMode_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cusolverDnGetMathMode(handle, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as cusolverMathMode_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusolverDnSetEmulationStrategy(
    handle: cusolverDnHandle_t,
    strategy: cudaEmulationStrategy_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnSetEmulationStrategy(handle, strategy) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnGetEmulationStrategy(
    handle: cusolverDnHandle_t,
) -> Result<cudaEmulationStrategy_t, crate::sys::cusolverStatus_t> {
    let mut out_1: std::mem::MaybeUninit<cudaEmulationStrategy_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cusolverDnGetEmulationStrategy(handle, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as cudaEmulationStrategy_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusolverDnSetFixedPointEmulationMantissaControl(
    handle: cusolverDnHandle_t,
    control: cudaEmulationMantissaControl_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnSetFixedPointEmulationMantissaControl(handle, control) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnGetFixedPointEmulationMantissaControl(
    handle: cusolverDnHandle_t,
) -> Result<cudaEmulationMantissaControl_t, crate::sys::cusolverStatus_t> {
    let mut out_1: std::mem::MaybeUninit<cudaEmulationMantissaControl_t> = std::mem::MaybeUninit::uninit();
    let status =
        unsafe { crate::sys::cusolverDnGetFixedPointEmulationMantissaControl(handle, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as cudaEmulationMantissaControl_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusolverDnSetFixedPointEmulationMaxMantissaBitCount(
    handle: cusolverDnHandle_t,
    mantissaBitCount: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status =
        unsafe { crate::sys::cusolverDnSetFixedPointEmulationMaxMantissaBitCount(handle, mantissaBitCount as _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnGetFixedPointEmulationMaxMantissaBitCount(
    handle: cusolverDnHandle_t,
) -> Result<i32, crate::sys::cusolverStatus_t> {
    let mut out_1: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cusolverDnGetFixedPointEmulationMaxMantissaBitCount(handle, out_1.as_mut_ptr() as *mut _)
    };
    if status as usize == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as i32) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusolverDnSetFixedPointEmulationMantissaBitOffset(
    handle: cusolverDnHandle_t,
    mantissaBitOffset: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status =
        unsafe { crate::sys::cusolverDnSetFixedPointEmulationMantissaBitOffset(handle, mantissaBitOffset as _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnGetFixedPointEmulationMantissaBitOffset(
    handle: cusolverDnHandle_t,
) -> Result<i32, crate::sys::cusolverStatus_t> {
    let mut out_1: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status =
        unsafe { crate::sys::cusolverDnGetFixedPointEmulationMantissaBitOffset(handle, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as i32) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusolverDnSetEmulationSpecialValuesSupport(
    handle: cusolverDnHandle_t,
    mask: cudaEmulationSpecialValuesSupport_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnSetEmulationSpecialValuesSupport(handle, mask) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnGetEmulationSpecialValuesSupport(
    handle: cusolverDnHandle_t,
) -> Result<cudaEmulationSpecialValuesSupport_t, crate::sys::cusolverStatus_t> {
    let mut out_1: std::mem::MaybeUninit<cudaEmulationSpecialValuesSupport_t> = std::mem::MaybeUninit::uninit();
    let status =
        unsafe { crate::sys::cusolverDnGetEmulationSpecialValuesSupport(handle, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as cudaEmulationSpecialValuesSupport_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusolverDnIRSParamsCreate() -> Result<cusolverDnIRSParams_t, crate::sys::cusolverStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cusolverDnIRSParams_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cusolverDnIRSParamsCreate(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cusolverDnIRSParams_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusolverDnIRSParamsDestroy(params: cusolverDnIRSParams_t) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnIRSParamsDestroy(params) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnIRSParamsSetRefinementSolver(
    params: cusolverDnIRSParams_t,
    refinement_solver: cusolverIRSRefinement_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnIRSParamsSetRefinementSolver(params, refinement_solver) };
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
    let status = unsafe { crate::sys::cusolverDnIRSParamsSetSolverMainPrecision(params, solver_main_precision) };
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
    let status = unsafe { crate::sys::cusolverDnIRSParamsSetSolverLowestPrecision(params, solver_lowest_precision) };
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
        crate::sys::cusolverDnIRSParamsSetSolverPrecisions(params, solver_main_precision, solver_lowest_precision)
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
    let status = unsafe { crate::sys::cusolverDnIRSParamsGetMaxIters(params, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as cusolver_int_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
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
pub unsafe fn cusolverDnIRSInfosDestroy(infos: cusolverDnIRSInfos_t) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnIRSInfosDestroy(infos) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnIRSInfosCreate() -> Result<cusolverDnIRSInfos_t, crate::sys::cusolverStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cusolverDnIRSInfos_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cusolverDnIRSInfosCreate(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cusolverDnIRSInfos_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusolverDnIRSInfosGetNiters(
    infos: cusolverDnIRSInfos_t,
) -> Result<cusolver_int_t, crate::sys::cusolverStatus_t> {
    let mut out_1: std::mem::MaybeUninit<cusolver_int_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cusolverDnIRSInfosGetNiters(infos, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as cusolver_int_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusolverDnIRSInfosGetOuterNiters(
    infos: cusolverDnIRSInfos_t,
) -> Result<cusolver_int_t, crate::sys::cusolverStatus_t> {
    let mut out_1: std::mem::MaybeUninit<cusolver_int_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cusolverDnIRSInfosGetOuterNiters(infos, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as cusolver_int_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
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
pub unsafe fn cusolverDnIRSInfosGetResidualHistory<T: types::CudaAsPtr>(
    infos: cusolverDnIRSInfos_t,
    mut residual_history: T,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status =
        unsafe { crate::sys::cusolverDnIRSInfosGetResidualHistory(infos, residual_history.as_mut_ptr() as *mut _) };
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
    let status = unsafe { crate::sys::cusolverDnIRSInfosGetMaxIters(infos, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as cusolver_int_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusolverDnZZgesv<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZCgesv<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZKgesv<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZEgesv<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZYgesv<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnCCgesv<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnCEgesv<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnCKgesv<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnCYgesv<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDDgesv<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDSgesv<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDHgesv<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDBgesv<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDXgesv<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnSSgesv<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnSHgesv<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnSBgesv<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnSXgesv<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZZgesv_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZCgesv_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZKgesv_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZEgesv_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZYgesv_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnCCgesv_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnCKgesv_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnCEgesv_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnCYgesv_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDDgesv_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDSgesv_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDHgesv_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDBgesv_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDXgesv_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnSSgesv_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnSHgesv_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnSBgesv_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnSXgesv_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dipiv.as_mut_ptr() as *mut _,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZZgels<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZCgels<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZKgels<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZEgels<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZYgels<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnCCgels<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnCKgels<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnCEgels<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnCYgels<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDDgels<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDSgels<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDHgels<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDBgels<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDXgels<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnSSgels<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnSHgels<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnSBgels<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnSXgels<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            iter.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZZgels_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZCgels_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZKgels_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZEgels_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZYgels_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnCCgels_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnCKgels_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnCEgels_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnCYgels_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDDgels_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDSgels_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDHgels_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDBgels_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDXgels_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnSSgels_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnSHgels_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnSBgels_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnSXgels_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnIRSXgesv<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            gesv_irs_params,
            gesv_irs_infos,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            niters.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnIRSXgesv_bufferSize<T: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    params: cusolverDnIRSParams_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut lwork_bytes: T,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnIRSXgesv_bufferSize(handle, params, n, nrhs, lwork_bytes.as_mut_ptr() as *mut _)
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnIRSXgels<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            gels_irs_params,
            gels_irs_infos,
            m,
            n,
            nrhs,
            dA.as_mut_ptr() as *mut _,
            ldda,
            dB.as_mut_ptr() as *mut _,
            lddb,
            dX.as_mut_ptr() as *mut _,
            lddx,
            dWorkspace.as_mut_ptr() as *mut _,
            lwork_bytes,
            niters.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnIRSXgels_bufferSize<T: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    params: cusolverDnIRSParams_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    mut lwork_bytes: T,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnIRSXgels_bufferSize(handle, params, m, n, nrhs, lwork_bytes.as_mut_ptr() as *mut _)
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnSpotrf_bufferSize<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    mut Lwork: U,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSpotrf_bufferSize(
            handle,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            Lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDpotrf_bufferSize<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    mut Lwork: U,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDpotrf_bufferSize(
            handle,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            Lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnCpotrf_bufferSize<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    mut Lwork: U,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCpotrf_bufferSize(
            handle,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            Lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZpotrf_bufferSize<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    mut Lwork: U,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZpotrf_bufferSize(
            handle,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            Lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnSpotrf<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    mut Workspace: U,
    Lwork: i32,
    mut devInfo: V,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSpotrf(
            handle,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            Workspace.as_mut_ptr() as *mut _,
            Lwork as _,
            devInfo.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDpotrf<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    mut Workspace: U,
    Lwork: i32,
    mut devInfo: V,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDpotrf(
            handle,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            Workspace.as_mut_ptr() as *mut _,
            Lwork as _,
            devInfo.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnCpotrf<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    mut Workspace: U,
    Lwork: i32,
    mut devInfo: V,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCpotrf(
            handle,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            Workspace.as_mut_ptr() as *mut _,
            Lwork as _,
            devInfo.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZpotrf<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    mut Workspace: U,
    Lwork: i32,
    mut devInfo: V,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZpotrf(
            handle,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            Workspace.as_mut_ptr() as *mut _,
            Lwork as _,
            devInfo.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnSpotrs<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    nrhs: i32,
    A: T,
    lda: i32,
    mut B: U,
    ldb: i32,
    mut devInfo: V,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSpotrs(
            handle,
            uplo,
            n as _,
            nrhs as _,
            A.as_const_ptr() as *const _,
            lda as _,
            B.as_mut_ptr() as *mut _,
            ldb as _,
            devInfo.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDpotrs<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    nrhs: i32,
    A: T,
    lda: i32,
    mut B: U,
    ldb: i32,
    mut devInfo: V,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDpotrs(
            handle,
            uplo,
            n as _,
            nrhs as _,
            A.as_const_ptr() as *const _,
            lda as _,
            B.as_mut_ptr() as *mut _,
            ldb as _,
            devInfo.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnCpotrs<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    nrhs: i32,
    A: T,
    lda: i32,
    mut B: U,
    ldb: i32,
    mut devInfo: V,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCpotrs(
            handle,
            uplo,
            n as _,
            nrhs as _,
            A.as_const_ptr() as *const _,
            lda as _,
            B.as_mut_ptr() as *mut _,
            ldb as _,
            devInfo.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZpotrs<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    nrhs: i32,
    A: T,
    lda: i32,
    mut B: U,
    ldb: i32,
    mut devInfo: V,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZpotrs(
            handle,
            uplo,
            n as _,
            nrhs as _,
            A.as_const_ptr() as *const _,
            lda as _,
            B.as_mut_ptr() as *mut _,
            ldb as _,
            devInfo.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnSpotrfBatched<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut Aarray: T,
    lda: i32,
    mut infoArray: U,
    batchSize: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSpotrfBatched(
            handle,
            uplo,
            n as _,
            Aarray.as_mut_ptr() as *mut _,
            lda as _,
            infoArray.as_mut_ptr() as *mut _,
            batchSize as _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDpotrfBatched<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut Aarray: T,
    lda: i32,
    mut infoArray: U,
    batchSize: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDpotrfBatched(
            handle,
            uplo,
            n as _,
            Aarray.as_mut_ptr() as *mut _,
            lda as _,
            infoArray.as_mut_ptr() as *mut _,
            batchSize as _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnCpotrfBatched<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut Aarray: T,
    lda: i32,
    mut infoArray: U,
    batchSize: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCpotrfBatched(
            handle,
            uplo,
            n as _,
            Aarray.as_mut_ptr() as *mut _,
            lda as _,
            infoArray.as_mut_ptr() as *mut _,
            batchSize as _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZpotrfBatched<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut Aarray: T,
    lda: i32,
    mut infoArray: U,
    batchSize: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZpotrfBatched(
            handle,
            uplo,
            n as _,
            Aarray.as_mut_ptr() as *mut _,
            lda as _,
            infoArray.as_mut_ptr() as *mut _,
            batchSize as _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnSpotrsBatched<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    nrhs: i32,
    mut A: T,
    lda: i32,
    mut B: U,
    ldb: i32,
    mut d_info: V,
    batchSize: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSpotrsBatched(
            handle,
            uplo,
            n as _,
            nrhs as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            B.as_mut_ptr() as *mut _,
            ldb as _,
            d_info.as_mut_ptr() as *mut _,
            batchSize as _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDpotrsBatched<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    nrhs: i32,
    mut A: T,
    lda: i32,
    mut B: U,
    ldb: i32,
    mut d_info: V,
    batchSize: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDpotrsBatched(
            handle,
            uplo,
            n as _,
            nrhs as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            B.as_mut_ptr() as *mut _,
            ldb as _,
            d_info.as_mut_ptr() as *mut _,
            batchSize as _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnCpotrsBatched<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    nrhs: i32,
    mut A: T,
    lda: i32,
    mut B: U,
    ldb: i32,
    mut d_info: V,
    batchSize: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCpotrsBatched(
            handle,
            uplo,
            n as _,
            nrhs as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            B.as_mut_ptr() as *mut _,
            ldb as _,
            d_info.as_mut_ptr() as *mut _,
            batchSize as _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZpotrsBatched<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    nrhs: i32,
    mut A: T,
    lda: i32,
    mut B: U,
    ldb: i32,
    mut d_info: V,
    batchSize: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZpotrsBatched(
            handle,
            uplo,
            n as _,
            nrhs as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            B.as_mut_ptr() as *mut _,
            ldb as _,
            d_info.as_mut_ptr() as *mut _,
            batchSize as _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnSpotri_bufferSize<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    mut lwork: U,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSpotri_bufferSize(
            handle,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDpotri_bufferSize<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    mut lwork: U,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDpotri_bufferSize(
            handle,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnCpotri_bufferSize<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    mut lwork: U,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCpotri_bufferSize(
            handle,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZpotri_bufferSize<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    mut lwork: U,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZpotri_bufferSize(
            handle,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnSpotri<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    mut work: U,
    lwork: i32,
    mut devInfo: V,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSpotri(
            handle,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            devInfo.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDpotri<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    mut work: U,
    lwork: i32,
    mut devInfo: V,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDpotri(
            handle,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            devInfo.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnCpotri<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    mut work: U,
    lwork: i32,
    mut devInfo: V,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCpotri(
            handle,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            devInfo.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZpotri<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    mut work: U,
    lwork: i32,
    mut devInfo: V,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZpotri(
            handle,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            devInfo.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnXtrtri_bufferSize<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
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
            handle,
            uplo,
            diag,
            n,
            dataTypeA,
            A.as_mut_ptr() as *mut _,
            lda,
            workspaceInBytesOnDevice.as_mut_ptr() as *mut _,
            workspaceInBytesOnHost.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnXtrtri<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
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
            handle,
            uplo,
            diag,
            n,
            dataTypeA,
            A.as_mut_ptr() as *mut _,
            lda,
            bufferOnDevice.as_mut_ptr() as *mut _,
            workspaceInBytesOnDevice,
            bufferOnHost.as_mut_ptr() as *mut _,
            workspaceInBytesOnHost,
            devInfo.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnSlauum_bufferSize<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    mut lwork: U,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSlauum_bufferSize(
            handle,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDlauum_bufferSize<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    mut lwork: U,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDlauum_bufferSize(
            handle,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnClauum_bufferSize<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    mut lwork: U,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnClauum_bufferSize(
            handle,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZlauum_bufferSize<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    mut lwork: U,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZlauum_bufferSize(
            handle,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnSlauum<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    mut work: U,
    lwork: i32,
    mut devInfo: V,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSlauum(
            handle,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            devInfo.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDlauum<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    mut work: U,
    lwork: i32,
    mut devInfo: V,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDlauum(
            handle,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            devInfo.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnClauum<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    mut work: U,
    lwork: i32,
    mut devInfo: V,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnClauum(
            handle,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            devInfo.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZlauum<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    mut work: U,
    lwork: i32,
    mut devInfo: V,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZlauum(
            handle,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            devInfo.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnSgetrf_bufferSize<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    m: i32,
    n: i32,
    mut A: T,
    lda: i32,
    mut Lwork: U,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSgetrf_bufferSize(
            handle,
            m as _,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            Lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDgetrf_bufferSize<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    m: i32,
    n: i32,
    mut A: T,
    lda: i32,
    mut Lwork: U,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDgetrf_bufferSize(
            handle,
            m as _,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            Lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnCgetrf_bufferSize<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    m: i32,
    n: i32,
    mut A: T,
    lda: i32,
    mut Lwork: U,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCgetrf_bufferSize(
            handle,
            m as _,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            Lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZgetrf_bufferSize<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    m: i32,
    n: i32,
    mut A: T,
    lda: i32,
    mut Lwork: U,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZgetrf_bufferSize(
            handle,
            m as _,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            Lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnSgetrf<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    m: i32,
    n: i32,
    mut A: T,
    lda: i32,
    mut Workspace: U,
    mut devIpiv: V,
    mut devInfo: W,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSgetrf(
            handle,
            m as _,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            Workspace.as_mut_ptr() as *mut _,
            devIpiv.as_mut_ptr() as *mut _,
            devInfo.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDgetrf<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    m: i32,
    n: i32,
    mut A: T,
    lda: i32,
    mut Workspace: U,
    mut devIpiv: V,
    mut devInfo: W,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDgetrf(
            handle,
            m as _,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            Workspace.as_mut_ptr() as *mut _,
            devIpiv.as_mut_ptr() as *mut _,
            devInfo.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnCgetrf<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    m: i32,
    n: i32,
    mut A: T,
    lda: i32,
    mut Workspace: U,
    mut devIpiv: V,
    mut devInfo: W,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCgetrf(
            handle,
            m as _,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            Workspace.as_mut_ptr() as *mut _,
            devIpiv.as_mut_ptr() as *mut _,
            devInfo.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZgetrf<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    m: i32,
    n: i32,
    mut A: T,
    lda: i32,
    mut Workspace: U,
    mut devIpiv: V,
    mut devInfo: W,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZgetrf(
            handle,
            m as _,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            Workspace.as_mut_ptr() as *mut _,
            devIpiv.as_mut_ptr() as *mut _,
            devInfo.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnSlaswp<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    n: i32,
    mut A: T,
    lda: i32,
    k1: i32,
    k2: i32,
    devIpiv: U,
    incx: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSlaswp(
            handle,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            k1 as _,
            k2 as _,
            devIpiv.as_const_ptr() as *const _,
            incx as _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDlaswp<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    n: i32,
    mut A: T,
    lda: i32,
    k1: i32,
    k2: i32,
    devIpiv: U,
    incx: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDlaswp(
            handle,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            k1 as _,
            k2 as _,
            devIpiv.as_const_ptr() as *const _,
            incx as _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnClaswp<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    n: i32,
    mut A: T,
    lda: i32,
    k1: i32,
    k2: i32,
    devIpiv: U,
    incx: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnClaswp(
            handle,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            k1 as _,
            k2 as _,
            devIpiv.as_const_ptr() as *const _,
            incx as _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZlaswp<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    n: i32,
    mut A: T,
    lda: i32,
    k1: i32,
    k2: i32,
    devIpiv: U,
    incx: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZlaswp(
            handle,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            k1 as _,
            k2 as _,
            devIpiv.as_const_ptr() as *const _,
            incx as _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnSgetrs<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    trans: cublasOperation_t,
    n: i32,
    nrhs: i32,
    A: T,
    lda: i32,
    devIpiv: U,
    mut B: V,
    ldb: i32,
    mut devInfo: W,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSgetrs(
            handle,
            trans,
            n as _,
            nrhs as _,
            A.as_const_ptr() as *const _,
            lda as _,
            devIpiv.as_const_ptr() as *const _,
            B.as_mut_ptr() as *mut _,
            ldb as _,
            devInfo.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDgetrs<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    trans: cublasOperation_t,
    n: i32,
    nrhs: i32,
    A: T,
    lda: i32,
    devIpiv: U,
    mut B: V,
    ldb: i32,
    mut devInfo: W,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDgetrs(
            handle,
            trans,
            n as _,
            nrhs as _,
            A.as_const_ptr() as *const _,
            lda as _,
            devIpiv.as_const_ptr() as *const _,
            B.as_mut_ptr() as *mut _,
            ldb as _,
            devInfo.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnCgetrs<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    trans: cublasOperation_t,
    n: i32,
    nrhs: i32,
    A: T,
    lda: i32,
    devIpiv: U,
    mut B: V,
    ldb: i32,
    mut devInfo: W,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCgetrs(
            handle,
            trans,
            n as _,
            nrhs as _,
            A.as_const_ptr() as *const _,
            lda as _,
            devIpiv.as_const_ptr() as *const _,
            B.as_mut_ptr() as *mut _,
            ldb as _,
            devInfo.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZgetrs<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    trans: cublasOperation_t,
    n: i32,
    nrhs: i32,
    A: T,
    lda: i32,
    devIpiv: U,
    mut B: V,
    ldb: i32,
    mut devInfo: W,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZgetrs(
            handle,
            trans,
            n as _,
            nrhs as _,
            A.as_const_ptr() as *const _,
            lda as _,
            devIpiv.as_const_ptr() as *const _,
            B.as_mut_ptr() as *mut _,
            ldb as _,
            devInfo.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnSgeqrf_bufferSize<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    m: i32,
    n: i32,
    mut A: T,
    lda: i32,
    mut lwork: U,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSgeqrf_bufferSize(
            handle,
            m as _,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDgeqrf_bufferSize<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    m: i32,
    n: i32,
    mut A: T,
    lda: i32,
    mut lwork: U,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDgeqrf_bufferSize(
            handle,
            m as _,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnCgeqrf_bufferSize<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    m: i32,
    n: i32,
    mut A: T,
    lda: i32,
    mut lwork: U,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCgeqrf_bufferSize(
            handle,
            m as _,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZgeqrf_bufferSize<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    m: i32,
    n: i32,
    mut A: T,
    lda: i32,
    mut lwork: U,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZgeqrf_bufferSize(
            handle,
            m as _,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnSgeqrf<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    m: i32,
    n: i32,
    mut A: T,
    lda: i32,
    mut TAU: U,
    mut Workspace: V,
    Lwork: i32,
    mut devInfo: W,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSgeqrf(
            handle,
            m as _,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            TAU.as_mut_ptr() as *mut _,
            Workspace.as_mut_ptr() as *mut _,
            Lwork as _,
            devInfo.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDgeqrf<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    m: i32,
    n: i32,
    mut A: T,
    lda: i32,
    mut TAU: U,
    mut Workspace: V,
    Lwork: i32,
    mut devInfo: W,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDgeqrf(
            handle,
            m as _,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            TAU.as_mut_ptr() as *mut _,
            Workspace.as_mut_ptr() as *mut _,
            Lwork as _,
            devInfo.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnCgeqrf<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    m: i32,
    n: i32,
    mut A: T,
    lda: i32,
    mut TAU: U,
    mut Workspace: V,
    Lwork: i32,
    mut devInfo: W,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCgeqrf(
            handle,
            m as _,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            TAU.as_mut_ptr() as *mut _,
            Workspace.as_mut_ptr() as *mut _,
            Lwork as _,
            devInfo.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZgeqrf<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    m: i32,
    n: i32,
    mut A: T,
    lda: i32,
    mut TAU: U,
    mut Workspace: V,
    Lwork: i32,
    mut devInfo: W,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZgeqrf(
            handle,
            m as _,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            TAU.as_mut_ptr() as *mut _,
            Workspace.as_mut_ptr() as *mut _,
            Lwork as _,
            devInfo.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnSorgqr_bufferSize<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    m: i32,
    n: i32,
    k: i32,
    A: T,
    lda: i32,
    tau: U,
    mut lwork: V,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSorgqr_bufferSize(
            handle,
            m as _,
            n as _,
            k as _,
            A.as_const_ptr() as *const _,
            lda as _,
            tau.as_const_ptr() as *const _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDorgqr_bufferSize<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    m: i32,
    n: i32,
    k: i32,
    A: T,
    lda: i32,
    tau: U,
    mut lwork: V,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDorgqr_bufferSize(
            handle,
            m as _,
            n as _,
            k as _,
            A.as_const_ptr() as *const _,
            lda as _,
            tau.as_const_ptr() as *const _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnCungqr_bufferSize<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    m: i32,
    n: i32,
    k: i32,
    A: T,
    lda: i32,
    tau: U,
    mut lwork: V,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCungqr_bufferSize(
            handle,
            m as _,
            n as _,
            k as _,
            A.as_const_ptr() as *const _,
            lda as _,
            tau.as_const_ptr() as *const _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZungqr_bufferSize<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    m: i32,
    n: i32,
    k: i32,
    A: T,
    lda: i32,
    tau: U,
    mut lwork: V,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZungqr_bufferSize(
            handle,
            m as _,
            n as _,
            k as _,
            A.as_const_ptr() as *const _,
            lda as _,
            tau.as_const_ptr() as *const _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnSorgqr<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    m: i32,
    n: i32,
    k: i32,
    mut A: T,
    lda: i32,
    tau: U,
    mut work: V,
    lwork: i32,
    mut info: W,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSorgqr(
            handle,
            m as _,
            n as _,
            k as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            tau.as_const_ptr() as *const _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDorgqr<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    m: i32,
    n: i32,
    k: i32,
    mut A: T,
    lda: i32,
    tau: U,
    mut work: V,
    lwork: i32,
    mut info: W,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDorgqr(
            handle,
            m as _,
            n as _,
            k as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            tau.as_const_ptr() as *const _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnCungqr<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    m: i32,
    n: i32,
    k: i32,
    mut A: T,
    lda: i32,
    tau: U,
    mut work: V,
    lwork: i32,
    mut info: W,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCungqr(
            handle,
            m as _,
            n as _,
            k as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            tau.as_const_ptr() as *const _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZungqr<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    m: i32,
    n: i32,
    k: i32,
    mut A: T,
    lda: i32,
    tau: U,
    mut work: V,
    lwork: i32,
    mut info: W,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZungqr(
            handle,
            m as _,
            n as _,
            k as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            tau.as_const_ptr() as *const _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnSormqr_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    A: T,
    lda: i32,
    tau: U,
    C: V,
    ldc: i32,
    mut lwork: W,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSormqr_bufferSize(
            handle,
            side,
            trans,
            m as _,
            n as _,
            k as _,
            A.as_const_ptr() as *const _,
            lda as _,
            tau.as_const_ptr() as *const _,
            C.as_const_ptr() as *const _,
            ldc as _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDormqr_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    A: T,
    lda: i32,
    tau: U,
    C: V,
    ldc: i32,
    mut lwork: W,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDormqr_bufferSize(
            handle,
            side,
            trans,
            m as _,
            n as _,
            k as _,
            A.as_const_ptr() as *const _,
            lda as _,
            tau.as_const_ptr() as *const _,
            C.as_const_ptr() as *const _,
            ldc as _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnCunmqr_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    A: T,
    lda: i32,
    tau: U,
    C: V,
    ldc: i32,
    mut lwork: W,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCunmqr_bufferSize(
            handle,
            side,
            trans,
            m as _,
            n as _,
            k as _,
            A.as_const_ptr() as *const _,
            lda as _,
            tau.as_const_ptr() as *const _,
            C.as_const_ptr() as *const _,
            ldc as _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZunmqr_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    A: T,
    lda: i32,
    tau: U,
    C: V,
    ldc: i32,
    mut lwork: W,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZunmqr_bufferSize(
            handle,
            side,
            trans,
            m as _,
            n as _,
            k as _,
            A.as_const_ptr() as *const _,
            lda as _,
            tau.as_const_ptr() as *const _,
            C.as_const_ptr() as *const _,
            ldc as _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnSormqr<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    A: T,
    lda: i32,
    tau: U,
    mut C: V,
    ldc: i32,
    mut work: W,
    lwork: i32,
    mut devInfo: X,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSormqr(
            handle,
            side,
            trans,
            m as _,
            n as _,
            k as _,
            A.as_const_ptr() as *const _,
            lda as _,
            tau.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc as _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            devInfo.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDormqr<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    A: T,
    lda: i32,
    tau: U,
    mut C: V,
    ldc: i32,
    mut work: W,
    lwork: i32,
    mut devInfo: X,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDormqr(
            handle,
            side,
            trans,
            m as _,
            n as _,
            k as _,
            A.as_const_ptr() as *const _,
            lda as _,
            tau.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc as _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            devInfo.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnCunmqr<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    A: T,
    lda: i32,
    tau: U,
    mut C: V,
    ldc: i32,
    mut work: W,
    lwork: i32,
    mut devInfo: X,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCunmqr(
            handle,
            side,
            trans,
            m as _,
            n as _,
            k as _,
            A.as_const_ptr() as *const _,
            lda as _,
            tau.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc as _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            devInfo.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZunmqr<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    A: T,
    lda: i32,
    tau: U,
    mut C: V,
    ldc: i32,
    mut work: W,
    lwork: i32,
    mut devInfo: X,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZunmqr(
            handle,
            side,
            trans,
            m as _,
            n as _,
            k as _,
            A.as_const_ptr() as *const _,
            lda as _,
            tau.as_const_ptr() as *const _,
            C.as_mut_ptr() as *mut _,
            ldc as _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            devInfo.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnSsytrf_bufferSize<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    n: i32,
    mut A: T,
    lda: i32,
    mut lwork: U,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSsytrf_bufferSize(
            handle,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDsytrf_bufferSize<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    n: i32,
    mut A: T,
    lda: i32,
    mut lwork: U,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDsytrf_bufferSize(
            handle,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnCsytrf_bufferSize<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    n: i32,
    mut A: T,
    lda: i32,
    mut lwork: U,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCsytrf_bufferSize(
            handle,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZsytrf_bufferSize<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    n: i32,
    mut A: T,
    lda: i32,
    mut lwork: U,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZsytrf_bufferSize(
            handle,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnSsytrf<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    mut ipiv: U,
    mut work: V,
    lwork: i32,
    mut info: W,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSsytrf(
            handle,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            ipiv.as_mut_ptr() as *mut _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDsytrf<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    mut ipiv: U,
    mut work: V,
    lwork: i32,
    mut info: W,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDsytrf(
            handle,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            ipiv.as_mut_ptr() as *mut _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnCsytrf<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    mut ipiv: U,
    mut work: V,
    lwork: i32,
    mut info: W,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCsytrf(
            handle,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            ipiv.as_mut_ptr() as *mut _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZsytrf<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    mut ipiv: U,
    mut work: V,
    lwork: i32,
    mut info: W,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZsytrf(
            handle,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            ipiv.as_mut_ptr() as *mut _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnXsytrs_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            uplo,
            n,
            nrhs,
            dataTypeA,
            A.as_const_ptr() as *const _,
            lda,
            ipiv.as_const_ptr() as *const _,
            dataTypeB,
            B.as_mut_ptr() as *mut _,
            ldb,
            workspaceInBytesOnDevice.as_mut_ptr() as *mut _,
            workspaceInBytesOnHost.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnXsytrs<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            uplo,
            n,
            nrhs,
            dataTypeA,
            A.as_const_ptr() as *const _,
            lda,
            ipiv.as_const_ptr() as *const _,
            dataTypeB,
            B.as_mut_ptr() as *mut _,
            ldb,
            bufferOnDevice.as_mut_ptr() as *mut _,
            workspaceInBytesOnDevice,
            bufferOnHost.as_mut_ptr() as *mut _,
            workspaceInBytesOnHost,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnSsytri_bufferSize<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    ipiv: U,
    mut lwork: V,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSsytri_bufferSize(
            handle,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            ipiv.as_const_ptr() as *const _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDsytri_bufferSize<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    ipiv: U,
    mut lwork: V,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDsytri_bufferSize(
            handle,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            ipiv.as_const_ptr() as *const _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnCsytri_bufferSize<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    ipiv: U,
    mut lwork: V,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCsytri_bufferSize(
            handle,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            ipiv.as_const_ptr() as *const _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZsytri_bufferSize<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    ipiv: U,
    mut lwork: V,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZsytri_bufferSize(
            handle,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            ipiv.as_const_ptr() as *const _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnSsytri<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    ipiv: U,
    mut work: V,
    lwork: i32,
    mut info: W,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSsytri(
            handle,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            ipiv.as_const_ptr() as *const _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDsytri<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    ipiv: U,
    mut work: V,
    lwork: i32,
    mut info: W,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDsytri(
            handle,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            ipiv.as_const_ptr() as *const _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnCsytri<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    ipiv: U,
    mut work: V,
    lwork: i32,
    mut info: W,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCsytri(
            handle,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            ipiv.as_const_ptr() as *const _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZsytri<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    ipiv: U,
    mut work: V,
    lwork: i32,
    mut info: W,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZsytri(
            handle,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            ipiv.as_const_ptr() as *const _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnSgebrd_bufferSize<T: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    m: i32,
    n: i32,
    mut Lwork: T,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status =
        unsafe { crate::sys::cusolverDnSgebrd_bufferSize(handle, m as _, n as _, Lwork.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDgebrd_bufferSize<T: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    m: i32,
    n: i32,
    mut Lwork: T,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status =
        unsafe { crate::sys::cusolverDnDgebrd_bufferSize(handle, m as _, n as _, Lwork.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnCgebrd_bufferSize<T: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    m: i32,
    n: i32,
    mut Lwork: T,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status =
        unsafe { crate::sys::cusolverDnCgebrd_bufferSize(handle, m as _, n as _, Lwork.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZgebrd_bufferSize<T: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    m: i32,
    n: i32,
    mut Lwork: T,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status =
        unsafe { crate::sys::cusolverDnZgebrd_bufferSize(handle, m as _, n as _, Lwork.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnSgebrd<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    m: i32,
    n: i32,
    mut A: T,
    lda: i32,
    mut D: U,
    mut E: V,
    mut TAUQ: W,
    mut TAUP: X,
    mut Work: Y,
    Lwork: i32,
    mut devInfo: Z,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSgebrd(
            handle,
            m as _,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            D.as_mut_ptr() as *mut _,
            E.as_mut_ptr() as *mut _,
            TAUQ.as_mut_ptr() as *mut _,
            TAUP.as_mut_ptr() as *mut _,
            Work.as_mut_ptr() as *mut _,
            Lwork as _,
            devInfo.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDgebrd<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    m: i32,
    n: i32,
    mut A: T,
    lda: i32,
    mut D: U,
    mut E: V,
    mut TAUQ: W,
    mut TAUP: X,
    mut Work: Y,
    Lwork: i32,
    mut devInfo: Z,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDgebrd(
            handle,
            m as _,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            D.as_mut_ptr() as *mut _,
            E.as_mut_ptr() as *mut _,
            TAUQ.as_mut_ptr() as *mut _,
            TAUP.as_mut_ptr() as *mut _,
            Work.as_mut_ptr() as *mut _,
            Lwork as _,
            devInfo.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnCgebrd<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    m: i32,
    n: i32,
    mut A: T,
    lda: i32,
    mut D: U,
    mut E: V,
    mut TAUQ: W,
    mut TAUP: X,
    mut Work: Y,
    Lwork: i32,
    mut devInfo: Z,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCgebrd(
            handle,
            m as _,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            D.as_mut_ptr() as *mut _,
            E.as_mut_ptr() as *mut _,
            TAUQ.as_mut_ptr() as *mut _,
            TAUP.as_mut_ptr() as *mut _,
            Work.as_mut_ptr() as *mut _,
            Lwork as _,
            devInfo.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZgebrd<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    m: i32,
    n: i32,
    mut A: T,
    lda: i32,
    mut D: U,
    mut E: V,
    mut TAUQ: W,
    mut TAUP: X,
    mut Work: Y,
    Lwork: i32,
    mut devInfo: Z,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZgebrd(
            handle,
            m as _,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            D.as_mut_ptr() as *mut _,
            E.as_mut_ptr() as *mut _,
            TAUQ.as_mut_ptr() as *mut _,
            TAUP.as_mut_ptr() as *mut _,
            Work.as_mut_ptr() as *mut _,
            Lwork as _,
            devInfo.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnSorgbr_bufferSize<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    m: i32,
    n: i32,
    k: i32,
    A: T,
    lda: i32,
    tau: U,
    mut lwork: V,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSorgbr_bufferSize(
            handle,
            side,
            m as _,
            n as _,
            k as _,
            A.as_const_ptr() as *const _,
            lda as _,
            tau.as_const_ptr() as *const _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDorgbr_bufferSize<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    m: i32,
    n: i32,
    k: i32,
    A: T,
    lda: i32,
    tau: U,
    mut lwork: V,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDorgbr_bufferSize(
            handle,
            side,
            m as _,
            n as _,
            k as _,
            A.as_const_ptr() as *const _,
            lda as _,
            tau.as_const_ptr() as *const _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnCungbr_bufferSize<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    m: i32,
    n: i32,
    k: i32,
    A: T,
    lda: i32,
    tau: U,
    mut lwork: V,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCungbr_bufferSize(
            handle,
            side,
            m as _,
            n as _,
            k as _,
            A.as_const_ptr() as *const _,
            lda as _,
            tau.as_const_ptr() as *const _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZungbr_bufferSize<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    m: i32,
    n: i32,
    k: i32,
    A: T,
    lda: i32,
    tau: U,
    mut lwork: V,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZungbr_bufferSize(
            handle,
            side,
            m as _,
            n as _,
            k as _,
            A.as_const_ptr() as *const _,
            lda as _,
            tau.as_const_ptr() as *const _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnSorgbr<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    m: i32,
    n: i32,
    k: i32,
    mut A: T,
    lda: i32,
    tau: U,
    mut work: V,
    lwork: i32,
    mut info: W,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSorgbr(
            handle,
            side,
            m as _,
            n as _,
            k as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            tau.as_const_ptr() as *const _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDorgbr<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    m: i32,
    n: i32,
    k: i32,
    mut A: T,
    lda: i32,
    tau: U,
    mut work: V,
    lwork: i32,
    mut info: W,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDorgbr(
            handle,
            side,
            m as _,
            n as _,
            k as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            tau.as_const_ptr() as *const _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnCungbr<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    m: i32,
    n: i32,
    k: i32,
    mut A: T,
    lda: i32,
    tau: U,
    mut work: V,
    lwork: i32,
    mut info: W,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCungbr(
            handle,
            side,
            m as _,
            n as _,
            k as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            tau.as_const_ptr() as *const _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZungbr<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    m: i32,
    n: i32,
    k: i32,
    mut A: T,
    lda: i32,
    tau: U,
    mut work: V,
    lwork: i32,
    mut info: W,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZungbr(
            handle,
            side,
            m as _,
            n as _,
            k as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            tau.as_const_ptr() as *const _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnSsytrd_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    A: T,
    lda: i32,
    d: U,
    e: V,
    tau: W,
    mut lwork: X,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSsytrd_bufferSize(
            handle,
            uplo,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            d.as_const_ptr() as *const _,
            e.as_const_ptr() as *const _,
            tau.as_const_ptr() as *const _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDsytrd_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    A: T,
    lda: i32,
    d: U,
    e: V,
    tau: W,
    mut lwork: X,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDsytrd_bufferSize(
            handle,
            uplo,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            d.as_const_ptr() as *const _,
            e.as_const_ptr() as *const _,
            tau.as_const_ptr() as *const _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnChetrd_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    A: T,
    lda: i32,
    d: U,
    e: V,
    tau: W,
    mut lwork: X,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnChetrd_bufferSize(
            handle,
            uplo,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            d.as_const_ptr() as *const _,
            e.as_const_ptr() as *const _,
            tau.as_const_ptr() as *const _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZhetrd_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    A: T,
    lda: i32,
    d: U,
    e: V,
    tau: W,
    mut lwork: X,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZhetrd_bufferSize(
            handle,
            uplo,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            d.as_const_ptr() as *const _,
            e.as_const_ptr() as *const _,
            tau.as_const_ptr() as *const _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnSsytrd<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    mut d: U,
    mut e: V,
    mut tau: W,
    mut work: X,
    lwork: i32,
    mut info: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSsytrd(
            handle,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            d.as_mut_ptr() as *mut _,
            e.as_mut_ptr() as *mut _,
            tau.as_mut_ptr() as *mut _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDsytrd<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    mut d: U,
    mut e: V,
    mut tau: W,
    mut work: X,
    lwork: i32,
    mut info: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDsytrd(
            handle,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            d.as_mut_ptr() as *mut _,
            e.as_mut_ptr() as *mut _,
            tau.as_mut_ptr() as *mut _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnChetrd<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    mut d: U,
    mut e: V,
    mut tau: W,
    mut work: X,
    lwork: i32,
    mut info: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnChetrd(
            handle,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            d.as_mut_ptr() as *mut _,
            e.as_mut_ptr() as *mut _,
            tau.as_mut_ptr() as *mut _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZhetrd<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    mut d: U,
    mut e: V,
    mut tau: W,
    mut work: X,
    lwork: i32,
    mut info: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZhetrd(
            handle,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            d.as_mut_ptr() as *mut _,
            e.as_mut_ptr() as *mut _,
            tau.as_mut_ptr() as *mut _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnSorgtr_bufferSize<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    A: T,
    lda: i32,
    tau: U,
    mut lwork: V,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSorgtr_bufferSize(
            handle,
            uplo,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            tau.as_const_ptr() as *const _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDorgtr_bufferSize<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    A: T,
    lda: i32,
    tau: U,
    mut lwork: V,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDorgtr_bufferSize(
            handle,
            uplo,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            tau.as_const_ptr() as *const _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnCungtr_bufferSize<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    A: T,
    lda: i32,
    tau: U,
    mut lwork: V,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCungtr_bufferSize(
            handle,
            uplo,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            tau.as_const_ptr() as *const _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZungtr_bufferSize<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    A: T,
    lda: i32,
    tau: U,
    mut lwork: V,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZungtr_bufferSize(
            handle,
            uplo,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            tau.as_const_ptr() as *const _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnSorgtr<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    tau: U,
    mut work: V,
    lwork: i32,
    mut info: W,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSorgtr(
            handle,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            tau.as_const_ptr() as *const _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDorgtr<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    tau: U,
    mut work: V,
    lwork: i32,
    mut info: W,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDorgtr(
            handle,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            tau.as_const_ptr() as *const _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnCungtr<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    tau: U,
    mut work: V,
    lwork: i32,
    mut info: W,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCungtr(
            handle,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            tau.as_const_ptr() as *const _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZungtr<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    tau: U,
    mut work: V,
    lwork: i32,
    mut info: W,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZungtr(
            handle,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            tau.as_const_ptr() as *const _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnSormtr_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    A: T,
    lda: i32,
    tau: U,
    C: V,
    ldc: i32,
    mut lwork: W,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSormtr_bufferSize(
            handle,
            side,
            uplo,
            trans,
            m as _,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            tau.as_const_ptr() as *const _,
            C.as_const_ptr() as *const _,
            ldc as _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDormtr_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    A: T,
    lda: i32,
    tau: U,
    C: V,
    ldc: i32,
    mut lwork: W,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDormtr_bufferSize(
            handle,
            side,
            uplo,
            trans,
            m as _,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            tau.as_const_ptr() as *const _,
            C.as_const_ptr() as *const _,
            ldc as _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnCunmtr_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    A: T,
    lda: i32,
    tau: U,
    C: V,
    ldc: i32,
    mut lwork: W,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCunmtr_bufferSize(
            handle,
            side,
            uplo,
            trans,
            m as _,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            tau.as_const_ptr() as *const _,
            C.as_const_ptr() as *const _,
            ldc as _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZunmtr_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    A: T,
    lda: i32,
    tau: U,
    C: V,
    ldc: i32,
    mut lwork: W,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZunmtr_bufferSize(
            handle,
            side,
            uplo,
            trans,
            m as _,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            tau.as_const_ptr() as *const _,
            C.as_const_ptr() as *const _,
            ldc as _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnSormtr<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    mut A: T,
    lda: i32,
    mut tau: U,
    mut C: V,
    ldc: i32,
    mut work: W,
    lwork: i32,
    mut info: X,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSormtr(
            handle,
            side,
            uplo,
            trans,
            m as _,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            tau.as_mut_ptr() as *mut _,
            C.as_mut_ptr() as *mut _,
            ldc as _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDormtr<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    mut A: T,
    lda: i32,
    mut tau: U,
    mut C: V,
    ldc: i32,
    mut work: W,
    lwork: i32,
    mut info: X,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDormtr(
            handle,
            side,
            uplo,
            trans,
            m as _,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            tau.as_mut_ptr() as *mut _,
            C.as_mut_ptr() as *mut _,
            ldc as _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnCunmtr<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    mut A: T,
    lda: i32,
    mut tau: U,
    mut C: V,
    ldc: i32,
    mut work: W,
    lwork: i32,
    mut info: X,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCunmtr(
            handle,
            side,
            uplo,
            trans,
            m as _,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            tau.as_mut_ptr() as *mut _,
            C.as_mut_ptr() as *mut _,
            ldc as _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZunmtr<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    mut A: T,
    lda: i32,
    mut tau: U,
    mut C: V,
    ldc: i32,
    mut work: W,
    lwork: i32,
    mut info: X,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZunmtr(
            handle,
            side,
            uplo,
            trans,
            m as _,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            tau.as_mut_ptr() as *mut _,
            C.as_mut_ptr() as *mut _,
            ldc as _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnSgesvd_bufferSize<T: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    m: i32,
    n: i32,
    mut lwork: T,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status =
        unsafe { crate::sys::cusolverDnSgesvd_bufferSize(handle, m as _, n as _, lwork.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDgesvd_bufferSize<T: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    m: i32,
    n: i32,
    mut lwork: T,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status =
        unsafe { crate::sys::cusolverDnDgesvd_bufferSize(handle, m as _, n as _, lwork.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnCgesvd_bufferSize<T: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    m: i32,
    n: i32,
    mut lwork: T,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status =
        unsafe { crate::sys::cusolverDnCgesvd_bufferSize(handle, m as _, n as _, lwork.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZgesvd_bufferSize<T: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    m: i32,
    n: i32,
    mut lwork: T,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status =
        unsafe { crate::sys::cusolverDnZgesvd_bufferSize(handle, m as _, n as _, lwork.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnSgesvd<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    jobu: ::std::os::raw::c_schar,
    jobvt: ::std::os::raw::c_schar,
    m: i32,
    n: i32,
    mut A: T,
    lda: i32,
    mut S: U,
    mut U: V,
    ldu: i32,
    mut VT: W,
    ldvt: i32,
    mut work: X,
    lwork: i32,
    mut rwork: Y,
    mut info: Z,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSgesvd(
            handle,
            jobu,
            jobvt,
            m as _,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            S.as_mut_ptr() as *mut _,
            U.as_mut_ptr() as *mut _,
            ldu as _,
            VT.as_mut_ptr() as *mut _,
            ldvt as _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            rwork.as_mut_ptr() as *mut _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDgesvd<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    jobu: ::std::os::raw::c_schar,
    jobvt: ::std::os::raw::c_schar,
    m: i32,
    n: i32,
    mut A: T,
    lda: i32,
    mut S: U,
    mut U: V,
    ldu: i32,
    mut VT: W,
    ldvt: i32,
    mut work: X,
    lwork: i32,
    mut rwork: Y,
    mut info: Z,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDgesvd(
            handle,
            jobu,
            jobvt,
            m as _,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            S.as_mut_ptr() as *mut _,
            U.as_mut_ptr() as *mut _,
            ldu as _,
            VT.as_mut_ptr() as *mut _,
            ldvt as _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            rwork.as_mut_ptr() as *mut _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnCgesvd<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    jobu: ::std::os::raw::c_schar,
    jobvt: ::std::os::raw::c_schar,
    m: i32,
    n: i32,
    mut A: T,
    lda: i32,
    mut S: U,
    mut U: V,
    ldu: i32,
    mut VT: W,
    ldvt: i32,
    mut work: X,
    lwork: i32,
    mut rwork: Y,
    mut info: Z,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCgesvd(
            handle,
            jobu,
            jobvt,
            m as _,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            S.as_mut_ptr() as *mut _,
            U.as_mut_ptr() as *mut _,
            ldu as _,
            VT.as_mut_ptr() as *mut _,
            ldvt as _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            rwork.as_mut_ptr() as *mut _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZgesvd<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    jobu: ::std::os::raw::c_schar,
    jobvt: ::std::os::raw::c_schar,
    m: i32,
    n: i32,
    mut A: T,
    lda: i32,
    mut S: U,
    mut U: V,
    ldu: i32,
    mut VT: W,
    ldvt: i32,
    mut work: X,
    lwork: i32,
    mut rwork: Y,
    mut info: Z,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZgesvd(
            handle,
            jobu,
            jobvt,
            m as _,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            S.as_mut_ptr() as *mut _,
            U.as_mut_ptr() as *mut _,
            ldu as _,
            VT.as_mut_ptr() as *mut _,
            ldvt as _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            rwork.as_mut_ptr() as *mut _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnSsyevd_bufferSize<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    A: T,
    lda: i32,
    W: U,
    mut lwork: V,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSsyevd_bufferSize(
            handle,
            jobz,
            uplo,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            W.as_const_ptr() as *const _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDsyevd_bufferSize<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    A: T,
    lda: i32,
    W: U,
    mut lwork: V,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDsyevd_bufferSize(
            handle,
            jobz,
            uplo,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            W.as_const_ptr() as *const _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnCheevd_bufferSize<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    A: T,
    lda: i32,
    W: U,
    mut lwork: V,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCheevd_bufferSize(
            handle,
            jobz,
            uplo,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            W.as_const_ptr() as *const _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZheevd_bufferSize<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    A: T,
    lda: i32,
    W: U,
    mut lwork: V,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZheevd_bufferSize(
            handle,
            jobz,
            uplo,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            W.as_const_ptr() as *const _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnSsyevd<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    mut W: U,
    mut work: V,
    lwork: i32,
    mut info: W,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSsyevd(
            handle,
            jobz,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            W.as_mut_ptr() as *mut _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDsyevd<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    mut W: U,
    mut work: V,
    lwork: i32,
    mut info: W,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDsyevd(
            handle,
            jobz,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            W.as_mut_ptr() as *mut _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnCheevd<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    mut W: U,
    mut work: V,
    lwork: i32,
    mut info: W,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCheevd(
            handle,
            jobz,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            W.as_mut_ptr() as *mut _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZheevd<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    mut W: U,
    mut work: V,
    lwork: i32,
    mut info: W,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZheevd(
            handle,
            jobz,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            W.as_mut_ptr() as *mut _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnSsyevdx_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: i32,
    A: T,
    lda: i32,
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    mut meig: U,
    W: V,
    mut lwork: W,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSsyevdx_bufferSize(
            handle,
            jobz,
            range,
            uplo,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            vl,
            vu,
            il as _,
            iu as _,
            meig.as_mut_ptr() as *mut _,
            W.as_const_ptr() as *const _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDsyevdx_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: i32,
    A: T,
    lda: i32,
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    mut meig: U,
    W: V,
    mut lwork: W,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDsyevdx_bufferSize(
            handle,
            jobz,
            range,
            uplo,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            vl,
            vu,
            il as _,
            iu as _,
            meig.as_mut_ptr() as *mut _,
            W.as_const_ptr() as *const _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnCheevdx_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: i32,
    A: T,
    lda: i32,
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    mut meig: U,
    W: V,
    mut lwork: W,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCheevdx_bufferSize(
            handle,
            jobz,
            range,
            uplo,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            vl,
            vu,
            il as _,
            iu as _,
            meig.as_mut_ptr() as *mut _,
            W.as_const_ptr() as *const _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZheevdx_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: i32,
    A: T,
    lda: i32,
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    mut meig: U,
    W: V,
    mut lwork: W,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZheevdx_bufferSize(
            handle,
            jobz,
            range,
            uplo,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            vl,
            vu,
            il as _,
            iu as _,
            meig.as_mut_ptr() as *mut _,
            W.as_const_ptr() as *const _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnSsyevdx<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    mut meig: U,
    mut W: V,
    mut work: W,
    lwork: i32,
    mut info: X,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSsyevdx(
            handle,
            jobz,
            range,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            vl,
            vu,
            il as _,
            iu as _,
            meig.as_mut_ptr() as *mut _,
            W.as_mut_ptr() as *mut _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDsyevdx<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    mut meig: U,
    mut W: V,
    mut work: W,
    lwork: i32,
    mut info: X,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDsyevdx(
            handle,
            jobz,
            range,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            vl,
            vu,
            il as _,
            iu as _,
            meig.as_mut_ptr() as *mut _,
            W.as_mut_ptr() as *mut _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnCheevdx<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    mut meig: U,
    mut W: V,
    mut work: W,
    lwork: i32,
    mut info: X,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCheevdx(
            handle,
            jobz,
            range,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            vl,
            vu,
            il as _,
            iu as _,
            meig.as_mut_ptr() as *mut _,
            W.as_mut_ptr() as *mut _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZheevdx<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    mut meig: U,
    mut W: V,
    mut work: W,
    lwork: i32,
    mut info: X,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZheevdx(
            handle,
            jobz,
            range,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            vl,
            vu,
            il as _,
            iu as _,
            meig.as_mut_ptr() as *mut _,
            W.as_mut_ptr() as *mut _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnSsygvdx_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: i32,
    A: T,
    lda: i32,
    B: U,
    ldb: i32,
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    mut meig: V,
    W: W,
    mut lwork: X,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSsygvdx_bufferSize(
            handle,
            itype,
            jobz,
            range,
            uplo,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            B.as_const_ptr() as *const _,
            ldb as _,
            vl,
            vu,
            il as _,
            iu as _,
            meig.as_mut_ptr() as *mut _,
            W.as_const_ptr() as *const _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDsygvdx_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: i32,
    A: T,
    lda: i32,
    B: U,
    ldb: i32,
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    mut meig: V,
    W: W,
    mut lwork: X,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDsygvdx_bufferSize(
            handle,
            itype,
            jobz,
            range,
            uplo,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            B.as_const_ptr() as *const _,
            ldb as _,
            vl,
            vu,
            il as _,
            iu as _,
            meig.as_mut_ptr() as *mut _,
            W.as_const_ptr() as *const _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnChegvdx_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: i32,
    A: T,
    lda: i32,
    B: U,
    ldb: i32,
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    mut meig: V,
    W: W,
    mut lwork: X,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnChegvdx_bufferSize(
            handle,
            itype,
            jobz,
            range,
            uplo,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            B.as_const_ptr() as *const _,
            ldb as _,
            vl,
            vu,
            il as _,
            iu as _,
            meig.as_mut_ptr() as *mut _,
            W.as_const_ptr() as *const _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZhegvdx_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: i32,
    A: T,
    lda: i32,
    B: U,
    ldb: i32,
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    mut meig: V,
    W: W,
    mut lwork: X,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZhegvdx_bufferSize(
            handle,
            itype,
            jobz,
            range,
            uplo,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            B.as_const_ptr() as *const _,
            ldb as _,
            vl,
            vu,
            il as _,
            iu as _,
            meig.as_mut_ptr() as *mut _,
            W.as_const_ptr() as *const _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnSsygvdx<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    mut B: U,
    ldb: i32,
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    mut meig: V,
    mut W: W,
    mut work: X,
    lwork: i32,
    mut info: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSsygvdx(
            handle,
            itype,
            jobz,
            range,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            B.as_mut_ptr() as *mut _,
            ldb as _,
            vl,
            vu,
            il as _,
            iu as _,
            meig.as_mut_ptr() as *mut _,
            W.as_mut_ptr() as *mut _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDsygvdx<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    mut B: U,
    ldb: i32,
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    mut meig: V,
    mut W: W,
    mut work: X,
    lwork: i32,
    mut info: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDsygvdx(
            handle,
            itype,
            jobz,
            range,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            B.as_mut_ptr() as *mut _,
            ldb as _,
            vl,
            vu,
            il as _,
            iu as _,
            meig.as_mut_ptr() as *mut _,
            W.as_mut_ptr() as *mut _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnChegvdx<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    mut B: U,
    ldb: i32,
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    mut meig: V,
    mut W: W,
    mut work: X,
    lwork: i32,
    mut info: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnChegvdx(
            handle,
            itype,
            jobz,
            range,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            B.as_mut_ptr() as *mut _,
            ldb as _,
            vl,
            vu,
            il as _,
            iu as _,
            meig.as_mut_ptr() as *mut _,
            W.as_mut_ptr() as *mut _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZhegvdx<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    mut B: U,
    ldb: i32,
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    mut meig: V,
    mut W: W,
    mut work: X,
    lwork: i32,
    mut info: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZhegvdx(
            handle,
            itype,
            jobz,
            range,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            B.as_mut_ptr() as *mut _,
            ldb as _,
            vl,
            vu,
            il as _,
            iu as _,
            meig.as_mut_ptr() as *mut _,
            W.as_mut_ptr() as *mut _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnSsygvd_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    A: T,
    lda: i32,
    B: U,
    ldb: i32,
    W: V,
    mut lwork: W,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSsygvd_bufferSize(
            handle,
            itype,
            jobz,
            uplo,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            B.as_const_ptr() as *const _,
            ldb as _,
            W.as_const_ptr() as *const _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDsygvd_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    A: T,
    lda: i32,
    B: U,
    ldb: i32,
    W: V,
    mut lwork: W,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDsygvd_bufferSize(
            handle,
            itype,
            jobz,
            uplo,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            B.as_const_ptr() as *const _,
            ldb as _,
            W.as_const_ptr() as *const _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnChegvd_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    A: T,
    lda: i32,
    B: U,
    ldb: i32,
    W: V,
    mut lwork: W,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnChegvd_bufferSize(
            handle,
            itype,
            jobz,
            uplo,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            B.as_const_ptr() as *const _,
            ldb as _,
            W.as_const_ptr() as *const _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZhegvd_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    A: T,
    lda: i32,
    B: U,
    ldb: i32,
    W: V,
    mut lwork: W,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZhegvd_bufferSize(
            handle,
            itype,
            jobz,
            uplo,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            B.as_const_ptr() as *const _,
            ldb as _,
            W.as_const_ptr() as *const _,
            lwork.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnSsygvd<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    mut B: U,
    ldb: i32,
    mut W: V,
    mut work: W,
    lwork: i32,
    mut info: X,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSsygvd(
            handle,
            itype,
            jobz,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            B.as_mut_ptr() as *mut _,
            ldb as _,
            W.as_mut_ptr() as *mut _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDsygvd<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    mut B: U,
    ldb: i32,
    mut W: V,
    mut work: W,
    lwork: i32,
    mut info: X,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDsygvd(
            handle,
            itype,
            jobz,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            B.as_mut_ptr() as *mut _,
            ldb as _,
            W.as_mut_ptr() as *mut _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnChegvd<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    mut B: U,
    ldb: i32,
    mut W: V,
    mut work: W,
    lwork: i32,
    mut info: X,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnChegvd(
            handle,
            itype,
            jobz,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            B.as_mut_ptr() as *mut _,
            ldb as _,
            W.as_mut_ptr() as *mut _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZhegvd<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    mut B: U,
    ldb: i32,
    mut W: V,
    mut work: W,
    lwork: i32,
    mut info: X,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZhegvd(
            handle,
            itype,
            jobz,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            B.as_mut_ptr() as *mut _,
            ldb as _,
            W.as_mut_ptr() as *mut _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnXsygvd_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            params,
            itype,
            jobz,
            uplo,
            n,
            dataTypeA,
            d_A.as_const_ptr() as *const _,
            lda,
            dataTypeB,
            d_B.as_const_ptr() as *const _,
            ldb,
            dataTypeW,
            d_W.as_const_ptr() as *const _,
            computeType,
            workspaceInBytesOnDevice.as_mut_ptr() as *mut _,
            workspaceInBytesOnHost.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnXsygvd<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            params,
            itype,
            jobz,
            uplo,
            n,
            dataTypeA,
            d_A.as_mut_ptr() as *mut _,
            lda,
            dataTypeB,
            d_B.as_mut_ptr() as *mut _,
            ldb,
            dataTypeW,
            d_W.as_mut_ptr() as *mut _,
            computeType,
            bufferOnDevice.as_mut_ptr() as *mut _,
            workspaceInBytesOnDevice,
            bufferOnHost.as_mut_ptr() as *mut _,
            workspaceInBytesOnHost,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnXsygvdx_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
    A: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            params,
            itype,
            jobz,
            uplo,
            n,
            dataTypeA,
            d_A.as_const_ptr() as *const _,
            lda,
            dataTypeB,
            d_B.as_const_ptr() as *const _,
            ldb,
            vl.as_mut_ptr() as *mut _,
            vu.as_mut_ptr() as *mut _,
            il,
            iu,
            meig.as_mut_ptr() as *mut _,
            dataTypeW,
            d_W.as_const_ptr() as *const _,
            computeType,
            workspaceInBytesOnDevice.as_mut_ptr() as *mut _,
            workspaceInBytesOnHost.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnXsygvdx<
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
    handle: cusolverDnHandle_t,
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
            handle,
            params,
            itype,
            jobz,
            range,
            uplo,
            n,
            dataTypeA,
            d_A.as_mut_ptr() as *mut _,
            lda,
            dataTypeB,
            d_B.as_mut_ptr() as *mut _,
            ldb,
            vl.as_mut_ptr() as *mut _,
            vu.as_mut_ptr() as *mut _,
            il,
            iu,
            meig.as_mut_ptr() as *mut _,
            dataTypeW,
            d_W.as_mut_ptr() as *mut _,
            computeType,
            bufferOnDevice.as_mut_ptr() as *mut _,
            workspaceInBytesOnDevice,
            bufferOnHost.as_mut_ptr() as *mut _,
            workspaceInBytesOnHost,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnCreateSyevjInfo() -> Result<syevjInfo_t, crate::sys::cusolverStatus_t> {
    let mut out_0: std::mem::MaybeUninit<syevjInfo_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cusolverDnCreateSyevjInfo(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as syevjInfo_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusolverDnDestroySyevjInfo(info: syevjInfo_t) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnDestroySyevjInfo(info) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
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
    max_sweeps: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnXsyevjSetMaxSweeps(info, max_sweeps as _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnXsyevjSetSortEig(info: syevjInfo_t, sort_eig: i32) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnXsyevjSetSortEig(info, sort_eig as _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnXsyevjGetResidual(
    handle: cusolverDnHandle_t,
    info: syevjInfo_t,
) -> Result<f64, crate::sys::cusolverStatus_t> {
    let mut out_2: std::mem::MaybeUninit<f64> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cusolverDnXsyevjGetResidual(handle, info, out_2.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS as usize {
        unsafe { Ok(out_2.assume_init() as f64) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusolverDnXsyevjGetSweeps(
    handle: cusolverDnHandle_t,
    info: syevjInfo_t,
) -> Result<i32, crate::sys::cusolverStatus_t> {
    let mut out_2: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cusolverDnXsyevjGetSweeps(handle, info, out_2.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS as usize {
        unsafe { Ok(out_2.assume_init() as i32) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusolverDnSsyevjBatched_bufferSize<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    A: T,
    lda: i32,
    W: U,
    mut lwork: V,
    params: syevjInfo_t,
    batchSize: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSsyevjBatched_bufferSize(
            handle,
            jobz,
            uplo,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            W.as_const_ptr() as *const _,
            lwork.as_mut_ptr() as *mut _,
            params,
            batchSize as _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDsyevjBatched_bufferSize<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    A: T,
    lda: i32,
    W: U,
    mut lwork: V,
    params: syevjInfo_t,
    batchSize: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDsyevjBatched_bufferSize(
            handle,
            jobz,
            uplo,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            W.as_const_ptr() as *const _,
            lwork.as_mut_ptr() as *mut _,
            params,
            batchSize as _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnCheevjBatched_bufferSize<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    A: T,
    lda: i32,
    W: U,
    mut lwork: V,
    params: syevjInfo_t,
    batchSize: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCheevjBatched_bufferSize(
            handle,
            jobz,
            uplo,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            W.as_const_ptr() as *const _,
            lwork.as_mut_ptr() as *mut _,
            params,
            batchSize as _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZheevjBatched_bufferSize<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    A: T,
    lda: i32,
    W: U,
    mut lwork: V,
    params: syevjInfo_t,
    batchSize: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZheevjBatched_bufferSize(
            handle,
            jobz,
            uplo,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            W.as_const_ptr() as *const _,
            lwork.as_mut_ptr() as *mut _,
            params,
            batchSize as _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnSsyevjBatched<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    mut W: U,
    mut work: V,
    lwork: i32,
    mut info: W,
    params: syevjInfo_t,
    batchSize: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSsyevjBatched(
            handle,
            jobz,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            W.as_mut_ptr() as *mut _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
            params,
            batchSize as _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDsyevjBatched<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    mut W: U,
    mut work: V,
    lwork: i32,
    mut info: W,
    params: syevjInfo_t,
    batchSize: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDsyevjBatched(
            handle,
            jobz,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            W.as_mut_ptr() as *mut _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
            params,
            batchSize as _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnCheevjBatched<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    mut W: U,
    mut work: V,
    lwork: i32,
    mut info: W,
    params: syevjInfo_t,
    batchSize: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCheevjBatched(
            handle,
            jobz,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            W.as_mut_ptr() as *mut _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
            params,
            batchSize as _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZheevjBatched<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    mut W: U,
    mut work: V,
    lwork: i32,
    mut info: W,
    params: syevjInfo_t,
    batchSize: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZheevjBatched(
            handle,
            jobz,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            W.as_mut_ptr() as *mut _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
            params,
            batchSize as _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnSsyevj_bufferSize<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    A: T,
    lda: i32,
    W: U,
    mut lwork: V,
    params: syevjInfo_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSsyevj_bufferSize(
            handle,
            jobz,
            uplo,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            W.as_const_ptr() as *const _,
            lwork.as_mut_ptr() as *mut _,
            params,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDsyevj_bufferSize<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    A: T,
    lda: i32,
    W: U,
    mut lwork: V,
    params: syevjInfo_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDsyevj_bufferSize(
            handle,
            jobz,
            uplo,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            W.as_const_ptr() as *const _,
            lwork.as_mut_ptr() as *mut _,
            params,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnCheevj_bufferSize<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    A: T,
    lda: i32,
    W: U,
    mut lwork: V,
    params: syevjInfo_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCheevj_bufferSize(
            handle,
            jobz,
            uplo,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            W.as_const_ptr() as *const _,
            lwork.as_mut_ptr() as *mut _,
            params,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZheevj_bufferSize<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    A: T,
    lda: i32,
    W: U,
    mut lwork: V,
    params: syevjInfo_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZheevj_bufferSize(
            handle,
            jobz,
            uplo,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            W.as_const_ptr() as *const _,
            lwork.as_mut_ptr() as *mut _,
            params,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnSsyevj<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    mut W: U,
    mut work: V,
    lwork: i32,
    mut info: W,
    params: syevjInfo_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSsyevj(
            handle,
            jobz,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            W.as_mut_ptr() as *mut _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
            params,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDsyevj<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    mut W: U,
    mut work: V,
    lwork: i32,
    mut info: W,
    params: syevjInfo_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDsyevj(
            handle,
            jobz,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            W.as_mut_ptr() as *mut _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
            params,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnCheevj<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    mut W: U,
    mut work: V,
    lwork: i32,
    mut info: W,
    params: syevjInfo_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCheevj(
            handle,
            jobz,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            W.as_mut_ptr() as *mut _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
            params,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZheevj<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    mut W: U,
    mut work: V,
    lwork: i32,
    mut info: W,
    params: syevjInfo_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZheevj(
            handle,
            jobz,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            W.as_mut_ptr() as *mut _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
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
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    A: T,
    lda: i32,
    B: U,
    ldb: i32,
    W: V,
    mut lwork: W,
    params: syevjInfo_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSsygvj_bufferSize(
            handle,
            itype,
            jobz,
            uplo,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            B.as_const_ptr() as *const _,
            ldb as _,
            W.as_const_ptr() as *const _,
            lwork.as_mut_ptr() as *mut _,
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
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    A: T,
    lda: i32,
    B: U,
    ldb: i32,
    W: V,
    mut lwork: W,
    params: syevjInfo_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDsygvj_bufferSize(
            handle,
            itype,
            jobz,
            uplo,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            B.as_const_ptr() as *const _,
            ldb as _,
            W.as_const_ptr() as *const _,
            lwork.as_mut_ptr() as *mut _,
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
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    A: T,
    lda: i32,
    B: U,
    ldb: i32,
    W: V,
    mut lwork: W,
    params: syevjInfo_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnChegvj_bufferSize(
            handle,
            itype,
            jobz,
            uplo,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            B.as_const_ptr() as *const _,
            ldb as _,
            W.as_const_ptr() as *const _,
            lwork.as_mut_ptr() as *mut _,
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
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    A: T,
    lda: i32,
    B: U,
    ldb: i32,
    W: V,
    mut lwork: W,
    params: syevjInfo_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZhegvj_bufferSize(
            handle,
            itype,
            jobz,
            uplo,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            B.as_const_ptr() as *const _,
            ldb as _,
            W.as_const_ptr() as *const _,
            lwork.as_mut_ptr() as *mut _,
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
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    mut B: U,
    ldb: i32,
    mut W: V,
    mut work: W,
    lwork: i32,
    mut info: X,
    params: syevjInfo_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSsygvj(
            handle,
            itype,
            jobz,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            B.as_mut_ptr() as *mut _,
            ldb as _,
            W.as_mut_ptr() as *mut _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
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
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    mut B: U,
    ldb: i32,
    mut W: V,
    mut work: W,
    lwork: i32,
    mut info: X,
    params: syevjInfo_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDsygvj(
            handle,
            itype,
            jobz,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            B.as_mut_ptr() as *mut _,
            ldb as _,
            W.as_mut_ptr() as *mut _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
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
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    mut B: U,
    ldb: i32,
    mut W: V,
    mut work: W,
    lwork: i32,
    mut info: X,
    params: syevjInfo_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnChegvj(
            handle,
            itype,
            jobz,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            B.as_mut_ptr() as *mut _,
            ldb as _,
            W.as_mut_ptr() as *mut _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
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
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i32,
    mut A: T,
    lda: i32,
    mut B: U,
    ldb: i32,
    mut W: V,
    mut work: W,
    lwork: i32,
    mut info: X,
    params: syevjInfo_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZhegvj(
            handle,
            itype,
            jobz,
            uplo,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            B.as_mut_ptr() as *mut _,
            ldb as _,
            W.as_mut_ptr() as *mut _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
            params,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnCreateGesvdjInfo() -> Result<gesvdjInfo_t, crate::sys::cusolverStatus_t> {
    let mut out_0: std::mem::MaybeUninit<gesvdjInfo_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cusolverDnCreateGesvdjInfo(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as gesvdjInfo_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusolverDnDestroyGesvdjInfo(info: gesvdjInfo_t) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnDestroyGesvdjInfo(info) };
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
    max_sweeps: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnXgesvdjSetMaxSweeps(info, max_sweeps as _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnXgesvdjSetSortEig(
    info: gesvdjInfo_t,
    sort_svd: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnXgesvdjSetSortEig(info, sort_svd as _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnXgesvdjGetResidual(
    handle: cusolverDnHandle_t,
    info: gesvdjInfo_t,
) -> Result<f64, crate::sys::cusolverStatus_t> {
    let mut out_2: std::mem::MaybeUninit<f64> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cusolverDnXgesvdjGetResidual(handle, info, out_2.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS as usize {
        unsafe { Ok(out_2.assume_init() as f64) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusolverDnXgesvdjGetSweeps(
    handle: cusolverDnHandle_t,
    info: gesvdjInfo_t,
) -> Result<i32, crate::sys::cusolverStatus_t> {
    let mut out_2: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cusolverDnXgesvdjGetSweeps(handle, info, out_2.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS as usize {
        unsafe { Ok(out_2.assume_init() as i32) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusolverDnSgesvdjBatched_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    m: i32,
    n: i32,
    A: T,
    lda: i32,
    S: U,
    U: V,
    ldu: i32,
    V: W,
    ldv: i32,
    mut lwork: X,
    params: gesvdjInfo_t,
    batchSize: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSgesvdjBatched_bufferSize(
            handle,
            jobz,
            m as _,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            S.as_const_ptr() as *const _,
            U.as_const_ptr() as *const _,
            ldu as _,
            V.as_const_ptr() as *const _,
            ldv as _,
            lwork.as_mut_ptr() as *mut _,
            params,
            batchSize as _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDgesvdjBatched_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    m: i32,
    n: i32,
    A: T,
    lda: i32,
    S: U,
    U: V,
    ldu: i32,
    V: W,
    ldv: i32,
    mut lwork: X,
    params: gesvdjInfo_t,
    batchSize: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDgesvdjBatched_bufferSize(
            handle,
            jobz,
            m as _,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            S.as_const_ptr() as *const _,
            U.as_const_ptr() as *const _,
            ldu as _,
            V.as_const_ptr() as *const _,
            ldv as _,
            lwork.as_mut_ptr() as *mut _,
            params,
            batchSize as _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnCgesvdjBatched_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    m: i32,
    n: i32,
    A: T,
    lda: i32,
    S: U,
    U: V,
    ldu: i32,
    V: W,
    ldv: i32,
    mut lwork: X,
    params: gesvdjInfo_t,
    batchSize: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCgesvdjBatched_bufferSize(
            handle,
            jobz,
            m as _,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            S.as_const_ptr() as *const _,
            U.as_const_ptr() as *const _,
            ldu as _,
            V.as_const_ptr() as *const _,
            ldv as _,
            lwork.as_mut_ptr() as *mut _,
            params,
            batchSize as _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZgesvdjBatched_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    m: i32,
    n: i32,
    A: T,
    lda: i32,
    S: U,
    U: V,
    ldu: i32,
    V: W,
    ldv: i32,
    mut lwork: X,
    params: gesvdjInfo_t,
    batchSize: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZgesvdjBatched_bufferSize(
            handle,
            jobz,
            m as _,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            S.as_const_ptr() as *const _,
            U.as_const_ptr() as *const _,
            ldu as _,
            V.as_const_ptr() as *const _,
            ldv as _,
            lwork.as_mut_ptr() as *mut _,
            params,
            batchSize as _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnSgesvdjBatched<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    m: i32,
    n: i32,
    mut A: T,
    lda: i32,
    mut S: U,
    mut U: V,
    ldu: i32,
    mut V: W,
    ldv: i32,
    mut work: X,
    lwork: i32,
    mut info: Y,
    params: gesvdjInfo_t,
    batchSize: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSgesvdjBatched(
            handle,
            jobz,
            m as _,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            S.as_mut_ptr() as *mut _,
            U.as_mut_ptr() as *mut _,
            ldu as _,
            V.as_mut_ptr() as *mut _,
            ldv as _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
            params,
            batchSize as _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDgesvdjBatched<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    m: i32,
    n: i32,
    mut A: T,
    lda: i32,
    mut S: U,
    mut U: V,
    ldu: i32,
    mut V: W,
    ldv: i32,
    mut work: X,
    lwork: i32,
    mut info: Y,
    params: gesvdjInfo_t,
    batchSize: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDgesvdjBatched(
            handle,
            jobz,
            m as _,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            S.as_mut_ptr() as *mut _,
            U.as_mut_ptr() as *mut _,
            ldu as _,
            V.as_mut_ptr() as *mut _,
            ldv as _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
            params,
            batchSize as _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnCgesvdjBatched<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    m: i32,
    n: i32,
    mut A: T,
    lda: i32,
    mut S: U,
    mut U: V,
    ldu: i32,
    mut V: W,
    ldv: i32,
    mut work: X,
    lwork: i32,
    mut info: Y,
    params: gesvdjInfo_t,
    batchSize: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCgesvdjBatched(
            handle,
            jobz,
            m as _,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            S.as_mut_ptr() as *mut _,
            U.as_mut_ptr() as *mut _,
            ldu as _,
            V.as_mut_ptr() as *mut _,
            ldv as _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
            params,
            batchSize as _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZgesvdjBatched<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    m: i32,
    n: i32,
    mut A: T,
    lda: i32,
    mut S: U,
    mut U: V,
    ldu: i32,
    mut V: W,
    ldv: i32,
    mut work: X,
    lwork: i32,
    mut info: Y,
    params: gesvdjInfo_t,
    batchSize: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZgesvdjBatched(
            handle,
            jobz,
            m as _,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            S.as_mut_ptr() as *mut _,
            U.as_mut_ptr() as *mut _,
            ldu as _,
            V.as_mut_ptr() as *mut _,
            ldv as _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
            params,
            batchSize as _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnSgesvdj_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    econ: i32,
    m: i32,
    n: i32,
    A: T,
    lda: i32,
    S: U,
    U: V,
    ldu: i32,
    V: W,
    ldv: i32,
    mut lwork: X,
    params: gesvdjInfo_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSgesvdj_bufferSize(
            handle,
            jobz,
            econ as _,
            m as _,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            S.as_const_ptr() as *const _,
            U.as_const_ptr() as *const _,
            ldu as _,
            V.as_const_ptr() as *const _,
            ldv as _,
            lwork.as_mut_ptr() as *mut _,
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
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    econ: i32,
    m: i32,
    n: i32,
    A: T,
    lda: i32,
    S: U,
    U: V,
    ldu: i32,
    V: W,
    ldv: i32,
    mut lwork: X,
    params: gesvdjInfo_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDgesvdj_bufferSize(
            handle,
            jobz,
            econ as _,
            m as _,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            S.as_const_ptr() as *const _,
            U.as_const_ptr() as *const _,
            ldu as _,
            V.as_const_ptr() as *const _,
            ldv as _,
            lwork.as_mut_ptr() as *mut _,
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
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    econ: i32,
    m: i32,
    n: i32,
    A: T,
    lda: i32,
    S: U,
    U: V,
    ldu: i32,
    V: W,
    ldv: i32,
    mut lwork: X,
    params: gesvdjInfo_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCgesvdj_bufferSize(
            handle,
            jobz,
            econ as _,
            m as _,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            S.as_const_ptr() as *const _,
            U.as_const_ptr() as *const _,
            ldu as _,
            V.as_const_ptr() as *const _,
            ldv as _,
            lwork.as_mut_ptr() as *mut _,
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
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    econ: i32,
    m: i32,
    n: i32,
    A: T,
    lda: i32,
    S: U,
    U: V,
    ldu: i32,
    V: W,
    ldv: i32,
    mut lwork: X,
    params: gesvdjInfo_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZgesvdj_bufferSize(
            handle,
            jobz,
            econ as _,
            m as _,
            n as _,
            A.as_const_ptr() as *const _,
            lda as _,
            S.as_const_ptr() as *const _,
            U.as_const_ptr() as *const _,
            ldu as _,
            V.as_const_ptr() as *const _,
            ldv as _,
            lwork.as_mut_ptr() as *mut _,
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
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    econ: i32,
    m: i32,
    n: i32,
    mut A: T,
    lda: i32,
    mut S: U,
    mut U: V,
    ldu: i32,
    mut V: W,
    ldv: i32,
    mut work: X,
    lwork: i32,
    mut info: Y,
    params: gesvdjInfo_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSgesvdj(
            handle,
            jobz,
            econ as _,
            m as _,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            S.as_mut_ptr() as *mut _,
            U.as_mut_ptr() as *mut _,
            ldu as _,
            V.as_mut_ptr() as *mut _,
            ldv as _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
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
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    econ: i32,
    m: i32,
    n: i32,
    mut A: T,
    lda: i32,
    mut S: U,
    mut U: V,
    ldu: i32,
    mut V: W,
    ldv: i32,
    mut work: X,
    lwork: i32,
    mut info: Y,
    params: gesvdjInfo_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDgesvdj(
            handle,
            jobz,
            econ as _,
            m as _,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            S.as_mut_ptr() as *mut _,
            U.as_mut_ptr() as *mut _,
            ldu as _,
            V.as_mut_ptr() as *mut _,
            ldv as _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
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
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    econ: i32,
    m: i32,
    n: i32,
    mut A: T,
    lda: i32,
    mut S: U,
    mut U: V,
    ldu: i32,
    mut V: W,
    ldv: i32,
    mut work: X,
    lwork: i32,
    mut info: Y,
    params: gesvdjInfo_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCgesvdj(
            handle,
            jobz,
            econ as _,
            m as _,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            S.as_mut_ptr() as *mut _,
            U.as_mut_ptr() as *mut _,
            ldu as _,
            V.as_mut_ptr() as *mut _,
            ldv as _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
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
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    econ: i32,
    m: i32,
    n: i32,
    mut A: T,
    lda: i32,
    mut S: U,
    mut U: V,
    ldu: i32,
    mut V: W,
    ldv: i32,
    mut work: X,
    lwork: i32,
    mut info: Y,
    params: gesvdjInfo_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZgesvdj(
            handle,
            jobz,
            econ as _,
            m as _,
            n as _,
            A.as_mut_ptr() as *mut _,
            lda as _,
            S.as_mut_ptr() as *mut _,
            U.as_mut_ptr() as *mut _,
            ldu as _,
            V.as_mut_ptr() as *mut _,
            ldv as _,
            work.as_mut_ptr() as *mut _,
            lwork as _,
            info.as_mut_ptr() as *mut _,
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
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    rank: i32,
    m: i32,
    n: i32,
    d_A: T,
    lda: i32,
    strideA: i64,
    d_S: U,
    strideS: i64,
    d_U: V,
    ldu: i32,
    strideU: i64,
    d_V: W,
    ldv: i32,
    strideV: i64,
    mut lwork: X,
    batchSize: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSgesvdaStridedBatched_bufferSize(
            handle,
            jobz,
            rank as _,
            m as _,
            n as _,
            d_A.as_const_ptr() as *const _,
            lda as _,
            strideA as _,
            d_S.as_const_ptr() as *const _,
            strideS as _,
            d_U.as_const_ptr() as *const _,
            ldu as _,
            strideU as _,
            d_V.as_const_ptr() as *const _,
            ldv as _,
            strideV as _,
            lwork.as_mut_ptr() as *mut _,
            batchSize as _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDgesvdaStridedBatched_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    rank: i32,
    m: i32,
    n: i32,
    d_A: T,
    lda: i32,
    strideA: i64,
    d_S: U,
    strideS: i64,
    d_U: V,
    ldu: i32,
    strideU: i64,
    d_V: W,
    ldv: i32,
    strideV: i64,
    mut lwork: X,
    batchSize: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDgesvdaStridedBatched_bufferSize(
            handle,
            jobz,
            rank as _,
            m as _,
            n as _,
            d_A.as_const_ptr() as *const _,
            lda as _,
            strideA as _,
            d_S.as_const_ptr() as *const _,
            strideS as _,
            d_U.as_const_ptr() as *const _,
            ldu as _,
            strideU as _,
            d_V.as_const_ptr() as *const _,
            ldv as _,
            strideV as _,
            lwork.as_mut_ptr() as *mut _,
            batchSize as _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnCgesvdaStridedBatched_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    rank: i32,
    m: i32,
    n: i32,
    d_A: T,
    lda: i32,
    strideA: i64,
    d_S: U,
    strideS: i64,
    d_U: V,
    ldu: i32,
    strideU: i64,
    d_V: W,
    ldv: i32,
    strideV: i64,
    mut lwork: X,
    batchSize: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCgesvdaStridedBatched_bufferSize(
            handle,
            jobz,
            rank as _,
            m as _,
            n as _,
            d_A.as_const_ptr() as *const _,
            lda as _,
            strideA as _,
            d_S.as_const_ptr() as *const _,
            strideS as _,
            d_U.as_const_ptr() as *const _,
            ldu as _,
            strideU as _,
            d_V.as_const_ptr() as *const _,
            ldv as _,
            strideV as _,
            lwork.as_mut_ptr() as *mut _,
            batchSize as _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZgesvdaStridedBatched_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    rank: i32,
    m: i32,
    n: i32,
    d_A: T,
    lda: i32,
    strideA: i64,
    d_S: U,
    strideS: i64,
    d_U: V,
    ldu: i32,
    strideU: i64,
    d_V: W,
    ldv: i32,
    strideV: i64,
    mut lwork: X,
    batchSize: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZgesvdaStridedBatched_bufferSize(
            handle,
            jobz,
            rank as _,
            m as _,
            n as _,
            d_A.as_const_ptr() as *const _,
            lda as _,
            strideA as _,
            d_S.as_const_ptr() as *const _,
            strideS as _,
            d_U.as_const_ptr() as *const _,
            ldu as _,
            strideU as _,
            d_V.as_const_ptr() as *const _,
            ldv as _,
            strideV as _,
            lwork.as_mut_ptr() as *mut _,
            batchSize as _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnSgesvdaStridedBatched<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    rank: i32,
    m: i32,
    n: i32,
    d_A: T,
    lda: i32,
    strideA: i64,
    mut d_S: U,
    strideS: i64,
    mut d_U: V,
    ldu: i32,
    strideU: i64,
    mut d_V: W,
    ldv: i32,
    strideV: i64,
    mut d_work: X,
    lwork: i32,
    mut d_info: Y,
    mut h_R_nrmF: Z,
    batchSize: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnSgesvdaStridedBatched(
            handle,
            jobz,
            rank as _,
            m as _,
            n as _,
            d_A.as_const_ptr() as *const _,
            lda as _,
            strideA as _,
            d_S.as_mut_ptr() as *mut _,
            strideS as _,
            d_U.as_mut_ptr() as *mut _,
            ldu as _,
            strideU as _,
            d_V.as_mut_ptr() as *mut _,
            ldv as _,
            strideV as _,
            d_work.as_mut_ptr() as *mut _,
            lwork as _,
            d_info.as_mut_ptr() as *mut _,
            h_R_nrmF.as_mut_ptr() as *mut _,
            batchSize as _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnDgesvdaStridedBatched<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    rank: i32,
    m: i32,
    n: i32,
    d_A: T,
    lda: i32,
    strideA: i64,
    mut d_S: U,
    strideS: i64,
    mut d_U: V,
    ldu: i32,
    strideU: i64,
    mut d_V: W,
    ldv: i32,
    strideV: i64,
    mut d_work: X,
    lwork: i32,
    mut d_info: Y,
    mut h_R_nrmF: Z,
    batchSize: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnDgesvdaStridedBatched(
            handle,
            jobz,
            rank as _,
            m as _,
            n as _,
            d_A.as_const_ptr() as *const _,
            lda as _,
            strideA as _,
            d_S.as_mut_ptr() as *mut _,
            strideS as _,
            d_U.as_mut_ptr() as *mut _,
            ldu as _,
            strideU as _,
            d_V.as_mut_ptr() as *mut _,
            ldv as _,
            strideV as _,
            d_work.as_mut_ptr() as *mut _,
            lwork as _,
            d_info.as_mut_ptr() as *mut _,
            h_R_nrmF.as_mut_ptr() as *mut _,
            batchSize as _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnCgesvdaStridedBatched<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    rank: i32,
    m: i32,
    n: i32,
    d_A: T,
    lda: i32,
    strideA: i64,
    mut d_S: U,
    strideS: i64,
    mut d_U: V,
    ldu: i32,
    strideU: i64,
    mut d_V: W,
    ldv: i32,
    strideV: i64,
    mut d_work: X,
    lwork: i32,
    mut d_info: Y,
    mut h_R_nrmF: Z,
    batchSize: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnCgesvdaStridedBatched(
            handle,
            jobz,
            rank as _,
            m as _,
            n as _,
            d_A.as_const_ptr() as *const _,
            lda as _,
            strideA as _,
            d_S.as_mut_ptr() as *mut _,
            strideS as _,
            d_U.as_mut_ptr() as *mut _,
            ldu as _,
            strideU as _,
            d_V.as_mut_ptr() as *mut _,
            ldv as _,
            strideV as _,
            d_work.as_mut_ptr() as *mut _,
            lwork as _,
            d_info.as_mut_ptr() as *mut _,
            h_R_nrmF.as_mut_ptr() as *mut _,
            batchSize as _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnZgesvdaStridedBatched<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    rank: i32,
    m: i32,
    n: i32,
    d_A: T,
    lda: i32,
    strideA: i64,
    mut d_S: U,
    strideS: i64,
    mut d_U: V,
    ldu: i32,
    strideU: i64,
    mut d_V: W,
    ldv: i32,
    strideV: i64,
    mut d_work: X,
    lwork: i32,
    mut d_info: Y,
    mut h_R_nrmF: Z,
    batchSize: i32,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverDnZgesvdaStridedBatched(
            handle,
            jobz,
            rank as _,
            m as _,
            n as _,
            d_A.as_const_ptr() as *const _,
            lda as _,
            strideA as _,
            d_S.as_mut_ptr() as *mut _,
            strideS as _,
            d_U.as_mut_ptr() as *mut _,
            ldu as _,
            strideU as _,
            d_V.as_mut_ptr() as *mut _,
            ldv as _,
            strideV as _,
            d_work.as_mut_ptr() as *mut _,
            lwork as _,
            d_info.as_mut_ptr() as *mut _,
            h_R_nrmF.as_mut_ptr() as *mut _,
            batchSize as _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnCreateParams() -> Result<cusolverDnParams_t, crate::sys::cusolverStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cusolverDnParams_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cusolverDnCreateParams(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cusolverDnParams_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusolverDnDestroyParams(params: cusolverDnParams_t) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnDestroyParams(params) };
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
pub unsafe fn cusolverDnXpotrf_bufferSize<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
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
            handle,
            params,
            uplo,
            n,
            dataTypeA,
            A.as_const_ptr() as *const _,
            lda,
            computeType,
            workspaceInBytesOnDevice.as_mut_ptr() as *mut _,
            workspaceInBytesOnHost.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnXpotrf<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
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
            handle,
            params,
            uplo,
            n,
            dataTypeA,
            A.as_mut_ptr() as *mut _,
            lda,
            computeType,
            bufferOnDevice.as_mut_ptr() as *mut _,
            workspaceInBytesOnDevice,
            bufferOnHost.as_mut_ptr() as *mut _,
            workspaceInBytesOnHost,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnXpotrs<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
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
            handle,
            params,
            uplo,
            n,
            nrhs,
            dataTypeA,
            A.as_const_ptr() as *const _,
            lda,
            dataTypeB,
            B.as_mut_ptr() as *mut _,
            ldb,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnXgeqrf_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            params,
            m,
            n,
            dataTypeA,
            A.as_const_ptr() as *const _,
            lda,
            dataTypeTau,
            tau.as_const_ptr() as *const _,
            computeType,
            workspaceInBytesOnDevice.as_mut_ptr() as *mut _,
            workspaceInBytesOnHost.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnXgeqrf<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            params,
            m,
            n,
            dataTypeA,
            A.as_mut_ptr() as *mut _,
            lda,
            dataTypeTau,
            tau.as_mut_ptr() as *mut _,
            computeType,
            bufferOnDevice.as_mut_ptr() as *mut _,
            workspaceInBytesOnDevice,
            bufferOnHost.as_mut_ptr() as *mut _,
            workspaceInBytesOnHost,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnXgetrf_bufferSize<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
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
            handle,
            params,
            m,
            n,
            dataTypeA,
            A.as_const_ptr() as *const _,
            lda,
            computeType,
            workspaceInBytesOnDevice.as_mut_ptr() as *mut _,
            workspaceInBytesOnHost.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnXgetrf<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            params,
            m,
            n,
            dataTypeA,
            A.as_mut_ptr() as *mut _,
            lda,
            ipiv.as_mut_ptr() as *mut _,
            computeType,
            bufferOnDevice.as_mut_ptr() as *mut _,
            workspaceInBytesOnDevice,
            bufferOnHost.as_mut_ptr() as *mut _,
            workspaceInBytesOnHost,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnXgetrs<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cusolverDnHandle_t,
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
            handle,
            params,
            trans,
            n,
            nrhs,
            dataTypeA,
            A.as_const_ptr() as *const _,
            lda,
            ipiv.as_const_ptr() as *const _,
            dataTypeB,
            B.as_mut_ptr() as *mut _,
            ldb,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnXsyevd_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            params,
            jobz,
            uplo,
            n,
            dataTypeA,
            A.as_const_ptr() as *const _,
            lda,
            dataTypeW,
            W.as_const_ptr() as *const _,
            computeType,
            workspaceInBytesOnDevice.as_mut_ptr() as *mut _,
            workspaceInBytesOnHost.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnXsyevd<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            params,
            jobz,
            uplo,
            n,
            dataTypeA,
            A.as_mut_ptr() as *mut _,
            lda,
            dataTypeW,
            W.as_mut_ptr() as *mut _,
            computeType,
            bufferOnDevice.as_mut_ptr() as *mut _,
            workspaceInBytesOnDevice,
            bufferOnHost.as_mut_ptr() as *mut _,
            workspaceInBytesOnHost,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnXstedc_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            params,
            compz,
            n,
            dataTypeDE,
            D.as_const_ptr() as *const _,
            E.as_const_ptr() as *const _,
            dataTypeZ,
            Z.as_const_ptr() as *const _,
            ldz,
            computeType,
            workspaceInBytesOnDevice.as_mut_ptr() as *mut _,
            workspaceInBytesOnHost.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnXstedc<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            params,
            compz,
            n,
            dataTypeDE,
            D.as_mut_ptr() as *mut _,
            E.as_mut_ptr() as *mut _,
            dataTypeZ,
            Z.as_mut_ptr() as *mut _,
            ldz,
            computeType,
            bufferOnDevice.as_mut_ptr() as *mut _,
            workspaceInBytesOnDevice,
            bufferOnHost.as_mut_ptr() as *mut _,
            workspaceInBytesOnHost,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnXsyevBatched_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            params,
            jobz,
            uplo,
            n,
            dataTypeA,
            A.as_const_ptr() as *const _,
            lda,
            dataTypeW,
            W.as_const_ptr() as *const _,
            computeType,
            workspaceInBytesOnDevice.as_mut_ptr() as *mut _,
            workspaceInBytesOnHost.as_mut_ptr() as *mut _,
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
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            params,
            jobz,
            uplo,
            n,
            dataTypeA,
            A.as_mut_ptr() as *mut _,
            lda,
            dataTypeW,
            W.as_mut_ptr() as *mut _,
            computeType,
            bufferOnDevice.as_mut_ptr() as *mut _,
            workspaceInBytesOnDevice,
            bufferOnHost.as_mut_ptr() as *mut _,
            workspaceInBytesOnHost,
            info.as_mut_ptr() as *mut _,
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
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            params,
            jobz,
            range,
            uplo,
            n,
            dataTypeA,
            A.as_const_ptr() as *const _,
            lda,
            vl.as_mut_ptr() as *mut _,
            vu.as_mut_ptr() as *mut _,
            il,
            iu,
            h_meig.as_mut_ptr() as *mut _,
            dataTypeW,
            W.as_const_ptr() as *const _,
            computeType,
            workspaceInBytesOnDevice.as_mut_ptr() as *mut _,
            workspaceInBytesOnHost.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnXsyevdx<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
    A: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            params,
            jobz,
            range,
            uplo,
            n,
            dataTypeA,
            A.as_mut_ptr() as *mut _,
            lda,
            vl.as_mut_ptr() as *mut _,
            vu.as_mut_ptr() as *mut _,
            il,
            iu,
            meig64.as_mut_ptr() as *mut _,
            dataTypeW,
            W.as_mut_ptr() as *mut _,
            computeType,
            bufferOnDevice.as_mut_ptr() as *mut _,
            workspaceInBytesOnDevice,
            bufferOnHost.as_mut_ptr() as *mut _,
            workspaceInBytesOnHost,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnXgeev_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            params,
            jobvl,
            jobvr,
            n,
            dataTypeA,
            A.as_const_ptr() as *const _,
            lda,
            dataTypeW,
            W.as_const_ptr() as *const _,
            dataTypeVL,
            VL.as_const_ptr() as *const _,
            ldvl,
            dataTypeVR,
            VR.as_const_ptr() as *const _,
            ldvr,
            computeType,
            workspaceInBytesOnDevice.as_mut_ptr() as *mut _,
            workspaceInBytesOnHost.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnXgeev<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            params,
            jobvl,
            jobvr,
            n,
            dataTypeA,
            A.as_mut_ptr() as *mut _,
            lda,
            dataTypeW,
            W.as_mut_ptr() as *mut _,
            dataTypeVL,
            VL.as_mut_ptr() as *mut _,
            ldvl,
            dataTypeVR,
            VR.as_mut_ptr() as *mut _,
            ldvr,
            computeType,
            bufferOnDevice.as_mut_ptr() as *mut _,
            workspaceInBytesOnDevice,
            bufferOnHost.as_mut_ptr() as *mut _,
            workspaceInBytesOnHost,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnXgesvd_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            params,
            jobu,
            jobvt,
            m,
            n,
            dataTypeA,
            A.as_const_ptr() as *const _,
            lda,
            dataTypeS,
            S.as_const_ptr() as *const _,
            dataTypeU,
            U.as_const_ptr() as *const _,
            ldu,
            dataTypeVT,
            VT.as_const_ptr() as *const _,
            ldvt,
            computeType,
            workspaceInBytesOnDevice.as_mut_ptr() as *mut _,
            workspaceInBytesOnHost.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnXgesvd<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            params,
            jobu,
            jobvt,
            m,
            n,
            dataTypeA,
            A.as_mut_ptr() as *mut _,
            lda,
            dataTypeS,
            S.as_mut_ptr() as *mut _,
            dataTypeU,
            U.as_mut_ptr() as *mut _,
            ldu,
            dataTypeVT,
            VT.as_mut_ptr() as *mut _,
            ldvt,
            computeType,
            bufferOnDevice.as_mut_ptr() as *mut _,
            workspaceInBytesOnDevice,
            bufferOnHost.as_mut_ptr() as *mut _,
            workspaceInBytesOnHost,
            info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnXgesvdp_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    jobz: cusolverEigMode_t,
    econ: i32,
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
            handle,
            params,
            jobz,
            econ as _,
            m,
            n,
            dataTypeA,
            A.as_const_ptr() as *const _,
            lda,
            dataTypeS,
            S.as_const_ptr() as *const _,
            dataTypeU,
            U.as_const_ptr() as *const _,
            ldu,
            dataTypeV,
            V.as_const_ptr() as *const _,
            ldv,
            computeType,
            workspaceInBytesOnDevice.as_mut_ptr() as *mut _,
            workspaceInBytesOnHost.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnXgesvdp<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
    A: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    jobz: cusolverEigMode_t,
    econ: i32,
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
            handle,
            params,
            jobz,
            econ as _,
            m,
            n,
            dataTypeA,
            A.as_mut_ptr() as *mut _,
            lda,
            dataTypeS,
            S.as_mut_ptr() as *mut _,
            dataTypeU,
            U.as_mut_ptr() as *mut _,
            ldu,
            dataTypeV,
            V.as_mut_ptr() as *mut _,
            ldv,
            computeType,
            bufferOnDevice.as_mut_ptr() as *mut _,
            workspaceInBytesOnDevice,
            bufferOnHost.as_mut_ptr() as *mut _,
            workspaceInBytesOnHost,
            d_info.as_mut_ptr() as *mut _,
            h_err_sigma.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnXgesvdr_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
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
            A.as_const_ptr() as *const _,
            lda,
            dataTypeSrand,
            Srand.as_const_ptr() as *const _,
            dataTypeUrand,
            Urand.as_const_ptr() as *const _,
            ldUrand,
            dataTypeVrand,
            Vrand.as_const_ptr() as *const _,
            ldVrand,
            computeType,
            workspaceInBytesOnDevice.as_mut_ptr() as *mut _,
            workspaceInBytesOnHost.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnXgesvdr<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
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
            A.as_mut_ptr() as *mut _,
            lda,
            dataTypeSrand,
            Srand.as_mut_ptr() as *mut _,
            dataTypeUrand,
            Urand.as_mut_ptr() as *mut _,
            ldUrand,
            dataTypeVrand,
            Vrand.as_mut_ptr() as *mut _,
            ldVrand,
            computeType,
            bufferOnDevice.as_mut_ptr() as *mut _,
            workspaceInBytesOnDevice,
            bufferOnHost.as_mut_ptr() as *mut _,
            workspaceInBytesOnHost,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnXlarft_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            params,
            direct,
            storev,
            n,
            k,
            dataTypeV,
            V.as_const_ptr() as *const _,
            ldv,
            dataTypeTau,
            tau.as_const_ptr() as *const _,
            dataTypeT,
            T.as_mut_ptr() as *mut _,
            ldt,
            computeType,
            workspaceInBytesOnDevice.as_mut_ptr() as *mut _,
            workspaceInBytesOnHost.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnXlarft<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            params,
            direct,
            storev,
            n,
            k,
            dataTypeV,
            V.as_const_ptr() as *const _,
            ldv,
            dataTypeTau,
            tau.as_const_ptr() as *const _,
            dataTypeT,
            T.as_mut_ptr() as *mut _,
            ldt,
            computeType,
            bufferOnDevice.as_mut_ptr() as *mut _,
            workspaceInBytesOnDevice,
            bufferOnHost.as_mut_ptr() as *mut _,
            workspaceInBytesOnHost,
        )
    };
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
pub unsafe fn cusolverDnLoggerSetFile<T: types::CudaAsPtr>(mut file: T) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnLoggerSetFile(file.as_mut_ptr() as *mut _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnLoggerOpenFile<T: types::CudaAsPtr>(logFile: T) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnLoggerOpenFile(logFile.as_const_ptr() as *const _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnLoggerSetLevel(level: i32) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnLoggerSetLevel(level as _) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnLoggerSetMask(mask: i32) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverDnLoggerSetMask(mask as _) };
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
pub unsafe fn cusolverDnXpolar_bufferSize<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            params,
            uplo,
            M,
            N,
            dataTypeA,
            A.as_const_ptr() as *const _,
            lda,
            dataTypeH,
            H.as_const_ptr() as *const _,
            ldh,
            computeType,
            workspaceInBytesOnDevice.as_mut_ptr() as *mut _,
            workspaceInBytesOnHost.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverDnXpolar<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
    A: types::CudaAsPtr,
>(
    handle: cusolverDnHandle_t,
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
            handle,
            params,
            uplo,
            M,
            N,
            dataTypeA,
            A.as_mut_ptr() as *mut _,
            lda,
            dataTypeH,
            H.as_mut_ptr() as *mut _,
            ldh,
            computeType,
            bufferOnDevice.as_mut_ptr() as *mut _,
            workspaceInBytesOnDevice,
            bufferOnHost.as_mut_ptr() as *mut _,
            workspaceInBytesOnHost,
            d_res_nrm.as_mut_ptr() as *mut _,
            d_A_nrmF.as_mut_ptr() as *mut _,
            d_rcond.as_mut_ptr() as *mut _,
            d_info.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpCreate() -> Result<cusolverSpHandle_t, crate::sys::cusolverStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cusolverSpHandle_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cusolverSpCreate(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cusolverSpHandle_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusolverSpDestroy(handle: cusolverSpHandle_t) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverSpDestroy(handle) };
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
pub unsafe fn cusolverSpGetStream(handle: cusolverSpHandle_t) -> Result<cudaStream_t, crate::sys::cusolverStatus_t> {
    let mut out_1: std::mem::MaybeUninit<cudaStream_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cusolverSpGetStream(handle, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as cudaStream_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusolverSpXcsrissymHost<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: i32,
    nnzA: i32,
    descrA: cusparseMatDescr_t,
    csrRowPtrA: T,
    csrEndPtrA: U,
    csrColIndA: V,
    mut issym: W,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpXcsrissymHost(
            handle,
            m as _,
            nnzA as _,
            descrA,
            csrRowPtrA.as_const_ptr() as *const _,
            csrEndPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            issym.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpScsrlsvluHost<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    n: i32,
    nnzA: i32,
    descrA: cusparseMatDescr_t,
    csrValA: T,
    csrRowPtrA: U,
    csrColIndA: V,
    b: W,
    tol: f32,
    reorder: i32,
    mut x: X,
    mut singularity: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpScsrlsvluHost(
            handle,
            n as _,
            nnzA as _,
            descrA,
            csrValA.as_const_ptr() as *const _,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            b.as_const_ptr() as *const _,
            tol,
            reorder as _,
            x.as_mut_ptr() as *mut _,
            singularity.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpDcsrlsvluHost<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    n: i32,
    nnzA: i32,
    descrA: cusparseMatDescr_t,
    csrValA: T,
    csrRowPtrA: U,
    csrColIndA: V,
    b: W,
    tol: f64,
    reorder: i32,
    mut x: X,
    mut singularity: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpDcsrlsvluHost(
            handle,
            n as _,
            nnzA as _,
            descrA,
            csrValA.as_const_ptr() as *const _,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            b.as_const_ptr() as *const _,
            tol,
            reorder as _,
            x.as_mut_ptr() as *mut _,
            singularity.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpCcsrlsvluHost<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    n: i32,
    nnzA: i32,
    descrA: cusparseMatDescr_t,
    csrValA: T,
    csrRowPtrA: U,
    csrColIndA: V,
    b: W,
    tol: f32,
    reorder: i32,
    mut x: X,
    mut singularity: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpCcsrlsvluHost(
            handle,
            n as _,
            nnzA as _,
            descrA,
            csrValA.as_const_ptr() as *const _,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            b.as_const_ptr() as *const _,
            tol,
            reorder as _,
            x.as_mut_ptr() as *mut _,
            singularity.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpZcsrlsvluHost<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    n: i32,
    nnzA: i32,
    descrA: cusparseMatDescr_t,
    csrValA: T,
    csrRowPtrA: U,
    csrColIndA: V,
    b: W,
    tol: f64,
    reorder: i32,
    mut x: X,
    mut singularity: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpZcsrlsvluHost(
            handle,
            n as _,
            nnzA as _,
            descrA,
            csrValA.as_const_ptr() as *const _,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            b.as_const_ptr() as *const _,
            tol,
            reorder as _,
            x.as_mut_ptr() as *mut _,
            singularity.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpScsrlsvqr<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrVal: T,
    csrRowPtr: U,
    csrColInd: V,
    b: W,
    tol: f32,
    reorder: i32,
    mut x: X,
    mut singularity: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpScsrlsvqr(
            handle,
            m as _,
            nnz as _,
            descrA,
            csrVal.as_const_ptr() as *const _,
            csrRowPtr.as_const_ptr() as *const _,
            csrColInd.as_const_ptr() as *const _,
            b.as_const_ptr() as *const _,
            tol,
            reorder as _,
            x.as_mut_ptr() as *mut _,
            singularity.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpDcsrlsvqr<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrVal: T,
    csrRowPtr: U,
    csrColInd: V,
    b: W,
    tol: f64,
    reorder: i32,
    mut x: X,
    mut singularity: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpDcsrlsvqr(
            handle,
            m as _,
            nnz as _,
            descrA,
            csrVal.as_const_ptr() as *const _,
            csrRowPtr.as_const_ptr() as *const _,
            csrColInd.as_const_ptr() as *const _,
            b.as_const_ptr() as *const _,
            tol,
            reorder as _,
            x.as_mut_ptr() as *mut _,
            singularity.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpCcsrlsvqr<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrVal: T,
    csrRowPtr: U,
    csrColInd: V,
    b: W,
    tol: f32,
    reorder: i32,
    mut x: X,
    mut singularity: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpCcsrlsvqr(
            handle,
            m as _,
            nnz as _,
            descrA,
            csrVal.as_const_ptr() as *const _,
            csrRowPtr.as_const_ptr() as *const _,
            csrColInd.as_const_ptr() as *const _,
            b.as_const_ptr() as *const _,
            tol,
            reorder as _,
            x.as_mut_ptr() as *mut _,
            singularity.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpZcsrlsvqr<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrVal: T,
    csrRowPtr: U,
    csrColInd: V,
    b: W,
    tol: f64,
    reorder: i32,
    mut x: X,
    mut singularity: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpZcsrlsvqr(
            handle,
            m as _,
            nnz as _,
            descrA,
            csrVal.as_const_ptr() as *const _,
            csrRowPtr.as_const_ptr() as *const _,
            csrColInd.as_const_ptr() as *const _,
            b.as_const_ptr() as *const _,
            tol,
            reorder as _,
            x.as_mut_ptr() as *mut _,
            singularity.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpScsrlsvqrHost<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrValA: T,
    csrRowPtrA: U,
    csrColIndA: V,
    b: W,
    tol: f32,
    reorder: i32,
    mut x: X,
    mut singularity: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpScsrlsvqrHost(
            handle,
            m as _,
            nnz as _,
            descrA,
            csrValA.as_const_ptr() as *const _,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            b.as_const_ptr() as *const _,
            tol,
            reorder as _,
            x.as_mut_ptr() as *mut _,
            singularity.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpDcsrlsvqrHost<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrValA: T,
    csrRowPtrA: U,
    csrColIndA: V,
    b: W,
    tol: f64,
    reorder: i32,
    mut x: X,
    mut singularity: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpDcsrlsvqrHost(
            handle,
            m as _,
            nnz as _,
            descrA,
            csrValA.as_const_ptr() as *const _,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            b.as_const_ptr() as *const _,
            tol,
            reorder as _,
            x.as_mut_ptr() as *mut _,
            singularity.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpCcsrlsvqrHost<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrValA: T,
    csrRowPtrA: U,
    csrColIndA: V,
    b: W,
    tol: f32,
    reorder: i32,
    mut x: X,
    mut singularity: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpCcsrlsvqrHost(
            handle,
            m as _,
            nnz as _,
            descrA,
            csrValA.as_const_ptr() as *const _,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            b.as_const_ptr() as *const _,
            tol,
            reorder as _,
            x.as_mut_ptr() as *mut _,
            singularity.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpZcsrlsvqrHost<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrValA: T,
    csrRowPtrA: U,
    csrColIndA: V,
    b: W,
    tol: f64,
    reorder: i32,
    mut x: X,
    mut singularity: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpZcsrlsvqrHost(
            handle,
            m as _,
            nnz as _,
            descrA,
            csrValA.as_const_ptr() as *const _,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            b.as_const_ptr() as *const _,
            tol,
            reorder as _,
            x.as_mut_ptr() as *mut _,
            singularity.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpScsrlsvcholHost<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrVal: T,
    csrRowPtr: U,
    csrColInd: V,
    b: W,
    tol: f32,
    reorder: i32,
    mut x: X,
    mut singularity: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpScsrlsvcholHost(
            handle,
            m as _,
            nnz as _,
            descrA,
            csrVal.as_const_ptr() as *const _,
            csrRowPtr.as_const_ptr() as *const _,
            csrColInd.as_const_ptr() as *const _,
            b.as_const_ptr() as *const _,
            tol,
            reorder as _,
            x.as_mut_ptr() as *mut _,
            singularity.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpDcsrlsvcholHost<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrVal: T,
    csrRowPtr: U,
    csrColInd: V,
    b: W,
    tol: f64,
    reorder: i32,
    mut x: X,
    mut singularity: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpDcsrlsvcholHost(
            handle,
            m as _,
            nnz as _,
            descrA,
            csrVal.as_const_ptr() as *const _,
            csrRowPtr.as_const_ptr() as *const _,
            csrColInd.as_const_ptr() as *const _,
            b.as_const_ptr() as *const _,
            tol,
            reorder as _,
            x.as_mut_ptr() as *mut _,
            singularity.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpCcsrlsvcholHost<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrVal: T,
    csrRowPtr: U,
    csrColInd: V,
    b: W,
    tol: f32,
    reorder: i32,
    mut x: X,
    mut singularity: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpCcsrlsvcholHost(
            handle,
            m as _,
            nnz as _,
            descrA,
            csrVal.as_const_ptr() as *const _,
            csrRowPtr.as_const_ptr() as *const _,
            csrColInd.as_const_ptr() as *const _,
            b.as_const_ptr() as *const _,
            tol,
            reorder as _,
            x.as_mut_ptr() as *mut _,
            singularity.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpZcsrlsvcholHost<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrVal: T,
    csrRowPtr: U,
    csrColInd: V,
    b: W,
    tol: f64,
    reorder: i32,
    mut x: X,
    mut singularity: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpZcsrlsvcholHost(
            handle,
            m as _,
            nnz as _,
            descrA,
            csrVal.as_const_ptr() as *const _,
            csrRowPtr.as_const_ptr() as *const _,
            csrColInd.as_const_ptr() as *const _,
            b.as_const_ptr() as *const _,
            tol,
            reorder as _,
            x.as_mut_ptr() as *mut _,
            singularity.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpScsrlsvchol<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrVal: T,
    csrRowPtr: U,
    csrColInd: V,
    b: W,
    tol: f32,
    reorder: i32,
    mut x: X,
    mut singularity: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpScsrlsvchol(
            handle,
            m as _,
            nnz as _,
            descrA,
            csrVal.as_const_ptr() as *const _,
            csrRowPtr.as_const_ptr() as *const _,
            csrColInd.as_const_ptr() as *const _,
            b.as_const_ptr() as *const _,
            tol,
            reorder as _,
            x.as_mut_ptr() as *mut _,
            singularity.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpDcsrlsvchol<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrVal: T,
    csrRowPtr: U,
    csrColInd: V,
    b: W,
    tol: f64,
    reorder: i32,
    mut x: X,
    mut singularity: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpDcsrlsvchol(
            handle,
            m as _,
            nnz as _,
            descrA,
            csrVal.as_const_ptr() as *const _,
            csrRowPtr.as_const_ptr() as *const _,
            csrColInd.as_const_ptr() as *const _,
            b.as_const_ptr() as *const _,
            tol,
            reorder as _,
            x.as_mut_ptr() as *mut _,
            singularity.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpCcsrlsvchol<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrVal: T,
    csrRowPtr: U,
    csrColInd: V,
    b: W,
    tol: f32,
    reorder: i32,
    mut x: X,
    mut singularity: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpCcsrlsvchol(
            handle,
            m as _,
            nnz as _,
            descrA,
            csrVal.as_const_ptr() as *const _,
            csrRowPtr.as_const_ptr() as *const _,
            csrColInd.as_const_ptr() as *const _,
            b.as_const_ptr() as *const _,
            tol,
            reorder as _,
            x.as_mut_ptr() as *mut _,
            singularity.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpZcsrlsvchol<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrVal: T,
    csrRowPtr: U,
    csrColInd: V,
    b: W,
    tol: f64,
    reorder: i32,
    mut x: X,
    mut singularity: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpZcsrlsvchol(
            handle,
            m as _,
            nnz as _,
            descrA,
            csrVal.as_const_ptr() as *const _,
            csrRowPtr.as_const_ptr() as *const _,
            csrColInd.as_const_ptr() as *const _,
            b.as_const_ptr() as *const _,
            tol,
            reorder as _,
            x.as_mut_ptr() as *mut _,
            singularity.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpScsrlsqvqrHost<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
    A: types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: i32,
    n: i32,
    nnz: i32,
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
            m as _,
            n as _,
            nnz as _,
            descrA,
            csrValA.as_const_ptr() as *const _,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            b.as_const_ptr() as *const _,
            tol,
            rankA.as_mut_ptr() as *mut _,
            x.as_mut_ptr() as *mut _,
            p.as_mut_ptr() as *mut _,
            min_norm.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpDcsrlsqvqrHost<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
    A: types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: i32,
    n: i32,
    nnz: i32,
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
            m as _,
            n as _,
            nnz as _,
            descrA,
            csrValA.as_const_ptr() as *const _,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            b.as_const_ptr() as *const _,
            tol,
            rankA.as_mut_ptr() as *mut _,
            x.as_mut_ptr() as *mut _,
            p.as_mut_ptr() as *mut _,
            min_norm.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpCcsrlsqvqrHost<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
    A: types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: i32,
    n: i32,
    nnz: i32,
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
            m as _,
            n as _,
            nnz as _,
            descrA,
            csrValA.as_const_ptr() as *const _,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            b.as_const_ptr() as *const _,
            tol,
            rankA.as_mut_ptr() as *mut _,
            x.as_mut_ptr() as *mut _,
            p.as_mut_ptr() as *mut _,
            min_norm.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpZcsrlsqvqrHost<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
    A: types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: i32,
    n: i32,
    nnz: i32,
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
            m as _,
            n as _,
            nnz as _,
            descrA,
            csrValA.as_const_ptr() as *const _,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            b.as_const_ptr() as *const _,
            tol,
            rankA.as_mut_ptr() as *mut _,
            x.as_mut_ptr() as *mut _,
            p.as_mut_ptr() as *mut _,
            min_norm.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpScsreigvsiHost<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrValA: T,
    csrRowPtrA: U,
    csrColIndA: V,
    mu0: f32,
    x0: W,
    maxite: i32,
    tol: f32,
    mut mu: X,
    mut x: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpScsreigvsiHost(
            handle,
            m as _,
            nnz as _,
            descrA,
            csrValA.as_const_ptr() as *const _,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            mu0,
            x0.as_const_ptr() as *const _,
            maxite as _,
            tol,
            mu.as_mut_ptr() as *mut _,
            x.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpDcsreigvsiHost<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrValA: T,
    csrRowPtrA: U,
    csrColIndA: V,
    mu0: f64,
    x0: W,
    maxite: i32,
    tol: f64,
    mut mu: X,
    mut x: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpDcsreigvsiHost(
            handle,
            m as _,
            nnz as _,
            descrA,
            csrValA.as_const_ptr() as *const _,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            mu0,
            x0.as_const_ptr() as *const _,
            maxite as _,
            tol,
            mu.as_mut_ptr() as *mut _,
            x.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpCcsreigvsiHost<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrValA: T,
    csrRowPtrA: U,
    csrColIndA: V,
    mu0: cuComplex,
    x0: W,
    maxite: i32,
    tol: f32,
    mut mu: X,
    mut x: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpCcsreigvsiHost(
            handle,
            m as _,
            nnz as _,
            descrA,
            csrValA.as_const_ptr() as *const _,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            mu0,
            x0.as_const_ptr() as *const _,
            maxite as _,
            tol,
            mu.as_mut_ptr() as *mut _,
            x.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpZcsreigvsiHost<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrValA: T,
    csrRowPtrA: U,
    csrColIndA: V,
    mu0: cuDoubleComplex,
    x0: W,
    maxite: i32,
    tol: f64,
    mut mu: X,
    mut x: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpZcsreigvsiHost(
            handle,
            m as _,
            nnz as _,
            descrA,
            csrValA.as_const_ptr() as *const _,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            mu0,
            x0.as_const_ptr() as *const _,
            maxite as _,
            tol,
            mu.as_mut_ptr() as *mut _,
            x.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpScsreigvsi<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrValA: T,
    csrRowPtrA: U,
    csrColIndA: V,
    mu0: f32,
    x0: W,
    maxite: i32,
    eps: f32,
    mut mu: X,
    mut x: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpScsreigvsi(
            handle,
            m as _,
            nnz as _,
            descrA,
            csrValA.as_const_ptr() as *const _,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            mu0,
            x0.as_const_ptr() as *const _,
            maxite as _,
            eps,
            mu.as_mut_ptr() as *mut _,
            x.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpDcsreigvsi<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrValA: T,
    csrRowPtrA: U,
    csrColIndA: V,
    mu0: f64,
    x0: W,
    maxite: i32,
    eps: f64,
    mut mu: X,
    mut x: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpDcsreigvsi(
            handle,
            m as _,
            nnz as _,
            descrA,
            csrValA.as_const_ptr() as *const _,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            mu0,
            x0.as_const_ptr() as *const _,
            maxite as _,
            eps,
            mu.as_mut_ptr() as *mut _,
            x.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpCcsreigvsi<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrValA: T,
    csrRowPtrA: U,
    csrColIndA: V,
    mu0: cuComplex,
    x0: W,
    maxite: i32,
    eps: f32,
    mut mu: X,
    mut x: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpCcsreigvsi(
            handle,
            m as _,
            nnz as _,
            descrA,
            csrValA.as_const_ptr() as *const _,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            mu0,
            x0.as_const_ptr() as *const _,
            maxite as _,
            eps,
            mu.as_mut_ptr() as *mut _,
            x.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpZcsreigvsi<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrValA: T,
    csrRowPtrA: U,
    csrColIndA: V,
    mu0: cuDoubleComplex,
    x0: W,
    maxite: i32,
    eps: f64,
    mut mu: X,
    mut x: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpZcsreigvsi(
            handle,
            m as _,
            nnz as _,
            descrA,
            csrValA.as_const_ptr() as *const _,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            mu0,
            x0.as_const_ptr() as *const _,
            maxite as _,
            eps,
            mu.as_mut_ptr() as *mut _,
            x.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpScsreigsHost<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: i32,
    nnz: i32,
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
            m as _,
            nnz as _,
            descrA,
            csrValA.as_const_ptr() as *const _,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            left_bottom_corner,
            right_upper_corner,
            num_eigs.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpDcsreigsHost<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: i32,
    nnz: i32,
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
            m as _,
            nnz as _,
            descrA,
            csrValA.as_const_ptr() as *const _,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            left_bottom_corner,
            right_upper_corner,
            num_eigs.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpCcsreigsHost<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: i32,
    nnz: i32,
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
            m as _,
            nnz as _,
            descrA,
            csrValA.as_const_ptr() as *const _,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            left_bottom_corner,
            right_upper_corner,
            num_eigs.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpZcsreigsHost<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: i32,
    nnz: i32,
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
            m as _,
            nnz as _,
            descrA,
            csrValA.as_const_ptr() as *const _,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            left_bottom_corner,
            right_upper_corner,
            num_eigs.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpXcsrsymrcmHost<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusolverSpHandle_t,
    n: i32,
    nnzA: i32,
    descrA: cusparseMatDescr_t,
    csrRowPtrA: T,
    csrColIndA: U,
    mut p: V,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpXcsrsymrcmHost(
            handle,
            n as _,
            nnzA as _,
            descrA,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            p.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpXcsrsymmdqHost<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusolverSpHandle_t,
    n: i32,
    nnzA: i32,
    descrA: cusparseMatDescr_t,
    csrRowPtrA: T,
    csrColIndA: U,
    mut p: V,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpXcsrsymmdqHost(
            handle,
            n as _,
            nnzA as _,
            descrA,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            p.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpXcsrsymamdHost<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cusolverSpHandle_t,
    n: i32,
    nnzA: i32,
    descrA: cusparseMatDescr_t,
    csrRowPtrA: T,
    csrColIndA: U,
    mut p: V,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpXcsrsymamdHost(
            handle,
            n as _,
            nnzA as _,
            descrA,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            p.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpXcsrmetisndHost<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    n: i32,
    nnzA: i32,
    descrA: cusparseMatDescr_t,
    csrRowPtrA: T,
    csrColIndA: U,
    options: V,
    mut p: W,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpXcsrmetisndHost(
            handle,
            n as _,
            nnzA as _,
            descrA,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            options.as_const_ptr() as *const _,
            p.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpScsrzfdHost<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    n: i32,
    nnz: i32,
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
            n as _,
            nnz as _,
            descrA,
            csrValA.as_const_ptr() as *const _,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            P.as_mut_ptr() as *mut _,
            numnz.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpDcsrzfdHost<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    n: i32,
    nnz: i32,
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
            n as _,
            nnz as _,
            descrA,
            csrValA.as_const_ptr() as *const _,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            P.as_mut_ptr() as *mut _,
            numnz.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpCcsrzfdHost<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    n: i32,
    nnz: i32,
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
            n as _,
            nnz as _,
            descrA,
            csrValA.as_const_ptr() as *const _,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            P.as_mut_ptr() as *mut _,
            numnz.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpZcsrzfdHost<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    n: i32,
    nnz: i32,
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
            n as _,
            nnz as _,
            descrA,
            csrValA.as_const_ptr() as *const _,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            P.as_mut_ptr() as *mut _,
            numnz.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpXcsrperm_bufferSizeHost<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: i32,
    n: i32,
    nnzA: i32,
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
            m as _,
            n as _,
            nnzA as _,
            descrA,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            p.as_const_ptr() as *const _,
            q.as_const_ptr() as *const _,
            bufferSizeInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpXcsrpermHost<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: i32,
    n: i32,
    nnzA: i32,
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
            m as _,
            n as _,
            nnzA as _,
            descrA,
            csrRowPtrA.as_mut_ptr() as *mut _,
            csrColIndA.as_mut_ptr() as *mut _,
            p.as_const_ptr() as *const _,
            q.as_const_ptr() as *const _,
            map.as_mut_ptr() as *mut _,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpCreateCsrqrInfo() -> Result<csrqrInfo_t, crate::sys::cusolverStatus_t> {
    let mut out_0: std::mem::MaybeUninit<csrqrInfo_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cusolverSpCreateCsrqrInfo(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as csrqrInfo_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cusolverSpDestroyCsrqrInfo(info: csrqrInfo_t) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe { crate::sys::cusolverSpDestroyCsrqrInfo(info) };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpXcsrqrAnalysisBatched<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    handle: cusolverSpHandle_t,
    m: i32,
    n: i32,
    nnzA: i32,
    descrA: cusparseMatDescr_t,
    csrRowPtrA: T,
    csrColIndA: U,
    info: csrqrInfo_t,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpXcsrqrAnalysisBatched(
            handle,
            m as _,
            n as _,
            nnzA as _,
            descrA,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
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
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: i32,
    n: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrVal: T,
    csrRowPtr: U,
    csrColInd: V,
    batchSize: i32,
    info: csrqrInfo_t,
    mut internalDataInBytes: W,
    mut workspaceInBytes: X,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpScsrqrBufferInfoBatched(
            handle,
            m as _,
            n as _,
            nnz as _,
            descrA,
            csrVal.as_const_ptr() as *const _,
            csrRowPtr.as_const_ptr() as *const _,
            csrColInd.as_const_ptr() as *const _,
            batchSize as _,
            info,
            internalDataInBytes.as_mut_ptr() as *mut _,
            workspaceInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpDcsrqrBufferInfoBatched<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: i32,
    n: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrVal: T,
    csrRowPtr: U,
    csrColInd: V,
    batchSize: i32,
    info: csrqrInfo_t,
    mut internalDataInBytes: W,
    mut workspaceInBytes: X,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpDcsrqrBufferInfoBatched(
            handle,
            m as _,
            n as _,
            nnz as _,
            descrA,
            csrVal.as_const_ptr() as *const _,
            csrRowPtr.as_const_ptr() as *const _,
            csrColInd.as_const_ptr() as *const _,
            batchSize as _,
            info,
            internalDataInBytes.as_mut_ptr() as *mut _,
            workspaceInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpCcsrqrBufferInfoBatched<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: i32,
    n: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrVal: T,
    csrRowPtr: U,
    csrColInd: V,
    batchSize: i32,
    info: csrqrInfo_t,
    mut internalDataInBytes: W,
    mut workspaceInBytes: X,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpCcsrqrBufferInfoBatched(
            handle,
            m as _,
            n as _,
            nnz as _,
            descrA,
            csrVal.as_const_ptr() as *const _,
            csrRowPtr.as_const_ptr() as *const _,
            csrColInd.as_const_ptr() as *const _,
            batchSize as _,
            info,
            internalDataInBytes.as_mut_ptr() as *mut _,
            workspaceInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpZcsrqrBufferInfoBatched<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: i32,
    n: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrVal: T,
    csrRowPtr: U,
    csrColInd: V,
    batchSize: i32,
    info: csrqrInfo_t,
    mut internalDataInBytes: W,
    mut workspaceInBytes: X,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpZcsrqrBufferInfoBatched(
            handle,
            m as _,
            n as _,
            nnz as _,
            descrA,
            csrVal.as_const_ptr() as *const _,
            csrRowPtr.as_const_ptr() as *const _,
            csrColInd.as_const_ptr() as *const _,
            batchSize as _,
            info,
            internalDataInBytes.as_mut_ptr() as *mut _,
            workspaceInBytes.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpScsrqrsvBatched<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: i32,
    n: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrValA: T,
    csrRowPtrA: U,
    csrColIndA: V,
    b: W,
    mut x: X,
    batchSize: i32,
    info: csrqrInfo_t,
    mut pBuffer: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpScsrqrsvBatched(
            handle,
            m as _,
            n as _,
            nnz as _,
            descrA,
            csrValA.as_const_ptr() as *const _,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            b.as_const_ptr() as *const _,
            x.as_mut_ptr() as *mut _,
            batchSize as _,
            info,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpDcsrqrsvBatched<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: i32,
    n: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrValA: T,
    csrRowPtrA: U,
    csrColIndA: V,
    b: W,
    mut x: X,
    batchSize: i32,
    info: csrqrInfo_t,
    mut pBuffer: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpDcsrqrsvBatched(
            handle,
            m as _,
            n as _,
            nnz as _,
            descrA,
            csrValA.as_const_ptr() as *const _,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            b.as_const_ptr() as *const _,
            x.as_mut_ptr() as *mut _,
            batchSize as _,
            info,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpCcsrqrsvBatched<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: i32,
    n: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrValA: T,
    csrRowPtrA: U,
    csrColIndA: V,
    b: W,
    mut x: X,
    batchSize: i32,
    info: csrqrInfo_t,
    mut pBuffer: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpCcsrqrsvBatched(
            handle,
            m as _,
            n as _,
            nnz as _,
            descrA,
            csrValA.as_const_ptr() as *const _,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            b.as_const_ptr() as *const _,
            x.as_mut_ptr() as *mut _,
            batchSize as _,
            info,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cusolverSpZcsrqrsvBatched<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cusolverSpHandle_t,
    m: i32,
    n: i32,
    nnz: i32,
    descrA: cusparseMatDescr_t,
    csrValA: T,
    csrRowPtrA: U,
    csrColIndA: V,
    b: W,
    mut x: X,
    batchSize: i32,
    info: csrqrInfo_t,
    mut pBuffer: Y,
) -> Result<(), crate::sys::cusolverStatus_t> {
    let status = unsafe {
        crate::sys::cusolverSpZcsrqrsvBatched(
            handle,
            m as _,
            n as _,
            nnz as _,
            descrA,
            csrValA.as_const_ptr() as *const _,
            csrRowPtrA.as_const_ptr() as *const _,
            csrColIndA.as_const_ptr() as *const _,
            b.as_const_ptr() as *const _,
            x.as_mut_ptr() as *mut _,
            batchSize as _,
            info,
            pBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
