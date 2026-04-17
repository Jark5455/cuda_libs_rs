pub use crate::sys::cudnnStatus_t as CudaTargetStatus;
#[allow(unused_imports)]
use crate::sys::*;
use cuda_libs_cudart;
#[allow(unused_imports)]
use cuda_libs_cudart::sys::*;
#[allow(unused_imports)]
use cuda_libs_cudart::types;
#[cfg(feature = "runtime-link")]
impl crate::sys::cudnnDebugStruct {
    pub fn cudnn_version(mut self, val: u32) -> Self {
        self.cudnn_version = val as _;
        self
    }
    pub fn cudnnStatus(mut self, val: cudnnStatus_t) -> Self {
        self.cudnnStatus = val;
        self
    }
    pub fn time_sec(mut self, val: u32) -> Self {
        self.time_sec = val as _;
        self
    }
    pub fn time_usec(mut self, val: u32) -> Self {
        self.time_usec = val as _;
        self
    }
    pub fn time_delta(mut self, val: u32) -> Self {
        self.time_delta = val as _;
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
    pub fn pid(mut self, val: u64) -> Self {
        self.pid = val as _;
        self
    }
    pub fn tid(mut self, val: u64) -> Self {
        self.tid = val as _;
        self
    }
    pub fn cudaDeviceId(mut self, val: i32) -> Self {
        self.cudaDeviceId = val as _;
        self
    }
    pub fn reserved(mut self, val: [::std::os::raw::c_int; 15usize]) -> Self {
        self.reserved = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
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
#[cfg(feature = "runtime-link")]
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
#[cfg(feature = "runtime-link")]
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
#[cfg(feature = "runtime-link")]
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
#[cfg(feature = "runtime-link")]
impl crate::sys::DynamicBindings {
    pub fn cudnnGetVersion(mut self, val: Option<unsafe extern "C" fn() -> usize>) -> Self {
        self.cudnnGetVersion = val;
        self
    }
    pub fn cudnnGetMaxDeviceVersion(mut self, val: Option<unsafe extern "C" fn() -> usize>) -> Self {
        self.cudnnGetMaxDeviceVersion = val;
        self
    }
    pub fn cudnnGetCudartVersion(mut self, val: Option<unsafe extern "C" fn() -> usize>) -> Self {
        self.cudnnGetCudartVersion = val;
        self
    }
    pub fn cudnnGetErrorString(
        mut self,
        val: Option<unsafe extern "C" fn(status: cudnnStatus_t) -> *const ::std::os::raw::c_char>,
    ) -> Self {
        self.cudnnGetErrorString = val;
        self
    }
    pub fn cudnnGetLastErrorString(
        mut self,
        val: Option<unsafe extern "C" fn(message: *mut ::std::os::raw::c_char, max_size: usize)>,
    ) -> Self {
        self.cudnnGetLastErrorString = val;
        self
    }
    pub fn cudnnQueryRuntimeError(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                rstatus: *mut cudnnStatus_t,
                mode: cudnnErrQueryMode_t,
                tag: *mut cudnnRuntimeTag_t,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnQueryRuntimeError = val;
        self
    }
    pub fn cudnnGetProperty(
        mut self,
        val: Option<
            unsafe extern "C" fn(type_: libraryPropertyType, value: *mut ::std::os::raw::c_int) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnGetProperty = val;
        self
    }
    pub fn cudnnCreate(
        mut self,
        val: Option<unsafe extern "C" fn(handle: *mut cudnnHandle_t) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnCreate = val;
        self
    }
    pub fn cudnnDestroy(mut self, val: Option<unsafe extern "C" fn(handle: cudnnHandle_t) -> cudnnStatus_t>) -> Self {
        self.cudnnDestroy = val;
        self
    }
    pub fn cudnnSetStream(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cudnnHandle_t, streamId: cudaStream_t) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnSetStream = val;
        self
    }
    pub fn cudnnGetStream(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cudnnHandle_t, streamId: *mut cudaStream_t) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnGetStream = val;
        self
    }
    pub fn cudnnSetCallback(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                mask: ::std::os::raw::c_uint,
                udata: *mut ::std::os::raw::c_void,
                fptr: cudnnCallback_t,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnSetCallback = val;
        self
    }
    pub fn cudnnGetCallback(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                mask: *mut ::std::os::raw::c_uint,
                udata: *mut *mut ::std::os::raw::c_void,
                fptr: *mut cudnnCallback_t,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnGetCallback = val;
        self
    }
    pub fn cudnnGraphVersionCheck(mut self, val: Option<unsafe extern "C" fn() -> cudnnStatus_t>) -> Self {
        self.cudnnGraphVersionCheck = val;
        self
    }
    pub fn cudnnBackendCreateDescriptor(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                descriptorType: cudnnBackendDescriptorType_t,
                descriptor: *mut cudnnBackendDescriptor_t,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnBackendCreateDescriptor = val;
        self
    }
    pub fn cudnnBackendDestroyDescriptor(
        mut self,
        val: Option<unsafe extern "C" fn(descriptor: cudnnBackendDescriptor_t) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnBackendDestroyDescriptor = val;
        self
    }
    pub fn cudnnBackendInitialize(
        mut self,
        val: Option<unsafe extern "C" fn(descriptor: cudnnBackendDescriptor_t) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnBackendInitialize = val;
        self
    }
    pub fn cudnnBackendFinalize(
        mut self,
        val: Option<unsafe extern "C" fn(descriptor: cudnnBackendDescriptor_t) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnBackendFinalize = val;
        self
    }
    pub fn cudnnBackendSetAttribute(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                descriptor: cudnnBackendDescriptor_t,
                attributeName: cudnnBackendAttributeName_t,
                attributeType: cudnnBackendAttributeType_t,
                elementCount: i64,
                arrayOfElements: *const ::std::os::raw::c_void,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnBackendSetAttribute = val;
        self
    }
    pub fn cudnnBackendGetAttribute(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                descriptor: cudnnBackendDescriptor_t,
                attributeName: cudnnBackendAttributeName_t,
                attributeType: cudnnBackendAttributeType_t,
                requestedElementCount: i64,
                elementCount: *mut i64,
                arrayOfElements: *mut ::std::os::raw::c_void,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnBackendGetAttribute = val;
        self
    }
    pub fn cudnnBackendExecute(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                executionPlan: cudnnBackendDescriptor_t,
                variantPack: cudnnBackendDescriptor_t,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnBackendExecute = val;
        self
    }
    pub fn cudnnBackendPopulateCudaGraph(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                executionPlan: cudnnBackendDescriptor_t,
                variantPack: cudnnBackendDescriptor_t,
                graph: cudaGraph_t,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnBackendPopulateCudaGraph = val;
        self
    }
    pub fn cudnnBackendUpdateCudaGraph(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                executionPlan: cudnnBackendDescriptor_t,
                variantPack: cudnnBackendDescriptor_t,
                graph: cudaGraph_t,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnBackendUpdateCudaGraph = val;
        self
    }
    pub fn cudnnCreateTensorDescriptor(
        mut self,
        val: Option<unsafe extern "C" fn(tensorDesc: *mut cudnnTensorDescriptor_t) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnCreateTensorDescriptor = val;
        self
    }
    pub fn cudnnSetTensor4dDescriptor(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                tensorDesc: cudnnTensorDescriptor_t,
                format: cudnnTensorFormat_t,
                dataType: cudnnDataType_t,
                n: ::std::os::raw::c_int,
                c: ::std::os::raw::c_int,
                h: ::std::os::raw::c_int,
                w: ::std::os::raw::c_int,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnSetTensor4dDescriptor = val;
        self
    }
    pub fn cudnnSetTensor4dDescriptorEx(
        mut self,
        val: Option<
            unsafe extern "C" fn(
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
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnSetTensor4dDescriptorEx = val;
        self
    }
    pub fn cudnnGetTensor4dDescriptor(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                tensorDesc: cudnnTensorDescriptor_t,
                dataType: *mut cudnnDataType_t,
                n: *mut ::std::os::raw::c_int,
                c: *mut ::std::os::raw::c_int,
                h: *mut ::std::os::raw::c_int,
                w: *mut ::std::os::raw::c_int,
                nStride: *mut ::std::os::raw::c_int,
                cStride: *mut ::std::os::raw::c_int,
                hStride: *mut ::std::os::raw::c_int,
                wStride: *mut ::std::os::raw::c_int,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnGetTensor4dDescriptor = val;
        self
    }
    pub fn cudnnSetTensorNdDescriptor(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                tensorDesc: cudnnTensorDescriptor_t,
                dataType: cudnnDataType_t,
                nbDims: ::std::os::raw::c_int,
                dimA: *const ::std::os::raw::c_int,
                strideA: *const ::std::os::raw::c_int,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnSetTensorNdDescriptor = val;
        self
    }
    pub fn cudnnSetTensorNdDescriptorEx(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                tensorDesc: cudnnTensorDescriptor_t,
                format: cudnnTensorFormat_t,
                dataType: cudnnDataType_t,
                nbDims: ::std::os::raw::c_int,
                dimA: *const ::std::os::raw::c_int,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnSetTensorNdDescriptorEx = val;
        self
    }
    pub fn cudnnGetTensorNdDescriptor(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                tensorDesc: cudnnTensorDescriptor_t,
                nbDimsRequested: ::std::os::raw::c_int,
                dataType: *mut cudnnDataType_t,
                nbDims: *mut ::std::os::raw::c_int,
                dimA: *mut ::std::os::raw::c_int,
                strideA: *mut ::std::os::raw::c_int,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnGetTensorNdDescriptor = val;
        self
    }
    pub fn cudnnGetTensorSizeInBytes(
        mut self,
        val: Option<unsafe extern "C" fn(tensorDesc: cudnnTensorDescriptor_t, size: *mut usize) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnGetTensorSizeInBytes = val;
        self
    }
    pub fn cudnnDestroyTensorDescriptor(
        mut self,
        val: Option<unsafe extern "C" fn(tensorDesc: cudnnTensorDescriptor_t) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnDestroyTensorDescriptor = val;
        self
    }
    pub fn cudnnInitTransformDest(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                transformDesc: cudnnTensorTransformDescriptor_t,
                srcDesc: cudnnTensorDescriptor_t,
                destDesc: cudnnTensorDescriptor_t,
                destSizeInBytes: *mut usize,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnInitTransformDest = val;
        self
    }
    pub fn cudnnCreateTensorTransformDescriptor(
        mut self,
        val: Option<unsafe extern "C" fn(transformDesc: *mut cudnnTensorTransformDescriptor_t) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnCreateTensorTransformDescriptor = val;
        self
    }
    pub fn cudnnSetTensorTransformDescriptor(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                transformDesc: cudnnTensorTransformDescriptor_t,
                nbDims: u32,
                destFormat: cudnnTensorFormat_t,
                padBeforeA: *const i32,
                padAfterA: *const i32,
                foldA: *const u32,
                direction: cudnnFoldingDirection_t,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnSetTensorTransformDescriptor = val;
        self
    }
    pub fn cudnnGetTensorTransformDescriptor(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                transformDesc: cudnnTensorTransformDescriptor_t,
                nbDimsRequested: u32,
                destFormat: *mut cudnnTensorFormat_t,
                padBeforeA: *mut i32,
                padAfterA: *mut i32,
                foldA: *mut u32,
                direction: *mut cudnnFoldingDirection_t,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnGetTensorTransformDescriptor = val;
        self
    }
    pub fn cudnnDestroyTensorTransformDescriptor(
        mut self,
        val: Option<unsafe extern "C" fn(transformDesc: cudnnTensorTransformDescriptor_t) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnDestroyTensorTransformDescriptor = val;
        self
    }
    pub fn cudnnTransformTensor(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                alpha: *const ::std::os::raw::c_void,
                xDesc: cudnnTensorDescriptor_t,
                x: *const ::std::os::raw::c_void,
                beta: *const ::std::os::raw::c_void,
                yDesc: cudnnTensorDescriptor_t,
                y: *mut ::std::os::raw::c_void,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnTransformTensor = val;
        self
    }
    pub fn cudnnTransformTensorEx(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                transDesc: cudnnTensorTransformDescriptor_t,
                alpha: *const ::std::os::raw::c_void,
                srcDesc: cudnnTensorDescriptor_t,
                srcData: *const ::std::os::raw::c_void,
                beta: *const ::std::os::raw::c_void,
                destDesc: cudnnTensorDescriptor_t,
                destData: *mut ::std::os::raw::c_void,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnTransformTensorEx = val;
        self
    }
    pub fn cudnnAddTensor(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                alpha: *const ::std::os::raw::c_void,
                aDesc: cudnnTensorDescriptor_t,
                A: *const ::std::os::raw::c_void,
                beta: *const ::std::os::raw::c_void,
                cDesc: cudnnTensorDescriptor_t,
                C: *mut ::std::os::raw::c_void,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnAddTensor = val;
        self
    }
    pub fn cudnnCreateOpTensorDescriptor(
        mut self,
        val: Option<unsafe extern "C" fn(opTensorDesc: *mut cudnnOpTensorDescriptor_t) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnCreateOpTensorDescriptor = val;
        self
    }
    pub fn cudnnSetOpTensorDescriptor(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                opTensorDesc: cudnnOpTensorDescriptor_t,
                opTensorOp: cudnnOpTensorOp_t,
                opTensorCompType: cudnnDataType_t,
                opTensorNanOpt: cudnnNanPropagation_t,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnSetOpTensorDescriptor = val;
        self
    }
    pub fn cudnnGetOpTensorDescriptor(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                opTensorDesc: cudnnOpTensorDescriptor_t,
                opTensorOp: *mut cudnnOpTensorOp_t,
                opTensorCompType: *mut cudnnDataType_t,
                opTensorNanOpt: *mut cudnnNanPropagation_t,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnGetOpTensorDescriptor = val;
        self
    }
    pub fn cudnnDestroyOpTensorDescriptor(
        mut self,
        val: Option<unsafe extern "C" fn(opTensorDesc: cudnnOpTensorDescriptor_t) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnDestroyOpTensorDescriptor = val;
        self
    }
    pub fn cudnnOpTensor(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                opTensorDesc: cudnnOpTensorDescriptor_t,
                alpha1: *const ::std::os::raw::c_void,
                aDesc: cudnnTensorDescriptor_t,
                A: *const ::std::os::raw::c_void,
                alpha2: *const ::std::os::raw::c_void,
                bDesc: cudnnTensorDescriptor_t,
                B: *const ::std::os::raw::c_void,
                beta: *const ::std::os::raw::c_void,
                cDesc: cudnnTensorDescriptor_t,
                C: *mut ::std::os::raw::c_void,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnOpTensor = val;
        self
    }
    pub fn cudnnCreateReduceTensorDescriptor(
        mut self,
        val: Option<unsafe extern "C" fn(reduceTensorDesc: *mut cudnnReduceTensorDescriptor_t) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnCreateReduceTensorDescriptor = val;
        self
    }
    pub fn cudnnSetReduceTensorDescriptor(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                reduceTensorDesc: cudnnReduceTensorDescriptor_t,
                reduceTensorOp: cudnnReduceTensorOp_t,
                reduceTensorCompType: cudnnDataType_t,
                reduceTensorNanOpt: cudnnNanPropagation_t,
                reduceTensorIndices: cudnnReduceTensorIndices_t,
                reduceTensorIndicesType: cudnnIndicesType_t,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnSetReduceTensorDescriptor = val;
        self
    }
    pub fn cudnnGetReduceTensorDescriptor(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                reduceTensorDesc: cudnnReduceTensorDescriptor_t,
                reduceTensorOp: *mut cudnnReduceTensorOp_t,
                reduceTensorCompType: *mut cudnnDataType_t,
                reduceTensorNanOpt: *mut cudnnNanPropagation_t,
                reduceTensorIndices: *mut cudnnReduceTensorIndices_t,
                reduceTensorIndicesType: *mut cudnnIndicesType_t,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnGetReduceTensorDescriptor = val;
        self
    }
    pub fn cudnnDestroyReduceTensorDescriptor(
        mut self,
        val: Option<unsafe extern "C" fn(reduceTensorDesc: cudnnReduceTensorDescriptor_t) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnDestroyReduceTensorDescriptor = val;
        self
    }
    pub fn cudnnGetReductionIndicesSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                reduceTensorDesc: cudnnReduceTensorDescriptor_t,
                aDesc: cudnnTensorDescriptor_t,
                cDesc: cudnnTensorDescriptor_t,
                sizeInBytes: *mut usize,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnGetReductionIndicesSize = val;
        self
    }
    pub fn cudnnGetReductionWorkspaceSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                reduceTensorDesc: cudnnReduceTensorDescriptor_t,
                aDesc: cudnnTensorDescriptor_t,
                cDesc: cudnnTensorDescriptor_t,
                sizeInBytes: *mut usize,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnGetReductionWorkspaceSize = val;
        self
    }
    pub fn cudnnReduceTensor(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                reduceTensorDesc: cudnnReduceTensorDescriptor_t,
                indices: *mut ::std::os::raw::c_void,
                indicesSizeInBytes: usize,
                workspace: *mut ::std::os::raw::c_void,
                workspaceSizeInBytes: usize,
                alpha: *const ::std::os::raw::c_void,
                aDesc: cudnnTensorDescriptor_t,
                A: *const ::std::os::raw::c_void,
                beta: *const ::std::os::raw::c_void,
                cDesc: cudnnTensorDescriptor_t,
                C: *mut ::std::os::raw::c_void,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnReduceTensor = val;
        self
    }
    pub fn cudnnSetTensor(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                yDesc: cudnnTensorDescriptor_t,
                y: *mut ::std::os::raw::c_void,
                valuePtr: *const ::std::os::raw::c_void,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnSetTensor = val;
        self
    }
    pub fn cudnnScaleTensor(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                yDesc: cudnnTensorDescriptor_t,
                y: *mut ::std::os::raw::c_void,
                alpha: *const ::std::os::raw::c_void,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnScaleTensor = val;
        self
    }
    pub fn cudnnCreateFilterDescriptor(
        mut self,
        val: Option<unsafe extern "C" fn(filterDesc: *mut cudnnFilterDescriptor_t) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnCreateFilterDescriptor = val;
        self
    }
    pub fn cudnnSetFilter4dDescriptor(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                filterDesc: cudnnFilterDescriptor_t,
                dataType: cudnnDataType_t,
                format: cudnnTensorFormat_t,
                k: ::std::os::raw::c_int,
                c: ::std::os::raw::c_int,
                h: ::std::os::raw::c_int,
                w: ::std::os::raw::c_int,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnSetFilter4dDescriptor = val;
        self
    }
    pub fn cudnnGetFilter4dDescriptor(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                filterDesc: cudnnFilterDescriptor_t,
                dataType: *mut cudnnDataType_t,
                format: *mut cudnnTensorFormat_t,
                k: *mut ::std::os::raw::c_int,
                c: *mut ::std::os::raw::c_int,
                h: *mut ::std::os::raw::c_int,
                w: *mut ::std::os::raw::c_int,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnGetFilter4dDescriptor = val;
        self
    }
    pub fn cudnnSetFilterNdDescriptor(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                filterDesc: cudnnFilterDescriptor_t,
                dataType: cudnnDataType_t,
                format: cudnnTensorFormat_t,
                nbDims: ::std::os::raw::c_int,
                filterDimA: *const ::std::os::raw::c_int,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnSetFilterNdDescriptor = val;
        self
    }
    pub fn cudnnGetFilterNdDescriptor(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                filterDesc: cudnnFilterDescriptor_t,
                nbDimsRequested: ::std::os::raw::c_int,
                dataType: *mut cudnnDataType_t,
                format: *mut cudnnTensorFormat_t,
                nbDims: *mut ::std::os::raw::c_int,
                filterDimA: *mut ::std::os::raw::c_int,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnGetFilterNdDescriptor = val;
        self
    }
    pub fn cudnnGetFilterSizeInBytes(
        mut self,
        val: Option<unsafe extern "C" fn(filterDesc: cudnnFilterDescriptor_t, size: *mut usize) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnGetFilterSizeInBytes = val;
        self
    }
    pub fn cudnnTransformFilter(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                transDesc: cudnnTensorTransformDescriptor_t,
                alpha: *const ::std::os::raw::c_void,
                srcDesc: cudnnFilterDescriptor_t,
                srcData: *const ::std::os::raw::c_void,
                beta: *const ::std::os::raw::c_void,
                destDesc: cudnnFilterDescriptor_t,
                destData: *mut ::std::os::raw::c_void,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnTransformFilter = val;
        self
    }
    pub fn cudnnDestroyFilterDescriptor(
        mut self,
        val: Option<unsafe extern "C" fn(filterDesc: cudnnFilterDescriptor_t) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnDestroyFilterDescriptor = val;
        self
    }
    pub fn cudnnSoftmaxForward(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                algo: cudnnSoftmaxAlgorithm_t,
                mode: cudnnSoftmaxMode_t,
                alpha: *const ::std::os::raw::c_void,
                xDesc: cudnnTensorDescriptor_t,
                x: *const ::std::os::raw::c_void,
                beta: *const ::std::os::raw::c_void,
                yDesc: cudnnTensorDescriptor_t,
                y: *mut ::std::os::raw::c_void,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnSoftmaxForward = val;
        self
    }
    pub fn cudnnCreatePoolingDescriptor(
        mut self,
        val: Option<unsafe extern "C" fn(poolingDesc: *mut cudnnPoolingDescriptor_t) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnCreatePoolingDescriptor = val;
        self
    }
    pub fn cudnnSetPooling2dDescriptor(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                poolingDesc: cudnnPoolingDescriptor_t,
                mode: cudnnPoolingMode_t,
                maxpoolingNanOpt: cudnnNanPropagation_t,
                windowHeight: ::std::os::raw::c_int,
                windowWidth: ::std::os::raw::c_int,
                verticalPadding: ::std::os::raw::c_int,
                horizontalPadding: ::std::os::raw::c_int,
                verticalStride: ::std::os::raw::c_int,
                horizontalStride: ::std::os::raw::c_int,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnSetPooling2dDescriptor = val;
        self
    }
    pub fn cudnnGetPooling2dDescriptor(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                poolingDesc: cudnnPoolingDescriptor_t,
                mode: *mut cudnnPoolingMode_t,
                maxpoolingNanOpt: *mut cudnnNanPropagation_t,
                windowHeight: *mut ::std::os::raw::c_int,
                windowWidth: *mut ::std::os::raw::c_int,
                verticalPadding: *mut ::std::os::raw::c_int,
                horizontalPadding: *mut ::std::os::raw::c_int,
                verticalStride: *mut ::std::os::raw::c_int,
                horizontalStride: *mut ::std::os::raw::c_int,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnGetPooling2dDescriptor = val;
        self
    }
    pub fn cudnnSetPoolingNdDescriptor(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                poolingDesc: cudnnPoolingDescriptor_t,
                mode: cudnnPoolingMode_t,
                maxpoolingNanOpt: cudnnNanPropagation_t,
                nbDims: ::std::os::raw::c_int,
                windowDimA: *const ::std::os::raw::c_int,
                paddingA: *const ::std::os::raw::c_int,
                strideA: *const ::std::os::raw::c_int,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnSetPoolingNdDescriptor = val;
        self
    }
    pub fn cudnnGetPoolingNdDescriptor(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                poolingDesc: cudnnPoolingDescriptor_t,
                nbDimsRequested: ::std::os::raw::c_int,
                mode: *mut cudnnPoolingMode_t,
                maxpoolingNanOpt: *mut cudnnNanPropagation_t,
                nbDims: *mut ::std::os::raw::c_int,
                windowDimA: *mut ::std::os::raw::c_int,
                paddingA: *mut ::std::os::raw::c_int,
                strideA: *mut ::std::os::raw::c_int,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnGetPoolingNdDescriptor = val;
        self
    }
    pub fn cudnnGetPoolingNdForwardOutputDim(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                poolingDesc: cudnnPoolingDescriptor_t,
                inputTensorDesc: cudnnTensorDescriptor_t,
                nbDims: ::std::os::raw::c_int,
                outputTensorDimA: *mut ::std::os::raw::c_int,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnGetPoolingNdForwardOutputDim = val;
        self
    }
    pub fn cudnnGetPooling2dForwardOutputDim(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                poolingDesc: cudnnPoolingDescriptor_t,
                inputTensorDesc: cudnnTensorDescriptor_t,
                n: *mut ::std::os::raw::c_int,
                c: *mut ::std::os::raw::c_int,
                h: *mut ::std::os::raw::c_int,
                w: *mut ::std::os::raw::c_int,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnGetPooling2dForwardOutputDim = val;
        self
    }
    pub fn cudnnDestroyPoolingDescriptor(
        mut self,
        val: Option<unsafe extern "C" fn(poolingDesc: cudnnPoolingDescriptor_t) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnDestroyPoolingDescriptor = val;
        self
    }
    pub fn cudnnPoolingForward(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                poolingDesc: cudnnPoolingDescriptor_t,
                alpha: *const ::std::os::raw::c_void,
                xDesc: cudnnTensorDescriptor_t,
                x: *const ::std::os::raw::c_void,
                beta: *const ::std::os::raw::c_void,
                yDesc: cudnnTensorDescriptor_t,
                y: *mut ::std::os::raw::c_void,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnPoolingForward = val;
        self
    }
    pub fn cudnnCreateActivationDescriptor(
        mut self,
        val: Option<unsafe extern "C" fn(activationDesc: *mut cudnnActivationDescriptor_t) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnCreateActivationDescriptor = val;
        self
    }
    pub fn cudnnSetActivationDescriptor(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                activationDesc: cudnnActivationDescriptor_t,
                mode: cudnnActivationMode_t,
                reluNanOpt: cudnnNanPropagation_t,
                coef: f64,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnSetActivationDescriptor = val;
        self
    }
    pub fn cudnnGetActivationDescriptor(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                activationDesc: cudnnActivationDescriptor_t,
                mode: *mut cudnnActivationMode_t,
                reluNanOpt: *mut cudnnNanPropagation_t,
                coef: *mut f64,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnGetActivationDescriptor = val;
        self
    }
    pub fn cudnnSetActivationDescriptorSwishBeta(
        mut self,
        val: Option<
            unsafe extern "C" fn(activationDesc: cudnnActivationDescriptor_t, swish_beta: f64) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnSetActivationDescriptorSwishBeta = val;
        self
    }
    pub fn cudnnGetActivationDescriptorSwishBeta(
        mut self,
        val: Option<
            unsafe extern "C" fn(activationDesc: cudnnActivationDescriptor_t, swish_beta: *mut f64) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnGetActivationDescriptorSwishBeta = val;
        self
    }
    pub fn cudnnDestroyActivationDescriptor(
        mut self,
        val: Option<unsafe extern "C" fn(activationDesc: cudnnActivationDescriptor_t) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnDestroyActivationDescriptor = val;
        self
    }
    pub fn cudnnActivationForward(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                activationDesc: cudnnActivationDescriptor_t,
                alpha: *const ::std::os::raw::c_void,
                xDesc: cudnnTensorDescriptor_t,
                x: *const ::std::os::raw::c_void,
                beta: *const ::std::os::raw::c_void,
                yDesc: cudnnTensorDescriptor_t,
                y: *mut ::std::os::raw::c_void,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnActivationForward = val;
        self
    }
    pub fn cudnnCreateLRNDescriptor(
        mut self,
        val: Option<unsafe extern "C" fn(normDesc: *mut cudnnLRNDescriptor_t) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnCreateLRNDescriptor = val;
        self
    }
    pub fn cudnnSetLRNDescriptor(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                normDesc: cudnnLRNDescriptor_t,
                lrnN: ::std::os::raw::c_uint,
                lrnAlpha: f64,
                lrnBeta: f64,
                lrnK: f64,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnSetLRNDescriptor = val;
        self
    }
    pub fn cudnnGetLRNDescriptor(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                normDesc: cudnnLRNDescriptor_t,
                lrnN: *mut ::std::os::raw::c_uint,
                lrnAlpha: *mut f64,
                lrnBeta: *mut f64,
                lrnK: *mut f64,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnGetLRNDescriptor = val;
        self
    }
    pub fn cudnnDestroyLRNDescriptor(
        mut self,
        val: Option<unsafe extern "C" fn(lrnDesc: cudnnLRNDescriptor_t) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnDestroyLRNDescriptor = val;
        self
    }
    pub fn cudnnLRNCrossChannelForward(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                normDesc: cudnnLRNDescriptor_t,
                lrnMode: cudnnLRNMode_t,
                alpha: *const ::std::os::raw::c_void,
                xDesc: cudnnTensorDescriptor_t,
                x: *const ::std::os::raw::c_void,
                beta: *const ::std::os::raw::c_void,
                yDesc: cudnnTensorDescriptor_t,
                y: *mut ::std::os::raw::c_void,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnLRNCrossChannelForward = val;
        self
    }
    pub fn cudnnDivisiveNormalizationForward(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                normDesc: cudnnLRNDescriptor_t,
                mode: cudnnDivNormMode_t,
                alpha: *const ::std::os::raw::c_void,
                xDesc: cudnnTensorDescriptor_t,
                x: *const ::std::os::raw::c_void,
                means: *const ::std::os::raw::c_void,
                temp: *mut ::std::os::raw::c_void,
                temp2: *mut ::std::os::raw::c_void,
                beta: *const ::std::os::raw::c_void,
                yDesc: cudnnTensorDescriptor_t,
                y: *mut ::std::os::raw::c_void,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnDivisiveNormalizationForward = val;
        self
    }
    pub fn cudnnDeriveBNTensorDescriptor(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                derivedBnDesc: cudnnTensorDescriptor_t,
                xDesc: cudnnTensorDescriptor_t,
                mode: cudnnBatchNormMode_t,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnDeriveBNTensorDescriptor = val;
        self
    }
    pub fn cudnnBatchNormalizationForwardInference(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                mode: cudnnBatchNormMode_t,
                alpha: *const ::std::os::raw::c_void,
                beta: *const ::std::os::raw::c_void,
                xDesc: cudnnTensorDescriptor_t,
                x: *const ::std::os::raw::c_void,
                yDesc: cudnnTensorDescriptor_t,
                y: *mut ::std::os::raw::c_void,
                bnScaleBiasMeanVarDesc: cudnnTensorDescriptor_t,
                bnScale: *const ::std::os::raw::c_void,
                bnBias: *const ::std::os::raw::c_void,
                estimatedMean: *const ::std::os::raw::c_void,
                estimatedVariance: *const ::std::os::raw::c_void,
                epsilon: f64,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnBatchNormalizationForwardInference = val;
        self
    }
    pub fn cudnnDeriveNormTensorDescriptor(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                derivedNormScaleBiasDesc: cudnnTensorDescriptor_t,
                derivedNormMeanVarDesc: cudnnTensorDescriptor_t,
                xDesc: cudnnTensorDescriptor_t,
                mode: cudnnNormMode_t,
                groupCnt: ::std::os::raw::c_int,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnDeriveNormTensorDescriptor = val;
        self
    }
    pub fn cudnnNormalizationForwardInference(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                mode: cudnnNormMode_t,
                normOps: cudnnNormOps_t,
                algo: cudnnNormAlgo_t,
                alpha: *const ::std::os::raw::c_void,
                beta: *const ::std::os::raw::c_void,
                xDesc: cudnnTensorDescriptor_t,
                x: *const ::std::os::raw::c_void,
                normScaleBiasDesc: cudnnTensorDescriptor_t,
                normScale: *const ::std::os::raw::c_void,
                normBias: *const ::std::os::raw::c_void,
                normMeanVarDesc: cudnnTensorDescriptor_t,
                estimatedMean: *const ::std::os::raw::c_void,
                estimatedVariance: *const ::std::os::raw::c_void,
                zDesc: cudnnTensorDescriptor_t,
                z: *const ::std::os::raw::c_void,
                activationDesc: cudnnActivationDescriptor_t,
                yDesc: cudnnTensorDescriptor_t,
                y: *mut ::std::os::raw::c_void,
                epsilon: f64,
                groupCnt: ::std::os::raw::c_int,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnNormalizationForwardInference = val;
        self
    }
    pub fn cudnnCreateSpatialTransformerDescriptor(
        mut self,
        val: Option<unsafe extern "C" fn(stDesc: *mut cudnnSpatialTransformerDescriptor_t) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnCreateSpatialTransformerDescriptor = val;
        self
    }
    pub fn cudnnSetSpatialTransformerNdDescriptor(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                stDesc: cudnnSpatialTransformerDescriptor_t,
                samplerType: cudnnSamplerType_t,
                dataType: cudnnDataType_t,
                nbDims: ::std::os::raw::c_int,
                dimA: *const ::std::os::raw::c_int,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnSetSpatialTransformerNdDescriptor = val;
        self
    }
    pub fn cudnnDestroySpatialTransformerDescriptor(
        mut self,
        val: Option<unsafe extern "C" fn(stDesc: cudnnSpatialTransformerDescriptor_t) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnDestroySpatialTransformerDescriptor = val;
        self
    }
    pub fn cudnnSpatialTfGridGeneratorForward(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                stDesc: cudnnSpatialTransformerDescriptor_t,
                theta: *const ::std::os::raw::c_void,
                grid: *mut ::std::os::raw::c_void,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnSpatialTfGridGeneratorForward = val;
        self
    }
    pub fn cudnnSpatialTfSamplerForward(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                stDesc: cudnnSpatialTransformerDescriptor_t,
                alpha: *const ::std::os::raw::c_void,
                xDesc: cudnnTensorDescriptor_t,
                x: *const ::std::os::raw::c_void,
                grid: *const ::std::os::raw::c_void,
                beta: *const ::std::os::raw::c_void,
                yDesc: cudnnTensorDescriptor_t,
                y: *mut ::std::os::raw::c_void,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnSpatialTfSamplerForward = val;
        self
    }
    pub fn cudnnCreateDropoutDescriptor(
        mut self,
        val: Option<unsafe extern "C" fn(dropoutDesc: *mut cudnnDropoutDescriptor_t) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnCreateDropoutDescriptor = val;
        self
    }
    pub fn cudnnDestroyDropoutDescriptor(
        mut self,
        val: Option<unsafe extern "C" fn(dropoutDesc: cudnnDropoutDescriptor_t) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnDestroyDropoutDescriptor = val;
        self
    }
    pub fn cudnnDropoutGetStatesSize(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cudnnHandle_t, sizeInBytes: *mut usize) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnDropoutGetStatesSize = val;
        self
    }
    pub fn cudnnDropoutGetReserveSpaceSize(
        mut self,
        val: Option<unsafe extern "C" fn(xdesc: cudnnTensorDescriptor_t, sizeInBytes: *mut usize) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnDropoutGetReserveSpaceSize = val;
        self
    }
    pub fn cudnnSetDropoutDescriptor(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                dropoutDesc: cudnnDropoutDescriptor_t,
                handle: cudnnHandle_t,
                dropout: f32,
                states: *mut ::std::os::raw::c_void,
                stateSizeInBytes: usize,
                seed: ::std::os::raw::c_ulonglong,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnSetDropoutDescriptor = val;
        self
    }
    pub fn cudnnRestoreDropoutDescriptor(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                dropoutDesc: cudnnDropoutDescriptor_t,
                handle: cudnnHandle_t,
                dropout: f32,
                states: *mut ::std::os::raw::c_void,
                stateSizeInBytes: usize,
                seed: ::std::os::raw::c_ulonglong,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnRestoreDropoutDescriptor = val;
        self
    }
    pub fn cudnnGetDropoutDescriptor(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                dropoutDesc: cudnnDropoutDescriptor_t,
                handle: cudnnHandle_t,
                dropout: *mut f32,
                states: *mut *mut ::std::os::raw::c_void,
                seed: *mut ::std::os::raw::c_ulonglong,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnGetDropoutDescriptor = val;
        self
    }
    pub fn cudnnDropoutForward(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                dropoutDesc: cudnnDropoutDescriptor_t,
                xdesc: cudnnTensorDescriptor_t,
                x: *const ::std::os::raw::c_void,
                ydesc: cudnnTensorDescriptor_t,
                y: *mut ::std::os::raw::c_void,
                reserveSpace: *mut ::std::os::raw::c_void,
                reserveSpaceSizeInBytes: usize,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnDropoutForward = val;
        self
    }
    pub fn cudnnOpsVersionCheck(mut self, val: Option<unsafe extern "C" fn() -> cudnnStatus_t>) -> Self {
        self.cudnnOpsVersionCheck = val;
        self
    }
    pub fn cudnnSoftmaxBackward(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                algo: cudnnSoftmaxAlgorithm_t,
                mode: cudnnSoftmaxMode_t,
                alpha: *const ::std::os::raw::c_void,
                yDesc: cudnnTensorDescriptor_t,
                y: *const ::std::os::raw::c_void,
                dyDesc: cudnnTensorDescriptor_t,
                dy: *const ::std::os::raw::c_void,
                beta: *const ::std::os::raw::c_void,
                dxDesc: cudnnTensorDescriptor_t,
                dx: *mut ::std::os::raw::c_void,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnSoftmaxBackward = val;
        self
    }
    pub fn cudnnPoolingBackward(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                poolingDesc: cudnnPoolingDescriptor_t,
                alpha: *const ::std::os::raw::c_void,
                yDesc: cudnnTensorDescriptor_t,
                y: *const ::std::os::raw::c_void,
                dyDesc: cudnnTensorDescriptor_t,
                dy: *const ::std::os::raw::c_void,
                xDesc: cudnnTensorDescriptor_t,
                x: *const ::std::os::raw::c_void,
                beta: *const ::std::os::raw::c_void,
                dxDesc: cudnnTensorDescriptor_t,
                dx: *mut ::std::os::raw::c_void,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnPoolingBackward = val;
        self
    }
    pub fn cudnnActivationBackward(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                activationDesc: cudnnActivationDescriptor_t,
                alpha: *const ::std::os::raw::c_void,
                yDesc: cudnnTensorDescriptor_t,
                y: *const ::std::os::raw::c_void,
                dyDesc: cudnnTensorDescriptor_t,
                dy: *const ::std::os::raw::c_void,
                xDesc: cudnnTensorDescriptor_t,
                x: *const ::std::os::raw::c_void,
                beta: *const ::std::os::raw::c_void,
                dxDesc: cudnnTensorDescriptor_t,
                dx: *mut ::std::os::raw::c_void,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnActivationBackward = val;
        self
    }
    pub fn cudnnLRNCrossChannelBackward(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                normDesc: cudnnLRNDescriptor_t,
                lrnMode: cudnnLRNMode_t,
                alpha: *const ::std::os::raw::c_void,
                yDesc: cudnnTensorDescriptor_t,
                y: *const ::std::os::raw::c_void,
                dyDesc: cudnnTensorDescriptor_t,
                dy: *const ::std::os::raw::c_void,
                xDesc: cudnnTensorDescriptor_t,
                x: *const ::std::os::raw::c_void,
                beta: *const ::std::os::raw::c_void,
                dxDesc: cudnnTensorDescriptor_t,
                dx: *mut ::std::os::raw::c_void,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnLRNCrossChannelBackward = val;
        self
    }
    pub fn cudnnDivisiveNormalizationBackward(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                normDesc: cudnnLRNDescriptor_t,
                mode: cudnnDivNormMode_t,
                alpha: *const ::std::os::raw::c_void,
                xDesc: cudnnTensorDescriptor_t,
                x: *const ::std::os::raw::c_void,
                means: *const ::std::os::raw::c_void,
                dy: *const ::std::os::raw::c_void,
                temp: *mut ::std::os::raw::c_void,
                temp2: *mut ::std::os::raw::c_void,
                beta: *const ::std::os::raw::c_void,
                dXdMeansDesc: cudnnTensorDescriptor_t,
                dx: *mut ::std::os::raw::c_void,
                dMeans: *mut ::std::os::raw::c_void,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnDivisiveNormalizationBackward = val;
        self
    }
    pub fn cudnnGetBatchNormalizationForwardTrainingExWorkspaceSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                mode: cudnnBatchNormMode_t,
                bnOps: cudnnBatchNormOps_t,
                xDesc: cudnnTensorDescriptor_t,
                zDesc: cudnnTensorDescriptor_t,
                yDesc: cudnnTensorDescriptor_t,
                bnScaleBiasMeanVarDesc: cudnnTensorDescriptor_t,
                activationDesc: cudnnActivationDescriptor_t,
                sizeInBytes: *mut usize,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnGetBatchNormalizationForwardTrainingExWorkspaceSize = val;
        self
    }
    pub fn cudnnGetBatchNormalizationBackwardExWorkspaceSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                mode: cudnnBatchNormMode_t,
                bnOps: cudnnBatchNormOps_t,
                xDesc: cudnnTensorDescriptor_t,
                yDesc: cudnnTensorDescriptor_t,
                dyDesc: cudnnTensorDescriptor_t,
                dzDesc: cudnnTensorDescriptor_t,
                dxDesc: cudnnTensorDescriptor_t,
                dBnScaleBiasDesc: cudnnTensorDescriptor_t,
                activationDesc: cudnnActivationDescriptor_t,
                sizeInBytes: *mut usize,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnGetBatchNormalizationBackwardExWorkspaceSize = val;
        self
    }
    pub fn cudnnGetBatchNormalizationTrainingExReserveSpaceSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                mode: cudnnBatchNormMode_t,
                bnOps: cudnnBatchNormOps_t,
                activationDesc: cudnnActivationDescriptor_t,
                xDesc: cudnnTensorDescriptor_t,
                sizeInBytes: *mut usize,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnGetBatchNormalizationTrainingExReserveSpaceSize = val;
        self
    }
    pub fn cudnnBatchNormalizationForwardTraining(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                mode: cudnnBatchNormMode_t,
                alpha: *const ::std::os::raw::c_void,
                beta: *const ::std::os::raw::c_void,
                xDesc: cudnnTensorDescriptor_t,
                x: *const ::std::os::raw::c_void,
                yDesc: cudnnTensorDescriptor_t,
                y: *mut ::std::os::raw::c_void,
                bnScaleBiasMeanVarDesc: cudnnTensorDescriptor_t,
                bnScale: *const ::std::os::raw::c_void,
                bnBias: *const ::std::os::raw::c_void,
                exponentialAverageFactor: f64,
                resultRunningMean: *mut ::std::os::raw::c_void,
                resultRunningVariance: *mut ::std::os::raw::c_void,
                epsilon: f64,
                resultSaveMean: *mut ::std::os::raw::c_void,
                resultSaveInvVariance: *mut ::std::os::raw::c_void,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnBatchNormalizationForwardTraining = val;
        self
    }
    pub fn cudnnBatchNormalizationForwardTrainingEx(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                mode: cudnnBatchNormMode_t,
                bnOps: cudnnBatchNormOps_t,
                alpha: *const ::std::os::raw::c_void,
                beta: *const ::std::os::raw::c_void,
                xDesc: cudnnTensorDescriptor_t,
                xData: *const ::std::os::raw::c_void,
                zDesc: cudnnTensorDescriptor_t,
                zData: *const ::std::os::raw::c_void,
                yDesc: cudnnTensorDescriptor_t,
                yData: *mut ::std::os::raw::c_void,
                bnScaleBiasMeanVarDesc: cudnnTensorDescriptor_t,
                bnScale: *const ::std::os::raw::c_void,
                bnBias: *const ::std::os::raw::c_void,
                exponentialAverageFactor: f64,
                resultRunningMean: *mut ::std::os::raw::c_void,
                resultRunningVariance: *mut ::std::os::raw::c_void,
                epsilon: f64,
                resultSaveMean: *mut ::std::os::raw::c_void,
                resultSaveInvVariance: *mut ::std::os::raw::c_void,
                activationDesc: cudnnActivationDescriptor_t,
                workspace: *mut ::std::os::raw::c_void,
                workSpaceSizeInBytes: usize,
                reserveSpace: *mut ::std::os::raw::c_void,
                reserveSpaceSizeInBytes: usize,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnBatchNormalizationForwardTrainingEx = val;
        self
    }
    pub fn cudnnBatchNormalizationBackward(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                mode: cudnnBatchNormMode_t,
                alphaDataDiff: *const ::std::os::raw::c_void,
                betaDataDiff: *const ::std::os::raw::c_void,
                alphaParamDiff: *const ::std::os::raw::c_void,
                betaParamDiff: *const ::std::os::raw::c_void,
                xDesc: cudnnTensorDescriptor_t,
                x: *const ::std::os::raw::c_void,
                dyDesc: cudnnTensorDescriptor_t,
                dy: *const ::std::os::raw::c_void,
                dxDesc: cudnnTensorDescriptor_t,
                dx: *mut ::std::os::raw::c_void,
                dBnScaleBiasDesc: cudnnTensorDescriptor_t,
                bnScale: *const ::std::os::raw::c_void,
                dBnScaleResult: *mut ::std::os::raw::c_void,
                dBnBiasResult: *mut ::std::os::raw::c_void,
                epsilon: f64,
                savedMean: *const ::std::os::raw::c_void,
                savedInvVariance: *const ::std::os::raw::c_void,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnBatchNormalizationBackward = val;
        self
    }
    pub fn cudnnBatchNormalizationBackwardEx(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                mode: cudnnBatchNormMode_t,
                bnOps: cudnnBatchNormOps_t,
                alphaDataDiff: *const ::std::os::raw::c_void,
                betaDataDiff: *const ::std::os::raw::c_void,
                alphaParamDiff: *const ::std::os::raw::c_void,
                betaParamDiff: *const ::std::os::raw::c_void,
                xDesc: cudnnTensorDescriptor_t,
                xData: *const ::std::os::raw::c_void,
                yDesc: cudnnTensorDescriptor_t,
                yData: *const ::std::os::raw::c_void,
                dyDesc: cudnnTensorDescriptor_t,
                dyData: *const ::std::os::raw::c_void,
                dzDesc: cudnnTensorDescriptor_t,
                dzData: *mut ::std::os::raw::c_void,
                dxDesc: cudnnTensorDescriptor_t,
                dxData: *mut ::std::os::raw::c_void,
                dBnScaleBiasDesc: cudnnTensorDescriptor_t,
                bnScaleData: *const ::std::os::raw::c_void,
                bnBiasData: *const ::std::os::raw::c_void,
                dBnScaleData: *mut ::std::os::raw::c_void,
                dBnBiasData: *mut ::std::os::raw::c_void,
                epsilon: f64,
                savedMean: *const ::std::os::raw::c_void,
                savedInvVariance: *const ::std::os::raw::c_void,
                activationDesc: cudnnActivationDescriptor_t,
                workSpace: *mut ::std::os::raw::c_void,
                workSpaceSizeInBytes: usize,
                reserveSpace: *mut ::std::os::raw::c_void,
                reserveSpaceSizeInBytes: usize,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnBatchNormalizationBackwardEx = val;
        self
    }
    pub fn cudnnGetNormalizationForwardTrainingWorkspaceSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                mode: cudnnNormMode_t,
                normOps: cudnnNormOps_t,
                algo: cudnnNormAlgo_t,
                xDesc: cudnnTensorDescriptor_t,
                zDesc: cudnnTensorDescriptor_t,
                yDesc: cudnnTensorDescriptor_t,
                normScaleBiasDesc: cudnnTensorDescriptor_t,
                activationDesc: cudnnActivationDescriptor_t,
                normMeanVarDesc: cudnnTensorDescriptor_t,
                sizeInBytes: *mut usize,
                groupCnt: ::std::os::raw::c_int,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnGetNormalizationForwardTrainingWorkspaceSize = val;
        self
    }
    pub fn cudnnGetNormalizationBackwardWorkspaceSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
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
                sizeInBytes: *mut usize,
                groupCnt: ::std::os::raw::c_int,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnGetNormalizationBackwardWorkspaceSize = val;
        self
    }
    pub fn cudnnGetNormalizationTrainingReserveSpaceSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                mode: cudnnNormMode_t,
                normOps: cudnnNormOps_t,
                algo: cudnnNormAlgo_t,
                activationDesc: cudnnActivationDescriptor_t,
                xDesc: cudnnTensorDescriptor_t,
                sizeInBytes: *mut usize,
                groupCnt: ::std::os::raw::c_int,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnGetNormalizationTrainingReserveSpaceSize = val;
        self
    }
    pub fn cudnnNormalizationForwardTraining(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                mode: cudnnNormMode_t,
                normOps: cudnnNormOps_t,
                algo: cudnnNormAlgo_t,
                alpha: *const ::std::os::raw::c_void,
                beta: *const ::std::os::raw::c_void,
                xDesc: cudnnTensorDescriptor_t,
                xData: *const ::std::os::raw::c_void,
                normScaleBiasDesc: cudnnTensorDescriptor_t,
                normScale: *const ::std::os::raw::c_void,
                normBias: *const ::std::os::raw::c_void,
                exponentialAverageFactor: f64,
                normMeanVarDesc: cudnnTensorDescriptor_t,
                resultRunningMean: *mut ::std::os::raw::c_void,
                resultRunningVariance: *mut ::std::os::raw::c_void,
                epsilon: f64,
                resultSaveMean: *mut ::std::os::raw::c_void,
                resultSaveInvVariance: *mut ::std::os::raw::c_void,
                activationDesc: cudnnActivationDescriptor_t,
                zDesc: cudnnTensorDescriptor_t,
                zData: *const ::std::os::raw::c_void,
                yDesc: cudnnTensorDescriptor_t,
                yData: *mut ::std::os::raw::c_void,
                workspace: *mut ::std::os::raw::c_void,
                workSpaceSizeInBytes: usize,
                reserveSpace: *mut ::std::os::raw::c_void,
                reserveSpaceSizeInBytes: usize,
                groupCnt: ::std::os::raw::c_int,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnNormalizationForwardTraining = val;
        self
    }
    pub fn cudnnNormalizationBackward(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                mode: cudnnNormMode_t,
                normOps: cudnnNormOps_t,
                algo: cudnnNormAlgo_t,
                alphaDataDiff: *const ::std::os::raw::c_void,
                betaDataDiff: *const ::std::os::raw::c_void,
                alphaParamDiff: *const ::std::os::raw::c_void,
                betaParamDiff: *const ::std::os::raw::c_void,
                xDesc: cudnnTensorDescriptor_t,
                xData: *const ::std::os::raw::c_void,
                yDesc: cudnnTensorDescriptor_t,
                yData: *const ::std::os::raw::c_void,
                dyDesc: cudnnTensorDescriptor_t,
                dyData: *const ::std::os::raw::c_void,
                dzDesc: cudnnTensorDescriptor_t,
                dzData: *mut ::std::os::raw::c_void,
                dxDesc: cudnnTensorDescriptor_t,
                dxData: *mut ::std::os::raw::c_void,
                dNormScaleBiasDesc: cudnnTensorDescriptor_t,
                normScaleData: *const ::std::os::raw::c_void,
                normBiasData: *const ::std::os::raw::c_void,
                dNormScaleData: *mut ::std::os::raw::c_void,
                dNormBiasData: *mut ::std::os::raw::c_void,
                epsilon: f64,
                normMeanVarDesc: cudnnTensorDescriptor_t,
                savedMean: *const ::std::os::raw::c_void,
                savedInvVariance: *const ::std::os::raw::c_void,
                activationDesc: cudnnActivationDescriptor_t,
                workSpace: *mut ::std::os::raw::c_void,
                workSpaceSizeInBytes: usize,
                reserveSpace: *mut ::std::os::raw::c_void,
                reserveSpaceSizeInBytes: usize,
                groupCnt: ::std::os::raw::c_int,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnNormalizationBackward = val;
        self
    }
    pub fn cudnnSpatialTfGridGeneratorBackward(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                stDesc: cudnnSpatialTransformerDescriptor_t,
                dgrid: *const ::std::os::raw::c_void,
                dtheta: *mut ::std::os::raw::c_void,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnSpatialTfGridGeneratorBackward = val;
        self
    }
    pub fn cudnnSpatialTfSamplerBackward(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                stDesc: cudnnSpatialTransformerDescriptor_t,
                alpha: *const ::std::os::raw::c_void,
                xDesc: cudnnTensorDescriptor_t,
                x: *const ::std::os::raw::c_void,
                beta: *const ::std::os::raw::c_void,
                dxDesc: cudnnTensorDescriptor_t,
                dx: *mut ::std::os::raw::c_void,
                alphaDgrid: *const ::std::os::raw::c_void,
                dyDesc: cudnnTensorDescriptor_t,
                dy: *const ::std::os::raw::c_void,
                grid: *const ::std::os::raw::c_void,
                betaDgrid: *const ::std::os::raw::c_void,
                dgrid: *mut ::std::os::raw::c_void,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnSpatialTfSamplerBackward = val;
        self
    }
    pub fn cudnnDropoutBackward(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                dropoutDesc: cudnnDropoutDescriptor_t,
                dydesc: cudnnTensorDescriptor_t,
                dy: *const ::std::os::raw::c_void,
                dxdesc: cudnnTensorDescriptor_t,
                dx: *mut ::std::os::raw::c_void,
                reserveSpace: *mut ::std::os::raw::c_void,
                reserveSpaceSizeInBytes: usize,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnDropoutBackward = val;
        self
    }
    pub fn cudnnCreateRNNDescriptor(
        mut self,
        val: Option<unsafe extern "C" fn(rnnDesc: *mut cudnnRNNDescriptor_t) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnCreateRNNDescriptor = val;
        self
    }
    pub fn cudnnDestroyRNNDescriptor(
        mut self,
        val: Option<unsafe extern "C" fn(rnnDesc: cudnnRNNDescriptor_t) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnDestroyRNNDescriptor = val;
        self
    }
    pub fn cudnnSetRNNDescriptor_v8(
        mut self,
        val: Option<
            unsafe extern "C" fn(
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
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnSetRNNDescriptor_v8 = val;
        self
    }
    pub fn cudnnGetRNNDescriptor_v8(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                rnnDesc: cudnnRNNDescriptor_t,
                algo: *mut cudnnRNNAlgo_t,
                cellMode: *mut cudnnRNNMode_t,
                biasMode: *mut cudnnRNNBiasMode_t,
                dirMode: *mut cudnnDirectionMode_t,
                inputMode: *mut cudnnRNNInputMode_t,
                dataType: *mut cudnnDataType_t,
                mathPrec: *mut cudnnDataType_t,
                mathType: *mut cudnnMathType_t,
                inputSize: *mut i32,
                hiddenSize: *mut i32,
                projSize: *mut i32,
                numLayers: *mut i32,
                dropoutDesc: *mut cudnnDropoutDescriptor_t,
                auxFlags: *mut u32,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnGetRNNDescriptor_v8 = val;
        self
    }
    pub fn cudnnRNNSetClip_v8(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                rnnDesc: cudnnRNNDescriptor_t,
                clipMode: cudnnRNNClipMode_t,
                clipNanOpt: cudnnNanPropagation_t,
                lclip: f64,
                rclip: f64,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnRNNSetClip_v8 = val;
        self
    }
    pub fn cudnnRNNSetClip_v9(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                rnnDesc: cudnnRNNDescriptor_t,
                clipMode: cudnnRNNClipMode_t,
                lclip: f64,
                rclip: f64,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnRNNSetClip_v9 = val;
        self
    }
    pub fn cudnnRNNGetClip_v8(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                rnnDesc: cudnnRNNDescriptor_t,
                clipMode: *mut cudnnRNNClipMode_t,
                clipNanOpt: *mut cudnnNanPropagation_t,
                lclip: *mut f64,
                rclip: *mut f64,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnRNNGetClip_v8 = val;
        self
    }
    pub fn cudnnRNNGetClip_v9(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                rnnDesc: cudnnRNNDescriptor_t,
                clipMode: *mut cudnnRNNClipMode_t,
                lclip: *mut f64,
                rclip: *mut f64,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnRNNGetClip_v9 = val;
        self
    }
    pub fn cudnnBuildRNNDynamic(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                rnnDesc: cudnnRNNDescriptor_t,
                miniBatch: ::std::os::raw::c_int,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnBuildRNNDynamic = val;
        self
    }
    pub fn cudnnGetRNNTempSpaceSizes(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                rnnDesc: cudnnRNNDescriptor_t,
                fwdMode: cudnnForwardMode_t,
                xDesc: cudnnRNNDataDescriptor_t,
                workSpaceSize: *mut usize,
                reserveSpaceSize: *mut usize,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnGetRNNTempSpaceSizes = val;
        self
    }
    pub fn cudnnGetRNNWeightSpaceSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                rnnDesc: cudnnRNNDescriptor_t,
                weightSpaceSize: *mut usize,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnGetRNNWeightSpaceSize = val;
        self
    }
    pub fn cudnnGetRNNWeightParams(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                rnnDesc: cudnnRNNDescriptor_t,
                pseudoLayer: i32,
                weightSpaceSize: usize,
                weightSpace: *const ::std::os::raw::c_void,
                linLayerID: i32,
                mDesc: cudnnTensorDescriptor_t,
                mAddr: *mut *mut ::std::os::raw::c_void,
                bDesc: cudnnTensorDescriptor_t,
                bAddr: *mut *mut ::std::os::raw::c_void,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnGetRNNWeightParams = val;
        self
    }
    pub fn cudnnCreateRNNDataDescriptor(
        mut self,
        val: Option<unsafe extern "C" fn(rnnDataDesc: *mut cudnnRNNDataDescriptor_t) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnCreateRNNDataDescriptor = val;
        self
    }
    pub fn cudnnDestroyRNNDataDescriptor(
        mut self,
        val: Option<unsafe extern "C" fn(rnnDataDesc: cudnnRNNDataDescriptor_t) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnDestroyRNNDataDescriptor = val;
        self
    }
    pub fn cudnnSetRNNDataDescriptor(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                rnnDataDesc: cudnnRNNDataDescriptor_t,
                dataType: cudnnDataType_t,
                layout: cudnnRNNDataLayout_t,
                maxSeqLength: ::std::os::raw::c_int,
                batchSize: ::std::os::raw::c_int,
                vectorSize: ::std::os::raw::c_int,
                seqLengthArray: *const ::std::os::raw::c_int,
                paddingFill: *mut ::std::os::raw::c_void,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnSetRNNDataDescriptor = val;
        self
    }
    pub fn cudnnGetRNNDataDescriptor(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                rnnDataDesc: cudnnRNNDataDescriptor_t,
                dataType: *mut cudnnDataType_t,
                layout: *mut cudnnRNNDataLayout_t,
                maxSeqLength: *mut ::std::os::raw::c_int,
                batchSize: *mut ::std::os::raw::c_int,
                vectorSize: *mut ::std::os::raw::c_int,
                arrayLengthRequested: ::std::os::raw::c_int,
                seqLengthArray: *mut ::std::os::raw::c_int,
                paddingFill: *mut ::std::os::raw::c_void,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnGetRNNDataDescriptor = val;
        self
    }
    pub fn cudnnRNNForward(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                rnnDesc: cudnnRNNDescriptor_t,
                fwdMode: cudnnForwardMode_t,
                devSeqLengths: *const i32,
                xDesc: cudnnRNNDataDescriptor_t,
                x: *const ::std::os::raw::c_void,
                yDesc: cudnnRNNDataDescriptor_t,
                y: *mut ::std::os::raw::c_void,
                hDesc: cudnnTensorDescriptor_t,
                hx: *const ::std::os::raw::c_void,
                hy: *mut ::std::os::raw::c_void,
                cDesc: cudnnTensorDescriptor_t,
                cx: *const ::std::os::raw::c_void,
                cy: *mut ::std::os::raw::c_void,
                weightSpaceSize: usize,
                weightSpace: *const ::std::os::raw::c_void,
                workSpaceSize: usize,
                workSpace: *mut ::std::os::raw::c_void,
                reserveSpaceSize: usize,
                reserveSpace: *mut ::std::os::raw::c_void,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnRNNForward = val;
        self
    }
    pub fn cudnnCreateSeqDataDescriptor(
        mut self,
        val: Option<unsafe extern "C" fn(seqDataDesc: *mut cudnnSeqDataDescriptor_t) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnCreateSeqDataDescriptor = val;
        self
    }
    pub fn cudnnDestroySeqDataDescriptor(
        mut self,
        val: Option<unsafe extern "C" fn(seqDataDesc: cudnnSeqDataDescriptor_t) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnDestroySeqDataDescriptor = val;
        self
    }
    pub fn cudnnSetSeqDataDescriptor(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                seqDataDesc: cudnnSeqDataDescriptor_t,
                dataType: cudnnDataType_t,
                nbDims: ::std::os::raw::c_int,
                dimA: *const ::std::os::raw::c_int,
                axes: *const cudnnSeqDataAxis_t,
                seqLengthArraySize: usize,
                seqLengthArray: *const ::std::os::raw::c_int,
                paddingFill: *mut ::std::os::raw::c_void,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnSetSeqDataDescriptor = val;
        self
    }
    pub fn cudnnGetSeqDataDescriptor(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                seqDataDesc: cudnnSeqDataDescriptor_t,
                dataType: *mut cudnnDataType_t,
                nbDims: *mut ::std::os::raw::c_int,
                nbDimsRequested: ::std::os::raw::c_int,
                dimA: *mut ::std::os::raw::c_int,
                axes: *mut cudnnSeqDataAxis_t,
                seqLengthArraySize: *mut usize,
                seqLengthSizeRequested: usize,
                seqLengthArray: *mut ::std::os::raw::c_int,
                paddingFill: *mut ::std::os::raw::c_void,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnGetSeqDataDescriptor = val;
        self
    }
    pub fn cudnnCreateAttnDescriptor(
        mut self,
        val: Option<unsafe extern "C" fn(attnDesc: *mut cudnnAttnDescriptor_t) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnCreateAttnDescriptor = val;
        self
    }
    pub fn cudnnDestroyAttnDescriptor(
        mut self,
        val: Option<unsafe extern "C" fn(attnDesc: cudnnAttnDescriptor_t) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnDestroyAttnDescriptor = val;
        self
    }
    pub fn cudnnSetAttnDescriptor(
        mut self,
        val: Option<
            unsafe extern "C" fn(
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
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnSetAttnDescriptor = val;
        self
    }
    pub fn cudnnGetAttnDescriptor(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                attnDesc: cudnnAttnDescriptor_t,
                attnMode: *mut ::std::os::raw::c_uint,
                nHeads: *mut ::std::os::raw::c_int,
                smScaler: *mut f64,
                dataType: *mut cudnnDataType_t,
                computePrec: *mut cudnnDataType_t,
                mathType: *mut cudnnMathType_t,
                attnDropoutDesc: *mut cudnnDropoutDescriptor_t,
                postDropoutDesc: *mut cudnnDropoutDescriptor_t,
                qSize: *mut ::std::os::raw::c_int,
                kSize: *mut ::std::os::raw::c_int,
                vSize: *mut ::std::os::raw::c_int,
                qProjSize: *mut ::std::os::raw::c_int,
                kProjSize: *mut ::std::os::raw::c_int,
                vProjSize: *mut ::std::os::raw::c_int,
                oProjSize: *mut ::std::os::raw::c_int,
                qoMaxSeqLength: *mut ::std::os::raw::c_int,
                kvMaxSeqLength: *mut ::std::os::raw::c_int,
                maxBatchSize: *mut ::std::os::raw::c_int,
                maxBeamSize: *mut ::std::os::raw::c_int,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnGetAttnDescriptor = val;
        self
    }
    pub fn cudnnGetMultiHeadAttnBuffers(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                attnDesc: cudnnAttnDescriptor_t,
                weightSizeInBytes: *mut usize,
                workSpaceSizeInBytes: *mut usize,
                reserveSpaceSizeInBytes: *mut usize,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnGetMultiHeadAttnBuffers = val;
        self
    }
    pub fn cudnnGetMultiHeadAttnWeights(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                attnDesc: cudnnAttnDescriptor_t,
                wKind: cudnnMultiHeadAttnWeightKind_t,
                weightSizeInBytes: usize,
                weights: *const ::std::os::raw::c_void,
                wDesc: cudnnTensorDescriptor_t,
                wAddr: *mut *mut ::std::os::raw::c_void,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnGetMultiHeadAttnWeights = val;
        self
    }
    pub fn cudnnMultiHeadAttnForward(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                attnDesc: cudnnAttnDescriptor_t,
                currIdx: ::std::os::raw::c_int,
                loWinIdx: *const ::std::os::raw::c_int,
                hiWinIdx: *const ::std::os::raw::c_int,
                devSeqLengthsQO: *const ::std::os::raw::c_int,
                devSeqLengthsKV: *const ::std::os::raw::c_int,
                qDesc: cudnnSeqDataDescriptor_t,
                queries: *const ::std::os::raw::c_void,
                residuals: *const ::std::os::raw::c_void,
                kDesc: cudnnSeqDataDescriptor_t,
                keys: *const ::std::os::raw::c_void,
                vDesc: cudnnSeqDataDescriptor_t,
                values: *const ::std::os::raw::c_void,
                oDesc: cudnnSeqDataDescriptor_t,
                out: *mut ::std::os::raw::c_void,
                weightSizeInBytes: usize,
                weights: *const ::std::os::raw::c_void,
                workSpaceSizeInBytes: usize,
                workSpace: *mut ::std::os::raw::c_void,
                reserveSpaceSizeInBytes: usize,
                reserveSpace: *mut ::std::os::raw::c_void,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnMultiHeadAttnForward = val;
        self
    }
    pub fn cudnnAdvVersionCheck(mut self, val: Option<unsafe extern "C" fn() -> cudnnStatus_t>) -> Self {
        self.cudnnAdvVersionCheck = val;
        self
    }
    pub fn cudnnRNNBackwardData_v8(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                rnnDesc: cudnnRNNDescriptor_t,
                devSeqLengths: *const i32,
                yDesc: cudnnRNNDataDescriptor_t,
                y: *const ::std::os::raw::c_void,
                dy: *const ::std::os::raw::c_void,
                xDesc: cudnnRNNDataDescriptor_t,
                dx: *mut ::std::os::raw::c_void,
                hDesc: cudnnTensorDescriptor_t,
                hx: *const ::std::os::raw::c_void,
                dhy: *const ::std::os::raw::c_void,
                dhx: *mut ::std::os::raw::c_void,
                cDesc: cudnnTensorDescriptor_t,
                cx: *const ::std::os::raw::c_void,
                dcy: *const ::std::os::raw::c_void,
                dcx: *mut ::std::os::raw::c_void,
                weightSpaceSize: usize,
                weightSpace: *const ::std::os::raw::c_void,
                workSpaceSize: usize,
                workSpace: *mut ::std::os::raw::c_void,
                reserveSpaceSize: usize,
                reserveSpace: *mut ::std::os::raw::c_void,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnRNNBackwardData_v8 = val;
        self
    }
    pub fn cudnnRNNBackwardWeights_v8(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                rnnDesc: cudnnRNNDescriptor_t,
                addGrad: cudnnWgradMode_t,
                devSeqLengths: *const i32,
                xDesc: cudnnRNNDataDescriptor_t,
                x: *const ::std::os::raw::c_void,
                hDesc: cudnnTensorDescriptor_t,
                hx: *const ::std::os::raw::c_void,
                yDesc: cudnnRNNDataDescriptor_t,
                y: *const ::std::os::raw::c_void,
                weightSpaceSize: usize,
                dweightSpace: *mut ::std::os::raw::c_void,
                workSpaceSize: usize,
                workSpace: *mut ::std::os::raw::c_void,
                reserveSpaceSize: usize,
                reserveSpace: *mut ::std::os::raw::c_void,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnRNNBackwardWeights_v8 = val;
        self
    }
    pub fn cudnnMultiHeadAttnBackwardData(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                attnDesc: cudnnAttnDescriptor_t,
                loWinIdx: *const ::std::os::raw::c_int,
                hiWinIdx: *const ::std::os::raw::c_int,
                devSeqLengthsDQDO: *const ::std::os::raw::c_int,
                devSeqLengthsDKDV: *const ::std::os::raw::c_int,
                doDesc: cudnnSeqDataDescriptor_t,
                dout: *const ::std::os::raw::c_void,
                dqDesc: cudnnSeqDataDescriptor_t,
                dqueries: *mut ::std::os::raw::c_void,
                queries: *const ::std::os::raw::c_void,
                dkDesc: cudnnSeqDataDescriptor_t,
                dkeys: *mut ::std::os::raw::c_void,
                keys: *const ::std::os::raw::c_void,
                dvDesc: cudnnSeqDataDescriptor_t,
                dvalues: *mut ::std::os::raw::c_void,
                values: *const ::std::os::raw::c_void,
                weightSizeInBytes: usize,
                weights: *const ::std::os::raw::c_void,
                workSpaceSizeInBytes: usize,
                workSpace: *mut ::std::os::raw::c_void,
                reserveSpaceSizeInBytes: usize,
                reserveSpace: *mut ::std::os::raw::c_void,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnMultiHeadAttnBackwardData = val;
        self
    }
    pub fn cudnnMultiHeadAttnBackwardWeights(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                attnDesc: cudnnAttnDescriptor_t,
                addGrad: cudnnWgradMode_t,
                qDesc: cudnnSeqDataDescriptor_t,
                queries: *const ::std::os::raw::c_void,
                kDesc: cudnnSeqDataDescriptor_t,
                keys: *const ::std::os::raw::c_void,
                vDesc: cudnnSeqDataDescriptor_t,
                values: *const ::std::os::raw::c_void,
                doDesc: cudnnSeqDataDescriptor_t,
                dout: *const ::std::os::raw::c_void,
                weightSizeInBytes: usize,
                weights: *const ::std::os::raw::c_void,
                dweights: *mut ::std::os::raw::c_void,
                workSpaceSizeInBytes: usize,
                workSpace: *mut ::std::os::raw::c_void,
                reserveSpaceSizeInBytes: usize,
                reserveSpace: *mut ::std::os::raw::c_void,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnMultiHeadAttnBackwardWeights = val;
        self
    }
    pub fn cudnnCreateCTCLossDescriptor(
        mut self,
        val: Option<unsafe extern "C" fn(ctcLossDesc: *mut cudnnCTCLossDescriptor_t) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnCreateCTCLossDescriptor = val;
        self
    }
    pub fn cudnnSetCTCLossDescriptor(
        mut self,
        val: Option<
            unsafe extern "C" fn(ctcLossDesc: cudnnCTCLossDescriptor_t, compType: cudnnDataType_t) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnSetCTCLossDescriptor = val;
        self
    }
    pub fn cudnnSetCTCLossDescriptorEx(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                ctcLossDesc: cudnnCTCLossDescriptor_t,
                compType: cudnnDataType_t,
                normMode: cudnnLossNormalizationMode_t,
                gradMode: cudnnNanPropagation_t,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnSetCTCLossDescriptorEx = val;
        self
    }
    pub fn cudnnSetCTCLossDescriptor_v8(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                ctcLossDesc: cudnnCTCLossDescriptor_t,
                compType: cudnnDataType_t,
                normMode: cudnnLossNormalizationMode_t,
                gradMode: cudnnNanPropagation_t,
                maxLabelLength: ::std::os::raw::c_int,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnSetCTCLossDescriptor_v8 = val;
        self
    }
    pub fn cudnnSetCTCLossDescriptor_v9(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                ctcLossDesc: cudnnCTCLossDescriptor_t,
                compType: cudnnDataType_t,
                normMode: cudnnLossNormalizationMode_t,
                ctcGradMode: cudnnCTCGradMode_t,
                maxLabelLength: ::std::os::raw::c_int,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnSetCTCLossDescriptor_v9 = val;
        self
    }
    pub fn cudnnGetCTCLossDescriptor(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                ctcLossDesc: cudnnCTCLossDescriptor_t,
                compType: *mut cudnnDataType_t,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnGetCTCLossDescriptor = val;
        self
    }
    pub fn cudnnGetCTCLossDescriptorEx(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                ctcLossDesc: cudnnCTCLossDescriptor_t,
                compType: *mut cudnnDataType_t,
                normMode: *mut cudnnLossNormalizationMode_t,
                gradMode: *mut cudnnNanPropagation_t,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnGetCTCLossDescriptorEx = val;
        self
    }
    pub fn cudnnGetCTCLossDescriptor_v8(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                ctcLossDesc: cudnnCTCLossDescriptor_t,
                compType: *mut cudnnDataType_t,
                normMode: *mut cudnnLossNormalizationMode_t,
                gradMode: *mut cudnnNanPropagation_t,
                maxLabelLength: *mut ::std::os::raw::c_int,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnGetCTCLossDescriptor_v8 = val;
        self
    }
    pub fn cudnnGetCTCLossDescriptor_v9(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                ctcLossDesc: cudnnCTCLossDescriptor_t,
                compType: *mut cudnnDataType_t,
                normMode: *mut cudnnLossNormalizationMode_t,
                ctcGradMode: *mut cudnnCTCGradMode_t,
                maxLabelLength: *mut ::std::os::raw::c_int,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnGetCTCLossDescriptor_v9 = val;
        self
    }
    pub fn cudnnDestroyCTCLossDescriptor(
        mut self,
        val: Option<unsafe extern "C" fn(ctcLossDesc: cudnnCTCLossDescriptor_t) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnDestroyCTCLossDescriptor = val;
        self
    }
    pub fn cudnnCTCLoss(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                probsDesc: cudnnTensorDescriptor_t,
                probs: *const ::std::os::raw::c_void,
                hostLabels: *const ::std::os::raw::c_int,
                hostLabelLengths: *const ::std::os::raw::c_int,
                hostInputLengths: *const ::std::os::raw::c_int,
                costs: *mut ::std::os::raw::c_void,
                gradientsDesc: cudnnTensorDescriptor_t,
                gradients: *mut ::std::os::raw::c_void,
                algo: cudnnCTCLossAlgo_t,
                ctcLossDesc: cudnnCTCLossDescriptor_t,
                workspace: *mut ::std::os::raw::c_void,
                workSpaceSizeInBytes: usize,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnCTCLoss = val;
        self
    }
    pub fn cudnnCTCLoss_v8(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                algo: cudnnCTCLossAlgo_t,
                ctcLossDesc: cudnnCTCLossDescriptor_t,
                probsDesc: cudnnTensorDescriptor_t,
                probs: *const ::std::os::raw::c_void,
                labels: *const ::std::os::raw::c_int,
                labelLengths: *const ::std::os::raw::c_int,
                inputLengths: *const ::std::os::raw::c_int,
                costs: *mut ::std::os::raw::c_void,
                gradientsDesc: cudnnTensorDescriptor_t,
                gradients: *mut ::std::os::raw::c_void,
                workSpaceSizeInBytes: usize,
                workspace: *mut ::std::os::raw::c_void,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnCTCLoss_v8 = val;
        self
    }
    pub fn cudnnGetCTCLossWorkspaceSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                probsDesc: cudnnTensorDescriptor_t,
                gradientsDesc: cudnnTensorDescriptor_t,
                labels: *const ::std::os::raw::c_int,
                labelLengths: *const ::std::os::raw::c_int,
                inputLengths: *const ::std::os::raw::c_int,
                algo: cudnnCTCLossAlgo_t,
                ctcLossDesc: cudnnCTCLossDescriptor_t,
                sizeInBytes: *mut usize,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnGetCTCLossWorkspaceSize = val;
        self
    }
    pub fn cudnnGetCTCLossWorkspaceSize_v8(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                algo: cudnnCTCLossAlgo_t,
                ctcLossDesc: cudnnCTCLossDescriptor_t,
                probsDesc: cudnnTensorDescriptor_t,
                gradientsDesc: cudnnTensorDescriptor_t,
                sizeInBytes: *mut usize,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnGetCTCLossWorkspaceSize_v8 = val;
        self
    }
    pub fn cudnnCreateConvolutionDescriptor(
        mut self,
        val: Option<unsafe extern "C" fn(convDesc: *mut cudnnConvolutionDescriptor_t) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnCreateConvolutionDescriptor = val;
        self
    }
    pub fn cudnnDestroyConvolutionDescriptor(
        mut self,
        val: Option<unsafe extern "C" fn(convDesc: cudnnConvolutionDescriptor_t) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnDestroyConvolutionDescriptor = val;
        self
    }
    pub fn cudnnSetConvolutionMathType(
        mut self,
        val: Option<
            unsafe extern "C" fn(convDesc: cudnnConvolutionDescriptor_t, mathType: cudnnMathType_t) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnSetConvolutionMathType = val;
        self
    }
    pub fn cudnnGetConvolutionMathType(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                convDesc: cudnnConvolutionDescriptor_t,
                mathType: *mut cudnnMathType_t,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnGetConvolutionMathType = val;
        self
    }
    pub fn cudnnSetConvolutionGroupCount(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                convDesc: cudnnConvolutionDescriptor_t,
                groupCount: ::std::os::raw::c_int,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnSetConvolutionGroupCount = val;
        self
    }
    pub fn cudnnGetConvolutionGroupCount(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                convDesc: cudnnConvolutionDescriptor_t,
                groupCount: *mut ::std::os::raw::c_int,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnGetConvolutionGroupCount = val;
        self
    }
    pub fn cudnnSetConvolutionReorderType(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                convDesc: cudnnConvolutionDescriptor_t,
                reorderType: cudnnReorderType_t,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnSetConvolutionReorderType = val;
        self
    }
    pub fn cudnnGetConvolutionReorderType(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                convDesc: cudnnConvolutionDescriptor_t,
                reorderType: *mut cudnnReorderType_t,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnGetConvolutionReorderType = val;
        self
    }
    pub fn cudnnSetConvolution2dDescriptor(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                convDesc: cudnnConvolutionDescriptor_t,
                pad_h: ::std::os::raw::c_int,
                pad_w: ::std::os::raw::c_int,
                u: ::std::os::raw::c_int,
                v: ::std::os::raw::c_int,
                dilation_h: ::std::os::raw::c_int,
                dilation_w: ::std::os::raw::c_int,
                mode: cudnnConvolutionMode_t,
                computeType: cudnnDataType_t,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnSetConvolution2dDescriptor = val;
        self
    }
    pub fn cudnnGetConvolution2dDescriptor(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                convDesc: cudnnConvolutionDescriptor_t,
                pad_h: *mut ::std::os::raw::c_int,
                pad_w: *mut ::std::os::raw::c_int,
                u: *mut ::std::os::raw::c_int,
                v: *mut ::std::os::raw::c_int,
                dilation_h: *mut ::std::os::raw::c_int,
                dilation_w: *mut ::std::os::raw::c_int,
                mode: *mut cudnnConvolutionMode_t,
                computeType: *mut cudnnDataType_t,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnGetConvolution2dDescriptor = val;
        self
    }
    pub fn cudnnSetConvolutionNdDescriptor(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                convDesc: cudnnConvolutionDescriptor_t,
                arrayLength: ::std::os::raw::c_int,
                padA: *const ::std::os::raw::c_int,
                filterStrideA: *const ::std::os::raw::c_int,
                dilationA: *const ::std::os::raw::c_int,
                mode: cudnnConvolutionMode_t,
                computeType: cudnnDataType_t,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnSetConvolutionNdDescriptor = val;
        self
    }
    pub fn cudnnGetConvolutionNdDescriptor(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                convDesc: cudnnConvolutionDescriptor_t,
                arrayLengthRequested: ::std::os::raw::c_int,
                arrayLength: *mut ::std::os::raw::c_int,
                padA: *mut ::std::os::raw::c_int,
                strideA: *mut ::std::os::raw::c_int,
                dilationA: *mut ::std::os::raw::c_int,
                mode: *mut cudnnConvolutionMode_t,
                computeType: *mut cudnnDataType_t,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnGetConvolutionNdDescriptor = val;
        self
    }
    pub fn cudnnGetConvolution2dForwardOutputDim(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                convDesc: cudnnConvolutionDescriptor_t,
                inputTensorDesc: cudnnTensorDescriptor_t,
                filterDesc: cudnnFilterDescriptor_t,
                n: *mut ::std::os::raw::c_int,
                c: *mut ::std::os::raw::c_int,
                h: *mut ::std::os::raw::c_int,
                w: *mut ::std::os::raw::c_int,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnGetConvolution2dForwardOutputDim = val;
        self
    }
    pub fn cudnnGetConvolutionNdForwardOutputDim(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                convDesc: cudnnConvolutionDescriptor_t,
                inputTensorDesc: cudnnTensorDescriptor_t,
                filterDesc: cudnnFilterDescriptor_t,
                nbDims: ::std::os::raw::c_int,
                tensorOuputDimA: *mut ::std::os::raw::c_int,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnGetConvolutionNdForwardOutputDim = val;
        self
    }
    pub fn cudnnGetConvolutionForwardAlgorithmMaxCount(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cudnnHandle_t, count: *mut ::std::os::raw::c_int) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnGetConvolutionForwardAlgorithmMaxCount = val;
        self
    }
    pub fn cudnnGetConvolutionForwardAlgorithm_v7(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                srcDesc: cudnnTensorDescriptor_t,
                filterDesc: cudnnFilterDescriptor_t,
                convDesc: cudnnConvolutionDescriptor_t,
                destDesc: cudnnTensorDescriptor_t,
                requestedAlgoCount: ::std::os::raw::c_int,
                returnedAlgoCount: *mut ::std::os::raw::c_int,
                perfResults: *mut cudnnConvolutionFwdAlgoPerf_t,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnGetConvolutionForwardAlgorithm_v7 = val;
        self
    }
    pub fn cudnnFindConvolutionForwardAlgorithm(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                xDesc: cudnnTensorDescriptor_t,
                wDesc: cudnnFilterDescriptor_t,
                convDesc: cudnnConvolutionDescriptor_t,
                yDesc: cudnnTensorDescriptor_t,
                requestedAlgoCount: ::std::os::raw::c_int,
                returnedAlgoCount: *mut ::std::os::raw::c_int,
                perfResults: *mut cudnnConvolutionFwdAlgoPerf_t,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnFindConvolutionForwardAlgorithm = val;
        self
    }
    pub fn cudnnFindConvolutionForwardAlgorithmEx(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                xDesc: cudnnTensorDescriptor_t,
                x: *const ::std::os::raw::c_void,
                wDesc: cudnnFilterDescriptor_t,
                w: *const ::std::os::raw::c_void,
                convDesc: cudnnConvolutionDescriptor_t,
                yDesc: cudnnTensorDescriptor_t,
                y: *mut ::std::os::raw::c_void,
                requestedAlgoCount: ::std::os::raw::c_int,
                returnedAlgoCount: *mut ::std::os::raw::c_int,
                perfResults: *mut cudnnConvolutionFwdAlgoPerf_t,
                workSpace: *mut ::std::os::raw::c_void,
                workSpaceSizeInBytes: usize,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnFindConvolutionForwardAlgorithmEx = val;
        self
    }
    pub fn cudnnIm2Col(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                xDesc: cudnnTensorDescriptor_t,
                x: *const ::std::os::raw::c_void,
                wDesc: cudnnFilterDescriptor_t,
                convDesc: cudnnConvolutionDescriptor_t,
                colBuffer: *mut ::std::os::raw::c_void,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnIm2Col = val;
        self
    }
    pub fn cudnnReorderFilterAndBias(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                filterDesc: cudnnFilterDescriptor_t,
                reorderType: cudnnReorderType_t,
                filterData: *const ::std::os::raw::c_void,
                reorderedFilterData: *mut ::std::os::raw::c_void,
                reorderBias: ::std::os::raw::c_int,
                biasData: *const ::std::os::raw::c_void,
                reorderedBiasData: *mut ::std::os::raw::c_void,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnReorderFilterAndBias = val;
        self
    }
    pub fn cudnnGetConvolutionForwardWorkspaceSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                xDesc: cudnnTensorDescriptor_t,
                wDesc: cudnnFilterDescriptor_t,
                convDesc: cudnnConvolutionDescriptor_t,
                yDesc: cudnnTensorDescriptor_t,
                algo: cudnnConvolutionFwdAlgo_t,
                sizeInBytes: *mut usize,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnGetConvolutionForwardWorkspaceSize = val;
        self
    }
    pub fn cudnnConvolutionForward(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                alpha: *const ::std::os::raw::c_void,
                xDesc: cudnnTensorDescriptor_t,
                x: *const ::std::os::raw::c_void,
                wDesc: cudnnFilterDescriptor_t,
                w: *const ::std::os::raw::c_void,
                convDesc: cudnnConvolutionDescriptor_t,
                algo: cudnnConvolutionFwdAlgo_t,
                workSpace: *mut ::std::os::raw::c_void,
                workSpaceSizeInBytes: usize,
                beta: *const ::std::os::raw::c_void,
                yDesc: cudnnTensorDescriptor_t,
                y: *mut ::std::os::raw::c_void,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnConvolutionForward = val;
        self
    }
    pub fn cudnnConvolutionBiasActivationForward(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                alpha1: *const ::std::os::raw::c_void,
                xDesc: cudnnTensorDescriptor_t,
                x: *const ::std::os::raw::c_void,
                wDesc: cudnnFilterDescriptor_t,
                w: *const ::std::os::raw::c_void,
                convDesc: cudnnConvolutionDescriptor_t,
                algo: cudnnConvolutionFwdAlgo_t,
                workSpace: *mut ::std::os::raw::c_void,
                workSpaceSizeInBytes: usize,
                alpha2: *const ::std::os::raw::c_void,
                zDesc: cudnnTensorDescriptor_t,
                z: *const ::std::os::raw::c_void,
                biasDesc: cudnnTensorDescriptor_t,
                bias: *const ::std::os::raw::c_void,
                activationDesc: cudnnActivationDescriptor_t,
                yDesc: cudnnTensorDescriptor_t,
                y: *mut ::std::os::raw::c_void,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnConvolutionBiasActivationForward = val;
        self
    }
    pub fn cudnnGetConvolutionBackwardDataAlgorithmMaxCount(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cudnnHandle_t, count: *mut ::std::os::raw::c_int) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnGetConvolutionBackwardDataAlgorithmMaxCount = val;
        self
    }
    pub fn cudnnFindConvolutionBackwardDataAlgorithm(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                wDesc: cudnnFilterDescriptor_t,
                dyDesc: cudnnTensorDescriptor_t,
                convDesc: cudnnConvolutionDescriptor_t,
                dxDesc: cudnnTensorDescriptor_t,
                requestedAlgoCount: ::std::os::raw::c_int,
                returnedAlgoCount: *mut ::std::os::raw::c_int,
                perfResults: *mut cudnnConvolutionBwdDataAlgoPerf_t,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnFindConvolutionBackwardDataAlgorithm = val;
        self
    }
    pub fn cudnnFindConvolutionBackwardDataAlgorithmEx(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                wDesc: cudnnFilterDescriptor_t,
                w: *const ::std::os::raw::c_void,
                dyDesc: cudnnTensorDescriptor_t,
                dy: *const ::std::os::raw::c_void,
                convDesc: cudnnConvolutionDescriptor_t,
                dxDesc: cudnnTensorDescriptor_t,
                dx: *mut ::std::os::raw::c_void,
                requestedAlgoCount: ::std::os::raw::c_int,
                returnedAlgoCount: *mut ::std::os::raw::c_int,
                perfResults: *mut cudnnConvolutionBwdDataAlgoPerf_t,
                workSpace: *mut ::std::os::raw::c_void,
                workSpaceSizeInBytes: usize,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnFindConvolutionBackwardDataAlgorithmEx = val;
        self
    }
    pub fn cudnnGetConvolutionBackwardDataAlgorithm_v7(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                filterDesc: cudnnFilterDescriptor_t,
                diffDesc: cudnnTensorDescriptor_t,
                convDesc: cudnnConvolutionDescriptor_t,
                gradDesc: cudnnTensorDescriptor_t,
                requestedAlgoCount: ::std::os::raw::c_int,
                returnedAlgoCount: *mut ::std::os::raw::c_int,
                perfResults: *mut cudnnConvolutionBwdDataAlgoPerf_t,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnGetConvolutionBackwardDataAlgorithm_v7 = val;
        self
    }
    pub fn cudnnGetConvolutionBackwardDataWorkspaceSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                wDesc: cudnnFilterDescriptor_t,
                dyDesc: cudnnTensorDescriptor_t,
                convDesc: cudnnConvolutionDescriptor_t,
                dxDesc: cudnnTensorDescriptor_t,
                algo: cudnnConvolutionBwdDataAlgo_t,
                sizeInBytes: *mut usize,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnGetConvolutionBackwardDataWorkspaceSize = val;
        self
    }
    pub fn cudnnConvolutionBackwardData(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                alpha: *const ::std::os::raw::c_void,
                wDesc: cudnnFilterDescriptor_t,
                w: *const ::std::os::raw::c_void,
                dyDesc: cudnnTensorDescriptor_t,
                dy: *const ::std::os::raw::c_void,
                convDesc: cudnnConvolutionDescriptor_t,
                algo: cudnnConvolutionBwdDataAlgo_t,
                workSpace: *mut ::std::os::raw::c_void,
                workSpaceSizeInBytes: usize,
                beta: *const ::std::os::raw::c_void,
                dxDesc: cudnnTensorDescriptor_t,
                dx: *mut ::std::os::raw::c_void,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnConvolutionBackwardData = val;
        self
    }
    pub fn cudnnGetFoldedConvBackwardDataDescriptors(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
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
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnGetFoldedConvBackwardDataDescriptors = val;
        self
    }
    pub fn cudnnCnnVersionCheck(mut self, val: Option<unsafe extern "C" fn() -> cudnnStatus_t>) -> Self {
        self.cudnnCnnVersionCheck = val;
        self
    }
    pub fn cudnnGetConvolutionBackwardFilterAlgorithmMaxCount(
        mut self,
        val: Option<unsafe extern "C" fn(handle: cudnnHandle_t, count: *mut ::std::os::raw::c_int) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnGetConvolutionBackwardFilterAlgorithmMaxCount = val;
        self
    }
    pub fn cudnnFindConvolutionBackwardFilterAlgorithm(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                xDesc: cudnnTensorDescriptor_t,
                dyDesc: cudnnTensorDescriptor_t,
                convDesc: cudnnConvolutionDescriptor_t,
                dwDesc: cudnnFilterDescriptor_t,
                requestedAlgoCount: ::std::os::raw::c_int,
                returnedAlgoCount: *mut ::std::os::raw::c_int,
                perfResults: *mut cudnnConvolutionBwdFilterAlgoPerf_t,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnFindConvolutionBackwardFilterAlgorithm = val;
        self
    }
    pub fn cudnnFindConvolutionBackwardFilterAlgorithmEx(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                xDesc: cudnnTensorDescriptor_t,
                x: *const ::std::os::raw::c_void,
                dyDesc: cudnnTensorDescriptor_t,
                y: *const ::std::os::raw::c_void,
                convDesc: cudnnConvolutionDescriptor_t,
                dwDesc: cudnnFilterDescriptor_t,
                dw: *mut ::std::os::raw::c_void,
                requestedAlgoCount: ::std::os::raw::c_int,
                returnedAlgoCount: *mut ::std::os::raw::c_int,
                perfResults: *mut cudnnConvolutionBwdFilterAlgoPerf_t,
                workSpace: *mut ::std::os::raw::c_void,
                workSpaceSizeInBytes: usize,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnFindConvolutionBackwardFilterAlgorithmEx = val;
        self
    }
    pub fn cudnnGetConvolutionBackwardFilterAlgorithm_v7(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                srcDesc: cudnnTensorDescriptor_t,
                diffDesc: cudnnTensorDescriptor_t,
                convDesc: cudnnConvolutionDescriptor_t,
                gradDesc: cudnnFilterDescriptor_t,
                requestedAlgoCount: ::std::os::raw::c_int,
                returnedAlgoCount: *mut ::std::os::raw::c_int,
                perfResults: *mut cudnnConvolutionBwdFilterAlgoPerf_t,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnGetConvolutionBackwardFilterAlgorithm_v7 = val;
        self
    }
    pub fn cudnnGetConvolutionBackwardFilterWorkspaceSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                xDesc: cudnnTensorDescriptor_t,
                dyDesc: cudnnTensorDescriptor_t,
                convDesc: cudnnConvolutionDescriptor_t,
                gradDesc: cudnnFilterDescriptor_t,
                algo: cudnnConvolutionBwdFilterAlgo_t,
                sizeInBytes: *mut usize,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnGetConvolutionBackwardFilterWorkspaceSize = val;
        self
    }
    pub fn cudnnConvolutionBackwardFilter(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                alpha: *const ::std::os::raw::c_void,
                xDesc: cudnnTensorDescriptor_t,
                x: *const ::std::os::raw::c_void,
                dyDesc: cudnnTensorDescriptor_t,
                dy: *const ::std::os::raw::c_void,
                convDesc: cudnnConvolutionDescriptor_t,
                algo: cudnnConvolutionBwdFilterAlgo_t,
                workSpace: *mut ::std::os::raw::c_void,
                workSpaceSizeInBytes: usize,
                beta: *const ::std::os::raw::c_void,
                dwDesc: cudnnFilterDescriptor_t,
                dw: *mut ::std::os::raw::c_void,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnConvolutionBackwardFilter = val;
        self
    }
    pub fn cudnnConvolutionBackwardBias(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                alpha: *const ::std::os::raw::c_void,
                dyDesc: cudnnTensorDescriptor_t,
                dy: *const ::std::os::raw::c_void,
                beta: *const ::std::os::raw::c_void,
                dbDesc: cudnnTensorDescriptor_t,
                db: *mut ::std::os::raw::c_void,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnConvolutionBackwardBias = val;
        self
    }
    pub fn cudnnCreateFusedOpsConstParamPack(
        mut self,
        val: Option<
            unsafe extern "C" fn(constPack: *mut cudnnFusedOpsConstParamPack_t, ops: cudnnFusedOps_t) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnCreateFusedOpsConstParamPack = val;
        self
    }
    pub fn cudnnDestroyFusedOpsConstParamPack(
        mut self,
        val: Option<unsafe extern "C" fn(constPack: cudnnFusedOpsConstParamPack_t) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnDestroyFusedOpsConstParamPack = val;
        self
    }
    pub fn cudnnSetFusedOpsConstParamPackAttribute(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                constPack: cudnnFusedOpsConstParamPack_t,
                paramLabel: cudnnFusedOpsConstParamLabel_t,
                param: *const ::std::os::raw::c_void,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnSetFusedOpsConstParamPackAttribute = val;
        self
    }
    pub fn cudnnGetFusedOpsConstParamPackAttribute(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                constPack: cudnnFusedOpsConstParamPack_t,
                paramLabel: cudnnFusedOpsConstParamLabel_t,
                param: *mut ::std::os::raw::c_void,
                isNULL: *mut ::std::os::raw::c_int,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnGetFusedOpsConstParamPackAttribute = val;
        self
    }
    pub fn cudnnCreateFusedOpsVariantParamPack(
        mut self,
        val: Option<
            unsafe extern "C" fn(varPack: *mut cudnnFusedOpsVariantParamPack_t, ops: cudnnFusedOps_t) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnCreateFusedOpsVariantParamPack = val;
        self
    }
    pub fn cudnnDestroyFusedOpsVariantParamPack(
        mut self,
        val: Option<unsafe extern "C" fn(varPack: cudnnFusedOpsVariantParamPack_t) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnDestroyFusedOpsVariantParamPack = val;
        self
    }
    pub fn cudnnSetFusedOpsVariantParamPackAttribute(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                varPack: cudnnFusedOpsVariantParamPack_t,
                paramLabel: cudnnFusedOpsVariantParamLabel_t,
                ptr: *mut ::std::os::raw::c_void,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnSetFusedOpsVariantParamPackAttribute = val;
        self
    }
    pub fn cudnnGetFusedOpsVariantParamPackAttribute(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                varPack: cudnnFusedOpsVariantParamPack_t,
                paramLabel: cudnnFusedOpsVariantParamLabel_t,
                ptr: *mut ::std::os::raw::c_void,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnGetFusedOpsVariantParamPackAttribute = val;
        self
    }
    pub fn cudnnCreateFusedOpsPlan(
        mut self,
        val: Option<unsafe extern "C" fn(plan: *mut cudnnFusedOpsPlan_t, ops: cudnnFusedOps_t) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnCreateFusedOpsPlan = val;
        self
    }
    pub fn cudnnDestroyFusedOpsPlan(
        mut self,
        val: Option<unsafe extern "C" fn(plan: cudnnFusedOpsPlan_t) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnDestroyFusedOpsPlan = val;
        self
    }
    pub fn cudnnMakeFusedOpsPlan(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                plan: cudnnFusedOpsPlan_t,
                constPack: cudnnFusedOpsConstParamPack_t,
                workspaceSizeInBytes: *mut usize,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnMakeFusedOpsPlan = val;
        self
    }
    pub fn cudnnFusedOpsExecute(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                handle: cudnnHandle_t,
                plan: cudnnFusedOpsPlan_t,
                varPack: cudnnFusedOpsVariantParamPack_t,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnFusedOpsExecute = val;
        self
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
pub unsafe fn cudnnGetLastErrorString<T: types::CudaAsPtr>(mut message: T, max_size: usize) {
    unsafe { crate::sys::cudnnGetLastErrorString(message.as_mut_ptr() as *mut _, max_size) }
}
pub unsafe fn cudnnQueryRuntimeError<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    handle: cudnnHandle_t,
    mut rstatus: T,
    mode: cudnnErrQueryMode_t,
    mut tag: U,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnQueryRuntimeError(handle, rstatus.as_mut_ptr() as *mut _, mode, tag.as_mut_ptr() as *mut _)
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnGetProperty(type_: libraryPropertyType) -> Result<i32, crate::sys::cudnnStatus_t> {
    let mut out_1: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudnnGetProperty(type_, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as i32) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnCreate() -> Result<cudnnHandle_t, crate::sys::cudnnStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cudnnHandle_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudnnCreate(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cudnnHandle_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnDestroy(handle: cudnnHandle_t) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnDestroy(handle) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnSetStream(handle: cudnnHandle_t, streamId: cudaStream_t) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnSetStream(handle, streamId) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnGetStream(handle: cudnnHandle_t) -> Result<cudaStream_t, crate::sys::cudnnStatus_t> {
    let mut out_1: std::mem::MaybeUninit<cudaStream_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudnnGetStream(handle, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as cudaStream_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnSetCallback<T: types::CudaAsPtr>(
    mask: u32,
    mut udata: T,
    fptr: cudnnCallback_t,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnSetCallback(mask as _, udata.as_mut_ptr() as *mut _, fptr) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnGetCallback(
    udata: *mut *mut ::std::os::raw::c_void,
) -> Result<(u32, cudnnCallback_t), crate::sys::cudnnStatus_t> {
    let mut out_0: std::mem::MaybeUninit<::std::os::raw::c_uint> = std::mem::MaybeUninit::uninit();
    let mut out_2: std::mem::MaybeUninit<cudnnCallback_t> = std::mem::MaybeUninit::uninit();
    let status =
        unsafe { crate::sys::cudnnGetCallback(out_0.as_mut_ptr() as *mut _, udata, out_2.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok((out_0.assume_init() as u32, out_2.assume_init() as cudnnCallback_t)) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
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
pub unsafe fn cudnnBackendCreateDescriptor(
    descriptorType: cudnnBackendDescriptorType_t,
) -> Result<cudnnBackendDescriptor_t, crate::sys::cudnnStatus_t> {
    let mut out_1: std::mem::MaybeUninit<cudnnBackendDescriptor_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudnnBackendCreateDescriptor(descriptorType, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as cudnnBackendDescriptor_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnBackendDestroyDescriptor(
    descriptor: cudnnBackendDescriptor_t,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnBackendDestroyDescriptor(descriptor) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnBackendInitialize(descriptor: cudnnBackendDescriptor_t) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnBackendInitialize(descriptor) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnBackendFinalize(descriptor: cudnnBackendDescriptor_t) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnBackendFinalize(descriptor) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnBackendSetAttribute<T: types::CudaAsPtr>(
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
            arrayOfElements.as_const_ptr() as *const _,
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
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_4.assume_init() as i64) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnBackendExecute(
    handle: cudnnHandle_t,
    executionPlan: cudnnBackendDescriptor_t,
    variantPack: cudnnBackendDescriptor_t,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnBackendExecute(handle, executionPlan, variantPack) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnBackendPopulateCudaGraph(
    handle: cudnnHandle_t,
    executionPlan: cudnnBackendDescriptor_t,
    variantPack: cudnnBackendDescriptor_t,
    graph: cudaGraph_t,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnBackendPopulateCudaGraph(handle, executionPlan, variantPack, graph) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnBackendUpdateCudaGraph(
    handle: cudnnHandle_t,
    executionPlan: cudnnBackendDescriptor_t,
    variantPack: cudnnBackendDescriptor_t,
    graph: cudaGraph_t,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnBackendUpdateCudaGraph(handle, executionPlan, variantPack, graph) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnCreateTensorDescriptor() -> Result<cudnnTensorDescriptor_t, crate::sys::cudnnStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cudnnTensorDescriptor_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudnnCreateTensorDescriptor(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cudnnTensorDescriptor_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnSetTensor4dDescriptor(
    tensorDesc: cudnnTensorDescriptor_t,
    format: cudnnTensorFormat_t,
    dataType: cudnnDataType_t,
    n: i32,
    c: i32,
    h: i32,
    w: i32,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status =
        unsafe { crate::sys::cudnnSetTensor4dDescriptor(tensorDesc, format, dataType, n as _, c as _, h as _, w as _) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnSetTensor4dDescriptorEx(
    tensorDesc: cudnnTensorDescriptor_t,
    dataType: cudnnDataType_t,
    n: i32,
    c: i32,
    h: i32,
    w: i32,
    nStride: i32,
    cStride: i32,
    hStride: i32,
    wStride: i32,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnSetTensor4dDescriptorEx(
            tensorDesc,
            dataType,
            n as _,
            c as _,
            h as _,
            w as _,
            nStride as _,
            cStride as _,
            hStride as _,
            wStride as _,
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
) -> Result<(cudnnDataType_t, i32, i32, i32, i32, i32, i32, i32, i32), crate::sys::cudnnStatus_t> {
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
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe {
            Ok((
                out_1.assume_init() as cudnnDataType_t,
                out_2.assume_init() as i32,
                out_3.assume_init() as i32,
                out_4.assume_init() as i32,
                out_5.assume_init() as i32,
                out_6.assume_init() as i32,
                out_7.assume_init() as i32,
                out_8.assume_init() as i32,
                out_9.assume_init() as i32,
            ))
        }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnSetTensorNdDescriptor<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    tensorDesc: cudnnTensorDescriptor_t,
    dataType: cudnnDataType_t,
    nbDims: i32,
    dimA: T,
    strideA: U,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnSetTensorNdDescriptor(
            tensorDesc,
            dataType,
            nbDims as _,
            dimA.as_const_ptr() as *const _,
            strideA.as_const_ptr() as *const _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnSetTensorNdDescriptorEx<T: types::CudaAsPtr>(
    tensorDesc: cudnnTensorDescriptor_t,
    format: cudnnTensorFormat_t,
    dataType: cudnnDataType_t,
    nbDims: i32,
    dimA: T,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnSetTensorNdDescriptorEx(
            tensorDesc,
            format,
            dataType,
            nbDims as _,
            dimA.as_const_ptr() as *const _,
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
    nbDimsRequested: i32,
) -> Result<(cudnnDataType_t, i32, i32, i32), crate::sys::cudnnStatus_t> {
    let mut out_2: std::mem::MaybeUninit<cudnnDataType_t> = std::mem::MaybeUninit::uninit();
    let mut out_3: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_4: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_5: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudnnGetTensorNdDescriptor(
            tensorDesc,
            nbDimsRequested as _,
            out_2.as_mut_ptr() as *mut _,
            out_3.as_mut_ptr() as *mut _,
            out_4.as_mut_ptr() as *mut _,
            out_5.as_mut_ptr() as *mut _,
        )
    };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe {
            Ok((
                out_2.assume_init() as cudnnDataType_t,
                out_3.assume_init() as i32,
                out_4.assume_init() as i32,
                out_5.assume_init() as i32,
            ))
        }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnGetTensorSizeInBytes(
    tensorDesc: cudnnTensorDescriptor_t,
) -> Result<usize, crate::sys::cudnnStatus_t> {
    let mut out_1: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudnnGetTensorSizeInBytes(tensorDesc, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as usize) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnDestroyTensorDescriptor(
    tensorDesc: cudnnTensorDescriptor_t,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnDestroyTensorDescriptor(tensorDesc) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Create a destination descriptor for cudnnTransformTensor"]
pub unsafe fn cudnnInitTransformDest<T: types::CudaAsPtr>(
    transformDesc: cudnnTensorTransformDescriptor_t,
    srcDesc: cudnnTensorDescriptor_t,
    destDesc: cudnnTensorDescriptor_t,
    mut destSizeInBytes: T,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnInitTransformDest(transformDesc, srcDesc, destDesc, destSizeInBytes.as_mut_ptr() as *mut _)
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Create an empty tensor transform descriptor"]
pub unsafe fn cudnnCreateTensorTransformDescriptor()
-> Result<cudnnTensorTransformDescriptor_t, crate::sys::cudnnStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cudnnTensorTransformDescriptor_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudnnCreateTensorTransformDescriptor(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cudnnTensorTransformDescriptor_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Initialize a previously created tensor transform descriptor."]
pub unsafe fn cudnnSetTensorTransformDescriptor<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
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
            padBeforeA.as_const_ptr() as *const _,
            padAfterA.as_const_ptr() as *const _,
            foldA.as_const_ptr() as *const _,
            direction,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Retrieves the values stored in a previously initialized tensor transform\ndescriptor."]
pub unsafe fn cudnnGetTensorTransformDescriptor(
    transformDesc: cudnnTensorTransformDescriptor_t,
    nbDimsRequested: u32,
) -> Result<(cudnnTensorFormat_t, i32, i32, u32, cudnnFoldingDirection_t), crate::sys::cudnnStatus_t> {
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
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe {
            Ok((
                out_2.assume_init() as cudnnTensorFormat_t,
                out_3.assume_init() as i32,
                out_4.assume_init() as i32,
                out_5.assume_init() as u32,
                out_6.assume_init() as cudnnFoldingDirection_t,
            ))
        }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Destroys a previously created tensor transform descriptor."]
pub unsafe fn cudnnDestroyTensorTransformDescriptor(
    transformDesc: cudnnTensorTransformDescriptor_t,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnDestroyTensorTransformDescriptor(transformDesc) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnTransformTensor<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cudnnHandle_t,
    alpha: T,
    xDesc: cudnnTensorDescriptor_t,
    x: U,
    beta: V,
    yDesc: cudnnTensorDescriptor_t,
    mut y: W,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnTransformTensor(
            handle,
            alpha.as_const_ptr() as *const _,
            xDesc,
            x.as_const_ptr() as *const _,
            beta.as_const_ptr() as *const _,
            yDesc,
            y.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnTransformTensorEx<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cudnnHandle_t,
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
            handle,
            transDesc,
            alpha.as_const_ptr() as *const _,
            srcDesc,
            srcData.as_const_ptr() as *const _,
            beta.as_const_ptr() as *const _,
            destDesc,
            destData.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnAddTensor<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    handle: cudnnHandle_t,
    alpha: T,
    aDesc: cudnnTensorDescriptor_t,
    A: U,
    beta: V,
    cDesc: cudnnTensorDescriptor_t,
    mut C: W,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnAddTensor(
            handle,
            alpha.as_const_ptr() as *const _,
            aDesc,
            A.as_const_ptr() as *const _,
            beta.as_const_ptr() as *const _,
            cDesc,
            C.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnCreateOpTensorDescriptor() -> Result<cudnnOpTensorDescriptor_t, crate::sys::cudnnStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cudnnOpTensorDescriptor_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudnnCreateOpTensorDescriptor(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cudnnOpTensorDescriptor_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnSetOpTensorDescriptor(
    opTensorDesc: cudnnOpTensorDescriptor_t,
    opTensorOp: cudnnOpTensorOp_t,
    opTensorCompType: cudnnDataType_t,
    opTensorNanOpt: cudnnNanPropagation_t,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status =
        unsafe { crate::sys::cudnnSetOpTensorDescriptor(opTensorDesc, opTensorOp, opTensorCompType, opTensorNanOpt) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnGetOpTensorDescriptor(
    opTensorDesc: cudnnOpTensorDescriptor_t,
) -> Result<(cudnnOpTensorOp_t, cudnnDataType_t, cudnnNanPropagation_t), crate::sys::cudnnStatus_t> {
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
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe {
            Ok((
                out_1.assume_init() as cudnnOpTensorOp_t,
                out_2.assume_init() as cudnnDataType_t,
                out_3.assume_init() as cudnnNanPropagation_t,
            ))
        }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnDestroyOpTensorDescriptor(
    opTensorDesc: cudnnOpTensorDescriptor_t,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnDestroyOpTensorDescriptor(opTensorDesc) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnOpTensor<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cudnnHandle_t,
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
            handle,
            opTensorDesc,
            alpha1.as_const_ptr() as *const _,
            aDesc,
            A.as_const_ptr() as *const _,
            alpha2.as_const_ptr() as *const _,
            bDesc,
            B.as_const_ptr() as *const _,
            beta.as_const_ptr() as *const _,
            cDesc,
            C.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnCreateReduceTensorDescriptor() -> Result<cudnnReduceTensorDescriptor_t, crate::sys::cudnnStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cudnnReduceTensorDescriptor_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudnnCreateReduceTensorDescriptor(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cudnnReduceTensorDescriptor_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
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
    let mut out_4: std::mem::MaybeUninit<cudnnReduceTensorIndices_t> = std::mem::MaybeUninit::uninit();
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
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe {
            Ok((
                out_1.assume_init() as cudnnReduceTensorOp_t,
                out_2.assume_init() as cudnnDataType_t,
                out_3.assume_init() as cudnnNanPropagation_t,
                out_4.assume_init() as cudnnReduceTensorIndices_t,
                out_5.assume_init() as cudnnIndicesType_t,
            ))
        }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnDestroyReduceTensorDescriptor(
    reduceTensorDesc: cudnnReduceTensorDescriptor_t,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnDestroyReduceTensorDescriptor(reduceTensorDesc) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnGetReductionIndicesSize(
    handle: cudnnHandle_t,
    reduceTensorDesc: cudnnReduceTensorDescriptor_t,
    aDesc: cudnnTensorDescriptor_t,
    cDesc: cudnnTensorDescriptor_t,
) -> Result<usize, crate::sys::cudnnStatus_t> {
    let mut out_4: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudnnGetReductionIndicesSize(handle, reduceTensorDesc, aDesc, cDesc, out_4.as_mut_ptr() as *mut _)
    };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_4.assume_init() as usize) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnGetReductionWorkspaceSize(
    handle: cudnnHandle_t,
    reduceTensorDesc: cudnnReduceTensorDescriptor_t,
    aDesc: cudnnTensorDescriptor_t,
    cDesc: cudnnTensorDescriptor_t,
) -> Result<usize, crate::sys::cudnnStatus_t> {
    let mut out_4: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudnnGetReductionWorkspaceSize(handle, reduceTensorDesc, aDesc, cDesc, out_4.as_mut_ptr() as *mut _)
    };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_4.assume_init() as usize) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnReduceTensor<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cudnnHandle_t,
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
            handle,
            reduceTensorDesc,
            indices.as_mut_ptr() as *mut _,
            indicesSizeInBytes,
            workspace.as_mut_ptr() as *mut _,
            workspaceSizeInBytes,
            alpha.as_const_ptr() as *const _,
            aDesc,
            A.as_const_ptr() as *const _,
            beta.as_const_ptr() as *const _,
            cDesc,
            C.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnSetTensor<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    handle: cudnnHandle_t,
    yDesc: cudnnTensorDescriptor_t,
    mut y: T,
    valuePtr: U,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnSetTensor(
            handle,
            yDesc,
            y.as_mut_ptr() as *mut _,
            valuePtr.as_const_ptr() as *const _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnScaleTensor<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    handle: cudnnHandle_t,
    yDesc: cudnnTensorDescriptor_t,
    mut y: T,
    alpha: U,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnScaleTensor(
            handle,
            yDesc,
            y.as_mut_ptr() as *mut _,
            alpha.as_const_ptr() as *const _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnCreateFilterDescriptor() -> Result<cudnnFilterDescriptor_t, crate::sys::cudnnStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cudnnFilterDescriptor_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudnnCreateFilterDescriptor(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cudnnFilterDescriptor_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnSetFilter4dDescriptor(
    filterDesc: cudnnFilterDescriptor_t,
    dataType: cudnnDataType_t,
    format: cudnnTensorFormat_t,
    k: i32,
    c: i32,
    h: i32,
    w: i32,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status =
        unsafe { crate::sys::cudnnSetFilter4dDescriptor(filterDesc, dataType, format, k as _, c as _, h as _, w as _) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnGetFilter4dDescriptor(
    filterDesc: cudnnFilterDescriptor_t,
) -> Result<(cudnnDataType_t, cudnnTensorFormat_t, i32, i32, i32, i32), crate::sys::cudnnStatus_t> {
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
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe {
            Ok((
                out_1.assume_init() as cudnnDataType_t,
                out_2.assume_init() as cudnnTensorFormat_t,
                out_3.assume_init() as i32,
                out_4.assume_init() as i32,
                out_5.assume_init() as i32,
                out_6.assume_init() as i32,
            ))
        }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnSetFilterNdDescriptor<T: types::CudaAsPtr>(
    filterDesc: cudnnFilterDescriptor_t,
    dataType: cudnnDataType_t,
    format: cudnnTensorFormat_t,
    nbDims: i32,
    filterDimA: T,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnSetFilterNdDescriptor(
            filterDesc,
            dataType,
            format,
            nbDims as _,
            filterDimA.as_const_ptr() as *const _,
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
    nbDimsRequested: i32,
) -> Result<(cudnnDataType_t, cudnnTensorFormat_t, i32, i32), crate::sys::cudnnStatus_t> {
    let mut out_2: std::mem::MaybeUninit<cudnnDataType_t> = std::mem::MaybeUninit::uninit();
    let mut out_3: std::mem::MaybeUninit<cudnnTensorFormat_t> = std::mem::MaybeUninit::uninit();
    let mut out_4: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_5: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudnnGetFilterNdDescriptor(
            filterDesc,
            nbDimsRequested as _,
            out_2.as_mut_ptr() as *mut _,
            out_3.as_mut_ptr() as *mut _,
            out_4.as_mut_ptr() as *mut _,
            out_5.as_mut_ptr() as *mut _,
        )
    };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe {
            Ok((
                out_2.assume_init() as cudnnDataType_t,
                out_3.assume_init() as cudnnTensorFormat_t,
                out_4.assume_init() as i32,
                out_5.assume_init() as i32,
            ))
        }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnGetFilterSizeInBytes(
    filterDesc: cudnnFilterDescriptor_t,
) -> Result<usize, crate::sys::cudnnStatus_t> {
    let mut out_1: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudnnGetFilterSizeInBytes(filterDesc, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as usize) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnTransformFilter<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cudnnHandle_t,
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
            handle,
            transDesc,
            alpha.as_const_ptr() as *const _,
            srcDesc,
            srcData.as_const_ptr() as *const _,
            beta.as_const_ptr() as *const _,
            destDesc,
            destData.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnDestroyFilterDescriptor(
    filterDesc: cudnnFilterDescriptor_t,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnDestroyFilterDescriptor(filterDesc) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnSoftmaxForward<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cudnnHandle_t,
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
            handle,
            algo,
            mode,
            alpha.as_const_ptr() as *const _,
            xDesc,
            x.as_const_ptr() as *const _,
            beta.as_const_ptr() as *const _,
            yDesc,
            y.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnCreatePoolingDescriptor() -> Result<cudnnPoolingDescriptor_t, crate::sys::cudnnStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cudnnPoolingDescriptor_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudnnCreatePoolingDescriptor(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cudnnPoolingDescriptor_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnSetPooling2dDescriptor(
    poolingDesc: cudnnPoolingDescriptor_t,
    mode: cudnnPoolingMode_t,
    maxpoolingNanOpt: cudnnNanPropagation_t,
    windowHeight: i32,
    windowWidth: i32,
    verticalPadding: i32,
    horizontalPadding: i32,
    verticalStride: i32,
    horizontalStride: i32,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnSetPooling2dDescriptor(
            poolingDesc,
            mode,
            maxpoolingNanOpt,
            windowHeight as _,
            windowWidth as _,
            verticalPadding as _,
            horizontalPadding as _,
            verticalStride as _,
            horizontalStride as _,
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
) -> Result<(cudnnPoolingMode_t, cudnnNanPropagation_t, i32, i32, i32, i32, i32, i32), crate::sys::cudnnStatus_t> {
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
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe {
            Ok((
                out_1.assume_init() as cudnnPoolingMode_t,
                out_2.assume_init() as cudnnNanPropagation_t,
                out_3.assume_init() as i32,
                out_4.assume_init() as i32,
                out_5.assume_init() as i32,
                out_6.assume_init() as i32,
                out_7.assume_init() as i32,
                out_8.assume_init() as i32,
            ))
        }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnSetPoolingNdDescriptor<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    poolingDesc: cudnnPoolingDescriptor_t,
    mode: cudnnPoolingMode_t,
    maxpoolingNanOpt: cudnnNanPropagation_t,
    nbDims: i32,
    windowDimA: T,
    paddingA: U,
    strideA: V,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnSetPoolingNdDescriptor(
            poolingDesc,
            mode,
            maxpoolingNanOpt,
            nbDims as _,
            windowDimA.as_const_ptr() as *const _,
            paddingA.as_const_ptr() as *const _,
            strideA.as_const_ptr() as *const _,
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
    nbDimsRequested: i32,
) -> Result<(cudnnPoolingMode_t, cudnnNanPropagation_t, i32, i32, i32, i32), crate::sys::cudnnStatus_t> {
    let mut out_2: std::mem::MaybeUninit<cudnnPoolingMode_t> = std::mem::MaybeUninit::uninit();
    let mut out_3: std::mem::MaybeUninit<cudnnNanPropagation_t> = std::mem::MaybeUninit::uninit();
    let mut out_4: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_5: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_6: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_7: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudnnGetPoolingNdDescriptor(
            poolingDesc,
            nbDimsRequested as _,
            out_2.as_mut_ptr() as *mut _,
            out_3.as_mut_ptr() as *mut _,
            out_4.as_mut_ptr() as *mut _,
            out_5.as_mut_ptr() as *mut _,
            out_6.as_mut_ptr() as *mut _,
            out_7.as_mut_ptr() as *mut _,
        )
    };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe {
            Ok((
                out_2.assume_init() as cudnnPoolingMode_t,
                out_3.assume_init() as cudnnNanPropagation_t,
                out_4.assume_init() as i32,
                out_5.assume_init() as i32,
                out_6.assume_init() as i32,
                out_7.assume_init() as i32,
            ))
        }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnGetPoolingNdForwardOutputDim(
    poolingDesc: cudnnPoolingDescriptor_t,
    inputTensorDesc: cudnnTensorDescriptor_t,
    nbDims: i32,
) -> Result<i32, crate::sys::cudnnStatus_t> {
    let mut out_3: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudnnGetPoolingNdForwardOutputDim(
            poolingDesc,
            inputTensorDesc,
            nbDims as _,
            out_3.as_mut_ptr() as *mut _,
        )
    };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_3.assume_init() as i32) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnGetPooling2dForwardOutputDim(
    poolingDesc: cudnnPoolingDescriptor_t,
    inputTensorDesc: cudnnTensorDescriptor_t,
) -> Result<(i32, i32, i32, i32), crate::sys::cudnnStatus_t> {
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
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe {
            Ok((
                out_2.assume_init() as i32,
                out_3.assume_init() as i32,
                out_4.assume_init() as i32,
                out_5.assume_init() as i32,
            ))
        }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnDestroyPoolingDescriptor(
    poolingDesc: cudnnPoolingDescriptor_t,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnDestroyPoolingDescriptor(poolingDesc) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnPoolingForward<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cudnnHandle_t,
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
            handle,
            poolingDesc,
            alpha.as_const_ptr() as *const _,
            xDesc,
            x.as_const_ptr() as *const _,
            beta.as_const_ptr() as *const _,
            yDesc,
            y.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnCreateActivationDescriptor() -> Result<cudnnActivationDescriptor_t, crate::sys::cudnnStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cudnnActivationDescriptor_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudnnCreateActivationDescriptor(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cudnnActivationDescriptor_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnSetActivationDescriptor(
    activationDesc: cudnnActivationDescriptor_t,
    mode: cudnnActivationMode_t,
    reluNanOpt: cudnnNanPropagation_t,
    coef: f64,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnSetActivationDescriptor(activationDesc, mode, reluNanOpt, coef) };
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
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe {
            Ok((
                out_1.assume_init() as cudnnActivationMode_t,
                out_2.assume_init() as cudnnNanPropagation_t,
                out_3.assume_init() as f64,
            ))
        }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnSetActivationDescriptorSwishBeta(
    activationDesc: cudnnActivationDescriptor_t,
    swish_beta: f64,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnSetActivationDescriptorSwishBeta(activationDesc, swish_beta) };
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
    let status =
        unsafe { crate::sys::cudnnGetActivationDescriptorSwishBeta(activationDesc, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as f64) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnDestroyActivationDescriptor(
    activationDesc: cudnnActivationDescriptor_t,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnDestroyActivationDescriptor(activationDesc) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnActivationForward<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cudnnHandle_t,
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
            handle,
            activationDesc,
            alpha.as_const_ptr() as *const _,
            xDesc,
            x.as_const_ptr() as *const _,
            beta.as_const_ptr() as *const _,
            yDesc,
            y.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnCreateLRNDescriptor() -> Result<cudnnLRNDescriptor_t, crate::sys::cudnnStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cudnnLRNDescriptor_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudnnCreateLRNDescriptor(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cudnnLRNDescriptor_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnSetLRNDescriptor(
    normDesc: cudnnLRNDescriptor_t,
    lrnN: u32,
    lrnAlpha: f64,
    lrnBeta: f64,
    lrnK: f64,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnSetLRNDescriptor(normDesc, lrnN as _, lrnAlpha, lrnBeta, lrnK) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnGetLRNDescriptor(
    normDesc: cudnnLRNDescriptor_t,
) -> Result<(u32, f64, f64, f64), crate::sys::cudnnStatus_t> {
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
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe {
            Ok((
                out_1.assume_init() as u32,
                out_2.assume_init() as f64,
                out_3.assume_init() as f64,
                out_4.assume_init() as f64,
            ))
        }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnDestroyLRNDescriptor(lrnDesc: cudnnLRNDescriptor_t) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnDestroyLRNDescriptor(lrnDesc) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnLRNCrossChannelForward<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cudnnHandle_t,
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
            handle,
            normDesc,
            lrnMode,
            alpha.as_const_ptr() as *const _,
            xDesc,
            x.as_const_ptr() as *const _,
            beta.as_const_ptr() as *const _,
            yDesc,
            y.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnDivisiveNormalizationForward<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cudnnHandle_t,
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
            handle,
            normDesc,
            mode,
            alpha.as_const_ptr() as *const _,
            xDesc,
            x.as_const_ptr() as *const _,
            means.as_const_ptr() as *const _,
            temp.as_mut_ptr() as *mut _,
            temp2.as_mut_ptr() as *mut _,
            beta.as_const_ptr() as *const _,
            yDesc,
            y.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
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
pub unsafe fn cudnnBatchNormalizationForwardInference<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
    A: types::CudaAsPtr,
>(
    handle: cudnnHandle_t,
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
            handle,
            mode,
            alpha.as_const_ptr() as *const _,
            beta.as_const_ptr() as *const _,
            xDesc,
            x.as_const_ptr() as *const _,
            yDesc,
            y.as_mut_ptr() as *mut _,
            bnScaleBiasMeanVarDesc,
            bnScale.as_const_ptr() as *const _,
            bnBias.as_const_ptr() as *const _,
            estimatedMean.as_const_ptr() as *const _,
            estimatedVariance.as_const_ptr() as *const _,
            epsilon,
        )
    };
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
    groupCnt: i32,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnDeriveNormTensorDescriptor(
            derivedNormScaleBiasDesc,
            derivedNormMeanVarDesc,
            xDesc,
            mode,
            groupCnt as _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnNormalizationForwardInference<
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
    handle: cudnnHandle_t,
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
    groupCnt: i32,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnNormalizationForwardInference(
            handle,
            mode,
            normOps,
            algo,
            alpha.as_const_ptr() as *const _,
            beta.as_const_ptr() as *const _,
            xDesc,
            x.as_const_ptr() as *const _,
            normScaleBiasDesc,
            normScale.as_const_ptr() as *const _,
            normBias.as_const_ptr() as *const _,
            normMeanVarDesc,
            estimatedMean.as_const_ptr() as *const _,
            estimatedVariance.as_const_ptr() as *const _,
            zDesc,
            z.as_const_ptr() as *const _,
            activationDesc,
            yDesc,
            y.as_mut_ptr() as *mut _,
            epsilon,
            groupCnt as _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnCreateSpatialTransformerDescriptor()
-> Result<cudnnSpatialTransformerDescriptor_t, crate::sys::cudnnStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cudnnSpatialTransformerDescriptor_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudnnCreateSpatialTransformerDescriptor(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cudnnSpatialTransformerDescriptor_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnSetSpatialTransformerNdDescriptor<T: types::CudaAsPtr>(
    stDesc: cudnnSpatialTransformerDescriptor_t,
    samplerType: cudnnSamplerType_t,
    dataType: cudnnDataType_t,
    nbDims: i32,
    dimA: T,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnSetSpatialTransformerNdDescriptor(
            stDesc,
            samplerType,
            dataType,
            nbDims as _,
            dimA.as_const_ptr() as *const _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnDestroySpatialTransformerDescriptor(
    stDesc: cudnnSpatialTransformerDescriptor_t,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnDestroySpatialTransformerDescriptor(stDesc) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnSpatialTfGridGeneratorForward<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    handle: cudnnHandle_t,
    stDesc: cudnnSpatialTransformerDescriptor_t,
    theta: T,
    mut grid: U,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnSpatialTfGridGeneratorForward(
            handle,
            stDesc,
            theta.as_const_ptr() as *const _,
            grid.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnSpatialTfSamplerForward<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cudnnHandle_t,
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
            handle,
            stDesc,
            alpha.as_const_ptr() as *const _,
            xDesc,
            x.as_const_ptr() as *const _,
            grid.as_const_ptr() as *const _,
            beta.as_const_ptr() as *const _,
            yDesc,
            y.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnCreateDropoutDescriptor() -> Result<cudnnDropoutDescriptor_t, crate::sys::cudnnStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cudnnDropoutDescriptor_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudnnCreateDropoutDescriptor(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cudnnDropoutDescriptor_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnDestroyDropoutDescriptor(
    dropoutDesc: cudnnDropoutDescriptor_t,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnDestroyDropoutDescriptor(dropoutDesc) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnDropoutGetStatesSize(handle: cudnnHandle_t) -> Result<usize, crate::sys::cudnnStatus_t> {
    let mut out_1: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudnnDropoutGetStatesSize(handle, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as usize) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnDropoutGetReserveSpaceSize(
    xdesc: cudnnTensorDescriptor_t,
) -> Result<usize, crate::sys::cudnnStatus_t> {
    let mut out_1: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudnnDropoutGetReserveSpaceSize(xdesc, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as usize) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnSetDropoutDescriptor<T: types::CudaAsPtr>(
    dropoutDesc: cudnnDropoutDescriptor_t,
    handle: cudnnHandle_t,
    dropout: f32,
    mut states: T,
    stateSizeInBytes: usize,
    seed: u64,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnSetDropoutDescriptor(
            dropoutDesc,
            handle,
            dropout,
            states.as_mut_ptr() as *mut _,
            stateSizeInBytes,
            seed as _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnRestoreDropoutDescriptor<T: types::CudaAsPtr>(
    dropoutDesc: cudnnDropoutDescriptor_t,
    handle: cudnnHandle_t,
    dropout: f32,
    mut states: T,
    stateSizeInBytes: usize,
    seed: u64,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnRestoreDropoutDescriptor(
            dropoutDesc,
            handle,
            dropout,
            states.as_mut_ptr() as *mut _,
            stateSizeInBytes,
            seed as _,
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
) -> Result<(f32, u64), crate::sys::cudnnStatus_t> {
    let mut out_2: std::mem::MaybeUninit<f32> = std::mem::MaybeUninit::uninit();
    let mut out_4: std::mem::MaybeUninit<::std::os::raw::c_ulonglong> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudnnGetDropoutDescriptor(
            dropoutDesc,
            handle,
            out_2.as_mut_ptr() as *mut _,
            states,
            out_4.as_mut_ptr() as *mut _,
        )
    };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok((out_2.assume_init() as f32, out_4.assume_init() as u64)) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnDropoutForward<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cudnnHandle_t,
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
            handle,
            dropoutDesc,
            xdesc,
            x.as_const_ptr() as *const _,
            ydesc,
            y.as_mut_ptr() as *mut _,
            reserveSpace.as_mut_ptr() as *mut _,
            reserveSpaceSizeInBytes,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
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
pub unsafe fn cudnnSoftmaxBackward<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    handle: cudnnHandle_t,
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
            handle,
            algo,
            mode,
            alpha.as_const_ptr() as *const _,
            yDesc,
            y.as_const_ptr() as *const _,
            dyDesc,
            dy.as_const_ptr() as *const _,
            beta.as_const_ptr() as *const _,
            dxDesc,
            dx.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnPoolingBackward<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cudnnHandle_t,
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
            handle,
            poolingDesc,
            alpha.as_const_ptr() as *const _,
            yDesc,
            y.as_const_ptr() as *const _,
            dyDesc,
            dy.as_const_ptr() as *const _,
            xDesc,
            x.as_const_ptr() as *const _,
            beta.as_const_ptr() as *const _,
            dxDesc,
            dx.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnActivationBackward<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cudnnHandle_t,
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
            handle,
            activationDesc,
            alpha.as_const_ptr() as *const _,
            yDesc,
            y.as_const_ptr() as *const _,
            dyDesc,
            dy.as_const_ptr() as *const _,
            xDesc,
            x.as_const_ptr() as *const _,
            beta.as_const_ptr() as *const _,
            dxDesc,
            dx.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnLRNCrossChannelBackward<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cudnnHandle_t,
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
            handle,
            normDesc,
            lrnMode,
            alpha.as_const_ptr() as *const _,
            yDesc,
            y.as_const_ptr() as *const _,
            dyDesc,
            dy.as_const_ptr() as *const _,
            xDesc,
            x.as_const_ptr() as *const _,
            beta.as_const_ptr() as *const _,
            dxDesc,
            dx.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnDivisiveNormalizationBackward<
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
    handle: cudnnHandle_t,
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
            handle,
            normDesc,
            mode,
            alpha.as_const_ptr() as *const _,
            xDesc,
            x.as_const_ptr() as *const _,
            means.as_const_ptr() as *const _,
            dy.as_const_ptr() as *const _,
            temp.as_mut_ptr() as *mut _,
            temp2.as_mut_ptr() as *mut _,
            beta.as_const_ptr() as *const _,
            dXdMeansDesc,
            dx.as_mut_ptr() as *mut _,
            dMeans.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnGetBatchNormalizationForwardTrainingExWorkspaceSize(
    handle: cudnnHandle_t,
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
            handle,
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
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_8.assume_init() as usize) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnGetBatchNormalizationBackwardExWorkspaceSize(
    handle: cudnnHandle_t,
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
            handle,
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
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_10.assume_init() as usize) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnGetBatchNormalizationTrainingExReserveSpaceSize(
    handle: cudnnHandle_t,
    mode: cudnnBatchNormMode_t,
    bnOps: cudnnBatchNormOps_t,
    activationDesc: cudnnActivationDescriptor_t,
    xDesc: cudnnTensorDescriptor_t,
) -> Result<usize, crate::sys::cudnnStatus_t> {
    let mut out_5: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudnnGetBatchNormalizationTrainingExReserveSpaceSize(
            handle,
            mode,
            bnOps,
            activationDesc,
            xDesc,
            out_5.as_mut_ptr() as *mut _,
        )
    };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_5.assume_init() as usize) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnBatchNormalizationForwardTraining<
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
>(
    handle: cudnnHandle_t,
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
            handle,
            mode,
            alpha.as_const_ptr() as *const _,
            beta.as_const_ptr() as *const _,
            xDesc,
            x.as_const_ptr() as *const _,
            yDesc,
            y.as_mut_ptr() as *mut _,
            bnScaleBiasMeanVarDesc,
            bnScale.as_const_ptr() as *const _,
            bnBias.as_const_ptr() as *const _,
            exponentialAverageFactor,
            resultRunningMean.as_mut_ptr() as *mut _,
            resultRunningVariance.as_mut_ptr() as *mut _,
            epsilon,
            resultSaveMean.as_mut_ptr() as *mut _,
            resultSaveInvVariance.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnBatchNormalizationForwardTrainingEx<
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
    F: types::CudaAsPtr,
>(
    handle: cudnnHandle_t,
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
            handle,
            mode,
            bnOps,
            alpha.as_const_ptr() as *const _,
            beta.as_const_ptr() as *const _,
            xDesc,
            xData.as_const_ptr() as *const _,
            zDesc,
            zData.as_const_ptr() as *const _,
            yDesc,
            yData.as_mut_ptr() as *mut _,
            bnScaleBiasMeanVarDesc,
            bnScale.as_const_ptr() as *const _,
            bnBias.as_const_ptr() as *const _,
            exponentialAverageFactor,
            resultRunningMean.as_mut_ptr() as *mut _,
            resultRunningVariance.as_mut_ptr() as *mut _,
            epsilon,
            resultSaveMean.as_mut_ptr() as *mut _,
            resultSaveInvVariance.as_mut_ptr() as *mut _,
            activationDesc,
            workspace.as_mut_ptr() as *mut _,
            workSpaceSizeInBytes,
            reserveSpace.as_mut_ptr() as *mut _,
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
    handle: cudnnHandle_t,
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
            handle,
            mode,
            alphaDataDiff.as_const_ptr() as *const _,
            betaDataDiff.as_const_ptr() as *const _,
            alphaParamDiff.as_const_ptr() as *const _,
            betaParamDiff.as_const_ptr() as *const _,
            xDesc,
            x.as_const_ptr() as *const _,
            dyDesc,
            dy.as_const_ptr() as *const _,
            dxDesc,
            dx.as_mut_ptr() as *mut _,
            dBnScaleBiasDesc,
            bnScale.as_const_ptr() as *const _,
            dBnScaleResult.as_mut_ptr() as *mut _,
            dBnBiasResult.as_mut_ptr() as *mut _,
            epsilon,
            savedMean.as_const_ptr() as *const _,
            savedInvVariance.as_const_ptr() as *const _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnBatchNormalizationBackwardEx<
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
    F: types::CudaAsPtr,
    T13: types::CudaAsPtr,
    T14: types::CudaAsPtr,
    T15: types::CudaAsPtr,
    T16: types::CudaAsPtr,
>(
    handle: cudnnHandle_t,
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
            handle,
            mode,
            bnOps,
            alphaDataDiff.as_const_ptr() as *const _,
            betaDataDiff.as_const_ptr() as *const _,
            alphaParamDiff.as_const_ptr() as *const _,
            betaParamDiff.as_const_ptr() as *const _,
            xDesc,
            xData.as_const_ptr() as *const _,
            yDesc,
            yData.as_const_ptr() as *const _,
            dyDesc,
            dyData.as_const_ptr() as *const _,
            dzDesc,
            dzData.as_mut_ptr() as *mut _,
            dxDesc,
            dxData.as_mut_ptr() as *mut _,
            dBnScaleBiasDesc,
            bnScaleData.as_const_ptr() as *const _,
            bnBiasData.as_const_ptr() as *const _,
            dBnScaleData.as_mut_ptr() as *mut _,
            dBnBiasData.as_mut_ptr() as *mut _,
            epsilon,
            savedMean.as_const_ptr() as *const _,
            savedInvVariance.as_const_ptr() as *const _,
            activationDesc,
            workSpace.as_mut_ptr() as *mut _,
            workSpaceSizeInBytes,
            reserveSpace.as_mut_ptr() as *mut _,
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
    handle: cudnnHandle_t,
    mode: cudnnNormMode_t,
    normOps: cudnnNormOps_t,
    algo: cudnnNormAlgo_t,
    xDesc: cudnnTensorDescriptor_t,
    zDesc: cudnnTensorDescriptor_t,
    yDesc: cudnnTensorDescriptor_t,
    normScaleBiasDesc: cudnnTensorDescriptor_t,
    activationDesc: cudnnActivationDescriptor_t,
    normMeanVarDesc: cudnnTensorDescriptor_t,
    groupCnt: i32,
) -> Result<usize, crate::sys::cudnnStatus_t> {
    let mut out_10: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudnnGetNormalizationForwardTrainingWorkspaceSize(
            handle,
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
            groupCnt as _,
        )
    };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_10.assume_init() as usize) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnGetNormalizationBackwardWorkspaceSize(
    handle: cudnnHandle_t,
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
    groupCnt: i32,
) -> Result<usize, crate::sys::cudnnStatus_t> {
    let mut out_12: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudnnGetNormalizationBackwardWorkspaceSize(
            handle,
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
            groupCnt as _,
        )
    };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_12.assume_init() as usize) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnGetNormalizationTrainingReserveSpaceSize(
    handle: cudnnHandle_t,
    mode: cudnnNormMode_t,
    normOps: cudnnNormOps_t,
    algo: cudnnNormAlgo_t,
    activationDesc: cudnnActivationDescriptor_t,
    xDesc: cudnnTensorDescriptor_t,
    groupCnt: i32,
) -> Result<usize, crate::sys::cudnnStatus_t> {
    let mut out_6: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudnnGetNormalizationTrainingReserveSpaceSize(
            handle,
            mode,
            normOps,
            algo,
            activationDesc,
            xDesc,
            out_6.as_mut_ptr() as *mut _,
            groupCnt as _,
        )
    };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_6.assume_init() as usize) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnNormalizationForwardTraining<
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
    F: types::CudaAsPtr,
>(
    handle: cudnnHandle_t,
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
    groupCnt: i32,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnNormalizationForwardTraining(
            handle,
            mode,
            normOps,
            algo,
            alpha.as_const_ptr() as *const _,
            beta.as_const_ptr() as *const _,
            xDesc,
            xData.as_const_ptr() as *const _,
            normScaleBiasDesc,
            normScale.as_const_ptr() as *const _,
            normBias.as_const_ptr() as *const _,
            exponentialAverageFactor,
            normMeanVarDesc,
            resultRunningMean.as_mut_ptr() as *mut _,
            resultRunningVariance.as_mut_ptr() as *mut _,
            epsilon,
            resultSaveMean.as_mut_ptr() as *mut _,
            resultSaveInvVariance.as_mut_ptr() as *mut _,
            activationDesc,
            zDesc,
            zData.as_const_ptr() as *const _,
            yDesc,
            yData.as_mut_ptr() as *mut _,
            workspace.as_mut_ptr() as *mut _,
            workSpaceSizeInBytes,
            reserveSpace.as_mut_ptr() as *mut _,
            reserveSpaceSizeInBytes,
            groupCnt as _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnNormalizationBackward<
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
    F: types::CudaAsPtr,
    T13: types::CudaAsPtr,
    T14: types::CudaAsPtr,
    T15: types::CudaAsPtr,
    T16: types::CudaAsPtr,
>(
    handle: cudnnHandle_t,
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
    groupCnt: i32,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnNormalizationBackward(
            handle,
            mode,
            normOps,
            algo,
            alphaDataDiff.as_const_ptr() as *const _,
            betaDataDiff.as_const_ptr() as *const _,
            alphaParamDiff.as_const_ptr() as *const _,
            betaParamDiff.as_const_ptr() as *const _,
            xDesc,
            xData.as_const_ptr() as *const _,
            yDesc,
            yData.as_const_ptr() as *const _,
            dyDesc,
            dyData.as_const_ptr() as *const _,
            dzDesc,
            dzData.as_mut_ptr() as *mut _,
            dxDesc,
            dxData.as_mut_ptr() as *mut _,
            dNormScaleBiasDesc,
            normScaleData.as_const_ptr() as *const _,
            normBiasData.as_const_ptr() as *const _,
            dNormScaleData.as_mut_ptr() as *mut _,
            dNormBiasData.as_mut_ptr() as *mut _,
            epsilon,
            normMeanVarDesc,
            savedMean.as_const_ptr() as *const _,
            savedInvVariance.as_const_ptr() as *const _,
            activationDesc,
            workSpace.as_mut_ptr() as *mut _,
            workSpaceSizeInBytes,
            reserveSpace.as_mut_ptr() as *mut _,
            reserveSpaceSizeInBytes,
            groupCnt as _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnSpatialTfGridGeneratorBackward<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    handle: cudnnHandle_t,
    stDesc: cudnnSpatialTransformerDescriptor_t,
    dgrid: T,
    mut dtheta: U,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnSpatialTfGridGeneratorBackward(
            handle,
            stDesc,
            dgrid.as_const_ptr() as *const _,
            dtheta.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnSpatialTfSamplerBackward<
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
    handle: cudnnHandle_t,
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
            handle,
            stDesc,
            alpha.as_const_ptr() as *const _,
            xDesc,
            x.as_const_ptr() as *const _,
            beta.as_const_ptr() as *const _,
            dxDesc,
            dx.as_mut_ptr() as *mut _,
            alphaDgrid.as_const_ptr() as *const _,
            dyDesc,
            dy.as_const_ptr() as *const _,
            grid.as_const_ptr() as *const _,
            betaDgrid.as_const_ptr() as *const _,
            dgrid.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnDropoutBackward<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cudnnHandle_t,
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
            handle,
            dropoutDesc,
            dydesc,
            dy.as_const_ptr() as *const _,
            dxdesc,
            dx.as_mut_ptr() as *mut _,
            reserveSpace.as_mut_ptr() as *mut _,
            reserveSpaceSizeInBytes,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnCreateRNNDescriptor() -> Result<cudnnRNNDescriptor_t, crate::sys::cudnnStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cudnnRNNDescriptor_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudnnCreateRNNDescriptor(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cudnnRNNDescriptor_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnDestroyRNNDescriptor(rnnDesc: cudnnRNNDescriptor_t) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnDestroyRNNDescriptor(rnnDesc) };
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
    let mut out_13: std::mem::MaybeUninit<cudnnDropoutDescriptor_t> = std::mem::MaybeUninit::uninit();
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
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe {
            Ok((
                out_1.assume_init() as cudnnRNNAlgo_t,
                out_2.assume_init() as cudnnRNNMode_t,
                out_3.assume_init() as cudnnRNNBiasMode_t,
                out_4.assume_init() as cudnnDirectionMode_t,
                out_5.assume_init() as cudnnRNNInputMode_t,
                out_6.assume_init() as cudnnDataType_t,
                out_7.assume_init() as cudnnDataType_t,
                out_8.assume_init() as cudnnMathType_t,
                out_9.assume_init() as i32,
                out_10.assume_init() as i32,
                out_11.assume_init() as i32,
                out_12.assume_init() as i32,
                out_13.assume_init() as cudnnDropoutDescriptor_t,
                out_14.assume_init() as u32,
            ))
        }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnRNNSetClip_v8(
    rnnDesc: cudnnRNNDescriptor_t,
    clipMode: cudnnRNNClipMode_t,
    clipNanOpt: cudnnNanPropagation_t,
    lclip: f64,
    rclip: f64,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnRNNSetClip_v8(rnnDesc, clipMode, clipNanOpt, lclip, rclip) };
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
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe {
            Ok((
                out_1.assume_init() as cudnnRNNClipMode_t,
                out_2.assume_init() as cudnnNanPropagation_t,
                out_3.assume_init() as f64,
                out_4.assume_init() as f64,
            ))
        }
    } else {
        Err(unsafe { std::mem::transmute(status) })
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
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe {
            Ok((
                out_1.assume_init() as cudnnRNNClipMode_t,
                out_2.assume_init() as f64,
                out_3.assume_init() as f64,
            ))
        }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnBuildRNNDynamic(
    handle: cudnnHandle_t,
    rnnDesc: cudnnRNNDescriptor_t,
    miniBatch: i32,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnBuildRNNDynamic(handle, rnnDesc, miniBatch as _) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnGetRNNTempSpaceSizes(
    handle: cudnnHandle_t,
    rnnDesc: cudnnRNNDescriptor_t,
    fwdMode: cudnnForwardMode_t,
    xDesc: cudnnRNNDataDescriptor_t,
) -> Result<(usize, usize), crate::sys::cudnnStatus_t> {
    let mut out_4: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let mut out_5: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudnnGetRNNTempSpaceSizes(
            handle,
            rnnDesc,
            fwdMode,
            xDesc,
            out_4.as_mut_ptr() as *mut _,
            out_5.as_mut_ptr() as *mut _,
        )
    };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok((out_4.assume_init() as usize, out_5.assume_init() as usize)) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnGetRNNWeightSpaceSize(
    handle: cudnnHandle_t,
    rnnDesc: cudnnRNNDescriptor_t,
) -> Result<usize, crate::sys::cudnnStatus_t> {
    let mut out_2: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudnnGetRNNWeightSpaceSize(handle, rnnDesc, out_2.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_2.assume_init() as usize) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnGetRNNWeightParams<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    handle: cudnnHandle_t,
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
            handle,
            rnnDesc,
            pseudoLayer,
            weightSpaceSize,
            weightSpace.as_const_ptr() as *const _,
            linLayerID,
            mDesc,
            mAddr.as_mut_ptr() as *mut _,
            bDesc,
            bAddr.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnCreateRNNDataDescriptor() -> Result<cudnnRNNDataDescriptor_t, crate::sys::cudnnStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cudnnRNNDataDescriptor_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudnnCreateRNNDataDescriptor(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cudnnRNNDataDescriptor_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnDestroyRNNDataDescriptor(
    rnnDataDesc: cudnnRNNDataDescriptor_t,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnDestroyRNNDataDescriptor(rnnDataDesc) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnSetRNNDataDescriptor<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    rnnDataDesc: cudnnRNNDataDescriptor_t,
    dataType: cudnnDataType_t,
    layout: cudnnRNNDataLayout_t,
    maxSeqLength: i32,
    batchSize: i32,
    vectorSize: i32,
    seqLengthArray: T,
    mut paddingFill: U,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnSetRNNDataDescriptor(
            rnnDataDesc,
            dataType,
            layout,
            maxSeqLength as _,
            batchSize as _,
            vectorSize as _,
            seqLengthArray.as_const_ptr() as *const _,
            paddingFill.as_mut_ptr() as *mut _,
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
    arrayLengthRequested: i32,
    seqLengthArray: *mut ::std::os::raw::c_int,
    paddingFill: *mut ::std::os::raw::c_void,
) -> Result<(cudnnDataType_t, cudnnRNNDataLayout_t, i32, i32, i32), crate::sys::cudnnStatus_t> {
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
            arrayLengthRequested as _,
            seqLengthArray,
            paddingFill,
        )
    };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe {
            Ok((
                out_1.assume_init() as cudnnDataType_t,
                out_2.assume_init() as cudnnRNNDataLayout_t,
                out_3.assume_init() as i32,
                out_4.assume_init() as i32,
                out_5.assume_init() as i32,
            ))
        }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnRNNForward<
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
>(
    handle: cudnnHandle_t,
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
            handle,
            rnnDesc,
            fwdMode,
            devSeqLengths.as_const_ptr() as *const _,
            xDesc,
            x.as_const_ptr() as *const _,
            yDesc,
            y.as_mut_ptr() as *mut _,
            hDesc,
            hx.as_const_ptr() as *const _,
            hy.as_mut_ptr() as *mut _,
            cDesc,
            cx.as_const_ptr() as *const _,
            cy.as_mut_ptr() as *mut _,
            weightSpaceSize,
            weightSpace.as_const_ptr() as *const _,
            workSpaceSize,
            workSpace.as_mut_ptr() as *mut _,
            reserveSpaceSize,
            reserveSpace.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnCreateSeqDataDescriptor() -> Result<cudnnSeqDataDescriptor_t, crate::sys::cudnnStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cudnnSeqDataDescriptor_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudnnCreateSeqDataDescriptor(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cudnnSeqDataDescriptor_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnDestroySeqDataDescriptor(
    seqDataDesc: cudnnSeqDataDescriptor_t,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnDestroySeqDataDescriptor(seqDataDesc) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnSetSeqDataDescriptor<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    seqDataDesc: cudnnSeqDataDescriptor_t,
    dataType: cudnnDataType_t,
    nbDims: i32,
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
            nbDims as _,
            dimA.as_const_ptr() as *const _,
            axes.as_const_ptr() as *const _,
            seqLengthArraySize,
            seqLengthArray.as_const_ptr() as *const _,
            paddingFill.as_mut_ptr() as *mut _,
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
    nbDimsRequested: i32,
    seqLengthArraySize: *mut usize,
    seqLengthSizeRequested: usize,
    seqLengthArray: *mut ::std::os::raw::c_int,
    paddingFill: *mut ::std::os::raw::c_void,
) -> Result<(cudnnDataType_t, i32, i32, cudnnSeqDataAxis_t), crate::sys::cudnnStatus_t> {
    let mut out_1: std::mem::MaybeUninit<cudnnDataType_t> = std::mem::MaybeUninit::uninit();
    let mut out_2: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_4: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_5: std::mem::MaybeUninit<cudnnSeqDataAxis_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudnnGetSeqDataDescriptor(
            seqDataDesc,
            out_1.as_mut_ptr() as *mut _,
            out_2.as_mut_ptr() as *mut _,
            nbDimsRequested as _,
            out_4.as_mut_ptr() as *mut _,
            out_5.as_mut_ptr() as *mut _,
            seqLengthArraySize,
            seqLengthSizeRequested,
            seqLengthArray,
            paddingFill,
        )
    };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe {
            Ok((
                out_1.assume_init() as cudnnDataType_t,
                out_2.assume_init() as i32,
                out_4.assume_init() as i32,
                out_5.assume_init() as cudnnSeqDataAxis_t,
            ))
        }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnCreateAttnDescriptor() -> Result<cudnnAttnDescriptor_t, crate::sys::cudnnStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cudnnAttnDescriptor_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudnnCreateAttnDescriptor(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cudnnAttnDescriptor_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnDestroyAttnDescriptor(attnDesc: cudnnAttnDescriptor_t) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnDestroyAttnDescriptor(attnDesc) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnSetAttnDescriptor(
    attnDesc: cudnnAttnDescriptor_t,
    attnMode: u32,
    nHeads: i32,
    smScaler: f64,
    dataType: cudnnDataType_t,
    computePrec: cudnnDataType_t,
    mathType: cudnnMathType_t,
    attnDropoutDesc: cudnnDropoutDescriptor_t,
    postDropoutDesc: cudnnDropoutDescriptor_t,
    qSize: i32,
    kSize: i32,
    vSize: i32,
    qProjSize: i32,
    kProjSize: i32,
    vProjSize: i32,
    oProjSize: i32,
    qoMaxSeqLength: i32,
    kvMaxSeqLength: i32,
    maxBatchSize: i32,
    maxBeamSize: i32,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnSetAttnDescriptor(
            attnDesc,
            attnMode as _,
            nHeads as _,
            smScaler,
            dataType,
            computePrec,
            mathType,
            attnDropoutDesc,
            postDropoutDesc,
            qSize as _,
            kSize as _,
            vSize as _,
            qProjSize as _,
            kProjSize as _,
            vProjSize as _,
            oProjSize as _,
            qoMaxSeqLength as _,
            kvMaxSeqLength as _,
            maxBatchSize as _,
            maxBeamSize as _,
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
        u32,
        i32,
        f64,
        cudnnDataType_t,
        cudnnDataType_t,
        cudnnMathType_t,
        cudnnDropoutDescriptor_t,
        cudnnDropoutDescriptor_t,
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
    ),
    crate::sys::cudnnStatus_t,
> {
    let mut out_1: std::mem::MaybeUninit<::std::os::raw::c_uint> = std::mem::MaybeUninit::uninit();
    let mut out_2: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_3: std::mem::MaybeUninit<f64> = std::mem::MaybeUninit::uninit();
    let mut out_4: std::mem::MaybeUninit<cudnnDataType_t> = std::mem::MaybeUninit::uninit();
    let mut out_5: std::mem::MaybeUninit<cudnnDataType_t> = std::mem::MaybeUninit::uninit();
    let mut out_6: std::mem::MaybeUninit<cudnnMathType_t> = std::mem::MaybeUninit::uninit();
    let mut out_7: std::mem::MaybeUninit<cudnnDropoutDescriptor_t> = std::mem::MaybeUninit::uninit();
    let mut out_8: std::mem::MaybeUninit<cudnnDropoutDescriptor_t> = std::mem::MaybeUninit::uninit();
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
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe {
            Ok((
                out_1.assume_init() as u32,
                out_2.assume_init() as i32,
                out_3.assume_init() as f64,
                out_4.assume_init() as cudnnDataType_t,
                out_5.assume_init() as cudnnDataType_t,
                out_6.assume_init() as cudnnMathType_t,
                out_7.assume_init() as cudnnDropoutDescriptor_t,
                out_8.assume_init() as cudnnDropoutDescriptor_t,
                out_9.assume_init() as i32,
                out_10.assume_init() as i32,
                out_11.assume_init() as i32,
                out_12.assume_init() as i32,
                out_13.assume_init() as i32,
                out_14.assume_init() as i32,
                out_15.assume_init() as i32,
                out_16.assume_init() as i32,
                out_17.assume_init() as i32,
                out_18.assume_init() as i32,
                out_19.assume_init() as i32,
            ))
        }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnGetMultiHeadAttnBuffers(
    handle: cudnnHandle_t,
    attnDesc: cudnnAttnDescriptor_t,
) -> Result<(usize, usize, usize), crate::sys::cudnnStatus_t> {
    let mut out_2: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let mut out_3: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let mut out_4: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudnnGetMultiHeadAttnBuffers(
            handle,
            attnDesc,
            out_2.as_mut_ptr() as *mut _,
            out_3.as_mut_ptr() as *mut _,
            out_4.as_mut_ptr() as *mut _,
        )
    };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe {
            Ok((
                out_2.assume_init() as usize,
                out_3.assume_init() as usize,
                out_4.assume_init() as usize,
            ))
        }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnGetMultiHeadAttnWeights<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    handle: cudnnHandle_t,
    attnDesc: cudnnAttnDescriptor_t,
    wKind: cudnnMultiHeadAttnWeightKind_t,
    weightSizeInBytes: usize,
    weights: T,
    wDesc: cudnnTensorDescriptor_t,
    mut wAddr: U,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnGetMultiHeadAttnWeights(
            handle,
            attnDesc,
            wKind,
            weightSizeInBytes,
            weights.as_const_ptr() as *const _,
            wDesc,
            wAddr.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnMultiHeadAttnForward<
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
    handle: cudnnHandle_t,
    attnDesc: cudnnAttnDescriptor_t,
    currIdx: i32,
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
            handle,
            attnDesc,
            currIdx as _,
            loWinIdx.as_const_ptr() as *const _,
            hiWinIdx.as_const_ptr() as *const _,
            devSeqLengthsQO.as_const_ptr() as *const _,
            devSeqLengthsKV.as_const_ptr() as *const _,
            qDesc,
            queries.as_const_ptr() as *const _,
            residuals.as_const_ptr() as *const _,
            kDesc,
            keys.as_const_ptr() as *const _,
            vDesc,
            values.as_const_ptr() as *const _,
            oDesc,
            out.as_mut_ptr() as *mut _,
            weightSizeInBytes,
            weights.as_const_ptr() as *const _,
            workSpaceSizeInBytes,
            workSpace.as_mut_ptr() as *mut _,
            reserveSpaceSizeInBytes,
            reserveSpace.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
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
pub unsafe fn cudnnRNNBackwardData_v8<
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
    F: types::CudaAsPtr,
>(
    handle: cudnnHandle_t,
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
            handle,
            rnnDesc,
            devSeqLengths.as_const_ptr() as *const _,
            yDesc,
            y.as_const_ptr() as *const _,
            dy.as_const_ptr() as *const _,
            xDesc,
            dx.as_mut_ptr() as *mut _,
            hDesc,
            hx.as_const_ptr() as *const _,
            dhy.as_const_ptr() as *const _,
            dhx.as_mut_ptr() as *mut _,
            cDesc,
            cx.as_const_ptr() as *const _,
            dcy.as_const_ptr() as *const _,
            dcx.as_mut_ptr() as *mut _,
            weightSpaceSize,
            weightSpace.as_const_ptr() as *const _,
            workSpaceSize,
            workSpace.as_mut_ptr() as *mut _,
            reserveSpaceSize,
            reserveSpace.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnRNNBackwardWeights_v8<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cudnnHandle_t,
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
            handle,
            rnnDesc,
            addGrad,
            devSeqLengths.as_const_ptr() as *const _,
            xDesc,
            x.as_const_ptr() as *const _,
            hDesc,
            hx.as_const_ptr() as *const _,
            yDesc,
            y.as_const_ptr() as *const _,
            weightSpaceSize,
            dweightSpace.as_mut_ptr() as *mut _,
            workSpaceSize,
            workSpace.as_mut_ptr() as *mut _,
            reserveSpaceSize,
            reserveSpace.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnMultiHeadAttnBackwardData<
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
    F: types::CudaAsPtr,
    T13: types::CudaAsPtr,
>(
    handle: cudnnHandle_t,
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
            handle,
            attnDesc,
            loWinIdx.as_const_ptr() as *const _,
            hiWinIdx.as_const_ptr() as *const _,
            devSeqLengthsDQDO.as_const_ptr() as *const _,
            devSeqLengthsDKDV.as_const_ptr() as *const _,
            doDesc,
            dout.as_const_ptr() as *const _,
            dqDesc,
            dqueries.as_mut_ptr() as *mut _,
            queries.as_const_ptr() as *const _,
            dkDesc,
            dkeys.as_mut_ptr() as *mut _,
            keys.as_const_ptr() as *const _,
            dvDesc,
            dvalues.as_mut_ptr() as *mut _,
            values.as_const_ptr() as *const _,
            weightSizeInBytes,
            weights.as_const_ptr() as *const _,
            workSpaceSizeInBytes,
            workSpace.as_mut_ptr() as *mut _,
            reserveSpaceSizeInBytes,
            reserveSpace.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnMultiHeadAttnBackwardWeights<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
    A: types::CudaAsPtr,
>(
    handle: cudnnHandle_t,
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
            handle,
            attnDesc,
            addGrad,
            qDesc,
            queries.as_const_ptr() as *const _,
            kDesc,
            keys.as_const_ptr() as *const _,
            vDesc,
            values.as_const_ptr() as *const _,
            doDesc,
            dout.as_const_ptr() as *const _,
            weightSizeInBytes,
            weights.as_const_ptr() as *const _,
            dweights.as_mut_ptr() as *mut _,
            workSpaceSizeInBytes,
            workSpace.as_mut_ptr() as *mut _,
            reserveSpaceSizeInBytes,
            reserveSpace.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnCreateCTCLossDescriptor() -> Result<cudnnCTCLossDescriptor_t, crate::sys::cudnnStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cudnnCTCLossDescriptor_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudnnCreateCTCLossDescriptor(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cudnnCTCLossDescriptor_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
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
    let status = unsafe { crate::sys::cudnnSetCTCLossDescriptorEx(ctcLossDesc, compType, normMode, gradMode) };
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
    maxLabelLength: i32,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnSetCTCLossDescriptor_v8(ctcLossDesc, compType, normMode, gradMode, maxLabelLength as _)
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
    maxLabelLength: i32,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnSetCTCLossDescriptor_v9(ctcLossDesc, compType, normMode, ctcGradMode, maxLabelLength as _)
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
    let status = unsafe { crate::sys::cudnnGetCTCLossDescriptor(ctcLossDesc, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as cudnnDataType_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnGetCTCLossDescriptorEx(
    ctcLossDesc: cudnnCTCLossDescriptor_t,
) -> Result<(cudnnDataType_t, cudnnLossNormalizationMode_t, cudnnNanPropagation_t), crate::sys::cudnnStatus_t> {
    let mut out_1: std::mem::MaybeUninit<cudnnDataType_t> = std::mem::MaybeUninit::uninit();
    let mut out_2: std::mem::MaybeUninit<cudnnLossNormalizationMode_t> = std::mem::MaybeUninit::uninit();
    let mut out_3: std::mem::MaybeUninit<cudnnNanPropagation_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudnnGetCTCLossDescriptorEx(
            ctcLossDesc,
            out_1.as_mut_ptr() as *mut _,
            out_2.as_mut_ptr() as *mut _,
            out_3.as_mut_ptr() as *mut _,
        )
    };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe {
            Ok((
                out_1.assume_init() as cudnnDataType_t,
                out_2.assume_init() as cudnnLossNormalizationMode_t,
                out_3.assume_init() as cudnnNanPropagation_t,
            ))
        }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnGetCTCLossDescriptor_v8(
    ctcLossDesc: cudnnCTCLossDescriptor_t,
) -> Result<
    (
        cudnnDataType_t,
        cudnnLossNormalizationMode_t,
        cudnnNanPropagation_t,
        i32,
    ),
    crate::sys::cudnnStatus_t,
> {
    let mut out_1: std::mem::MaybeUninit<cudnnDataType_t> = std::mem::MaybeUninit::uninit();
    let mut out_2: std::mem::MaybeUninit<cudnnLossNormalizationMode_t> = std::mem::MaybeUninit::uninit();
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
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe {
            Ok((
                out_1.assume_init() as cudnnDataType_t,
                out_2.assume_init() as cudnnLossNormalizationMode_t,
                out_3.assume_init() as cudnnNanPropagation_t,
                out_4.assume_init() as i32,
            ))
        }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnGetCTCLossDescriptor_v9(
    ctcLossDesc: cudnnCTCLossDescriptor_t,
) -> Result<(cudnnDataType_t, cudnnLossNormalizationMode_t, cudnnCTCGradMode_t, i32), crate::sys::cudnnStatus_t> {
    let mut out_1: std::mem::MaybeUninit<cudnnDataType_t> = std::mem::MaybeUninit::uninit();
    let mut out_2: std::mem::MaybeUninit<cudnnLossNormalizationMode_t> = std::mem::MaybeUninit::uninit();
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
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe {
            Ok((
                out_1.assume_init() as cudnnDataType_t,
                out_2.assume_init() as cudnnLossNormalizationMode_t,
                out_3.assume_init() as cudnnCTCGradMode_t,
                out_4.assume_init() as i32,
            ))
        }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnDestroyCTCLossDescriptor(
    ctcLossDesc: cudnnCTCLossDescriptor_t,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnDestroyCTCLossDescriptor(ctcLossDesc) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnCTCLoss<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cudnnHandle_t,
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
            handle,
            probsDesc,
            probs.as_const_ptr() as *const _,
            hostLabels.as_const_ptr() as *const _,
            hostLabelLengths.as_const_ptr() as *const _,
            hostInputLengths.as_const_ptr() as *const _,
            costs.as_mut_ptr() as *mut _,
            gradientsDesc,
            gradients.as_mut_ptr() as *mut _,
            algo,
            ctcLossDesc,
            workspace.as_mut_ptr() as *mut _,
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
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
>(
    handle: cudnnHandle_t,
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
            handle,
            algo,
            ctcLossDesc,
            probsDesc,
            probs.as_const_ptr() as *const _,
            labels.as_const_ptr() as *const _,
            labelLengths.as_const_ptr() as *const _,
            inputLengths.as_const_ptr() as *const _,
            costs.as_mut_ptr() as *mut _,
            gradientsDesc,
            gradients.as_mut_ptr() as *mut _,
            workSpaceSizeInBytes,
            workspace.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnGetCTCLossWorkspaceSize(
    handle: cudnnHandle_t,
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
            handle,
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
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_8.assume_init() as usize) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnGetCTCLossWorkspaceSize_v8(
    handle: cudnnHandle_t,
    algo: cudnnCTCLossAlgo_t,
    ctcLossDesc: cudnnCTCLossDescriptor_t,
    probsDesc: cudnnTensorDescriptor_t,
    gradientsDesc: cudnnTensorDescriptor_t,
) -> Result<usize, crate::sys::cudnnStatus_t> {
    let mut out_5: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudnnGetCTCLossWorkspaceSize_v8(
            handle,
            algo,
            ctcLossDesc,
            probsDesc,
            gradientsDesc,
            out_5.as_mut_ptr() as *mut _,
        )
    };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_5.assume_init() as usize) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnCreateConvolutionDescriptor() -> Result<cudnnConvolutionDescriptor_t, crate::sys::cudnnStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cudnnConvolutionDescriptor_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudnnCreateConvolutionDescriptor(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cudnnConvolutionDescriptor_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnDestroyConvolutionDescriptor(
    convDesc: cudnnConvolutionDescriptor_t,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnDestroyConvolutionDescriptor(convDesc) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
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
    let status = unsafe { crate::sys::cudnnGetConvolutionMathType(convDesc, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as cudnnMathType_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnSetConvolutionGroupCount(
    convDesc: cudnnConvolutionDescriptor_t,
    groupCount: i32,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnSetConvolutionGroupCount(convDesc, groupCount as _) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnGetConvolutionGroupCount(
    convDesc: cudnnConvolutionDescriptor_t,
) -> Result<i32, crate::sys::cudnnStatus_t> {
    let mut out_1: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudnnGetConvolutionGroupCount(convDesc, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as i32) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
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
    let status = unsafe { crate::sys::cudnnGetConvolutionReorderType(convDesc, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as cudnnReorderType_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnSetConvolution2dDescriptor(
    convDesc: cudnnConvolutionDescriptor_t,
    pad_h: i32,
    pad_w: i32,
    u: i32,
    v: i32,
    dilation_h: i32,
    dilation_w: i32,
    mode: cudnnConvolutionMode_t,
    computeType: cudnnDataType_t,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnSetConvolution2dDescriptor(
            convDesc,
            pad_h as _,
            pad_w as _,
            u as _,
            v as _,
            dilation_h as _,
            dilation_w as _,
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
) -> Result<(i32, i32, i32, i32, i32, i32, cudnnConvolutionMode_t, cudnnDataType_t), crate::sys::cudnnStatus_t> {
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
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe {
            Ok((
                out_1.assume_init() as i32,
                out_2.assume_init() as i32,
                out_3.assume_init() as i32,
                out_4.assume_init() as i32,
                out_5.assume_init() as i32,
                out_6.assume_init() as i32,
                out_7.assume_init() as cudnnConvolutionMode_t,
                out_8.assume_init() as cudnnDataType_t,
            ))
        }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnSetConvolutionNdDescriptor<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    convDesc: cudnnConvolutionDescriptor_t,
    arrayLength: i32,
    padA: T,
    filterStrideA: U,
    dilationA: V,
    mode: cudnnConvolutionMode_t,
    computeType: cudnnDataType_t,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnSetConvolutionNdDescriptor(
            convDesc,
            arrayLength as _,
            padA.as_const_ptr() as *const _,
            filterStrideA.as_const_ptr() as *const _,
            dilationA.as_const_ptr() as *const _,
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
    arrayLengthRequested: i32,
) -> Result<(i32, i32, i32, i32, cudnnConvolutionMode_t, cudnnDataType_t), crate::sys::cudnnStatus_t> {
    let mut out_2: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_3: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_4: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_5: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_6: std::mem::MaybeUninit<cudnnConvolutionMode_t> = std::mem::MaybeUninit::uninit();
    let mut out_7: std::mem::MaybeUninit<cudnnDataType_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudnnGetConvolutionNdDescriptor(
            convDesc,
            arrayLengthRequested as _,
            out_2.as_mut_ptr() as *mut _,
            out_3.as_mut_ptr() as *mut _,
            out_4.as_mut_ptr() as *mut _,
            out_5.as_mut_ptr() as *mut _,
            out_6.as_mut_ptr() as *mut _,
            out_7.as_mut_ptr() as *mut _,
        )
    };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe {
            Ok((
                out_2.assume_init() as i32,
                out_3.assume_init() as i32,
                out_4.assume_init() as i32,
                out_5.assume_init() as i32,
                out_6.assume_init() as cudnnConvolutionMode_t,
                out_7.assume_init() as cudnnDataType_t,
            ))
        }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnGetConvolution2dForwardOutputDim(
    convDesc: cudnnConvolutionDescriptor_t,
    inputTensorDesc: cudnnTensorDescriptor_t,
    filterDesc: cudnnFilterDescriptor_t,
) -> Result<(i32, i32, i32, i32), crate::sys::cudnnStatus_t> {
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
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe {
            Ok((
                out_3.assume_init() as i32,
                out_4.assume_init() as i32,
                out_5.assume_init() as i32,
                out_6.assume_init() as i32,
            ))
        }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnGetConvolutionNdForwardOutputDim(
    convDesc: cudnnConvolutionDescriptor_t,
    inputTensorDesc: cudnnTensorDescriptor_t,
    filterDesc: cudnnFilterDescriptor_t,
    nbDims: i32,
) -> Result<i32, crate::sys::cudnnStatus_t> {
    let mut out_4: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudnnGetConvolutionNdForwardOutputDim(
            convDesc,
            inputTensorDesc,
            filterDesc,
            nbDims as _,
            out_4.as_mut_ptr() as *mut _,
        )
    };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_4.assume_init() as i32) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnGetConvolutionForwardAlgorithmMaxCount(
    handle: cudnnHandle_t,
) -> Result<i32, crate::sys::cudnnStatus_t> {
    let mut out_1: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status =
        unsafe { crate::sys::cudnnGetConvolutionForwardAlgorithmMaxCount(handle, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as i32) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnGetConvolutionForwardAlgorithm_v7(
    handle: cudnnHandle_t,
    srcDesc: cudnnTensorDescriptor_t,
    filterDesc: cudnnFilterDescriptor_t,
    convDesc: cudnnConvolutionDescriptor_t,
    destDesc: cudnnTensorDescriptor_t,
    requestedAlgoCount: i32,
) -> Result<(i32, cudnnConvolutionFwdAlgoPerf_t), crate::sys::cudnnStatus_t> {
    let mut out_6: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_7: std::mem::MaybeUninit<cudnnConvolutionFwdAlgoPerf_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudnnGetConvolutionForwardAlgorithm_v7(
            handle,
            srcDesc,
            filterDesc,
            convDesc,
            destDesc,
            requestedAlgoCount as _,
            out_6.as_mut_ptr() as *mut _,
            out_7.as_mut_ptr() as *mut _,
        )
    };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe {
            Ok((
                out_6.assume_init() as i32,
                out_7.assume_init() as cudnnConvolutionFwdAlgoPerf_t,
            ))
        }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnFindConvolutionForwardAlgorithm<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    handle: cudnnHandle_t,
    xDesc: cudnnTensorDescriptor_t,
    wDesc: cudnnFilterDescriptor_t,
    convDesc: cudnnConvolutionDescriptor_t,
    yDesc: cudnnTensorDescriptor_t,
    requestedAlgoCount: i32,
    mut returnedAlgoCount: T,
    mut perfResults: U,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnFindConvolutionForwardAlgorithm(
            handle,
            xDesc,
            wDesc,
            convDesc,
            yDesc,
            requestedAlgoCount as _,
            returnedAlgoCount.as_mut_ptr() as *mut _,
            perfResults.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnFindConvolutionForwardAlgorithmEx<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cudnnHandle_t,
    xDesc: cudnnTensorDescriptor_t,
    x: T,
    wDesc: cudnnFilterDescriptor_t,
    w: U,
    convDesc: cudnnConvolutionDescriptor_t,
    yDesc: cudnnTensorDescriptor_t,
    mut y: V,
    requestedAlgoCount: i32,
    mut returnedAlgoCount: W,
    mut perfResults: X,
    mut workSpace: Y,
    workSpaceSizeInBytes: usize,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnFindConvolutionForwardAlgorithmEx(
            handle,
            xDesc,
            x.as_const_ptr() as *const _,
            wDesc,
            w.as_const_ptr() as *const _,
            convDesc,
            yDesc,
            y.as_mut_ptr() as *mut _,
            requestedAlgoCount as _,
            returnedAlgoCount.as_mut_ptr() as *mut _,
            perfResults.as_mut_ptr() as *mut _,
            workSpace.as_mut_ptr() as *mut _,
            workSpaceSizeInBytes,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnIm2Col<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    handle: cudnnHandle_t,
    xDesc: cudnnTensorDescriptor_t,
    x: T,
    wDesc: cudnnFilterDescriptor_t,
    convDesc: cudnnConvolutionDescriptor_t,
    mut colBuffer: U,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnIm2Col(
            handle,
            xDesc,
            x.as_const_ptr() as *const _,
            wDesc,
            convDesc,
            colBuffer.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnReorderFilterAndBias<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cudnnHandle_t,
    filterDesc: cudnnFilterDescriptor_t,
    reorderType: cudnnReorderType_t,
    filterData: T,
    mut reorderedFilterData: U,
    reorderBias: i32,
    biasData: V,
    mut reorderedBiasData: W,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnReorderFilterAndBias(
            handle,
            filterDesc,
            reorderType,
            filterData.as_const_ptr() as *const _,
            reorderedFilterData.as_mut_ptr() as *mut _,
            reorderBias as _,
            biasData.as_const_ptr() as *const _,
            reorderedBiasData.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnGetConvolutionForwardWorkspaceSize(
    handle: cudnnHandle_t,
    xDesc: cudnnTensorDescriptor_t,
    wDesc: cudnnFilterDescriptor_t,
    convDesc: cudnnConvolutionDescriptor_t,
    yDesc: cudnnTensorDescriptor_t,
    algo: cudnnConvolutionFwdAlgo_t,
) -> Result<usize, crate::sys::cudnnStatus_t> {
    let mut out_6: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudnnGetConvolutionForwardWorkspaceSize(
            handle,
            xDesc,
            wDesc,
            convDesc,
            yDesc,
            algo,
            out_6.as_mut_ptr() as *mut _,
        )
    };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_6.assume_init() as usize) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnConvolutionForward<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cudnnHandle_t,
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
            handle,
            alpha.as_const_ptr() as *const _,
            xDesc,
            x.as_const_ptr() as *const _,
            wDesc,
            w.as_const_ptr() as *const _,
            convDesc,
            algo,
            workSpace.as_mut_ptr() as *mut _,
            workSpaceSizeInBytes,
            beta.as_const_ptr() as *const _,
            yDesc,
            y.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnConvolutionBiasActivationForward<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
    Z: types::CudaAsPtr,
    A: types::CudaAsPtr,
>(
    handle: cudnnHandle_t,
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
            handle,
            alpha1.as_const_ptr() as *const _,
            xDesc,
            x.as_const_ptr() as *const _,
            wDesc,
            w.as_const_ptr() as *const _,
            convDesc,
            algo,
            workSpace.as_mut_ptr() as *mut _,
            workSpaceSizeInBytes,
            alpha2.as_const_ptr() as *const _,
            zDesc,
            z.as_const_ptr() as *const _,
            biasDesc,
            bias.as_const_ptr() as *const _,
            activationDesc,
            yDesc,
            y.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnGetConvolutionBackwardDataAlgorithmMaxCount(
    handle: cudnnHandle_t,
) -> Result<i32, crate::sys::cudnnStatus_t> {
    let mut out_1: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status =
        unsafe { crate::sys::cudnnGetConvolutionBackwardDataAlgorithmMaxCount(handle, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as i32) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnFindConvolutionBackwardDataAlgorithm<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    handle: cudnnHandle_t,
    wDesc: cudnnFilterDescriptor_t,
    dyDesc: cudnnTensorDescriptor_t,
    convDesc: cudnnConvolutionDescriptor_t,
    dxDesc: cudnnTensorDescriptor_t,
    requestedAlgoCount: i32,
    mut returnedAlgoCount: T,
    mut perfResults: U,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnFindConvolutionBackwardDataAlgorithm(
            handle,
            wDesc,
            dyDesc,
            convDesc,
            dxDesc,
            requestedAlgoCount as _,
            returnedAlgoCount.as_mut_ptr() as *mut _,
            perfResults.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnFindConvolutionBackwardDataAlgorithmEx<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cudnnHandle_t,
    wDesc: cudnnFilterDescriptor_t,
    w: T,
    dyDesc: cudnnTensorDescriptor_t,
    dy: U,
    convDesc: cudnnConvolutionDescriptor_t,
    dxDesc: cudnnTensorDescriptor_t,
    mut dx: V,
    requestedAlgoCount: i32,
    mut returnedAlgoCount: W,
    mut perfResults: X,
    mut workSpace: Y,
    workSpaceSizeInBytes: usize,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnFindConvolutionBackwardDataAlgorithmEx(
            handle,
            wDesc,
            w.as_const_ptr() as *const _,
            dyDesc,
            dy.as_const_ptr() as *const _,
            convDesc,
            dxDesc,
            dx.as_mut_ptr() as *mut _,
            requestedAlgoCount as _,
            returnedAlgoCount.as_mut_ptr() as *mut _,
            perfResults.as_mut_ptr() as *mut _,
            workSpace.as_mut_ptr() as *mut _,
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
    handle: cudnnHandle_t,
    filterDesc: cudnnFilterDescriptor_t,
    diffDesc: cudnnTensorDescriptor_t,
    convDesc: cudnnConvolutionDescriptor_t,
    gradDesc: cudnnTensorDescriptor_t,
    requestedAlgoCount: i32,
) -> Result<(i32, cudnnConvolutionBwdDataAlgoPerf_t), crate::sys::cudnnStatus_t> {
    let mut out_6: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_7: std::mem::MaybeUninit<cudnnConvolutionBwdDataAlgoPerf_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudnnGetConvolutionBackwardDataAlgorithm_v7(
            handle,
            filterDesc,
            diffDesc,
            convDesc,
            gradDesc,
            requestedAlgoCount as _,
            out_6.as_mut_ptr() as *mut _,
            out_7.as_mut_ptr() as *mut _,
        )
    };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe {
            Ok((
                out_6.assume_init() as i32,
                out_7.assume_init() as cudnnConvolutionBwdDataAlgoPerf_t,
            ))
        }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnGetConvolutionBackwardDataWorkspaceSize(
    handle: cudnnHandle_t,
    wDesc: cudnnFilterDescriptor_t,
    dyDesc: cudnnTensorDescriptor_t,
    convDesc: cudnnConvolutionDescriptor_t,
    dxDesc: cudnnTensorDescriptor_t,
    algo: cudnnConvolutionBwdDataAlgo_t,
) -> Result<usize, crate::sys::cudnnStatus_t> {
    let mut out_6: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudnnGetConvolutionBackwardDataWorkspaceSize(
            handle,
            wDesc,
            dyDesc,
            convDesc,
            dxDesc,
            algo,
            out_6.as_mut_ptr() as *mut _,
        )
    };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_6.assume_init() as usize) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnConvolutionBackwardData<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cudnnHandle_t,
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
            handle,
            alpha.as_const_ptr() as *const _,
            wDesc,
            w.as_const_ptr() as *const _,
            dyDesc,
            dy.as_const_ptr() as *const _,
            convDesc,
            algo,
            workSpace.as_mut_ptr() as *mut _,
            workSpaceSizeInBytes,
            beta.as_const_ptr() as *const _,
            dxDesc,
            dx.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnGetFoldedConvBackwardDataDescriptors(
    handle: cudnnHandle_t,
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
            handle,
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
pub unsafe fn cudnnCnnVersionCheck() -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnCnnVersionCheck() };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnGetConvolutionBackwardFilterAlgorithmMaxCount(
    handle: cudnnHandle_t,
) -> Result<i32, crate::sys::cudnnStatus_t> {
    let mut out_1: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status =
        unsafe { crate::sys::cudnnGetConvolutionBackwardFilterAlgorithmMaxCount(handle, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as i32) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnFindConvolutionBackwardFilterAlgorithm<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    handle: cudnnHandle_t,
    xDesc: cudnnTensorDescriptor_t,
    dyDesc: cudnnTensorDescriptor_t,
    convDesc: cudnnConvolutionDescriptor_t,
    dwDesc: cudnnFilterDescriptor_t,
    requestedAlgoCount: i32,
    mut returnedAlgoCount: T,
    mut perfResults: U,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnFindConvolutionBackwardFilterAlgorithm(
            handle,
            xDesc,
            dyDesc,
            convDesc,
            dwDesc,
            requestedAlgoCount as _,
            returnedAlgoCount.as_mut_ptr() as *mut _,
            perfResults.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnFindConvolutionBackwardFilterAlgorithmEx<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cudnnHandle_t,
    xDesc: cudnnTensorDescriptor_t,
    x: T,
    dyDesc: cudnnTensorDescriptor_t,
    y: U,
    convDesc: cudnnConvolutionDescriptor_t,
    dwDesc: cudnnFilterDescriptor_t,
    mut dw: V,
    requestedAlgoCount: i32,
    mut returnedAlgoCount: W,
    mut perfResults: X,
    mut workSpace: Y,
    workSpaceSizeInBytes: usize,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnFindConvolutionBackwardFilterAlgorithmEx(
            handle,
            xDesc,
            x.as_const_ptr() as *const _,
            dyDesc,
            y.as_const_ptr() as *const _,
            convDesc,
            dwDesc,
            dw.as_mut_ptr() as *mut _,
            requestedAlgoCount as _,
            returnedAlgoCount.as_mut_ptr() as *mut _,
            perfResults.as_mut_ptr() as *mut _,
            workSpace.as_mut_ptr() as *mut _,
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
    handle: cudnnHandle_t,
    srcDesc: cudnnTensorDescriptor_t,
    diffDesc: cudnnTensorDescriptor_t,
    convDesc: cudnnConvolutionDescriptor_t,
    gradDesc: cudnnFilterDescriptor_t,
    requestedAlgoCount: i32,
) -> Result<(i32, cudnnConvolutionBwdFilterAlgoPerf_t), crate::sys::cudnnStatus_t> {
    let mut out_6: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_7: std::mem::MaybeUninit<cudnnConvolutionBwdFilterAlgoPerf_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudnnGetConvolutionBackwardFilterAlgorithm_v7(
            handle,
            srcDesc,
            diffDesc,
            convDesc,
            gradDesc,
            requestedAlgoCount as _,
            out_6.as_mut_ptr() as *mut _,
            out_7.as_mut_ptr() as *mut _,
        )
    };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe {
            Ok((
                out_6.assume_init() as i32,
                out_7.assume_init() as cudnnConvolutionBwdFilterAlgoPerf_t,
            ))
        }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnGetConvolutionBackwardFilterWorkspaceSize(
    handle: cudnnHandle_t,
    xDesc: cudnnTensorDescriptor_t,
    dyDesc: cudnnTensorDescriptor_t,
    convDesc: cudnnConvolutionDescriptor_t,
    gradDesc: cudnnFilterDescriptor_t,
    algo: cudnnConvolutionBwdFilterAlgo_t,
) -> Result<usize, crate::sys::cudnnStatus_t> {
    let mut out_6: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudnnGetConvolutionBackwardFilterWorkspaceSize(
            handle,
            xDesc,
            dyDesc,
            convDesc,
            gradDesc,
            algo,
            out_6.as_mut_ptr() as *mut _,
        )
    };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_6.assume_init() as usize) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnConvolutionBackwardFilter<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    handle: cudnnHandle_t,
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
            handle,
            alpha.as_const_ptr() as *const _,
            xDesc,
            x.as_const_ptr() as *const _,
            dyDesc,
            dy.as_const_ptr() as *const _,
            convDesc,
            algo,
            workSpace.as_mut_ptr() as *mut _,
            workSpaceSizeInBytes,
            beta.as_const_ptr() as *const _,
            dwDesc,
            dw.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnConvolutionBackwardBias<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    handle: cudnnHandle_t,
    alpha: T,
    dyDesc: cudnnTensorDescriptor_t,
    dy: U,
    beta: V,
    dbDesc: cudnnTensorDescriptor_t,
    mut db: W,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnConvolutionBackwardBias(
            handle,
            alpha.as_const_ptr() as *const _,
            dyDesc,
            dy.as_const_ptr() as *const _,
            beta.as_const_ptr() as *const _,
            dbDesc,
            db.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnCreateFusedOpsConstParamPack(
    ops: cudnnFusedOps_t,
) -> Result<cudnnFusedOpsConstParamPack_t, crate::sys::cudnnStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cudnnFusedOpsConstParamPack_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudnnCreateFusedOpsConstParamPack(out_0.as_mut_ptr() as *mut _, ops) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cudnnFusedOpsConstParamPack_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnDestroyFusedOpsConstParamPack(
    constPack: cudnnFusedOpsConstParamPack_t,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnDestroyFusedOpsConstParamPack(constPack) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnSetFusedOpsConstParamPackAttribute<T: types::CudaAsPtr>(
    constPack: cudnnFusedOpsConstParamPack_t,
    paramLabel: cudnnFusedOpsConstParamLabel_t,
    param: T,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnSetFusedOpsConstParamPackAttribute(constPack, paramLabel, param.as_const_ptr() as *const _)
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
) -> Result<i32, crate::sys::cudnnStatus_t> {
    let mut out_3: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudnnGetFusedOpsConstParamPackAttribute(constPack, paramLabel, param, out_3.as_mut_ptr() as *mut _)
    };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_3.assume_init() as i32) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnCreateFusedOpsVariantParamPack(
    ops: cudnnFusedOps_t,
) -> Result<cudnnFusedOpsVariantParamPack_t, crate::sys::cudnnStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cudnnFusedOpsVariantParamPack_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudnnCreateFusedOpsVariantParamPack(out_0.as_mut_ptr() as *mut _, ops) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cudnnFusedOpsVariantParamPack_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnDestroyFusedOpsVariantParamPack(
    varPack: cudnnFusedOpsVariantParamPack_t,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnDestroyFusedOpsVariantParamPack(varPack) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnSetFusedOpsVariantParamPackAttribute<T: types::CudaAsPtr>(
    varPack: cudnnFusedOpsVariantParamPack_t,
    paramLabel: cudnnFusedOpsVariantParamLabel_t,
    mut ptr: T,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnSetFusedOpsVariantParamPackAttribute(varPack, paramLabel, ptr.as_mut_ptr() as *mut _)
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnGetFusedOpsVariantParamPackAttribute<T: types::CudaAsPtr>(
    varPack: cudnnFusedOpsVariantParamPack_t,
    paramLabel: cudnnFusedOpsVariantParamLabel_t,
    mut ptr: T,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnGetFusedOpsVariantParamPackAttribute(varPack, paramLabel, ptr.as_mut_ptr() as *mut _)
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnCreateFusedOpsPlan(ops: cudnnFusedOps_t) -> Result<cudnnFusedOpsPlan_t, crate::sys::cudnnStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cudnnFusedOpsPlan_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudnnCreateFusedOpsPlan(out_0.as_mut_ptr() as *mut _, ops) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cudnnFusedOpsPlan_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnDestroyFusedOpsPlan(plan: cudnnFusedOpsPlan_t) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnDestroyFusedOpsPlan(plan) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudnnMakeFusedOpsPlan(
    handle: cudnnHandle_t,
    plan: cudnnFusedOpsPlan_t,
    constPack: cudnnFusedOpsConstParamPack_t,
) -> Result<usize, crate::sys::cudnnStatus_t> {
    let mut out_3: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudnnMakeFusedOpsPlan(handle, plan, constPack, out_3.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_3.assume_init() as usize) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudnnFusedOpsExecute(
    handle: cudnnHandle_t,
    plan: cudnnFusedOpsPlan_t,
    varPack: cudnnFusedOpsVariantParamPack_t,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnFusedOpsExecute(handle, plan, varPack) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
