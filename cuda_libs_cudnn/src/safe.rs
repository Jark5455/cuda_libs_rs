pub use crate::sys::cudnnStatus_t as CudaTargetStatus;
#[allow(unused_imports)]
use crate::sys::*;
use cuda_libs_cudart::sys::*;
impl crate::sys::cudnnDebugStruct {
    pub fn cudnn_version(mut self, val: ::std::os::raw::c_uint) -> Self {
        self.cudnn_version = val;
        self
    }
    pub fn cudnnStatus(mut self, val: cudnnStatus_t) -> Self {
        self.cudnnStatus = val;
        self
    }
    pub fn time_sec(mut self, val: ::std::os::raw::c_uint) -> Self {
        self.time_sec = val;
        self
    }
    pub fn time_usec(mut self, val: ::std::os::raw::c_uint) -> Self {
        self.time_usec = val;
        self
    }
    pub fn time_delta(mut self, val: ::std::os::raw::c_uint) -> Self {
        self.time_delta = val;
        self
    }
    pub fn handle(mut self, val: cudnnHandle_t) -> Self {
        self.handle = val;
        self
    }
    pub fn stream(mut self, val: cudaStream_t) -> Self {
        self.stream = val;
        self
    }
    pub fn pid(mut self, val: ::std::os::raw::c_ulonglong) -> Self {
        self.pid = val;
        self
    }
    pub fn tid(mut self, val: ::std::os::raw::c_ulonglong) -> Self {
        self.tid = val;
        self
    }
    pub fn cudaDeviceId(mut self, val: ::std::os::raw::c_int) -> Self {
        self.cudaDeviceId = val;
        self
    }
    pub fn reserved(mut self, val: [::std::os::raw::c_int; 15usize]) -> Self {
        self.reserved = val;
        self
    }
}
impl crate::sys::cudnnFractionStruct {
    pub fn numerator(mut self, val: i64) -> Self {
        self.numerator = val;
        self
    }
    pub fn denominator(mut self, val: i64) -> Self {
        self.denominator = val;
        self
    }
}
impl crate::sys::cudnnConvolutionFwdAlgoPerfStruct {
    pub fn algo(mut self, val: cudnnConvolutionFwdAlgo_t) -> Self {
        self.algo = val;
        self
    }
    pub fn status(mut self, val: cudnnStatus_t) -> Self {
        self.status = val;
        self
    }
    pub fn time(mut self, val: f32) -> Self {
        self.time = val;
        self
    }
    pub fn memory(mut self, val: usize) -> Self {
        self.memory = val;
        self
    }
    pub fn determinism(mut self, val: cudnnDeterminism_t) -> Self {
        self.determinism = val;
        self
    }
    pub fn mathType(mut self, val: cudnnMathType_t) -> Self {
        self.mathType = val;
        self
    }
    pub fn reserved(mut self, val: [::std::os::raw::c_int; 3usize]) -> Self {
        self.reserved = val;
        self
    }
}
impl crate::sys::cudnnConvolutionBwdDataAlgoPerfStruct {
    pub fn algo(mut self, val: cudnnConvolutionBwdDataAlgo_t) -> Self {
        self.algo = val;
        self
    }
    pub fn status(mut self, val: cudnnStatus_t) -> Self {
        self.status = val;
        self
    }
    pub fn time(mut self, val: f32) -> Self {
        self.time = val;
        self
    }
    pub fn memory(mut self, val: usize) -> Self {
        self.memory = val;
        self
    }
    pub fn determinism(mut self, val: cudnnDeterminism_t) -> Self {
        self.determinism = val;
        self
    }
    pub fn mathType(mut self, val: cudnnMathType_t) -> Self {
        self.mathType = val;
        self
    }
    pub fn reserved(mut self, val: [::std::os::raw::c_int; 3usize]) -> Self {
        self.reserved = val;
        self
    }
}
impl crate::sys::cudnnConvolutionBwdFilterAlgoPerfStruct {
    pub fn algo(mut self, val: cudnnConvolutionBwdFilterAlgo_t) -> Self {
        self.algo = val;
        self
    }
    pub fn status(mut self, val: cudnnStatus_t) -> Self {
        self.status = val;
        self
    }
    pub fn time(mut self, val: f32) -> Self {
        self.time = val;
        self
    }
    pub fn memory(mut self, val: usize) -> Self {
        self.memory = val;
        self
    }
    pub fn determinism(mut self, val: cudnnDeterminism_t) -> Self {
        self.determinism = val;
        self
    }
    pub fn mathType(mut self, val: cudnnMathType_t) -> Self {
        self.mathType = val;
        self
    }
    pub fn reserved(mut self, val: [::std::os::raw::c_int; 3usize]) -> Self {
        self.reserved = val;
        self
    }
}
pub struct CudnnHandle {
    pub(crate) handle: crate::sys::cudnnHandle_t,
}
impl CudnnHandle {
    pub unsafe fn cudnnQueryRuntimeError<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        mut rstatus: T,
        mode: cudnnErrQueryMode_t,
        mut tag: U,
    ) -> Result<(), crate::sys::cudnnStatus_t> {
        let status = unsafe {
            crate::sys::cudnnQueryRuntimeError(
                self.handle,
                rstatus.as_mut_ptr() as *mut cudnnStatus_t,
                mode,
                tag.as_mut_ptr() as *mut cudnnRuntimeTag_t,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnSetStream(
        &self,
        streamId: cudaStream_t,
    ) -> Result<(), crate::sys::cudnnStatus_t> {
        let status = unsafe { crate::sys::cudnnSetStream(self.handle, streamId) };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnGetStream(&self) -> Result<cudaStream_t, crate::sys::cudnnStatus_t> {
        let mut out_1: std::mem::MaybeUninit<cudaStream_t> = std::mem::MaybeUninit::uninit();
        let status =
            unsafe { crate::sys::cudnnGetStream(self.handle, out_1.as_mut_ptr() as *mut _) };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            unsafe { Ok(out_1.assume_init()) }
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnBackendExecute(
        &self,
        executionPlan: cudnnBackendDescriptor_t,
        variantPack: cudnnBackendDescriptor_t,
    ) -> Result<(), crate::sys::cudnnStatus_t> {
        let status =
            unsafe { crate::sys::cudnnBackendExecute(self.handle, executionPlan, variantPack) };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnBackendPopulateCudaGraph(
        &self,
        executionPlan: cudnnBackendDescriptor_t,
        variantPack: cudnnBackendDescriptor_t,
        graph: cudaGraph_t,
    ) -> Result<(), crate::sys::cudnnStatus_t> {
        let status = unsafe {
            crate::sys::cudnnBackendPopulateCudaGraph(
                self.handle,
                executionPlan,
                variantPack,
                graph,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnBackendUpdateCudaGraph(
        &self,
        executionPlan: cudnnBackendDescriptor_t,
        variantPack: cudnnBackendDescriptor_t,
        graph: cudaGraph_t,
    ) -> Result<(), crate::sys::cudnnStatus_t> {
        let status = unsafe {
            crate::sys::cudnnBackendUpdateCudaGraph(self.handle, executionPlan, variantPack, graph)
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnTransformTensor<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        alpha: T,
        xDesc: cudnnTensorDescriptor_t,
        x: U,
        beta: V,
        yDesc: cudnnTensorDescriptor_t,
        mut y: W,
    ) -> Result<(), crate::sys::cudnnStatus_t> {
        let status = unsafe {
            crate::sys::cudnnTransformTensor(
                self.handle,
                alpha.as_const_ptr() as *const ::std::os::raw::c_void,
                xDesc,
                x.as_const_ptr() as *const ::std::os::raw::c_void,
                beta.as_const_ptr() as *const ::std::os::raw::c_void,
                yDesc,
                y.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnTransformTensorEx<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        transDesc: cudnnTensorTransformDescriptor_t,
        alpha: T,
        srcDesc: cudnnTensorDescriptor_t,
        srcData: U,
        beta: V,
        destDesc: cudnnTensorDescriptor_t,
        mut destData: W,
    ) -> Result<(), crate::sys::cudnnStatus_t> {
        let status = unsafe {
            crate::sys::cudnnTransformTensorEx(
                self.handle,
                transDesc,
                alpha.as_const_ptr() as *const ::std::os::raw::c_void,
                srcDesc,
                srcData.as_const_ptr() as *const ::std::os::raw::c_void,
                beta.as_const_ptr() as *const ::std::os::raw::c_void,
                destDesc,
                destData.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnAddTensor<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        alpha: T,
        aDesc: cudnnTensorDescriptor_t,
        A: U,
        beta: V,
        cDesc: cudnnTensorDescriptor_t,
        mut C: W,
    ) -> Result<(), crate::sys::cudnnStatus_t> {
        let status = unsafe {
            crate::sys::cudnnAddTensor(
                self.handle,
                alpha.as_const_ptr() as *const ::std::os::raw::c_void,
                aDesc,
                A.as_const_ptr() as *const ::std::os::raw::c_void,
                beta.as_const_ptr() as *const ::std::os::raw::c_void,
                cDesc,
                C.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnOpTensor<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        opTensorDesc: cudnnOpTensorDescriptor_t,
        alpha1: T,
        aDesc: cudnnTensorDescriptor_t,
        A: U,
        alpha2: V,
        bDesc: cudnnTensorDescriptor_t,
        B: W,
        beta: X,
        cDesc: cudnnTensorDescriptor_t,
        mut C: Y,
    ) -> Result<(), crate::sys::cudnnStatus_t> {
        let status = unsafe {
            crate::sys::cudnnOpTensor(
                self.handle,
                opTensorDesc,
                alpha1.as_const_ptr() as *const ::std::os::raw::c_void,
                aDesc,
                A.as_const_ptr() as *const ::std::os::raw::c_void,
                alpha2.as_const_ptr() as *const ::std::os::raw::c_void,
                bDesc,
                B.as_const_ptr() as *const ::std::os::raw::c_void,
                beta.as_const_ptr() as *const ::std::os::raw::c_void,
                cDesc,
                C.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnGetReductionIndicesSize(
        &self,
        reduceTensorDesc: cudnnReduceTensorDescriptor_t,
        aDesc: cudnnTensorDescriptor_t,
        cDesc: cudnnTensorDescriptor_t,
    ) -> Result<usize, crate::sys::cudnnStatus_t> {
        let mut out_4: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
        let status = unsafe {
            crate::sys::cudnnGetReductionIndicesSize(
                self.handle,
                reduceTensorDesc,
                aDesc,
                cDesc,
                out_4.as_mut_ptr() as *mut _,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            unsafe { Ok(out_4.assume_init()) }
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnGetReductionWorkspaceSize(
        &self,
        reduceTensorDesc: cudnnReduceTensorDescriptor_t,
        aDesc: cudnnTensorDescriptor_t,
        cDesc: cudnnTensorDescriptor_t,
    ) -> Result<usize, crate::sys::cudnnStatus_t> {
        let mut out_4: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
        let status = unsafe {
            crate::sys::cudnnGetReductionWorkspaceSize(
                self.handle,
                reduceTensorDesc,
                aDesc,
                cDesc,
                out_4.as_mut_ptr() as *mut _,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            unsafe { Ok(out_4.assume_init()) }
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnReduceTensor<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        reduceTensorDesc: cudnnReduceTensorDescriptor_t,
        mut indices: T,
        indicesSizeInBytes: usize,
        mut workspace: U,
        workspaceSizeInBytes: usize,
        alpha: V,
        aDesc: cudnnTensorDescriptor_t,
        A: W,
        beta: X,
        cDesc: cudnnTensorDescriptor_t,
        mut C: Y,
    ) -> Result<(), crate::sys::cudnnStatus_t> {
        let status = unsafe {
            crate::sys::cudnnReduceTensor(
                self.handle,
                reduceTensorDesc,
                indices.as_mut_ptr() as *mut ::std::os::raw::c_void,
                indicesSizeInBytes,
                workspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                workspaceSizeInBytes,
                alpha.as_const_ptr() as *const ::std::os::raw::c_void,
                aDesc,
                A.as_const_ptr() as *const ::std::os::raw::c_void,
                beta.as_const_ptr() as *const ::std::os::raw::c_void,
                cDesc,
                C.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnSetTensor<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        yDesc: cudnnTensorDescriptor_t,
        mut y: T,
        valuePtr: U,
    ) -> Result<(), crate::sys::cudnnStatus_t> {
        let status = unsafe {
            crate::sys::cudnnSetTensor(
                self.handle,
                yDesc,
                y.as_mut_ptr() as *mut ::std::os::raw::c_void,
                valuePtr.as_const_ptr() as *const ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnScaleTensor<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        yDesc: cudnnTensorDescriptor_t,
        mut y: T,
        alpha: U,
    ) -> Result<(), crate::sys::cudnnStatus_t> {
        let status = unsafe {
            crate::sys::cudnnScaleTensor(
                self.handle,
                yDesc,
                y.as_mut_ptr() as *mut ::std::os::raw::c_void,
                alpha.as_const_ptr() as *const ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnTransformFilter<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        transDesc: cudnnTensorTransformDescriptor_t,
        alpha: T,
        srcDesc: cudnnFilterDescriptor_t,
        srcData: U,
        beta: V,
        destDesc: cudnnFilterDescriptor_t,
        mut destData: W,
    ) -> Result<(), crate::sys::cudnnStatus_t> {
        let status = unsafe {
            crate::sys::cudnnTransformFilter(
                self.handle,
                transDesc,
                alpha.as_const_ptr() as *const ::std::os::raw::c_void,
                srcDesc,
                srcData.as_const_ptr() as *const ::std::os::raw::c_void,
                beta.as_const_ptr() as *const ::std::os::raw::c_void,
                destDesc,
                destData.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnSoftmaxForward<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        algo: cudnnSoftmaxAlgorithm_t,
        mode: cudnnSoftmaxMode_t,
        alpha: T,
        xDesc: cudnnTensorDescriptor_t,
        x: U,
        beta: V,
        yDesc: cudnnTensorDescriptor_t,
        mut y: W,
    ) -> Result<(), crate::sys::cudnnStatus_t> {
        let status = unsafe {
            crate::sys::cudnnSoftmaxForward(
                self.handle,
                algo,
                mode,
                alpha.as_const_ptr() as *const ::std::os::raw::c_void,
                xDesc,
                x.as_const_ptr() as *const ::std::os::raw::c_void,
                beta.as_const_ptr() as *const ::std::os::raw::c_void,
                yDesc,
                y.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnPoolingForward<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        poolingDesc: cudnnPoolingDescriptor_t,
        alpha: T,
        xDesc: cudnnTensorDescriptor_t,
        x: U,
        beta: V,
        yDesc: cudnnTensorDescriptor_t,
        mut y: W,
    ) -> Result<(), crate::sys::cudnnStatus_t> {
        let status = unsafe {
            crate::sys::cudnnPoolingForward(
                self.handle,
                poolingDesc,
                alpha.as_const_ptr() as *const ::std::os::raw::c_void,
                xDesc,
                x.as_const_ptr() as *const ::std::os::raw::c_void,
                beta.as_const_ptr() as *const ::std::os::raw::c_void,
                yDesc,
                y.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnActivationForward<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        activationDesc: cudnnActivationDescriptor_t,
        alpha: T,
        xDesc: cudnnTensorDescriptor_t,
        x: U,
        beta: V,
        yDesc: cudnnTensorDescriptor_t,
        mut y: W,
    ) -> Result<(), crate::sys::cudnnStatus_t> {
        let status = unsafe {
            crate::sys::cudnnActivationForward(
                self.handle,
                activationDesc,
                alpha.as_const_ptr() as *const ::std::os::raw::c_void,
                xDesc,
                x.as_const_ptr() as *const ::std::os::raw::c_void,
                beta.as_const_ptr() as *const ::std::os::raw::c_void,
                yDesc,
                y.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnLRNCrossChannelForward<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        normDesc: cudnnLRNDescriptor_t,
        lrnMode: cudnnLRNMode_t,
        alpha: T,
        xDesc: cudnnTensorDescriptor_t,
        x: U,
        beta: V,
        yDesc: cudnnTensorDescriptor_t,
        mut y: W,
    ) -> Result<(), crate::sys::cudnnStatus_t> {
        let status = unsafe {
            crate::sys::cudnnLRNCrossChannelForward(
                self.handle,
                normDesc,
                lrnMode,
                alpha.as_const_ptr() as *const ::std::os::raw::c_void,
                xDesc,
                x.as_const_ptr() as *const ::std::os::raw::c_void,
                beta.as_const_ptr() as *const ::std::os::raw::c_void,
                yDesc,
                y.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnDivisiveNormalizationForward<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        normDesc: cudnnLRNDescriptor_t,
        mode: cudnnDivNormMode_t,
        alpha: T,
        xDesc: cudnnTensorDescriptor_t,
        x: U,
        means: V,
        mut temp: W,
        mut temp2: X,
        beta: Y,
        yDesc: cudnnTensorDescriptor_t,
        mut y: Z,
    ) -> Result<(), crate::sys::cudnnStatus_t> {
        let status = unsafe {
            crate::sys::cudnnDivisiveNormalizationForward(
                self.handle,
                normDesc,
                mode,
                alpha.as_const_ptr() as *const ::std::os::raw::c_void,
                xDesc,
                x.as_const_ptr() as *const ::std::os::raw::c_void,
                means.as_const_ptr() as *const ::std::os::raw::c_void,
                temp.as_mut_ptr() as *mut ::std::os::raw::c_void,
                temp2.as_mut_ptr() as *mut ::std::os::raw::c_void,
                beta.as_const_ptr() as *const ::std::os::raw::c_void,
                yDesc,
                y.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnBatchNormalizationForwardInference<
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
        mode: cudnnBatchNormMode_t,
        alpha: T,
        beta: U,
        xDesc: cudnnTensorDescriptor_t,
        x: V,
        yDesc: cudnnTensorDescriptor_t,
        mut y: W,
        bnScaleBiasMeanVarDesc: cudnnTensorDescriptor_t,
        bnScale: X,
        bnBias: Y,
        estimatedMean: Z,
        estimatedVariance: A,
        epsilon: f64,
    ) -> Result<(), crate::sys::cudnnStatus_t> {
        let status = unsafe {
            crate::sys::cudnnBatchNormalizationForwardInference(
                self.handle,
                mode,
                alpha.as_const_ptr() as *const ::std::os::raw::c_void,
                beta.as_const_ptr() as *const ::std::os::raw::c_void,
                xDesc,
                x.as_const_ptr() as *const ::std::os::raw::c_void,
                yDesc,
                y.as_mut_ptr() as *mut ::std::os::raw::c_void,
                bnScaleBiasMeanVarDesc,
                bnScale.as_const_ptr() as *const ::std::os::raw::c_void,
                bnBias.as_const_ptr() as *const ::std::os::raw::c_void,
                estimatedMean.as_const_ptr() as *const ::std::os::raw::c_void,
                estimatedVariance.as_const_ptr() as *const ::std::os::raw::c_void,
                epsilon,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnNormalizationForwardInference<
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
        mode: cudnnNormMode_t,
        normOps: cudnnNormOps_t,
        algo: cudnnNormAlgo_t,
        alpha: T,
        beta: U,
        xDesc: cudnnTensorDescriptor_t,
        x: V,
        normScaleBiasDesc: cudnnTensorDescriptor_t,
        normScale: W,
        normBias: X,
        normMeanVarDesc: cudnnTensorDescriptor_t,
        estimatedMean: Y,
        estimatedVariance: Z,
        zDesc: cudnnTensorDescriptor_t,
        z: A,
        activationDesc: cudnnActivationDescriptor_t,
        yDesc: cudnnTensorDescriptor_t,
        mut y: B,
        epsilon: f64,
        groupCnt: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cudnnStatus_t> {
        let status = unsafe {
            crate::sys::cudnnNormalizationForwardInference(
                self.handle,
                mode,
                normOps,
                algo,
                alpha.as_const_ptr() as *const ::std::os::raw::c_void,
                beta.as_const_ptr() as *const ::std::os::raw::c_void,
                xDesc,
                x.as_const_ptr() as *const ::std::os::raw::c_void,
                normScaleBiasDesc,
                normScale.as_const_ptr() as *const ::std::os::raw::c_void,
                normBias.as_const_ptr() as *const ::std::os::raw::c_void,
                normMeanVarDesc,
                estimatedMean.as_const_ptr() as *const ::std::os::raw::c_void,
                estimatedVariance.as_const_ptr() as *const ::std::os::raw::c_void,
                zDesc,
                z.as_const_ptr() as *const ::std::os::raw::c_void,
                activationDesc,
                yDesc,
                y.as_mut_ptr() as *mut ::std::os::raw::c_void,
                epsilon,
                groupCnt,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnSpatialTfGridGeneratorForward<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        stDesc: cudnnSpatialTransformerDescriptor_t,
        theta: T,
        mut grid: U,
    ) -> Result<(), crate::sys::cudnnStatus_t> {
        let status = unsafe {
            crate::sys::cudnnSpatialTfGridGeneratorForward(
                self.handle,
                stDesc,
                theta.as_const_ptr() as *const ::std::os::raw::c_void,
                grid.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnSpatialTfSamplerForward<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        stDesc: cudnnSpatialTransformerDescriptor_t,
        alpha: T,
        xDesc: cudnnTensorDescriptor_t,
        x: U,
        grid: V,
        beta: W,
        yDesc: cudnnTensorDescriptor_t,
        mut y: X,
    ) -> Result<(), crate::sys::cudnnStatus_t> {
        let status = unsafe {
            crate::sys::cudnnSpatialTfSamplerForward(
                self.handle,
                stDesc,
                alpha.as_const_ptr() as *const ::std::os::raw::c_void,
                xDesc,
                x.as_const_ptr() as *const ::std::os::raw::c_void,
                grid.as_const_ptr() as *const ::std::os::raw::c_void,
                beta.as_const_ptr() as *const ::std::os::raw::c_void,
                yDesc,
                y.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnDropoutGetStatesSize(&self) -> Result<usize, crate::sys::cudnnStatus_t> {
        let mut out_1: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
        let status = unsafe {
            crate::sys::cudnnDropoutGetStatesSize(self.handle, out_1.as_mut_ptr() as *mut _)
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            unsafe { Ok(out_1.assume_init()) }
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnDropoutForward<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dropoutDesc: cudnnDropoutDescriptor_t,
        xdesc: cudnnTensorDescriptor_t,
        x: T,
        ydesc: cudnnTensorDescriptor_t,
        mut y: U,
        mut reserveSpace: V,
        reserveSpaceSizeInBytes: usize,
    ) -> Result<(), crate::sys::cudnnStatus_t> {
        let status = unsafe {
            crate::sys::cudnnDropoutForward(
                self.handle,
                dropoutDesc,
                xdesc,
                x.as_const_ptr() as *const ::std::os::raw::c_void,
                ydesc,
                y.as_mut_ptr() as *mut ::std::os::raw::c_void,
                reserveSpace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                reserveSpaceSizeInBytes,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnSoftmaxBackward<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        algo: cudnnSoftmaxAlgorithm_t,
        mode: cudnnSoftmaxMode_t,
        alpha: T,
        yDesc: cudnnTensorDescriptor_t,
        y: U,
        dyDesc: cudnnTensorDescriptor_t,
        dy: V,
        beta: W,
        dxDesc: cudnnTensorDescriptor_t,
        mut dx: X,
    ) -> Result<(), crate::sys::cudnnStatus_t> {
        let status = unsafe {
            crate::sys::cudnnSoftmaxBackward(
                self.handle,
                algo,
                mode,
                alpha.as_const_ptr() as *const ::std::os::raw::c_void,
                yDesc,
                y.as_const_ptr() as *const ::std::os::raw::c_void,
                dyDesc,
                dy.as_const_ptr() as *const ::std::os::raw::c_void,
                beta.as_const_ptr() as *const ::std::os::raw::c_void,
                dxDesc,
                dx.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnPoolingBackward<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        poolingDesc: cudnnPoolingDescriptor_t,
        alpha: T,
        yDesc: cudnnTensorDescriptor_t,
        y: U,
        dyDesc: cudnnTensorDescriptor_t,
        dy: V,
        xDesc: cudnnTensorDescriptor_t,
        x: W,
        beta: X,
        dxDesc: cudnnTensorDescriptor_t,
        mut dx: Y,
    ) -> Result<(), crate::sys::cudnnStatus_t> {
        let status = unsafe {
            crate::sys::cudnnPoolingBackward(
                self.handle,
                poolingDesc,
                alpha.as_const_ptr() as *const ::std::os::raw::c_void,
                yDesc,
                y.as_const_ptr() as *const ::std::os::raw::c_void,
                dyDesc,
                dy.as_const_ptr() as *const ::std::os::raw::c_void,
                xDesc,
                x.as_const_ptr() as *const ::std::os::raw::c_void,
                beta.as_const_ptr() as *const ::std::os::raw::c_void,
                dxDesc,
                dx.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnActivationBackward<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        activationDesc: cudnnActivationDescriptor_t,
        alpha: T,
        yDesc: cudnnTensorDescriptor_t,
        y: U,
        dyDesc: cudnnTensorDescriptor_t,
        dy: V,
        xDesc: cudnnTensorDescriptor_t,
        x: W,
        beta: X,
        dxDesc: cudnnTensorDescriptor_t,
        mut dx: Y,
    ) -> Result<(), crate::sys::cudnnStatus_t> {
        let status = unsafe {
            crate::sys::cudnnActivationBackward(
                self.handle,
                activationDesc,
                alpha.as_const_ptr() as *const ::std::os::raw::c_void,
                yDesc,
                y.as_const_ptr() as *const ::std::os::raw::c_void,
                dyDesc,
                dy.as_const_ptr() as *const ::std::os::raw::c_void,
                xDesc,
                x.as_const_ptr() as *const ::std::os::raw::c_void,
                beta.as_const_ptr() as *const ::std::os::raw::c_void,
                dxDesc,
                dx.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnLRNCrossChannelBackward<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        normDesc: cudnnLRNDescriptor_t,
        lrnMode: cudnnLRNMode_t,
        alpha: T,
        yDesc: cudnnTensorDescriptor_t,
        y: U,
        dyDesc: cudnnTensorDescriptor_t,
        dy: V,
        xDesc: cudnnTensorDescriptor_t,
        x: W,
        beta: X,
        dxDesc: cudnnTensorDescriptor_t,
        mut dx: Y,
    ) -> Result<(), crate::sys::cudnnStatus_t> {
        let status = unsafe {
            crate::sys::cudnnLRNCrossChannelBackward(
                self.handle,
                normDesc,
                lrnMode,
                alpha.as_const_ptr() as *const ::std::os::raw::c_void,
                yDesc,
                y.as_const_ptr() as *const ::std::os::raw::c_void,
                dyDesc,
                dy.as_const_ptr() as *const ::std::os::raw::c_void,
                xDesc,
                x.as_const_ptr() as *const ::std::os::raw::c_void,
                beta.as_const_ptr() as *const ::std::os::raw::c_void,
                dxDesc,
                dx.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnDivisiveNormalizationBackward<
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
        normDesc: cudnnLRNDescriptor_t,
        mode: cudnnDivNormMode_t,
        alpha: T,
        xDesc: cudnnTensorDescriptor_t,
        x: U,
        means: V,
        dy: W,
        mut temp: X,
        mut temp2: Y,
        beta: Z,
        dXdMeansDesc: cudnnTensorDescriptor_t,
        mut dx: A,
        mut dMeans: B,
    ) -> Result<(), crate::sys::cudnnStatus_t> {
        let status = unsafe {
            crate::sys::cudnnDivisiveNormalizationBackward(
                self.handle,
                normDesc,
                mode,
                alpha.as_const_ptr() as *const ::std::os::raw::c_void,
                xDesc,
                x.as_const_ptr() as *const ::std::os::raw::c_void,
                means.as_const_ptr() as *const ::std::os::raw::c_void,
                dy.as_const_ptr() as *const ::std::os::raw::c_void,
                temp.as_mut_ptr() as *mut ::std::os::raw::c_void,
                temp2.as_mut_ptr() as *mut ::std::os::raw::c_void,
                beta.as_const_ptr() as *const ::std::os::raw::c_void,
                dXdMeansDesc,
                dx.as_mut_ptr() as *mut ::std::os::raw::c_void,
                dMeans.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnGetBatchNormalizationForwardTrainingExWorkspaceSize(
        &self,
        mode: cudnnBatchNormMode_t,
        bnOps: cudnnBatchNormOps_t,
        xDesc: cudnnTensorDescriptor_t,
        zDesc: cudnnTensorDescriptor_t,
        yDesc: cudnnTensorDescriptor_t,
        bnScaleBiasMeanVarDesc: cudnnTensorDescriptor_t,
        activationDesc: cudnnActivationDescriptor_t,
    ) -> Result<usize, crate::sys::cudnnStatus_t> {
        let mut out_8: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
        let status = unsafe {
            crate::sys::cudnnGetBatchNormalizationForwardTrainingExWorkspaceSize(
                self.handle,
                mode,
                bnOps,
                xDesc,
                zDesc,
                yDesc,
                bnScaleBiasMeanVarDesc,
                activationDesc,
                out_8.as_mut_ptr() as *mut _,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            unsafe { Ok(out_8.assume_init()) }
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnGetBatchNormalizationBackwardExWorkspaceSize(
        &self,
        mode: cudnnBatchNormMode_t,
        bnOps: cudnnBatchNormOps_t,
        xDesc: cudnnTensorDescriptor_t,
        yDesc: cudnnTensorDescriptor_t,
        dyDesc: cudnnTensorDescriptor_t,
        dzDesc: cudnnTensorDescriptor_t,
        dxDesc: cudnnTensorDescriptor_t,
        dBnScaleBiasDesc: cudnnTensorDescriptor_t,
        activationDesc: cudnnActivationDescriptor_t,
    ) -> Result<usize, crate::sys::cudnnStatus_t> {
        let mut out_10: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
        let status = unsafe {
            crate::sys::cudnnGetBatchNormalizationBackwardExWorkspaceSize(
                self.handle,
                mode,
                bnOps,
                xDesc,
                yDesc,
                dyDesc,
                dzDesc,
                dxDesc,
                dBnScaleBiasDesc,
                activationDesc,
                out_10.as_mut_ptr() as *mut _,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            unsafe { Ok(out_10.assume_init()) }
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnGetBatchNormalizationTrainingExReserveSpaceSize(
        &self,
        mode: cudnnBatchNormMode_t,
        bnOps: cudnnBatchNormOps_t,
        activationDesc: cudnnActivationDescriptor_t,
        xDesc: cudnnTensorDescriptor_t,
    ) -> Result<usize, crate::sys::cudnnStatus_t> {
        let mut out_5: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
        let status = unsafe {
            crate::sys::cudnnGetBatchNormalizationTrainingExReserveSpaceSize(
                self.handle,
                mode,
                bnOps,
                activationDesc,
                xDesc,
                out_5.as_mut_ptr() as *mut _,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            unsafe { Ok(out_5.assume_init()) }
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnBatchNormalizationForwardTraining<
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
    >(
        &self,
        mode: cudnnBatchNormMode_t,
        alpha: T,
        beta: U,
        xDesc: cudnnTensorDescriptor_t,
        x: V,
        yDesc: cudnnTensorDescriptor_t,
        mut y: W,
        bnScaleBiasMeanVarDesc: cudnnTensorDescriptor_t,
        bnScale: X,
        bnBias: Y,
        exponentialAverageFactor: f64,
        mut resultRunningMean: Z,
        mut resultRunningVariance: A,
        epsilon: f64,
        mut resultSaveMean: B,
        mut resultSaveInvVariance: C,
    ) -> Result<(), crate::sys::cudnnStatus_t> {
        let status = unsafe {
            crate::sys::cudnnBatchNormalizationForwardTraining(
                self.handle,
                mode,
                alpha.as_const_ptr() as *const ::std::os::raw::c_void,
                beta.as_const_ptr() as *const ::std::os::raw::c_void,
                xDesc,
                x.as_const_ptr() as *const ::std::os::raw::c_void,
                yDesc,
                y.as_mut_ptr() as *mut ::std::os::raw::c_void,
                bnScaleBiasMeanVarDesc,
                bnScale.as_const_ptr() as *const ::std::os::raw::c_void,
                bnBias.as_const_ptr() as *const ::std::os::raw::c_void,
                exponentialAverageFactor,
                resultRunningMean.as_mut_ptr() as *mut ::std::os::raw::c_void,
                resultRunningVariance.as_mut_ptr() as *mut ::std::os::raw::c_void,
                epsilon,
                resultSaveMean.as_mut_ptr() as *mut ::std::os::raw::c_void,
                resultSaveInvVariance.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnBatchNormalizationForwardTrainingEx<
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
    >(
        &self,
        mode: cudnnBatchNormMode_t,
        bnOps: cudnnBatchNormOps_t,
        alpha: T,
        beta: U,
        xDesc: cudnnTensorDescriptor_t,
        xData: V,
        zDesc: cudnnTensorDescriptor_t,
        zData: W,
        yDesc: cudnnTensorDescriptor_t,
        mut yData: X,
        bnScaleBiasMeanVarDesc: cudnnTensorDescriptor_t,
        bnScale: Y,
        bnBias: Z,
        exponentialAverageFactor: f64,
        mut resultRunningMean: A,
        mut resultRunningVariance: B,
        epsilon: f64,
        mut resultSaveMean: C,
        mut resultSaveInvVariance: D,
        activationDesc: cudnnActivationDescriptor_t,
        mut workspace: E,
        workSpaceSizeInBytes: usize,
        mut reserveSpace: F,
        reserveSpaceSizeInBytes: usize,
    ) -> Result<(), crate::sys::cudnnStatus_t> {
        let status = unsafe {
            crate::sys::cudnnBatchNormalizationForwardTrainingEx(
                self.handle,
                mode,
                bnOps,
                alpha.as_const_ptr() as *const ::std::os::raw::c_void,
                beta.as_const_ptr() as *const ::std::os::raw::c_void,
                xDesc,
                xData.as_const_ptr() as *const ::std::os::raw::c_void,
                zDesc,
                zData.as_const_ptr() as *const ::std::os::raw::c_void,
                yDesc,
                yData.as_mut_ptr() as *mut ::std::os::raw::c_void,
                bnScaleBiasMeanVarDesc,
                bnScale.as_const_ptr() as *const ::std::os::raw::c_void,
                bnBias.as_const_ptr() as *const ::std::os::raw::c_void,
                exponentialAverageFactor,
                resultRunningMean.as_mut_ptr() as *mut ::std::os::raw::c_void,
                resultRunningVariance.as_mut_ptr() as *mut ::std::os::raw::c_void,
                epsilon,
                resultSaveMean.as_mut_ptr() as *mut ::std::os::raw::c_void,
                resultSaveInvVariance.as_mut_ptr() as *mut ::std::os::raw::c_void,
                activationDesc,
                workspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                workSpaceSizeInBytes,
                reserveSpace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                reserveSpaceSizeInBytes,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnBatchNormalizationBackward<
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
        mode: cudnnBatchNormMode_t,
        alphaDataDiff: T,
        betaDataDiff: U,
        alphaParamDiff: V,
        betaParamDiff: W,
        xDesc: cudnnTensorDescriptor_t,
        x: X,
        dyDesc: cudnnTensorDescriptor_t,
        dy: Y,
        dxDesc: cudnnTensorDescriptor_t,
        mut dx: Z,
        dBnScaleBiasDesc: cudnnTensorDescriptor_t,
        bnScale: A,
        mut dBnScaleResult: B,
        mut dBnBiasResult: C,
        epsilon: f64,
        savedMean: D,
        savedInvVariance: E,
    ) -> Result<(), crate::sys::cudnnStatus_t> {
        let status = unsafe {
            crate::sys::cudnnBatchNormalizationBackward(
                self.handle,
                mode,
                alphaDataDiff.as_const_ptr() as *const ::std::os::raw::c_void,
                betaDataDiff.as_const_ptr() as *const ::std::os::raw::c_void,
                alphaParamDiff.as_const_ptr() as *const ::std::os::raw::c_void,
                betaParamDiff.as_const_ptr() as *const ::std::os::raw::c_void,
                xDesc,
                x.as_const_ptr() as *const ::std::os::raw::c_void,
                dyDesc,
                dy.as_const_ptr() as *const ::std::os::raw::c_void,
                dxDesc,
                dx.as_mut_ptr() as *mut ::std::os::raw::c_void,
                dBnScaleBiasDesc,
                bnScale.as_const_ptr() as *const ::std::os::raw::c_void,
                dBnScaleResult.as_mut_ptr() as *mut ::std::os::raw::c_void,
                dBnBiasResult.as_mut_ptr() as *mut ::std::os::raw::c_void,
                epsilon,
                savedMean.as_const_ptr() as *const ::std::os::raw::c_void,
                savedInvVariance.as_const_ptr() as *const ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnBatchNormalizationBackwardEx<
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
        T14: ::cuda_libs::types::CudaAsPtr,
        T15: ::cuda_libs::types::CudaAsPtr,
        T16: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        mode: cudnnBatchNormMode_t,
        bnOps: cudnnBatchNormOps_t,
        alphaDataDiff: T,
        betaDataDiff: U,
        alphaParamDiff: V,
        betaParamDiff: W,
        xDesc: cudnnTensorDescriptor_t,
        xData: X,
        yDesc: cudnnTensorDescriptor_t,
        yData: Y,
        dyDesc: cudnnTensorDescriptor_t,
        dyData: Z,
        dzDesc: cudnnTensorDescriptor_t,
        mut dzData: A,
        dxDesc: cudnnTensorDescriptor_t,
        mut dxData: B,
        dBnScaleBiasDesc: cudnnTensorDescriptor_t,
        bnScaleData: C,
        bnBiasData: D,
        mut dBnScaleData: E,
        mut dBnBiasData: F,
        epsilon: f64,
        savedMean: T13,
        savedInvVariance: T14,
        activationDesc: cudnnActivationDescriptor_t,
        mut workSpace: T15,
        workSpaceSizeInBytes: usize,
        mut reserveSpace: T16,
        reserveSpaceSizeInBytes: usize,
    ) -> Result<(), crate::sys::cudnnStatus_t> {
        let status = unsafe {
            crate::sys::cudnnBatchNormalizationBackwardEx(
                self.handle,
                mode,
                bnOps,
                alphaDataDiff.as_const_ptr() as *const ::std::os::raw::c_void,
                betaDataDiff.as_const_ptr() as *const ::std::os::raw::c_void,
                alphaParamDiff.as_const_ptr() as *const ::std::os::raw::c_void,
                betaParamDiff.as_const_ptr() as *const ::std::os::raw::c_void,
                xDesc,
                xData.as_const_ptr() as *const ::std::os::raw::c_void,
                yDesc,
                yData.as_const_ptr() as *const ::std::os::raw::c_void,
                dyDesc,
                dyData.as_const_ptr() as *const ::std::os::raw::c_void,
                dzDesc,
                dzData.as_mut_ptr() as *mut ::std::os::raw::c_void,
                dxDesc,
                dxData.as_mut_ptr() as *mut ::std::os::raw::c_void,
                dBnScaleBiasDesc,
                bnScaleData.as_const_ptr() as *const ::std::os::raw::c_void,
                bnBiasData.as_const_ptr() as *const ::std::os::raw::c_void,
                dBnScaleData.as_mut_ptr() as *mut ::std::os::raw::c_void,
                dBnBiasData.as_mut_ptr() as *mut ::std::os::raw::c_void,
                epsilon,
                savedMean.as_const_ptr() as *const ::std::os::raw::c_void,
                savedInvVariance.as_const_ptr() as *const ::std::os::raw::c_void,
                activationDesc,
                workSpace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                workSpaceSizeInBytes,
                reserveSpace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                reserveSpaceSizeInBytes,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnGetNormalizationForwardTrainingWorkspaceSize(
        &self,
        mode: cudnnNormMode_t,
        normOps: cudnnNormOps_t,
        algo: cudnnNormAlgo_t,
        xDesc: cudnnTensorDescriptor_t,
        zDesc: cudnnTensorDescriptor_t,
        yDesc: cudnnTensorDescriptor_t,
        normScaleBiasDesc: cudnnTensorDescriptor_t,
        activationDesc: cudnnActivationDescriptor_t,
        normMeanVarDesc: cudnnTensorDescriptor_t,
        groupCnt: ::std::os::raw::c_int,
    ) -> Result<usize, crate::sys::cudnnStatus_t> {
        let mut out_10: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
        let status = unsafe {
            crate::sys::cudnnGetNormalizationForwardTrainingWorkspaceSize(
                self.handle,
                mode,
                normOps,
                algo,
                xDesc,
                zDesc,
                yDesc,
                normScaleBiasDesc,
                activationDesc,
                normMeanVarDesc,
                out_10.as_mut_ptr() as *mut _,
                groupCnt,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            unsafe { Ok(out_10.assume_init()) }
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnGetNormalizationBackwardWorkspaceSize(
        &self,
        mode: cudnnNormMode_t,
        normOps: cudnnNormOps_t,
        algo: cudnnNormAlgo_t,
        xDesc: cudnnTensorDescriptor_t,
        yDesc: cudnnTensorDescriptor_t,
        dyDesc: cudnnTensorDescriptor_t,
        dzDesc: cudnnTensorDescriptor_t,
        dxDesc: cudnnTensorDescriptor_t,
        dNormScaleBiasDesc: cudnnTensorDescriptor_t,
        activationDesc: cudnnActivationDescriptor_t,
        normMeanVarDesc: cudnnTensorDescriptor_t,
        groupCnt: ::std::os::raw::c_int,
    ) -> Result<usize, crate::sys::cudnnStatus_t> {
        let mut out_12: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
        let status = unsafe {
            crate::sys::cudnnGetNormalizationBackwardWorkspaceSize(
                self.handle,
                mode,
                normOps,
                algo,
                xDesc,
                yDesc,
                dyDesc,
                dzDesc,
                dxDesc,
                dNormScaleBiasDesc,
                activationDesc,
                normMeanVarDesc,
                out_12.as_mut_ptr() as *mut _,
                groupCnt,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            unsafe { Ok(out_12.assume_init()) }
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnGetNormalizationTrainingReserveSpaceSize(
        &self,
        mode: cudnnNormMode_t,
        normOps: cudnnNormOps_t,
        algo: cudnnNormAlgo_t,
        activationDesc: cudnnActivationDescriptor_t,
        xDesc: cudnnTensorDescriptor_t,
        groupCnt: ::std::os::raw::c_int,
    ) -> Result<usize, crate::sys::cudnnStatus_t> {
        let mut out_6: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
        let status = unsafe {
            crate::sys::cudnnGetNormalizationTrainingReserveSpaceSize(
                self.handle,
                mode,
                normOps,
                algo,
                activationDesc,
                xDesc,
                out_6.as_mut_ptr() as *mut _,
                groupCnt,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            unsafe { Ok(out_6.assume_init()) }
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnNormalizationForwardTraining<
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
    >(
        &self,
        mode: cudnnNormMode_t,
        normOps: cudnnNormOps_t,
        algo: cudnnNormAlgo_t,
        alpha: T,
        beta: U,
        xDesc: cudnnTensorDescriptor_t,
        xData: V,
        normScaleBiasDesc: cudnnTensorDescriptor_t,
        normScale: W,
        normBias: X,
        exponentialAverageFactor: f64,
        normMeanVarDesc: cudnnTensorDescriptor_t,
        mut resultRunningMean: Y,
        mut resultRunningVariance: Z,
        epsilon: f64,
        mut resultSaveMean: A,
        mut resultSaveInvVariance: B,
        activationDesc: cudnnActivationDescriptor_t,
        zDesc: cudnnTensorDescriptor_t,
        zData: C,
        yDesc: cudnnTensorDescriptor_t,
        mut yData: D,
        mut workspace: E,
        workSpaceSizeInBytes: usize,
        mut reserveSpace: F,
        reserveSpaceSizeInBytes: usize,
        groupCnt: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cudnnStatus_t> {
        let status = unsafe {
            crate::sys::cudnnNormalizationForwardTraining(
                self.handle,
                mode,
                normOps,
                algo,
                alpha.as_const_ptr() as *const ::std::os::raw::c_void,
                beta.as_const_ptr() as *const ::std::os::raw::c_void,
                xDesc,
                xData.as_const_ptr() as *const ::std::os::raw::c_void,
                normScaleBiasDesc,
                normScale.as_const_ptr() as *const ::std::os::raw::c_void,
                normBias.as_const_ptr() as *const ::std::os::raw::c_void,
                exponentialAverageFactor,
                normMeanVarDesc,
                resultRunningMean.as_mut_ptr() as *mut ::std::os::raw::c_void,
                resultRunningVariance.as_mut_ptr() as *mut ::std::os::raw::c_void,
                epsilon,
                resultSaveMean.as_mut_ptr() as *mut ::std::os::raw::c_void,
                resultSaveInvVariance.as_mut_ptr() as *mut ::std::os::raw::c_void,
                activationDesc,
                zDesc,
                zData.as_const_ptr() as *const ::std::os::raw::c_void,
                yDesc,
                yData.as_mut_ptr() as *mut ::std::os::raw::c_void,
                workspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                workSpaceSizeInBytes,
                reserveSpace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                reserveSpaceSizeInBytes,
                groupCnt,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnNormalizationBackward<
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
        T14: ::cuda_libs::types::CudaAsPtr,
        T15: ::cuda_libs::types::CudaAsPtr,
        T16: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        mode: cudnnNormMode_t,
        normOps: cudnnNormOps_t,
        algo: cudnnNormAlgo_t,
        alphaDataDiff: T,
        betaDataDiff: U,
        alphaParamDiff: V,
        betaParamDiff: W,
        xDesc: cudnnTensorDescriptor_t,
        xData: X,
        yDesc: cudnnTensorDescriptor_t,
        yData: Y,
        dyDesc: cudnnTensorDescriptor_t,
        dyData: Z,
        dzDesc: cudnnTensorDescriptor_t,
        mut dzData: A,
        dxDesc: cudnnTensorDescriptor_t,
        mut dxData: B,
        dNormScaleBiasDesc: cudnnTensorDescriptor_t,
        normScaleData: C,
        normBiasData: D,
        mut dNormScaleData: E,
        mut dNormBiasData: F,
        epsilon: f64,
        normMeanVarDesc: cudnnTensorDescriptor_t,
        savedMean: T13,
        savedInvVariance: T14,
        activationDesc: cudnnActivationDescriptor_t,
        mut workSpace: T15,
        workSpaceSizeInBytes: usize,
        mut reserveSpace: T16,
        reserveSpaceSizeInBytes: usize,
        groupCnt: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cudnnStatus_t> {
        let status = unsafe {
            crate::sys::cudnnNormalizationBackward(
                self.handle,
                mode,
                normOps,
                algo,
                alphaDataDiff.as_const_ptr() as *const ::std::os::raw::c_void,
                betaDataDiff.as_const_ptr() as *const ::std::os::raw::c_void,
                alphaParamDiff.as_const_ptr() as *const ::std::os::raw::c_void,
                betaParamDiff.as_const_ptr() as *const ::std::os::raw::c_void,
                xDesc,
                xData.as_const_ptr() as *const ::std::os::raw::c_void,
                yDesc,
                yData.as_const_ptr() as *const ::std::os::raw::c_void,
                dyDesc,
                dyData.as_const_ptr() as *const ::std::os::raw::c_void,
                dzDesc,
                dzData.as_mut_ptr() as *mut ::std::os::raw::c_void,
                dxDesc,
                dxData.as_mut_ptr() as *mut ::std::os::raw::c_void,
                dNormScaleBiasDesc,
                normScaleData.as_const_ptr() as *const ::std::os::raw::c_void,
                normBiasData.as_const_ptr() as *const ::std::os::raw::c_void,
                dNormScaleData.as_mut_ptr() as *mut ::std::os::raw::c_void,
                dNormBiasData.as_mut_ptr() as *mut ::std::os::raw::c_void,
                epsilon,
                normMeanVarDesc,
                savedMean.as_const_ptr() as *const ::std::os::raw::c_void,
                savedInvVariance.as_const_ptr() as *const ::std::os::raw::c_void,
                activationDesc,
                workSpace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                workSpaceSizeInBytes,
                reserveSpace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                reserveSpaceSizeInBytes,
                groupCnt,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnSpatialTfGridGeneratorBackward<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        stDesc: cudnnSpatialTransformerDescriptor_t,
        dgrid: T,
        mut dtheta: U,
    ) -> Result<(), crate::sys::cudnnStatus_t> {
        let status = unsafe {
            crate::sys::cudnnSpatialTfGridGeneratorBackward(
                self.handle,
                stDesc,
                dgrid.as_const_ptr() as *const ::std::os::raw::c_void,
                dtheta.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnSpatialTfSamplerBackward<
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
        stDesc: cudnnSpatialTransformerDescriptor_t,
        alpha: T,
        xDesc: cudnnTensorDescriptor_t,
        x: U,
        beta: V,
        dxDesc: cudnnTensorDescriptor_t,
        mut dx: W,
        alphaDgrid: X,
        dyDesc: cudnnTensorDescriptor_t,
        dy: Y,
        grid: Z,
        betaDgrid: A,
        mut dgrid: B,
    ) -> Result<(), crate::sys::cudnnStatus_t> {
        let status = unsafe {
            crate::sys::cudnnSpatialTfSamplerBackward(
                self.handle,
                stDesc,
                alpha.as_const_ptr() as *const ::std::os::raw::c_void,
                xDesc,
                x.as_const_ptr() as *const ::std::os::raw::c_void,
                beta.as_const_ptr() as *const ::std::os::raw::c_void,
                dxDesc,
                dx.as_mut_ptr() as *mut ::std::os::raw::c_void,
                alphaDgrid.as_const_ptr() as *const ::std::os::raw::c_void,
                dyDesc,
                dy.as_const_ptr() as *const ::std::os::raw::c_void,
                grid.as_const_ptr() as *const ::std::os::raw::c_void,
                betaDgrid.as_const_ptr() as *const ::std::os::raw::c_void,
                dgrid.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnDropoutBackward<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        dropoutDesc: cudnnDropoutDescriptor_t,
        dydesc: cudnnTensorDescriptor_t,
        dy: T,
        dxdesc: cudnnTensorDescriptor_t,
        mut dx: U,
        mut reserveSpace: V,
        reserveSpaceSizeInBytes: usize,
    ) -> Result<(), crate::sys::cudnnStatus_t> {
        let status = unsafe {
            crate::sys::cudnnDropoutBackward(
                self.handle,
                dropoutDesc,
                dydesc,
                dy.as_const_ptr() as *const ::std::os::raw::c_void,
                dxdesc,
                dx.as_mut_ptr() as *mut ::std::os::raw::c_void,
                reserveSpace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                reserveSpaceSizeInBytes,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnBuildRNNDynamic(
        &self,
        rnnDesc: cudnnRNNDescriptor_t,
        miniBatch: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cudnnStatus_t> {
        let status = unsafe { crate::sys::cudnnBuildRNNDynamic(self.handle, rnnDesc, miniBatch) };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnGetRNNTempSpaceSizes(
        &self,
        rnnDesc: cudnnRNNDescriptor_t,
        fwdMode: cudnnForwardMode_t,
        xDesc: cudnnRNNDataDescriptor_t,
    ) -> Result<(usize, usize), crate::sys::cudnnStatus_t> {
        let mut out_4: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
        let mut out_5: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
        let status = unsafe {
            crate::sys::cudnnGetRNNTempSpaceSizes(
                self.handle,
                rnnDesc,
                fwdMode,
                xDesc,
                out_4.as_mut_ptr() as *mut _,
                out_5.as_mut_ptr() as *mut _,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            unsafe { Ok((out_4.assume_init(), out_5.assume_init())) }
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnGetRNNWeightSpaceSize(
        &self,
        rnnDesc: cudnnRNNDescriptor_t,
    ) -> Result<usize, crate::sys::cudnnStatus_t> {
        let mut out_2: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
        let status = unsafe {
            crate::sys::cudnnGetRNNWeightSpaceSize(
                self.handle,
                rnnDesc,
                out_2.as_mut_ptr() as *mut _,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            unsafe { Ok(out_2.assume_init()) }
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnGetRNNWeightParams<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        rnnDesc: cudnnRNNDescriptor_t,
        pseudoLayer: i32,
        weightSpaceSize: usize,
        weightSpace: T,
        linLayerID: i32,
        mDesc: cudnnTensorDescriptor_t,
        mut mAddr: U,
        bDesc: cudnnTensorDescriptor_t,
        mut bAddr: V,
    ) -> Result<(), crate::sys::cudnnStatus_t> {
        let status = unsafe {
            crate::sys::cudnnGetRNNWeightParams(
                self.handle,
                rnnDesc,
                pseudoLayer,
                weightSpaceSize,
                weightSpace.as_const_ptr() as *const ::std::os::raw::c_void,
                linLayerID,
                mDesc,
                mAddr.as_mut_ptr() as *mut *mut ::std::os::raw::c_void,
                bDesc,
                bAddr.as_mut_ptr() as *mut *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnRNNForward<
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
    >(
        &self,
        rnnDesc: cudnnRNNDescriptor_t,
        fwdMode: cudnnForwardMode_t,
        devSeqLengths: T,
        xDesc: cudnnRNNDataDescriptor_t,
        x: U,
        yDesc: cudnnRNNDataDescriptor_t,
        mut y: V,
        hDesc: cudnnTensorDescriptor_t,
        hx: W,
        mut hy: X,
        cDesc: cudnnTensorDescriptor_t,
        cx: Y,
        mut cy: Z,
        weightSpaceSize: usize,
        weightSpace: A,
        workSpaceSize: usize,
        mut workSpace: B,
        reserveSpaceSize: usize,
        mut reserveSpace: C,
    ) -> Result<(), crate::sys::cudnnStatus_t> {
        let status = unsafe {
            crate::sys::cudnnRNNForward(
                self.handle,
                rnnDesc,
                fwdMode,
                devSeqLengths.as_const_ptr() as *const i32,
                xDesc,
                x.as_const_ptr() as *const ::std::os::raw::c_void,
                yDesc,
                y.as_mut_ptr() as *mut ::std::os::raw::c_void,
                hDesc,
                hx.as_const_ptr() as *const ::std::os::raw::c_void,
                hy.as_mut_ptr() as *mut ::std::os::raw::c_void,
                cDesc,
                cx.as_const_ptr() as *const ::std::os::raw::c_void,
                cy.as_mut_ptr() as *mut ::std::os::raw::c_void,
                weightSpaceSize,
                weightSpace.as_const_ptr() as *const ::std::os::raw::c_void,
                workSpaceSize,
                workSpace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                reserveSpaceSize,
                reserveSpace.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnGetMultiHeadAttnBuffers(
        &self,
        attnDesc: cudnnAttnDescriptor_t,
    ) -> Result<(usize, usize, usize), crate::sys::cudnnStatus_t> {
        let mut out_2: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
        let mut out_3: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
        let mut out_4: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
        let status = unsafe {
            crate::sys::cudnnGetMultiHeadAttnBuffers(
                self.handle,
                attnDesc,
                out_2.as_mut_ptr() as *mut _,
                out_3.as_mut_ptr() as *mut _,
                out_4.as_mut_ptr() as *mut _,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            unsafe {
                Ok((
                    out_2.assume_init(),
                    out_3.assume_init(),
                    out_4.assume_init(),
                ))
            }
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnGetMultiHeadAttnWeights<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        attnDesc: cudnnAttnDescriptor_t,
        wKind: cudnnMultiHeadAttnWeightKind_t,
        weightSizeInBytes: usize,
        weights: T,
        wDesc: cudnnTensorDescriptor_t,
        mut wAddr: U,
    ) -> Result<(), crate::sys::cudnnStatus_t> {
        let status = unsafe {
            crate::sys::cudnnGetMultiHeadAttnWeights(
                self.handle,
                attnDesc,
                wKind,
                weightSizeInBytes,
                weights.as_const_ptr() as *const ::std::os::raw::c_void,
                wDesc,
                wAddr.as_mut_ptr() as *mut *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnMultiHeadAttnForward<
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
        attnDesc: cudnnAttnDescriptor_t,
        currIdx: ::std::os::raw::c_int,
        loWinIdx: T,
        hiWinIdx: U,
        devSeqLengthsQO: V,
        devSeqLengthsKV: W,
        qDesc: cudnnSeqDataDescriptor_t,
        queries: X,
        residuals: Y,
        kDesc: cudnnSeqDataDescriptor_t,
        keys: Z,
        vDesc: cudnnSeqDataDescriptor_t,
        values: A,
        oDesc: cudnnSeqDataDescriptor_t,
        mut out: B,
        weightSizeInBytes: usize,
        weights: C,
        workSpaceSizeInBytes: usize,
        mut workSpace: D,
        reserveSpaceSizeInBytes: usize,
        mut reserveSpace: E,
    ) -> Result<(), crate::sys::cudnnStatus_t> {
        let status = unsafe {
            crate::sys::cudnnMultiHeadAttnForward(
                self.handle,
                attnDesc,
                currIdx,
                loWinIdx.as_const_ptr() as *const ::std::os::raw::c_int,
                hiWinIdx.as_const_ptr() as *const ::std::os::raw::c_int,
                devSeqLengthsQO.as_const_ptr() as *const ::std::os::raw::c_int,
                devSeqLengthsKV.as_const_ptr() as *const ::std::os::raw::c_int,
                qDesc,
                queries.as_const_ptr() as *const ::std::os::raw::c_void,
                residuals.as_const_ptr() as *const ::std::os::raw::c_void,
                kDesc,
                keys.as_const_ptr() as *const ::std::os::raw::c_void,
                vDesc,
                values.as_const_ptr() as *const ::std::os::raw::c_void,
                oDesc,
                out.as_mut_ptr() as *mut ::std::os::raw::c_void,
                weightSizeInBytes,
                weights.as_const_ptr() as *const ::std::os::raw::c_void,
                workSpaceSizeInBytes,
                workSpace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                reserveSpaceSizeInBytes,
                reserveSpace.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnRNNBackwardData_v8<
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
    >(
        &self,
        rnnDesc: cudnnRNNDescriptor_t,
        devSeqLengths: T,
        yDesc: cudnnRNNDataDescriptor_t,
        y: U,
        dy: V,
        xDesc: cudnnRNNDataDescriptor_t,
        mut dx: W,
        hDesc: cudnnTensorDescriptor_t,
        hx: X,
        dhy: Y,
        mut dhx: Z,
        cDesc: cudnnTensorDescriptor_t,
        cx: A,
        dcy: B,
        mut dcx: C,
        weightSpaceSize: usize,
        weightSpace: D,
        workSpaceSize: usize,
        mut workSpace: E,
        reserveSpaceSize: usize,
        mut reserveSpace: F,
    ) -> Result<(), crate::sys::cudnnStatus_t> {
        let status = unsafe {
            crate::sys::cudnnRNNBackwardData_v8(
                self.handle,
                rnnDesc,
                devSeqLengths.as_const_ptr() as *const i32,
                yDesc,
                y.as_const_ptr() as *const ::std::os::raw::c_void,
                dy.as_const_ptr() as *const ::std::os::raw::c_void,
                xDesc,
                dx.as_mut_ptr() as *mut ::std::os::raw::c_void,
                hDesc,
                hx.as_const_ptr() as *const ::std::os::raw::c_void,
                dhy.as_const_ptr() as *const ::std::os::raw::c_void,
                dhx.as_mut_ptr() as *mut ::std::os::raw::c_void,
                cDesc,
                cx.as_const_ptr() as *const ::std::os::raw::c_void,
                dcy.as_const_ptr() as *const ::std::os::raw::c_void,
                dcx.as_mut_ptr() as *mut ::std::os::raw::c_void,
                weightSpaceSize,
                weightSpace.as_const_ptr() as *const ::std::os::raw::c_void,
                workSpaceSize,
                workSpace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                reserveSpaceSize,
                reserveSpace.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnRNNBackwardWeights_v8<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        rnnDesc: cudnnRNNDescriptor_t,
        addGrad: cudnnWgradMode_t,
        devSeqLengths: T,
        xDesc: cudnnRNNDataDescriptor_t,
        x: U,
        hDesc: cudnnTensorDescriptor_t,
        hx: V,
        yDesc: cudnnRNNDataDescriptor_t,
        y: W,
        weightSpaceSize: usize,
        mut dweightSpace: X,
        workSpaceSize: usize,
        mut workSpace: Y,
        reserveSpaceSize: usize,
        mut reserveSpace: Z,
    ) -> Result<(), crate::sys::cudnnStatus_t> {
        let status = unsafe {
            crate::sys::cudnnRNNBackwardWeights_v8(
                self.handle,
                rnnDesc,
                addGrad,
                devSeqLengths.as_const_ptr() as *const i32,
                xDesc,
                x.as_const_ptr() as *const ::std::os::raw::c_void,
                hDesc,
                hx.as_const_ptr() as *const ::std::os::raw::c_void,
                yDesc,
                y.as_const_ptr() as *const ::std::os::raw::c_void,
                weightSpaceSize,
                dweightSpace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                workSpaceSize,
                workSpace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                reserveSpaceSize,
                reserveSpace.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnMultiHeadAttnBackwardData<
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
        attnDesc: cudnnAttnDescriptor_t,
        loWinIdx: T,
        hiWinIdx: U,
        devSeqLengthsDQDO: V,
        devSeqLengthsDKDV: W,
        doDesc: cudnnSeqDataDescriptor_t,
        dout: X,
        dqDesc: cudnnSeqDataDescriptor_t,
        mut dqueries: Y,
        queries: Z,
        dkDesc: cudnnSeqDataDescriptor_t,
        mut dkeys: A,
        keys: B,
        dvDesc: cudnnSeqDataDescriptor_t,
        mut dvalues: C,
        values: D,
        weightSizeInBytes: usize,
        weights: E,
        workSpaceSizeInBytes: usize,
        mut workSpace: F,
        reserveSpaceSizeInBytes: usize,
        mut reserveSpace: T13,
    ) -> Result<(), crate::sys::cudnnStatus_t> {
        let status = unsafe {
            crate::sys::cudnnMultiHeadAttnBackwardData(
                self.handle,
                attnDesc,
                loWinIdx.as_const_ptr() as *const ::std::os::raw::c_int,
                hiWinIdx.as_const_ptr() as *const ::std::os::raw::c_int,
                devSeqLengthsDQDO.as_const_ptr() as *const ::std::os::raw::c_int,
                devSeqLengthsDKDV.as_const_ptr() as *const ::std::os::raw::c_int,
                doDesc,
                dout.as_const_ptr() as *const ::std::os::raw::c_void,
                dqDesc,
                dqueries.as_mut_ptr() as *mut ::std::os::raw::c_void,
                queries.as_const_ptr() as *const ::std::os::raw::c_void,
                dkDesc,
                dkeys.as_mut_ptr() as *mut ::std::os::raw::c_void,
                keys.as_const_ptr() as *const ::std::os::raw::c_void,
                dvDesc,
                dvalues.as_mut_ptr() as *mut ::std::os::raw::c_void,
                values.as_const_ptr() as *const ::std::os::raw::c_void,
                weightSizeInBytes,
                weights.as_const_ptr() as *const ::std::os::raw::c_void,
                workSpaceSizeInBytes,
                workSpace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                reserveSpaceSizeInBytes,
                reserveSpace.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnMultiHeadAttnBackwardWeights<
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
        attnDesc: cudnnAttnDescriptor_t,
        addGrad: cudnnWgradMode_t,
        qDesc: cudnnSeqDataDescriptor_t,
        queries: T,
        kDesc: cudnnSeqDataDescriptor_t,
        keys: U,
        vDesc: cudnnSeqDataDescriptor_t,
        values: V,
        doDesc: cudnnSeqDataDescriptor_t,
        dout: W,
        weightSizeInBytes: usize,
        weights: X,
        mut dweights: Y,
        workSpaceSizeInBytes: usize,
        mut workSpace: Z,
        reserveSpaceSizeInBytes: usize,
        mut reserveSpace: A,
    ) -> Result<(), crate::sys::cudnnStatus_t> {
        let status = unsafe {
            crate::sys::cudnnMultiHeadAttnBackwardWeights(
                self.handle,
                attnDesc,
                addGrad,
                qDesc,
                queries.as_const_ptr() as *const ::std::os::raw::c_void,
                kDesc,
                keys.as_const_ptr() as *const ::std::os::raw::c_void,
                vDesc,
                values.as_const_ptr() as *const ::std::os::raw::c_void,
                doDesc,
                dout.as_const_ptr() as *const ::std::os::raw::c_void,
                weightSizeInBytes,
                weights.as_const_ptr() as *const ::std::os::raw::c_void,
                dweights.as_mut_ptr() as *mut ::std::os::raw::c_void,
                workSpaceSizeInBytes,
                workSpace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                reserveSpaceSizeInBytes,
                reserveSpace.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnCTCLoss<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        probsDesc: cudnnTensorDescriptor_t,
        probs: T,
        hostLabels: U,
        hostLabelLengths: V,
        hostInputLengths: W,
        mut costs: X,
        gradientsDesc: cudnnTensorDescriptor_t,
        mut gradients: Y,
        algo: cudnnCTCLossAlgo_t,
        ctcLossDesc: cudnnCTCLossDescriptor_t,
        mut workspace: Z,
        workSpaceSizeInBytes: usize,
    ) -> Result<(), crate::sys::cudnnStatus_t> {
        let status = unsafe {
            crate::sys::cudnnCTCLoss(
                self.handle,
                probsDesc,
                probs.as_const_ptr() as *const ::std::os::raw::c_void,
                hostLabels.as_const_ptr() as *const ::std::os::raw::c_int,
                hostLabelLengths.as_const_ptr() as *const ::std::os::raw::c_int,
                hostInputLengths.as_const_ptr() as *const ::std::os::raw::c_int,
                costs.as_mut_ptr() as *mut ::std::os::raw::c_void,
                gradientsDesc,
                gradients.as_mut_ptr() as *mut ::std::os::raw::c_void,
                algo,
                ctcLossDesc,
                workspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                workSpaceSizeInBytes,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnCTCLoss_v8<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
        Z: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        algo: cudnnCTCLossAlgo_t,
        ctcLossDesc: cudnnCTCLossDescriptor_t,
        probsDesc: cudnnTensorDescriptor_t,
        probs: T,
        labels: U,
        labelLengths: V,
        inputLengths: W,
        mut costs: X,
        gradientsDesc: cudnnTensorDescriptor_t,
        mut gradients: Y,
        workSpaceSizeInBytes: usize,
        mut workspace: Z,
    ) -> Result<(), crate::sys::cudnnStatus_t> {
        let status = unsafe {
            crate::sys::cudnnCTCLoss_v8(
                self.handle,
                algo,
                ctcLossDesc,
                probsDesc,
                probs.as_const_ptr() as *const ::std::os::raw::c_void,
                labels.as_const_ptr() as *const ::std::os::raw::c_int,
                labelLengths.as_const_ptr() as *const ::std::os::raw::c_int,
                inputLengths.as_const_ptr() as *const ::std::os::raw::c_int,
                costs.as_mut_ptr() as *mut ::std::os::raw::c_void,
                gradientsDesc,
                gradients.as_mut_ptr() as *mut ::std::os::raw::c_void,
                workSpaceSizeInBytes,
                workspace.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnGetCTCLossWorkspaceSize(
        &self,
        probsDesc: cudnnTensorDescriptor_t,
        gradientsDesc: cudnnTensorDescriptor_t,
        labels: *const ::std::os::raw::c_int,
        labelLengths: *const ::std::os::raw::c_int,
        inputLengths: *const ::std::os::raw::c_int,
        algo: cudnnCTCLossAlgo_t,
        ctcLossDesc: cudnnCTCLossDescriptor_t,
    ) -> Result<usize, crate::sys::cudnnStatus_t> {
        let mut out_8: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
        let status = unsafe {
            crate::sys::cudnnGetCTCLossWorkspaceSize(
                self.handle,
                probsDesc,
                gradientsDesc,
                labels,
                labelLengths,
                inputLengths,
                algo,
                ctcLossDesc,
                out_8.as_mut_ptr() as *mut _,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            unsafe { Ok(out_8.assume_init()) }
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnGetCTCLossWorkspaceSize_v8(
        &self,
        algo: cudnnCTCLossAlgo_t,
        ctcLossDesc: cudnnCTCLossDescriptor_t,
        probsDesc: cudnnTensorDescriptor_t,
        gradientsDesc: cudnnTensorDescriptor_t,
    ) -> Result<usize, crate::sys::cudnnStatus_t> {
        let mut out_5: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
        let status = unsafe {
            crate::sys::cudnnGetCTCLossWorkspaceSize_v8(
                self.handle,
                algo,
                ctcLossDesc,
                probsDesc,
                gradientsDesc,
                out_5.as_mut_ptr() as *mut _,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            unsafe { Ok(out_5.assume_init()) }
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnGetConvolutionForwardAlgorithmMaxCount(
        &self,
    ) -> Result<::std::os::raw::c_int, crate::sys::cudnnStatus_t> {
        let mut out_1: std::mem::MaybeUninit<::std::os::raw::c_int> =
            std::mem::MaybeUninit::uninit();
        let status = unsafe {
            crate::sys::cudnnGetConvolutionForwardAlgorithmMaxCount(
                self.handle,
                out_1.as_mut_ptr() as *mut _,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            unsafe { Ok(out_1.assume_init()) }
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnGetConvolutionForwardAlgorithm_v7(
        &self,
        srcDesc: cudnnTensorDescriptor_t,
        filterDesc: cudnnFilterDescriptor_t,
        convDesc: cudnnConvolutionDescriptor_t,
        destDesc: cudnnTensorDescriptor_t,
        requestedAlgoCount: ::std::os::raw::c_int,
    ) -> Result<(::std::os::raw::c_int, cudnnConvolutionFwdAlgoPerf_t), crate::sys::cudnnStatus_t>
    {
        let mut out_6: std::mem::MaybeUninit<::std::os::raw::c_int> =
            std::mem::MaybeUninit::uninit();
        let mut out_7: std::mem::MaybeUninit<cudnnConvolutionFwdAlgoPerf_t> =
            std::mem::MaybeUninit::uninit();
        let status = unsafe {
            crate::sys::cudnnGetConvolutionForwardAlgorithm_v7(
                self.handle,
                srcDesc,
                filterDesc,
                convDesc,
                destDesc,
                requestedAlgoCount,
                out_6.as_mut_ptr() as *mut _,
                out_7.as_mut_ptr() as *mut _,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            unsafe { Ok((out_6.assume_init(), out_7.assume_init())) }
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnFindConvolutionForwardAlgorithm<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        xDesc: cudnnTensorDescriptor_t,
        wDesc: cudnnFilterDescriptor_t,
        convDesc: cudnnConvolutionDescriptor_t,
        yDesc: cudnnTensorDescriptor_t,
        requestedAlgoCount: ::std::os::raw::c_int,
        mut returnedAlgoCount: T,
        mut perfResults: U,
    ) -> Result<(), crate::sys::cudnnStatus_t> {
        let status = unsafe {
            crate::sys::cudnnFindConvolutionForwardAlgorithm(
                self.handle,
                xDesc,
                wDesc,
                convDesc,
                yDesc,
                requestedAlgoCount,
                returnedAlgoCount.as_mut_ptr() as *mut ::std::os::raw::c_int,
                perfResults.as_mut_ptr() as *mut cudnnConvolutionFwdAlgoPerf_t,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnFindConvolutionForwardAlgorithmEx<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        xDesc: cudnnTensorDescriptor_t,
        x: T,
        wDesc: cudnnFilterDescriptor_t,
        w: U,
        convDesc: cudnnConvolutionDescriptor_t,
        yDesc: cudnnTensorDescriptor_t,
        mut y: V,
        requestedAlgoCount: ::std::os::raw::c_int,
        mut returnedAlgoCount: W,
        mut perfResults: X,
        mut workSpace: Y,
        workSpaceSizeInBytes: usize,
    ) -> Result<(), crate::sys::cudnnStatus_t> {
        let status = unsafe {
            crate::sys::cudnnFindConvolutionForwardAlgorithmEx(
                self.handle,
                xDesc,
                x.as_const_ptr() as *const ::std::os::raw::c_void,
                wDesc,
                w.as_const_ptr() as *const ::std::os::raw::c_void,
                convDesc,
                yDesc,
                y.as_mut_ptr() as *mut ::std::os::raw::c_void,
                requestedAlgoCount,
                returnedAlgoCount.as_mut_ptr() as *mut ::std::os::raw::c_int,
                perfResults.as_mut_ptr() as *mut cudnnConvolutionFwdAlgoPerf_t,
                workSpace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                workSpaceSizeInBytes,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnIm2Col<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        xDesc: cudnnTensorDescriptor_t,
        x: T,
        wDesc: cudnnFilterDescriptor_t,
        convDesc: cudnnConvolutionDescriptor_t,
        mut colBuffer: U,
    ) -> Result<(), crate::sys::cudnnStatus_t> {
        let status = unsafe {
            crate::sys::cudnnIm2Col(
                self.handle,
                xDesc,
                x.as_const_ptr() as *const ::std::os::raw::c_void,
                wDesc,
                convDesc,
                colBuffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnReorderFilterAndBias<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        filterDesc: cudnnFilterDescriptor_t,
        reorderType: cudnnReorderType_t,
        filterData: T,
        mut reorderedFilterData: U,
        reorderBias: ::std::os::raw::c_int,
        biasData: V,
        mut reorderedBiasData: W,
    ) -> Result<(), crate::sys::cudnnStatus_t> {
        let status = unsafe {
            crate::sys::cudnnReorderFilterAndBias(
                self.handle,
                filterDesc,
                reorderType,
                filterData.as_const_ptr() as *const ::std::os::raw::c_void,
                reorderedFilterData.as_mut_ptr() as *mut ::std::os::raw::c_void,
                reorderBias,
                biasData.as_const_ptr() as *const ::std::os::raw::c_void,
                reorderedBiasData.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnGetConvolutionForwardWorkspaceSize(
        &self,
        xDesc: cudnnTensorDescriptor_t,
        wDesc: cudnnFilterDescriptor_t,
        convDesc: cudnnConvolutionDescriptor_t,
        yDesc: cudnnTensorDescriptor_t,
        algo: cudnnConvolutionFwdAlgo_t,
    ) -> Result<usize, crate::sys::cudnnStatus_t> {
        let mut out_6: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
        let status = unsafe {
            crate::sys::cudnnGetConvolutionForwardWorkspaceSize(
                self.handle,
                xDesc,
                wDesc,
                convDesc,
                yDesc,
                algo,
                out_6.as_mut_ptr() as *mut _,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            unsafe { Ok(out_6.assume_init()) }
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnConvolutionForward<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        alpha: T,
        xDesc: cudnnTensorDescriptor_t,
        x: U,
        wDesc: cudnnFilterDescriptor_t,
        w: V,
        convDesc: cudnnConvolutionDescriptor_t,
        algo: cudnnConvolutionFwdAlgo_t,
        mut workSpace: W,
        workSpaceSizeInBytes: usize,
        beta: X,
        yDesc: cudnnTensorDescriptor_t,
        mut y: Y,
    ) -> Result<(), crate::sys::cudnnStatus_t> {
        let status = unsafe {
            crate::sys::cudnnConvolutionForward(
                self.handle,
                alpha.as_const_ptr() as *const ::std::os::raw::c_void,
                xDesc,
                x.as_const_ptr() as *const ::std::os::raw::c_void,
                wDesc,
                w.as_const_ptr() as *const ::std::os::raw::c_void,
                convDesc,
                algo,
                workSpace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                workSpaceSizeInBytes,
                beta.as_const_ptr() as *const ::std::os::raw::c_void,
                yDesc,
                y.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnConvolutionBiasActivationForward<
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
        alpha1: T,
        xDesc: cudnnTensorDescriptor_t,
        x: U,
        wDesc: cudnnFilterDescriptor_t,
        w: V,
        convDesc: cudnnConvolutionDescriptor_t,
        algo: cudnnConvolutionFwdAlgo_t,
        mut workSpace: W,
        workSpaceSizeInBytes: usize,
        alpha2: X,
        zDesc: cudnnTensorDescriptor_t,
        z: Y,
        biasDesc: cudnnTensorDescriptor_t,
        bias: Z,
        activationDesc: cudnnActivationDescriptor_t,
        yDesc: cudnnTensorDescriptor_t,
        mut y: A,
    ) -> Result<(), crate::sys::cudnnStatus_t> {
        let status = unsafe {
            crate::sys::cudnnConvolutionBiasActivationForward(
                self.handle,
                alpha1.as_const_ptr() as *const ::std::os::raw::c_void,
                xDesc,
                x.as_const_ptr() as *const ::std::os::raw::c_void,
                wDesc,
                w.as_const_ptr() as *const ::std::os::raw::c_void,
                convDesc,
                algo,
                workSpace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                workSpaceSizeInBytes,
                alpha2.as_const_ptr() as *const ::std::os::raw::c_void,
                zDesc,
                z.as_const_ptr() as *const ::std::os::raw::c_void,
                biasDesc,
                bias.as_const_ptr() as *const ::std::os::raw::c_void,
                activationDesc,
                yDesc,
                y.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnGetConvolutionBackwardDataAlgorithmMaxCount(
        &self,
    ) -> Result<::std::os::raw::c_int, crate::sys::cudnnStatus_t> {
        let mut out_1: std::mem::MaybeUninit<::std::os::raw::c_int> =
            std::mem::MaybeUninit::uninit();
        let status = unsafe {
            crate::sys::cudnnGetConvolutionBackwardDataAlgorithmMaxCount(
                self.handle,
                out_1.as_mut_ptr() as *mut _,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            unsafe { Ok(out_1.assume_init()) }
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnFindConvolutionBackwardDataAlgorithm<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        wDesc: cudnnFilterDescriptor_t,
        dyDesc: cudnnTensorDescriptor_t,
        convDesc: cudnnConvolutionDescriptor_t,
        dxDesc: cudnnTensorDescriptor_t,
        requestedAlgoCount: ::std::os::raw::c_int,
        mut returnedAlgoCount: T,
        mut perfResults: U,
    ) -> Result<(), crate::sys::cudnnStatus_t> {
        let status = unsafe {
            crate::sys::cudnnFindConvolutionBackwardDataAlgorithm(
                self.handle,
                wDesc,
                dyDesc,
                convDesc,
                dxDesc,
                requestedAlgoCount,
                returnedAlgoCount.as_mut_ptr() as *mut ::std::os::raw::c_int,
                perfResults.as_mut_ptr() as *mut cudnnConvolutionBwdDataAlgoPerf_t,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnFindConvolutionBackwardDataAlgorithmEx<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        wDesc: cudnnFilterDescriptor_t,
        w: T,
        dyDesc: cudnnTensorDescriptor_t,
        dy: U,
        convDesc: cudnnConvolutionDescriptor_t,
        dxDesc: cudnnTensorDescriptor_t,
        mut dx: V,
        requestedAlgoCount: ::std::os::raw::c_int,
        mut returnedAlgoCount: W,
        mut perfResults: X,
        mut workSpace: Y,
        workSpaceSizeInBytes: usize,
    ) -> Result<(), crate::sys::cudnnStatus_t> {
        let status = unsafe {
            crate::sys::cudnnFindConvolutionBackwardDataAlgorithmEx(
                self.handle,
                wDesc,
                w.as_const_ptr() as *const ::std::os::raw::c_void,
                dyDesc,
                dy.as_const_ptr() as *const ::std::os::raw::c_void,
                convDesc,
                dxDesc,
                dx.as_mut_ptr() as *mut ::std::os::raw::c_void,
                requestedAlgoCount,
                returnedAlgoCount.as_mut_ptr() as *mut ::std::os::raw::c_int,
                perfResults.as_mut_ptr() as *mut cudnnConvolutionBwdDataAlgoPerf_t,
                workSpace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                workSpaceSizeInBytes,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnGetConvolutionBackwardDataAlgorithm_v7(
        &self,
        filterDesc: cudnnFilterDescriptor_t,
        diffDesc: cudnnTensorDescriptor_t,
        convDesc: cudnnConvolutionDescriptor_t,
        gradDesc: cudnnTensorDescriptor_t,
        requestedAlgoCount: ::std::os::raw::c_int,
    ) -> Result<(::std::os::raw::c_int, cudnnConvolutionBwdDataAlgoPerf_t), crate::sys::cudnnStatus_t>
    {
        let mut out_6: std::mem::MaybeUninit<::std::os::raw::c_int> =
            std::mem::MaybeUninit::uninit();
        let mut out_7: std::mem::MaybeUninit<cudnnConvolutionBwdDataAlgoPerf_t> =
            std::mem::MaybeUninit::uninit();
        let status = unsafe {
            crate::sys::cudnnGetConvolutionBackwardDataAlgorithm_v7(
                self.handle,
                filterDesc,
                diffDesc,
                convDesc,
                gradDesc,
                requestedAlgoCount,
                out_6.as_mut_ptr() as *mut _,
                out_7.as_mut_ptr() as *mut _,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            unsafe { Ok((out_6.assume_init(), out_7.assume_init())) }
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnGetConvolutionBackwardDataWorkspaceSize(
        &self,
        wDesc: cudnnFilterDescriptor_t,
        dyDesc: cudnnTensorDescriptor_t,
        convDesc: cudnnConvolutionDescriptor_t,
        dxDesc: cudnnTensorDescriptor_t,
        algo: cudnnConvolutionBwdDataAlgo_t,
    ) -> Result<usize, crate::sys::cudnnStatus_t> {
        let mut out_6: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
        let status = unsafe {
            crate::sys::cudnnGetConvolutionBackwardDataWorkspaceSize(
                self.handle,
                wDesc,
                dyDesc,
                convDesc,
                dxDesc,
                algo,
                out_6.as_mut_ptr() as *mut _,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            unsafe { Ok(out_6.assume_init()) }
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnConvolutionBackwardData<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        alpha: T,
        wDesc: cudnnFilterDescriptor_t,
        w: U,
        dyDesc: cudnnTensorDescriptor_t,
        dy: V,
        convDesc: cudnnConvolutionDescriptor_t,
        algo: cudnnConvolutionBwdDataAlgo_t,
        mut workSpace: W,
        workSpaceSizeInBytes: usize,
        beta: X,
        dxDesc: cudnnTensorDescriptor_t,
        mut dx: Y,
    ) -> Result<(), crate::sys::cudnnStatus_t> {
        let status = unsafe {
            crate::sys::cudnnConvolutionBackwardData(
                self.handle,
                alpha.as_const_ptr() as *const ::std::os::raw::c_void,
                wDesc,
                w.as_const_ptr() as *const ::std::os::raw::c_void,
                dyDesc,
                dy.as_const_ptr() as *const ::std::os::raw::c_void,
                convDesc,
                algo,
                workSpace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                workSpaceSizeInBytes,
                beta.as_const_ptr() as *const ::std::os::raw::c_void,
                dxDesc,
                dx.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnGetFoldedConvBackwardDataDescriptors(
        &self,
        filterDesc: cudnnFilterDescriptor_t,
        diffDesc: cudnnTensorDescriptor_t,
        convDesc: cudnnConvolutionDescriptor_t,
        gradDesc: cudnnTensorDescriptor_t,
        transformFormat: cudnnTensorFormat_t,
        foldedFilterDesc: cudnnFilterDescriptor_t,
        paddedDiffDesc: cudnnTensorDescriptor_t,
        foldedConvDesc: cudnnConvolutionDescriptor_t,
        foldedGradDesc: cudnnTensorDescriptor_t,
        filterFoldTransDesc: cudnnTensorTransformDescriptor_t,
        diffPadTransDesc: cudnnTensorTransformDescriptor_t,
        gradFoldTransDesc: cudnnTensorTransformDescriptor_t,
        gradUnfoldTransDesc: cudnnTensorTransformDescriptor_t,
    ) -> Result<(), crate::sys::cudnnStatus_t> {
        let status = unsafe {
            crate::sys::cudnnGetFoldedConvBackwardDataDescriptors(
                self.handle,
                filterDesc,
                diffDesc,
                convDesc,
                gradDesc,
                transformFormat,
                foldedFilterDesc,
                paddedDiffDesc,
                foldedConvDesc,
                foldedGradDesc,
                filterFoldTransDesc,
                diffPadTransDesc,
                gradFoldTransDesc,
                gradUnfoldTransDesc,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnGetConvolutionBackwardFilterAlgorithmMaxCount(
        &self,
    ) -> Result<::std::os::raw::c_int, crate::sys::cudnnStatus_t> {
        let mut out_1: std::mem::MaybeUninit<::std::os::raw::c_int> =
            std::mem::MaybeUninit::uninit();
        let status = unsafe {
            crate::sys::cudnnGetConvolutionBackwardFilterAlgorithmMaxCount(
                self.handle,
                out_1.as_mut_ptr() as *mut _,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            unsafe { Ok(out_1.assume_init()) }
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnFindConvolutionBackwardFilterAlgorithm<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        xDesc: cudnnTensorDescriptor_t,
        dyDesc: cudnnTensorDescriptor_t,
        convDesc: cudnnConvolutionDescriptor_t,
        dwDesc: cudnnFilterDescriptor_t,
        requestedAlgoCount: ::std::os::raw::c_int,
        mut returnedAlgoCount: T,
        mut perfResults: U,
    ) -> Result<(), crate::sys::cudnnStatus_t> {
        let status = unsafe {
            crate::sys::cudnnFindConvolutionBackwardFilterAlgorithm(
                self.handle,
                xDesc,
                dyDesc,
                convDesc,
                dwDesc,
                requestedAlgoCount,
                returnedAlgoCount.as_mut_ptr() as *mut ::std::os::raw::c_int,
                perfResults.as_mut_ptr() as *mut cudnnConvolutionBwdFilterAlgoPerf_t,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnFindConvolutionBackwardFilterAlgorithmEx<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        xDesc: cudnnTensorDescriptor_t,
        x: T,
        dyDesc: cudnnTensorDescriptor_t,
        y: U,
        convDesc: cudnnConvolutionDescriptor_t,
        dwDesc: cudnnFilterDescriptor_t,
        mut dw: V,
        requestedAlgoCount: ::std::os::raw::c_int,
        mut returnedAlgoCount: W,
        mut perfResults: X,
        mut workSpace: Y,
        workSpaceSizeInBytes: usize,
    ) -> Result<(), crate::sys::cudnnStatus_t> {
        let status = unsafe {
            crate::sys::cudnnFindConvolutionBackwardFilterAlgorithmEx(
                self.handle,
                xDesc,
                x.as_const_ptr() as *const ::std::os::raw::c_void,
                dyDesc,
                y.as_const_ptr() as *const ::std::os::raw::c_void,
                convDesc,
                dwDesc,
                dw.as_mut_ptr() as *mut ::std::os::raw::c_void,
                requestedAlgoCount,
                returnedAlgoCount.as_mut_ptr() as *mut ::std::os::raw::c_int,
                perfResults.as_mut_ptr() as *mut cudnnConvolutionBwdFilterAlgoPerf_t,
                workSpace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                workSpaceSizeInBytes,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnGetConvolutionBackwardFilterAlgorithm_v7(
        &self,
        srcDesc: cudnnTensorDescriptor_t,
        diffDesc: cudnnTensorDescriptor_t,
        convDesc: cudnnConvolutionDescriptor_t,
        gradDesc: cudnnFilterDescriptor_t,
        requestedAlgoCount: ::std::os::raw::c_int,
    ) -> Result<
        (::std::os::raw::c_int, cudnnConvolutionBwdFilterAlgoPerf_t),
        crate::sys::cudnnStatus_t,
    > {
        let mut out_6: std::mem::MaybeUninit<::std::os::raw::c_int> =
            std::mem::MaybeUninit::uninit();
        let mut out_7: std::mem::MaybeUninit<cudnnConvolutionBwdFilterAlgoPerf_t> =
            std::mem::MaybeUninit::uninit();
        let status = unsafe {
            crate::sys::cudnnGetConvolutionBackwardFilterAlgorithm_v7(
                self.handle,
                srcDesc,
                diffDesc,
                convDesc,
                gradDesc,
                requestedAlgoCount,
                out_6.as_mut_ptr() as *mut _,
                out_7.as_mut_ptr() as *mut _,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            unsafe { Ok((out_6.assume_init(), out_7.assume_init())) }
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnGetConvolutionBackwardFilterWorkspaceSize(
        &self,
        xDesc: cudnnTensorDescriptor_t,
        dyDesc: cudnnTensorDescriptor_t,
        convDesc: cudnnConvolutionDescriptor_t,
        gradDesc: cudnnFilterDescriptor_t,
        algo: cudnnConvolutionBwdFilterAlgo_t,
    ) -> Result<usize, crate::sys::cudnnStatus_t> {
        let mut out_6: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
        let status = unsafe {
            crate::sys::cudnnGetConvolutionBackwardFilterWorkspaceSize(
                self.handle,
                xDesc,
                dyDesc,
                convDesc,
                gradDesc,
                algo,
                out_6.as_mut_ptr() as *mut _,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            unsafe { Ok(out_6.assume_init()) }
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnConvolutionBackwardFilter<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
        X: ::cuda_libs::types::CudaAsPtr,
        Y: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        alpha: T,
        xDesc: cudnnTensorDescriptor_t,
        x: U,
        dyDesc: cudnnTensorDescriptor_t,
        dy: V,
        convDesc: cudnnConvolutionDescriptor_t,
        algo: cudnnConvolutionBwdFilterAlgo_t,
        mut workSpace: W,
        workSpaceSizeInBytes: usize,
        beta: X,
        dwDesc: cudnnFilterDescriptor_t,
        mut dw: Y,
    ) -> Result<(), crate::sys::cudnnStatus_t> {
        let status = unsafe {
            crate::sys::cudnnConvolutionBackwardFilter(
                self.handle,
                alpha.as_const_ptr() as *const ::std::os::raw::c_void,
                xDesc,
                x.as_const_ptr() as *const ::std::os::raw::c_void,
                dyDesc,
                dy.as_const_ptr() as *const ::std::os::raw::c_void,
                convDesc,
                algo,
                workSpace.as_mut_ptr() as *mut ::std::os::raw::c_void,
                workSpaceSizeInBytes,
                beta.as_const_ptr() as *const ::std::os::raw::c_void,
                dwDesc,
                dw.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnConvolutionBackwardBias<
        T: ::cuda_libs::types::CudaAsPtr,
        U: ::cuda_libs::types::CudaAsPtr,
        V: ::cuda_libs::types::CudaAsPtr,
        W: ::cuda_libs::types::CudaAsPtr,
    >(
        &self,
        alpha: T,
        dyDesc: cudnnTensorDescriptor_t,
        dy: U,
        beta: V,
        dbDesc: cudnnTensorDescriptor_t,
        mut db: W,
    ) -> Result<(), crate::sys::cudnnStatus_t> {
        let status = unsafe {
            crate::sys::cudnnConvolutionBackwardBias(
                self.handle,
                alpha.as_const_ptr() as *const ::std::os::raw::c_void,
                dyDesc,
                dy.as_const_ptr() as *const ::std::os::raw::c_void,
                beta.as_const_ptr() as *const ::std::os::raw::c_void,
                dbDesc,
                db.as_mut_ptr() as *mut ::std::os::raw::c_void,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnMakeFusedOpsPlan<T: ::cuda_libs::types::CudaAsPtr>(
        &self,
        plan: cudnnFusedOpsPlan_t,
        constPack: cudnnFusedOpsConstParamPack_t,
        mut workspaceSizeInBytes: T,
    ) -> Result<(), crate::sys::cudnnStatus_t> {
        let status = unsafe {
            crate::sys::cudnnMakeFusedOpsPlan(
                self.handle,
                plan,
                constPack,
                workspaceSizeInBytes.as_mut_ptr() as *mut usize,
            )
        };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
    pub unsafe fn cudnnFusedOpsExecute(
        &self,
        plan: cudnnFusedOpsPlan_t,
        varPack: cudnnFusedOpsVariantParamPack_t,
    ) -> Result<(), crate::sys::cudnnStatus_t> {
        let status = unsafe { crate::sys::cudnnFusedOpsExecute(self.handle, plan, varPack) };
        if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
}
pub unsafe fn cudnnGetVersion() -> usize {
    unsafe { crate::sys::cudnnGetVersion() }
}
pub unsafe fn cudnnGetMaxDeviceVersion() -> usize {
    unsafe { crate::sys::cudnnGetMaxDeviceVersion() }
}
pub unsafe fn cudnnGetCudartVersion() -> usize {
    unsafe { crate::sys::cudnnGetCudartVersion() }
}
pub unsafe fn cudnnGetErrorString(status: cudnnStatus_t) -> *const ::std::os::raw::c_char {
    unsafe { crate::sys::cudnnGetErrorString(status) }
}
pub unsafe fn cudnnGetLastErrorString<T: ::cuda_libs::types::CudaAsPtr>(
    mut message: T,
    max_size: usize,
) {
    unsafe {
        crate::sys::cudnnGetLastErrorString(
            message.as_mut_ptr() as *mut ::std::os::raw::c_char,
            max_size,
        )
    }
}
pub unsafe fn cudnnGetProperty(
    type_: libraryPropertyType,
) -> Result<::std::os::raw::c_int, crate::sys::cudnnStatus_t> {
    let mut out_1: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudnnGetProperty(type_, out_1.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        unsafe { Ok(out_1.assume_init()) }
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnSetCallback<T: ::cuda_libs::types::CudaAsPtr>(
    mask: ::std::os::raw::c_uint,
    mut udata: T,
    fptr: cudnnCallback_t,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnSetCallback(
            mask,
            udata.as_mut_ptr() as *mut ::std::os::raw::c_void,
            fptr,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnGetCallback(
    udata: *mut *mut ::std::os::raw::c_void,
) -> Result<(::std::os::raw::c_uint, cudnnCallback_t), crate::sys::cudnnStatus_t> {
    let mut out_0: std::mem::MaybeUninit<::std::os::raw::c_uint> = std::mem::MaybeUninit::uninit();
    let mut out_2: std::mem::MaybeUninit<cudnnCallback_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudnnGetCallback(
            out_0.as_mut_ptr() as *mut _,
            udata,
            out_2.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        unsafe { Ok((out_0.assume_init(), out_2.assume_init())) }
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnGraphVersionCheck() -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnGraphVersionCheck() };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnBackendInitialize(
    descriptor: cudnnBackendDescriptor_t,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnBackendInitialize(descriptor) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnBackendFinalize(
    descriptor: cudnnBackendDescriptor_t,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnBackendFinalize(descriptor) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnBackendSetAttribute<T: ::cuda_libs::types::CudaAsPtr>(
    descriptor: cudnnBackendDescriptor_t,
    attributeName: cudnnBackendAttributeName_t,
    attributeType: cudnnBackendAttributeType_t,
    elementCount: i64,
    arrayOfElements: T,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnBackendSetAttribute(
            descriptor,
            attributeName,
            attributeType,
            elementCount,
            arrayOfElements.as_const_ptr() as *const ::std::os::raw::c_void,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnBackendGetAttribute(
    descriptor: cudnnBackendDescriptor_t,
    attributeName: cudnnBackendAttributeName_t,
    attributeType: cudnnBackendAttributeType_t,
    requestedElementCount: i64,
    arrayOfElements: *mut ::std::os::raw::c_void,
) -> Result<i64, crate::sys::cudnnStatus_t> {
    let mut out_4: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudnnBackendGetAttribute(
            descriptor,
            attributeName,
            attributeType,
            requestedElementCount,
            out_4.as_mut_ptr() as *mut _,
            arrayOfElements,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        unsafe { Ok(out_4.assume_init()) }
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnSetTensor4dDescriptor(
    tensorDesc: cudnnTensorDescriptor_t,
    format: cudnnTensorFormat_t,
    dataType: cudnnDataType_t,
    n: ::std::os::raw::c_int,
    c: ::std::os::raw::c_int,
    h: ::std::os::raw::c_int,
    w: ::std::os::raw::c_int,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status =
        unsafe { crate::sys::cudnnSetTensor4dDescriptor(tensorDesc, format, dataType, n, c, h, w) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnSetTensor4dDescriptorEx(
    tensorDesc: cudnnTensorDescriptor_t,
    dataType: cudnnDataType_t,
    n: ::std::os::raw::c_int,
    c: ::std::os::raw::c_int,
    h: ::std::os::raw::c_int,
    w: ::std::os::raw::c_int,
    nStride: ::std::os::raw::c_int,
    cStride: ::std::os::raw::c_int,
    hStride: ::std::os::raw::c_int,
    wStride: ::std::os::raw::c_int,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnSetTensor4dDescriptorEx(
            tensorDesc, dataType, n, c, h, w, nStride, cStride, hStride, wStride,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnGetTensor4dDescriptor(
    tensorDesc: cudnnTensorDescriptor_t,
) -> Result<
    (
        cudnnDataType_t,
        ::std::os::raw::c_int,
        ::std::os::raw::c_int,
        ::std::os::raw::c_int,
        ::std::os::raw::c_int,
        ::std::os::raw::c_int,
        ::std::os::raw::c_int,
        ::std::os::raw::c_int,
        ::std::os::raw::c_int,
    ),
    crate::sys::cudnnStatus_t,
> {
    let mut out_1: std::mem::MaybeUninit<cudnnDataType_t> = std::mem::MaybeUninit::uninit();
    let mut out_2: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_3: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_4: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_5: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_6: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_7: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_8: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_9: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudnnGetTensor4dDescriptor(
            tensorDesc,
            out_1.as_mut_ptr() as *mut _,
            out_2.as_mut_ptr() as *mut _,
            out_3.as_mut_ptr() as *mut _,
            out_4.as_mut_ptr() as *mut _,
            out_5.as_mut_ptr() as *mut _,
            out_6.as_mut_ptr() as *mut _,
            out_7.as_mut_ptr() as *mut _,
            out_8.as_mut_ptr() as *mut _,
            out_9.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        unsafe {
            Ok((
                out_1.assume_init(),
                out_2.assume_init(),
                out_3.assume_init(),
                out_4.assume_init(),
                out_5.assume_init(),
                out_6.assume_init(),
                out_7.assume_init(),
                out_8.assume_init(),
                out_9.assume_init(),
            ))
        }
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnSetTensorNdDescriptor<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
>(
    tensorDesc: cudnnTensorDescriptor_t,
    dataType: cudnnDataType_t,
    nbDims: ::std::os::raw::c_int,
    dimA: T,
    strideA: U,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnSetTensorNdDescriptor(
            tensorDesc,
            dataType,
            nbDims,
            dimA.as_const_ptr() as *const ::std::os::raw::c_int,
            strideA.as_const_ptr() as *const ::std::os::raw::c_int,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnSetTensorNdDescriptorEx<T: ::cuda_libs::types::CudaAsPtr>(
    tensorDesc: cudnnTensorDescriptor_t,
    format: cudnnTensorFormat_t,
    dataType: cudnnDataType_t,
    nbDims: ::std::os::raw::c_int,
    dimA: T,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnSetTensorNdDescriptorEx(
            tensorDesc,
            format,
            dataType,
            nbDims,
            dimA.as_const_ptr() as *const ::std::os::raw::c_int,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnGetTensorNdDescriptor(
    tensorDesc: cudnnTensorDescriptor_t,
    nbDimsRequested: ::std::os::raw::c_int,
) -> Result<
    (
        cudnnDataType_t,
        ::std::os::raw::c_int,
        ::std::os::raw::c_int,
        ::std::os::raw::c_int,
    ),
    crate::sys::cudnnStatus_t,
> {
    let mut out_2: std::mem::MaybeUninit<cudnnDataType_t> = std::mem::MaybeUninit::uninit();
    let mut out_3: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_4: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_5: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudnnGetTensorNdDescriptor(
            tensorDesc,
            nbDimsRequested,
            out_2.as_mut_ptr() as *mut _,
            out_3.as_mut_ptr() as *mut _,
            out_4.as_mut_ptr() as *mut _,
            out_5.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        unsafe {
            Ok((
                out_2.assume_init(),
                out_3.assume_init(),
                out_4.assume_init(),
                out_5.assume_init(),
            ))
        }
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnGetTensorSizeInBytes(
    tensorDesc: cudnnTensorDescriptor_t,
) -> Result<usize, crate::sys::cudnnStatus_t> {
    let mut out_1: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let status =
        unsafe { crate::sys::cudnnGetTensorSizeInBytes(tensorDesc, out_1.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        unsafe { Ok(out_1.assume_init()) }
    } else {
        Err(status)
    }
}
#[doc = " Create a destination descriptor for cudnnTransformTensor"]
pub unsafe fn cudnnInitTransformDest<T: ::cuda_libs::types::CudaAsPtr>(
    transformDesc: cudnnTensorTransformDescriptor_t,
    srcDesc: cudnnTensorDescriptor_t,
    destDesc: cudnnTensorDescriptor_t,
    mut destSizeInBytes: T,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnInitTransformDest(
            transformDesc,
            srcDesc,
            destDesc,
            destSizeInBytes.as_mut_ptr() as *mut usize,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " Initialize a previously created tensor transform descriptor."]
pub unsafe fn cudnnSetTensorTransformDescriptor<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
>(
    transformDesc: cudnnTensorTransformDescriptor_t,
    nbDims: u32,
    destFormat: cudnnTensorFormat_t,
    padBeforeA: T,
    padAfterA: U,
    foldA: V,
    direction: cudnnFoldingDirection_t,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnSetTensorTransformDescriptor(
            transformDesc,
            nbDims,
            destFormat,
            padBeforeA.as_const_ptr() as *const i32,
            padAfterA.as_const_ptr() as *const i32,
            foldA.as_const_ptr() as *const u32,
            direction,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " Retrieves the values stored in a previously initialized tensor transform\n descriptor."]
pub unsafe fn cudnnGetTensorTransformDescriptor(
    transformDesc: cudnnTensorTransformDescriptor_t,
    nbDimsRequested: u32,
) -> Result<(cudnnTensorFormat_t, i32, i32, u32, cudnnFoldingDirection_t), crate::sys::cudnnStatus_t>
{
    let mut out_2: std::mem::MaybeUninit<cudnnTensorFormat_t> = std::mem::MaybeUninit::uninit();
    let mut out_3: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let mut out_4: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let mut out_5: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let mut out_6: std::mem::MaybeUninit<cudnnFoldingDirection_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudnnGetTensorTransformDescriptor(
            transformDesc,
            nbDimsRequested,
            out_2.as_mut_ptr() as *mut _,
            out_3.as_mut_ptr() as *mut _,
            out_4.as_mut_ptr() as *mut _,
            out_5.as_mut_ptr() as *mut _,
            out_6.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        unsafe {
            Ok((
                out_2.assume_init(),
                out_3.assume_init(),
                out_4.assume_init(),
                out_5.assume_init(),
                out_6.assume_init(),
            ))
        }
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnSetOpTensorDescriptor(
    opTensorDesc: cudnnOpTensorDescriptor_t,
    opTensorOp: cudnnOpTensorOp_t,
    opTensorCompType: cudnnDataType_t,
    opTensorNanOpt: cudnnNanPropagation_t,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnSetOpTensorDescriptor(
            opTensorDesc,
            opTensorOp,
            opTensorCompType,
            opTensorNanOpt,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnGetOpTensorDescriptor(
    opTensorDesc: cudnnOpTensorDescriptor_t,
) -> Result<(cudnnOpTensorOp_t, cudnnDataType_t, cudnnNanPropagation_t), crate::sys::cudnnStatus_t>
{
    let mut out_1: std::mem::MaybeUninit<cudnnOpTensorOp_t> = std::mem::MaybeUninit::uninit();
    let mut out_2: std::mem::MaybeUninit<cudnnDataType_t> = std::mem::MaybeUninit::uninit();
    let mut out_3: std::mem::MaybeUninit<cudnnNanPropagation_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudnnGetOpTensorDescriptor(
            opTensorDesc,
            out_1.as_mut_ptr() as *mut _,
            out_2.as_mut_ptr() as *mut _,
            out_3.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
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
pub unsafe fn cudnnSetReduceTensorDescriptor(
    reduceTensorDesc: cudnnReduceTensorDescriptor_t,
    reduceTensorOp: cudnnReduceTensorOp_t,
    reduceTensorCompType: cudnnDataType_t,
    reduceTensorNanOpt: cudnnNanPropagation_t,
    reduceTensorIndices: cudnnReduceTensorIndices_t,
    reduceTensorIndicesType: cudnnIndicesType_t,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnSetReduceTensorDescriptor(
            reduceTensorDesc,
            reduceTensorOp,
            reduceTensorCompType,
            reduceTensorNanOpt,
            reduceTensorIndices,
            reduceTensorIndicesType,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnGetReduceTensorDescriptor(
    reduceTensorDesc: cudnnReduceTensorDescriptor_t,
) -> Result<
    (
        cudnnReduceTensorOp_t,
        cudnnDataType_t,
        cudnnNanPropagation_t,
        cudnnReduceTensorIndices_t,
        cudnnIndicesType_t,
    ),
    crate::sys::cudnnStatus_t,
> {
    let mut out_1: std::mem::MaybeUninit<cudnnReduceTensorOp_t> = std::mem::MaybeUninit::uninit();
    let mut out_2: std::mem::MaybeUninit<cudnnDataType_t> = std::mem::MaybeUninit::uninit();
    let mut out_3: std::mem::MaybeUninit<cudnnNanPropagation_t> = std::mem::MaybeUninit::uninit();
    let mut out_4: std::mem::MaybeUninit<cudnnReduceTensorIndices_t> =
        std::mem::MaybeUninit::uninit();
    let mut out_5: std::mem::MaybeUninit<cudnnIndicesType_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudnnGetReduceTensorDescriptor(
            reduceTensorDesc,
            out_1.as_mut_ptr() as *mut _,
            out_2.as_mut_ptr() as *mut _,
            out_3.as_mut_ptr() as *mut _,
            out_4.as_mut_ptr() as *mut _,
            out_5.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        unsafe {
            Ok((
                out_1.assume_init(),
                out_2.assume_init(),
                out_3.assume_init(),
                out_4.assume_init(),
                out_5.assume_init(),
            ))
        }
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnSetFilter4dDescriptor(
    filterDesc: cudnnFilterDescriptor_t,
    dataType: cudnnDataType_t,
    format: cudnnTensorFormat_t,
    k: ::std::os::raw::c_int,
    c: ::std::os::raw::c_int,
    h: ::std::os::raw::c_int,
    w: ::std::os::raw::c_int,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status =
        unsafe { crate::sys::cudnnSetFilter4dDescriptor(filterDesc, dataType, format, k, c, h, w) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnGetFilter4dDescriptor(
    filterDesc: cudnnFilterDescriptor_t,
) -> Result<
    (
        cudnnDataType_t,
        cudnnTensorFormat_t,
        ::std::os::raw::c_int,
        ::std::os::raw::c_int,
        ::std::os::raw::c_int,
        ::std::os::raw::c_int,
    ),
    crate::sys::cudnnStatus_t,
> {
    let mut out_1: std::mem::MaybeUninit<cudnnDataType_t> = std::mem::MaybeUninit::uninit();
    let mut out_2: std::mem::MaybeUninit<cudnnTensorFormat_t> = std::mem::MaybeUninit::uninit();
    let mut out_3: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_4: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_5: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_6: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudnnGetFilter4dDescriptor(
            filterDesc,
            out_1.as_mut_ptr() as *mut _,
            out_2.as_mut_ptr() as *mut _,
            out_3.as_mut_ptr() as *mut _,
            out_4.as_mut_ptr() as *mut _,
            out_5.as_mut_ptr() as *mut _,
            out_6.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        unsafe {
            Ok((
                out_1.assume_init(),
                out_2.assume_init(),
                out_3.assume_init(),
                out_4.assume_init(),
                out_5.assume_init(),
                out_6.assume_init(),
            ))
        }
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnSetFilterNdDescriptor<T: ::cuda_libs::types::CudaAsPtr>(
    filterDesc: cudnnFilterDescriptor_t,
    dataType: cudnnDataType_t,
    format: cudnnTensorFormat_t,
    nbDims: ::std::os::raw::c_int,
    filterDimA: T,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnSetFilterNdDescriptor(
            filterDesc,
            dataType,
            format,
            nbDims,
            filterDimA.as_const_ptr() as *const ::std::os::raw::c_int,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnGetFilterNdDescriptor(
    filterDesc: cudnnFilterDescriptor_t,
    nbDimsRequested: ::std::os::raw::c_int,
) -> Result<
    (
        cudnnDataType_t,
        cudnnTensorFormat_t,
        ::std::os::raw::c_int,
        ::std::os::raw::c_int,
    ),
    crate::sys::cudnnStatus_t,
> {
    let mut out_2: std::mem::MaybeUninit<cudnnDataType_t> = std::mem::MaybeUninit::uninit();
    let mut out_3: std::mem::MaybeUninit<cudnnTensorFormat_t> = std::mem::MaybeUninit::uninit();
    let mut out_4: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_5: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudnnGetFilterNdDescriptor(
            filterDesc,
            nbDimsRequested,
            out_2.as_mut_ptr() as *mut _,
            out_3.as_mut_ptr() as *mut _,
            out_4.as_mut_ptr() as *mut _,
            out_5.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        unsafe {
            Ok((
                out_2.assume_init(),
                out_3.assume_init(),
                out_4.assume_init(),
                out_5.assume_init(),
            ))
        }
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnGetFilterSizeInBytes(
    filterDesc: cudnnFilterDescriptor_t,
) -> Result<usize, crate::sys::cudnnStatus_t> {
    let mut out_1: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let status =
        unsafe { crate::sys::cudnnGetFilterSizeInBytes(filterDesc, out_1.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        unsafe { Ok(out_1.assume_init()) }
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnSetPooling2dDescriptor(
    poolingDesc: cudnnPoolingDescriptor_t,
    mode: cudnnPoolingMode_t,
    maxpoolingNanOpt: cudnnNanPropagation_t,
    windowHeight: ::std::os::raw::c_int,
    windowWidth: ::std::os::raw::c_int,
    verticalPadding: ::std::os::raw::c_int,
    horizontalPadding: ::std::os::raw::c_int,
    verticalStride: ::std::os::raw::c_int,
    horizontalStride: ::std::os::raw::c_int,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnSetPooling2dDescriptor(
            poolingDesc,
            mode,
            maxpoolingNanOpt,
            windowHeight,
            windowWidth,
            verticalPadding,
            horizontalPadding,
            verticalStride,
            horizontalStride,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnGetPooling2dDescriptor(
    poolingDesc: cudnnPoolingDescriptor_t,
) -> Result<
    (
        cudnnPoolingMode_t,
        cudnnNanPropagation_t,
        ::std::os::raw::c_int,
        ::std::os::raw::c_int,
        ::std::os::raw::c_int,
        ::std::os::raw::c_int,
        ::std::os::raw::c_int,
        ::std::os::raw::c_int,
    ),
    crate::sys::cudnnStatus_t,
> {
    let mut out_1: std::mem::MaybeUninit<cudnnPoolingMode_t> = std::mem::MaybeUninit::uninit();
    let mut out_2: std::mem::MaybeUninit<cudnnNanPropagation_t> = std::mem::MaybeUninit::uninit();
    let mut out_3: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_4: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_5: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_6: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_7: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_8: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudnnGetPooling2dDescriptor(
            poolingDesc,
            out_1.as_mut_ptr() as *mut _,
            out_2.as_mut_ptr() as *mut _,
            out_3.as_mut_ptr() as *mut _,
            out_4.as_mut_ptr() as *mut _,
            out_5.as_mut_ptr() as *mut _,
            out_6.as_mut_ptr() as *mut _,
            out_7.as_mut_ptr() as *mut _,
            out_8.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        unsafe {
            Ok((
                out_1.assume_init(),
                out_2.assume_init(),
                out_3.assume_init(),
                out_4.assume_init(),
                out_5.assume_init(),
                out_6.assume_init(),
                out_7.assume_init(),
                out_8.assume_init(),
            ))
        }
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnSetPoolingNdDescriptor<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
>(
    poolingDesc: cudnnPoolingDescriptor_t,
    mode: cudnnPoolingMode_t,
    maxpoolingNanOpt: cudnnNanPropagation_t,
    nbDims: ::std::os::raw::c_int,
    windowDimA: T,
    paddingA: U,
    strideA: V,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnSetPoolingNdDescriptor(
            poolingDesc,
            mode,
            maxpoolingNanOpt,
            nbDims,
            windowDimA.as_const_ptr() as *const ::std::os::raw::c_int,
            paddingA.as_const_ptr() as *const ::std::os::raw::c_int,
            strideA.as_const_ptr() as *const ::std::os::raw::c_int,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnGetPoolingNdDescriptor(
    poolingDesc: cudnnPoolingDescriptor_t,
    nbDimsRequested: ::std::os::raw::c_int,
) -> Result<
    (
        cudnnPoolingMode_t,
        cudnnNanPropagation_t,
        ::std::os::raw::c_int,
        ::std::os::raw::c_int,
        ::std::os::raw::c_int,
        ::std::os::raw::c_int,
    ),
    crate::sys::cudnnStatus_t,
> {
    let mut out_2: std::mem::MaybeUninit<cudnnPoolingMode_t> = std::mem::MaybeUninit::uninit();
    let mut out_3: std::mem::MaybeUninit<cudnnNanPropagation_t> = std::mem::MaybeUninit::uninit();
    let mut out_4: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_5: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_6: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_7: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudnnGetPoolingNdDescriptor(
            poolingDesc,
            nbDimsRequested,
            out_2.as_mut_ptr() as *mut _,
            out_3.as_mut_ptr() as *mut _,
            out_4.as_mut_ptr() as *mut _,
            out_5.as_mut_ptr() as *mut _,
            out_6.as_mut_ptr() as *mut _,
            out_7.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        unsafe {
            Ok((
                out_2.assume_init(),
                out_3.assume_init(),
                out_4.assume_init(),
                out_5.assume_init(),
                out_6.assume_init(),
                out_7.assume_init(),
            ))
        }
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnGetPoolingNdForwardOutputDim(
    poolingDesc: cudnnPoolingDescriptor_t,
    inputTensorDesc: cudnnTensorDescriptor_t,
    nbDims: ::std::os::raw::c_int,
) -> Result<::std::os::raw::c_int, crate::sys::cudnnStatus_t> {
    let mut out_3: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudnnGetPoolingNdForwardOutputDim(
            poolingDesc,
            inputTensorDesc,
            nbDims,
            out_3.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        unsafe { Ok(out_3.assume_init()) }
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnGetPooling2dForwardOutputDim(
    poolingDesc: cudnnPoolingDescriptor_t,
    inputTensorDesc: cudnnTensorDescriptor_t,
) -> Result<
    (
        ::std::os::raw::c_int,
        ::std::os::raw::c_int,
        ::std::os::raw::c_int,
        ::std::os::raw::c_int,
    ),
    crate::sys::cudnnStatus_t,
> {
    let mut out_2: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_3: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_4: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_5: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudnnGetPooling2dForwardOutputDim(
            poolingDesc,
            inputTensorDesc,
            out_2.as_mut_ptr() as *mut _,
            out_3.as_mut_ptr() as *mut _,
            out_4.as_mut_ptr() as *mut _,
            out_5.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        unsafe {
            Ok((
                out_2.assume_init(),
                out_3.assume_init(),
                out_4.assume_init(),
                out_5.assume_init(),
            ))
        }
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnSetActivationDescriptor(
    activationDesc: cudnnActivationDescriptor_t,
    mode: cudnnActivationMode_t,
    reluNanOpt: cudnnNanPropagation_t,
    coef: f64,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status =
        unsafe { crate::sys::cudnnSetActivationDescriptor(activationDesc, mode, reluNanOpt, coef) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnGetActivationDescriptor(
    activationDesc: cudnnActivationDescriptor_t,
) -> Result<(cudnnActivationMode_t, cudnnNanPropagation_t, f64), crate::sys::cudnnStatus_t> {
    let mut out_1: std::mem::MaybeUninit<cudnnActivationMode_t> = std::mem::MaybeUninit::uninit();
    let mut out_2: std::mem::MaybeUninit<cudnnNanPropagation_t> = std::mem::MaybeUninit::uninit();
    let mut out_3: std::mem::MaybeUninit<f64> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudnnGetActivationDescriptor(
            activationDesc,
            out_1.as_mut_ptr() as *mut _,
            out_2.as_mut_ptr() as *mut _,
            out_3.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
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
pub unsafe fn cudnnSetActivationDescriptorSwishBeta(
    activationDesc: cudnnActivationDescriptor_t,
    swish_beta: f64,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status =
        unsafe { crate::sys::cudnnSetActivationDescriptorSwishBeta(activationDesc, swish_beta) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnGetActivationDescriptorSwishBeta(
    activationDesc: cudnnActivationDescriptor_t,
) -> Result<f64, crate::sys::cudnnStatus_t> {
    let mut out_1: std::mem::MaybeUninit<f64> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudnnGetActivationDescriptorSwishBeta(
            activationDesc,
            out_1.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        unsafe { Ok(out_1.assume_init()) }
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnSetLRNDescriptor(
    normDesc: cudnnLRNDescriptor_t,
    lrnN: ::std::os::raw::c_uint,
    lrnAlpha: f64,
    lrnBeta: f64,
    lrnK: f64,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status =
        unsafe { crate::sys::cudnnSetLRNDescriptor(normDesc, lrnN, lrnAlpha, lrnBeta, lrnK) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnGetLRNDescriptor(
    normDesc: cudnnLRNDescriptor_t,
) -> Result<(::std::os::raw::c_uint, f64, f64, f64), crate::sys::cudnnStatus_t> {
    let mut out_1: std::mem::MaybeUninit<::std::os::raw::c_uint> = std::mem::MaybeUninit::uninit();
    let mut out_2: std::mem::MaybeUninit<f64> = std::mem::MaybeUninit::uninit();
    let mut out_3: std::mem::MaybeUninit<f64> = std::mem::MaybeUninit::uninit();
    let mut out_4: std::mem::MaybeUninit<f64> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudnnGetLRNDescriptor(
            normDesc,
            out_1.as_mut_ptr() as *mut _,
            out_2.as_mut_ptr() as *mut _,
            out_3.as_mut_ptr() as *mut _,
            out_4.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        unsafe {
            Ok((
                out_1.assume_init(),
                out_2.assume_init(),
                out_3.assume_init(),
                out_4.assume_init(),
            ))
        }
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnDeriveBNTensorDescriptor(
    derivedBnDesc: cudnnTensorDescriptor_t,
    xDesc: cudnnTensorDescriptor_t,
    mode: cudnnBatchNormMode_t,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnDeriveBNTensorDescriptor(derivedBnDesc, xDesc, mode) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnDeriveNormTensorDescriptor(
    derivedNormScaleBiasDesc: cudnnTensorDescriptor_t,
    derivedNormMeanVarDesc: cudnnTensorDescriptor_t,
    xDesc: cudnnTensorDescriptor_t,
    mode: cudnnNormMode_t,
    groupCnt: ::std::os::raw::c_int,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnDeriveNormTensorDescriptor(
            derivedNormScaleBiasDesc,
            derivedNormMeanVarDesc,
            xDesc,
            mode,
            groupCnt,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnSetSpatialTransformerNdDescriptor<T: ::cuda_libs::types::CudaAsPtr>(
    stDesc: cudnnSpatialTransformerDescriptor_t,
    samplerType: cudnnSamplerType_t,
    dataType: cudnnDataType_t,
    nbDims: ::std::os::raw::c_int,
    dimA: T,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnSetSpatialTransformerNdDescriptor(
            stDesc,
            samplerType,
            dataType,
            nbDims,
            dimA.as_const_ptr() as *const ::std::os::raw::c_int,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnDropoutGetReserveSpaceSize(
    xdesc: cudnnTensorDescriptor_t,
) -> Result<usize, crate::sys::cudnnStatus_t> {
    let mut out_1: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let status =
        unsafe { crate::sys::cudnnDropoutGetReserveSpaceSize(xdesc, out_1.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        unsafe { Ok(out_1.assume_init()) }
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnSetDropoutDescriptor<T: ::cuda_libs::types::CudaAsPtr>(
    dropoutDesc: cudnnDropoutDescriptor_t,
    handle: cudnnHandle_t,
    dropout: f32,
    mut states: T,
    stateSizeInBytes: usize,
    seed: ::std::os::raw::c_ulonglong,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnSetDropoutDescriptor(
            dropoutDesc,
            handle,
            dropout,
            states.as_mut_ptr() as *mut ::std::os::raw::c_void,
            stateSizeInBytes,
            seed,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnRestoreDropoutDescriptor<T: ::cuda_libs::types::CudaAsPtr>(
    dropoutDesc: cudnnDropoutDescriptor_t,
    handle: cudnnHandle_t,
    dropout: f32,
    mut states: T,
    stateSizeInBytes: usize,
    seed: ::std::os::raw::c_ulonglong,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnRestoreDropoutDescriptor(
            dropoutDesc,
            handle,
            dropout,
            states.as_mut_ptr() as *mut ::std::os::raw::c_void,
            stateSizeInBytes,
            seed,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnGetDropoutDescriptor(
    dropoutDesc: cudnnDropoutDescriptor_t,
    handle: cudnnHandle_t,
    states: *mut *mut ::std::os::raw::c_void,
) -> Result<(f32, ::std::os::raw::c_ulonglong), crate::sys::cudnnStatus_t> {
    let mut out_2: std::mem::MaybeUninit<f32> = std::mem::MaybeUninit::uninit();
    let mut out_4: std::mem::MaybeUninit<::std::os::raw::c_ulonglong> =
        std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudnnGetDropoutDescriptor(
            dropoutDesc,
            handle,
            out_2.as_mut_ptr() as *mut _,
            states,
            out_4.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        unsafe { Ok((out_2.assume_init(), out_4.assume_init())) }
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnOpsVersionCheck() -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnOpsVersionCheck() };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnSetRNNDescriptor_v8(
    rnnDesc: cudnnRNNDescriptor_t,
    algo: cudnnRNNAlgo_t,
    cellMode: cudnnRNNMode_t,
    biasMode: cudnnRNNBiasMode_t,
    dirMode: cudnnDirectionMode_t,
    inputMode: cudnnRNNInputMode_t,
    dataType: cudnnDataType_t,
    mathPrec: cudnnDataType_t,
    mathType: cudnnMathType_t,
    inputSize: i32,
    hiddenSize: i32,
    projSize: i32,
    numLayers: i32,
    dropoutDesc: cudnnDropoutDescriptor_t,
    auxFlags: u32,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnSetRNNDescriptor_v8(
            rnnDesc,
            algo,
            cellMode,
            biasMode,
            dirMode,
            inputMode,
            dataType,
            mathPrec,
            mathType,
            inputSize,
            hiddenSize,
            projSize,
            numLayers,
            dropoutDesc,
            auxFlags,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnGetRNNDescriptor_v8(
    rnnDesc: cudnnRNNDescriptor_t,
) -> Result<
    (
        cudnnRNNAlgo_t,
        cudnnRNNMode_t,
        cudnnRNNBiasMode_t,
        cudnnDirectionMode_t,
        cudnnRNNInputMode_t,
        cudnnDataType_t,
        cudnnDataType_t,
        cudnnMathType_t,
        i32,
        i32,
        i32,
        i32,
        cudnnDropoutDescriptor_t,
        u32,
    ),
    crate::sys::cudnnStatus_t,
> {
    let mut out_1: std::mem::MaybeUninit<cudnnRNNAlgo_t> = std::mem::MaybeUninit::uninit();
    let mut out_2: std::mem::MaybeUninit<cudnnRNNMode_t> = std::mem::MaybeUninit::uninit();
    let mut out_3: std::mem::MaybeUninit<cudnnRNNBiasMode_t> = std::mem::MaybeUninit::uninit();
    let mut out_4: std::mem::MaybeUninit<cudnnDirectionMode_t> = std::mem::MaybeUninit::uninit();
    let mut out_5: std::mem::MaybeUninit<cudnnRNNInputMode_t> = std::mem::MaybeUninit::uninit();
    let mut out_6: std::mem::MaybeUninit<cudnnDataType_t> = std::mem::MaybeUninit::uninit();
    let mut out_7: std::mem::MaybeUninit<cudnnDataType_t> = std::mem::MaybeUninit::uninit();
    let mut out_8: std::mem::MaybeUninit<cudnnMathType_t> = std::mem::MaybeUninit::uninit();
    let mut out_9: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let mut out_10: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let mut out_11: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let mut out_12: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let mut out_13: std::mem::MaybeUninit<cudnnDropoutDescriptor_t> =
        std::mem::MaybeUninit::uninit();
    let mut out_14: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudnnGetRNNDescriptor_v8(
            rnnDesc,
            out_1.as_mut_ptr() as *mut _,
            out_2.as_mut_ptr() as *mut _,
            out_3.as_mut_ptr() as *mut _,
            out_4.as_mut_ptr() as *mut _,
            out_5.as_mut_ptr() as *mut _,
            out_6.as_mut_ptr() as *mut _,
            out_7.as_mut_ptr() as *mut _,
            out_8.as_mut_ptr() as *mut _,
            out_9.as_mut_ptr() as *mut _,
            out_10.as_mut_ptr() as *mut _,
            out_11.as_mut_ptr() as *mut _,
            out_12.as_mut_ptr() as *mut _,
            out_13.as_mut_ptr() as *mut _,
            out_14.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        unsafe {
            Ok((
                out_1.assume_init(),
                out_2.assume_init(),
                out_3.assume_init(),
                out_4.assume_init(),
                out_5.assume_init(),
                out_6.assume_init(),
                out_7.assume_init(),
                out_8.assume_init(),
                out_9.assume_init(),
                out_10.assume_init(),
                out_11.assume_init(),
                out_12.assume_init(),
                out_13.assume_init(),
                out_14.assume_init(),
            ))
        }
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnRNNSetClip_v8(
    rnnDesc: cudnnRNNDescriptor_t,
    clipMode: cudnnRNNClipMode_t,
    clipNanOpt: cudnnNanPropagation_t,
    lclip: f64,
    rclip: f64,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status =
        unsafe { crate::sys::cudnnRNNSetClip_v8(rnnDesc, clipMode, clipNanOpt, lclip, rclip) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnRNNSetClip_v9(
    rnnDesc: cudnnRNNDescriptor_t,
    clipMode: cudnnRNNClipMode_t,
    lclip: f64,
    rclip: f64,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnRNNSetClip_v9(rnnDesc, clipMode, lclip, rclip) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnRNNGetClip_v8(
    rnnDesc: cudnnRNNDescriptor_t,
) -> Result<(cudnnRNNClipMode_t, cudnnNanPropagation_t, f64, f64), crate::sys::cudnnStatus_t> {
    let mut out_1: std::mem::MaybeUninit<cudnnRNNClipMode_t> = std::mem::MaybeUninit::uninit();
    let mut out_2: std::mem::MaybeUninit<cudnnNanPropagation_t> = std::mem::MaybeUninit::uninit();
    let mut out_3: std::mem::MaybeUninit<f64> = std::mem::MaybeUninit::uninit();
    let mut out_4: std::mem::MaybeUninit<f64> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudnnRNNGetClip_v8(
            rnnDesc,
            out_1.as_mut_ptr() as *mut _,
            out_2.as_mut_ptr() as *mut _,
            out_3.as_mut_ptr() as *mut _,
            out_4.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        unsafe {
            Ok((
                out_1.assume_init(),
                out_2.assume_init(),
                out_3.assume_init(),
                out_4.assume_init(),
            ))
        }
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnRNNGetClip_v9(
    rnnDesc: cudnnRNNDescriptor_t,
) -> Result<(cudnnRNNClipMode_t, f64, f64), crate::sys::cudnnStatus_t> {
    let mut out_1: std::mem::MaybeUninit<cudnnRNNClipMode_t> = std::mem::MaybeUninit::uninit();
    let mut out_2: std::mem::MaybeUninit<f64> = std::mem::MaybeUninit::uninit();
    let mut out_3: std::mem::MaybeUninit<f64> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudnnRNNGetClip_v9(
            rnnDesc,
            out_1.as_mut_ptr() as *mut _,
            out_2.as_mut_ptr() as *mut _,
            out_3.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
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
pub unsafe fn cudnnSetRNNDataDescriptor<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
>(
    rnnDataDesc: cudnnRNNDataDescriptor_t,
    dataType: cudnnDataType_t,
    layout: cudnnRNNDataLayout_t,
    maxSeqLength: ::std::os::raw::c_int,
    batchSize: ::std::os::raw::c_int,
    vectorSize: ::std::os::raw::c_int,
    seqLengthArray: T,
    mut paddingFill: U,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnSetRNNDataDescriptor(
            rnnDataDesc,
            dataType,
            layout,
            maxSeqLength,
            batchSize,
            vectorSize,
            seqLengthArray.as_const_ptr() as *const ::std::os::raw::c_int,
            paddingFill.as_mut_ptr() as *mut ::std::os::raw::c_void,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnGetRNNDataDescriptor(
    rnnDataDesc: cudnnRNNDataDescriptor_t,
    arrayLengthRequested: ::std::os::raw::c_int,
    seqLengthArray: *mut ::std::os::raw::c_int,
    paddingFill: *mut ::std::os::raw::c_void,
) -> Result<
    (
        cudnnDataType_t,
        cudnnRNNDataLayout_t,
        ::std::os::raw::c_int,
        ::std::os::raw::c_int,
        ::std::os::raw::c_int,
    ),
    crate::sys::cudnnStatus_t,
> {
    let mut out_1: std::mem::MaybeUninit<cudnnDataType_t> = std::mem::MaybeUninit::uninit();
    let mut out_2: std::mem::MaybeUninit<cudnnRNNDataLayout_t> = std::mem::MaybeUninit::uninit();
    let mut out_3: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_4: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_5: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudnnGetRNNDataDescriptor(
            rnnDataDesc,
            out_1.as_mut_ptr() as *mut _,
            out_2.as_mut_ptr() as *mut _,
            out_3.as_mut_ptr() as *mut _,
            out_4.as_mut_ptr() as *mut _,
            out_5.as_mut_ptr() as *mut _,
            arrayLengthRequested,
            seqLengthArray,
            paddingFill,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        unsafe {
            Ok((
                out_1.assume_init(),
                out_2.assume_init(),
                out_3.assume_init(),
                out_4.assume_init(),
                out_5.assume_init(),
            ))
        }
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnSetSeqDataDescriptor<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
    W: ::cuda_libs::types::CudaAsPtr,
>(
    seqDataDesc: cudnnSeqDataDescriptor_t,
    dataType: cudnnDataType_t,
    nbDims: ::std::os::raw::c_int,
    dimA: T,
    axes: U,
    seqLengthArraySize: usize,
    seqLengthArray: V,
    mut paddingFill: W,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnSetSeqDataDescriptor(
            seqDataDesc,
            dataType,
            nbDims,
            dimA.as_const_ptr() as *const ::std::os::raw::c_int,
            axes.as_const_ptr() as *const cudnnSeqDataAxis_t,
            seqLengthArraySize,
            seqLengthArray.as_const_ptr() as *const ::std::os::raw::c_int,
            paddingFill.as_mut_ptr() as *mut ::std::os::raw::c_void,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnGetSeqDataDescriptor(
    seqDataDesc: cudnnSeqDataDescriptor_t,
    nbDimsRequested: ::std::os::raw::c_int,
    seqLengthArraySize: *mut usize,
    seqLengthSizeRequested: usize,
    seqLengthArray: *mut ::std::os::raw::c_int,
    paddingFill: *mut ::std::os::raw::c_void,
) -> Result<
    (
        cudnnDataType_t,
        ::std::os::raw::c_int,
        ::std::os::raw::c_int,
        cudnnSeqDataAxis_t,
    ),
    crate::sys::cudnnStatus_t,
> {
    let mut out_1: std::mem::MaybeUninit<cudnnDataType_t> = std::mem::MaybeUninit::uninit();
    let mut out_2: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_4: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_5: std::mem::MaybeUninit<cudnnSeqDataAxis_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudnnGetSeqDataDescriptor(
            seqDataDesc,
            out_1.as_mut_ptr() as *mut _,
            out_2.as_mut_ptr() as *mut _,
            nbDimsRequested,
            out_4.as_mut_ptr() as *mut _,
            out_5.as_mut_ptr() as *mut _,
            seqLengthArraySize,
            seqLengthSizeRequested,
            seqLengthArray,
            paddingFill,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        unsafe {
            Ok((
                out_1.assume_init(),
                out_2.assume_init(),
                out_4.assume_init(),
                out_5.assume_init(),
            ))
        }
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnSetAttnDescriptor(
    attnDesc: cudnnAttnDescriptor_t,
    attnMode: ::std::os::raw::c_uint,
    nHeads: ::std::os::raw::c_int,
    smScaler: f64,
    dataType: cudnnDataType_t,
    computePrec: cudnnDataType_t,
    mathType: cudnnMathType_t,
    attnDropoutDesc: cudnnDropoutDescriptor_t,
    postDropoutDesc: cudnnDropoutDescriptor_t,
    qSize: ::std::os::raw::c_int,
    kSize: ::std::os::raw::c_int,
    vSize: ::std::os::raw::c_int,
    qProjSize: ::std::os::raw::c_int,
    kProjSize: ::std::os::raw::c_int,
    vProjSize: ::std::os::raw::c_int,
    oProjSize: ::std::os::raw::c_int,
    qoMaxSeqLength: ::std::os::raw::c_int,
    kvMaxSeqLength: ::std::os::raw::c_int,
    maxBatchSize: ::std::os::raw::c_int,
    maxBeamSize: ::std::os::raw::c_int,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnSetAttnDescriptor(
            attnDesc,
            attnMode,
            nHeads,
            smScaler,
            dataType,
            computePrec,
            mathType,
            attnDropoutDesc,
            postDropoutDesc,
            qSize,
            kSize,
            vSize,
            qProjSize,
            kProjSize,
            vProjSize,
            oProjSize,
            qoMaxSeqLength,
            kvMaxSeqLength,
            maxBatchSize,
            maxBeamSize,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnGetAttnDescriptor(
    attnDesc: cudnnAttnDescriptor_t,
) -> Result<
    (
        ::std::os::raw::c_uint,
        ::std::os::raw::c_int,
        f64,
        cudnnDataType_t,
        cudnnDataType_t,
        cudnnMathType_t,
        cudnnDropoutDescriptor_t,
        cudnnDropoutDescriptor_t,
        ::std::os::raw::c_int,
        ::std::os::raw::c_int,
        ::std::os::raw::c_int,
        ::std::os::raw::c_int,
        ::std::os::raw::c_int,
        ::std::os::raw::c_int,
        ::std::os::raw::c_int,
        ::std::os::raw::c_int,
        ::std::os::raw::c_int,
        ::std::os::raw::c_int,
        ::std::os::raw::c_int,
    ),
    crate::sys::cudnnStatus_t,
> {
    let mut out_1: std::mem::MaybeUninit<::std::os::raw::c_uint> = std::mem::MaybeUninit::uninit();
    let mut out_2: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_3: std::mem::MaybeUninit<f64> = std::mem::MaybeUninit::uninit();
    let mut out_4: std::mem::MaybeUninit<cudnnDataType_t> = std::mem::MaybeUninit::uninit();
    let mut out_5: std::mem::MaybeUninit<cudnnDataType_t> = std::mem::MaybeUninit::uninit();
    let mut out_6: std::mem::MaybeUninit<cudnnMathType_t> = std::mem::MaybeUninit::uninit();
    let mut out_7: std::mem::MaybeUninit<cudnnDropoutDescriptor_t> =
        std::mem::MaybeUninit::uninit();
    let mut out_8: std::mem::MaybeUninit<cudnnDropoutDescriptor_t> =
        std::mem::MaybeUninit::uninit();
    let mut out_9: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_10: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_11: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_12: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_13: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_14: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_15: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_16: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_17: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_18: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_19: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudnnGetAttnDescriptor(
            attnDesc,
            out_1.as_mut_ptr() as *mut _,
            out_2.as_mut_ptr() as *mut _,
            out_3.as_mut_ptr() as *mut _,
            out_4.as_mut_ptr() as *mut _,
            out_5.as_mut_ptr() as *mut _,
            out_6.as_mut_ptr() as *mut _,
            out_7.as_mut_ptr() as *mut _,
            out_8.as_mut_ptr() as *mut _,
            out_9.as_mut_ptr() as *mut _,
            out_10.as_mut_ptr() as *mut _,
            out_11.as_mut_ptr() as *mut _,
            out_12.as_mut_ptr() as *mut _,
            out_13.as_mut_ptr() as *mut _,
            out_14.as_mut_ptr() as *mut _,
            out_15.as_mut_ptr() as *mut _,
            out_16.as_mut_ptr() as *mut _,
            out_17.as_mut_ptr() as *mut _,
            out_18.as_mut_ptr() as *mut _,
            out_19.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        unsafe {
            Ok((
                out_1.assume_init(),
                out_2.assume_init(),
                out_3.assume_init(),
                out_4.assume_init(),
                out_5.assume_init(),
                out_6.assume_init(),
                out_7.assume_init(),
                out_8.assume_init(),
                out_9.assume_init(),
                out_10.assume_init(),
                out_11.assume_init(),
                out_12.assume_init(),
                out_13.assume_init(),
                out_14.assume_init(),
                out_15.assume_init(),
                out_16.assume_init(),
                out_17.assume_init(),
                out_18.assume_init(),
                out_19.assume_init(),
            ))
        }
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnAdvVersionCheck() -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnAdvVersionCheck() };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnSetCTCLossDescriptor(
    ctcLossDesc: cudnnCTCLossDescriptor_t,
    compType: cudnnDataType_t,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnSetCTCLossDescriptor(ctcLossDesc, compType) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnSetCTCLossDescriptorEx(
    ctcLossDesc: cudnnCTCLossDescriptor_t,
    compType: cudnnDataType_t,
    normMode: cudnnLossNormalizationMode_t,
    gradMode: cudnnNanPropagation_t,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnSetCTCLossDescriptorEx(ctcLossDesc, compType, normMode, gradMode)
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnSetCTCLossDescriptor_v8(
    ctcLossDesc: cudnnCTCLossDescriptor_t,
    compType: cudnnDataType_t,
    normMode: cudnnLossNormalizationMode_t,
    gradMode: cudnnNanPropagation_t,
    maxLabelLength: ::std::os::raw::c_int,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnSetCTCLossDescriptor_v8(
            ctcLossDesc,
            compType,
            normMode,
            gradMode,
            maxLabelLength,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnSetCTCLossDescriptor_v9(
    ctcLossDesc: cudnnCTCLossDescriptor_t,
    compType: cudnnDataType_t,
    normMode: cudnnLossNormalizationMode_t,
    ctcGradMode: cudnnCTCGradMode_t,
    maxLabelLength: ::std::os::raw::c_int,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnSetCTCLossDescriptor_v9(
            ctcLossDesc,
            compType,
            normMode,
            ctcGradMode,
            maxLabelLength,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnGetCTCLossDescriptor(
    ctcLossDesc: cudnnCTCLossDescriptor_t,
) -> Result<cudnnDataType_t, crate::sys::cudnnStatus_t> {
    let mut out_1: std::mem::MaybeUninit<cudnnDataType_t> = std::mem::MaybeUninit::uninit();
    let status =
        unsafe { crate::sys::cudnnGetCTCLossDescriptor(ctcLossDesc, out_1.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        unsafe { Ok(out_1.assume_init()) }
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnGetCTCLossDescriptorEx(
    ctcLossDesc: cudnnCTCLossDescriptor_t,
) -> Result<
    (
        cudnnDataType_t,
        cudnnLossNormalizationMode_t,
        cudnnNanPropagation_t,
    ),
    crate::sys::cudnnStatus_t,
> {
    let mut out_1: std::mem::MaybeUninit<cudnnDataType_t> = std::mem::MaybeUninit::uninit();
    let mut out_2: std::mem::MaybeUninit<cudnnLossNormalizationMode_t> =
        std::mem::MaybeUninit::uninit();
    let mut out_3: std::mem::MaybeUninit<cudnnNanPropagation_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudnnGetCTCLossDescriptorEx(
            ctcLossDesc,
            out_1.as_mut_ptr() as *mut _,
            out_2.as_mut_ptr() as *mut _,
            out_3.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
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
pub unsafe fn cudnnGetCTCLossDescriptor_v8(
    ctcLossDesc: cudnnCTCLossDescriptor_t,
) -> Result<
    (
        cudnnDataType_t,
        cudnnLossNormalizationMode_t,
        cudnnNanPropagation_t,
        ::std::os::raw::c_int,
    ),
    crate::sys::cudnnStatus_t,
> {
    let mut out_1: std::mem::MaybeUninit<cudnnDataType_t> = std::mem::MaybeUninit::uninit();
    let mut out_2: std::mem::MaybeUninit<cudnnLossNormalizationMode_t> =
        std::mem::MaybeUninit::uninit();
    let mut out_3: std::mem::MaybeUninit<cudnnNanPropagation_t> = std::mem::MaybeUninit::uninit();
    let mut out_4: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudnnGetCTCLossDescriptor_v8(
            ctcLossDesc,
            out_1.as_mut_ptr() as *mut _,
            out_2.as_mut_ptr() as *mut _,
            out_3.as_mut_ptr() as *mut _,
            out_4.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        unsafe {
            Ok((
                out_1.assume_init(),
                out_2.assume_init(),
                out_3.assume_init(),
                out_4.assume_init(),
            ))
        }
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnGetCTCLossDescriptor_v9(
    ctcLossDesc: cudnnCTCLossDescriptor_t,
) -> Result<
    (
        cudnnDataType_t,
        cudnnLossNormalizationMode_t,
        cudnnCTCGradMode_t,
        ::std::os::raw::c_int,
    ),
    crate::sys::cudnnStatus_t,
> {
    let mut out_1: std::mem::MaybeUninit<cudnnDataType_t> = std::mem::MaybeUninit::uninit();
    let mut out_2: std::mem::MaybeUninit<cudnnLossNormalizationMode_t> =
        std::mem::MaybeUninit::uninit();
    let mut out_3: std::mem::MaybeUninit<cudnnCTCGradMode_t> = std::mem::MaybeUninit::uninit();
    let mut out_4: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudnnGetCTCLossDescriptor_v9(
            ctcLossDesc,
            out_1.as_mut_ptr() as *mut _,
            out_2.as_mut_ptr() as *mut _,
            out_3.as_mut_ptr() as *mut _,
            out_4.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        unsafe {
            Ok((
                out_1.assume_init(),
                out_2.assume_init(),
                out_3.assume_init(),
                out_4.assume_init(),
            ))
        }
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnSetConvolutionMathType(
    convDesc: cudnnConvolutionDescriptor_t,
    mathType: cudnnMathType_t,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnSetConvolutionMathType(convDesc, mathType) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnGetConvolutionMathType(
    convDesc: cudnnConvolutionDescriptor_t,
) -> Result<cudnnMathType_t, crate::sys::cudnnStatus_t> {
    let mut out_1: std::mem::MaybeUninit<cudnnMathType_t> = std::mem::MaybeUninit::uninit();
    let status =
        unsafe { crate::sys::cudnnGetConvolutionMathType(convDesc, out_1.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        unsafe { Ok(out_1.assume_init()) }
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnSetConvolutionGroupCount(
    convDesc: cudnnConvolutionDescriptor_t,
    groupCount: ::std::os::raw::c_int,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnSetConvolutionGroupCount(convDesc, groupCount) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnGetConvolutionGroupCount(
    convDesc: cudnnConvolutionDescriptor_t,
) -> Result<::std::os::raw::c_int, crate::sys::cudnnStatus_t> {
    let mut out_1: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudnnGetConvolutionGroupCount(convDesc, out_1.as_mut_ptr() as *mut _)
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        unsafe { Ok(out_1.assume_init()) }
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnSetConvolutionReorderType(
    convDesc: cudnnConvolutionDescriptor_t,
    reorderType: cudnnReorderType_t,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnSetConvolutionReorderType(convDesc, reorderType) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnGetConvolutionReorderType(
    convDesc: cudnnConvolutionDescriptor_t,
) -> Result<cudnnReorderType_t, crate::sys::cudnnStatus_t> {
    let mut out_1: std::mem::MaybeUninit<cudnnReorderType_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudnnGetConvolutionReorderType(convDesc, out_1.as_mut_ptr() as *mut _)
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        unsafe { Ok(out_1.assume_init()) }
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnSetConvolution2dDescriptor(
    convDesc: cudnnConvolutionDescriptor_t,
    pad_h: ::std::os::raw::c_int,
    pad_w: ::std::os::raw::c_int,
    u: ::std::os::raw::c_int,
    v: ::std::os::raw::c_int,
    dilation_h: ::std::os::raw::c_int,
    dilation_w: ::std::os::raw::c_int,
    mode: cudnnConvolutionMode_t,
    computeType: cudnnDataType_t,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnSetConvolution2dDescriptor(
            convDesc,
            pad_h,
            pad_w,
            u,
            v,
            dilation_h,
            dilation_w,
            mode,
            computeType,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnGetConvolution2dDescriptor(
    convDesc: cudnnConvolutionDescriptor_t,
) -> Result<
    (
        ::std::os::raw::c_int,
        ::std::os::raw::c_int,
        ::std::os::raw::c_int,
        ::std::os::raw::c_int,
        ::std::os::raw::c_int,
        ::std::os::raw::c_int,
        cudnnConvolutionMode_t,
        cudnnDataType_t,
    ),
    crate::sys::cudnnStatus_t,
> {
    let mut out_1: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_2: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_3: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_4: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_5: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_6: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_7: std::mem::MaybeUninit<cudnnConvolutionMode_t> = std::mem::MaybeUninit::uninit();
    let mut out_8: std::mem::MaybeUninit<cudnnDataType_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudnnGetConvolution2dDescriptor(
            convDesc,
            out_1.as_mut_ptr() as *mut _,
            out_2.as_mut_ptr() as *mut _,
            out_3.as_mut_ptr() as *mut _,
            out_4.as_mut_ptr() as *mut _,
            out_5.as_mut_ptr() as *mut _,
            out_6.as_mut_ptr() as *mut _,
            out_7.as_mut_ptr() as *mut _,
            out_8.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        unsafe {
            Ok((
                out_1.assume_init(),
                out_2.assume_init(),
                out_3.assume_init(),
                out_4.assume_init(),
                out_5.assume_init(),
                out_6.assume_init(),
                out_7.assume_init(),
                out_8.assume_init(),
            ))
        }
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnSetConvolutionNdDescriptor<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
>(
    convDesc: cudnnConvolutionDescriptor_t,
    arrayLength: ::std::os::raw::c_int,
    padA: T,
    filterStrideA: U,
    dilationA: V,
    mode: cudnnConvolutionMode_t,
    computeType: cudnnDataType_t,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnSetConvolutionNdDescriptor(
            convDesc,
            arrayLength,
            padA.as_const_ptr() as *const ::std::os::raw::c_int,
            filterStrideA.as_const_ptr() as *const ::std::os::raw::c_int,
            dilationA.as_const_ptr() as *const ::std::os::raw::c_int,
            mode,
            computeType,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnGetConvolutionNdDescriptor(
    convDesc: cudnnConvolutionDescriptor_t,
    arrayLengthRequested: ::std::os::raw::c_int,
) -> Result<
    (
        ::std::os::raw::c_int,
        ::std::os::raw::c_int,
        ::std::os::raw::c_int,
        ::std::os::raw::c_int,
        cudnnConvolutionMode_t,
        cudnnDataType_t,
    ),
    crate::sys::cudnnStatus_t,
> {
    let mut out_2: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_3: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_4: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_5: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_6: std::mem::MaybeUninit<cudnnConvolutionMode_t> = std::mem::MaybeUninit::uninit();
    let mut out_7: std::mem::MaybeUninit<cudnnDataType_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudnnGetConvolutionNdDescriptor(
            convDesc,
            arrayLengthRequested,
            out_2.as_mut_ptr() as *mut _,
            out_3.as_mut_ptr() as *mut _,
            out_4.as_mut_ptr() as *mut _,
            out_5.as_mut_ptr() as *mut _,
            out_6.as_mut_ptr() as *mut _,
            out_7.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        unsafe {
            Ok((
                out_2.assume_init(),
                out_3.assume_init(),
                out_4.assume_init(),
                out_5.assume_init(),
                out_6.assume_init(),
                out_7.assume_init(),
            ))
        }
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnGetConvolution2dForwardOutputDim(
    convDesc: cudnnConvolutionDescriptor_t,
    inputTensorDesc: cudnnTensorDescriptor_t,
    filterDesc: cudnnFilterDescriptor_t,
) -> Result<
    (
        ::std::os::raw::c_int,
        ::std::os::raw::c_int,
        ::std::os::raw::c_int,
        ::std::os::raw::c_int,
    ),
    crate::sys::cudnnStatus_t,
> {
    let mut out_3: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_4: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_5: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_6: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudnnGetConvolution2dForwardOutputDim(
            convDesc,
            inputTensorDesc,
            filterDesc,
            out_3.as_mut_ptr() as *mut _,
            out_4.as_mut_ptr() as *mut _,
            out_5.as_mut_ptr() as *mut _,
            out_6.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        unsafe {
            Ok((
                out_3.assume_init(),
                out_4.assume_init(),
                out_5.assume_init(),
                out_6.assume_init(),
            ))
        }
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnGetConvolutionNdForwardOutputDim(
    convDesc: cudnnConvolutionDescriptor_t,
    inputTensorDesc: cudnnTensorDescriptor_t,
    filterDesc: cudnnFilterDescriptor_t,
    nbDims: ::std::os::raw::c_int,
) -> Result<::std::os::raw::c_int, crate::sys::cudnnStatus_t> {
    let mut out_4: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudnnGetConvolutionNdForwardOutputDim(
            convDesc,
            inputTensorDesc,
            filterDesc,
            nbDims,
            out_4.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        unsafe { Ok(out_4.assume_init()) }
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnCnnVersionCheck() -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnCnnVersionCheck() };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnSetFusedOpsConstParamPackAttribute<T: ::cuda_libs::types::CudaAsPtr>(
    constPack: cudnnFusedOpsConstParamPack_t,
    paramLabel: cudnnFusedOpsConstParamLabel_t,
    param: T,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnSetFusedOpsConstParamPackAttribute(
            constPack,
            paramLabel,
            param.as_const_ptr() as *const ::std::os::raw::c_void,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnGetFusedOpsConstParamPackAttribute(
    constPack: cudnnFusedOpsConstParamPack_t,
    paramLabel: cudnnFusedOpsConstParamLabel_t,
    param: *mut ::std::os::raw::c_void,
) -> Result<::std::os::raw::c_int, crate::sys::cudnnStatus_t> {
    let mut out_3: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudnnGetFusedOpsConstParamPackAttribute(
            constPack,
            paramLabel,
            param,
            out_3.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        unsafe { Ok(out_3.assume_init()) }
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnSetFusedOpsVariantParamPackAttribute<T: ::cuda_libs::types::CudaAsPtr>(
    varPack: cudnnFusedOpsVariantParamPack_t,
    paramLabel: cudnnFusedOpsVariantParamLabel_t,
    mut ptr: T,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnSetFusedOpsVariantParamPackAttribute(
            varPack,
            paramLabel,
            ptr.as_mut_ptr() as *mut ::std::os::raw::c_void,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnGetFusedOpsVariantParamPackAttribute<T: ::cuda_libs::types::CudaAsPtr>(
    varPack: cudnnFusedOpsVariantParamPack_t,
    paramLabel: cudnnFusedOpsVariantParamLabel_t,
    mut ptr: T,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnGetFusedOpsVariantParamPackAttribute(
            varPack,
            paramLabel,
            ptr.as_mut_ptr() as *mut ::std::os::raw::c_void,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
impl CudnnHandle {
    pub fn new() -> Result<Self, crate::sys::cudnnStatus_t> {
        unsafe {
            let mut handle = std::ptr::null_mut();
            let status = crate::sys::cudnnCreate(&mut handle);
            if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
                Ok(Self { handle })
            } else {
                Err(status)
            }
        }
    }
}
impl Drop for CudnnHandle {
    fn drop(&mut self) {
        unsafe {
            crate::sys::cudnnDestroy(self.handle);
        }
    }
}
