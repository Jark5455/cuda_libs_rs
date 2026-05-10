#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unsafe_op_in_unsafe_fn)]
pub use crate::sys::CUDNN_ADV_MAJOR;
pub use crate::sys::CUDNN_ADV_MINOR;
pub use crate::sys::CUDNN_ADV_PATCH;
pub use crate::sys::CUDNN_ATTN_DISABLE_PROJ_BIASES;
pub use crate::sys::CUDNN_ATTN_ENABLE_PROJ_BIASES;
pub use crate::sys::CUDNN_ATTN_QUERYMAP_ALL_TO_ONE;
pub use crate::sys::CUDNN_ATTN_QUERYMAP_ONE_TO_ONE;
pub use crate::sys::CUDNN_ATTN_WKIND_COUNT;
pub use crate::sys::CUDNN_BN_MIN_EPSILON;
pub use crate::sys::CUDNN_CNN_MAJOR;
pub use crate::sys::CUDNN_CNN_MINOR;
pub use crate::sys::CUDNN_CNN_PATCH;
pub use crate::sys::CUDNN_DIM_MAX;
pub use crate::sys::CUDNN_GRAPH_MAJOR;
pub use crate::sys::CUDNN_GRAPH_MINOR;
pub use crate::sys::CUDNN_GRAPH_PATCH;
pub use crate::sys::CUDNN_LRN_MAX_N;
pub use crate::sys::CUDNN_LRN_MIN_BETA;
pub use crate::sys::CUDNN_LRN_MIN_K;
pub use crate::sys::CUDNN_LRN_MIN_N;
pub use crate::sys::CUDNN_MAJOR;
pub use crate::sys::CUDNN_MAX_DEVICE_VERSION;
pub use crate::sys::CUDNN_MAX_SM_MAJOR_NUMBER;
pub use crate::sys::CUDNN_MAX_SM_MINOR_NUMBER;
pub use crate::sys::CUDNN_MINOR;
pub use crate::sys::CUDNN_OPS_MAJOR;
pub use crate::sys::CUDNN_OPS_MINOR;
pub use crate::sys::CUDNN_OPS_PATCH;
pub use crate::sys::CUDNN_PATCHLEVEL;
pub use crate::sys::CUDNN_RNN_PADDED_IO_DISABLED;
pub use crate::sys::CUDNN_RNN_PADDED_IO_ENABLED;
pub use crate::sys::CUDNN_SEQDATA_DIM_COUNT;
pub use crate::sys::CUDNN_VERSION;
pub use crate::sys::CUgraph_st;
pub use crate::sys::cudnnActivationDescriptor_t;
pub use crate::sys::cudnnActivationMode_t;
pub use crate::sys::cudnnActivationStruct;
pub use crate::sys::cudnnAttnDescriptor_t;
pub use crate::sys::cudnnAttnStruct;
pub use crate::sys::cudnnBackendAttributeName_t;
pub use crate::sys::cudnnBackendAttributeType_t;
pub use crate::sys::cudnnBackendBehaviorNote_t;
pub use crate::sys::cudnnBackendDescriptor_t;
pub use crate::sys::cudnnBackendDescriptorType_t;
pub use crate::sys::cudnnBackendHeurMode_t;
pub use crate::sys::cudnnBackendKnobType_t;
pub use crate::sys::cudnnBackendLayoutType_t;
pub use crate::sys::cudnnBackendNormFwdPhase_t;
pub use crate::sys::cudnnBackendNormMode_t;
pub use crate::sys::cudnnBackendNumericalNote_t;
pub use crate::sys::cudnnBackendReshapeMode_t;
pub use crate::sys::cudnnBackendTensorReordering_t;
pub use crate::sys::cudnnBatchNormMode_t;
pub use crate::sys::cudnnBatchNormOps_t;
pub use crate::sys::cudnnBnFinalizeStatsMode_t;
pub use crate::sys::cudnnCTCGradMode_t;
pub use crate::sys::cudnnCTCLossAlgo_t;
pub use crate::sys::cudnnCTCLossDescriptor_t;
pub use crate::sys::cudnnCTCLossStruct;
pub use crate::sys::cudnnCallback_t;
pub use crate::sys::cudnnCausalConv1dActivation_t;
pub use crate::sys::cudnnContext;
pub use crate::sys::cudnnConvolutionBwdDataAlgo_t;
pub use crate::sys::cudnnConvolutionBwdDataAlgoPerf_t;
pub use crate::sys::cudnnConvolutionBwdDataAlgoPerfStruct;
pub use crate::sys::cudnnConvolutionBwdFilterAlgo_t;
pub use crate::sys::cudnnConvolutionBwdFilterAlgoPerf_t;
pub use crate::sys::cudnnConvolutionBwdFilterAlgoPerfStruct;
pub use crate::sys::cudnnConvolutionDescriptor_t;
pub use crate::sys::cudnnConvolutionFwdAlgo_t;
pub use crate::sys::cudnnConvolutionFwdAlgoPerf_t;
pub use crate::sys::cudnnConvolutionFwdAlgoPerfStruct;
pub use crate::sys::cudnnConvolutionMode_t;
pub use crate::sys::cudnnConvolutionStruct;
pub use crate::sys::cudnnDataType_t;
pub use crate::sys::cudnnDebug_t;
pub use crate::sys::cudnnDebugStruct;
pub use crate::sys::cudnnDeterminism_t;
pub use crate::sys::cudnnDirectionMode_t;
pub use crate::sys::cudnnDivNormMode_t;
pub use crate::sys::cudnnDropoutDescriptor_t;
pub use crate::sys::cudnnDropoutStruct;
pub use crate::sys::cudnnErrQueryMode_t;
pub use crate::sys::cudnnFilterDescriptor_t;
pub use crate::sys::cudnnFilterStruct;
pub use crate::sys::cudnnFoldingDirection_t;
pub use crate::sys::cudnnForwardMode_t;
pub use crate::sys::cudnnFraction_t;
pub use crate::sys::cudnnFractionStruct;
pub use crate::sys::cudnnFusedOps_t;
pub use crate::sys::cudnnFusedOpsConstParamLabel_t;
pub use crate::sys::cudnnFusedOpsConstParamPack_t;
pub use crate::sys::cudnnFusedOpsConstParamStruct;
pub use crate::sys::cudnnFusedOpsPlan_t;
pub use crate::sys::cudnnFusedOpsPlanStruct;
pub use crate::sys::cudnnFusedOpsPointerPlaceHolder_t;
pub use crate::sys::cudnnFusedOpsVariantParamLabel_t;
pub use crate::sys::cudnnFusedOpsVariantParamPack_t;
pub use crate::sys::cudnnFusedOpsVariantParamStruct;
pub use crate::sys::cudnnGenStatsMode_t;
pub use crate::sys::cudnnHandle_t;
pub use crate::sys::cudnnIndicesType_t;
pub use crate::sys::cudnnLRNDescriptor_t;
pub use crate::sys::cudnnLRNMode_t;
pub use crate::sys::cudnnLRNStruct;
pub use crate::sys::cudnnLossNormalizationMode_t;
pub use crate::sys::cudnnMathType_t;
pub use crate::sys::cudnnMoeGroupedMatmulMode_t;
pub use crate::sys::cudnnMultiHeadAttnWeightKind_t;
pub use crate::sys::cudnnNanPropagation_t;
pub use crate::sys::cudnnNormAlgo_t;
pub use crate::sys::cudnnNormMode_t;
pub use crate::sys::cudnnNormOps_t;
pub use crate::sys::cudnnOpTensorDescriptor_t;
pub use crate::sys::cudnnOpTensorOp_t;
pub use crate::sys::cudnnOpTensorStruct;
pub use crate::sys::cudnnPaddingMode_t;
pub use crate::sys::cudnnPointwiseMode_t;
pub use crate::sys::cudnnPoolingDescriptor_t;
pub use crate::sys::cudnnPoolingMode_t;
pub use crate::sys::cudnnPoolingStruct;
pub use crate::sys::cudnnRNNAlgo_t;
pub use crate::sys::cudnnRNNBiasMode_t;
pub use crate::sys::cudnnRNNClipMode_t;
pub use crate::sys::cudnnRNNDataDescriptor_t;
pub use crate::sys::cudnnRNNDataLayout_t;
pub use crate::sys::cudnnRNNDataStruct;
pub use crate::sys::cudnnRNNDescriptor_t;
pub use crate::sys::cudnnRNNInputMode_t;
pub use crate::sys::cudnnRNNMode_t;
pub use crate::sys::cudnnRNNStruct;
pub use crate::sys::cudnnReduceTensorDescriptor_t;
pub use crate::sys::cudnnReduceTensorIndices_t;
pub use crate::sys::cudnnReduceTensorOp_t;
pub use crate::sys::cudnnReduceTensorStruct;
pub use crate::sys::cudnnReorderType_t;
pub use crate::sys::cudnnResampleMode_t;
pub use crate::sys::cudnnRngDistribution_t;
pub use crate::sys::cudnnRuntimeTag_t;
pub use crate::sys::cudnnSamplerType_t;
pub use crate::sys::cudnnSeqDataAxis_t;
pub use crate::sys::cudnnSeqDataDescriptor_t;
pub use crate::sys::cudnnSeqDataStruct;
pub use crate::sys::cudnnSeverity_t;
pub use crate::sys::cudnnSignalMode_t;
pub use crate::sys::cudnnSoftmaxAlgorithm_t;
pub use crate::sys::cudnnSoftmaxMode_t;
pub use crate::sys::cudnnSpatialTransformerDescriptor_t;
pub use crate::sys::cudnnSpatialTransformerStruct;
pub use crate::sys::cudnnStatus_t as CudaTargetStatus;
pub use crate::sys::cudnnStatus_t;
pub use crate::sys::cudnnTensorDescriptor_t;
pub use crate::sys::cudnnTensorFormat_t;
pub use crate::sys::cudnnTensorStruct;
pub use crate::sys::cudnnTensorTransformDescriptor_t;
pub use crate::sys::cudnnTensorTransformStruct;
pub use crate::sys::cudnnWgradMode_t;
#[allow(unused_imports)]
use crate::sys::*;
use cuda_libs_cudart;
#[allow(unused_imports)]
use cuda_libs_cudart::sys::*;
#[allow(unused_imports)]
use cuda_libs_cudart::types;
#[cfg(feature = "runtime-link")]
impl crate::sys::cudnnDebugStruct {
    pub fn cudnn_version(mut self, val: ::core::ffi::c_uint) -> Self {
        self.cudnn_version = val;
        self
    }
    pub fn cudnnStatus(mut self, val: cudnnStatus_t) -> Self {
        self.cudnnStatus = val;
        self
    }
    pub fn time_sec(mut self, val: ::core::ffi::c_uint) -> Self {
        self.time_sec = val;
        self
    }
    pub fn time_usec(mut self, val: ::core::ffi::c_uint) -> Self {
        self.time_usec = val;
        self
    }
    pub fn time_delta(mut self, val: ::core::ffi::c_uint) -> Self {
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
    pub fn pid(mut self, val: ::core::ffi::c_ulonglong) -> Self {
        self.pid = val;
        self
    }
    pub fn tid(mut self, val: ::core::ffi::c_ulonglong) -> Self {
        self.tid = val;
        self
    }
    pub fn cudaDeviceId(mut self, val: ::core::ffi::c_int) -> Self {
        self.cudaDeviceId = val;
        self
    }
    pub fn reserved(mut self, val: [::core::ffi::c_int; 15usize]) -> Self {
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
    pub fn reserved(mut self, val: [::core::ffi::c_int; 3usize]) -> Self {
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
    pub fn reserved(mut self, val: [::core::ffi::c_int; 3usize]) -> Self {
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
    pub fn reserved(mut self, val: [::core::ffi::c_int; 3usize]) -> Self {
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
    pub fn cudnnGetErrorString(mut self, val: Option<unsafe extern "C" fn(cudnnStatus_t) -> *const ::core::ffi::c_char>) -> Self {
        self.cudnnGetErrorString = val;
        self
    }
    pub fn cudnnGetLastErrorString(mut self, val: Option<unsafe extern "C" fn(*mut ::core::ffi::c_char, usize)>) -> Self {
        self.cudnnGetLastErrorString = val;
        self
    }
    pub fn cudnnQueryRuntimeError(mut self, val: Option<unsafe extern "C" fn(cudnnHandle_t, *mut cudnnStatus_t, cudnnErrQueryMode_t, *mut cudnnRuntimeTag_t) -> cudnnStatus_t>) -> Self {
        self.cudnnQueryRuntimeError = val;
        self
    }
    pub fn cudnnGetProperty(mut self, val: Option<unsafe extern "C" fn(libraryPropertyType, *mut ::core::ffi::c_int) -> cudnnStatus_t>) -> Self {
        self.cudnnGetProperty = val;
        self
    }
    pub fn cudnnCreate(mut self, val: Option<unsafe extern "C" fn(*mut cudnnHandle_t) -> cudnnStatus_t>) -> Self {
        self.cudnnCreate = val;
        self
    }
    pub fn cudnnDestroy(mut self, val: Option<unsafe extern "C" fn(cudnnHandle_t) -> cudnnStatus_t>) -> Self {
        self.cudnnDestroy = val;
        self
    }
    pub fn cudnnSetStream(mut self, val: Option<unsafe extern "C" fn(cudnnHandle_t, cudaStream_t) -> cudnnStatus_t>) -> Self {
        self.cudnnSetStream = val;
        self
    }
    pub fn cudnnGetStream(mut self, val: Option<unsafe extern "C" fn(cudnnHandle_t, *mut cudaStream_t) -> cudnnStatus_t>) -> Self {
        self.cudnnGetStream = val;
        self
    }
    pub fn cudnnSetCallback(mut self, val: Option<unsafe extern "C" fn(::core::ffi::c_uint, *mut ::core::ffi::c_void, cudnnCallback_t) -> cudnnStatus_t>) -> Self {
        self.cudnnSetCallback = val;
        self
    }
    pub fn cudnnGetCallback(mut self, val: Option<unsafe extern "C" fn(*mut ::core::ffi::c_uint, *mut *mut ::core::ffi::c_void, *mut cudnnCallback_t) -> cudnnStatus_t>) -> Self {
        self.cudnnGetCallback = val;
        self
    }
    pub fn cudnnGraphVersionCheck(mut self, val: Option<unsafe extern "C" fn() -> cudnnStatus_t>) -> Self {
        self.cudnnGraphVersionCheck = val;
        self
    }
    pub fn cudnnBackendCreateDescriptor(mut self, val: Option<unsafe extern "C" fn(cudnnBackendDescriptorType_t, *mut cudnnBackendDescriptor_t) -> cudnnStatus_t>) -> Self {
        self.cudnnBackendCreateDescriptor = val;
        self
    }
    pub fn cudnnBackendDestroyDescriptor(mut self, val: Option<unsafe extern "C" fn(cudnnBackendDescriptor_t) -> cudnnStatus_t>) -> Self {
        self.cudnnBackendDestroyDescriptor = val;
        self
    }
    pub fn cudnnBackendInitialize(mut self, val: Option<unsafe extern "C" fn(cudnnBackendDescriptor_t) -> cudnnStatus_t>) -> Self {
        self.cudnnBackendInitialize = val;
        self
    }
    pub fn cudnnBackendFinalize(mut self, val: Option<unsafe extern "C" fn(cudnnBackendDescriptor_t) -> cudnnStatus_t>) -> Self {
        self.cudnnBackendFinalize = val;
        self
    }
    pub fn cudnnBackendSetAttribute(mut self, val: Option<unsafe extern "C" fn(cudnnBackendDescriptor_t, cudnnBackendAttributeName_t, cudnnBackendAttributeType_t, i64, *const ::core::ffi::c_void) -> cudnnStatus_t>) -> Self {
        self.cudnnBackendSetAttribute = val;
        self
    }
    pub fn cudnnBackendGetAttribute(mut self, val: Option<unsafe extern "C" fn(cudnnBackendDescriptor_t, cudnnBackendAttributeName_t, cudnnBackendAttributeType_t, i64, *mut i64, *mut ::core::ffi::c_void) -> cudnnStatus_t>) -> Self {
        self.cudnnBackendGetAttribute = val;
        self
    }
    pub fn cudnnBackendExecute(mut self, val: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnBackendDescriptor_t, cudnnBackendDescriptor_t) -> cudnnStatus_t>) -> Self {
        self.cudnnBackendExecute = val;
        self
    }
    pub fn cudnnBackendPopulateCudaGraph(mut self, val: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnBackendDescriptor_t, cudnnBackendDescriptor_t, cudaGraph_t) -> cudnnStatus_t>) -> Self {
        self.cudnnBackendPopulateCudaGraph = val;
        self
    }
    pub fn cudnnBackendUpdateCudaGraph(mut self, val: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnBackendDescriptor_t, cudnnBackendDescriptor_t, cudaGraph_t) -> cudnnStatus_t>) -> Self {
        self.cudnnBackendUpdateCudaGraph = val;
        self
    }
    pub fn cudnnCreateTensorDescriptor(mut self, val: Option<unsafe extern "C" fn(*mut cudnnTensorDescriptor_t) -> cudnnStatus_t>) -> Self {
        self.cudnnCreateTensorDescriptor = val;
        self
    }
    pub fn cudnnSetTensor4dDescriptor(mut self, val: Option<unsafe extern "C" fn(cudnnTensorDescriptor_t, cudnnTensorFormat_t, cudnnDataType_t, ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int) -> cudnnStatus_t>) -> Self {
        self.cudnnSetTensor4dDescriptor = val;
        self
    }
    pub fn cudnnSetTensor4dDescriptorEx(
        mut self,
        val: Option<unsafe extern "C" fn(cudnnTensorDescriptor_t, cudnnDataType_t, ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnSetTensor4dDescriptorEx = val;
        self
    }
    pub fn cudnnGetTensor4dDescriptor(
        mut self,
        val: Option<
            unsafe extern "C" fn(cudnnTensorDescriptor_t, *mut cudnnDataType_t, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnGetTensor4dDescriptor = val;
        self
    }
    pub fn cudnnSetTensorNdDescriptor(mut self, val: Option<unsafe extern "C" fn(cudnnTensorDescriptor_t, cudnnDataType_t, ::core::ffi::c_int, *const ::core::ffi::c_int, *const ::core::ffi::c_int) -> cudnnStatus_t>) -> Self {
        self.cudnnSetTensorNdDescriptor = val;
        self
    }
    pub fn cudnnSetTensorNdDescriptorEx(mut self, val: Option<unsafe extern "C" fn(cudnnTensorDescriptor_t, cudnnTensorFormat_t, cudnnDataType_t, ::core::ffi::c_int, *const ::core::ffi::c_int) -> cudnnStatus_t>) -> Self {
        self.cudnnSetTensorNdDescriptorEx = val;
        self
    }
    pub fn cudnnGetTensorNdDescriptor(mut self, val: Option<unsafe extern "C" fn(cudnnTensorDescriptor_t, ::core::ffi::c_int, *mut cudnnDataType_t, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int) -> cudnnStatus_t>) -> Self {
        self.cudnnGetTensorNdDescriptor = val;
        self
    }
    pub fn cudnnGetTensorSizeInBytes(mut self, val: Option<unsafe extern "C" fn(cudnnTensorDescriptor_t, *mut usize) -> cudnnStatus_t>) -> Self {
        self.cudnnGetTensorSizeInBytes = val;
        self
    }
    pub fn cudnnDestroyTensorDescriptor(mut self, val: Option<unsafe extern "C" fn(cudnnTensorDescriptor_t) -> cudnnStatus_t>) -> Self {
        self.cudnnDestroyTensorDescriptor = val;
        self
    }
    pub fn cudnnInitTransformDest(mut self, val: Option<unsafe extern "C" fn(cudnnTensorTransformDescriptor_t, cudnnTensorDescriptor_t, cudnnTensorDescriptor_t, *mut usize) -> cudnnStatus_t>) -> Self {
        self.cudnnInitTransformDest = val;
        self
    }
    pub fn cudnnCreateTensorTransformDescriptor(mut self, val: Option<unsafe extern "C" fn(*mut cudnnTensorTransformDescriptor_t) -> cudnnStatus_t>) -> Self {
        self.cudnnCreateTensorTransformDescriptor = val;
        self
    }
    pub fn cudnnSetTensorTransformDescriptor(mut self, val: Option<unsafe extern "C" fn(cudnnTensorTransformDescriptor_t, u32, cudnnTensorFormat_t, *const i32, *const i32, *const u32, cudnnFoldingDirection_t) -> cudnnStatus_t>) -> Self {
        self.cudnnSetTensorTransformDescriptor = val;
        self
    }
    pub fn cudnnGetTensorTransformDescriptor(mut self, val: Option<unsafe extern "C" fn(cudnnTensorTransformDescriptor_t, u32, *mut cudnnTensorFormat_t, *mut i32, *mut i32, *mut u32, *mut cudnnFoldingDirection_t) -> cudnnStatus_t>) -> Self {
        self.cudnnGetTensorTransformDescriptor = val;
        self
    }
    pub fn cudnnDestroyTensorTransformDescriptor(mut self, val: Option<unsafe extern "C" fn(cudnnTensorTransformDescriptor_t) -> cudnnStatus_t>) -> Self {
        self.cudnnDestroyTensorTransformDescriptor = val;
        self
    }
    pub fn cudnnTransformTensor(mut self, val: Option<unsafe extern "C" fn(cudnnHandle_t, *const ::core::ffi::c_void, cudnnTensorDescriptor_t, *const ::core::ffi::c_void, *const ::core::ffi::c_void, cudnnTensorDescriptor_t, *mut ::core::ffi::c_void) -> cudnnStatus_t>) -> Self {
        self.cudnnTransformTensor = val;
        self
    }
    pub fn cudnnTransformTensorEx(
        mut self,
        val: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnTensorTransformDescriptor_t, *const ::core::ffi::c_void, cudnnTensorDescriptor_t, *const ::core::ffi::c_void, *const ::core::ffi::c_void, cudnnTensorDescriptor_t, *mut ::core::ffi::c_void) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnTransformTensorEx = val;
        self
    }
    pub fn cudnnAddTensor(mut self, val: Option<unsafe extern "C" fn(cudnnHandle_t, *const ::core::ffi::c_void, cudnnTensorDescriptor_t, *const ::core::ffi::c_void, *const ::core::ffi::c_void, cudnnTensorDescriptor_t, *mut ::core::ffi::c_void) -> cudnnStatus_t>) -> Self {
        self.cudnnAddTensor = val;
        self
    }
    pub fn cudnnCreateOpTensorDescriptor(mut self, val: Option<unsafe extern "C" fn(*mut cudnnOpTensorDescriptor_t) -> cudnnStatus_t>) -> Self {
        self.cudnnCreateOpTensorDescriptor = val;
        self
    }
    pub fn cudnnSetOpTensorDescriptor(mut self, val: Option<unsafe extern "C" fn(cudnnOpTensorDescriptor_t, cudnnOpTensorOp_t, cudnnDataType_t, cudnnNanPropagation_t) -> cudnnStatus_t>) -> Self {
        self.cudnnSetOpTensorDescriptor = val;
        self
    }
    pub fn cudnnGetOpTensorDescriptor(mut self, val: Option<unsafe extern "C" fn(cudnnOpTensorDescriptor_t, *mut cudnnOpTensorOp_t, *mut cudnnDataType_t, *mut cudnnNanPropagation_t) -> cudnnStatus_t>) -> Self {
        self.cudnnGetOpTensorDescriptor = val;
        self
    }
    pub fn cudnnDestroyOpTensorDescriptor(mut self, val: Option<unsafe extern "C" fn(cudnnOpTensorDescriptor_t) -> cudnnStatus_t>) -> Self {
        self.cudnnDestroyOpTensorDescriptor = val;
        self
    }
    pub fn cudnnOpTensor(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cudnnHandle_t,
                cudnnOpTensorDescriptor_t,
                *const ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *mut ::core::ffi::c_void,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnOpTensor = val;
        self
    }
    pub fn cudnnCreateReduceTensorDescriptor(mut self, val: Option<unsafe extern "C" fn(*mut cudnnReduceTensorDescriptor_t) -> cudnnStatus_t>) -> Self {
        self.cudnnCreateReduceTensorDescriptor = val;
        self
    }
    pub fn cudnnSetReduceTensorDescriptor(mut self, val: Option<unsafe extern "C" fn(cudnnReduceTensorDescriptor_t, cudnnReduceTensorOp_t, cudnnDataType_t, cudnnNanPropagation_t, cudnnReduceTensorIndices_t, cudnnIndicesType_t) -> cudnnStatus_t>) -> Self {
        self.cudnnSetReduceTensorDescriptor = val;
        self
    }
    pub fn cudnnGetReduceTensorDescriptor(mut self, val: Option<unsafe extern "C" fn(cudnnReduceTensorDescriptor_t, *mut cudnnReduceTensorOp_t, *mut cudnnDataType_t, *mut cudnnNanPropagation_t, *mut cudnnReduceTensorIndices_t, *mut cudnnIndicesType_t) -> cudnnStatus_t>) -> Self {
        self.cudnnGetReduceTensorDescriptor = val;
        self
    }
    pub fn cudnnDestroyReduceTensorDescriptor(mut self, val: Option<unsafe extern "C" fn(cudnnReduceTensorDescriptor_t) -> cudnnStatus_t>) -> Self {
        self.cudnnDestroyReduceTensorDescriptor = val;
        self
    }
    pub fn cudnnGetReductionIndicesSize(mut self, val: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnReduceTensorDescriptor_t, cudnnTensorDescriptor_t, cudnnTensorDescriptor_t, *mut usize) -> cudnnStatus_t>) -> Self {
        self.cudnnGetReductionIndicesSize = val;
        self
    }
    pub fn cudnnGetReductionWorkspaceSize(mut self, val: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnReduceTensorDescriptor_t, cudnnTensorDescriptor_t, cudnnTensorDescriptor_t, *mut usize) -> cudnnStatus_t>) -> Self {
        self.cudnnGetReductionWorkspaceSize = val;
        self
    }
    pub fn cudnnReduceTensor(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cudnnHandle_t,
                cudnnReduceTensorDescriptor_t,
                *mut ::core::ffi::c_void,
                usize,
                *mut ::core::ffi::c_void,
                usize,
                *const ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *mut ::core::ffi::c_void,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnReduceTensor = val;
        self
    }
    pub fn cudnnSetTensor(mut self, val: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnTensorDescriptor_t, *mut ::core::ffi::c_void, *const ::core::ffi::c_void) -> cudnnStatus_t>) -> Self {
        self.cudnnSetTensor = val;
        self
    }
    pub fn cudnnScaleTensor(mut self, val: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnTensorDescriptor_t, *mut ::core::ffi::c_void, *const ::core::ffi::c_void) -> cudnnStatus_t>) -> Self {
        self.cudnnScaleTensor = val;
        self
    }
    pub fn cudnnCreateFilterDescriptor(mut self, val: Option<unsafe extern "C" fn(*mut cudnnFilterDescriptor_t) -> cudnnStatus_t>) -> Self {
        self.cudnnCreateFilterDescriptor = val;
        self
    }
    pub fn cudnnSetFilter4dDescriptor(mut self, val: Option<unsafe extern "C" fn(cudnnFilterDescriptor_t, cudnnDataType_t, cudnnTensorFormat_t, ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int) -> cudnnStatus_t>) -> Self {
        self.cudnnSetFilter4dDescriptor = val;
        self
    }
    pub fn cudnnGetFilter4dDescriptor(mut self, val: Option<unsafe extern "C" fn(cudnnFilterDescriptor_t, *mut cudnnDataType_t, *mut cudnnTensorFormat_t, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int) -> cudnnStatus_t>) -> Self {
        self.cudnnGetFilter4dDescriptor = val;
        self
    }
    pub fn cudnnSetFilterNdDescriptor(mut self, val: Option<unsafe extern "C" fn(cudnnFilterDescriptor_t, cudnnDataType_t, cudnnTensorFormat_t, ::core::ffi::c_int, *const ::core::ffi::c_int) -> cudnnStatus_t>) -> Self {
        self.cudnnSetFilterNdDescriptor = val;
        self
    }
    pub fn cudnnGetFilterNdDescriptor(mut self, val: Option<unsafe extern "C" fn(cudnnFilterDescriptor_t, ::core::ffi::c_int, *mut cudnnDataType_t, *mut cudnnTensorFormat_t, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int) -> cudnnStatus_t>) -> Self {
        self.cudnnGetFilterNdDescriptor = val;
        self
    }
    pub fn cudnnGetFilterSizeInBytes(mut self, val: Option<unsafe extern "C" fn(cudnnFilterDescriptor_t, *mut usize) -> cudnnStatus_t>) -> Self {
        self.cudnnGetFilterSizeInBytes = val;
        self
    }
    pub fn cudnnTransformFilter(
        mut self,
        val: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnTensorTransformDescriptor_t, *const ::core::ffi::c_void, cudnnFilterDescriptor_t, *const ::core::ffi::c_void, *const ::core::ffi::c_void, cudnnFilterDescriptor_t, *mut ::core::ffi::c_void) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnTransformFilter = val;
        self
    }
    pub fn cudnnDestroyFilterDescriptor(mut self, val: Option<unsafe extern "C" fn(cudnnFilterDescriptor_t) -> cudnnStatus_t>) -> Self {
        self.cudnnDestroyFilterDescriptor = val;
        self
    }
    pub fn cudnnSoftmaxForward(
        mut self,
        val: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnSoftmaxAlgorithm_t, cudnnSoftmaxMode_t, *const ::core::ffi::c_void, cudnnTensorDescriptor_t, *const ::core::ffi::c_void, *const ::core::ffi::c_void, cudnnTensorDescriptor_t, *mut ::core::ffi::c_void) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnSoftmaxForward = val;
        self
    }
    pub fn cudnnCreatePoolingDescriptor(mut self, val: Option<unsafe extern "C" fn(*mut cudnnPoolingDescriptor_t) -> cudnnStatus_t>) -> Self {
        self.cudnnCreatePoolingDescriptor = val;
        self
    }
    pub fn cudnnSetPooling2dDescriptor(mut self, val: Option<unsafe extern "C" fn(cudnnPoolingDescriptor_t, cudnnPoolingMode_t, cudnnNanPropagation_t, ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int) -> cudnnStatus_t>) -> Self {
        self.cudnnSetPooling2dDescriptor = val;
        self
    }
    pub fn cudnnGetPooling2dDescriptor(
        mut self,
        val: Option<unsafe extern "C" fn(cudnnPoolingDescriptor_t, *mut cudnnPoolingMode_t, *mut cudnnNanPropagation_t, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnGetPooling2dDescriptor = val;
        self
    }
    pub fn cudnnSetPoolingNdDescriptor(mut self, val: Option<unsafe extern "C" fn(cudnnPoolingDescriptor_t, cudnnPoolingMode_t, cudnnNanPropagation_t, ::core::ffi::c_int, *const ::core::ffi::c_int, *const ::core::ffi::c_int, *const ::core::ffi::c_int) -> cudnnStatus_t>) -> Self {
        self.cudnnSetPoolingNdDescriptor = val;
        self
    }
    pub fn cudnnGetPoolingNdDescriptor(
        mut self,
        val: Option<unsafe extern "C" fn(cudnnPoolingDescriptor_t, ::core::ffi::c_int, *mut cudnnPoolingMode_t, *mut cudnnNanPropagation_t, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnGetPoolingNdDescriptor = val;
        self
    }
    pub fn cudnnGetPoolingNdForwardOutputDim(mut self, val: Option<unsafe extern "C" fn(cudnnPoolingDescriptor_t, cudnnTensorDescriptor_t, ::core::ffi::c_int, *mut ::core::ffi::c_int) -> cudnnStatus_t>) -> Self {
        self.cudnnGetPoolingNdForwardOutputDim = val;
        self
    }
    pub fn cudnnGetPooling2dForwardOutputDim(mut self, val: Option<unsafe extern "C" fn(cudnnPoolingDescriptor_t, cudnnTensorDescriptor_t, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int) -> cudnnStatus_t>) -> Self {
        self.cudnnGetPooling2dForwardOutputDim = val;
        self
    }
    pub fn cudnnDestroyPoolingDescriptor(mut self, val: Option<unsafe extern "C" fn(cudnnPoolingDescriptor_t) -> cudnnStatus_t>) -> Self {
        self.cudnnDestroyPoolingDescriptor = val;
        self
    }
    pub fn cudnnPoolingForward(
        mut self,
        val: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnPoolingDescriptor_t, *const ::core::ffi::c_void, cudnnTensorDescriptor_t, *const ::core::ffi::c_void, *const ::core::ffi::c_void, cudnnTensorDescriptor_t, *mut ::core::ffi::c_void) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnPoolingForward = val;
        self
    }
    pub fn cudnnCreateActivationDescriptor(mut self, val: Option<unsafe extern "C" fn(*mut cudnnActivationDescriptor_t) -> cudnnStatus_t>) -> Self {
        self.cudnnCreateActivationDescriptor = val;
        self
    }
    pub fn cudnnSetActivationDescriptor(mut self, val: Option<unsafe extern "C" fn(cudnnActivationDescriptor_t, cudnnActivationMode_t, cudnnNanPropagation_t, f64) -> cudnnStatus_t>) -> Self {
        self.cudnnSetActivationDescriptor = val;
        self
    }
    pub fn cudnnGetActivationDescriptor(mut self, val: Option<unsafe extern "C" fn(cudnnActivationDescriptor_t, *mut cudnnActivationMode_t, *mut cudnnNanPropagation_t, *mut f64) -> cudnnStatus_t>) -> Self {
        self.cudnnGetActivationDescriptor = val;
        self
    }
    pub fn cudnnSetActivationDescriptorSwishBeta(mut self, val: Option<unsafe extern "C" fn(cudnnActivationDescriptor_t, f64) -> cudnnStatus_t>) -> Self {
        self.cudnnSetActivationDescriptorSwishBeta = val;
        self
    }
    pub fn cudnnGetActivationDescriptorSwishBeta(mut self, val: Option<unsafe extern "C" fn(cudnnActivationDescriptor_t, *mut f64) -> cudnnStatus_t>) -> Self {
        self.cudnnGetActivationDescriptorSwishBeta = val;
        self
    }
    pub fn cudnnDestroyActivationDescriptor(mut self, val: Option<unsafe extern "C" fn(cudnnActivationDescriptor_t) -> cudnnStatus_t>) -> Self {
        self.cudnnDestroyActivationDescriptor = val;
        self
    }
    pub fn cudnnActivationForward(
        mut self,
        val: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnActivationDescriptor_t, *const ::core::ffi::c_void, cudnnTensorDescriptor_t, *const ::core::ffi::c_void, *const ::core::ffi::c_void, cudnnTensorDescriptor_t, *mut ::core::ffi::c_void) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnActivationForward = val;
        self
    }
    pub fn cudnnCreateLRNDescriptor(mut self, val: Option<unsafe extern "C" fn(*mut cudnnLRNDescriptor_t) -> cudnnStatus_t>) -> Self {
        self.cudnnCreateLRNDescriptor = val;
        self
    }
    pub fn cudnnSetLRNDescriptor(mut self, val: Option<unsafe extern "C" fn(cudnnLRNDescriptor_t, ::core::ffi::c_uint, f64, f64, f64) -> cudnnStatus_t>) -> Self {
        self.cudnnSetLRNDescriptor = val;
        self
    }
    pub fn cudnnGetLRNDescriptor(mut self, val: Option<unsafe extern "C" fn(cudnnLRNDescriptor_t, *mut ::core::ffi::c_uint, *mut f64, *mut f64, *mut f64) -> cudnnStatus_t>) -> Self {
        self.cudnnGetLRNDescriptor = val;
        self
    }
    pub fn cudnnDestroyLRNDescriptor(mut self, val: Option<unsafe extern "C" fn(cudnnLRNDescriptor_t) -> cudnnStatus_t>) -> Self {
        self.cudnnDestroyLRNDescriptor = val;
        self
    }
    pub fn cudnnLRNCrossChannelForward(
        mut self,
        val: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnLRNDescriptor_t, cudnnLRNMode_t, *const ::core::ffi::c_void, cudnnTensorDescriptor_t, *const ::core::ffi::c_void, *const ::core::ffi::c_void, cudnnTensorDescriptor_t, *mut ::core::ffi::c_void) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnLRNCrossChannelForward = val;
        self
    }
    pub fn cudnnDivisiveNormalizationForward(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cudnnHandle_t,
                cudnnLRNDescriptor_t,
                cudnnDivNormMode_t,
                *const ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *mut ::core::ffi::c_void,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnDivisiveNormalizationForward = val;
        self
    }
    pub fn cudnnDeriveBNTensorDescriptor(mut self, val: Option<unsafe extern "C" fn(cudnnTensorDescriptor_t, cudnnTensorDescriptor_t, cudnnBatchNormMode_t) -> cudnnStatus_t>) -> Self {
        self.cudnnDeriveBNTensorDescriptor = val;
        self
    }
    pub fn cudnnBatchNormalizationForwardInference(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cudnnHandle_t,
                cudnnBatchNormMode_t,
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *const ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *mut ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                f64,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnBatchNormalizationForwardInference = val;
        self
    }
    pub fn cudnnDeriveNormTensorDescriptor(mut self, val: Option<unsafe extern "C" fn(cudnnTensorDescriptor_t, cudnnTensorDescriptor_t, cudnnTensorDescriptor_t, cudnnNormMode_t, ::core::ffi::c_int) -> cudnnStatus_t>) -> Self {
        self.cudnnDeriveNormTensorDescriptor = val;
        self
    }
    pub fn cudnnNormalizationForwardInference(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cudnnHandle_t,
                cudnnNormMode_t,
                cudnnNormOps_t,
                cudnnNormAlgo_t,
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *const ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *const ::core::ffi::c_void,
                cudnnActivationDescriptor_t,
                cudnnTensorDescriptor_t,
                *mut ::core::ffi::c_void,
                f64,
                ::core::ffi::c_int,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnNormalizationForwardInference = val;
        self
    }
    pub fn cudnnCreateSpatialTransformerDescriptor(mut self, val: Option<unsafe extern "C" fn(*mut cudnnSpatialTransformerDescriptor_t) -> cudnnStatus_t>) -> Self {
        self.cudnnCreateSpatialTransformerDescriptor = val;
        self
    }
    pub fn cudnnSetSpatialTransformerNdDescriptor(mut self, val: Option<unsafe extern "C" fn(cudnnSpatialTransformerDescriptor_t, cudnnSamplerType_t, cudnnDataType_t, ::core::ffi::c_int, *const ::core::ffi::c_int) -> cudnnStatus_t>) -> Self {
        self.cudnnSetSpatialTransformerNdDescriptor = val;
        self
    }
    pub fn cudnnDestroySpatialTransformerDescriptor(mut self, val: Option<unsafe extern "C" fn(cudnnSpatialTransformerDescriptor_t) -> cudnnStatus_t>) -> Self {
        self.cudnnDestroySpatialTransformerDescriptor = val;
        self
    }
    pub fn cudnnSpatialTfGridGeneratorForward(mut self, val: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnSpatialTransformerDescriptor_t, *const ::core::ffi::c_void, *mut ::core::ffi::c_void) -> cudnnStatus_t>) -> Self {
        self.cudnnSpatialTfGridGeneratorForward = val;
        self
    }
    pub fn cudnnSpatialTfSamplerForward(
        mut self,
        val: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnSpatialTransformerDescriptor_t, *const ::core::ffi::c_void, cudnnTensorDescriptor_t, *const ::core::ffi::c_void, *const ::core::ffi::c_void, *const ::core::ffi::c_void, cudnnTensorDescriptor_t, *mut ::core::ffi::c_void) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnSpatialTfSamplerForward = val;
        self
    }
    pub fn cudnnCreateDropoutDescriptor(mut self, val: Option<unsafe extern "C" fn(*mut cudnnDropoutDescriptor_t) -> cudnnStatus_t>) -> Self {
        self.cudnnCreateDropoutDescriptor = val;
        self
    }
    pub fn cudnnDestroyDropoutDescriptor(mut self, val: Option<unsafe extern "C" fn(cudnnDropoutDescriptor_t) -> cudnnStatus_t>) -> Self {
        self.cudnnDestroyDropoutDescriptor = val;
        self
    }
    pub fn cudnnDropoutGetStatesSize(mut self, val: Option<unsafe extern "C" fn(cudnnHandle_t, *mut usize) -> cudnnStatus_t>) -> Self {
        self.cudnnDropoutGetStatesSize = val;
        self
    }
    pub fn cudnnDropoutGetReserveSpaceSize(mut self, val: Option<unsafe extern "C" fn(cudnnTensorDescriptor_t, *mut usize) -> cudnnStatus_t>) -> Self {
        self.cudnnDropoutGetReserveSpaceSize = val;
        self
    }
    pub fn cudnnSetDropoutDescriptor(mut self, val: Option<unsafe extern "C" fn(cudnnDropoutDescriptor_t, cudnnHandle_t, f32, *mut ::core::ffi::c_void, usize, ::core::ffi::c_ulonglong) -> cudnnStatus_t>) -> Self {
        self.cudnnSetDropoutDescriptor = val;
        self
    }
    pub fn cudnnRestoreDropoutDescriptor(mut self, val: Option<unsafe extern "C" fn(cudnnDropoutDescriptor_t, cudnnHandle_t, f32, *mut ::core::ffi::c_void, usize, ::core::ffi::c_ulonglong) -> cudnnStatus_t>) -> Self {
        self.cudnnRestoreDropoutDescriptor = val;
        self
    }
    pub fn cudnnGetDropoutDescriptor(mut self, val: Option<unsafe extern "C" fn(cudnnDropoutDescriptor_t, cudnnHandle_t, *mut f32, *mut *mut ::core::ffi::c_void, *mut ::core::ffi::c_ulonglong) -> cudnnStatus_t>) -> Self {
        self.cudnnGetDropoutDescriptor = val;
        self
    }
    pub fn cudnnDropoutForward(mut self, val: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnDropoutDescriptor_t, cudnnTensorDescriptor_t, *const ::core::ffi::c_void, cudnnTensorDescriptor_t, *mut ::core::ffi::c_void, *mut ::core::ffi::c_void, usize) -> cudnnStatus_t>) -> Self {
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
                cudnnHandle_t,
                cudnnSoftmaxAlgorithm_t,
                cudnnSoftmaxMode_t,
                *const ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *const ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *mut ::core::ffi::c_void,
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
                cudnnHandle_t,
                cudnnPoolingDescriptor_t,
                *const ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *const ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *const ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *mut ::core::ffi::c_void,
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
                cudnnHandle_t,
                cudnnActivationDescriptor_t,
                *const ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *const ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *const ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *mut ::core::ffi::c_void,
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
                cudnnHandle_t,
                cudnnLRNDescriptor_t,
                cudnnLRNMode_t,
                *const ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *const ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *const ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *mut ::core::ffi::c_void,
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
                cudnnHandle_t,
                cudnnLRNDescriptor_t,
                cudnnDivNormMode_t,
                *const ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnDivisiveNormalizationBackward = val;
        self
    }
    pub fn cudnnGetBatchNormalizationForwardTrainingExWorkspaceSize(
        mut self,
        val: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnBatchNormMode_t, cudnnBatchNormOps_t, cudnnTensorDescriptor_t, cudnnTensorDescriptor_t, cudnnTensorDescriptor_t, cudnnTensorDescriptor_t, cudnnActivationDescriptor_t, *mut usize) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnGetBatchNormalizationForwardTrainingExWorkspaceSize = val;
        self
    }
    pub fn cudnnGetBatchNormalizationBackwardExWorkspaceSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(cudnnHandle_t, cudnnBatchNormMode_t, cudnnBatchNormOps_t, cudnnTensorDescriptor_t, cudnnTensorDescriptor_t, cudnnTensorDescriptor_t, cudnnTensorDescriptor_t, cudnnTensorDescriptor_t, cudnnTensorDescriptor_t, cudnnActivationDescriptor_t, *mut usize) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnGetBatchNormalizationBackwardExWorkspaceSize = val;
        self
    }
    pub fn cudnnGetBatchNormalizationTrainingExReserveSpaceSize(mut self, val: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnBatchNormMode_t, cudnnBatchNormOps_t, cudnnActivationDescriptor_t, cudnnTensorDescriptor_t, *mut usize) -> cudnnStatus_t>) -> Self {
        self.cudnnGetBatchNormalizationTrainingExReserveSpaceSize = val;
        self
    }
    pub fn cudnnBatchNormalizationForwardTraining(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cudnnHandle_t,
                cudnnBatchNormMode_t,
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *const ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *mut ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                f64,
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
                f64,
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
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
                cudnnHandle_t,
                cudnnBatchNormMode_t,
                cudnnBatchNormOps_t,
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *const ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *const ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *mut ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                f64,
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
                f64,
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
                cudnnActivationDescriptor_t,
                *mut ::core::ffi::c_void,
                usize,
                *mut ::core::ffi::c_void,
                usize,
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
                cudnnHandle_t,
                cudnnBatchNormMode_t,
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *const ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *const ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *mut ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *const ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
                f64,
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
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
                cudnnHandle_t,
                cudnnBatchNormMode_t,
                cudnnBatchNormOps_t,
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *const ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *const ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *const ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *mut ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *mut ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
                f64,
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                cudnnActivationDescriptor_t,
                *mut ::core::ffi::c_void,
                usize,
                *mut ::core::ffi::c_void,
                usize,
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
                cudnnHandle_t,
                cudnnNormMode_t,
                cudnnNormOps_t,
                cudnnNormAlgo_t,
                cudnnTensorDescriptor_t,
                cudnnTensorDescriptor_t,
                cudnnTensorDescriptor_t,
                cudnnTensorDescriptor_t,
                cudnnActivationDescriptor_t,
                cudnnTensorDescriptor_t,
                *mut usize,
                ::core::ffi::c_int,
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
                cudnnHandle_t,
                cudnnNormMode_t,
                cudnnNormOps_t,
                cudnnNormAlgo_t,
                cudnnTensorDescriptor_t,
                cudnnTensorDescriptor_t,
                cudnnTensorDescriptor_t,
                cudnnTensorDescriptor_t,
                cudnnTensorDescriptor_t,
                cudnnTensorDescriptor_t,
                cudnnActivationDescriptor_t,
                cudnnTensorDescriptor_t,
                *mut usize,
                ::core::ffi::c_int,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnGetNormalizationBackwardWorkspaceSize = val;
        self
    }
    pub fn cudnnGetNormalizationTrainingReserveSpaceSize(mut self, val: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnNormMode_t, cudnnNormOps_t, cudnnNormAlgo_t, cudnnActivationDescriptor_t, cudnnTensorDescriptor_t, *mut usize, ::core::ffi::c_int) -> cudnnStatus_t>) -> Self {
        self.cudnnGetNormalizationTrainingReserveSpaceSize = val;
        self
    }
    pub fn cudnnNormalizationForwardTraining(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cudnnHandle_t,
                cudnnNormMode_t,
                cudnnNormOps_t,
                cudnnNormAlgo_t,
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *const ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                f64,
                cudnnTensorDescriptor_t,
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
                f64,
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
                cudnnActivationDescriptor_t,
                cudnnTensorDescriptor_t,
                *const ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
                usize,
                *mut ::core::ffi::c_void,
                usize,
                ::core::ffi::c_int,
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
                cudnnHandle_t,
                cudnnNormMode_t,
                cudnnNormOps_t,
                cudnnNormAlgo_t,
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *const ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *const ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *const ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *mut ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *mut ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
                f64,
                cudnnTensorDescriptor_t,
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                cudnnActivationDescriptor_t,
                *mut ::core::ffi::c_void,
                usize,
                *mut ::core::ffi::c_void,
                usize,
                ::core::ffi::c_int,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnNormalizationBackward = val;
        self
    }
    pub fn cudnnSpatialTfGridGeneratorBackward(mut self, val: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnSpatialTransformerDescriptor_t, *const ::core::ffi::c_void, *mut ::core::ffi::c_void) -> cudnnStatus_t>) -> Self {
        self.cudnnSpatialTfGridGeneratorBackward = val;
        self
    }
    pub fn cudnnSpatialTfSamplerBackward(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cudnnHandle_t,
                cudnnSpatialTransformerDescriptor_t,
                *const ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnSpatialTfSamplerBackward = val;
        self
    }
    pub fn cudnnDropoutBackward(mut self, val: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnDropoutDescriptor_t, cudnnTensorDescriptor_t, *const ::core::ffi::c_void, cudnnTensorDescriptor_t, *mut ::core::ffi::c_void, *mut ::core::ffi::c_void, usize) -> cudnnStatus_t>) -> Self {
        self.cudnnDropoutBackward = val;
        self
    }
    pub fn cudnnCreateRNNDescriptor(mut self, val: Option<unsafe extern "C" fn(*mut cudnnRNNDescriptor_t) -> cudnnStatus_t>) -> Self {
        self.cudnnCreateRNNDescriptor = val;
        self
    }
    pub fn cudnnDestroyRNNDescriptor(mut self, val: Option<unsafe extern "C" fn(cudnnRNNDescriptor_t) -> cudnnStatus_t>) -> Self {
        self.cudnnDestroyRNNDescriptor = val;
        self
    }
    pub fn cudnnSetRNNDescriptor_v8(
        mut self,
        val: Option<unsafe extern "C" fn(cudnnRNNDescriptor_t, cudnnRNNAlgo_t, cudnnRNNMode_t, cudnnRNNBiasMode_t, cudnnDirectionMode_t, cudnnRNNInputMode_t, cudnnDataType_t, cudnnDataType_t, cudnnMathType_t, i32, i32, i32, i32, cudnnDropoutDescriptor_t, u32) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnSetRNNDescriptor_v8 = val;
        self
    }
    pub fn cudnnGetRNNDescriptor_v8(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cudnnRNNDescriptor_t,
                *mut cudnnRNNAlgo_t,
                *mut cudnnRNNMode_t,
                *mut cudnnRNNBiasMode_t,
                *mut cudnnDirectionMode_t,
                *mut cudnnRNNInputMode_t,
                *mut cudnnDataType_t,
                *mut cudnnDataType_t,
                *mut cudnnMathType_t,
                *mut i32,
                *mut i32,
                *mut i32,
                *mut i32,
                *mut cudnnDropoutDescriptor_t,
                *mut u32,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnGetRNNDescriptor_v8 = val;
        self
    }
    pub fn cudnnRNNSetClip_v8(mut self, val: Option<unsafe extern "C" fn(cudnnRNNDescriptor_t, cudnnRNNClipMode_t, cudnnNanPropagation_t, f64, f64) -> cudnnStatus_t>) -> Self {
        self.cudnnRNNSetClip_v8 = val;
        self
    }
    pub fn cudnnRNNSetClip_v9(mut self, val: Option<unsafe extern "C" fn(cudnnRNNDescriptor_t, cudnnRNNClipMode_t, f64, f64) -> cudnnStatus_t>) -> Self {
        self.cudnnRNNSetClip_v9 = val;
        self
    }
    pub fn cudnnRNNGetClip_v8(mut self, val: Option<unsafe extern "C" fn(cudnnRNNDescriptor_t, *mut cudnnRNNClipMode_t, *mut cudnnNanPropagation_t, *mut f64, *mut f64) -> cudnnStatus_t>) -> Self {
        self.cudnnRNNGetClip_v8 = val;
        self
    }
    pub fn cudnnRNNGetClip_v9(mut self, val: Option<unsafe extern "C" fn(cudnnRNNDescriptor_t, *mut cudnnRNNClipMode_t, *mut f64, *mut f64) -> cudnnStatus_t>) -> Self {
        self.cudnnRNNGetClip_v9 = val;
        self
    }
    pub fn cudnnBuildRNNDynamic(mut self, val: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnRNNDescriptor_t, ::core::ffi::c_int) -> cudnnStatus_t>) -> Self {
        self.cudnnBuildRNNDynamic = val;
        self
    }
    pub fn cudnnGetRNNTempSpaceSizes(mut self, val: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnRNNDescriptor_t, cudnnForwardMode_t, cudnnRNNDataDescriptor_t, *mut usize, *mut usize) -> cudnnStatus_t>) -> Self {
        self.cudnnGetRNNTempSpaceSizes = val;
        self
    }
    pub fn cudnnGetRNNWeightSpaceSize(mut self, val: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnRNNDescriptor_t, *mut usize) -> cudnnStatus_t>) -> Self {
        self.cudnnGetRNNWeightSpaceSize = val;
        self
    }
    pub fn cudnnGetRNNWeightParams(mut self, val: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnRNNDescriptor_t, i32, usize, *const ::core::ffi::c_void, i32, cudnnTensorDescriptor_t, *mut *mut ::core::ffi::c_void, cudnnTensorDescriptor_t, *mut *mut ::core::ffi::c_void) -> cudnnStatus_t>) -> Self {
        self.cudnnGetRNNWeightParams = val;
        self
    }
    pub fn cudnnCreateRNNDataDescriptor(mut self, val: Option<unsafe extern "C" fn(*mut cudnnRNNDataDescriptor_t) -> cudnnStatus_t>) -> Self {
        self.cudnnCreateRNNDataDescriptor = val;
        self
    }
    pub fn cudnnDestroyRNNDataDescriptor(mut self, val: Option<unsafe extern "C" fn(cudnnRNNDataDescriptor_t) -> cudnnStatus_t>) -> Self {
        self.cudnnDestroyRNNDataDescriptor = val;
        self
    }
    pub fn cudnnSetRNNDataDescriptor(mut self, val: Option<unsafe extern "C" fn(cudnnRNNDataDescriptor_t, cudnnDataType_t, cudnnRNNDataLayout_t, ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int, *const ::core::ffi::c_int, *mut ::core::ffi::c_void) -> cudnnStatus_t>) -> Self {
        self.cudnnSetRNNDataDescriptor = val;
        self
    }
    pub fn cudnnGetRNNDataDescriptor(
        mut self,
        val: Option<unsafe extern "C" fn(cudnnRNNDataDescriptor_t, *mut cudnnDataType_t, *mut cudnnRNNDataLayout_t, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int, ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut ::core::ffi::c_void) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnGetRNNDataDescriptor = val;
        self
    }
    pub fn cudnnRNNForward(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cudnnHandle_t,
                cudnnRNNDescriptor_t,
                cudnnForwardMode_t,
                *const i32,
                cudnnRNNDataDescriptor_t,
                *const ::core::ffi::c_void,
                cudnnRNNDataDescriptor_t,
                *mut ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *const ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *const ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
                usize,
                *const ::core::ffi::c_void,
                usize,
                *mut ::core::ffi::c_void,
                usize,
                *mut ::core::ffi::c_void,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnRNNForward = val;
        self
    }
    pub fn cudnnCreateSeqDataDescriptor(mut self, val: Option<unsafe extern "C" fn(*mut cudnnSeqDataDescriptor_t) -> cudnnStatus_t>) -> Self {
        self.cudnnCreateSeqDataDescriptor = val;
        self
    }
    pub fn cudnnDestroySeqDataDescriptor(mut self, val: Option<unsafe extern "C" fn(cudnnSeqDataDescriptor_t) -> cudnnStatus_t>) -> Self {
        self.cudnnDestroySeqDataDescriptor = val;
        self
    }
    pub fn cudnnSetSeqDataDescriptor(mut self, val: Option<unsafe extern "C" fn(cudnnSeqDataDescriptor_t, cudnnDataType_t, ::core::ffi::c_int, *const ::core::ffi::c_int, *const cudnnSeqDataAxis_t, usize, *const ::core::ffi::c_int, *mut ::core::ffi::c_void) -> cudnnStatus_t>) -> Self {
        self.cudnnSetSeqDataDescriptor = val;
        self
    }
    pub fn cudnnGetSeqDataDescriptor(
        mut self,
        val: Option<unsafe extern "C" fn(cudnnSeqDataDescriptor_t, *mut cudnnDataType_t, *mut ::core::ffi::c_int, ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut cudnnSeqDataAxis_t, *mut usize, usize, *mut ::core::ffi::c_int, *mut ::core::ffi::c_void) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnGetSeqDataDescriptor = val;
        self
    }
    pub fn cudnnCreateAttnDescriptor(mut self, val: Option<unsafe extern "C" fn(*mut cudnnAttnDescriptor_t) -> cudnnStatus_t>) -> Self {
        self.cudnnCreateAttnDescriptor = val;
        self
    }
    pub fn cudnnDestroyAttnDescriptor(mut self, val: Option<unsafe extern "C" fn(cudnnAttnDescriptor_t) -> cudnnStatus_t>) -> Self {
        self.cudnnDestroyAttnDescriptor = val;
        self
    }
    pub fn cudnnSetAttnDescriptor(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cudnnAttnDescriptor_t,
                ::core::ffi::c_uint,
                ::core::ffi::c_int,
                f64,
                cudnnDataType_t,
                cudnnDataType_t,
                cudnnMathType_t,
                cudnnDropoutDescriptor_t,
                cudnnDropoutDescriptor_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
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
                cudnnAttnDescriptor_t,
                *mut ::core::ffi::c_uint,
                *mut ::core::ffi::c_int,
                *mut f64,
                *mut cudnnDataType_t,
                *mut cudnnDataType_t,
                *mut cudnnMathType_t,
                *mut cudnnDropoutDescriptor_t,
                *mut cudnnDropoutDescriptor_t,
                *mut ::core::ffi::c_int,
                *mut ::core::ffi::c_int,
                *mut ::core::ffi::c_int,
                *mut ::core::ffi::c_int,
                *mut ::core::ffi::c_int,
                *mut ::core::ffi::c_int,
                *mut ::core::ffi::c_int,
                *mut ::core::ffi::c_int,
                *mut ::core::ffi::c_int,
                *mut ::core::ffi::c_int,
                *mut ::core::ffi::c_int,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnGetAttnDescriptor = val;
        self
    }
    pub fn cudnnGetMultiHeadAttnBuffers(mut self, val: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnAttnDescriptor_t, *mut usize, *mut usize, *mut usize) -> cudnnStatus_t>) -> Self {
        self.cudnnGetMultiHeadAttnBuffers = val;
        self
    }
    pub fn cudnnGetMultiHeadAttnWeights(mut self, val: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnAttnDescriptor_t, cudnnMultiHeadAttnWeightKind_t, usize, *const ::core::ffi::c_void, cudnnTensorDescriptor_t, *mut *mut ::core::ffi::c_void) -> cudnnStatus_t>) -> Self {
        self.cudnnGetMultiHeadAttnWeights = val;
        self
    }
    pub fn cudnnMultiHeadAttnForward(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cudnnHandle_t,
                cudnnAttnDescriptor_t,
                ::core::ffi::c_int,
                *const ::core::ffi::c_int,
                *const ::core::ffi::c_int,
                *const ::core::ffi::c_int,
                *const ::core::ffi::c_int,
                cudnnSeqDataDescriptor_t,
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                cudnnSeqDataDescriptor_t,
                *const ::core::ffi::c_void,
                cudnnSeqDataDescriptor_t,
                *const ::core::ffi::c_void,
                cudnnSeqDataDescriptor_t,
                *mut ::core::ffi::c_void,
                usize,
                *const ::core::ffi::c_void,
                usize,
                *mut ::core::ffi::c_void,
                usize,
                *mut ::core::ffi::c_void,
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
                cudnnHandle_t,
                cudnnRNNDescriptor_t,
                *const i32,
                cudnnRNNDataDescriptor_t,
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                cudnnRNNDataDescriptor_t,
                *mut ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
                usize,
                *const ::core::ffi::c_void,
                usize,
                *mut ::core::ffi::c_void,
                usize,
                *mut ::core::ffi::c_void,
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
                cudnnHandle_t,
                cudnnRNNDescriptor_t,
                cudnnWgradMode_t,
                *const i32,
                cudnnRNNDataDescriptor_t,
                *const ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *const ::core::ffi::c_void,
                cudnnRNNDataDescriptor_t,
                *const ::core::ffi::c_void,
                usize,
                *mut ::core::ffi::c_void,
                usize,
                *mut ::core::ffi::c_void,
                usize,
                *mut ::core::ffi::c_void,
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
                cudnnHandle_t,
                cudnnAttnDescriptor_t,
                *const ::core::ffi::c_int,
                *const ::core::ffi::c_int,
                *const ::core::ffi::c_int,
                *const ::core::ffi::c_int,
                cudnnSeqDataDescriptor_t,
                *const ::core::ffi::c_void,
                cudnnSeqDataDescriptor_t,
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                cudnnSeqDataDescriptor_t,
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                cudnnSeqDataDescriptor_t,
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                usize,
                *const ::core::ffi::c_void,
                usize,
                *mut ::core::ffi::c_void,
                usize,
                *mut ::core::ffi::c_void,
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
                cudnnHandle_t,
                cudnnAttnDescriptor_t,
                cudnnWgradMode_t,
                cudnnSeqDataDescriptor_t,
                *const ::core::ffi::c_void,
                cudnnSeqDataDescriptor_t,
                *const ::core::ffi::c_void,
                cudnnSeqDataDescriptor_t,
                *const ::core::ffi::c_void,
                cudnnSeqDataDescriptor_t,
                *const ::core::ffi::c_void,
                usize,
                *const ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
                usize,
                *mut ::core::ffi::c_void,
                usize,
                *mut ::core::ffi::c_void,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnMultiHeadAttnBackwardWeights = val;
        self
    }
    pub fn cudnnCreateCTCLossDescriptor(mut self, val: Option<unsafe extern "C" fn(*mut cudnnCTCLossDescriptor_t) -> cudnnStatus_t>) -> Self {
        self.cudnnCreateCTCLossDescriptor = val;
        self
    }
    pub fn cudnnSetCTCLossDescriptor(mut self, val: Option<unsafe extern "C" fn(cudnnCTCLossDescriptor_t, cudnnDataType_t) -> cudnnStatus_t>) -> Self {
        self.cudnnSetCTCLossDescriptor = val;
        self
    }
    pub fn cudnnSetCTCLossDescriptorEx(mut self, val: Option<unsafe extern "C" fn(cudnnCTCLossDescriptor_t, cudnnDataType_t, cudnnLossNormalizationMode_t, cudnnNanPropagation_t) -> cudnnStatus_t>) -> Self {
        self.cudnnSetCTCLossDescriptorEx = val;
        self
    }
    pub fn cudnnSetCTCLossDescriptor_v8(mut self, val: Option<unsafe extern "C" fn(cudnnCTCLossDescriptor_t, cudnnDataType_t, cudnnLossNormalizationMode_t, cudnnNanPropagation_t, ::core::ffi::c_int) -> cudnnStatus_t>) -> Self {
        self.cudnnSetCTCLossDescriptor_v8 = val;
        self
    }
    pub fn cudnnSetCTCLossDescriptor_v9(mut self, val: Option<unsafe extern "C" fn(cudnnCTCLossDescriptor_t, cudnnDataType_t, cudnnLossNormalizationMode_t, cudnnCTCGradMode_t, ::core::ffi::c_int) -> cudnnStatus_t>) -> Self {
        self.cudnnSetCTCLossDescriptor_v9 = val;
        self
    }
    pub fn cudnnGetCTCLossDescriptor(mut self, val: Option<unsafe extern "C" fn(cudnnCTCLossDescriptor_t, *mut cudnnDataType_t) -> cudnnStatus_t>) -> Self {
        self.cudnnGetCTCLossDescriptor = val;
        self
    }
    pub fn cudnnGetCTCLossDescriptorEx(mut self, val: Option<unsafe extern "C" fn(cudnnCTCLossDescriptor_t, *mut cudnnDataType_t, *mut cudnnLossNormalizationMode_t, *mut cudnnNanPropagation_t) -> cudnnStatus_t>) -> Self {
        self.cudnnGetCTCLossDescriptorEx = val;
        self
    }
    pub fn cudnnGetCTCLossDescriptor_v8(mut self, val: Option<unsafe extern "C" fn(cudnnCTCLossDescriptor_t, *mut cudnnDataType_t, *mut cudnnLossNormalizationMode_t, *mut cudnnNanPropagation_t, *mut ::core::ffi::c_int) -> cudnnStatus_t>) -> Self {
        self.cudnnGetCTCLossDescriptor_v8 = val;
        self
    }
    pub fn cudnnGetCTCLossDescriptor_v9(mut self, val: Option<unsafe extern "C" fn(cudnnCTCLossDescriptor_t, *mut cudnnDataType_t, *mut cudnnLossNormalizationMode_t, *mut cudnnCTCGradMode_t, *mut ::core::ffi::c_int) -> cudnnStatus_t>) -> Self {
        self.cudnnGetCTCLossDescriptor_v9 = val;
        self
    }
    pub fn cudnnDestroyCTCLossDescriptor(mut self, val: Option<unsafe extern "C" fn(cudnnCTCLossDescriptor_t) -> cudnnStatus_t>) -> Self {
        self.cudnnDestroyCTCLossDescriptor = val;
        self
    }
    pub fn cudnnCTCLoss(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cudnnHandle_t,
                cudnnTensorDescriptor_t,
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_int,
                *const ::core::ffi::c_int,
                *const ::core::ffi::c_int,
                *mut ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *mut ::core::ffi::c_void,
                cudnnCTCLossAlgo_t,
                cudnnCTCLossDescriptor_t,
                *mut ::core::ffi::c_void,
                usize,
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
                cudnnHandle_t,
                cudnnCTCLossAlgo_t,
                cudnnCTCLossDescriptor_t,
                cudnnTensorDescriptor_t,
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_int,
                *const ::core::ffi::c_int,
                *const ::core::ffi::c_int,
                *mut ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *mut ::core::ffi::c_void,
                usize,
                *mut ::core::ffi::c_void,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnCTCLoss_v8 = val;
        self
    }
    pub fn cudnnGetCTCLossWorkspaceSize(
        mut self,
        val: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnTensorDescriptor_t, cudnnTensorDescriptor_t, *const ::core::ffi::c_int, *const ::core::ffi::c_int, *const ::core::ffi::c_int, cudnnCTCLossAlgo_t, cudnnCTCLossDescriptor_t, *mut usize) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnGetCTCLossWorkspaceSize = val;
        self
    }
    pub fn cudnnGetCTCLossWorkspaceSize_v8(mut self, val: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnCTCLossAlgo_t, cudnnCTCLossDescriptor_t, cudnnTensorDescriptor_t, cudnnTensorDescriptor_t, *mut usize) -> cudnnStatus_t>) -> Self {
        self.cudnnGetCTCLossWorkspaceSize_v8 = val;
        self
    }
    pub fn cudnnCreateConvolutionDescriptor(mut self, val: Option<unsafe extern "C" fn(*mut cudnnConvolutionDescriptor_t) -> cudnnStatus_t>) -> Self {
        self.cudnnCreateConvolutionDescriptor = val;
        self
    }
    pub fn cudnnDestroyConvolutionDescriptor(mut self, val: Option<unsafe extern "C" fn(cudnnConvolutionDescriptor_t) -> cudnnStatus_t>) -> Self {
        self.cudnnDestroyConvolutionDescriptor = val;
        self
    }
    pub fn cudnnSetConvolutionMathType(mut self, val: Option<unsafe extern "C" fn(cudnnConvolutionDescriptor_t, cudnnMathType_t) -> cudnnStatus_t>) -> Self {
        self.cudnnSetConvolutionMathType = val;
        self
    }
    pub fn cudnnGetConvolutionMathType(mut self, val: Option<unsafe extern "C" fn(cudnnConvolutionDescriptor_t, *mut cudnnMathType_t) -> cudnnStatus_t>) -> Self {
        self.cudnnGetConvolutionMathType = val;
        self
    }
    pub fn cudnnSetConvolutionGroupCount(mut self, val: Option<unsafe extern "C" fn(cudnnConvolutionDescriptor_t, ::core::ffi::c_int) -> cudnnStatus_t>) -> Self {
        self.cudnnSetConvolutionGroupCount = val;
        self
    }
    pub fn cudnnGetConvolutionGroupCount(mut self, val: Option<unsafe extern "C" fn(cudnnConvolutionDescriptor_t, *mut ::core::ffi::c_int) -> cudnnStatus_t>) -> Self {
        self.cudnnGetConvolutionGroupCount = val;
        self
    }
    pub fn cudnnSetConvolutionReorderType(mut self, val: Option<unsafe extern "C" fn(cudnnConvolutionDescriptor_t, cudnnReorderType_t) -> cudnnStatus_t>) -> Self {
        self.cudnnSetConvolutionReorderType = val;
        self
    }
    pub fn cudnnGetConvolutionReorderType(mut self, val: Option<unsafe extern "C" fn(cudnnConvolutionDescriptor_t, *mut cudnnReorderType_t) -> cudnnStatus_t>) -> Self {
        self.cudnnGetConvolutionReorderType = val;
        self
    }
    pub fn cudnnSetConvolution2dDescriptor(
        mut self,
        val: Option<unsafe extern "C" fn(cudnnConvolutionDescriptor_t, ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int, cudnnConvolutionMode_t, cudnnDataType_t) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnSetConvolution2dDescriptor = val;
        self
    }
    pub fn cudnnGetConvolution2dDescriptor(
        mut self,
        val: Option<unsafe extern "C" fn(cudnnConvolutionDescriptor_t, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut cudnnConvolutionMode_t, *mut cudnnDataType_t) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnGetConvolution2dDescriptor = val;
        self
    }
    pub fn cudnnSetConvolutionNdDescriptor(mut self, val: Option<unsafe extern "C" fn(cudnnConvolutionDescriptor_t, ::core::ffi::c_int, *const ::core::ffi::c_int, *const ::core::ffi::c_int, *const ::core::ffi::c_int, cudnnConvolutionMode_t, cudnnDataType_t) -> cudnnStatus_t>) -> Self {
        self.cudnnSetConvolutionNdDescriptor = val;
        self
    }
    pub fn cudnnGetConvolutionNdDescriptor(
        mut self,
        val: Option<unsafe extern "C" fn(cudnnConvolutionDescriptor_t, ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut cudnnConvolutionMode_t, *mut cudnnDataType_t) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnGetConvolutionNdDescriptor = val;
        self
    }
    pub fn cudnnGetConvolution2dForwardOutputDim(mut self, val: Option<unsafe extern "C" fn(cudnnConvolutionDescriptor_t, cudnnTensorDescriptor_t, cudnnFilterDescriptor_t, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut ::core::ffi::c_int) -> cudnnStatus_t>) -> Self {
        self.cudnnGetConvolution2dForwardOutputDim = val;
        self
    }
    pub fn cudnnGetConvolutionNdForwardOutputDim(mut self, val: Option<unsafe extern "C" fn(cudnnConvolutionDescriptor_t, cudnnTensorDescriptor_t, cudnnFilterDescriptor_t, ::core::ffi::c_int, *mut ::core::ffi::c_int) -> cudnnStatus_t>) -> Self {
        self.cudnnGetConvolutionNdForwardOutputDim = val;
        self
    }
    pub fn cudnnGetConvolutionForwardAlgorithmMaxCount(mut self, val: Option<unsafe extern "C" fn(cudnnHandle_t, *mut ::core::ffi::c_int) -> cudnnStatus_t>) -> Self {
        self.cudnnGetConvolutionForwardAlgorithmMaxCount = val;
        self
    }
    pub fn cudnnGetConvolutionForwardAlgorithm_v7(
        mut self,
        val: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnTensorDescriptor_t, cudnnFilterDescriptor_t, cudnnConvolutionDescriptor_t, cudnnTensorDescriptor_t, ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut cudnnConvolutionFwdAlgoPerf_t) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnGetConvolutionForwardAlgorithm_v7 = val;
        self
    }
    pub fn cudnnFindConvolutionForwardAlgorithm(
        mut self,
        val: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnTensorDescriptor_t, cudnnFilterDescriptor_t, cudnnConvolutionDescriptor_t, cudnnTensorDescriptor_t, ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut cudnnConvolutionFwdAlgoPerf_t) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnFindConvolutionForwardAlgorithm = val;
        self
    }
    pub fn cudnnFindConvolutionForwardAlgorithmEx(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cudnnHandle_t,
                cudnnTensorDescriptor_t,
                *const ::core::ffi::c_void,
                cudnnFilterDescriptor_t,
                *const ::core::ffi::c_void,
                cudnnConvolutionDescriptor_t,
                cudnnTensorDescriptor_t,
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
                *mut ::core::ffi::c_int,
                *mut cudnnConvolutionFwdAlgoPerf_t,
                *mut ::core::ffi::c_void,
                usize,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnFindConvolutionForwardAlgorithmEx = val;
        self
    }
    pub fn cudnnIm2Col(mut self, val: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnTensorDescriptor_t, *const ::core::ffi::c_void, cudnnFilterDescriptor_t, cudnnConvolutionDescriptor_t, *mut ::core::ffi::c_void) -> cudnnStatus_t>) -> Self {
        self.cudnnIm2Col = val;
        self
    }
    pub fn cudnnReorderFilterAndBias(mut self, val: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnFilterDescriptor_t, cudnnReorderType_t, *const ::core::ffi::c_void, *mut ::core::ffi::c_void, ::core::ffi::c_int, *const ::core::ffi::c_void, *mut ::core::ffi::c_void) -> cudnnStatus_t>) -> Self {
        self.cudnnReorderFilterAndBias = val;
        self
    }
    pub fn cudnnGetConvolutionForwardWorkspaceSize(mut self, val: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnTensorDescriptor_t, cudnnFilterDescriptor_t, cudnnConvolutionDescriptor_t, cudnnTensorDescriptor_t, cudnnConvolutionFwdAlgo_t, *mut usize) -> cudnnStatus_t>) -> Self {
        self.cudnnGetConvolutionForwardWorkspaceSize = val;
        self
    }
    pub fn cudnnConvolutionForward(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cudnnHandle_t,
                *const ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *const ::core::ffi::c_void,
                cudnnFilterDescriptor_t,
                *const ::core::ffi::c_void,
                cudnnConvolutionDescriptor_t,
                cudnnConvolutionFwdAlgo_t,
                *mut ::core::ffi::c_void,
                usize,
                *const ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *mut ::core::ffi::c_void,
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
                cudnnHandle_t,
                *const ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *const ::core::ffi::c_void,
                cudnnFilterDescriptor_t,
                *const ::core::ffi::c_void,
                cudnnConvolutionDescriptor_t,
                cudnnConvolutionFwdAlgo_t,
                *mut ::core::ffi::c_void,
                usize,
                *const ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *const ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *const ::core::ffi::c_void,
                cudnnActivationDescriptor_t,
                cudnnTensorDescriptor_t,
                *mut ::core::ffi::c_void,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnConvolutionBiasActivationForward = val;
        self
    }
    pub fn cudnnGetConvolutionBackwardDataAlgorithmMaxCount(mut self, val: Option<unsafe extern "C" fn(cudnnHandle_t, *mut ::core::ffi::c_int) -> cudnnStatus_t>) -> Self {
        self.cudnnGetConvolutionBackwardDataAlgorithmMaxCount = val;
        self
    }
    pub fn cudnnFindConvolutionBackwardDataAlgorithm(
        mut self,
        val: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnFilterDescriptor_t, cudnnTensorDescriptor_t, cudnnConvolutionDescriptor_t, cudnnTensorDescriptor_t, ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut cudnnConvolutionBwdDataAlgoPerf_t) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnFindConvolutionBackwardDataAlgorithm = val;
        self
    }
    pub fn cudnnFindConvolutionBackwardDataAlgorithmEx(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cudnnHandle_t,
                cudnnFilterDescriptor_t,
                *const ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *const ::core::ffi::c_void,
                cudnnConvolutionDescriptor_t,
                cudnnTensorDescriptor_t,
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
                *mut ::core::ffi::c_int,
                *mut cudnnConvolutionBwdDataAlgoPerf_t,
                *mut ::core::ffi::c_void,
                usize,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnFindConvolutionBackwardDataAlgorithmEx = val;
        self
    }
    pub fn cudnnGetConvolutionBackwardDataAlgorithm_v7(
        mut self,
        val: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnFilterDescriptor_t, cudnnTensorDescriptor_t, cudnnConvolutionDescriptor_t, cudnnTensorDescriptor_t, ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut cudnnConvolutionBwdDataAlgoPerf_t) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnGetConvolutionBackwardDataAlgorithm_v7 = val;
        self
    }
    pub fn cudnnGetConvolutionBackwardDataWorkspaceSize(mut self, val: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnFilterDescriptor_t, cudnnTensorDescriptor_t, cudnnConvolutionDescriptor_t, cudnnTensorDescriptor_t, cudnnConvolutionBwdDataAlgo_t, *mut usize) -> cudnnStatus_t>) -> Self {
        self.cudnnGetConvolutionBackwardDataWorkspaceSize = val;
        self
    }
    pub fn cudnnConvolutionBackwardData(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cudnnHandle_t,
                *const ::core::ffi::c_void,
                cudnnFilterDescriptor_t,
                *const ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *const ::core::ffi::c_void,
                cudnnConvolutionDescriptor_t,
                cudnnConvolutionBwdDataAlgo_t,
                *mut ::core::ffi::c_void,
                usize,
                *const ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *mut ::core::ffi::c_void,
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
                cudnnHandle_t,
                cudnnFilterDescriptor_t,
                cudnnTensorDescriptor_t,
                cudnnConvolutionDescriptor_t,
                cudnnTensorDescriptor_t,
                cudnnTensorFormat_t,
                cudnnFilterDescriptor_t,
                cudnnTensorDescriptor_t,
                cudnnConvolutionDescriptor_t,
                cudnnTensorDescriptor_t,
                cudnnTensorTransformDescriptor_t,
                cudnnTensorTransformDescriptor_t,
                cudnnTensorTransformDescriptor_t,
                cudnnTensorTransformDescriptor_t,
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
    pub fn cudnnGetConvolutionBackwardFilterAlgorithmMaxCount(mut self, val: Option<unsafe extern "C" fn(cudnnHandle_t, *mut ::core::ffi::c_int) -> cudnnStatus_t>) -> Self {
        self.cudnnGetConvolutionBackwardFilterAlgorithmMaxCount = val;
        self
    }
    pub fn cudnnFindConvolutionBackwardFilterAlgorithm(
        mut self,
        val: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnTensorDescriptor_t, cudnnTensorDescriptor_t, cudnnConvolutionDescriptor_t, cudnnFilterDescriptor_t, ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut cudnnConvolutionBwdFilterAlgoPerf_t) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnFindConvolutionBackwardFilterAlgorithm = val;
        self
    }
    pub fn cudnnFindConvolutionBackwardFilterAlgorithmEx(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cudnnHandle_t,
                cudnnTensorDescriptor_t,
                *const ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *const ::core::ffi::c_void,
                cudnnConvolutionDescriptor_t,
                cudnnFilterDescriptor_t,
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
                *mut ::core::ffi::c_int,
                *mut cudnnConvolutionBwdFilterAlgoPerf_t,
                *mut ::core::ffi::c_void,
                usize,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnFindConvolutionBackwardFilterAlgorithmEx = val;
        self
    }
    pub fn cudnnGetConvolutionBackwardFilterAlgorithm_v7(
        mut self,
        val: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnTensorDescriptor_t, cudnnTensorDescriptor_t, cudnnConvolutionDescriptor_t, cudnnFilterDescriptor_t, ::core::ffi::c_int, *mut ::core::ffi::c_int, *mut cudnnConvolutionBwdFilterAlgoPerf_t) -> cudnnStatus_t>,
    ) -> Self {
        self.cudnnGetConvolutionBackwardFilterAlgorithm_v7 = val;
        self
    }
    pub fn cudnnGetConvolutionBackwardFilterWorkspaceSize(mut self, val: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnTensorDescriptor_t, cudnnTensorDescriptor_t, cudnnConvolutionDescriptor_t, cudnnFilterDescriptor_t, cudnnConvolutionBwdFilterAlgo_t, *mut usize) -> cudnnStatus_t>) -> Self {
        self.cudnnGetConvolutionBackwardFilterWorkspaceSize = val;
        self
    }
    pub fn cudnnConvolutionBackwardFilter(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cudnnHandle_t,
                *const ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *const ::core::ffi::c_void,
                cudnnTensorDescriptor_t,
                *const ::core::ffi::c_void,
                cudnnConvolutionDescriptor_t,
                cudnnConvolutionBwdFilterAlgo_t,
                *mut ::core::ffi::c_void,
                usize,
                *const ::core::ffi::c_void,
                cudnnFilterDescriptor_t,
                *mut ::core::ffi::c_void,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnConvolutionBackwardFilter = val;
        self
    }
    pub fn cudnnConvolutionBackwardBias(mut self, val: Option<unsafe extern "C" fn(cudnnHandle_t, *const ::core::ffi::c_void, cudnnTensorDescriptor_t, *const ::core::ffi::c_void, *const ::core::ffi::c_void, cudnnTensorDescriptor_t, *mut ::core::ffi::c_void) -> cudnnStatus_t>) -> Self {
        self.cudnnConvolutionBackwardBias = val;
        self
    }
    pub fn cudnnCreateFusedOpsConstParamPack(mut self, val: Option<unsafe extern "C" fn(*mut cudnnFusedOpsConstParamPack_t, cudnnFusedOps_t) -> cudnnStatus_t>) -> Self {
        self.cudnnCreateFusedOpsConstParamPack = val;
        self
    }
    pub fn cudnnDestroyFusedOpsConstParamPack(mut self, val: Option<unsafe extern "C" fn(cudnnFusedOpsConstParamPack_t) -> cudnnStatus_t>) -> Self {
        self.cudnnDestroyFusedOpsConstParamPack = val;
        self
    }
    pub fn cudnnSetFusedOpsConstParamPackAttribute(mut self, val: Option<unsafe extern "C" fn(cudnnFusedOpsConstParamPack_t, cudnnFusedOpsConstParamLabel_t, *const ::core::ffi::c_void) -> cudnnStatus_t>) -> Self {
        self.cudnnSetFusedOpsConstParamPackAttribute = val;
        self
    }
    pub fn cudnnGetFusedOpsConstParamPackAttribute(mut self, val: Option<unsafe extern "C" fn(cudnnFusedOpsConstParamPack_t, cudnnFusedOpsConstParamLabel_t, *mut ::core::ffi::c_void, *mut ::core::ffi::c_int) -> cudnnStatus_t>) -> Self {
        self.cudnnGetFusedOpsConstParamPackAttribute = val;
        self
    }
    pub fn cudnnCreateFusedOpsVariantParamPack(mut self, val: Option<unsafe extern "C" fn(*mut cudnnFusedOpsVariantParamPack_t, cudnnFusedOps_t) -> cudnnStatus_t>) -> Self {
        self.cudnnCreateFusedOpsVariantParamPack = val;
        self
    }
    pub fn cudnnDestroyFusedOpsVariantParamPack(mut self, val: Option<unsafe extern "C" fn(cudnnFusedOpsVariantParamPack_t) -> cudnnStatus_t>) -> Self {
        self.cudnnDestroyFusedOpsVariantParamPack = val;
        self
    }
    pub fn cudnnSetFusedOpsVariantParamPackAttribute(mut self, val: Option<unsafe extern "C" fn(cudnnFusedOpsVariantParamPack_t, cudnnFusedOpsVariantParamLabel_t, *mut ::core::ffi::c_void) -> cudnnStatus_t>) -> Self {
        self.cudnnSetFusedOpsVariantParamPackAttribute = val;
        self
    }
    pub fn cudnnGetFusedOpsVariantParamPackAttribute(mut self, val: Option<unsafe extern "C" fn(cudnnFusedOpsVariantParamPack_t, cudnnFusedOpsVariantParamLabel_t, *mut ::core::ffi::c_void) -> cudnnStatus_t>) -> Self {
        self.cudnnGetFusedOpsVariantParamPackAttribute = val;
        self
    }
    pub fn cudnnCreateFusedOpsPlan(mut self, val: Option<unsafe extern "C" fn(*mut cudnnFusedOpsPlan_t, cudnnFusedOps_t) -> cudnnStatus_t>) -> Self {
        self.cudnnCreateFusedOpsPlan = val;
        self
    }
    pub fn cudnnDestroyFusedOpsPlan(mut self, val: Option<unsafe extern "C" fn(cudnnFusedOpsPlan_t) -> cudnnStatus_t>) -> Self {
        self.cudnnDestroyFusedOpsPlan = val;
        self
    }
    pub fn cudnnMakeFusedOpsPlan(mut self, val: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnFusedOpsPlan_t, cudnnFusedOpsConstParamPack_t, *mut usize) -> cudnnStatus_t>) -> Self {
        self.cudnnMakeFusedOpsPlan = val;
        self
    }
    pub fn cudnnFusedOpsExecute(mut self, val: Option<unsafe extern "C" fn(cudnnHandle_t, cudnnFusedOpsPlan_t, cudnnFusedOpsVariantParamPack_t) -> cudnnStatus_t>) -> Self {
        self.cudnnFusedOpsExecute = val;
        self
    }
    pub fn cudnnSubquadraticOpsVersionCheck(mut self, val: Option<unsafe extern "C" fn() -> cudnnStatus_t>) -> Self {
        self.cudnnSubquadraticOpsVersionCheck = val;
        self
    }
    pub fn cudnnCausalConv1dForward(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cudaStream_t,
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                cudnnDataType_t,
                cudnnCausalConv1dActivation_t,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnCausalConv1dForward = val;
        self
    }
    pub fn cudnnCausalConv1dBackward(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                cudaStream_t,
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                cudnnDataType_t,
                cudnnDataType_t,
                cudnnCausalConv1dActivation_t,
            ) -> cudnnStatus_t,
        >,
    ) -> Self {
        self.cudnnCausalConv1dBackward = val;
        self
    }
}
#[doc = "Returns cuDNN library version (MAJOR*10000 + MINOR*100 + PATCH).\n\n# Returns\n\nThe cuDNN version as an encoded integer.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnGetVersion() -> usize {
    unsafe { crate::sys::cudnnGetVersion() }
}
#[doc = "Returns max supported GPU compute capability.\n\n# Returns\n\nThe maximum supported device version.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnGetMaxDeviceVersion() -> usize {
    unsafe { crate::sys::cudnnGetMaxDeviceVersion() }
}
#[doc = "Returns CUDA Runtime version linked against cuDNN.\n\n# Returns\n\nThe CUDA Runtime version.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnGetCudartVersion() -> usize {
    unsafe { crate::sys::cudnnGetCudartVersion() }
}
#[doc = "Converts status code to human-readable string.\n\n# Arguments\n\n* `status` [in]  - The cuDNN status code to convert.\n\n# Returns\n\nPointer to a static string describing the status code.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnGetErrorString(status: cudnnStatus_t) -> *const ::core::ffi::c_char {
    unsafe { crate::sys::cudnnGetErrorString(status) }
}
#[doc = "Retrieves most recent error message. Thread-safe.\n\n# Arguments\n\n* `message` [out]  -   Buffer to receive the error message string.\n* `max_size` [in]  -  Maximum number of bytes to write into `message.`\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnGetLastErrorString<T0: types::CudaAsMutPtr>(mut message: T0, max_size: usize) -> () {
    unsafe { crate::sys::cudnnGetLastErrorString(message.as_mut_ptr() as *mut _, max_size) }
}
#[doc = "Queries remote kernel error state.\n> **Deprecated** Use cudnnGetLastErrorString instead.\n\n# Arguments\n\n* `handle` [in]  -   cuDNN handle.\n* `rstatus` [out]  -  Pointer to receive the runtime status.\n* `mode` [in]  -     Error query mode.\n* `tag` [out]  -      Runtime tag (unused, may be NULL).\n\n# Returns\n\ncuDNN status code.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnQueryRuntimeError<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr>(handle: cudnnHandle_t, mut rstatus: T0, mode: cudnnErrQueryMode_t, mut tag: T1) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnQueryRuntimeError(handle, rstatus.as_mut_ptr() as *mut _, mode, tag.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Queries library property (major, minor, or patch version).\n\n# Arguments\n\n* `type` [in]  -   The property type to query (MAJOR_VERSION, MINOR_VERSION, or PATCH_LEVEL).\n* `value` [out]  -  Pointer to receive the property value.\n\n# Returns\n\ncuDNN status code.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnGetProperty(type_: libraryPropertyType) -> Result<::core::ffi::c_int, crate::sys::cudnnStatus_t> {
    let mut out_1: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnGetProperty(type_, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as ::core::ffi::c_int) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Creates cuDNN context. Must precede all other cuDNN library calls.\n\n# Arguments\n\n* `handle` [out]  -  Pointer to receive the newly created cuDNN handle.\n@retval CUDNN_STATUS_SUCCESS\n@retval CUDNN_STATUS_BAD_PARAM\n@retval CUDNN_STATUS_NOT_INITIALIZED\n@retval CUDNN_STATUS_NOT_SUPPORTED_ARCH_MISMATCH\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnCreate() -> Result<cudnnHandle_t, crate::sys::cudnnStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cudnnHandle_t> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnCreate(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cudnnHandle_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Destroys cuDNN context. Calls cudaDeviceSynchronize.\n\n# Arguments\n\n* `handle` [in]  -  The cuDNN handle to destroy.\n\n# Returns\n\ncuDNN status code.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnDestroy(handle: cudnnHandle_t) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnDestroy(handle) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Associates CUDA stream with cuDNN handle.\n\n# Arguments\n\n* `handle` [in]  -    cuDNN handle.\n* `streamId` [in]  -  CUDA stream to associate.\n@retval CUDNN_STATUS_SUCCESS\n@retval CUDNN_STATUS_BAD_PARAM\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnSetStream(handle: cudnnHandle_t, streamId: cudaStream_t) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnSetStream(handle, streamId) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Retrieves CUDA stream from cuDNN handle.\n\n# Arguments\n\n* `handle` [in]  -    cuDNN handle.\n* `streamId` [out]  -  Pointer to receive the associated CUDA stream.\n@retval CUDNN_STATUS_SUCCESS\n@retval CUDNN_STATUS_BAD_PARAM\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnGetStream(handle: cudnnHandle_t) -> Result<cudaStream_t, crate::sys::cudnnStatus_t> {
    let mut out_1: std::mem::MaybeUninit<cudaStream_t> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnGetStream(handle, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as cudaStream_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Registers debug callback with message mask.\n\n# Arguments\n\n* `mask` [in]  -   Bitmask of severity levels to enable (see CUDNN_SEV_*_EN).\n* `udata` [in]  -  User data pointer passed to callback.\n* `fptr` [in]  -   Callback function pointer.\n\n# Returns\n\ncuDNN status code.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnSetCallback<T0: types::CudaAsMutPtr>(mask: ::core::ffi::c_uint, mut udata: T0, fptr: cudnnCallback_t) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnSetCallback(mask, udata.as_mut_ptr() as *mut _, fptr) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Retrieves registered debug callback and its configuration.\n\n# Arguments\n\n* `mask` [out]  -   Pointer to receive the current severity mask.\n* `udata` [out]  -  Pointer to receive the user data pointer.\n* `fptr` [out]  -   Pointer to receive the callback function pointer.\n\n# Returns\n\ncuDNN status code.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnGetCallback(udata: *mut *mut ::core::ffi::c_void) -> Result<(::core::ffi::c_uint, cudnnCallback_t), crate::sys::cudnnStatus_t> {
    let mut out_0: std::mem::MaybeUninit<::core::ffi::c_uint> = std::mem::MaybeUninit::zeroed();
    let mut out_2: std::mem::MaybeUninit<cudnnCallback_t> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnGetCallback(out_0.as_mut_ptr() as *mut _, udata, out_2.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok((out_0.assume_init() as ::core::ffi::c_uint, out_2.assume_init() as cudnnCallback_t)) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Cross-library version checker.\nThis function is implemented differently in each sub-library. Each sublib\nchecks whether its own version matches that of its dependencies.\n@retval CUDNN_STATUS_SUCCESS if the version check passes.\n@retval CUDNN_STATUS_SUBLIBRARY_VERSION_MISMATCH if the versions are inconsistent.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnGraphVersionCheck() -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnGraphVersionCheck() };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Allocates memory for a backend descriptor of the specified type.\n\n# Arguments\n\n* `descriptorType` [in]  -  The type of descriptor to create.\n* `descriptor` [out]  -      Pointer to receive the newly created descriptor.\n@retval CUDNN_STATUS_SUCCESS\n@retval CUDNN_STATUS_NOT_SUPPORTED\n@retval CUDNN_STATUS_ALLOC_FAILED\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnBackendCreateDescriptor<T0: types::CudaAsMutPtr>(descriptorType: cudnnBackendDescriptorType_t, mut descriptor: T0) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnBackendCreateDescriptor(descriptorType, descriptor.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Deallocates a backend descriptor and frees associated memory.\n\n# Arguments\n\n* `descriptor` [in]  -  The descriptor to destroy.\n@retval CUDNN_STATUS_SUCCESS\n@retval CUDNN_STATUS_ALLOC_FAILED\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnBackendDestroyDescriptor(descriptor: cudnnBackendDescriptor_t) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnBackendDestroyDescriptor(descriptor) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Repurposes pre-allocated memory for a backend descriptor.\n> **Deprecated** Since cuDNN 9.2. Use cudnnBackendCreateDescriptor instead.\n\n# Arguments\n\n* `descriptor` [in]  -  The descriptor to initialize.\n\n# Returns\n\ncuDNN status code.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnBackendInitialize(descriptor: cudnnBackendDescriptor_t) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnBackendInitialize(descriptor) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Validates and finalizes a descriptor. After finalization, attributes become read-only.\n\n# Arguments\n\n* `descriptor` [in]  -  The descriptor to finalize.\n@retval CUDNN_STATUS_SUCCESS\n@retval CUDNN_STATUS_BAD_PARAM\n@retval CUDNN_STATUS_NOT_SUPPORTED\n@retval CUDNN_STATUS_INTERNAL_ERROR\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnBackendFinalize(descriptor: cudnnBackendDescriptor_t) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnBackendFinalize(descriptor) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Sets an attribute on an unfinalized backend descriptor.\n\n# Arguments\n\n* `descriptor` [in]  -       The target descriptor (must not be finalized).\n* `attributeName` [in]  -    The attribute to set.\n* `attributeType` [in]  -    The data type of the attribute values.\n* `elementCount` [in]  -     Number of elements in `arrayOfElements.`\n* `arrayOfElements` [in]  -  Pointer to the attribute value(s).\n@retval CUDNN_STATUS_SUCCESS\n@retval CUDNN_STATUS_NOT_INITIALIZED\n@retval CUDNN_STATUS_BAD_PARAM\n@retval CUDNN_STATUS_NOT_SUPPORTED\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnBackendSetAttribute<T0: types::CudaAsPtr>(descriptor: cudnnBackendDescriptor_t, attributeName: cudnnBackendAttributeName_t, attributeType: cudnnBackendAttributeType_t, elementCount: i64, arrayOfElements: T0) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnBackendSetAttribute(descriptor, attributeName, attributeType, elementCount, arrayOfElements.as_const_ptr() as *const _) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Retrieves an attribute from a finalized backend descriptor.\n\n# Arguments\n\n* `descriptor` [in]  -             The source descriptor (must be finalized).\n* `attributeName` [in]  -          The attribute to query.\n* `attributeType` [in]  -          The expected data type of the attribute.\n* `requestedElementCount` [in]  -  Maximum number of elements to retrieve.\n* `elementCount` [out]  -           Pointer to receive the actual element count.\n* `arrayOfElements` [out]  -        Buffer to receive the attribute value(s).\n@retval CUDNN_STATUS_SUCCESS\n@retval CUDNN_STATUS_BAD_PARAM\n@retval CUDNN_STATUS_NOT_INITIALIZED\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnBackendGetAttribute(descriptor: cudnnBackendDescriptor_t, attributeName: cudnnBackendAttributeName_t, attributeType: cudnnBackendAttributeType_t, requestedElementCount: i64, arrayOfElements: *mut ::core::ffi::c_void) -> Result<i64, crate::sys::cudnnStatus_t> {
    let mut out_4: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnBackendGetAttribute(descriptor, attributeName, attributeType, requestedElementCount, out_4.as_mut_ptr() as *mut _, arrayOfElements) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_4.assume_init() as i64) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Runs an execution plan with the given variant pack containing data pointers.\n\n# Arguments\n\n* `handle` [in]  -         cuDNN handle.\n* `executionPlan` [in]  -  Finalized execution plan descriptor.\n* `variantPack` [in]  -    Finalized variant pack descriptor with data pointers.\n@retval CUDNN_STATUS_SUCCESS\n@retval CUDNN_STATUS_BAD_PARAM\n@retval CUDNN_STATUS_INTERNAL_ERROR\n@retval CUDNN_STATUS_EXECUTION_FAILED\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnBackendExecute(handle: cudnnHandle_t, executionPlan: cudnnBackendDescriptor_t, variantPack: cudnnBackendDescriptor_t) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnBackendExecute(handle, executionPlan, variantPack) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Populates a CUDA graph with nodes from an execution plan.\n\n# Arguments\n\n* `handle` [in]  -         cuDNN handle.\n* `executionPlan` [in]  -  Finalized execution plan descriptor.\n* `variantPack` [in]  -    Finalized variant pack descriptor.\n* `graph` [inout]  -          CUDA graph to populate.\n@retval CUDNN_STATUS_SUCCESS\n@retval CUDNN_STATUS_BAD_PARAM\n@retval CUDNN_STATUS_INTERNAL_ERROR\n@retval CUDNN_STATUS_EXECUTION_FAILED\n@retval CUDNN_STATUS_NOT_SUPPORTED\n> **Since** cuDNN 9.5.0"]
pub unsafe fn cudnnBackendPopulateCudaGraph(handle: cudnnHandle_t, executionPlan: cudnnBackendDescriptor_t, variantPack: cudnnBackendDescriptor_t, graph: cudaGraph_t) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnBackendPopulateCudaGraph(handle, executionPlan, variantPack, graph) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Updates an existing CUDA graph with new data pointers from a variant pack.\n\n# Arguments\n\n* `handle` [in]  -         cuDNN handle.\n* `executionPlan` [in]  -  Finalized execution plan descriptor.\n* `variantPack` [in]  -    Finalized variant pack with updated data pointers.\n* `graph` [inout]  -          CUDA graph to update.\n@retval CUDNN_STATUS_SUCCESS\n@retval CUDNN_STATUS_BAD_PARAM\n@retval CUDNN_STATUS_INTERNAL_ERROR\n@retval CUDNN_STATUS_EXECUTION_FAILED\n@retval CUDNN_STATUS_NOT_SUPPORTED\n> **Since** cuDNN 9.5.0"]
pub unsafe fn cudnnBackendUpdateCudaGraph(handle: cudnnHandle_t, executionPlan: cudnnBackendDescriptor_t, variantPack: cudnnBackendDescriptor_t, graph: cudaGraph_t) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnBackendUpdateCudaGraph(handle, executionPlan, variantPack, graph) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Creates a tensor descriptor.\nAllocates and initializes a new tensor descriptor object.\n\n# Arguments\n\n* `tensorDesc` [out]  -  Pointer to the newly created tensor descriptor.\n@retval CUDNN_STATUS_SUCCESS           The descriptor was created successfully.\n@retval CUDNN_STATUS_ALLOC_FAILED      Memory allocation failed.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnDestroyTensorDescriptor,`] cudnnSetTensor4dDescriptor"]
pub unsafe fn cudnnCreateTensorDescriptor() -> Result<cudnnTensorDescriptor_t, crate::sys::cudnnStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cudnnTensorDescriptor_t> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnCreateTensorDescriptor(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cudnnTensorDescriptor_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Sets a 4D tensor descriptor.\nInitializes a previously created tensor descriptor with the specified format,\ndata type, and dimensions. Strides are computed automatically based on the format.\n\n# Arguments\n\n* `tensorDesc` [in,out]  -  Tensor descriptor to initialize.\n* `format` [in]  -      Memory layout format (e.g., NCHW or NHWC).\n* `dataType` [in]  -    Data type of the tensor elements.\n* `n` [in]  -           Number of images (batch size).\n* `c` [in]  -           Number of feature maps (channels).\n* `h` [in]  -           Height of each feature map.\n* `w` [in]  -           Width of each feature map.\n@retval CUDNN_STATUS_SUCCESS           The descriptor was set successfully.\n@retval CUDNN_STATUS_BAD_PARAM         An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnSetTensor4dDescriptorEx,`] cudnnGetTensor4dDescriptor"]
pub unsafe fn cudnnSetTensor4dDescriptor(tensorDesc: cudnnTensorDescriptor_t, format: cudnnTensorFormat_t, dataType: cudnnDataType_t, n: ::core::ffi::c_int, c: ::core::ffi::c_int, h: ::core::ffi::c_int, w: ::core::ffi::c_int) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnSetTensor4dDescriptor(tensorDesc, format, dataType, n, c, h, w) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Sets a 4D tensor descriptor with explicit strides.\nInitializes a previously created tensor descriptor with the specified data type,\ndimensions, and explicit stride values for each dimension.\n\n# Arguments\n\n* `tensorDesc` [in,out]  -  Tensor descriptor to initialize.\n* `dataType` [in]  -    Data type of the tensor elements.\n* `n` [in]  -           Number of images (batch size).\n* `c` [in]  -           Number of feature maps (channels).\n* `h` [in]  -           Height of each feature map.\n* `w` [in]  -           Width of each feature map.\n* `nStride` [in]  -     Stride between images.\n* `cStride` [in]  -     Stride between feature maps.\n* `hStride` [in]  -     Stride between rows.\n* `wStride` [in]  -     Stride between columns.\n@retval CUDNN_STATUS_SUCCESS           The descriptor was set successfully.\n@retval CUDNN_STATUS_BAD_PARAM         An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnSetTensor4dDescriptor,`] cudnnGetTensor4dDescriptor"]
pub unsafe fn cudnnSetTensor4dDescriptorEx(
    tensorDesc: cudnnTensorDescriptor_t,
    dataType: cudnnDataType_t,
    n: ::core::ffi::c_int,
    c: ::core::ffi::c_int,
    h: ::core::ffi::c_int,
    w: ::core::ffi::c_int,
    nStride: ::core::ffi::c_int,
    cStride: ::core::ffi::c_int,
    hStride: ::core::ffi::c_int,
    wStride: ::core::ffi::c_int,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnSetTensor4dDescriptorEx(tensorDesc, dataType, n, c, h, w, nStride, cStride, hStride, wStride) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Retrieves the settings of a previously initialized 4D tensor descriptor.\n\n# Arguments\n\n* `tensorDesc` [in]  -  Tensor descriptor to query.\n* `dataType` [out]  -    Data type of the tensor.\n* `n` [out]  -           Number of images (batch size).\n* `c` [out]  -           Number of feature maps (channels).\n* `h` [out]  -           Height of each feature map.\n* `w` [out]  -           Width of each feature map.\n* `nStride` [out]  -     Stride between images.\n* `cStride` [out]  -     Stride between feature maps.\n* `hStride` [out]  -     Stride between rows.\n* `wStride` [out]  -     Stride between columns.\n@retval CUDNN_STATUS_SUCCESS  The descriptor was queried successfully.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnSetTensor4dDescriptor`]"]
pub unsafe fn cudnnGetTensor4dDescriptor(tensorDesc: cudnnTensorDescriptor_t) -> Result<(cudnnDataType_t, ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int), crate::sys::cudnnStatus_t> {
    let mut out_1: std::mem::MaybeUninit<cudnnDataType_t> = std::mem::MaybeUninit::zeroed();
    let mut out_2: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let mut out_3: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let mut out_4: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let mut out_5: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let mut out_6: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let mut out_7: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let mut out_8: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let mut out_9: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
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
                out_2.assume_init() as ::core::ffi::c_int,
                out_3.assume_init() as ::core::ffi::c_int,
                out_4.assume_init() as ::core::ffi::c_int,
                out_5.assume_init() as ::core::ffi::c_int,
                out_6.assume_init() as ::core::ffi::c_int,
                out_7.assume_init() as ::core::ffi::c_int,
                out_8.assume_init() as ::core::ffi::c_int,
                out_9.assume_init() as ::core::ffi::c_int,
            ))
        }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Sets an N-dimensional tensor descriptor.\nInitializes a tensor descriptor with arbitrary dimensionality, data type, dimensions, and strides.\n\n# Arguments\n\n* `tensorDesc` [in,out]  -  Tensor descriptor to initialize.\n* `dataType` [in]  -    Data type of the tensor elements.\n* `nbDims` [in]  -      Number of dimensions.\n* `dimA` [in]  -        Array of dimension sizes (length nbDims).\n* `strideA` [in]  -     Array of strides (length nbDims).\n@retval CUDNN_STATUS_SUCCESS     The descriptor was set successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnGetTensorNdDescriptor`]"]
pub unsafe fn cudnnSetTensorNdDescriptor<T0: types::CudaAsPtr, T1: types::CudaAsPtr>(tensorDesc: cudnnTensorDescriptor_t, dataType: cudnnDataType_t, nbDims: ::core::ffi::c_int, dimA: T0, strideA: T1) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnSetTensorNdDescriptor(tensorDesc, dataType, nbDims, dimA.as_const_ptr() as *const _, strideA.as_const_ptr() as *const _) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Sets an N-dimensional tensor descriptor with automatic stride computation.\nInitializes a tensor descriptor with the specified format; strides are computed\nautomatically from the format and dimensions.\n\n# Arguments\n\n* `tensorDesc` [in,out]  -  Tensor descriptor to initialize.\n* `format` [in]  -      Memory layout format.\n* `dataType` [in]  -    Data type of the tensor elements.\n* `nbDims` [in]  -      Number of dimensions.\n* `dimA` [in]  -        Array of dimension sizes (length nbDims).\n@retval CUDNN_STATUS_SUCCESS     The descriptor was set successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnSetTensorNdDescriptor`]"]
pub unsafe fn cudnnSetTensorNdDescriptorEx<T0: types::CudaAsPtr>(tensorDesc: cudnnTensorDescriptor_t, format: cudnnTensorFormat_t, dataType: cudnnDataType_t, nbDims: ::core::ffi::c_int, dimA: T0) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnSetTensorNdDescriptorEx(tensorDesc, format, dataType, nbDims, dimA.as_const_ptr() as *const _) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Retrieves the settings of a previously initialized N-dimensional tensor descriptor.\n\n# Arguments\n\n* `tensorDesc` [in]  -      Tensor descriptor to query.\n* `nbDimsRequested` [in]  - Number of dimensions to retrieve.\n* `dataType` [out]  -        Data type of the tensor.\n* `nbDims` [out]  -          Actual number of dimensions in the descriptor.\n* `dimA` [out]  -            Array to receive dimension sizes (length nbDimsRequested).\n* `strideA` [out]  -         Array to receive strides (length nbDimsRequested).\n@retval CUDNN_STATUS_SUCCESS  The descriptor was queried successfully.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnSetTensorNdDescriptor`]"]
pub unsafe fn cudnnGetTensorNdDescriptor(tensorDesc: cudnnTensorDescriptor_t, nbDimsRequested: ::core::ffi::c_int) -> Result<(cudnnDataType_t, ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int), crate::sys::cudnnStatus_t> {
    let mut out_2: std::mem::MaybeUninit<cudnnDataType_t> = std::mem::MaybeUninit::zeroed();
    let mut out_3: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let mut out_4: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let mut out_5: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnGetTensorNdDescriptor(tensorDesc, nbDimsRequested, out_2.as_mut_ptr() as *mut _, out_3.as_mut_ptr() as *mut _, out_4.as_mut_ptr() as *mut _, out_5.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok((out_2.assume_init() as cudnnDataType_t, out_3.assume_init() as ::core::ffi::c_int, out_4.assume_init() as ::core::ffi::c_int, out_5.assume_init() as ::core::ffi::c_int)) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Returns the memory size in bytes required by a tensor.\n\n# Arguments\n\n* `tensorDesc` [in]  -  Tensor descriptor to query.\n* `size` [out]  -        Memory size in bytes.\n@retval CUDNN_STATUS_SUCCESS  The size was returned successfully.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnGetTensorSizeInBytes(tensorDesc: cudnnTensorDescriptor_t) -> Result<usize, crate::sys::cudnnStatus_t> {
    let mut out_1: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnGetTensorSizeInBytes(tensorDesc, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as usize) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Destroys a tensor descriptor.\nReleases the resources associated with a tensor descriptor object.\n\n# Arguments\n\n* `tensorDesc` [in]  -  Tensor descriptor to destroy.\n@retval CUDNN_STATUS_SUCCESS  The descriptor was destroyed successfully.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnCreateTensorDescriptor`]"]
pub unsafe fn cudnnDestroyTensorDescriptor(tensorDesc: cudnnTensorDescriptor_t) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnDestroyTensorDescriptor(tensorDesc) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Initializes the destination tensor descriptor for a tensor transform.\nComputes the destination tensor dimensions and size based on the transform and source descriptors.\n\n# Arguments\n\n* `transformDesc` [in]  -   Transform descriptor specifying the operation.\n* `srcDesc` [in]  -         Source tensor descriptor.\n* `destDesc` [in,out]  -        Destination tensor descriptor to be initialized.\n* `destSizeInBytes` [out]  - Memory size in bytes of the destination tensor.\n@retval CUDNN_STATUS_SUCCESS     The destination descriptor was initialized successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnTransformTensorEx`]"]
pub unsafe fn cudnnInitTransformDest<T0: types::CudaAsMutPtr>(transformDesc: cudnnTensorTransformDescriptor_t, srcDesc: cudnnTensorDescriptor_t, destDesc: cudnnTensorDescriptor_t, mut destSizeInBytes: T0) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnInitTransformDest(transformDesc, srcDesc, destDesc, destSizeInBytes.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Creates a tensor transform descriptor.\nAllocates and initializes a new tensor transform descriptor object.\n\n# Arguments\n\n* `transformDesc` [out]  -  Pointer to the newly created transform descriptor.\n@retval CUDNN_STATUS_SUCCESS       The descriptor was created successfully.\n@retval CUDNN_STATUS_ALLOC_FAILED  Memory allocation failed.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnDestroyTensorTransformDescriptor,`] cudnnSetTensorTransformDescriptor"]
pub unsafe fn cudnnCreateTensorTransformDescriptor() -> Result<cudnnTensorTransformDescriptor_t, crate::sys::cudnnStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cudnnTensorTransformDescriptor_t> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnCreateTensorTransformDescriptor(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cudnnTensorTransformDescriptor_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Configures a tensor transform descriptor.\nSets the parameters of a previously created tensor transform descriptor including\npadding, folding, and destination format.\n\n# Arguments\n\n* `transformDesc` [in,out]  -  Transform descriptor to configure.\n* `nbDims` [in]  -         Number of dimensions.\n* `destFormat` [in]  -     Destination tensor format.\n* `padBeforeA` [in]  -     Array of padding values before each dimension.\n* `padAfterA` [in]  -      Array of padding values after each dimension.\n* `foldA` [in]  -          Array of fold parameters per dimension.\n* `direction` [in]  -      Folding direction (fold or unfold).\n@retval CUDNN_STATUS_SUCCESS     The descriptor was configured successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnGetTensorTransformDescriptor`]"]
pub unsafe fn cudnnSetTensorTransformDescriptor<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr>(
    transformDesc: cudnnTensorTransformDescriptor_t,
    nbDims: u32,
    destFormat: cudnnTensorFormat_t,
    padBeforeA: T0,
    padAfterA: T1,
    foldA: T2,
    direction: cudnnFoldingDirection_t,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnSetTensorTransformDescriptor(transformDesc, nbDims, destFormat, padBeforeA.as_const_ptr() as *const _, padAfterA.as_const_ptr() as *const _, foldA.as_const_ptr() as *const _, direction) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Retrieves the settings of a previously initialized tensor transform descriptor.\n\n# Arguments\n\n* `transformDesc` [in]  -   Transform descriptor to query.\n* `nbDimsRequested` [in]  - Number of dimensions to retrieve.\n* `destFormat` [out]  -      Destination tensor format.\n* `padBeforeA` [out]  -      Array to receive pre-padding values.\n* `padAfterA` [out]  -       Array to receive post-padding values.\n* `foldA` [out]  -           Array to receive fold parameters.\n* `direction` [out]  -       Folding direction.\n@retval CUDNN_STATUS_SUCCESS  The descriptor was queried successfully.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnSetTensorTransformDescriptor`]"]
pub unsafe fn cudnnGetTensorTransformDescriptor(transformDesc: cudnnTensorTransformDescriptor_t, nbDimsRequested: u32) -> Result<(cudnnTensorFormat_t, i32, i32, u32, cudnnFoldingDirection_t), crate::sys::cudnnStatus_t> {
    let mut out_2: std::mem::MaybeUninit<cudnnTensorFormat_t> = std::mem::MaybeUninit::zeroed();
    let mut out_3: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::zeroed();
    let mut out_4: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::zeroed();
    let mut out_5: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::zeroed();
    let mut out_6: std::mem::MaybeUninit<cudnnFoldingDirection_t> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnGetTensorTransformDescriptor(transformDesc, nbDimsRequested, out_2.as_mut_ptr() as *mut _, out_3.as_mut_ptr() as *mut _, out_4.as_mut_ptr() as *mut _, out_5.as_mut_ptr() as *mut _, out_6.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok((out_2.assume_init() as cudnnTensorFormat_t, out_3.assume_init() as i32, out_4.assume_init() as i32, out_5.assume_init() as u32, out_6.assume_init() as cudnnFoldingDirection_t)) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Destroys a tensor transform descriptor.\nReleases the resources associated with a tensor transform descriptor.\n\n# Arguments\n\n* `transformDesc` [in]  -  Transform descriptor to destroy.\n@retval CUDNN_STATUS_SUCCESS  The descriptor was destroyed successfully.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnCreateTensorTransformDescriptor`]"]
pub unsafe fn cudnnDestroyTensorTransformDescriptor(transformDesc: cudnnTensorTransformDescriptor_t) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnDestroyTensorTransformDescriptor(transformDesc) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Copies and converts tensor data between layouts with alpha/beta blending.\nPerforms y = alpha * x + beta * y, converting between tensor formats as needed.\n\n# Arguments\n\n* `handle` [in]  -  cuDNN library handle.\n* `alpha` [in]  -   Scaling factor for the source tensor.\n* `xDesc` [in]  -   Source tensor descriptor.\n* `x` [in]  -       Pointer to source tensor data.\n* `beta` [in]  -    Scaling factor for the destination tensor.\n* `yDesc` [in]  -   Destination tensor descriptor.\n* `y` [in,out]  -       Pointer to destination tensor data.\n@retval CUDNN_STATUS_SUCCESS     The operation completed successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnTransformTensorEx`]"]
pub unsafe fn cudnnTransformTensor<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(handle: cudnnHandle_t, alpha: T0, xDesc: cudnnTensorDescriptor_t, x: T1, beta: T2, yDesc: cudnnTensorDescriptor_t, mut y: T3) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnTransformTensor(handle, alpha.as_const_ptr() as *const _, xDesc, x.as_const_ptr() as *const _, beta.as_const_ptr() as *const _, yDesc, y.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Extended tensor transform with folding/padding support.\nPerforms dest = alpha * transform(src) + beta * dest, using the specified\ntransform descriptor for padding and folding configuration.\n\n# Arguments\n\n* `handle` [in]  -    cuDNN library handle.\n* `transDesc` [in]  - Transform descriptor specifying the operation.\n* `alpha` [in]  -     Scaling factor for the source tensor.\n* `srcDesc` [in]  -   Source tensor descriptor.\n* `srcData` [in]  -   Pointer to source tensor data.\n* `beta` [in]  -      Scaling factor for the destination tensor.\n* `destDesc` [in]  -  Destination tensor descriptor.\n* `destData` [in,out]  -  Pointer to destination tensor data.\n@retval CUDNN_STATUS_SUCCESS     The operation completed successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnTransformTensor,`] cudnnSetTensorTransformDescriptor"]
pub unsafe fn cudnnTransformTensorEx<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(
    handle: cudnnHandle_t,
    transDesc: cudnnTensorTransformDescriptor_t,
    alpha: T0,
    srcDesc: cudnnTensorDescriptor_t,
    srcData: T1,
    beta: T2,
    destDesc: cudnnTensorDescriptor_t,
    mut destData: T3,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnTransformTensorEx(handle, transDesc, alpha.as_const_ptr() as *const _, srcDesc, srcData.as_const_ptr() as *const _, beta.as_const_ptr() as *const _, destDesc, destData.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Adds a scaled bias tensor to a destination tensor with broadcasting.\nPerforms C = alpha * A + beta * C, where A is broadcast to match C dimensions.\n\n# Arguments\n\n* `handle` [in]  -  cuDNN library handle.\n* `alpha` [in]  -   Scaling factor for the bias tensor A.\n* `aDesc` [in]  -   Bias tensor descriptor.\n* `A` [in]  -       Pointer to bias tensor data.\n* `beta` [in]  -    Scaling factor for the destination tensor C.\n* `cDesc` [in]  -   Destination tensor descriptor.\n* `C` [in,out]  -       Pointer to destination tensor data.\n@retval CUDNN_STATUS_SUCCESS     The operation completed successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead."]
pub unsafe fn cudnnAddTensor<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(handle: cudnnHandle_t, alpha: T0, aDesc: cudnnTensorDescriptor_t, A: T1, beta: T2, cDesc: cudnnTensorDescriptor_t, mut C: T3) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnAddTensor(handle, alpha.as_const_ptr() as *const _, aDesc, A.as_const_ptr() as *const _, beta.as_const_ptr() as *const _, cDesc, C.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Creates an op tensor descriptor.\n\n# Arguments\n\n* `opTensorDesc` [out]  -  Pointer to the newly created op tensor descriptor.\n@retval CUDNN_STATUS_SUCCESS       The descriptor was created successfully.\n@retval CUDNN_STATUS_ALLOC_FAILED  Memory allocation failed.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnDestroyOpTensorDescriptor`]"]
pub unsafe fn cudnnCreateOpTensorDescriptor() -> Result<cudnnOpTensorDescriptor_t, crate::sys::cudnnStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cudnnOpTensorDescriptor_t> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnCreateOpTensorDescriptor(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cudnnOpTensorDescriptor_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Configures an op tensor descriptor.\n\n# Arguments\n\n* `opTensorDesc` [in,out]  -     Op tensor descriptor to configure.\n* `opTensorOp` [in]  -       Tensor operation to perform.\n* `opTensorCompType` [in]  - Computation data type.\n* `opTensorNanOpt` [in]  -   NaN propagation policy.\n@retval CUDNN_STATUS_SUCCESS     The descriptor was set successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnGetOpTensorDescriptor`]"]
pub unsafe fn cudnnSetOpTensorDescriptor(opTensorDesc: cudnnOpTensorDescriptor_t, opTensorOp: cudnnOpTensorOp_t, opTensorCompType: cudnnDataType_t, opTensorNanOpt: cudnnNanPropagation_t) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnSetOpTensorDescriptor(opTensorDesc, opTensorOp, opTensorCompType, opTensorNanOpt) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Retrieves the settings of an op tensor descriptor.\n\n# Arguments\n\n* `opTensorDesc` [in]  -     Op tensor descriptor to query.\n* `opTensorOp` [out]  -       Tensor operation type.\n* `opTensorCompType` [out]  - Computation data type.\n* `opTensorNanOpt` [out]  -   NaN propagation policy.\n@retval CUDNN_STATUS_SUCCESS  The descriptor was queried successfully.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnSetOpTensorDescriptor`]"]
pub unsafe fn cudnnGetOpTensorDescriptor(opTensorDesc: cudnnOpTensorDescriptor_t) -> Result<(cudnnOpTensorOp_t, cudnnDataType_t, cudnnNanPropagation_t), crate::sys::cudnnStatus_t> {
    let mut out_1: std::mem::MaybeUninit<cudnnOpTensorOp_t> = std::mem::MaybeUninit::zeroed();
    let mut out_2: std::mem::MaybeUninit<cudnnDataType_t> = std::mem::MaybeUninit::zeroed();
    let mut out_3: std::mem::MaybeUninit<cudnnNanPropagation_t> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnGetOpTensorDescriptor(opTensorDesc, out_1.as_mut_ptr() as *mut _, out_2.as_mut_ptr() as *mut _, out_3.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok((out_1.assume_init() as cudnnOpTensorOp_t, out_2.assume_init() as cudnnDataType_t, out_3.assume_init() as cudnnNanPropagation_t)) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Destroys an op tensor descriptor.\n\n# Arguments\n\n* `opTensorDesc` [in]  -  Op tensor descriptor to destroy.\n@retval CUDNN_STATUS_SUCCESS  The descriptor was destroyed successfully.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnCreateOpTensorDescriptor`]"]
pub unsafe fn cudnnDestroyOpTensorDescriptor(opTensorDesc: cudnnOpTensorDescriptor_t) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnDestroyOpTensorDescriptor(opTensorDesc) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Performs element-wise tensor operations.\nComputes C = op(alpha1 * A, alpha2 * B) + beta * C. The B tensor is ignored\nfor CUDNN_OP_TENSOR_SQRT and CUDNN_OP_TENSOR_NOT (unary operations).\n\n# Arguments\n\n* `handle` [in]  -       cuDNN library handle.\n* `opTensorDesc` [in]  - Op tensor descriptor specifying the operation.\n* `alpha1` [in]  -       Scaling factor for tensor A.\n* `aDesc` [in]  -        Descriptor for tensor A.\n* `A` [in]  -            Pointer to tensor A data.\n* `alpha2` [in]  -       Scaling factor for tensor B.\n* `bDesc` [in]  -        Descriptor for tensor B.\n* `B` [in]  -            Pointer to tensor B data.\n* `beta` [in]  -         Scaling factor for tensor C.\n* `cDesc` [in]  -        Descriptor for tensor C.\n* `C` [in,out]  -            Pointer to tensor C data.\n@retval CUDNN_STATUS_SUCCESS     The operation completed successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnSetOpTensorDescriptor`]"]
pub unsafe fn cudnnOpTensor<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsPtr, T5: types::CudaAsMutPtr>(
    handle: cudnnHandle_t,
    opTensorDesc: cudnnOpTensorDescriptor_t,
    alpha1: T0,
    aDesc: cudnnTensorDescriptor_t,
    A: T1,
    alpha2: T2,
    bDesc: cudnnTensorDescriptor_t,
    B: T3,
    beta: T4,
    cDesc: cudnnTensorDescriptor_t,
    mut C: T5,
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
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Creates a reduce tensor descriptor.\n\n# Arguments\n\n* `reduceTensorDesc` [out]  -  Pointer to the newly created reduce tensor descriptor.\n@retval CUDNN_STATUS_SUCCESS       The descriptor was created successfully.\n@retval CUDNN_STATUS_ALLOC_FAILED  Memory allocation failed.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnDestroyReduceTensorDescriptor`]"]
pub unsafe fn cudnnCreateReduceTensorDescriptor() -> Result<cudnnReduceTensorDescriptor_t, crate::sys::cudnnStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cudnnReduceTensorDescriptor_t> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnCreateReduceTensorDescriptor(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cudnnReduceTensorDescriptor_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Configures a reduce tensor descriptor.\n\n# Arguments\n\n* `reduceTensorDesc` [in,out]  -        Reduce tensor descriptor to configure.\n* `reduceTensorOp` [in]  -          Reduction operation to perform.\n* `reduceTensorCompType` [in]  -    Computation data type.\n* `reduceTensorNanOpt` [in]  -      NaN propagation policy (applies to min/max only).\n* `reduceTensorIndices` [in]  -     Whether to compute indices.\n* `reduceTensorIndicesType` [in]  - Data type for computed indices.\n@retval CUDNN_STATUS_SUCCESS     The descriptor was set successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnGetReduceTensorDescriptor`]"]
pub unsafe fn cudnnSetReduceTensorDescriptor(
    reduceTensorDesc: cudnnReduceTensorDescriptor_t,
    reduceTensorOp: cudnnReduceTensorOp_t,
    reduceTensorCompType: cudnnDataType_t,
    reduceTensorNanOpt: cudnnNanPropagation_t,
    reduceTensorIndices: cudnnReduceTensorIndices_t,
    reduceTensorIndicesType: cudnnIndicesType_t,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnSetReduceTensorDescriptor(reduceTensorDesc, reduceTensorOp, reduceTensorCompType, reduceTensorNanOpt, reduceTensorIndices, reduceTensorIndicesType) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Retrieves the settings of a reduce tensor descriptor.\n\n# Arguments\n\n* `reduceTensorDesc` [in]  -        Reduce tensor descriptor to query.\n* `reduceTensorOp` [out]  -          Reduction operation type.\n* `reduceTensorCompType` [out]  -    Computation data type.\n* `reduceTensorNanOpt` [out]  -      NaN propagation policy.\n* `reduceTensorIndices` [out]  -     Whether indices are computed.\n* `reduceTensorIndicesType` [out]  - Data type for computed indices.\n@retval CUDNN_STATUS_SUCCESS  The descriptor was queried successfully.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnSetReduceTensorDescriptor`]"]
pub unsafe fn cudnnGetReduceTensorDescriptor(reduceTensorDesc: cudnnReduceTensorDescriptor_t) -> Result<(cudnnReduceTensorOp_t, cudnnDataType_t, cudnnNanPropagation_t, cudnnReduceTensorIndices_t, cudnnIndicesType_t), crate::sys::cudnnStatus_t> {
    let mut out_1: std::mem::MaybeUninit<cudnnReduceTensorOp_t> = std::mem::MaybeUninit::zeroed();
    let mut out_2: std::mem::MaybeUninit<cudnnDataType_t> = std::mem::MaybeUninit::zeroed();
    let mut out_3: std::mem::MaybeUninit<cudnnNanPropagation_t> = std::mem::MaybeUninit::zeroed();
    let mut out_4: std::mem::MaybeUninit<cudnnReduceTensorIndices_t> = std::mem::MaybeUninit::zeroed();
    let mut out_5: std::mem::MaybeUninit<cudnnIndicesType_t> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnGetReduceTensorDescriptor(reduceTensorDesc, out_1.as_mut_ptr() as *mut _, out_2.as_mut_ptr() as *mut _, out_3.as_mut_ptr() as *mut _, out_4.as_mut_ptr() as *mut _, out_5.as_mut_ptr() as *mut _) };
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
#[doc = "Destroys a reduce tensor descriptor.\n\n# Arguments\n\n* `reduceTensorDesc` [in]  -  Reduce tensor descriptor to destroy.\n@retval CUDNN_STATUS_SUCCESS  The descriptor was destroyed successfully.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnCreateReduceTensorDescriptor`]"]
pub unsafe fn cudnnDestroyReduceTensorDescriptor(reduceTensorDesc: cudnnReduceTensorDescriptor_t) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnDestroyReduceTensorDescriptor(reduceTensorDesc) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Returns the minimum size of the index space for a reduction operation.\n\n# Arguments\n\n* `handle` [in]  -           cuDNN library handle.\n* `reduceTensorDesc` [in]  - Reduce tensor descriptor.\n* `aDesc` [in]  -            Input tensor descriptor.\n* `cDesc` [in]  -            Output tensor descriptor.\n* `sizeInBytes` [out]  -      Minimum index space size in bytes.\n@retval CUDNN_STATUS_SUCCESS  The size was returned successfully.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnReduceTensor`]"]
pub unsafe fn cudnnGetReductionIndicesSize(handle: cudnnHandle_t, reduceTensorDesc: cudnnReduceTensorDescriptor_t, aDesc: cudnnTensorDescriptor_t, cDesc: cudnnTensorDescriptor_t) -> Result<usize, crate::sys::cudnnStatus_t> {
    let mut out_4: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnGetReductionIndicesSize(handle, reduceTensorDesc, aDesc, cDesc, out_4.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_4.assume_init() as usize) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Returns the minimum workspace size required for a reduction operation.\n\n# Arguments\n\n* `handle` [in]  -           cuDNN library handle.\n* `reduceTensorDesc` [in]  - Reduce tensor descriptor.\n* `aDesc` [in]  -            Input tensor descriptor.\n* `cDesc` [in]  -            Output tensor descriptor.\n* `sizeInBytes` [out]  -      Minimum workspace size in bytes.\n@retval CUDNN_STATUS_SUCCESS  The size was returned successfully.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnReduceTensor`]"]
pub unsafe fn cudnnGetReductionWorkspaceSize(handle: cudnnHandle_t, reduceTensorDesc: cudnnReduceTensorDescriptor_t, aDesc: cudnnTensorDescriptor_t, cDesc: cudnnTensorDescriptor_t) -> Result<usize, crate::sys::cudnnStatus_t> {
    let mut out_4: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnGetReductionWorkspaceSize(handle, reduceTensorDesc, aDesc, cDesc, out_4.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_4.assume_init() as usize) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Performs a tensor reduction operation.\nComputes C = reduce_op(alpha * A) + beta * C. NaN propagation applies only\nto min and max operations. The indices space is ignored for operations other\nthan min or max.\n\n# Arguments\n\n* `handle` [in]  -              cuDNN library handle.\n* `reduceTensorDesc` [in]  -    Reduce tensor descriptor.\n* `indices` [out]  -             Pointer to index space (for min/max ops).\n* `indicesSizeInBytes` [in]  -  Size of the index space in bytes.\n* `workspace` [out]  -           Pointer to workspace memory.\n* `workspaceSizeInBytes` [in]  - Size of the workspace in bytes.\n* `alpha` [in]  -               Scaling factor for the input tensor.\n* `aDesc` [in]  -               Input tensor descriptor.\n* `A` [in]  -                   Pointer to input tensor data.\n* `beta` [in]  -                Scaling factor for the output tensor.\n* `cDesc` [in]  -               Output tensor descriptor.\n* `C` [in,out]  -                   Pointer to output tensor data.\n@retval CUDNN_STATUS_SUCCESS     The operation completed successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnGetReductionWorkspaceSize,`] cudnnGetReductionIndicesSize"]
pub unsafe fn cudnnReduceTensor<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsPtr, T5: types::CudaAsMutPtr>(
    handle: cudnnHandle_t,
    reduceTensorDesc: cudnnReduceTensorDescriptor_t,
    mut indices: T0,
    indicesSizeInBytes: usize,
    mut workspace: T1,
    workspaceSizeInBytes: usize,
    alpha: T2,
    aDesc: cudnnTensorDescriptor_t,
    A: T3,
    beta: T4,
    cDesc: cudnnTensorDescriptor_t,
    mut C: T5,
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
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Fills a tensor with a constant value.\nSets every element of the tensor to the specified value: y[i] = value[0].\n\n# Arguments\n\n* `handle` [in]  -    cuDNN library handle.\n* `yDesc` [in]  -     Tensor descriptor.\n* `y` [out]  -         Pointer to tensor data.\n* `valuePtr` [in]  -  Pointer to the fill value (type matches tensor data type).\n@retval CUDNN_STATUS_SUCCESS     The operation completed successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnScaleTensor`]"]
pub unsafe fn cudnnSetTensor<T0: types::CudaAsMutPtr, T1: types::CudaAsPtr>(handle: cudnnHandle_t, yDesc: cudnnTensorDescriptor_t, mut y: T0, valuePtr: T1) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnSetTensor(handle, yDesc, y.as_mut_ptr() as *mut _, valuePtr.as_const_ptr() as *const _) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Scales all elements of a tensor by a constant factor.\nPerforms y[i] = alpha * y[i] for every element.\n\n# Arguments\n\n* `handle` [in]  -  cuDNN library handle.\n* `yDesc` [in]  -   Tensor descriptor.\n* `y` [in,out]  -       Pointer to tensor data.\n* `alpha` [in]  -   Scaling factor (type matches tensor computation type).\n@retval CUDNN_STATUS_SUCCESS     The operation completed successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnSetTensor`]"]
pub unsafe fn cudnnScaleTensor<T0: types::CudaAsMutPtr, T1: types::CudaAsPtr>(handle: cudnnHandle_t, yDesc: cudnnTensorDescriptor_t, mut y: T0, alpha: T1) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnScaleTensor(handle, yDesc, y.as_mut_ptr() as *mut _, alpha.as_const_ptr() as *const _) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Creates a filter descriptor.\nAllocates and initializes a new filter (convolution kernel) descriptor.\n\n# Arguments\n\n* `filterDesc` [out]  -  Pointer to the newly created filter descriptor.\n@retval CUDNN_STATUS_SUCCESS       The descriptor was created successfully.\n@retval CUDNN_STATUS_ALLOC_FAILED  Memory allocation failed.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnDestroyFilterDescriptor`]"]
pub unsafe fn cudnnCreateFilterDescriptor() -> Result<cudnnFilterDescriptor_t, crate::sys::cudnnStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cudnnFilterDescriptor_t> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnCreateFilterDescriptor(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cudnnFilterDescriptor_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Sets a 4D filter descriptor.\nInitializes a filter descriptor with the specified data type, format, and dimensions.\n\n# Arguments\n\n* `filterDesc` [in,out]  -  Filter descriptor to initialize.\n* `dataType` [in]  -    Data type of the filter elements.\n* `format` [in]  -      Memory layout format.\n* `k` [in]  -           Number of output feature maps.\n* `c` [in]  -           Number of input feature maps.\n* `h` [in]  -           Height of each filter.\n* `w` [in]  -           Width of each filter.\n@retval CUDNN_STATUS_SUCCESS     The descriptor was set successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnGetFilter4dDescriptor`]"]
pub unsafe fn cudnnSetFilter4dDescriptor(filterDesc: cudnnFilterDescriptor_t, dataType: cudnnDataType_t, format: cudnnTensorFormat_t, k: ::core::ffi::c_int, c: ::core::ffi::c_int, h: ::core::ffi::c_int, w: ::core::ffi::c_int) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnSetFilter4dDescriptor(filterDesc, dataType, format, k, c, h, w) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Retrieves the settings of a 4D filter descriptor.\n\n# Arguments\n\n* `filterDesc` [in]  -  Filter descriptor to query.\n* `dataType` [out]  -    Data type of the filter.\n* `format` [out]  -      Memory layout format.\n* `k` [out]  -           Number of output feature maps.\n* `c` [out]  -           Number of input feature maps.\n* `h` [out]  -           Height of each filter.\n* `w` [out]  -           Width of each filter.\n@retval CUDNN_STATUS_SUCCESS  The descriptor was queried successfully.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnSetFilter4dDescriptor`]"]
pub unsafe fn cudnnGetFilter4dDescriptor(filterDesc: cudnnFilterDescriptor_t) -> Result<(cudnnDataType_t, cudnnTensorFormat_t, ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int), crate::sys::cudnnStatus_t> {
    let mut out_1: std::mem::MaybeUninit<cudnnDataType_t> = std::mem::MaybeUninit::zeroed();
    let mut out_2: std::mem::MaybeUninit<cudnnTensorFormat_t> = std::mem::MaybeUninit::zeroed();
    let mut out_3: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let mut out_4: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let mut out_5: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let mut out_6: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
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
                out_3.assume_init() as ::core::ffi::c_int,
                out_4.assume_init() as ::core::ffi::c_int,
                out_5.assume_init() as ::core::ffi::c_int,
                out_6.assume_init() as ::core::ffi::c_int,
            ))
        }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Sets an N-dimensional filter descriptor.\n\n# Arguments\n\n* `filterDesc` [in,out]  -  Filter descriptor to initialize.\n* `dataType` [in]  -    Data type of the filter elements.\n* `format` [in]  -      Memory layout format.\n* `nbDims` [in]  -      Number of dimensions.\n* `filterDimA` [in]  -  Array of filter dimension sizes (length nbDims).\n@retval CUDNN_STATUS_SUCCESS     The descriptor was set successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnGetFilterNdDescriptor`]"]
pub unsafe fn cudnnSetFilterNdDescriptor<T0: types::CudaAsPtr>(filterDesc: cudnnFilterDescriptor_t, dataType: cudnnDataType_t, format: cudnnTensorFormat_t, nbDims: ::core::ffi::c_int, filterDimA: T0) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnSetFilterNdDescriptor(filterDesc, dataType, format, nbDims, filterDimA.as_const_ptr() as *const _) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Retrieves the settings of an N-dimensional filter descriptor.\n\n# Arguments\n\n* `filterDesc` [in]  -      Filter descriptor to query.\n* `nbDimsRequested` [in]  - Number of dimensions to retrieve.\n* `dataType` [out]  -        Data type of the filter.\n* `format` [out]  -          Memory layout format.\n* `nbDims` [out]  -          Actual number of dimensions.\n* `filterDimA` [out]  -      Array to receive dimension sizes.\n@retval CUDNN_STATUS_SUCCESS  The descriptor was queried successfully.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnSetFilterNdDescriptor`]"]
pub unsafe fn cudnnGetFilterNdDescriptor(filterDesc: cudnnFilterDescriptor_t, nbDimsRequested: ::core::ffi::c_int) -> Result<(cudnnDataType_t, cudnnTensorFormat_t, ::core::ffi::c_int, ::core::ffi::c_int), crate::sys::cudnnStatus_t> {
    let mut out_2: std::mem::MaybeUninit<cudnnDataType_t> = std::mem::MaybeUninit::zeroed();
    let mut out_3: std::mem::MaybeUninit<cudnnTensorFormat_t> = std::mem::MaybeUninit::zeroed();
    let mut out_4: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let mut out_5: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnGetFilterNdDescriptor(filterDesc, nbDimsRequested, out_2.as_mut_ptr() as *mut _, out_3.as_mut_ptr() as *mut _, out_4.as_mut_ptr() as *mut _, out_5.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok((out_2.assume_init() as cudnnDataType_t, out_3.assume_init() as cudnnTensorFormat_t, out_4.assume_init() as ::core::ffi::c_int, out_5.assume_init() as ::core::ffi::c_int)) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Returns the memory size in bytes required by a filter.\n\n# Arguments\n\n* `filterDesc` [in]  -  Filter descriptor to query.\n* `size` [out]  -        Memory size in bytes.\n@retval CUDNN_STATUS_SUCCESS  The size was returned successfully.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead."]
pub unsafe fn cudnnGetFilterSizeInBytes(filterDesc: cudnnFilterDescriptor_t) -> Result<usize, crate::sys::cudnnStatus_t> {
    let mut out_1: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnGetFilterSizeInBytes(filterDesc, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as usize) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Transforms filter data between layouts.\nConverts filter data from one format to another using the specified transform descriptor.\n\n# Arguments\n\n* `handle` [in]  -    cuDNN library handle.\n* `transDesc` [in]  - Transform descriptor specifying the operation.\n* `alpha` [in]  -     Scaling factor for the source filter.\n* `srcDesc` [in]  -   Source filter descriptor.\n* `srcData` [in]  -   Pointer to source filter data.\n* `beta` [in]  -      Scaling factor for the destination filter.\n* `destDesc` [in]  -  Destination filter descriptor.\n* `destData` [in,out]  -  Pointer to destination filter data.\n@retval CUDNN_STATUS_SUCCESS     The operation completed successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnTransformTensorEx`]"]
pub unsafe fn cudnnTransformFilter<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(
    handle: cudnnHandle_t,
    transDesc: cudnnTensorTransformDescriptor_t,
    alpha: T0,
    srcDesc: cudnnFilterDescriptor_t,
    srcData: T1,
    beta: T2,
    destDesc: cudnnFilterDescriptor_t,
    mut destData: T3,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnTransformFilter(handle, transDesc, alpha.as_const_ptr() as *const _, srcDesc, srcData.as_const_ptr() as *const _, beta.as_const_ptr() as *const _, destDesc, destData.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Destroys a filter descriptor.\n\n# Arguments\n\n* `filterDesc` [in]  -  Filter descriptor to destroy.\n@retval CUDNN_STATUS_SUCCESS  The descriptor was destroyed successfully.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnCreateFilterDescriptor`]"]
pub unsafe fn cudnnDestroyFilterDescriptor(filterDesc: cudnnFilterDescriptor_t) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnDestroyFilterDescriptor(filterDesc) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Performs forward softmax computation.\nComputes y = alpha * softmax(x) + beta * y.\n\n# Arguments\n\n* `handle` [in]  -  cuDNN library handle.\n* `algo` [in]  -    Softmax algorithm to use.\n* `mode` [in]  -    Softmax computation scope.\n* `alpha` [in]  -   Scaling factor for the result.\n* `xDesc` [in]  -   Input tensor descriptor.\n* `x` [in]  -       Pointer to input tensor data.\n* `beta` [in]  -    Scaling factor for the destination tensor.\n* `yDesc` [in]  -   Output tensor descriptor.\n* `y` [in,out]  -       Pointer to output tensor data.\n@retval CUDNN_STATUS_SUCCESS     The operation completed successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnSoftmaxBackward`]"]
pub unsafe fn cudnnSoftmaxForward<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(
    handle: cudnnHandle_t,
    algo: cudnnSoftmaxAlgorithm_t,
    mode: cudnnSoftmaxMode_t,
    alpha: T0,
    xDesc: cudnnTensorDescriptor_t,
    x: T1,
    beta: T2,
    yDesc: cudnnTensorDescriptor_t,
    mut y: T3,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnSoftmaxForward(handle, algo, mode, alpha.as_const_ptr() as *const _, xDesc, x.as_const_ptr() as *const _, beta.as_const_ptr() as *const _, yDesc, y.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Creates a pooling descriptor.\n\n# Arguments\n\n* `poolingDesc` [out]  -  Pointer to the newly created pooling descriptor.\n@retval CUDNN_STATUS_SUCCESS       The descriptor was created successfully.\n@retval CUDNN_STATUS_ALLOC_FAILED  Memory allocation failed.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnDestroyPoolingDescriptor`]"]
pub unsafe fn cudnnCreatePoolingDescriptor() -> Result<cudnnPoolingDescriptor_t, crate::sys::cudnnStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cudnnPoolingDescriptor_t> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnCreatePoolingDescriptor(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cudnnPoolingDescriptor_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Configures a 2D pooling descriptor.\n\n# Arguments\n\n* `poolingDesc` [in,out]  -       Pooling descriptor to configure.\n* `mode` [in]  -              Pooling mode (max, average, etc.).\n* `maxpoolingNanOpt` [in]  -  NaN propagation policy for max pooling.\n* `windowHeight` [in]  -      Height of the pooling window.\n* `windowWidth` [in]  -       Width of the pooling window.\n* `verticalPadding` [in]  -   Vertical padding size.\n* `horizontalPadding` [in]  - Horizontal padding size.\n* `verticalStride` [in]  -    Vertical stride.\n* `horizontalStride` [in]  -  Horizontal stride.\n@retval CUDNN_STATUS_SUCCESS     The descriptor was set successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnGetPooling2dDescriptor`]"]
pub unsafe fn cudnnSetPooling2dDescriptor(
    poolingDesc: cudnnPoolingDescriptor_t,
    mode: cudnnPoolingMode_t,
    maxpoolingNanOpt: cudnnNanPropagation_t,
    windowHeight: ::core::ffi::c_int,
    windowWidth: ::core::ffi::c_int,
    verticalPadding: ::core::ffi::c_int,
    horizontalPadding: ::core::ffi::c_int,
    verticalStride: ::core::ffi::c_int,
    horizontalStride: ::core::ffi::c_int,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnSetPooling2dDescriptor(poolingDesc, mode, maxpoolingNanOpt, windowHeight, windowWidth, verticalPadding, horizontalPadding, verticalStride, horizontalStride) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Retrieves the settings of a 2D pooling descriptor.\n\n# Arguments\n\n* `poolingDesc` [in]  -       Pooling descriptor to query.\n* `mode` [out]  -              Pooling mode.\n* `maxpoolingNanOpt` [out]  -  NaN propagation policy.\n* `windowHeight` [out]  -      Height of the pooling window.\n* `windowWidth` [out]  -       Width of the pooling window.\n* `verticalPadding` [out]  -   Vertical padding size.\n* `horizontalPadding` [out]  - Horizontal padding size.\n* `verticalStride` [out]  -    Vertical stride.\n* `horizontalStride` [out]  -  Horizontal stride.\n@retval CUDNN_STATUS_SUCCESS  The descriptor was queried successfully.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnSetPooling2dDescriptor`]"]
pub unsafe fn cudnnGetPooling2dDescriptor(poolingDesc: cudnnPoolingDescriptor_t) -> Result<(cudnnPoolingMode_t, cudnnNanPropagation_t, ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int), crate::sys::cudnnStatus_t> {
    let mut out_1: std::mem::MaybeUninit<cudnnPoolingMode_t> = std::mem::MaybeUninit::zeroed();
    let mut out_2: std::mem::MaybeUninit<cudnnNanPropagation_t> = std::mem::MaybeUninit::zeroed();
    let mut out_3: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let mut out_4: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let mut out_5: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let mut out_6: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let mut out_7: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let mut out_8: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
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
                out_3.assume_init() as ::core::ffi::c_int,
                out_4.assume_init() as ::core::ffi::c_int,
                out_5.assume_init() as ::core::ffi::c_int,
                out_6.assume_init() as ::core::ffi::c_int,
                out_7.assume_init() as ::core::ffi::c_int,
                out_8.assume_init() as ::core::ffi::c_int,
            ))
        }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Configures an N-dimensional pooling descriptor.\n\n# Arguments\n\n* `poolingDesc` [in,out]  -     Pooling descriptor to configure.\n* `mode` [in]  -            Pooling mode.\n* `maxpoolingNanOpt` [in]  - NaN propagation policy for max pooling.\n* `nbDims` [in]  -          Number of dimensions.\n* `windowDimA` [in]  -      Array of pooling window sizes (length nbDims).\n* `paddingA` [in]  -        Array of padding sizes (length nbDims).\n* `strideA` [in]  -         Array of strides (length nbDims).\n@retval CUDNN_STATUS_SUCCESS     The descriptor was set successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnGetPoolingNdDescriptor`]"]
pub unsafe fn cudnnSetPoolingNdDescriptor<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr>(
    poolingDesc: cudnnPoolingDescriptor_t,
    mode: cudnnPoolingMode_t,
    maxpoolingNanOpt: cudnnNanPropagation_t,
    nbDims: ::core::ffi::c_int,
    windowDimA: T0,
    paddingA: T1,
    strideA: T2,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnSetPoolingNdDescriptor(poolingDesc, mode, maxpoolingNanOpt, nbDims, windowDimA.as_const_ptr() as *const _, paddingA.as_const_ptr() as *const _, strideA.as_const_ptr() as *const _) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Retrieves the settings of an N-dimensional pooling descriptor.\n\n# Arguments\n\n* `poolingDesc` [in]  -     Pooling descriptor to query.\n* `nbDimsRequested` [in]  - Number of dimensions to retrieve.\n* `mode` [out]  -            Pooling mode.\n* `maxpoolingNanOpt` [out]  - NaN propagation policy.\n* `nbDims` [out]  -          Actual number of dimensions.\n* `windowDimA` [out]  -      Array to receive window sizes.\n* `paddingA` [out]  -        Array to receive padding sizes.\n* `strideA` [out]  -         Array to receive strides.\n@retval CUDNN_STATUS_SUCCESS  The descriptor was queried successfully.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnSetPoolingNdDescriptor`]"]
pub unsafe fn cudnnGetPoolingNdDescriptor(poolingDesc: cudnnPoolingDescriptor_t, nbDimsRequested: ::core::ffi::c_int) -> Result<(cudnnPoolingMode_t, cudnnNanPropagation_t, ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int), crate::sys::cudnnStatus_t> {
    let mut out_2: std::mem::MaybeUninit<cudnnPoolingMode_t> = std::mem::MaybeUninit::zeroed();
    let mut out_3: std::mem::MaybeUninit<cudnnNanPropagation_t> = std::mem::MaybeUninit::zeroed();
    let mut out_4: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let mut out_5: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let mut out_6: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let mut out_7: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
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
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe {
            Ok((
                out_2.assume_init() as cudnnPoolingMode_t,
                out_3.assume_init() as cudnnNanPropagation_t,
                out_4.assume_init() as ::core::ffi::c_int,
                out_5.assume_init() as ::core::ffi::c_int,
                out_6.assume_init() as ::core::ffi::c_int,
                out_7.assume_init() as ::core::ffi::c_int,
            ))
        }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Computes the output dimensions of an N-dimensional pooling operation.\n\n# Arguments\n\n* `poolingDesc` [in]  -      Pooling descriptor.\n* `inputTensorDesc` [in]  -  Input tensor descriptor.\n* `nbDims` [in]  -           Number of dimensions.\n* `outputTensorDimA` [out]  - Array to receive output dimension sizes.\n@retval CUDNN_STATUS_SUCCESS     The dimensions were computed successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead."]
pub unsafe fn cudnnGetPoolingNdForwardOutputDim(poolingDesc: cudnnPoolingDescriptor_t, inputTensorDesc: cudnnTensorDescriptor_t, nbDims: ::core::ffi::c_int) -> Result<::core::ffi::c_int, crate::sys::cudnnStatus_t> {
    let mut out_3: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnGetPoolingNdForwardOutputDim(poolingDesc, inputTensorDesc, nbDims, out_3.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_3.assume_init() as ::core::ffi::c_int) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Computes the output dimensions of a 2D pooling operation.\n\n# Arguments\n\n* `poolingDesc` [in]  -     Pooling descriptor.\n* `inputTensorDesc` [in]  - Input tensor descriptor.\n* `n` [out]  -               Output batch size.\n* `c` [out]  -               Output number of channels.\n* `h` [out]  -               Output height.\n* `w` [out]  -               Output width.\n@retval CUDNN_STATUS_SUCCESS     The dimensions were computed successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead."]
pub unsafe fn cudnnGetPooling2dForwardOutputDim(poolingDesc: cudnnPoolingDescriptor_t, inputTensorDesc: cudnnTensorDescriptor_t) -> Result<(::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int), crate::sys::cudnnStatus_t> {
    let mut out_2: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let mut out_3: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let mut out_4: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let mut out_5: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnGetPooling2dForwardOutputDim(poolingDesc, inputTensorDesc, out_2.as_mut_ptr() as *mut _, out_3.as_mut_ptr() as *mut _, out_4.as_mut_ptr() as *mut _, out_5.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok((out_2.assume_init() as ::core::ffi::c_int, out_3.assume_init() as ::core::ffi::c_int, out_4.assume_init() as ::core::ffi::c_int, out_5.assume_init() as ::core::ffi::c_int)) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Destroys a pooling descriptor.\n\n# Arguments\n\n* `poolingDesc` [in]  -  Pooling descriptor to destroy.\n@retval CUDNN_STATUS_SUCCESS  The descriptor was destroyed successfully.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnCreatePoolingDescriptor`]"]
pub unsafe fn cudnnDestroyPoolingDescriptor(poolingDesc: cudnnPoolingDescriptor_t) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnDestroyPoolingDescriptor(poolingDesc) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Performs forward pooling.\nComputes y = alpha * pool(x) + beta * y.\n\n# Arguments\n\n* `handle` [in]  -      cuDNN library handle.\n* `poolingDesc` [in]  - Pooling descriptor.\n* `alpha` [in]  -       Scaling factor for the pooling result.\n* `xDesc` [in]  -       Input tensor descriptor.\n* `x` [in]  -           Pointer to input tensor data.\n* `beta` [in]  -        Scaling factor for the destination tensor.\n* `yDesc` [in]  -       Output tensor descriptor.\n* `y` [in,out]  -           Pointer to output tensor data.\n@retval CUDNN_STATUS_SUCCESS     The operation completed successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnPoolingBackward`]"]
pub unsafe fn cudnnPoolingForward<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(
    handle: cudnnHandle_t,
    poolingDesc: cudnnPoolingDescriptor_t,
    alpha: T0,
    xDesc: cudnnTensorDescriptor_t,
    x: T1,
    beta: T2,
    yDesc: cudnnTensorDescriptor_t,
    mut y: T3,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnPoolingForward(handle, poolingDesc, alpha.as_const_ptr() as *const _, xDesc, x.as_const_ptr() as *const _, beta.as_const_ptr() as *const _, yDesc, y.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Creates an activation descriptor.\n\n# Arguments\n\n* `activationDesc` [out]  -  Pointer to the newly created activation descriptor.\n@retval CUDNN_STATUS_SUCCESS       The descriptor was created successfully.\n@retval CUDNN_STATUS_ALLOC_FAILED  Memory allocation failed.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnDestroyActivationDescriptor`]"]
pub unsafe fn cudnnCreateActivationDescriptor() -> Result<cudnnActivationDescriptor_t, crate::sys::cudnnStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cudnnActivationDescriptor_t> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnCreateActivationDescriptor(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cudnnActivationDescriptor_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Configures an activation descriptor.\n\n# Arguments\n\n* `activationDesc` [in,out]  -  Activation descriptor to configure.\n* `mode` [in]  -            Activation function type.\n* `reluNanOpt` [in]  -      NaN propagation policy for ReLU.\n* `coef` [in]  -            Ceiling for clipped ReLU, or alpha for ELU.\n@retval CUDNN_STATUS_SUCCESS     The descriptor was set successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnGetActivationDescriptor`]"]
pub unsafe fn cudnnSetActivationDescriptor(activationDesc: cudnnActivationDescriptor_t, mode: cudnnActivationMode_t, reluNanOpt: cudnnNanPropagation_t, coef: f64) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnSetActivationDescriptor(activationDesc, mode, reluNanOpt, coef) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Retrieves the settings of an activation descriptor.\n\n# Arguments\n\n* `activationDesc` [in]  -  Activation descriptor to query.\n* `mode` [out]  -            Activation function type.\n* `reluNanOpt` [out]  -      NaN propagation policy.\n* `coef` [out]  -            Ceiling for clipped ReLU, or alpha for ELU.\n@retval CUDNN_STATUS_SUCCESS  The descriptor was queried successfully.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnSetActivationDescriptor`]"]
pub unsafe fn cudnnGetActivationDescriptor(activationDesc: cudnnActivationDescriptor_t) -> Result<(cudnnActivationMode_t, cudnnNanPropagation_t, f64), crate::sys::cudnnStatus_t> {
    let mut out_1: std::mem::MaybeUninit<cudnnActivationMode_t> = std::mem::MaybeUninit::zeroed();
    let mut out_2: std::mem::MaybeUninit<cudnnNanPropagation_t> = std::mem::MaybeUninit::zeroed();
    let mut out_3: std::mem::MaybeUninit<f64> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnGetActivationDescriptor(activationDesc, out_1.as_mut_ptr() as *mut _, out_2.as_mut_ptr() as *mut _, out_3.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok((out_1.assume_init() as cudnnActivationMode_t, out_2.assume_init() as cudnnNanPropagation_t, out_3.assume_init() as f64)) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Sets the beta parameter for Swish activation.\n\n# Arguments\n\n* `activationDesc` [in,out]  -  Activation descriptor to modify.\n* `swish_beta` [in]  -      Beta value for the Swish activation function.\n@retval CUDNN_STATUS_SUCCESS     The parameter was set successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnGetActivationDescriptorSwishBeta`]"]
pub unsafe fn cudnnSetActivationDescriptorSwishBeta(activationDesc: cudnnActivationDescriptor_t, swish_beta: f64) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnSetActivationDescriptorSwishBeta(activationDesc, swish_beta) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Retrieves the beta parameter for Swish activation.\n\n# Arguments\n\n* `activationDesc` [in]  -  Activation descriptor to query.\n* `swish_beta` [out]  -      Beta value for the Swish activation function.\n@retval CUDNN_STATUS_SUCCESS  The parameter was retrieved successfully.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnSetActivationDescriptorSwishBeta`]"]
pub unsafe fn cudnnGetActivationDescriptorSwishBeta(activationDesc: cudnnActivationDescriptor_t) -> Result<f64, crate::sys::cudnnStatus_t> {
    let mut out_1: std::mem::MaybeUninit<f64> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnGetActivationDescriptorSwishBeta(activationDesc, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as f64) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Destroys an activation descriptor.\n\n# Arguments\n\n* `activationDesc` [in]  -  Activation descriptor to destroy.\n@retval CUDNN_STATUS_SUCCESS  The descriptor was destroyed successfully.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnCreateActivationDescriptor`]"]
pub unsafe fn cudnnDestroyActivationDescriptor(activationDesc: cudnnActivationDescriptor_t) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnDestroyActivationDescriptor(activationDesc) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Performs forward activation.\nComputes y = alpha * activation(x) + beta * y.\n\n# Arguments\n\n* `handle` [in]  -          cuDNN library handle.\n* `activationDesc` [in]  -  Activation descriptor.\n* `alpha` [in]  -           Scaling factor for the activation result.\n* `xDesc` [in]  -           Input tensor descriptor.\n* `x` [in]  -               Pointer to input tensor data.\n* `beta` [in]  -            Scaling factor for the destination tensor.\n* `yDesc` [in]  -           Output tensor descriptor.\n* `y` [in,out]  -               Pointer to output tensor data.\n@retval CUDNN_STATUS_SUCCESS     The operation completed successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnActivationBackward`]"]
pub unsafe fn cudnnActivationForward<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(
    handle: cudnnHandle_t,
    activationDesc: cudnnActivationDescriptor_t,
    alpha: T0,
    xDesc: cudnnTensorDescriptor_t,
    x: T1,
    beta: T2,
    yDesc: cudnnTensorDescriptor_t,
    mut y: T3,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnActivationForward(handle, activationDesc, alpha.as_const_ptr() as *const _, xDesc, x.as_const_ptr() as *const _, beta.as_const_ptr() as *const _, yDesc, y.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Creates a Local Response Normalization (LRN) descriptor.\nUses lrnN=5, lrnAlpha=1e-4, lrnBeta=0.75, lrnK=2.0 as defaults from\nKrizhevsky'12 ImageNet paper.\n\n# Arguments\n\n* `normDesc` [out]  -  Pointer to the newly created LRN descriptor.\n@retval CUDNN_STATUS_SUCCESS       The descriptor was created successfully.\n@retval CUDNN_STATUS_ALLOC_FAILED  Memory allocation failed.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnDestroyLRNDescriptor,`] cudnnSetLRNDescriptor"]
pub unsafe fn cudnnCreateLRNDescriptor() -> Result<cudnnLRNDescriptor_t, crate::sys::cudnnStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cudnnLRNDescriptor_t> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnCreateLRNDescriptor(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cudnnLRNDescriptor_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Configures an LRN descriptor.\nUses a window [center-lookBehind, center+lookAhead], where\nlookBehind = floor((lrnN-1)/2), lookAhead = lrnN-lookBehind-1.\nValues of double parameters are cast to the tensor data type.\n\n# Arguments\n\n* `normDesc` [in,out]  -  LRN descriptor to configure.\n* `lrnN` [in]  -      Normalization window size (must be in [CUDNN_LRN_MIN_N, CUDNN_LRN_MAX_N]).\n* `lrnAlpha` [in]  -  Alpha parameter (must be >= CUDNN_LRN_MIN_K).\n* `lrnBeta` [in]  -   Beta parameter (must be >= CUDNN_LRN_MIN_BETA).\n* `lrnK` [in]  -      K parameter.\n@retval CUDNN_STATUS_SUCCESS     The descriptor was set successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnGetLRNDescriptor`]"]
pub unsafe fn cudnnSetLRNDescriptor(normDesc: cudnnLRNDescriptor_t, lrnN: ::core::ffi::c_uint, lrnAlpha: f64, lrnBeta: f64, lrnK: f64) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnSetLRNDescriptor(normDesc, lrnN, lrnAlpha, lrnBeta, lrnK) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Retrieves the settings of an LRN descriptor.\nAny of the output pointers can be NULL (the corresponding value will not be returned).\n\n# Arguments\n\n* `normDesc` [in]  -  LRN descriptor to query.\n* `lrnN` [out]  -      Normalization window size.\n* `lrnAlpha` [out]  -  Alpha parameter.\n* `lrnBeta` [out]  -   Beta parameter.\n* `lrnK` [out]  -      K parameter.\n@retval CUDNN_STATUS_SUCCESS  The descriptor was queried successfully.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnSetLRNDescriptor`]"]
pub unsafe fn cudnnGetLRNDescriptor(normDesc: cudnnLRNDescriptor_t) -> Result<(::core::ffi::c_uint, f64, f64, f64), crate::sys::cudnnStatus_t> {
    let mut out_1: std::mem::MaybeUninit<::core::ffi::c_uint> = std::mem::MaybeUninit::zeroed();
    let mut out_2: std::mem::MaybeUninit<f64> = std::mem::MaybeUninit::zeroed();
    let mut out_3: std::mem::MaybeUninit<f64> = std::mem::MaybeUninit::zeroed();
    let mut out_4: std::mem::MaybeUninit<f64> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnGetLRNDescriptor(normDesc, out_1.as_mut_ptr() as *mut _, out_2.as_mut_ptr() as *mut _, out_3.as_mut_ptr() as *mut _, out_4.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok((out_1.assume_init() as ::core::ffi::c_uint, out_2.assume_init() as f64, out_3.assume_init() as f64, out_4.assume_init() as f64)) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Destroys an LRN descriptor.\n\n# Arguments\n\n* `lrnDesc` [in]  -  LRN descriptor to destroy.\n@retval CUDNN_STATUS_SUCCESS  The descriptor was destroyed successfully.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnCreateLRNDescriptor`]"]
pub unsafe fn cudnnDestroyLRNDescriptor(lrnDesc: cudnnLRNDescriptor_t) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnDestroyLRNDescriptor(lrnDesc) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Performs forward LRN cross-channel normalization.\nComputes y = alpha * normalize(x) + beta * y. Double parameters are cast\nto the tensor data type.\n\n# Arguments\n\n* `handle` [in]  -    cuDNN library handle.\n* `normDesc` [in]  -  LRN descriptor.\n* `lrnMode` [in]  -   LRN mode.\n* `alpha` [in]  -     Scaling factor for the normalization result.\n* `xDesc` [in]  -     Input tensor descriptor.\n* `x` [in]  -         Pointer to input tensor data.\n* `beta` [in]  -      Scaling factor for the destination tensor.\n* `yDesc` [in]  -     Output tensor descriptor.\n* `y` [in,out]  -         Pointer to output tensor data.\n@retval CUDNN_STATUS_SUCCESS     The operation completed successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnLRNCrossChannelBackward`]"]
pub unsafe fn cudnnLRNCrossChannelForward<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(
    handle: cudnnHandle_t,
    normDesc: cudnnLRNDescriptor_t,
    lrnMode: cudnnLRNMode_t,
    alpha: T0,
    xDesc: cudnnTensorDescriptor_t,
    x: T1,
    beta: T2,
    yDesc: cudnnTensorDescriptor_t,
    mut y: T3,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnLRNCrossChannelForward(handle, normDesc, lrnMode, alpha.as_const_ptr() as *const _, xDesc, x.as_const_ptr() as *const _, beta.as_const_ptr() as *const _, yDesc, y.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Performs forward divisive normalization.\nComputes y = alpha * normalize(x) + beta * y. If means is NULL, means are\nassumed to be zero. The xDesc is used for means, temp, and temp2 as well.\n\n# Arguments\n\n* `handle` [in]  -    cuDNN library handle.\n* `normDesc` [in]  -  LRN descriptor (shared with LRN functions).\n* `mode` [in]  -      Divisive normalization mode.\n* `alpha` [in]  -     Scaling factor for the normalization result.\n* `xDesc` [in]  -     Input tensor descriptor (also used for means, temp, temp2).\n* `x` [in]  -         Pointer to input tensor data.\n* `means` [in]  -     Pointer to means tensor data (NULL for zero means).\n* `temp` [out]  -      Temporary workspace tensor.\n* `temp2` [out]  -     Temporary workspace tensor.\n* `beta` [in]  -      Scaling factor for the destination tensor.\n* `yDesc` [in]  -     Output tensor descriptor.\n* `y` [in,out]  -         Pointer to output tensor data.\n@retval CUDNN_STATUS_SUCCESS     The operation completed successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnDivisiveNormalizationBackward`]"]
pub unsafe fn cudnnDivisiveNormalizationForward<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsPtr, T6: types::CudaAsMutPtr>(
    handle: cudnnHandle_t,
    normDesc: cudnnLRNDescriptor_t,
    mode: cudnnDivNormMode_t,
    alpha: T0,
    xDesc: cudnnTensorDescriptor_t,
    x: T1,
    means: T2,
    mut temp: T3,
    mut temp2: T4,
    beta: T5,
    yDesc: cudnnTensorDescriptor_t,
    mut y: T6,
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
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Derives a tensor descriptor for batch normalization parameters.\nComputes the dimensions for bnScale, bnBias, mean, and variance tensors based\non the input tensor descriptor and batch normalization mode. Use this for\nbnScaleBiasMeanVarDesc and bnScaleBiasDiffDesc parameters.\n\n# Arguments\n\n* `derivedBnDesc` [in,out]  -  Tensor descriptor to be derived.\n* `xDesc` [in]  -          Input tensor descriptor.\n* `mode` [in]  -           Batch normalization mode.\n@retval CUDNN_STATUS_SUCCESS     The descriptor was derived successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnBatchNormalizationForwardTraining`]"]
pub unsafe fn cudnnDeriveBNTensorDescriptor(derivedBnDesc: cudnnTensorDescriptor_t, xDesc: cudnnTensorDescriptor_t, mode: cudnnBatchNormMode_t) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnDeriveBNTensorDescriptor(derivedBnDesc, xDesc, mode) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Performs batch normalization during inference.\nComputes y[i] = bnScale[k]*(x[i]-estimatedMean[k])/sqrt(epsilon+estimatedVariance[k]) + bnBias[k],\nwith tensors indexed according to spatial or per-activation mode.\n\n# Arguments\n\n* `handle` [in]  -                  cuDNN library handle.\n* `mode` [in]  -                    Batch normalization mode.\n* `alpha` [in]  -                   Result blend factor.\n* `beta` [in]  -                    Destination layer blend factor.\n* `xDesc` [in]  -                   Input tensor descriptor.\n* `x` [in]  -                       Pointer to input tensor data (NxCxHxW).\n* `yDesc` [in]  -                   Output tensor descriptor.\n* `y` [in,out]  -                       Pointer to output tensor data (NxCxHxW).\n* `bnScaleBiasMeanVarDesc` [in]  -  Descriptor for scale, bias, mean, variance tensors.\n* `bnScale` [in]  -                 Pointer to scale (gamma) tensor data.\n* `bnBias` [in]  -                  Pointer to bias (beta) tensor data.\n* `estimatedMean` [in]  -           Pointer to running mean tensor data.\n* `estimatedVariance` [in]  -       Pointer to running variance tensor data.\n* `epsilon` [in]  -                 Epsilon value (must be >= CUDNN_BN_MIN_EPSILON).\n@retval CUDNN_STATUS_SUCCESS     The operation completed successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnBatchNormalizationForwardTraining,`] cudnnDeriveBNTensorDescriptor"]
pub unsafe fn cudnnBatchNormalizationForwardInference<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsPtr, T5: types::CudaAsPtr, T6: types::CudaAsPtr, T7: types::CudaAsPtr>(
    handle: cudnnHandle_t,
    mode: cudnnBatchNormMode_t,
    alpha: T0,
    beta: T1,
    xDesc: cudnnTensorDescriptor_t,
    x: T2,
    yDesc: cudnnTensorDescriptor_t,
    mut y: T3,
    bnScaleBiasMeanVarDesc: cudnnTensorDescriptor_t,
    bnScale: T4,
    bnBias: T5,
    estimatedMean: T6,
    estimatedVariance: T7,
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
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Derives tensor descriptors for normalization parameters.\nComputes the dimensions for normScale, normBias, mean, and variance tensors based\non the input tensor descriptor and normalization mode.\n\n# Arguments\n\n* `derivedNormScaleBiasDesc` [in,out]  -  Descriptor to be derived for scale/bias tensors.\n* `derivedNormMeanVarDesc` [in,out]  -    Descriptor to be derived for mean/variance tensors.\n* `xDesc` [in]  -                     Input tensor descriptor.\n* `mode` [in]  -                      Normalization mode.\n* `groupCnt` [in]  -                  Group count (reserved, should be set to 1).\n@retval CUDNN_STATUS_SUCCESS     The descriptors were derived successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnNormalizationForwardTraining`]"]
pub unsafe fn cudnnDeriveNormTensorDescriptor(derivedNormScaleBiasDesc: cudnnTensorDescriptor_t, derivedNormMeanVarDesc: cudnnTensorDescriptor_t, xDesc: cudnnTensorDescriptor_t, mode: cudnnNormMode_t, groupCnt: ::core::ffi::c_int) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnDeriveNormTensorDescriptor(derivedNormScaleBiasDesc, derivedNormMeanVarDesc, xDesc, mode, groupCnt) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Performs normalization during inference.\nComputes y[i] = normScale[k]*(x[i]-estimatedMean[k])/sqrt(epsilon+estimatedVariance[k]) + normBias[k],\nwith tensors indexed according to per-channel or per-activation mode.\n\n# Arguments\n\n* `handle` [in]  -             cuDNN library handle.\n* `mode` [in]  -               Normalization mode.\n* `normOps` [in]  -            Extended normalization operation mode.\n* `algo` [in]  -               Normalization algorithm.\n* `alpha` [in]  -              Result blend factor.\n* `beta` [in]  -               Destination layer blend factor.\n* `xDesc` [in]  -              Input tensor descriptor.\n* `x` [in]  -                  Pointer to input tensor data (NxCxHxW).\n* `normScaleBiasDesc` [in]  -  Descriptor for normalization scale/bias tensors.\n* `normScale` [in]  -          Pointer to normalization scale tensor data.\n* `normBias` [in]  -           Pointer to normalization bias tensor data.\n* `normMeanVarDesc` [in]  -    Descriptor for mean/variance tensors.\n* `estimatedMean` [in]  -      Pointer to running mean tensor data.\n* `estimatedVariance` [in]  -  Pointer to running variance tensor data.\n* `zDesc` [in]  -              Descriptor for z tensor (used with add operations).\n* `z` [in]  -                  Pointer to z tensor data.\n* `activationDesc` [in]  -     Activation descriptor (used with activation operations).\n* `yDesc` [in]  -              Output tensor descriptor.\n* `y` [in,out]  -                  Pointer to output tensor data (NxCxHxW).\n* `epsilon` [in]  -            Epsilon value (must be >= 0).\n* `groupCnt` [in]  -           Group count (reserved, should be set to 1).\n@retval CUDNN_STATUS_SUCCESS     The operation completed successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnNormalizationForwardTraining,`] cudnnDeriveNormTensorDescriptor"]
pub unsafe fn cudnnNormalizationForwardInference<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsPtr, T5: types::CudaAsPtr, T6: types::CudaAsPtr, T7: types::CudaAsPtr, T8: types::CudaAsMutPtr>(
    handle: cudnnHandle_t,
    mode: cudnnNormMode_t,
    normOps: cudnnNormOps_t,
    algo: cudnnNormAlgo_t,
    alpha: T0,
    beta: T1,
    xDesc: cudnnTensorDescriptor_t,
    x: T2,
    normScaleBiasDesc: cudnnTensorDescriptor_t,
    normScale: T3,
    normBias: T4,
    normMeanVarDesc: cudnnTensorDescriptor_t,
    estimatedMean: T5,
    estimatedVariance: T6,
    zDesc: cudnnTensorDescriptor_t,
    z: T7,
    activationDesc: cudnnActivationDescriptor_t,
    yDesc: cudnnTensorDescriptor_t,
    mut y: T8,
    epsilon: f64,
    groupCnt: ::core::ffi::c_int,
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
            groupCnt,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Creates a spatial transformer descriptor.\n\n# Arguments\n\n* `stDesc` [out]  -  Pointer to the newly created spatial transformer descriptor.\n@retval CUDNN_STATUS_SUCCESS       The descriptor was created successfully.\n@retval CUDNN_STATUS_ALLOC_FAILED  Memory allocation failed.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnDestroySpatialTransformerDescriptor`]"]
pub unsafe fn cudnnCreateSpatialTransformerDescriptor() -> Result<cudnnSpatialTransformerDescriptor_t, crate::sys::cudnnStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cudnnSpatialTransformerDescriptor_t> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnCreateSpatialTransformerDescriptor(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cudnnSpatialTransformerDescriptor_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Configures an N-dimensional spatial transformer descriptor.\n\n# Arguments\n\n* `stDesc` [in,out]  -      Spatial transformer descriptor to configure.\n* `samplerType` [in]  - Type of sampler to use.\n* `dataType` [in]  -    Data type of the tensors.\n* `nbDims` [in]  -      Number of dimensions.\n* `dimA` [in]  -        Array of dimension sizes (length nbDims).\n@retval CUDNN_STATUS_SUCCESS     The descriptor was set successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnSpatialTfGridGeneratorForward,`] cudnnSpatialTfSamplerForward"]
pub unsafe fn cudnnSetSpatialTransformerNdDescriptor<T0: types::CudaAsPtr>(stDesc: cudnnSpatialTransformerDescriptor_t, samplerType: cudnnSamplerType_t, dataType: cudnnDataType_t, nbDims: ::core::ffi::c_int, dimA: T0) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnSetSpatialTransformerNdDescriptor(stDesc, samplerType, dataType, nbDims, dimA.as_const_ptr() as *const _) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Destroys a spatial transformer descriptor.\n\n# Arguments\n\n* `stDesc` [in]  -  Spatial transformer descriptor to destroy.\n@retval CUDNN_STATUS_SUCCESS  The descriptor was destroyed successfully.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnCreateSpatialTransformerDescriptor`]"]
pub unsafe fn cudnnDestroySpatialTransformerDescriptor(stDesc: cudnnSpatialTransformerDescriptor_t) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnDestroySpatialTransformerDescriptor(stDesc) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Generates a sampling grid for a spatial transformer (forward).\nGenerates a grid of sampling coordinates from the affine transformation matrix theta.\n\n# Arguments\n\n* `handle` [in]  -  cuDNN library handle.\n* `stDesc` [in]  -  Spatial transformer descriptor.\n* `theta` [in]  -   Pointer to affine transformation matrices.\n* `grid` [out]  -    Pointer to output sampling grid data.\n@retval CUDNN_STATUS_SUCCESS     The operation completed successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnSpatialTfGridGeneratorBackward`]"]
pub unsafe fn cudnnSpatialTfGridGeneratorForward<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cudnnHandle_t, stDesc: cudnnSpatialTransformerDescriptor_t, theta: T0, mut grid: T1) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnSpatialTfGridGeneratorForward(handle, stDesc, theta.as_const_ptr() as *const _, grid.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Performs spatial transformer sampling (forward).\nSamples the input tensor at the grid coordinates to produce the output tensor.\n\n# Arguments\n\n* `handle` [in]  -  cuDNN library handle.\n* `stDesc` [in]  -  Spatial transformer descriptor.\n* `alpha` [in]  -   Scaling factor for the sampled result.\n* `xDesc` [in]  -   Input tensor descriptor.\n* `x` [in]  -       Pointer to input tensor data.\n* `grid` [in]  -    Pointer to sampling grid data.\n* `beta` [in]  -    Scaling factor for the destination tensor.\n* `yDesc` [in]  -   Output tensor descriptor.\n* `y` [in,out]  -       Pointer to output tensor data.\n@retval CUDNN_STATUS_SUCCESS     The operation completed successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnSpatialTfSamplerBackward`]"]
pub unsafe fn cudnnSpatialTfSamplerForward<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cudnnHandle_t,
    stDesc: cudnnSpatialTransformerDescriptor_t,
    alpha: T0,
    xDesc: cudnnTensorDescriptor_t,
    x: T1,
    grid: T2,
    beta: T3,
    yDesc: cudnnTensorDescriptor_t,
    mut y: T4,
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
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Creates a dropout descriptor.\n\n# Arguments\n\n* `dropoutDesc` [out]  -  Pointer to the newly created dropout descriptor.\n@retval CUDNN_STATUS_SUCCESS       The descriptor was created successfully.\n@retval CUDNN_STATUS_ALLOC_FAILED  Memory allocation failed.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnDestroyDropoutDescriptor`]"]
pub unsafe fn cudnnCreateDropoutDescriptor() -> Result<cudnnDropoutDescriptor_t, crate::sys::cudnnStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cudnnDropoutDescriptor_t> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnCreateDropoutDescriptor(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cudnnDropoutDescriptor_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Destroys a dropout descriptor.\n\n# Arguments\n\n* `dropoutDesc` [in]  -  Dropout descriptor to destroy.\n@retval CUDNN_STATUS_SUCCESS  The descriptor was destroyed successfully.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnCreateDropoutDescriptor`]"]
pub unsafe fn cudnnDestroyDropoutDescriptor(dropoutDesc: cudnnDropoutDescriptor_t) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnDestroyDropoutDescriptor(dropoutDesc) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Returns the size of the states buffer required for dropout.\n\n# Arguments\n\n* `handle` [in]  -      cuDNN library handle.\n* `sizeInBytes` [out]  - Size of the required states buffer in bytes.\n@retval CUDNN_STATUS_SUCCESS  The size was returned successfully.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnSetDropoutDescriptor`]"]
pub unsafe fn cudnnDropoutGetStatesSize(handle: cudnnHandle_t) -> Result<usize, crate::sys::cudnnStatus_t> {
    let mut out_1: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnDropoutGetStatesSize(handle, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as usize) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Returns the size of the reserve space required for dropout forward/backward.\n\n# Arguments\n\n* `xdesc` [in]  -       Input tensor descriptor.\n* `sizeInBytes` [out]  - Size of the required reserve space in bytes.\n@retval CUDNN_STATUS_SUCCESS  The size was returned successfully.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnDropoutForward,`] cudnnDropoutBackward"]
pub unsafe fn cudnnDropoutGetReserveSpaceSize(xdesc: cudnnTensorDescriptor_t) -> Result<usize, crate::sys::cudnnStatus_t> {
    let mut out_1: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnDropoutGetReserveSpaceSize(xdesc, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as usize) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Configures a dropout descriptor and initializes random state.\n\n# Arguments\n\n* `dropoutDesc` [in,out]  -      Dropout descriptor to configure.\n* `handle` [in]  -           cuDNN library handle.\n* `dropout` [in]  -          Probability of dropping (0 = no dropout, 1 = all dropped).\n* `states` [in,out]  -           Pointer to device memory for RNG state storage.\n* `stateSizeInBytes` [in]  - Size of the states buffer in bytes.\n* `seed` [in]  -             Seed for the random number generator.\n@retval CUDNN_STATUS_SUCCESS     The descriptor was configured successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnGetDropoutDescriptor,`] cudnnRestoreDropoutDescriptor"]
pub unsafe fn cudnnSetDropoutDescriptor<T0: types::CudaAsMutPtr>(dropoutDesc: cudnnDropoutDescriptor_t, handle: cudnnHandle_t, dropout: f32, mut states: T0, stateSizeInBytes: usize, seed: ::core::ffi::c_ulonglong) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnSetDropoutDescriptor(dropoutDesc, handle, dropout, states.as_mut_ptr() as *mut _, stateSizeInBytes, seed) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Restores a dropout descriptor to a previously saved state.\n\n# Arguments\n\n* `dropoutDesc` [in,out]  -      Dropout descriptor to restore.\n* `handle` [in]  -           cuDNN library handle.\n* `dropout` [in]  -          Dropout probability.\n* `states` [in]  -           Pointer to previously saved RNG state.\n* `stateSizeInBytes` [in]  - Size of the states buffer in bytes.\n* `seed` [in]  -             Seed used to initialize the original state.\n@retval CUDNN_STATUS_SUCCESS     The descriptor was restored successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnSetDropoutDescriptor`]"]
pub unsafe fn cudnnRestoreDropoutDescriptor<T0: types::CudaAsMutPtr>(dropoutDesc: cudnnDropoutDescriptor_t, handle: cudnnHandle_t, dropout: f32, mut states: T0, stateSizeInBytes: usize, seed: ::core::ffi::c_ulonglong) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnRestoreDropoutDescriptor(dropoutDesc, handle, dropout, states.as_mut_ptr() as *mut _, stateSizeInBytes, seed) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Retrieves the settings of a dropout descriptor.\n\n# Arguments\n\n* `dropoutDesc` [in]  -  Dropout descriptor to query.\n* `handle` [in]  -       cuDNN library handle.\n* `dropout` [out]  -      Dropout probability.\n* `states` [out]  -       Pointer to RNG state memory.\n* `seed` [out]  -         Seed used for the RNG.\n@retval CUDNN_STATUS_SUCCESS  The descriptor was queried successfully.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnSetDropoutDescriptor`]"]
pub unsafe fn cudnnGetDropoutDescriptor(dropoutDesc: cudnnDropoutDescriptor_t, handle: cudnnHandle_t, states: *mut *mut ::core::ffi::c_void) -> Result<(f32, ::core::ffi::c_ulonglong), crate::sys::cudnnStatus_t> {
    let mut out_2: std::mem::MaybeUninit<f32> = std::mem::MaybeUninit::zeroed();
    let mut out_4: std::mem::MaybeUninit<::core::ffi::c_ulonglong> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnGetDropoutDescriptor(dropoutDesc, handle, out_2.as_mut_ptr() as *mut _, states, out_4.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok((out_2.assume_init() as f32, out_4.assume_init() as ::core::ffi::c_ulonglong)) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Performs forward dropout.\nRandomly sets elements to zero based on the dropout probability. The reserve\nspace stores the mask for use in the backward pass.\n\n# Arguments\n\n* `handle` [in]  -                  cuDNN library handle.\n* `dropoutDesc` [in]  -             Dropout descriptor.\n* `xdesc` [in]  -                   Input tensor descriptor.\n* `x` [in]  -                       Pointer to input tensor data.\n* `ydesc` [in]  -                   Output tensor descriptor.\n* `y` [out]  -                       Pointer to output tensor data.\n* `reserveSpace` [out]  -            Pointer to reserve space for the dropout mask.\n* `reserveSpaceSizeInBytes` [in]  - Size of reserve space in bytes.\n@retval CUDNN_STATUS_SUCCESS     The operation completed successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnDropoutBackward,`] cudnnDropoutGetReserveSpaceSize"]
pub unsafe fn cudnnDropoutForward<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr>(
    handle: cudnnHandle_t,
    dropoutDesc: cudnnDropoutDescriptor_t,
    xdesc: cudnnTensorDescriptor_t,
    x: T0,
    ydesc: cudnnTensorDescriptor_t,
    mut y: T1,
    mut reserveSpace: T2,
    reserveSpaceSizeInBytes: usize,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnDropoutForward(handle, dropoutDesc, xdesc, x.as_const_ptr() as *const _, ydesc, y.as_mut_ptr() as *mut _, reserveSpace.as_mut_ptr() as *mut _, reserveSpaceSizeInBytes) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Cross-library version checker for the ops sub-library.\nThis function is implemented differently in each sub-library. Each sub-library\nchecks whether its own version matches that of its dependencies.\n@retval CUDNN_STATUS_SUCCESS                       The version check passed.\n@retval CUDNN_STATUS_SUBLIBRARY_VERSION_MISMATCH   The versions are inconsistent.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnOpsVersionCheck() -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnOpsVersionCheck() };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Performs backward softmax computation.\nComputes the gradient of the softmax function.\n\n# Arguments\n\n* `handle` [in]  -  cuDNN library handle.\n* `algo` [in]  -    Softmax algorithm used in the forward pass.\n* `mode` [in]  -    Softmax computation scope.\n* `alpha` [in]  -   Scaling factor for the result.\n* `yDesc` [in]  -   Output tensor descriptor (from forward pass).\n* `y` [in]  -       Pointer to output tensor data (from forward pass).\n* `dyDesc` [in]  -  Output gradient tensor descriptor.\n* `dy` [in]  -      Pointer to output gradient tensor data.\n* `beta` [in]  -    Scaling factor for the destination tensor.\n* `dxDesc` [in]  -  Input gradient tensor descriptor.\n* `dx` [in,out]  -      Pointer to input gradient tensor data.\n@retval CUDNN_STATUS_SUCCESS     The operation completed successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnSoftmaxForward`]"]
pub unsafe fn cudnnSoftmaxBackward<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr>(
    handle: cudnnHandle_t,
    algo: cudnnSoftmaxAlgorithm_t,
    mode: cudnnSoftmaxMode_t,
    alpha: T0,
    yDesc: cudnnTensorDescriptor_t,
    y: T1,
    dyDesc: cudnnTensorDescriptor_t,
    dy: T2,
    beta: T3,
    dxDesc: cudnnTensorDescriptor_t,
    mut dx: T4,
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
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Performs backward pooling.\nComputes the gradient of the pooling operation.\n\n# Arguments\n\n* `handle` [in]  -      cuDNN library handle.\n* `poolingDesc` [in]  - Pooling descriptor.\n* `alpha` [in]  -       Scaling factor for the result.\n* `yDesc` [in]  -       Output tensor descriptor (from forward pass).\n* `y` [in]  -           Pointer to output tensor data (from forward pass).\n* `dyDesc` [in]  -      Output gradient tensor descriptor.\n* `dy` [in]  -          Pointer to output gradient tensor data.\n* `xDesc` [in]  -       Input tensor descriptor (from forward pass).\n* `x` [in]  -           Pointer to input tensor data (from forward pass).\n* `beta` [in]  -        Scaling factor for the destination tensor.\n* `dxDesc` [in]  -      Input gradient tensor descriptor.\n* `dx` [in,out]  -          Pointer to input gradient tensor data.\n@retval CUDNN_STATUS_SUCCESS     The operation completed successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnPoolingForward`]"]
pub unsafe fn cudnnPoolingBackward<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsPtr, T5: types::CudaAsMutPtr>(
    handle: cudnnHandle_t,
    poolingDesc: cudnnPoolingDescriptor_t,
    alpha: T0,
    yDesc: cudnnTensorDescriptor_t,
    y: T1,
    dyDesc: cudnnTensorDescriptor_t,
    dy: T2,
    xDesc: cudnnTensorDescriptor_t,
    x: T3,
    beta: T4,
    dxDesc: cudnnTensorDescriptor_t,
    mut dx: T5,
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
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Performs backward activation.\nComputes the gradient of the activation function.\n\n# Arguments\n\n* `handle` [in]  -          cuDNN library handle.\n* `activationDesc` [in]  -  Activation descriptor.\n* `alpha` [in]  -           Scaling factor for the result.\n* `yDesc` [in]  -           Output tensor descriptor (from forward pass).\n* `y` [in]  -               Pointer to output tensor data (from forward pass).\n* `dyDesc` [in]  -          Output gradient tensor descriptor.\n* `dy` [in]  -              Pointer to output gradient tensor data.\n* `xDesc` [in]  -           Input tensor descriptor (from forward pass).\n* `x` [in]  -               Pointer to input tensor data (from forward pass).\n* `beta` [in]  -            Scaling factor for the destination tensor.\n* `dxDesc` [in]  -          Input gradient tensor descriptor.\n* `dx` [in,out]  -              Pointer to input gradient tensor data.\n@retval CUDNN_STATUS_SUCCESS     The operation completed successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnActivationForward`]"]
pub unsafe fn cudnnActivationBackward<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsPtr, T5: types::CudaAsMutPtr>(
    handle: cudnnHandle_t,
    activationDesc: cudnnActivationDescriptor_t,
    alpha: T0,
    yDesc: cudnnTensorDescriptor_t,
    y: T1,
    dyDesc: cudnnTensorDescriptor_t,
    dy: T2,
    xDesc: cudnnTensorDescriptor_t,
    x: T3,
    beta: T4,
    dxDesc: cudnnTensorDescriptor_t,
    mut dx: T5,
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
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Performs backward LRN cross-channel normalization.\nComputes the gradient of the LRN cross-channel normalization. Double\nparameters are cast to the tensor data type.\n\n# Arguments\n\n* `handle` [in]  -    cuDNN library handle.\n* `normDesc` [in]  -  LRN descriptor.\n* `lrnMode` [in]  -   LRN mode.\n* `alpha` [in]  -     Scaling factor for the result.\n* `yDesc` [in]  -     Output tensor descriptor (from forward pass).\n* `y` [in]  -         Pointer to output tensor data (from forward pass).\n* `dyDesc` [in]  -    Output gradient tensor descriptor.\n* `dy` [in]  -        Pointer to output gradient tensor data.\n* `xDesc` [in]  -     Input tensor descriptor (from forward pass).\n* `x` [in]  -         Pointer to input tensor data (from forward pass).\n* `beta` [in]  -      Scaling factor for the destination tensor.\n* `dxDesc` [in]  -    Input gradient tensor descriptor.\n* `dx` [in,out]  -        Pointer to input gradient tensor data.\n@retval CUDNN_STATUS_SUCCESS     The operation completed successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnLRNCrossChannelForward`]"]
pub unsafe fn cudnnLRNCrossChannelBackward<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsPtr, T5: types::CudaAsMutPtr>(
    handle: cudnnHandle_t,
    normDesc: cudnnLRNDescriptor_t,
    lrnMode: cudnnLRNMode_t,
    alpha: T0,
    yDesc: cudnnTensorDescriptor_t,
    y: T1,
    dyDesc: cudnnTensorDescriptor_t,
    dy: T2,
    xDesc: cudnnTensorDescriptor_t,
    x: T3,
    beta: T4,
    dxDesc: cudnnTensorDescriptor_t,
    mut dx: T5,
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
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Performs backward divisive normalization.\nComputes the gradients of the divisive normalization operation. If means is NULL,\nmeans are assumed to be zero.\n\n# Arguments\n\n* `handle` [in]  -       cuDNN library handle.\n* `normDesc` [in]  -     LRN descriptor (shared with LRN functions).\n* `mode` [in]  -         Divisive normalization mode.\n* `alpha` [in]  -        Scaling factor for the result.\n* `xDesc` [in]  -        Input tensor descriptor (also used for means, dy, temp, temp2).\n* `x` [in]  -            Pointer to input tensor data.\n* `means` [in]  -        Pointer to means tensor data (NULL for zero means).\n* `dy` [in]  -           Pointer to output gradient tensor data.\n* `temp` [out]  -         Temporary workspace tensor.\n* `temp2` [out]  -        Temporary workspace tensor.\n* `beta` [in]  -         Scaling factor for the destination tensors.\n* `dXdMeansDesc` [in]  - Descriptor for dx and dMeans tensors.\n* `dx` [in,out]  -           Pointer to input gradient tensor data.\n* `dMeans` [in,out]  -       Pointer to means gradient tensor data (can be NULL).\n@retval CUDNN_STATUS_SUCCESS     The operation completed successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnDivisiveNormalizationForward`]"]
pub unsafe fn cudnnDivisiveNormalizationBackward<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr, T6: types::CudaAsPtr, T7: types::CudaAsMutPtr, T8: types::CudaAsMutPtr>(
    handle: cudnnHandle_t,
    normDesc: cudnnLRNDescriptor_t,
    mode: cudnnDivNormMode_t,
    alpha: T0,
    xDesc: cudnnTensorDescriptor_t,
    x: T1,
    means: T2,
    dy: T3,
    mut temp: T4,
    mut temp2: T5,
    beta: T6,
    dXdMeansDesc: cudnnTensorDescriptor_t,
    mut dx: T7,
    mut dMeans: T8,
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
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Returns the workspace size for extended batch normalization forward training.\n\n# Arguments\n\n* `handle` [in]  -                  cuDNN library handle.\n* `mode` [in]  -                    Batch normalization mode.\n* `bnOps` [in]  -                   Extended batch normalization operation.\n* `xDesc` [in]  -                   Input tensor descriptor.\n* `zDesc` [in]  -                   Z tensor descriptor (for add operations).\n* `yDesc` [in]  -                   Output tensor descriptor.\n* `bnScaleBiasMeanVarDesc` [in]  -  Descriptor for BN parameter tensors.\n* `activationDesc` [in]  -          Activation descriptor.\n* `sizeInBytes` [out]  -             Required workspace size in bytes.\n@retval CUDNN_STATUS_SUCCESS  The size was returned successfully.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnBatchNormalizationForwardTrainingEx`]"]
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
    let mut out_8: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnGetBatchNormalizationForwardTrainingExWorkspaceSize(handle, mode, bnOps, xDesc, zDesc, yDesc, bnScaleBiasMeanVarDesc, activationDesc, out_8.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_8.assume_init() as usize) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Returns the workspace size for extended batch normalization backward.\n\n# Arguments\n\n* `handle` [in]  -            cuDNN library handle.\n* `mode` [in]  -              Batch normalization mode.\n* `bnOps` [in]  -             Extended batch normalization operation.\n* `xDesc` [in]  -             Input tensor descriptor.\n* `yDesc` [in]  -             Output tensor descriptor.\n* `dyDesc` [in]  -            Output gradient tensor descriptor.\n* `dzDesc` [in]  -            Z gradient tensor descriptor.\n* `dxDesc` [in]  -            Input gradient tensor descriptor.\n* `dBnScaleBiasDesc` [in]  -  Descriptor for BN parameter gradient tensors.\n* `activationDesc` [in]  -    Activation descriptor.\n* `sizeInBytes` [out]  -       Required workspace size in bytes.\n@retval CUDNN_STATUS_SUCCESS  The size was returned successfully.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnBatchNormalizationBackwardEx`]"]
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
    let mut out_10: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnGetBatchNormalizationBackwardExWorkspaceSize(handle, mode, bnOps, xDesc, yDesc, dyDesc, dzDesc, dxDesc, dBnScaleBiasDesc, activationDesc, out_10.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_10.assume_init() as usize) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Returns the reserve space size for extended batch normalization training.\n\n# Arguments\n\n* `handle` [in]  -          cuDNN library handle.\n* `mode` [in]  -            Batch normalization mode.\n* `bnOps` [in]  -           Extended batch normalization operation.\n* `activationDesc` [in]  -  Activation descriptor.\n* `xDesc` [in]  -           Input tensor descriptor.\n* `sizeInBytes` [out]  -     Required reserve space size in bytes.\n@retval CUDNN_STATUS_SUCCESS  The size was returned successfully.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnBatchNormalizationForwardTrainingEx`]"]
pub unsafe fn cudnnGetBatchNormalizationTrainingExReserveSpaceSize(handle: cudnnHandle_t, mode: cudnnBatchNormMode_t, bnOps: cudnnBatchNormOps_t, activationDesc: cudnnActivationDescriptor_t, xDesc: cudnnTensorDescriptor_t) -> Result<usize, crate::sys::cudnnStatus_t> {
    let mut out_5: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnGetBatchNormalizationTrainingExReserveSpaceSize(handle, mode, bnOps, activationDesc, xDesc, out_5.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_5.assume_init() as usize) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Performs batch normalization forward training.\nComputes y = BN(x). Also accumulates moving averages of mean and inverse variances.\n\n# Arguments\n\n* `handle` [in]  -                     cuDNN library handle.\n* `mode` [in]  -                       Batch normalization mode.\n* `alpha` [in]  -                      Result blend factor.\n* `beta` [in]  -                       Destination layer blend factor.\n* `xDesc` [in]  -                      Input tensor descriptor.\n* `x` [in]  -                          Pointer to input tensor data (NxCxHxW).\n* `yDesc` [in]  -                      Output tensor descriptor.\n* `y` [out]  -                          Pointer to output tensor data (NxCxHxW).\n* `bnScaleBiasMeanVarDesc` [in]  -     Descriptor for BN parameter tensors.\n* `bnScale` [in]  -                    Pointer to scale (gamma) tensor data.\n* `bnBias` [in]  -                     Pointer to bias (beta) tensor data.\n* `exponentialAverageFactor` [in]  -   Factor for computing running averages.\n* `resultRunningMean` [in,out]  -          Running mean (updated with exponential average).\n* `resultRunningVariance` [in,out]  -      Running variance (updated with exponential average).\n* `epsilon` [in]  -                    Epsilon value (must be >= CUDNN_BN_MIN_EPSILON).\n* `resultSaveMean` [out]  -             Optionally cached mean for backward pass (NULL if unused).\n* `resultSaveInvVariance` [out]  -      Optionally cached inverse variance for backward pass (NULL if unused).\n@retval CUDNN_STATUS_SUCCESS     The operation completed successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnBatchNormalizationBackward,`] cudnnDeriveBNTensorDescriptor"]
pub unsafe fn cudnnBatchNormalizationForwardTraining<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsPtr, T5: types::CudaAsPtr, T6: types::CudaAsMutPtr, T7: types::CudaAsMutPtr, T8: types::CudaAsMutPtr, T9: types::CudaAsMutPtr>(
    handle: cudnnHandle_t,
    mode: cudnnBatchNormMode_t,
    alpha: T0,
    beta: T1,
    xDesc: cudnnTensorDescriptor_t,
    x: T2,
    yDesc: cudnnTensorDescriptor_t,
    mut y: T3,
    bnScaleBiasMeanVarDesc: cudnnTensorDescriptor_t,
    bnScale: T4,
    bnBias: T5,
    exponentialAverageFactor: f64,
    mut resultRunningMean: T6,
    mut resultRunningVariance: T7,
    epsilon: f64,
    mut resultSaveMean: T8,
    mut resultSaveInvVariance: T9,
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
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Performs extended batch normalization forward training with optional activation.\nComputes y = relu(BN(x) + z). Also accumulates moving averages of mean and inverse variances.\nSupports fused batch normalization + activation and batch normalization + add + activation.\n\n# Arguments\n\n* `handle` [in]  -                     cuDNN library handle.\n* `mode` [in]  -                       Batch normalization mode.\n* `bnOps` [in]  -                      Extended batch normalization operation.\n* `alpha` [in]  -                      Result blend factor.\n* `beta` [in]  -                       Destination layer blend factor.\n* `xDesc` [in]  -                      Input tensor descriptor.\n* `xData` [in]  -                      Pointer to input tensor data.\n* `zDesc` [in]  -                      Z tensor descriptor (for add operations).\n* `zData` [in]  -                      Pointer to z tensor data.\n* `yDesc` [in]  -                      Output tensor descriptor.\n* `yData` [out]  -                      Pointer to output tensor data.\n* `bnScaleBiasMeanVarDesc` [in]  -     Descriptor for BN parameter tensors.\n* `bnScale` [in]  -                    Pointer to scale tensor data.\n* `bnBias` [in]  -                     Pointer to bias tensor data.\n* `exponentialAverageFactor` [in]  -   Factor for computing running averages.\n* `resultRunningMean` [in,out]  -          Running mean.\n* `resultRunningVariance` [in,out]  -      Running variance.\n* `epsilon` [in]  -                    Epsilon value (must be >= CUDNN_BN_MIN_EPSILON).\n* `resultSaveMean` [out]  -             Cached mean for backward pass (NULL if unused).\n* `resultSaveInvVariance` [out]  -      Cached inverse variance for backward pass (NULL if unused).\n* `activationDesc` [in]  -             Activation descriptor.\n* `workspace` [in,out]  -                  Pointer to workspace memory.\n* `workSpaceSizeInBytes` [in]  -       Size of workspace in bytes.\n* `reserveSpace` [in,out]  -               Pointer to reserve space memory.\n* `reserveSpaceSizeInBytes` [in]  -    Size of reserve space in bytes.\n@retval CUDNN_STATUS_SUCCESS     The operation completed successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnBatchNormalizationBackwardEx,`] cudnnGetBatchNormalizationForwardTrainingExWorkspaceSize"]
pub unsafe fn cudnnBatchNormalizationForwardTrainingEx<
    T0: types::CudaAsPtr,
    T1: types::CudaAsPtr,
    T2: types::CudaAsPtr,
    T3: types::CudaAsPtr,
    T4: types::CudaAsMutPtr,
    T5: types::CudaAsPtr,
    T6: types::CudaAsPtr,
    T7: types::CudaAsMutPtr,
    T8: types::CudaAsMutPtr,
    T9: types::CudaAsMutPtr,
    T10: types::CudaAsMutPtr,
    T11: types::CudaAsMutPtr,
    T12: types::CudaAsMutPtr,
>(
    handle: cudnnHandle_t,
    mode: cudnnBatchNormMode_t,
    bnOps: cudnnBatchNormOps_t,
    alpha: T0,
    beta: T1,
    xDesc: cudnnTensorDescriptor_t,
    xData: T2,
    zDesc: cudnnTensorDescriptor_t,
    zData: T3,
    yDesc: cudnnTensorDescriptor_t,
    mut yData: T4,
    bnScaleBiasMeanVarDesc: cudnnTensorDescriptor_t,
    bnScale: T5,
    bnBias: T6,
    exponentialAverageFactor: f64,
    mut resultRunningMean: T7,
    mut resultRunningVariance: T8,
    epsilon: f64,
    mut resultSaveMean: T9,
    mut resultSaveInvVariance: T10,
    activationDesc: cudnnActivationDescriptor_t,
    mut workspace: T11,
    workSpaceSizeInBytes: usize,
    mut reserveSpace: T12,
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
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Performs backward batch normalization.\nComputes gradients for x, bnScale, and bnBias.\n\n# Arguments\n\n* `handle` [in]  -              cuDNN library handle.\n* `mode` [in]  -                Batch normalization mode.\n* `alphaDataDiff` [in]  -       Scaling factor for dx result.\n* `betaDataDiff` [in]  -        Scaling factor for dx destination.\n* `alphaParamDiff` [in]  -      Scaling factor for parameter gradient results.\n* `betaParamDiff` [in]  -       Scaling factor for parameter gradient destinations.\n* `xDesc` [in]  -               Input tensor descriptor (same for x, dx, dy).\n* `x` [in]  -                   Pointer to input tensor data.\n* `dyDesc` [in]  -              Output gradient tensor descriptor.\n* `dy` [in]  -                  Pointer to output gradient tensor data.\n* `dxDesc` [in]  -              Input gradient tensor descriptor.\n* `dx` [in,out]  -                  Pointer to input gradient tensor data.\n* `dBnScaleBiasDesc` [in]  -    Shared descriptor for parameter gradient tensors.\n* `bnScale` [in]  -             Pointer to scale tensor data.\n* `dBnScaleResult` [out]  -      Pointer to scale gradient result.\n* `dBnBiasResult` [out]  -       Pointer to bias gradient result.\n* `epsilon` [in]  -             Same epsilon as forward pass.\n* `savedMean` [in]  -           Optionally cached mean from forward pass.\n* `savedInvVariance` [in]  -    Optionally cached inverse variance from forward pass.\n@retval CUDNN_STATUS_SUCCESS     The operation completed successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnBatchNormalizationForwardTraining`]"]
pub unsafe fn cudnnBatchNormalizationBackward<
    T0: types::CudaAsPtr,
    T1: types::CudaAsPtr,
    T2: types::CudaAsPtr,
    T3: types::CudaAsPtr,
    T4: types::CudaAsPtr,
    T5: types::CudaAsPtr,
    T6: types::CudaAsMutPtr,
    T7: types::CudaAsPtr,
    T8: types::CudaAsMutPtr,
    T9: types::CudaAsMutPtr,
    T10: types::CudaAsPtr,
    T11: types::CudaAsPtr,
>(
    handle: cudnnHandle_t,
    mode: cudnnBatchNormMode_t,
    alphaDataDiff: T0,
    betaDataDiff: T1,
    alphaParamDiff: T2,
    betaParamDiff: T3,
    xDesc: cudnnTensorDescriptor_t,
    x: T4,
    dyDesc: cudnnTensorDescriptor_t,
    dy: T5,
    dxDesc: cudnnTensorDescriptor_t,
    mut dx: T6,
    dBnScaleBiasDesc: cudnnTensorDescriptor_t,
    bnScale: T7,
    mut dBnScaleResult: T8,
    mut dBnBiasResult: T9,
    epsilon: f64,
    savedMean: T10,
    savedInvVariance: T11,
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
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Performs extended backward batch normalization with optional activation.\nComputes gradients for the fused batch normalization + activation operations.\n\n# Arguments\n\n* `handle` [in]  -                  cuDNN library handle.\n* `mode` [in]  -                    Batch normalization mode.\n* `bnOps` [in]  -                   Extended batch normalization operation.\n* `alphaDataDiff` [in]  -           Scaling factor for data gradient results.\n* `betaDataDiff` [in]  -            Scaling factor for data gradient destinations.\n* `alphaParamDiff` [in]  -          Scaling factor for parameter gradient results.\n* `betaParamDiff` [in]  -           Scaling factor for parameter gradient destinations.\n* `xDesc` [in]  -                   Input tensor descriptor.\n* `xData` [in]  -                   Pointer to input tensor data.\n* `yDesc` [in]  -                   Output tensor descriptor.\n* `yData` [in]  -                   Pointer to output tensor data.\n* `dyDesc` [in]  -                  Output gradient tensor descriptor.\n* `dyData` [in]  -                  Pointer to output gradient tensor data.\n* `dzDesc` [in]  -                  Z gradient tensor descriptor.\n* `dzData` [in,out]  -                  Pointer to z gradient tensor data.\n* `dxDesc` [in]  -                  Input gradient tensor descriptor.\n* `dxData` [in,out]  -                  Pointer to input gradient tensor data.\n* `dBnScaleBiasDesc` [in]  -        Shared descriptor for parameter gradient tensors.\n* `bnScaleData` [in]  -             Pointer to scale tensor data.\n* `bnBiasData` [in]  -              Pointer to bias tensor data (needed for activation).\n* `dBnScaleData` [out]  -            Pointer to scale gradient result.\n* `dBnBiasData` [out]  -             Pointer to bias gradient result.\n* `epsilon` [in]  -                 Same epsilon as forward pass.\n* `savedMean` [in]  -               Optionally cached mean from forward pass.\n* `savedInvVariance` [in]  -        Optionally cached inverse variance from forward pass.\n* `activationDesc` [in]  -          Activation descriptor.\n* `workSpace` [in,out]  -               Pointer to workspace memory.\n* `workSpaceSizeInBytes` [in]  -    Size of workspace in bytes.\n* `reserveSpace` [in,out]  -            Pointer to reserve space memory.\n* `reserveSpaceSizeInBytes` [in]  - Size of reserve space in bytes.\n@retval CUDNN_STATUS_SUCCESS     The operation completed successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnBatchNormalizationForwardTrainingEx`]"]
pub unsafe fn cudnnBatchNormalizationBackwardEx<
    T0: types::CudaAsPtr,
    T1: types::CudaAsPtr,
    T2: types::CudaAsPtr,
    T3: types::CudaAsPtr,
    T4: types::CudaAsPtr,
    T5: types::CudaAsPtr,
    T6: types::CudaAsPtr,
    T7: types::CudaAsMutPtr,
    T8: types::CudaAsMutPtr,
    T9: types::CudaAsPtr,
    T10: types::CudaAsPtr,
    T11: types::CudaAsMutPtr,
    T12: types::CudaAsMutPtr,
    T13: types::CudaAsPtr,
    T14: types::CudaAsPtr,
    T15: types::CudaAsMutPtr,
    T16: types::CudaAsMutPtr,
>(
    handle: cudnnHandle_t,
    mode: cudnnBatchNormMode_t,
    bnOps: cudnnBatchNormOps_t,
    alphaDataDiff: T0,
    betaDataDiff: T1,
    alphaParamDiff: T2,
    betaParamDiff: T3,
    xDesc: cudnnTensorDescriptor_t,
    xData: T4,
    yDesc: cudnnTensorDescriptor_t,
    yData: T5,
    dyDesc: cudnnTensorDescriptor_t,
    dyData: T6,
    dzDesc: cudnnTensorDescriptor_t,
    mut dzData: T7,
    dxDesc: cudnnTensorDescriptor_t,
    mut dxData: T8,
    dBnScaleBiasDesc: cudnnTensorDescriptor_t,
    bnScaleData: T9,
    bnBiasData: T10,
    mut dBnScaleData: T11,
    mut dBnBiasData: T12,
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
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Returns the workspace size for normalization forward training.\n\n# Arguments\n\n* `handle` [in]  -             cuDNN library handle.\n* `mode` [in]  -               Normalization mode.\n* `normOps` [in]  -            Extended normalization operation.\n* `algo` [in]  -               Normalization algorithm.\n* `xDesc` [in]  -              Input tensor descriptor.\n* `zDesc` [in]  -              Z tensor descriptor (for add operations).\n* `yDesc` [in]  -              Output tensor descriptor.\n* `normScaleBiasDesc` [in]  -  Descriptor for normalization scale/bias tensors.\n* `activationDesc` [in]  -     Activation descriptor.\n* `normMeanVarDesc` [in]  -    Descriptor for mean/variance tensors.\n* `sizeInBytes` [out]  -        Required workspace size in bytes.\n* `groupCnt` [in]  -           Group count (reserved, should be set to 1).\n@retval CUDNN_STATUS_SUCCESS  The size was returned successfully.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnNormalizationForwardTraining`]"]
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
    groupCnt: ::core::ffi::c_int,
) -> Result<usize, crate::sys::cudnnStatus_t> {
    let mut out_10: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnGetNormalizationForwardTrainingWorkspaceSize(handle, mode, normOps, algo, xDesc, zDesc, yDesc, normScaleBiasDesc, activationDesc, normMeanVarDesc, out_10.as_mut_ptr() as *mut _, groupCnt) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_10.assume_init() as usize) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Returns the workspace size for normalization backward.\n\n# Arguments\n\n* `handle` [in]  -              cuDNN library handle.\n* `mode` [in]  -                Normalization mode.\n* `normOps` [in]  -             Extended normalization operation.\n* `algo` [in]  -                Normalization algorithm.\n* `xDesc` [in]  -               Input tensor descriptor.\n* `yDesc` [in]  -               Output tensor descriptor.\n* `dyDesc` [in]  -              Output gradient tensor descriptor.\n* `dzDesc` [in]  -              Z gradient tensor descriptor.\n* `dxDesc` [in]  -              Input gradient tensor descriptor.\n* `dNormScaleBiasDesc` [in]  -  Descriptor for normalization parameter gradient tensors.\n* `activationDesc` [in]  -      Activation descriptor.\n* `normMeanVarDesc` [in]  -     Descriptor for mean/variance tensors.\n* `sizeInBytes` [out]  -         Required workspace size in bytes.\n* `groupCnt` [in]  -            Group count (reserved, should be set to 1).\n@retval CUDNN_STATUS_SUCCESS  The size was returned successfully.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnNormalizationBackward`]"]
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
    groupCnt: ::core::ffi::c_int,
) -> Result<usize, crate::sys::cudnnStatus_t> {
    let mut out_12: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnGetNormalizationBackwardWorkspaceSize(handle, mode, normOps, algo, xDesc, yDesc, dyDesc, dzDesc, dxDesc, dNormScaleBiasDesc, activationDesc, normMeanVarDesc, out_12.as_mut_ptr() as *mut _, groupCnt) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_12.assume_init() as usize) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Returns the reserve space size for normalization training.\n\n# Arguments\n\n* `handle` [in]  -          cuDNN library handle.\n* `mode` [in]  -            Normalization mode.\n* `normOps` [in]  -         Extended normalization operation.\n* `algo` [in]  -            Normalization algorithm.\n* `activationDesc` [in]  -  Activation descriptor.\n* `xDesc` [in]  -           Input tensor descriptor.\n* `sizeInBytes` [out]  -     Required reserve space size in bytes.\n* `groupCnt` [in]  -        Group count (reserved, should be set to 1).\n@retval CUDNN_STATUS_SUCCESS  The size was returned successfully.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnNormalizationForwardTraining`]"]
pub unsafe fn cudnnGetNormalizationTrainingReserveSpaceSize(
    handle: cudnnHandle_t,
    mode: cudnnNormMode_t,
    normOps: cudnnNormOps_t,
    algo: cudnnNormAlgo_t,
    activationDesc: cudnnActivationDescriptor_t,
    xDesc: cudnnTensorDescriptor_t,
    groupCnt: ::core::ffi::c_int,
) -> Result<usize, crate::sys::cudnnStatus_t> {
    let mut out_6: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnGetNormalizationTrainingReserveSpaceSize(handle, mode, normOps, algo, activationDesc, xDesc, out_6.as_mut_ptr() as *mut _, groupCnt) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_6.assume_init() as usize) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Performs normalization forward training with optional activation.\nComputes y = relu(Norm(x) + z). Also accumulates moving averages of mean\nand inverse variances.\n\n# Arguments\n\n* `handle` [in]  -                     cuDNN library handle.\n* `mode` [in]  -                       Normalization mode.\n* `normOps` [in]  -                    Extended normalization operation.\n* `algo` [in]  -                       Normalization algorithm.\n* `alpha` [in]  -                      Result blend factor.\n* `beta` [in]  -                       Destination layer blend factor.\n* `xDesc` [in]  -                      Input tensor descriptor.\n* `xData` [in]  -                      Pointer to input tensor data.\n* `normScaleBiasDesc` [in]  -          Descriptor for normalization scale/bias tensors.\n* `normScale` [in]  -                  Pointer to scale tensor data.\n* `normBias` [in]  -                   Pointer to bias tensor data.\n* `exponentialAverageFactor` [in]  -   Factor for computing running averages.\n* `normMeanVarDesc` [in]  -            Descriptor for mean/variance tensors.\n* `resultRunningMean` [in,out]  -          Running mean.\n* `resultRunningVariance` [in,out]  -      Running variance.\n* `epsilon` [in]  -                    Epsilon value (must be >= 0).\n* `resultSaveMean` [out]  -             Cached mean for backward pass (NULL if unused).\n* `resultSaveInvVariance` [out]  -      Cached inverse variance for backward pass (NULL if unused).\n* `activationDesc` [in]  -             Activation descriptor.\n* `zDesc` [in]  -                      Z tensor descriptor (for add operations).\n* `zData` [in]  -                      Pointer to z tensor data.\n* `yDesc` [in]  -                      Output tensor descriptor.\n* `yData` [out]  -                      Pointer to output tensor data.\n* `workspace` [in,out]  -                  Pointer to workspace memory.\n* `workSpaceSizeInBytes` [in]  -       Size of workspace in bytes.\n* `reserveSpace` [in,out]  -               Pointer to reserve space memory.\n* `reserveSpaceSizeInBytes` [in]  -    Size of reserve space in bytes.\n* `groupCnt` [in]  -                   Group count (reserved, should be set to 1).\n@retval CUDNN_STATUS_SUCCESS     The operation completed successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnNormalizationBackward,`] cudnnGetNormalizationForwardTrainingWorkspaceSize"]
pub unsafe fn cudnnNormalizationForwardTraining<
    T0: types::CudaAsPtr,
    T1: types::CudaAsPtr,
    T2: types::CudaAsPtr,
    T3: types::CudaAsPtr,
    T4: types::CudaAsPtr,
    T5: types::CudaAsMutPtr,
    T6: types::CudaAsMutPtr,
    T7: types::CudaAsMutPtr,
    T8: types::CudaAsMutPtr,
    T9: types::CudaAsPtr,
    T10: types::CudaAsMutPtr,
    T11: types::CudaAsMutPtr,
    T12: types::CudaAsMutPtr,
>(
    handle: cudnnHandle_t,
    mode: cudnnNormMode_t,
    normOps: cudnnNormOps_t,
    algo: cudnnNormAlgo_t,
    alpha: T0,
    beta: T1,
    xDesc: cudnnTensorDescriptor_t,
    xData: T2,
    normScaleBiasDesc: cudnnTensorDescriptor_t,
    normScale: T3,
    normBias: T4,
    exponentialAverageFactor: f64,
    normMeanVarDesc: cudnnTensorDescriptor_t,
    mut resultRunningMean: T5,
    mut resultRunningVariance: T6,
    epsilon: f64,
    mut resultSaveMean: T7,
    mut resultSaveInvVariance: T8,
    activationDesc: cudnnActivationDescriptor_t,
    zDesc: cudnnTensorDescriptor_t,
    zData: T9,
    yDesc: cudnnTensorDescriptor_t,
    mut yData: T10,
    mut workspace: T11,
    workSpaceSizeInBytes: usize,
    mut reserveSpace: T12,
    reserveSpaceSizeInBytes: usize,
    groupCnt: ::core::ffi::c_int,
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
            groupCnt,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Performs backward normalization.\nComputes gradients for the normalization operation, including optional activation\nand element-wise add gradients.\n\n# Arguments\n\n* `handle` [in]  -                  cuDNN library handle.\n* `mode` [in]  -                    Normalization mode.\n* `normOps` [in]  -                 Extended normalization operation.\n* `algo` [in]  -                    Normalization algorithm.\n* `alphaDataDiff` [in]  -           Scaling factor for data gradient results.\n* `betaDataDiff` [in]  -            Scaling factor for data gradient destinations.\n* `alphaParamDiff` [in]  -          Scaling factor for parameter gradient results.\n* `betaParamDiff` [in]  -           Scaling factor for parameter gradient destinations.\n* `xDesc` [in]  -                   Input tensor descriptor.\n* `xData` [in]  -                   Pointer to input tensor data.\n* `yDesc` [in]  -                   Output tensor descriptor.\n* `yData` [in]  -                   Pointer to output tensor data.\n* `dyDesc` [in]  -                  Output gradient tensor descriptor.\n* `dyData` [in]  -                  Pointer to output gradient tensor data.\n* `dzDesc` [in]  -                  Z gradient tensor descriptor.\n* `dzData` [in,out]  -                  Pointer to z gradient tensor data.\n* `dxDesc` [in]  -                  Input gradient tensor descriptor.\n* `dxData` [in,out]  -                  Pointer to input gradient tensor data.\n* `dNormScaleBiasDesc` [in]  -      Shared descriptor for parameter gradient tensors.\n* `normScaleData` [in]  -           Pointer to scale tensor data.\n* `normBiasData` [in]  -            Pointer to bias tensor data (needed for activation).\n* `dNormScaleData` [out]  -          Pointer to scale gradient result.\n* `dNormBiasData` [out]  -           Pointer to bias gradient result.\n* `epsilon` [in]  -                 Same epsilon as forward pass.\n* `normMeanVarDesc` [in]  -         Descriptor for mean/variance tensors.\n* `savedMean` [in]  -               Optionally cached mean from forward pass.\n* `savedInvVariance` [in]  -        Optionally cached inverse variance from forward pass.\n* `activationDesc` [in]  -          Activation descriptor.\n* `workSpace` [in,out]  -               Pointer to workspace memory.\n* `workSpaceSizeInBytes` [in]  -    Size of workspace in bytes.\n* `reserveSpace` [in,out]  -            Pointer to reserve space memory.\n* `reserveSpaceSizeInBytes` [in]  - Size of reserve space in bytes.\n* `groupCnt` [in]  -                Group count (reserved, should be set to 1).\n@retval CUDNN_STATUS_SUCCESS     The operation completed successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n\n# See also\n\n> [`cudnnNormalizationForwardTraining`]"]
pub unsafe fn cudnnNormalizationBackward<
    T0: types::CudaAsPtr,
    T1: types::CudaAsPtr,
    T2: types::CudaAsPtr,
    T3: types::CudaAsPtr,
    T4: types::CudaAsPtr,
    T5: types::CudaAsPtr,
    T6: types::CudaAsPtr,
    T7: types::CudaAsMutPtr,
    T8: types::CudaAsMutPtr,
    T9: types::CudaAsPtr,
    T10: types::CudaAsPtr,
    T11: types::CudaAsMutPtr,
    T12: types::CudaAsMutPtr,
    T13: types::CudaAsPtr,
    T14: types::CudaAsPtr,
    T15: types::CudaAsMutPtr,
    T16: types::CudaAsMutPtr,
>(
    handle: cudnnHandle_t,
    mode: cudnnNormMode_t,
    normOps: cudnnNormOps_t,
    algo: cudnnNormAlgo_t,
    alphaDataDiff: T0,
    betaDataDiff: T1,
    alphaParamDiff: T2,
    betaParamDiff: T3,
    xDesc: cudnnTensorDescriptor_t,
    xData: T4,
    yDesc: cudnnTensorDescriptor_t,
    yData: T5,
    dyDesc: cudnnTensorDescriptor_t,
    dyData: T6,
    dzDesc: cudnnTensorDescriptor_t,
    mut dzData: T7,
    dxDesc: cudnnTensorDescriptor_t,
    mut dxData: T8,
    dNormScaleBiasDesc: cudnnTensorDescriptor_t,
    normScaleData: T9,
    normBiasData: T10,
    mut dNormScaleData: T11,
    mut dNormBiasData: T12,
    epsilon: f64,
    normMeanVarDesc: cudnnTensorDescriptor_t,
    savedMean: T13,
    savedInvVariance: T14,
    activationDesc: cudnnActivationDescriptor_t,
    mut workSpace: T15,
    workSpaceSizeInBytes: usize,
    mut reserveSpace: T16,
    reserveSpaceSizeInBytes: usize,
    groupCnt: ::core::ffi::c_int,
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
            groupCnt,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Computes the gradient of the spatial transformer grid generator (backward).\n\n# Arguments\n\n* `handle` [in]  -  cuDNN library handle.\n* `stDesc` [in]  -  Spatial transformer descriptor.\n* `dgrid` [in]  -   Pointer to the grid gradient data.\n* `dtheta` [out]  -  Pointer to the theta gradient data.\n@retval CUDNN_STATUS_SUCCESS     The operation completed successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnSpatialTfGridGeneratorForward`]"]
pub unsafe fn cudnnSpatialTfGridGeneratorBackward<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cudnnHandle_t, stDesc: cudnnSpatialTransformerDescriptor_t, dgrid: T0, mut dtheta: T1) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnSpatialTfGridGeneratorBackward(handle, stDesc, dgrid.as_const_ptr() as *const _, dtheta.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Performs spatial transformer sampling backward.\nComputes the gradients of the spatial transformer sampler.\n\n# Arguments\n\n* `handle` [in]  -     cuDNN library handle.\n* `stDesc` [in]  -     Spatial transformer descriptor.\n* `alpha` [in]  -      Scaling factor for the dx result.\n* `xDesc` [in]  -      Input tensor descriptor.\n* `x` [in]  -          Pointer to input tensor data.\n* `beta` [in]  -       Scaling factor for the dx destination.\n* `dxDesc` [in]  -     Input gradient tensor descriptor.\n* `dx` [in,out]  -         Pointer to input gradient tensor data.\n* `alphaDgrid` [in]  - Scaling factor for the dgrid result.\n* `dyDesc` [in]  -     Output gradient tensor descriptor.\n* `dy` [in]  -         Pointer to output gradient tensor data.\n* `grid` [in]  -       Pointer to sampling grid data.\n* `betaDgrid` [in]  -  Scaling factor for the dgrid destination.\n* `dgrid` [in,out]  -      Pointer to grid gradient tensor data.\n@retval CUDNN_STATUS_SUCCESS     The operation completed successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnSpatialTfSamplerForward`]"]
pub unsafe fn cudnnSpatialTfSamplerBackward<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsPtr, T5: types::CudaAsPtr, T6: types::CudaAsPtr, T7: types::CudaAsPtr, T8: types::CudaAsMutPtr>(
    handle: cudnnHandle_t,
    stDesc: cudnnSpatialTransformerDescriptor_t,
    alpha: T0,
    xDesc: cudnnTensorDescriptor_t,
    x: T1,
    beta: T2,
    dxDesc: cudnnTensorDescriptor_t,
    mut dx: T3,
    alphaDgrid: T4,
    dyDesc: cudnnTensorDescriptor_t,
    dy: T5,
    grid: T6,
    betaDgrid: T7,
    mut dgrid: T8,
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
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Performs backward dropout.\nApplies the same dropout mask from the forward pass (stored in reserveSpace)\nto the gradient tensor.\n\n# Arguments\n\n* `handle` [in]  -                  cuDNN library handle.\n* `dropoutDesc` [in]  -             Dropout descriptor.\n* `dydesc` [in]  -                  Output gradient tensor descriptor.\n* `dy` [in]  -                      Pointer to output gradient tensor data.\n* `dxdesc` [in]  -                  Input gradient tensor descriptor.\n* `dx` [out]  -                      Pointer to input gradient tensor data.\n* `reserveSpace` [in]  -            Pointer to reserve space from forward pass.\n* `reserveSpaceSizeInBytes` [in]  - Size of reserve space in bytes.\n@retval CUDNN_STATUS_SUCCESS     The operation completed successfully.\n@retval CUDNN_STATUS_BAD_PARAM   An invalid parameter was provided.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnDropoutForward`]"]
pub unsafe fn cudnnDropoutBackward<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr>(
    handle: cudnnHandle_t,
    dropoutDesc: cudnnDropoutDescriptor_t,
    dydesc: cudnnTensorDescriptor_t,
    dy: T0,
    dxdesc: cudnnTensorDescriptor_t,
    mut dx: T1,
    mut reserveSpace: T2,
    reserveSpaceSizeInBytes: usize,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnDropoutBackward(handle, dropoutDesc, dydesc, dy.as_const_ptr() as *const _, dxdesc, dx.as_mut_ptr() as *mut _, reserveSpace.as_mut_ptr() as *mut _, reserveSpaceSizeInBytes) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Creates an RNN descriptor.\n\n# Arguments\n\n* `rnnDesc` [out]  -  Pointer to the created RNN descriptor.\n@retval CUDNN_STATUS_SUCCESS  Descriptor created successfully.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnDestroyRNNDescriptor`]"]
pub unsafe fn cudnnCreateRNNDescriptor() -> Result<cudnnRNNDescriptor_t, crate::sys::cudnnStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cudnnRNNDescriptor_t> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnCreateRNNDescriptor(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cudnnRNNDescriptor_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Destroys an RNN descriptor.\n\n# Arguments\n\n* `rnnDesc` [in]  -  RNN descriptor to destroy.\n@retval CUDNN_STATUS_SUCCESS  Descriptor destroyed successfully.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnCreateRNNDescriptor`]"]
pub unsafe fn cudnnDestroyRNNDescriptor(rnnDesc: cudnnRNNDescriptor_t) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnDestroyRNNDescriptor(rnnDesc) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Configures an RNN descriptor with network parameters.\n\n# Arguments\n\n* `rnnDesc` [in,out]  -     RNN descriptor to configure.\n* `algo` [in]  -        RNN computation algorithm.\n* `cellMode` [in]  -    RNN cell type (RELU, TANH, LSTM, GRU).\n* `biasMode` [in]  -    Bias configuration.\n* `dirMode` [in]  -     Unidirectional or bidirectional.\n* `inputMode` [in]  -   First layer input behavior.\n* `dataType` [in]  -    Input/output and weight data type.\n* `mathPrec` [in]  -    Compute precision.\n* `mathType` [in]  -    Tensor Core usage preference.\n* `inputSize` [in]  -   Input vector size.\n* `hiddenSize` [in]  -  Hidden state size.\n* `projSize` [in]  -    Recurrent projection size (0 to disable).\n* `numLayers` [in]  -   Number of stacked RNN layers.\n* `dropoutDesc` [in]  - Dropout descriptor for inter-layer dropout.\n* `auxFlags` [in]  -    Auxiliary flags (e.g., CUDNN_RNN_PADDED_IO_ENABLED).\n@retval CUDNN_STATUS_SUCCESS       Descriptor configured successfully.\n@retval CUDNN_STATUS_BAD_PARAM     Invalid parameter.\n@retval CUDNN_STATUS_NOT_SUPPORTED Unsupported configuration.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnGetRNNDescriptor_v8`]"]
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
    let status = unsafe { crate::sys::cudnnSetRNNDescriptor_v8(rnnDesc, algo, cellMode, biasMode, dirMode, inputMode, dataType, mathPrec, mathType, inputSize, hiddenSize, projSize, numLayers, dropoutDesc, auxFlags) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Retrieves RNN descriptor parameters.\n\n# Arguments\n\n* `rnnDesc` [in]  -     RNN descriptor to query.\n* `algo` [out]  -        RNN algorithm.\n* `cellMode` [out]  -    Cell type.\n* `biasMode` [out]  -    Bias configuration.\n* `dirMode` [out]  -     Direction mode.\n* `inputMode` [out]  -   Input mode.\n* `dataType` [out]  -    Data type.\n* `mathPrec` [out]  -    Math precision.\n* `mathType` [out]  -    Math type.\n* `inputSize` [out]  -   Input size.\n* `hiddenSize` [out]  -  Hidden size.\n* `projSize` [out]  -    Projection size.\n* `numLayers` [out]  -   Number of layers.\n* `dropoutDesc` [out]  - Dropout descriptor.\n* `auxFlags` [out]  -    Auxiliary flags.\n@retval CUDNN_STATUS_SUCCESS  Query succeeded.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnSetRNNDescriptor_v8`]"]
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
    let mut out_1: std::mem::MaybeUninit<cudnnRNNAlgo_t> = std::mem::MaybeUninit::zeroed();
    let mut out_2: std::mem::MaybeUninit<cudnnRNNMode_t> = std::mem::MaybeUninit::zeroed();
    let mut out_3: std::mem::MaybeUninit<cudnnRNNBiasMode_t> = std::mem::MaybeUninit::zeroed();
    let mut out_4: std::mem::MaybeUninit<cudnnDirectionMode_t> = std::mem::MaybeUninit::zeroed();
    let mut out_5: std::mem::MaybeUninit<cudnnRNNInputMode_t> = std::mem::MaybeUninit::zeroed();
    let mut out_6: std::mem::MaybeUninit<cudnnDataType_t> = std::mem::MaybeUninit::zeroed();
    let mut out_7: std::mem::MaybeUninit<cudnnDataType_t> = std::mem::MaybeUninit::zeroed();
    let mut out_8: std::mem::MaybeUninit<cudnnMathType_t> = std::mem::MaybeUninit::zeroed();
    let mut out_9: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::zeroed();
    let mut out_10: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::zeroed();
    let mut out_11: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::zeroed();
    let mut out_12: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::zeroed();
    let mut out_13: std::mem::MaybeUninit<cudnnDropoutDescriptor_t> = std::mem::MaybeUninit::zeroed();
    let mut out_14: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::zeroed();
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
#[doc = "Configures LSTM cell clipping parameters.\n> **Deprecated** Since cuDNN 9.0.0. Use cudnnRNNSetClip_v9 instead.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnRNNSetClip_v8(rnnDesc: cudnnRNNDescriptor_t, clipMode: cudnnRNNClipMode_t, clipNanOpt: cudnnNanPropagation_t, lclip: f64, rclip: f64) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnRNNSetClip_v8(rnnDesc, clipMode, clipNanOpt, lclip, rclip) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Configures LSTM cell clipping parameters.\n\n# Arguments\n\n* `rnnDesc` [in,out]  -   RNN descriptor.\n* `clipMode` [in]  -  Clipping mode (NONE or MINMAX).\n* `lclip` [in]  -     Left (minimum) clipping value.\n* `rclip` [in]  -     Right (maximum) clipping value.\n@retval CUDNN_STATUS_SUCCESS  Clipping configured.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnRNNGetClip_v9`]"]
pub unsafe fn cudnnRNNSetClip_v9(rnnDesc: cudnnRNNDescriptor_t, clipMode: cudnnRNNClipMode_t, lclip: f64, rclip: f64) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnRNNSetClip_v9(rnnDesc, clipMode, lclip, rclip) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Retrieves LSTM cell clipping settings.\n> **Deprecated** Since cuDNN 9.0.0. Use cudnnRNNGetClip_v9 instead.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnRNNGetClip_v8(rnnDesc: cudnnRNNDescriptor_t) -> Result<(cudnnRNNClipMode_t, cudnnNanPropagation_t, f64, f64), crate::sys::cudnnStatus_t> {
    let mut out_1: std::mem::MaybeUninit<cudnnRNNClipMode_t> = std::mem::MaybeUninit::zeroed();
    let mut out_2: std::mem::MaybeUninit<cudnnNanPropagation_t> = std::mem::MaybeUninit::zeroed();
    let mut out_3: std::mem::MaybeUninit<f64> = std::mem::MaybeUninit::zeroed();
    let mut out_4: std::mem::MaybeUninit<f64> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnRNNGetClip_v8(rnnDesc, out_1.as_mut_ptr() as *mut _, out_2.as_mut_ptr() as *mut _, out_3.as_mut_ptr() as *mut _, out_4.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok((out_1.assume_init() as cudnnRNNClipMode_t, out_2.assume_init() as cudnnNanPropagation_t, out_3.assume_init() as f64, out_4.assume_init() as f64)) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Retrieves LSTM cell clipping settings.\n\n# Arguments\n\n* `rnnDesc` [in]  -   RNN descriptor.\n* `clipMode` [out]  -  Clipping mode.\n* `lclip` [out]  -     Left clipping value.\n* `rclip` [out]  -     Right clipping value.\n@retval CUDNN_STATUS_SUCCESS  Query succeeded.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnRNNSetClip_v9`]"]
pub unsafe fn cudnnRNNGetClip_v9(rnnDesc: cudnnRNNDescriptor_t) -> Result<(cudnnRNNClipMode_t, f64, f64), crate::sys::cudnnStatus_t> {
    let mut out_1: std::mem::MaybeUninit<cudnnRNNClipMode_t> = std::mem::MaybeUninit::zeroed();
    let mut out_2: std::mem::MaybeUninit<f64> = std::mem::MaybeUninit::zeroed();
    let mut out_3: std::mem::MaybeUninit<f64> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnRNNGetClip_v9(rnnDesc, out_1.as_mut_ptr() as *mut _, out_2.as_mut_ptr() as *mut _, out_3.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok((out_1.assume_init() as cudnnRNNClipMode_t, out_2.assume_init() as f64, out_3.assume_init() as f64)) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Compiles persistent RNN code using NVRTC for dynamic algorithm.\n\n# Arguments\n\n* `handle` [in]  -     cuDNN handle.\n* `rnnDesc` [in]  -    RNN descriptor (must use PERSIST_DYNAMIC algorithm).\n* `miniBatch` [in]  -  Exact mini-batch size for compilation.\n@retval CUDNN_STATUS_SUCCESS       Compilation succeeded.\n@retval CUDNN_STATUS_NOT_SUPPORTED Unsupported configuration.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnBuildRNNDynamic(handle: cudnnHandle_t, rnnDesc: cudnnRNNDescriptor_t, miniBatch: ::core::ffi::c_int) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnBuildRNNDynamic(handle, rnnDesc, miniBatch) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Computes workspace and reserve space buffer sizes for RNN.\n\n# Arguments\n\n* `handle` [in]  -          cuDNN handle.\n* `rnnDesc` [in]  -         RNN descriptor.\n* `fwdMode` [in]  -         Inference or training mode.\n* `xDesc` [in]  -           Input data descriptor.\n* `workSpaceSize` [out]  -   Required workspace size in bytes.\n* `reserveSpaceSize` [out]  - Required reserve space size in bytes (training only).\n@retval CUDNN_STATUS_SUCCESS  Sizes computed successfully.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnGetRNNTempSpaceSizes(handle: cudnnHandle_t, rnnDesc: cudnnRNNDescriptor_t, fwdMode: cudnnForwardMode_t, xDesc: cudnnRNNDataDescriptor_t) -> Result<(usize, usize), crate::sys::cudnnStatus_t> {
    let mut out_4: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::zeroed();
    let mut out_5: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnGetRNNTempSpaceSizes(handle, rnnDesc, fwdMode, xDesc, out_4.as_mut_ptr() as *mut _, out_5.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok((out_4.assume_init() as usize, out_5.assume_init() as usize)) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Reports required GPU memory for all RNN weight parameters.\n\n# Arguments\n\n* `handle` [in]  -          cuDNN handle.\n* `rnnDesc` [in]  -         RNN descriptor.\n* `weightSpaceSize` [out]  - Required weight space size in bytes.\n@retval CUDNN_STATUS_SUCCESS  Size computed.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnGetRNNWeightSpaceSize(handle: cudnnHandle_t, rnnDesc: cudnnRNNDescriptor_t) -> Result<usize, crate::sys::cudnnStatus_t> {
    let mut out_2: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnGetRNNWeightSpaceSize(handle, rnnDesc, out_2.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_2.assume_init() as usize) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Obtains start address and shape of RNN weight matrices and bias vectors.\n\n# Arguments\n\n* `handle` [in]  -          cuDNN handle.\n* `rnnDesc` [in]  -         RNN descriptor.\n* `pseudoLayer` [in]  -     Pseudo-layer index (physical layer and direction).\n* `weightSpaceSize` [in]  - Total weight space size.\n* `weightSpace` [in]  -     Pointer to weight space.\n* `linLayerID` [in]  -      Linear layer ID within the RNN cell.\n* `mDesc` [out]  -           Tensor descriptor for the weight matrix.\n* `mAddr` [out]  -           Start address of the weight matrix.\n* `bDesc` [out]  -           Tensor descriptor for the bias vector.\n* `bAddr` [out]  -           Start address of the bias vector.\n@retval CUDNN_STATUS_SUCCESS  Parameters retrieved.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnGetRNNWeightParams<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsMutPtr>(
    handle: cudnnHandle_t,
    rnnDesc: cudnnRNNDescriptor_t,
    pseudoLayer: i32,
    weightSpaceSize: usize,
    weightSpace: T0,
    linLayerID: i32,
    mDesc: cudnnTensorDescriptor_t,
    mut mAddr: T1,
    bDesc: cudnnTensorDescriptor_t,
    mut bAddr: T2,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnGetRNNWeightParams(handle, rnnDesc, pseudoLayer, weightSpaceSize, weightSpace.as_const_ptr() as *const _, linLayerID, mDesc, mAddr.as_mut_ptr() as *mut _, bDesc, bAddr.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Creates an RNN data descriptor.\n\n# Arguments\n\n* `rnnDataDesc` [out]  -  Pointer to created descriptor.\n@retval CUDNN_STATUS_SUCCESS  Descriptor created.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnCreateRNNDataDescriptor() -> Result<cudnnRNNDataDescriptor_t, crate::sys::cudnnStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cudnnRNNDataDescriptor_t> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnCreateRNNDataDescriptor(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cudnnRNNDataDescriptor_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Destroys an RNN data descriptor.\n\n# Arguments\n\n* `rnnDataDesc` [in]  -  Descriptor to destroy.\n@retval CUDNN_STATUS_SUCCESS  Descriptor destroyed.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnDestroyRNNDataDescriptor(rnnDataDesc: cudnnRNNDataDescriptor_t) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnDestroyRNNDataDescriptor(rnnDataDesc) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Configures an RNN data descriptor with layout and sequence information.\n\n# Arguments\n\n* `rnnDataDesc` [in,out]  -     RNN data descriptor.\n* `dataType` [in]  -        Data type.\n* `layout` [in]  -          Data layout (sequence-major or batch-major).\n* `maxSeqLength` [in]  -    Maximum sequence length.\n* `batchSize` [in]  -       Batch size.\n* `vectorSize` [in]  -      Input vector size.\n* `seqLengthArray` [in]  -  Length of each sequence in the batch.\n* `paddingFill` [in,out]  -     Symbol for filling padding positions.\n@retval CUDNN_STATUS_SUCCESS  Descriptor configured.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnSetRNNDataDescriptor<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(
    rnnDataDesc: cudnnRNNDataDescriptor_t,
    dataType: cudnnDataType_t,
    layout: cudnnRNNDataLayout_t,
    maxSeqLength: ::core::ffi::c_int,
    batchSize: ::core::ffi::c_int,
    vectorSize: ::core::ffi::c_int,
    seqLengthArray: T0,
    mut paddingFill: T1,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnSetRNNDataDescriptor(rnnDataDesc, dataType, layout, maxSeqLength, batchSize, vectorSize, seqLengthArray.as_const_ptr() as *const _, paddingFill.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Retrieves RNN data descriptor parameters.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnGetRNNDataDescriptor(
    rnnDataDesc: cudnnRNNDataDescriptor_t,
    arrayLengthRequested: ::core::ffi::c_int,
    seqLengthArray: *mut ::core::ffi::c_int,
    paddingFill: *mut ::core::ffi::c_void,
) -> Result<(cudnnDataType_t, cudnnRNNDataLayout_t, ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int), crate::sys::cudnnStatus_t> {
    let mut out_1: std::mem::MaybeUninit<cudnnDataType_t> = std::mem::MaybeUninit::zeroed();
    let mut out_2: std::mem::MaybeUninit<cudnnRNNDataLayout_t> = std::mem::MaybeUninit::zeroed();
    let mut out_3: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let mut out_4: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let mut out_5: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
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
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe {
            Ok((
                out_1.assume_init() as cudnnDataType_t,
                out_2.assume_init() as cudnnRNNDataLayout_t,
                out_3.assume_init() as ::core::ffi::c_int,
                out_4.assume_init() as ::core::ffi::c_int,
                out_5.assume_init() as ::core::ffi::c_int,
            ))
        }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Computes the forward pass of an RNN network.\n\n# Arguments\n\n* `handle` [in]  -           cuDNN handle.\n* `rnnDesc` [in]  -          RNN descriptor.\n* `fwdMode` [in]  -          Inference or training mode.\n* `devSeqLengths` [in]  -    Per-batch sequence lengths (device memory).\n* `xDesc` [in]  -            Input data descriptor.\n* `x` [in]  -                Input data pointer.\n* `yDesc` [in]  -            Output data descriptor.\n* `y` [out]  -                Output data pointer.\n* `hDesc` [in]  -            Hidden state descriptor.\n* `hx` [in]  -               Initial hidden state (NULL for zero).\n* `hy` [out]  -               Final hidden state (NULL to discard).\n* `cDesc` [in]  -            Cell state descriptor (LSTM only).\n* `cx` [in]  -               Initial cell state (NULL for zero).\n* `cy` [out]  -               Final cell state (NULL to discard).\n* `weightSpaceSize` [in]  -  Weight space size in bytes.\n* `weightSpace` [in]  -      Weight space pointer.\n* `workSpaceSize` [in]  -    Workspace size in bytes.\n* `workSpace` [in,out]  -        Workspace pointer.\n* `reserveSpaceSize` [in]  - Reserve space size (training only).\n* `reserveSpace` [in,out]  -     Reserve space pointer (training only).\n@retval CUDNN_STATUS_SUCCESS        Forward pass completed.\n@retval CUDNN_STATUS_BAD_PARAM      Invalid parameter.\n@retval CUDNN_STATUS_EXECUTION_FAILED  Execution failed.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnRNNBackwardData_v8,`] cudnnRNNBackwardWeights_v8"]
pub unsafe fn cudnnRNNForward<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsPtr, T6: types::CudaAsMutPtr, T7: types::CudaAsPtr, T8: types::CudaAsMutPtr, T9: types::CudaAsMutPtr>(
    handle: cudnnHandle_t,
    rnnDesc: cudnnRNNDescriptor_t,
    fwdMode: cudnnForwardMode_t,
    devSeqLengths: T0,
    xDesc: cudnnRNNDataDescriptor_t,
    x: T1,
    yDesc: cudnnRNNDataDescriptor_t,
    mut y: T2,
    hDesc: cudnnTensorDescriptor_t,
    hx: T3,
    mut hy: T4,
    cDesc: cudnnTensorDescriptor_t,
    cx: T5,
    mut cy: T6,
    weightSpaceSize: usize,
    weightSpace: T7,
    workSpaceSize: usize,
    mut workSpace: T8,
    reserveSpaceSize: usize,
    mut reserveSpace: T9,
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
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Creates a sequence data descriptor.\n> **Deprecated** Since cuDNN 9.0.0. Use RNN data descriptors instead.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnCreateSeqDataDescriptor() -> Result<cudnnSeqDataDescriptor_t, crate::sys::cudnnStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cudnnSeqDataDescriptor_t> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnCreateSeqDataDescriptor(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cudnnSeqDataDescriptor_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Destroys a sequence data descriptor.\n> **Deprecated** Since cuDNN 9.0.0.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnDestroySeqDataDescriptor(seqDataDesc: cudnnSeqDataDescriptor_t) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnDestroySeqDataDescriptor(seqDataDesc) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Configures a sequence data descriptor.\n> **Deprecated** Since cuDNN 9.0.0.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnSetSeqDataDescriptor<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(
    seqDataDesc: cudnnSeqDataDescriptor_t,
    dataType: cudnnDataType_t,
    nbDims: ::core::ffi::c_int,
    dimA: T0,
    axes: T1,
    seqLengthArraySize: usize,
    seqLengthArray: T2,
    mut paddingFill: T3,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnSetSeqDataDescriptor(
            seqDataDesc,
            dataType,
            nbDims,
            dimA.as_const_ptr() as *const _,
            axes.as_const_ptr() as *const _,
            seqLengthArraySize,
            seqLengthArray.as_const_ptr() as *const _,
            paddingFill.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Retrieves sequence data descriptor parameters.\n> **Deprecated** Since cuDNN 9.0.0.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnGetSeqDataDescriptor(
    seqDataDesc: cudnnSeqDataDescriptor_t,
    nbDimsRequested: ::core::ffi::c_int,
    seqLengthArraySize: *mut usize,
    seqLengthSizeRequested: usize,
    seqLengthArray: *mut ::core::ffi::c_int,
    paddingFill: *mut ::core::ffi::c_void,
) -> Result<(cudnnDataType_t, ::core::ffi::c_int, ::core::ffi::c_int, cudnnSeqDataAxis_t), crate::sys::cudnnStatus_t> {
    let mut out_1: std::mem::MaybeUninit<cudnnDataType_t> = std::mem::MaybeUninit::zeroed();
    let mut out_2: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let mut out_4: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let mut out_5: std::mem::MaybeUninit<cudnnSeqDataAxis_t> = std::mem::MaybeUninit::zeroed();
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
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok((out_1.assume_init() as cudnnDataType_t, out_2.assume_init() as ::core::ffi::c_int, out_4.assume_init() as ::core::ffi::c_int, out_5.assume_init() as cudnnSeqDataAxis_t)) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Creates a multi-head attention descriptor.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API SDPA operations instead.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnCreateAttnDescriptor() -> Result<cudnnAttnDescriptor_t, crate::sys::cudnnStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cudnnAttnDescriptor_t> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnCreateAttnDescriptor(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cudnnAttnDescriptor_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Destroys a multi-head attention descriptor.\n> **Deprecated** Since cuDNN 9.0.0.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnDestroyAttnDescriptor(attnDesc: cudnnAttnDescriptor_t) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnDestroyAttnDescriptor(attnDesc) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Configures a multi-head attention descriptor.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API SDPA operations instead.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnSetAttnDescriptor(
    attnDesc: cudnnAttnDescriptor_t,
    attnMode: ::core::ffi::c_uint,
    nHeads: ::core::ffi::c_int,
    smScaler: f64,
    dataType: cudnnDataType_t,
    computePrec: cudnnDataType_t,
    mathType: cudnnMathType_t,
    attnDropoutDesc: cudnnDropoutDescriptor_t,
    postDropoutDesc: cudnnDropoutDescriptor_t,
    qSize: ::core::ffi::c_int,
    kSize: ::core::ffi::c_int,
    vSize: ::core::ffi::c_int,
    qProjSize: ::core::ffi::c_int,
    kProjSize: ::core::ffi::c_int,
    vProjSize: ::core::ffi::c_int,
    oProjSize: ::core::ffi::c_int,
    qoMaxSeqLength: ::core::ffi::c_int,
    kvMaxSeqLength: ::core::ffi::c_int,
    maxBatchSize: ::core::ffi::c_int,
    maxBeamSize: ::core::ffi::c_int,
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
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Retrieves multi-head attention descriptor parameters.\n> **Deprecated** Since cuDNN 9.0.0.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnGetAttnDescriptor(
    attnDesc: cudnnAttnDescriptor_t,
) -> Result<
    (
        ::core::ffi::c_uint,
        ::core::ffi::c_int,
        f64,
        cudnnDataType_t,
        cudnnDataType_t,
        cudnnMathType_t,
        cudnnDropoutDescriptor_t,
        cudnnDropoutDescriptor_t,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
    ),
    crate::sys::cudnnStatus_t,
> {
    let mut out_1: std::mem::MaybeUninit<::core::ffi::c_uint> = std::mem::MaybeUninit::zeroed();
    let mut out_2: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let mut out_3: std::mem::MaybeUninit<f64> = std::mem::MaybeUninit::zeroed();
    let mut out_4: std::mem::MaybeUninit<cudnnDataType_t> = std::mem::MaybeUninit::zeroed();
    let mut out_5: std::mem::MaybeUninit<cudnnDataType_t> = std::mem::MaybeUninit::zeroed();
    let mut out_6: std::mem::MaybeUninit<cudnnMathType_t> = std::mem::MaybeUninit::zeroed();
    let mut out_7: std::mem::MaybeUninit<cudnnDropoutDescriptor_t> = std::mem::MaybeUninit::zeroed();
    let mut out_8: std::mem::MaybeUninit<cudnnDropoutDescriptor_t> = std::mem::MaybeUninit::zeroed();
    let mut out_9: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let mut out_10: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let mut out_11: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let mut out_12: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let mut out_13: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let mut out_14: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let mut out_15: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let mut out_16: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let mut out_17: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let mut out_18: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let mut out_19: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
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
                out_1.assume_init() as ::core::ffi::c_uint,
                out_2.assume_init() as ::core::ffi::c_int,
                out_3.assume_init() as f64,
                out_4.assume_init() as cudnnDataType_t,
                out_5.assume_init() as cudnnDataType_t,
                out_6.assume_init() as cudnnMathType_t,
                out_7.assume_init() as cudnnDropoutDescriptor_t,
                out_8.assume_init() as cudnnDropoutDescriptor_t,
                out_9.assume_init() as ::core::ffi::c_int,
                out_10.assume_init() as ::core::ffi::c_int,
                out_11.assume_init() as ::core::ffi::c_int,
                out_12.assume_init() as ::core::ffi::c_int,
                out_13.assume_init() as ::core::ffi::c_int,
                out_14.assume_init() as ::core::ffi::c_int,
                out_15.assume_init() as ::core::ffi::c_int,
                out_16.assume_init() as ::core::ffi::c_int,
                out_17.assume_init() as ::core::ffi::c_int,
                out_18.assume_init() as ::core::ffi::c_int,
                out_19.assume_init() as ::core::ffi::c_int,
            ))
        }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Computes weight, workspace, and reserve space sizes for multi-head attention.\n> **Deprecated** Since cuDNN 9.0.0.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnGetMultiHeadAttnBuffers(handle: cudnnHandle_t, attnDesc: cudnnAttnDescriptor_t) -> Result<(usize, usize, usize), crate::sys::cudnnStatus_t> {
    let mut out_2: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::zeroed();
    let mut out_3: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::zeroed();
    let mut out_4: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnGetMultiHeadAttnBuffers(handle, attnDesc, out_2.as_mut_ptr() as *mut _, out_3.as_mut_ptr() as *mut _, out_4.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok((out_2.assume_init() as usize, out_3.assume_init() as usize, out_4.assume_init() as usize)) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Obtains shape and start address of attention weight/bias tensors.\n> **Deprecated** Since cuDNN 9.0.0.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnGetMultiHeadAttnWeights<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(
    handle: cudnnHandle_t,
    attnDesc: cudnnAttnDescriptor_t,
    wKind: cudnnMultiHeadAttnWeightKind_t,
    weightSizeInBytes: usize,
    weights: T0,
    wDesc: cudnnTensorDescriptor_t,
    mut wAddr: T1,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnGetMultiHeadAttnWeights(handle, attnDesc, wKind, weightSizeInBytes, weights.as_const_ptr() as *const _, wDesc, wAddr.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Computes multi-head attention forward pass.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API SDPA operations instead.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnMultiHeadAttnForward<
    T0: types::CudaAsPtr,
    T1: types::CudaAsPtr,
    T2: types::CudaAsPtr,
    T3: types::CudaAsPtr,
    T4: types::CudaAsPtr,
    T5: types::CudaAsPtr,
    T6: types::CudaAsPtr,
    T7: types::CudaAsPtr,
    T8: types::CudaAsMutPtr,
    T9: types::CudaAsPtr,
    T10: types::CudaAsMutPtr,
    T11: types::CudaAsMutPtr,
>(
    handle: cudnnHandle_t,
    attnDesc: cudnnAttnDescriptor_t,
    currIdx: ::core::ffi::c_int,
    loWinIdx: T0,
    hiWinIdx: T1,
    devSeqLengthsQO: T2,
    devSeqLengthsKV: T3,
    qDesc: cudnnSeqDataDescriptor_t,
    queries: T4,
    residuals: T5,
    kDesc: cudnnSeqDataDescriptor_t,
    keys: T6,
    vDesc: cudnnSeqDataDescriptor_t,
    values: T7,
    oDesc: cudnnSeqDataDescriptor_t,
    mut out: T8,
    weightSizeInBytes: usize,
    weights: T9,
    workSpaceSizeInBytes: usize,
    mut workSpace: T10,
    reserveSpaceSizeInBytes: usize,
    mut reserveSpace: T11,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnMultiHeadAttnForward(
            handle,
            attnDesc,
            currIdx,
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
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn cudnnAdvVersionCheck() -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnAdvVersionCheck() };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Computes RNN data gradients (backward pass with respect to inputs).\n\n# Arguments\n\n* `handle` [in]  -           cuDNN handle.\n* `rnnDesc` [in]  -          RNN descriptor.\n* `devSeqLengths` [in]  -    Per-batch sequence lengths (device memory).\n* `yDesc` [in]  -            Output data descriptor.\n* `y` [in]  -                Forward output data.\n* `dy` [in]  -               Output gradient data.\n* `xDesc` [in]  -            Input data descriptor.\n* `dx` [out]  -               Computed input gradient.\n* `hDesc` [in]  -            Hidden state descriptor.\n* `hx` [in]  -               Initial hidden state from forward pass.\n* `dhy` [in]  -              Hidden state gradient (from upstream).\n* `dhx` [out]  -              Computed initial hidden state gradient.\n* `cDesc` [in]  -            Cell state descriptor (LSTM only).\n* `cx` [in]  -               Initial cell state from forward pass.\n* `dcy` [in]  -              Cell state gradient (from upstream).\n* `dcx` [out]  -              Computed initial cell state gradient.\n* `weightSpaceSize` [in]  -  Weight space size.\n* `weightSpace` [in]  -      Weight space pointer.\n* `workSpaceSize` [in]  -    Workspace size.\n* `workSpace` [in,out]  -        Workspace pointer.\n* `reserveSpaceSize` [in]  - Reserve space size.\n* `reserveSpace` [in,out]  -     Reserve space (from forward training pass).\n@retval CUDNN_STATUS_SUCCESS  Backward data pass completed.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnRNNForward,`] cudnnRNNBackwardWeights_v8"]
pub unsafe fn cudnnRNNBackwardData_v8<
    T0: types::CudaAsPtr,
    T1: types::CudaAsPtr,
    T2: types::CudaAsPtr,
    T3: types::CudaAsMutPtr,
    T4: types::CudaAsPtr,
    T5: types::CudaAsPtr,
    T6: types::CudaAsMutPtr,
    T7: types::CudaAsPtr,
    T8: types::CudaAsPtr,
    T9: types::CudaAsMutPtr,
    T10: types::CudaAsPtr,
    T11: types::CudaAsMutPtr,
    T12: types::CudaAsMutPtr,
>(
    handle: cudnnHandle_t,
    rnnDesc: cudnnRNNDescriptor_t,
    devSeqLengths: T0,
    yDesc: cudnnRNNDataDescriptor_t,
    y: T1,
    dy: T2,
    xDesc: cudnnRNNDataDescriptor_t,
    mut dx: T3,
    hDesc: cudnnTensorDescriptor_t,
    hx: T4,
    dhy: T5,
    mut dhx: T6,
    cDesc: cudnnTensorDescriptor_t,
    cx: T7,
    dcy: T8,
    mut dcx: T9,
    weightSpaceSize: usize,
    weightSpace: T10,
    workSpaceSize: usize,
    mut workSpace: T11,
    reserveSpaceSize: usize,
    mut reserveSpace: T12,
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
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Computes RNN weight gradients (backward pass with respect to parameters).\n\n# Arguments\n\n* `handle` [in]  -           cuDNN handle.\n* `rnnDesc` [in]  -          RNN descriptor.\n* `addGrad` [in]  -          Accumulate (ADD) or overwrite (SET) gradients.\n* `devSeqLengths` [in]  -    Per-batch sequence lengths (device memory).\n* `xDesc` [in]  -            Input data descriptor.\n* `x` [in]  -                Input data.\n* `hDesc` [in]  -            Hidden state descriptor.\n* `hx` [in]  -               Initial hidden state.\n* `yDesc` [in]  -            Output data descriptor.\n* `y` [in]  -                Forward output data.\n* `weightSpaceSize` [in]  -  Weight space size.\n* `dweightSpace` [in,out]  -     Computed weight gradients.\n* `workSpaceSize` [in]  -    Workspace size.\n* `workSpace` [in,out]  -        Workspace pointer.\n* `reserveSpaceSize` [in]  - Reserve space size.\n* `reserveSpace` [in,out]  -     Reserve space (from forward training pass).\n@retval CUDNN_STATUS_SUCCESS  Weight gradients computed.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnRNNForward,`] cudnnRNNBackwardData_v8"]
pub unsafe fn cudnnRNNBackwardWeights_v8<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr, T6: types::CudaAsMutPtr>(
    handle: cudnnHandle_t,
    rnnDesc: cudnnRNNDescriptor_t,
    addGrad: cudnnWgradMode_t,
    devSeqLengths: T0,
    xDesc: cudnnRNNDataDescriptor_t,
    x: T1,
    hDesc: cudnnTensorDescriptor_t,
    hx: T2,
    yDesc: cudnnRNNDataDescriptor_t,
    y: T3,
    weightSpaceSize: usize,
    mut dweightSpace: T4,
    workSpaceSize: usize,
    mut workSpace: T5,
    reserveSpaceSize: usize,
    mut reserveSpace: T6,
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
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Computes multi-head attention data gradients.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API SDPA operations instead.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnMultiHeadAttnBackwardData<
    T0: types::CudaAsPtr,
    T1: types::CudaAsPtr,
    T2: types::CudaAsPtr,
    T3: types::CudaAsPtr,
    T4: types::CudaAsPtr,
    T5: types::CudaAsMutPtr,
    T6: types::CudaAsPtr,
    T7: types::CudaAsMutPtr,
    T8: types::CudaAsPtr,
    T9: types::CudaAsMutPtr,
    T10: types::CudaAsPtr,
    T11: types::CudaAsPtr,
    T12: types::CudaAsMutPtr,
    T13: types::CudaAsMutPtr,
>(
    handle: cudnnHandle_t,
    attnDesc: cudnnAttnDescriptor_t,
    loWinIdx: T0,
    hiWinIdx: T1,
    devSeqLengthsDQDO: T2,
    devSeqLengthsDKDV: T3,
    doDesc: cudnnSeqDataDescriptor_t,
    dout: T4,
    dqDesc: cudnnSeqDataDescriptor_t,
    mut dqueries: T5,
    queries: T6,
    dkDesc: cudnnSeqDataDescriptor_t,
    mut dkeys: T7,
    keys: T8,
    dvDesc: cudnnSeqDataDescriptor_t,
    mut dvalues: T9,
    values: T10,
    weightSizeInBytes: usize,
    weights: T11,
    workSpaceSizeInBytes: usize,
    mut workSpace: T12,
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
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Computes multi-head attention weight gradients.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API SDPA operations instead.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnMultiHeadAttnBackwardWeights<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsPtr, T5: types::CudaAsMutPtr, T6: types::CudaAsMutPtr, T7: types::CudaAsMutPtr>(
    handle: cudnnHandle_t,
    attnDesc: cudnnAttnDescriptor_t,
    addGrad: cudnnWgradMode_t,
    qDesc: cudnnSeqDataDescriptor_t,
    queries: T0,
    kDesc: cudnnSeqDataDescriptor_t,
    keys: T1,
    vDesc: cudnnSeqDataDescriptor_t,
    values: T2,
    doDesc: cudnnSeqDataDescriptor_t,
    dout: T3,
    weightSizeInBytes: usize,
    weights: T4,
    mut dweights: T5,
    workSpaceSizeInBytes: usize,
    mut workSpace: T6,
    reserveSpaceSizeInBytes: usize,
    mut reserveSpace: T7,
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
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Creates a CTC loss descriptor.\n\n# Arguments\n\n* `ctcLossDesc` [out]  -  Pointer to created descriptor.\n@retval CUDNN_STATUS_SUCCESS  Descriptor created.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnCreateCTCLossDescriptor() -> Result<cudnnCTCLossDescriptor_t, crate::sys::cudnnStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cudnnCTCLossDescriptor_t> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnCreateCTCLossDescriptor(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cudnnCTCLossDescriptor_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Configures a CTC loss descriptor with compute type.\n> **Deprecated** Since cuDNN 9.0.0. Use cudnnSetCTCLossDescriptor_v9 instead.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnSetCTCLossDescriptor(ctcLossDesc: cudnnCTCLossDescriptor_t, compType: cudnnDataType_t) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnSetCTCLossDescriptor(ctcLossDesc, compType) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Configures CTC loss with normalization mode.\n> **Deprecated** Since cuDNN 9.0.0. Use cudnnSetCTCLossDescriptor_v9 instead.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnSetCTCLossDescriptorEx(ctcLossDesc: cudnnCTCLossDescriptor_t, compType: cudnnDataType_t, normMode: cudnnLossNormalizationMode_t, gradMode: cudnnNanPropagation_t) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnSetCTCLossDescriptorEx(ctcLossDesc, compType, normMode, gradMode) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Configures CTC loss with normalization, gradient mode, and max label length.\n> **Deprecated** Since cuDNN 9.0.0. Use cudnnSetCTCLossDescriptor_v9 instead.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnSetCTCLossDescriptor_v8(ctcLossDesc: cudnnCTCLossDescriptor_t, compType: cudnnDataType_t, normMode: cudnnLossNormalizationMode_t, gradMode: cudnnNanPropagation_t, maxLabelLength: ::core::ffi::c_int) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnSetCTCLossDescriptor_v8(ctcLossDesc, compType, normMode, gradMode, maxLabelLength) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Configures CTC loss with normalization, CTC gradient mode, and max label length.\n\n# Arguments\n\n* `ctcLossDesc` [in,out]  -    CTC loss descriptor.\n* `compType` [in]  -       Compute data type.\n* `normMode` [in]  -       Loss normalization mode.\n* `ctcGradMode` [in]  -    Gradient mode for out-of-bounds samples.\n* `maxLabelLength` [in]  - Maximum label length.\n@retval CUDNN_STATUS_SUCCESS  Descriptor configured.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnSetCTCLossDescriptor_v9(ctcLossDesc: cudnnCTCLossDescriptor_t, compType: cudnnDataType_t, normMode: cudnnLossNormalizationMode_t, ctcGradMode: cudnnCTCGradMode_t, maxLabelLength: ::core::ffi::c_int) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnSetCTCLossDescriptor_v9(ctcLossDesc, compType, normMode, ctcGradMode, maxLabelLength) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Retrieves CTC loss compute type.\n> **Deprecated** Since cuDNN 9.0.0. Use cudnnGetCTCLossDescriptor_v9 instead.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnGetCTCLossDescriptor(ctcLossDesc: cudnnCTCLossDescriptor_t) -> Result<cudnnDataType_t, crate::sys::cudnnStatus_t> {
    let mut out_1: std::mem::MaybeUninit<cudnnDataType_t> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnGetCTCLossDescriptor(ctcLossDesc, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as cudnnDataType_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Retrieves CTC loss extended parameters.\n> **Deprecated** Since cuDNN 9.0.0. Use cudnnGetCTCLossDescriptor_v9 instead.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnGetCTCLossDescriptorEx(ctcLossDesc: cudnnCTCLossDescriptor_t) -> Result<(cudnnDataType_t, cudnnLossNormalizationMode_t, cudnnNanPropagation_t), crate::sys::cudnnStatus_t> {
    let mut out_1: std::mem::MaybeUninit<cudnnDataType_t> = std::mem::MaybeUninit::zeroed();
    let mut out_2: std::mem::MaybeUninit<cudnnLossNormalizationMode_t> = std::mem::MaybeUninit::zeroed();
    let mut out_3: std::mem::MaybeUninit<cudnnNanPropagation_t> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnGetCTCLossDescriptorEx(ctcLossDesc, out_1.as_mut_ptr() as *mut _, out_2.as_mut_ptr() as *mut _, out_3.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok((out_1.assume_init() as cudnnDataType_t, out_2.assume_init() as cudnnLossNormalizationMode_t, out_3.assume_init() as cudnnNanPropagation_t)) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Retrieves CTC loss v8 parameters.\n> **Deprecated** Since cuDNN 9.0.0. Use cudnnGetCTCLossDescriptor_v9 instead.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnGetCTCLossDescriptor_v8(ctcLossDesc: cudnnCTCLossDescriptor_t) -> Result<(cudnnDataType_t, cudnnLossNormalizationMode_t, cudnnNanPropagation_t, ::core::ffi::c_int), crate::sys::cudnnStatus_t> {
    let mut out_1: std::mem::MaybeUninit<cudnnDataType_t> = std::mem::MaybeUninit::zeroed();
    let mut out_2: std::mem::MaybeUninit<cudnnLossNormalizationMode_t> = std::mem::MaybeUninit::zeroed();
    let mut out_3: std::mem::MaybeUninit<cudnnNanPropagation_t> = std::mem::MaybeUninit::zeroed();
    let mut out_4: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnGetCTCLossDescriptor_v8(ctcLossDesc, out_1.as_mut_ptr() as *mut _, out_2.as_mut_ptr() as *mut _, out_3.as_mut_ptr() as *mut _, out_4.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok((out_1.assume_init() as cudnnDataType_t, out_2.assume_init() as cudnnLossNormalizationMode_t, out_3.assume_init() as cudnnNanPropagation_t, out_4.assume_init() as ::core::ffi::c_int)) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Retrieves CTC loss v9 parameters.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnGetCTCLossDescriptor_v9(ctcLossDesc: cudnnCTCLossDescriptor_t) -> Result<(cudnnDataType_t, cudnnLossNormalizationMode_t, cudnnCTCGradMode_t, ::core::ffi::c_int), crate::sys::cudnnStatus_t> {
    let mut out_1: std::mem::MaybeUninit<cudnnDataType_t> = std::mem::MaybeUninit::zeroed();
    let mut out_2: std::mem::MaybeUninit<cudnnLossNormalizationMode_t> = std::mem::MaybeUninit::zeroed();
    let mut out_3: std::mem::MaybeUninit<cudnnCTCGradMode_t> = std::mem::MaybeUninit::zeroed();
    let mut out_4: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnGetCTCLossDescriptor_v9(ctcLossDesc, out_1.as_mut_ptr() as *mut _, out_2.as_mut_ptr() as *mut _, out_3.as_mut_ptr() as *mut _, out_4.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok((out_1.assume_init() as cudnnDataType_t, out_2.assume_init() as cudnnLossNormalizationMode_t, out_3.assume_init() as cudnnCTCGradMode_t, out_4.assume_init() as ::core::ffi::c_int)) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Destroys a CTC loss descriptor.\n\n# Arguments\n\n* `ctcLossDesc` [in]  -  Descriptor to destroy.\n@retval CUDNN_STATUS_SUCCESS  Descriptor destroyed.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnDestroyCTCLossDescriptor(ctcLossDesc: cudnnCTCLossDescriptor_t) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnDestroyCTCLossDescriptor(ctcLossDesc) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Computes CTC loss and gradients given probabilities and labels.\nLabels and sequence lengths are in CPU memory. For GPU-memory variant, use cudnnCTCLoss_v8.\n\n# Arguments\n\n* `handle` [in]  -              cuDNN handle.\n* `probsDesc` [in]  -           Tensor descriptor for probabilities (T x N x A).\n* `probs` [in]  -               Probabilities after softmax (GPU memory).\n* `hostLabels` [in]  -           Labels (CPU memory).\n* `hostLabelLengths` [in]  -     Length of each label (CPU memory).\n* `hostInputLengths` [in]  -     Timing step lengths per batch (CPU memory).\n* `costs` [out]  -               CTC costs (GPU memory).\n* `gradientsDesc` [in]  -       Tensor descriptor for gradients (T x N x A).\n* `gradients` [out]  -           CTC gradients (GPU memory, NULL for costs only).\n* `algo` [in]  -                CTC loss algorithm.\n* `ctcLossDesc` [in]  -         CTC loss descriptor.\n* `workspace` [in]  -           Workspace (GPU memory).\n* `workSpaceSizeInBytes` [in]  - Workspace size.\n@retval CUDNN_STATUS_SUCCESS  CTC loss computed.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnCTCLoss_v8,`] cudnnGetCTCLossWorkspaceSize"]
pub unsafe fn cudnnCTCLoss<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr, T6: types::CudaAsMutPtr>(
    handle: cudnnHandle_t,
    probsDesc: cudnnTensorDescriptor_t,
    probs: T0,
    hostLabels: T1,
    hostLabelLengths: T2,
    hostInputLengths: T3,
    mut costs: T4,
    gradientsDesc: cudnnTensorDescriptor_t,
    mut gradients: T5,
    algo: cudnnCTCLossAlgo_t,
    ctcLossDesc: cudnnCTCLossDescriptor_t,
    mut workspace: T6,
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
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Computes CTC loss and gradients (v8, supports CUDA graphs with GPU memory labels).\nLabels and sequence lengths are in GPU memory (unlike cudnnCTCLoss which uses CPU memory).\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnCTCLoss,`] cudnnGetCTCLossWorkspaceSize_v8"]
pub unsafe fn cudnnCTCLoss_v8<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr, T6: types::CudaAsMutPtr>(
    handle: cudnnHandle_t,
    algo: cudnnCTCLossAlgo_t,
    ctcLossDesc: cudnnCTCLossDescriptor_t,
    probsDesc: cudnnTensorDescriptor_t,
    probs: T0,
    labels: T1,
    labelLengths: T2,
    inputLengths: T3,
    mut costs: T4,
    gradientsDesc: cudnnTensorDescriptor_t,
    mut gradients: T5,
    workSpaceSizeInBytes: usize,
    mut workspace: T6,
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
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Returns the GPU workspace size required for CTC loss computation.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnCTCLoss`]"]
pub unsafe fn cudnnGetCTCLossWorkspaceSize(
    handle: cudnnHandle_t,
    probsDesc: cudnnTensorDescriptor_t,
    gradientsDesc: cudnnTensorDescriptor_t,
    labels: *const ::core::ffi::c_int,
    labelLengths: *const ::core::ffi::c_int,
    inputLengths: *const ::core::ffi::c_int,
    algo: cudnnCTCLossAlgo_t,
    ctcLossDesc: cudnnCTCLossDescriptor_t,
) -> Result<usize, crate::sys::cudnnStatus_t> {
    let mut out_8: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnGetCTCLossWorkspaceSize(handle, probsDesc, gradientsDesc, labels, labelLengths, inputLengths, algo, ctcLossDesc, out_8.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_8.assume_init() as usize) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Returns the GPU workspace size required for CTC loss v8 computation.\n> **Since** cuDNN 9.0.0\n\n# See also\n\n> [`cudnnCTCLoss_v8`]"]
pub unsafe fn cudnnGetCTCLossWorkspaceSize_v8(handle: cudnnHandle_t, algo: cudnnCTCLossAlgo_t, ctcLossDesc: cudnnCTCLossDescriptor_t, probsDesc: cudnnTensorDescriptor_t, gradientsDesc: cudnnTensorDescriptor_t) -> Result<usize, crate::sys::cudnnStatus_t> {
    let mut out_5: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnGetCTCLossWorkspaceSize_v8(handle, algo, ctcLossDesc, probsDesc, gradientsDesc, out_5.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_5.assume_init() as usize) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Create an instance of convolution descriptor.\n\n# Arguments\n\n* `convDesc` [out]  - Pointer to receive the newly created convolution descriptor.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnCreateConvolutionDescriptor() -> Result<cudnnConvolutionDescriptor_t, crate::sys::cudnnStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cudnnConvolutionDescriptor_t> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnCreateConvolutionDescriptor(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cudnnConvolutionDescriptor_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Destroy an instance of convolution descriptor.\n\n# Arguments\n\n* `convDesc` [in]  - The convolution descriptor to destroy.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnDestroyConvolutionDescriptor(convDesc: cudnnConvolutionDescriptor_t) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnDestroyConvolutionDescriptor(convDesc) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Set the math type for a convolution descriptor.\n\n# Arguments\n\n* `convDesc` [in,out]  - The convolution descriptor.\n* `mathType` [in]  - The math type to set.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnSetConvolutionMathType(convDesc: cudnnConvolutionDescriptor_t, mathType: cudnnMathType_t) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnSetConvolutionMathType(convDesc, mathType) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Get the math type from a convolution descriptor.\n\n# Arguments\n\n* `convDesc` [in]  - The convolution descriptor.\n* `mathType` [out]  - Pointer to receive the math type.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnGetConvolutionMathType(convDesc: cudnnConvolutionDescriptor_t) -> Result<cudnnMathType_t, crate::sys::cudnnStatus_t> {
    let mut out_1: std::mem::MaybeUninit<cudnnMathType_t> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnGetConvolutionMathType(convDesc, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as cudnnMathType_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Set the group count for a convolution descriptor.\n\n# Arguments\n\n* `convDesc` [in,out]  -   The convolution descriptor.\n* `groupCount` [in]  - The number of groups for grouped convolution.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnSetConvolutionGroupCount(convDesc: cudnnConvolutionDescriptor_t, groupCount: ::core::ffi::c_int) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnSetConvolutionGroupCount(convDesc, groupCount) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Get the group count from a convolution descriptor.\n\n# Arguments\n\n* `convDesc` [in]  -   The convolution descriptor.\n* `groupCount` [out]  - Pointer to receive the group count.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnGetConvolutionGroupCount(convDesc: cudnnConvolutionDescriptor_t) -> Result<::core::ffi::c_int, crate::sys::cudnnStatus_t> {
    let mut out_1: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnGetConvolutionGroupCount(convDesc, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as ::core::ffi::c_int) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Set the reorder type for a convolution descriptor.\n\n# Arguments\n\n* `convDesc` [in,out]  -    The convolution descriptor.\n* `reorderType` [in]  - The reorder type to set.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnSetConvolutionReorderType(convDesc: cudnnConvolutionDescriptor_t, reorderType: cudnnReorderType_t) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnSetConvolutionReorderType(convDesc, reorderType) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Get the reorder type from a convolution descriptor.\n\n# Arguments\n\n* `convDesc` [in]  -    The convolution descriptor.\n* `reorderType` [out]  - Pointer to receive the reorder type.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnGetConvolutionReorderType(convDesc: cudnnConvolutionDescriptor_t) -> Result<cudnnReorderType_t, crate::sys::cudnnStatus_t> {
    let mut out_1: std::mem::MaybeUninit<cudnnReorderType_t> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnGetConvolutionReorderType(convDesc, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as cudnnReorderType_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Set a 2D convolution descriptor with padding, stride, dilation, mode, and compute type.\n\n# Arguments\n\n* `convDesc` [in,out]  -    The convolution descriptor to initialize.\n* `pad_h` [in]  -       Zero-padding height.\n* `pad_w` [in]  -       Zero-padding width.\n* `u` [in]  -           Vertical filter stride.\n* `v` [in]  -           Horizontal filter stride.\n* `dilation_h` [in]  -  Filter dilation in the vertical dimension.\n* `dilation_w` [in]  -  Filter dilation in the horizontal dimension.\n* `mode` [in]  -        Convolution mode (cross-correlation or convolution).\n* `computeType` [in]  - Data type for convolution computation.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnSetConvolution2dDescriptor(
    convDesc: cudnnConvolutionDescriptor_t,
    pad_h: ::core::ffi::c_int,
    pad_w: ::core::ffi::c_int,
    u: ::core::ffi::c_int,
    v: ::core::ffi::c_int,
    dilation_h: ::core::ffi::c_int,
    dilation_w: ::core::ffi::c_int,
    mode: cudnnConvolutionMode_t,
    computeType: cudnnDataType_t,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnSetConvolution2dDescriptor(convDesc, pad_h, pad_w, u, v, dilation_h, dilation_w, mode, computeType) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Get the parameters of a 2D convolution descriptor.\n\n# Arguments\n\n* `convDesc` [in]  -    The convolution descriptor to query.\n* `pad_h` [out]  -       Pointer to receive zero-padding height.\n* `pad_w` [out]  -       Pointer to receive zero-padding width.\n* `u` [out]  -           Pointer to receive vertical filter stride.\n* `v` [out]  -           Pointer to receive horizontal filter stride.\n* `dilation_h` [out]  -  Pointer to receive filter dilation in the vertical dimension.\n* `dilation_w` [out]  -  Pointer to receive filter dilation in the horizontal dimension.\n* `mode` [out]  -        Pointer to receive convolution mode.\n* `computeType` [out]  - Pointer to receive compute data type.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnGetConvolution2dDescriptor(convDesc: cudnnConvolutionDescriptor_t) -> Result<(::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int, cudnnConvolutionMode_t, cudnnDataType_t), crate::sys::cudnnStatus_t> {
    let mut out_1: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let mut out_2: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let mut out_3: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let mut out_4: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let mut out_5: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let mut out_6: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let mut out_7: std::mem::MaybeUninit<cudnnConvolutionMode_t> = std::mem::MaybeUninit::zeroed();
    let mut out_8: std::mem::MaybeUninit<cudnnDataType_t> = std::mem::MaybeUninit::zeroed();
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
                out_1.assume_init() as ::core::ffi::c_int,
                out_2.assume_init() as ::core::ffi::c_int,
                out_3.assume_init() as ::core::ffi::c_int,
                out_4.assume_init() as ::core::ffi::c_int,
                out_5.assume_init() as ::core::ffi::c_int,
                out_6.assume_init() as ::core::ffi::c_int,
                out_7.assume_init() as cudnnConvolutionMode_t,
                out_8.assume_init() as cudnnDataType_t,
            ))
        }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Set an N-dimensional convolution descriptor.\n\n# Arguments\n\n* `convDesc` [in,out]  -      The convolution descriptor to initialize.\n* `arrayLength` [in]  -   Number of dimensions (nbDims-2 size).\n* `padA` [in]  -          Array of zero-padding values per dimension.\n* `filterStrideA` [in]  - Array of filter strides per dimension.\n* `dilationA` [in]  -     Array of dilation values per dimension.\n* `mode` [in]  -          Convolution mode.\n* `computeType` [in]  -   Data type for convolution computation.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnSetConvolutionNdDescriptor<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr>(
    convDesc: cudnnConvolutionDescriptor_t,
    arrayLength: ::core::ffi::c_int,
    padA: T0,
    filterStrideA: T1,
    dilationA: T2,
    mode: cudnnConvolutionMode_t,
    computeType: cudnnDataType_t,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnSetConvolutionNdDescriptor(convDesc, arrayLength, padA.as_const_ptr() as *const _, filterStrideA.as_const_ptr() as *const _, dilationA.as_const_ptr() as *const _, mode, computeType) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Get the parameters of an N-dimensional convolution descriptor.\n\n# Arguments\n\n* `convDesc` [in]  -             The convolution descriptor to query.\n* `arrayLengthRequested` [in]  - Maximum number of dimensions to retrieve.\n* `arrayLength` [out]  -          Pointer to receive the actual number of dimensions.\n* `padA` [out]  -                 Array to receive zero-padding values.\n* `strideA` [out]  -              Array to receive stride values.\n* `dilationA` [out]  -            Array to receive dilation values.\n* `mode` [out]  -                 Pointer to receive convolution mode.\n* `computeType` [out]  -          Pointer to receive compute data type.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnGetConvolutionNdDescriptor(convDesc: cudnnConvolutionDescriptor_t, arrayLengthRequested: ::core::ffi::c_int) -> Result<(::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int, cudnnConvolutionMode_t, cudnnDataType_t), crate::sys::cudnnStatus_t> {
    let mut out_2: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let mut out_3: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let mut out_4: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let mut out_5: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let mut out_6: std::mem::MaybeUninit<cudnnConvolutionMode_t> = std::mem::MaybeUninit::zeroed();
    let mut out_7: std::mem::MaybeUninit<cudnnDataType_t> = std::mem::MaybeUninit::zeroed();
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
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe {
            Ok((
                out_2.assume_init() as ::core::ffi::c_int,
                out_3.assume_init() as ::core::ffi::c_int,
                out_4.assume_init() as ::core::ffi::c_int,
                out_5.assume_init() as ::core::ffi::c_int,
                out_6.assume_init() as cudnnConvolutionMode_t,
                out_7.assume_init() as cudnnDataType_t,
            ))
        }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Compute the output dimensions of a 2D convolution.\n\n# Arguments\n\n* `convDesc` [in]  -        The convolution descriptor.\n* `inputTensorDesc` [in]  - Descriptor for the input tensor.\n* `filterDesc` [in]  -      Descriptor for the filter.\n* `n` [out]  -               Pointer to receive the output batch size.\n* `c` [out]  -               Pointer to receive the output channels.\n* `h` [out]  -               Pointer to receive the output height.\n* `w` [out]  -               Pointer to receive the output width.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnGetConvolution2dForwardOutputDim(convDesc: cudnnConvolutionDescriptor_t, inputTensorDesc: cudnnTensorDescriptor_t, filterDesc: cudnnFilterDescriptor_t) -> Result<(::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int), crate::sys::cudnnStatus_t> {
    let mut out_3: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let mut out_4: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let mut out_5: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let mut out_6: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnGetConvolution2dForwardOutputDim(convDesc, inputTensorDesc, filterDesc, out_3.as_mut_ptr() as *mut _, out_4.as_mut_ptr() as *mut _, out_5.as_mut_ptr() as *mut _, out_6.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok((out_3.assume_init() as ::core::ffi::c_int, out_4.assume_init() as ::core::ffi::c_int, out_5.assume_init() as ::core::ffi::c_int, out_6.assume_init() as ::core::ffi::c_int)) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Compute the output dimensions of an N-dimensional convolution.\n\n# Arguments\n\n* `convDesc` [in]  -         The convolution descriptor.\n* `inputTensorDesc` [in]  -  Descriptor for the input tensor.\n* `filterDesc` [in]  -       Descriptor for the filter.\n* `nbDims` [in]  -           Number of dimensions.\n* `tensorOuputDimA` [out]  -  Array to receive the output dimensions.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnGetConvolutionNdForwardOutputDim(convDesc: cudnnConvolutionDescriptor_t, inputTensorDesc: cudnnTensorDescriptor_t, filterDesc: cudnnFilterDescriptor_t, nbDims: ::core::ffi::c_int) -> Result<::core::ffi::c_int, crate::sys::cudnnStatus_t> {
    let mut out_4: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnGetConvolutionNdForwardOutputDim(convDesc, inputTensorDesc, filterDesc, nbDims, out_4.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_4.assume_init() as ::core::ffi::c_int) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Get the maximum number of forward convolution algorithms available.\n\n# Arguments\n\n* `handle` [in]  - The cuDNN handle.\n* `count` [out]  -  Pointer to receive the maximum algorithm count.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnGetConvolutionForwardAlgorithmMaxCount(handle: cudnnHandle_t) -> Result<::core::ffi::c_int, crate::sys::cudnnStatus_t> {
    let mut out_1: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnGetConvolutionForwardAlgorithmMaxCount(handle, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as ::core::ffi::c_int) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Get forward convolution algorithm recommendations without executing them.\nReturns a list of algorithms sorted by expected performance. Does not\nrequire a workspace or run actual convolutions.\n\n# Arguments\n\n* `handle` [in]  -              The cuDNN handle.\n* `srcDesc` [in]  -             Descriptor for the input tensor.\n* `filterDesc` [in]  -          Descriptor for the filter.\n* `convDesc` [in]  -            The convolution descriptor.\n* `destDesc` [in]  -            Descriptor for the output tensor.\n* `requestedAlgoCount` [in]  -  Maximum number of algorithms to return.\n* `returnedAlgoCount` [out]  -   Pointer to receive the actual number returned.\n* `perfResults` [out]  -         Array to receive the algorithm performance results.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnGetConvolutionForwardAlgorithm_v7(
    handle: cudnnHandle_t,
    srcDesc: cudnnTensorDescriptor_t,
    filterDesc: cudnnFilterDescriptor_t,
    convDesc: cudnnConvolutionDescriptor_t,
    destDesc: cudnnTensorDescriptor_t,
    requestedAlgoCount: ::core::ffi::c_int,
) -> Result<(::core::ffi::c_int, cudnnConvolutionFwdAlgoPerf_t), crate::sys::cudnnStatus_t> {
    let mut out_6: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let mut out_7: std::mem::MaybeUninit<cudnnConvolutionFwdAlgoPerf_t> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnGetConvolutionForwardAlgorithm_v7(handle, srcDesc, filterDesc, convDesc, destDesc, requestedAlgoCount, out_6.as_mut_ptr() as *mut _, out_7.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok((out_6.assume_init() as ::core::ffi::c_int, out_7.assume_init() as cudnnConvolutionFwdAlgoPerf_t)) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Find the best forward convolution algorithm by running benchmarks.\nExecutes all applicable algorithms and returns performance results\nsorted by execution time. Does not require user-allocated workspace.\n\n# Arguments\n\n* `handle` [in]  -              The cuDNN handle.\n* `xDesc` [in]  -               Descriptor for the input tensor.\n* `wDesc` [in]  -               Descriptor for the filter.\n* `convDesc` [in]  -            The convolution descriptor.\n* `yDesc` [in]  -               Descriptor for the output tensor.\n* `requestedAlgoCount` [in]  -  Maximum number of algorithms to test.\n* `returnedAlgoCount` [out]  -   Pointer to receive the actual number returned.\n* `perfResults` [out]  -         Array to receive the algorithm performance results.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnFindConvolutionForwardAlgorithm<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr>(
    handle: cudnnHandle_t,
    xDesc: cudnnTensorDescriptor_t,
    wDesc: cudnnFilterDescriptor_t,
    convDesc: cudnnConvolutionDescriptor_t,
    yDesc: cudnnTensorDescriptor_t,
    requestedAlgoCount: ::core::ffi::c_int,
    mut returnedAlgoCount: T0,
    mut perfResults: T1,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnFindConvolutionForwardAlgorithm(handle, xDesc, wDesc, convDesc, yDesc, requestedAlgoCount, returnedAlgoCount.as_mut_ptr() as *mut _, perfResults.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Find the best forward convolution algorithm by running benchmarks with user-provided buffers.\nSimilar to cudnnFindConvolutionForwardAlgorithm but uses caller-provided\ndata buffers and workspace.\n\n# Arguments\n\n* `handle` [in]  -                The cuDNN handle.\n* `xDesc` [in]  -                 Descriptor for the input tensor.\n* `x` [in]  -                     Pointer to input data in device memory.\n* `wDesc` [in]  -                 Descriptor for the filter.\n* `w` [in]  -                     Pointer to filter data in device memory.\n* `convDesc` [in]  -              The convolution descriptor.\n* `yDesc` [in]  -                 Descriptor for the output tensor.\n* `y` [out]  -                     Pointer to output data in device memory.\n* `requestedAlgoCount` [in]  -    Maximum number of algorithms to test.\n* `returnedAlgoCount` [out]  -     Pointer to receive the actual number returned.\n* `perfResults` [out]  -           Array to receive the algorithm performance results.\n* `workSpace` [in]  -             Pointer to workspace in device memory.\n* `workSpaceSizeInBytes` [in]  -  Size of the workspace in bytes.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnFindConvolutionForwardAlgorithmEx<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cudnnHandle_t,
    xDesc: cudnnTensorDescriptor_t,
    x: T0,
    wDesc: cudnnFilterDescriptor_t,
    w: T1,
    convDesc: cudnnConvolutionDescriptor_t,
    yDesc: cudnnTensorDescriptor_t,
    mut y: T2,
    requestedAlgoCount: ::core::ffi::c_int,
    mut returnedAlgoCount: T3,
    mut perfResults: T4,
    mut workSpace: T5,
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
            requestedAlgoCount,
            returnedAlgoCount.as_mut_ptr() as *mut _,
            perfResults.as_mut_ptr() as *mut _,
            workSpace.as_mut_ptr() as *mut _,
            workSpaceSizeInBytes,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Perform the Im2Col transform for convolution.\nRearranges image data into a column matrix suitable for matrix multiplication\nbased convolution.\n\n# Arguments\n\n* `handle` [in]  -   The cuDNN handle.\n* `xDesc` [in]  -    Descriptor for the input tensor.\n* `x` [in]  -        Pointer to input data in device memory.\n* `wDesc` [in]  -    Descriptor for the filter.\n* `convDesc` [in]  - The convolution descriptor.\n* `colBuffer` [out]  - Pointer to the output column buffer in device memory.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnIm2Col<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr>(handle: cudnnHandle_t, xDesc: cudnnTensorDescriptor_t, x: T0, wDesc: cudnnFilterDescriptor_t, convDesc: cudnnConvolutionDescriptor_t, mut colBuffer: T1) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnIm2Col(handle, xDesc, x.as_const_ptr() as *const _, wDesc, convDesc, colBuffer.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Reorder filter and bias data for optimized convolution execution.\nRearranges filter and optionally bias data into a layout optimized for\nthe specified reorder type.\n\n# Arguments\n\n* `handle` [in]  -              The cuDNN handle.\n* `filterDesc` [in]  -          Descriptor for the filter.\n* `reorderType` [in]  -         The reorder type to apply.\n* `filterData` [in]  -          Pointer to source filter data in device memory.\n* `reorderedFilterData` [out]  - Pointer to destination filter data in device memory.\n* `reorderBias` [in]  -         Non-zero to also reorder bias data.\n* `biasData` [in]  -            Pointer to source bias data in device memory (may be NULL).\n* `reorderedBiasData` [out]  -   Pointer to destination bias data in device memory (may be NULL).\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnReorderFilterAndBias<T0: types::CudaAsPtr, T1: types::CudaAsMutPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(
    handle: cudnnHandle_t,
    filterDesc: cudnnFilterDescriptor_t,
    reorderType: cudnnReorderType_t,
    filterData: T0,
    mut reorderedFilterData: T1,
    reorderBias: ::core::ffi::c_int,
    biasData: T2,
    mut reorderedBiasData: T3,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnReorderFilterAndBias(
            handle,
            filterDesc,
            reorderType,
            filterData.as_const_ptr() as *const _,
            reorderedFilterData.as_mut_ptr() as *mut _,
            reorderBias,
            biasData.as_const_ptr() as *const _,
            reorderedBiasData.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Get the minimum workspace size required for a forward convolution algorithm.\n\n# Arguments\n\n* `handle` [in]  -       The cuDNN handle.\n* `xDesc` [in]  -        Descriptor for the input tensor.\n* `wDesc` [in]  -        Descriptor for the filter.\n* `convDesc` [in]  -     The convolution descriptor.\n* `yDesc` [in]  -        Descriptor for the output tensor.\n* `algo` [in]  -         The forward convolution algorithm.\n* `sizeInBytes` [out]  -  Pointer to receive the required workspace size.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnGetConvolutionForwardWorkspaceSize(handle: cudnnHandle_t, xDesc: cudnnTensorDescriptor_t, wDesc: cudnnFilterDescriptor_t, convDesc: cudnnConvolutionDescriptor_t, yDesc: cudnnTensorDescriptor_t, algo: cudnnConvolutionFwdAlgo_t) -> Result<usize, crate::sys::cudnnStatus_t> {
    let mut out_6: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnGetConvolutionForwardWorkspaceSize(handle, xDesc, wDesc, convDesc, yDesc, algo, out_6.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_6.assume_init() as usize) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Execute a forward convolution.\nComputes: y = alpha * conv(x, w) + beta * y\n\n# Arguments\n\n* `handle` [in]  -                The cuDNN handle.\n* `alpha` [in]  -                 Pointer to scaling factor for the convolution result.\n* `xDesc` [in]  -                 Descriptor for the input tensor.\n* `x` [in]  -                     Pointer to input data in device memory.\n* `wDesc` [in]  -                 Descriptor for the filter.\n* `w` [in]  -                     Pointer to filter data in device memory.\n* `convDesc` [in]  -              The convolution descriptor.\n* `algo` [in]  -                  The forward convolution algorithm to use.\n* `workSpace` [in]  -             Pointer to workspace in device memory.\n* `workSpaceSizeInBytes` [in]  -  Size of the workspace in bytes.\n* `beta` [in]  -                  Pointer to scaling factor for the prior output.\n* `yDesc` [in]  -                 Descriptor for the output tensor.\n* `y` [in,out]  -                     Pointer to output data in device memory.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnConvolutionForward<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsPtr, T5: types::CudaAsMutPtr>(
    handle: cudnnHandle_t,
    alpha: T0,
    xDesc: cudnnTensorDescriptor_t,
    x: T1,
    wDesc: cudnnFilterDescriptor_t,
    w: T2,
    convDesc: cudnnConvolutionDescriptor_t,
    algo: cudnnConvolutionFwdAlgo_t,
    mut workSpace: T3,
    workSpaceSizeInBytes: usize,
    beta: T4,
    yDesc: cudnnTensorDescriptor_t,
    mut y: T5,
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
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Execute a fused convolution with bias and activation.\nComputes: y = Act( alpha1 * conv(x) + alpha2 * z + bias )\n\n# Arguments\n\n* `handle` [in]  -                The cuDNN handle.\n* `alpha1` [in]  -                Pointer to scaling factor for the convolution result.\n* `xDesc` [in]  -                 Descriptor for the input tensor.\n* `x` [in]  -                     Pointer to input data in device memory.\n* `wDesc` [in]  -                 Descriptor for the filter.\n* `w` [in]  -                     Pointer to filter data in device memory.\n* `convDesc` [in]  -              The convolution descriptor.\n* `algo` [in]  -                  The forward convolution algorithm to use.\n* `workSpace` [in]  -             Pointer to workspace in device memory.\n* `workSpaceSizeInBytes` [in]  -  Size of the workspace in bytes.\n* `alpha2` [in]  -                Pointer to scaling factor for the residual input z.\n* `zDesc` [in]  -                 Descriptor for the residual input tensor.\n* `z` [in]  -                     Pointer to residual data in device memory.\n* `biasDesc` [in]  -              Descriptor for the bias tensor.\n* `bias` [in]  -                  Pointer to bias data in device memory.\n* `activationDesc` [in]  -        Descriptor for the activation operation.\n* `yDesc` [in]  -                 Descriptor for the output tensor.\n* `y` [in,out]  -                     Pointer to output data in device memory.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnConvolutionBiasActivationForward<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsPtr, T5: types::CudaAsPtr, T6: types::CudaAsPtr, T7: types::CudaAsMutPtr>(
    handle: cudnnHandle_t,
    alpha1: T0,
    xDesc: cudnnTensorDescriptor_t,
    x: T1,
    wDesc: cudnnFilterDescriptor_t,
    w: T2,
    convDesc: cudnnConvolutionDescriptor_t,
    algo: cudnnConvolutionFwdAlgo_t,
    mut workSpace: T3,
    workSpaceSizeInBytes: usize,
    alpha2: T4,
    zDesc: cudnnTensorDescriptor_t,
    z: T5,
    biasDesc: cudnnTensorDescriptor_t,
    bias: T6,
    activationDesc: cudnnActivationDescriptor_t,
    yDesc: cudnnTensorDescriptor_t,
    mut y: T7,
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
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Get the maximum number of backward data convolution algorithms available.\n\n# Arguments\n\n* `handle` [in]  - The cuDNN handle.\n* `count` [out]  -  Pointer to receive the maximum algorithm count.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnGetConvolutionBackwardDataAlgorithmMaxCount(handle: cudnnHandle_t) -> Result<::core::ffi::c_int, crate::sys::cudnnStatus_t> {
    let mut out_1: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnGetConvolutionBackwardDataAlgorithmMaxCount(handle, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as ::core::ffi::c_int) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Find the best backward data convolution algorithm by running benchmarks.\n\n# Arguments\n\n* `handle` [in]  -              The cuDNN handle.\n* `wDesc` [in]  -               Descriptor for the filter.\n* `dyDesc` [in]  -              Descriptor for the gradient output tensor.\n* `convDesc` [in]  -            The convolution descriptor.\n* `dxDesc` [in]  -              Descriptor for the gradient input tensor.\n* `requestedAlgoCount` [in]  -  Maximum number of algorithms to test.\n* `returnedAlgoCount` [out]  -   Pointer to receive the actual number returned.\n* `perfResults` [out]  -         Array to receive the algorithm performance results.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnFindConvolutionBackwardDataAlgorithm<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr>(
    handle: cudnnHandle_t,
    wDesc: cudnnFilterDescriptor_t,
    dyDesc: cudnnTensorDescriptor_t,
    convDesc: cudnnConvolutionDescriptor_t,
    dxDesc: cudnnTensorDescriptor_t,
    requestedAlgoCount: ::core::ffi::c_int,
    mut returnedAlgoCount: T0,
    mut perfResults: T1,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnFindConvolutionBackwardDataAlgorithm(handle, wDesc, dyDesc, convDesc, dxDesc, requestedAlgoCount, returnedAlgoCount.as_mut_ptr() as *mut _, perfResults.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Find the best backward data convolution algorithm with user-provided buffers.\n\n# Arguments\n\n* `handle` [in]  -                The cuDNN handle.\n* `wDesc` [in]  -                 Descriptor for the filter.\n* `w` [in]  -                     Pointer to filter data in device memory.\n* `dyDesc` [in]  -                Descriptor for the gradient output tensor.\n* `dy` [in]  -                    Pointer to gradient output data in device memory.\n* `convDesc` [in]  -              The convolution descriptor.\n* `dxDesc` [in]  -                Descriptor for the gradient input tensor.\n* `dx` [out]  -                    Pointer to gradient input data in device memory.\n* `requestedAlgoCount` [in]  -    Maximum number of algorithms to test.\n* `returnedAlgoCount` [out]  -     Pointer to receive the actual number returned.\n* `perfResults` [out]  -           Array to receive the algorithm performance results.\n* `workSpace` [in]  -             Pointer to workspace in device memory.\n* `workSpaceSizeInBytes` [in]  -  Size of the workspace in bytes.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnFindConvolutionBackwardDataAlgorithmEx<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cudnnHandle_t,
    wDesc: cudnnFilterDescriptor_t,
    w: T0,
    dyDesc: cudnnTensorDescriptor_t,
    dy: T1,
    convDesc: cudnnConvolutionDescriptor_t,
    dxDesc: cudnnTensorDescriptor_t,
    mut dx: T2,
    requestedAlgoCount: ::core::ffi::c_int,
    mut returnedAlgoCount: T3,
    mut perfResults: T4,
    mut workSpace: T5,
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
            requestedAlgoCount,
            returnedAlgoCount.as_mut_ptr() as *mut _,
            perfResults.as_mut_ptr() as *mut _,
            workSpace.as_mut_ptr() as *mut _,
            workSpaceSizeInBytes,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Get backward data convolution algorithm recommendations without executing them.\n\n# Arguments\n\n* `handle` [in]  -              The cuDNN handle.\n* `filterDesc` [in]  -          Descriptor for the filter.\n* `diffDesc` [in]  -            Descriptor for the gradient output tensor.\n* `convDesc` [in]  -            The convolution descriptor.\n* `gradDesc` [in]  -            Descriptor for the gradient input tensor.\n* `requestedAlgoCount` [in]  -  Maximum number of algorithms to return.\n* `returnedAlgoCount` [out]  -   Pointer to receive the actual number returned.\n* `perfResults` [out]  -         Array to receive the algorithm performance results.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnGetConvolutionBackwardDataAlgorithm_v7(
    handle: cudnnHandle_t,
    filterDesc: cudnnFilterDescriptor_t,
    diffDesc: cudnnTensorDescriptor_t,
    convDesc: cudnnConvolutionDescriptor_t,
    gradDesc: cudnnTensorDescriptor_t,
    requestedAlgoCount: ::core::ffi::c_int,
) -> Result<(::core::ffi::c_int, cudnnConvolutionBwdDataAlgoPerf_t), crate::sys::cudnnStatus_t> {
    let mut out_6: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let mut out_7: std::mem::MaybeUninit<cudnnConvolutionBwdDataAlgoPerf_t> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnGetConvolutionBackwardDataAlgorithm_v7(handle, filterDesc, diffDesc, convDesc, gradDesc, requestedAlgoCount, out_6.as_mut_ptr() as *mut _, out_7.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok((out_6.assume_init() as ::core::ffi::c_int, out_7.assume_init() as cudnnConvolutionBwdDataAlgoPerf_t)) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Get the minimum workspace size required for a backward data convolution algorithm.\n\n# Arguments\n\n* `handle` [in]  -       The cuDNN handle.\n* `wDesc` [in]  -        Descriptor for the filter.\n* `dyDesc` [in]  -       Descriptor for the gradient output tensor.\n* `convDesc` [in]  -     The convolution descriptor.\n* `dxDesc` [in]  -       Descriptor for the gradient input tensor.\n* `algo` [in]  -         The backward data convolution algorithm.\n* `sizeInBytes` [out]  -  Pointer to receive the required workspace size.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnGetConvolutionBackwardDataWorkspaceSize(
    handle: cudnnHandle_t,
    wDesc: cudnnFilterDescriptor_t,
    dyDesc: cudnnTensorDescriptor_t,
    convDesc: cudnnConvolutionDescriptor_t,
    dxDesc: cudnnTensorDescriptor_t,
    algo: cudnnConvolutionBwdDataAlgo_t,
) -> Result<usize, crate::sys::cudnnStatus_t> {
    let mut out_6: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnGetConvolutionBackwardDataWorkspaceSize(handle, wDesc, dyDesc, convDesc, dxDesc, algo, out_6.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_6.assume_init() as usize) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Execute a backward data convolution (compute gradient with respect to input data).\nComputes: dx = alpha * dconv(w, dy) + beta * dx\n\n# Arguments\n\n* `handle` [in]  -                The cuDNN handle.\n* `alpha` [in]  -                 Pointer to scaling factor for the convolution result.\n* `wDesc` [in]  -                 Descriptor for the filter.\n* `w` [in]  -                     Pointer to filter data in device memory.\n* `dyDesc` [in]  -                Descriptor for the gradient output tensor.\n* `dy` [in]  -                    Pointer to gradient output data in device memory.\n* `convDesc` [in]  -              The convolution descriptor.\n* `algo` [in]  -                  The backward data convolution algorithm.\n* `workSpace` [in]  -             Pointer to workspace in device memory.\n* `workSpaceSizeInBytes` [in]  -  Size of the workspace in bytes.\n* `beta` [in]  -                  Pointer to scaling factor for the prior gradient input.\n* `dxDesc` [in]  -                Descriptor for the gradient input tensor.\n* `dx` [in,out]  -                    Pointer to gradient input data in device memory.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnConvolutionBackwardData<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsPtr, T5: types::CudaAsMutPtr>(
    handle: cudnnHandle_t,
    alpha: T0,
    wDesc: cudnnFilterDescriptor_t,
    w: T1,
    dyDesc: cudnnTensorDescriptor_t,
    dy: T2,
    convDesc: cudnnConvolutionDescriptor_t,
    algo: cudnnConvolutionBwdDataAlgo_t,
    mut workSpace: T3,
    workSpaceSizeInBytes: usize,
    beta: T4,
    dxDesc: cudnnTensorDescriptor_t,
    mut dx: T5,
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
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Calculate folding descriptors for backward data convolution (dgrad).\nComputes the folded descriptors needed for tensor transform operations\nused in backward data gradient computation.\n\n# Arguments\n\n* `handle` [in]  -                 The cuDNN handle.\n* `filterDesc` [in]  -             Descriptor for the filter.\n* `diffDesc` [in]  -               Descriptor for the gradient output tensor.\n* `convDesc` [in]  -               The convolution descriptor.\n* `gradDesc` [in]  -               Descriptor for the gradient input tensor.\n* `transformFormat` [in]  -         The tensor format for the transform.\n* `foldedFilterDesc` [out]  -       Descriptor for the folded filter.\n* `paddedDiffDesc` [out]  -         Descriptor for the padded gradient output.\n* `foldedConvDesc` [out]  -         Descriptor for the folded convolution.\n* `foldedGradDesc` [out]  -         Descriptor for the folded gradient input.\n* `filterFoldTransDesc` [out]  -    Transform descriptor for filter folding.\n* `diffPadTransDesc` [out]  -       Transform descriptor for diff padding.\n* `gradFoldTransDesc` [out]  -      Transform descriptor for gradient folding.\n* `gradUnfoldTransDesc` [out]  -    Transform descriptor for gradient unfolding.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
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
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Check the version of the cuDNN CNN library.\nVerifies that the CNN sub-library version matches the core cuDNN version.\n\n# Returns\n\ncudnnStatus_t indicating success or version mismatch.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnCnnVersionCheck() -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnCnnVersionCheck() };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Get the maximum number of backward filter convolution algorithms available.\n\n# Arguments\n\n* `handle` [in]  - The cuDNN handle.\n* `count` [out]  -  Pointer to receive the maximum algorithm count.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnGetConvolutionBackwardFilterAlgorithmMaxCount(handle: cudnnHandle_t) -> Result<::core::ffi::c_int, crate::sys::cudnnStatus_t> {
    let mut out_1: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnGetConvolutionBackwardFilterAlgorithmMaxCount(handle, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as ::core::ffi::c_int) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Find the best backward filter convolution algorithm by running benchmarks.\n\n# Arguments\n\n* `handle` [in]  -              The cuDNN handle.\n* `xDesc` [in]  -               Descriptor for the input tensor.\n* `dyDesc` [in]  -              Descriptor for the gradient output tensor.\n* `convDesc` [in]  -            The convolution descriptor.\n* `dwDesc` [in]  -              Descriptor for the filter gradient.\n* `requestedAlgoCount` [in]  -  Maximum number of algorithms to test.\n* `returnedAlgoCount` [out]  -   Pointer to receive the actual number returned.\n* `perfResults` [out]  -         Array to receive the algorithm performance results.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnFindConvolutionBackwardFilterAlgorithm<T0: types::CudaAsMutPtr, T1: types::CudaAsMutPtr>(
    handle: cudnnHandle_t,
    xDesc: cudnnTensorDescriptor_t,
    dyDesc: cudnnTensorDescriptor_t,
    convDesc: cudnnConvolutionDescriptor_t,
    dwDesc: cudnnFilterDescriptor_t,
    requestedAlgoCount: ::core::ffi::c_int,
    mut returnedAlgoCount: T0,
    mut perfResults: T1,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnFindConvolutionBackwardFilterAlgorithm(handle, xDesc, dyDesc, convDesc, dwDesc, requestedAlgoCount, returnedAlgoCount.as_mut_ptr() as *mut _, perfResults.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Find the best backward filter convolution algorithm with user-provided buffers.\n\n# Arguments\n\n* `handle` [in]  -                The cuDNN handle.\n* `xDesc` [in]  -                 Descriptor for the input tensor.\n* `x` [in]  -                     Pointer to input data in device memory.\n* `dyDesc` [in]  -                Descriptor for the gradient output tensor.\n* `y` [in]  -                     Pointer to gradient output data in device memory.\n* `convDesc` [in]  -              The convolution descriptor.\n* `dwDesc` [in]  -                Descriptor for the filter gradient.\n* `dw` [out]  -                    Pointer to filter gradient data in device memory.\n* `requestedAlgoCount` [in]  -    Maximum number of algorithms to test.\n* `returnedAlgoCount` [out]  -     Pointer to receive the actual number returned.\n* `perfResults` [out]  -           Array to receive the algorithm performance results.\n* `workSpace` [in]  -             Pointer to workspace in device memory.\n* `workSpaceSizeInBytes` [in]  -  Size of the workspace in bytes.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnFindConvolutionBackwardFilterAlgorithmEx<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsMutPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr>(
    handle: cudnnHandle_t,
    xDesc: cudnnTensorDescriptor_t,
    x: T0,
    dyDesc: cudnnTensorDescriptor_t,
    y: T1,
    convDesc: cudnnConvolutionDescriptor_t,
    dwDesc: cudnnFilterDescriptor_t,
    mut dw: T2,
    requestedAlgoCount: ::core::ffi::c_int,
    mut returnedAlgoCount: T3,
    mut perfResults: T4,
    mut workSpace: T5,
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
            requestedAlgoCount,
            returnedAlgoCount.as_mut_ptr() as *mut _,
            perfResults.as_mut_ptr() as *mut _,
            workSpace.as_mut_ptr() as *mut _,
            workSpaceSizeInBytes,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Get backward filter convolution algorithm recommendations without executing them.\n\n# Arguments\n\n* `handle` [in]  -              The cuDNN handle.\n* `srcDesc` [in]  -             Descriptor for the input tensor.\n* `diffDesc` [in]  -            Descriptor for the gradient output tensor.\n* `convDesc` [in]  -            The convolution descriptor.\n* `gradDesc` [in]  -            Descriptor for the filter gradient.\n* `requestedAlgoCount` [in]  -  Maximum number of algorithms to return.\n* `returnedAlgoCount` [out]  -   Pointer to receive the actual number returned.\n* `perfResults` [out]  -         Array to receive the algorithm performance results.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnGetConvolutionBackwardFilterAlgorithm_v7(
    handle: cudnnHandle_t,
    srcDesc: cudnnTensorDescriptor_t,
    diffDesc: cudnnTensorDescriptor_t,
    convDesc: cudnnConvolutionDescriptor_t,
    gradDesc: cudnnFilterDescriptor_t,
    requestedAlgoCount: ::core::ffi::c_int,
) -> Result<(::core::ffi::c_int, cudnnConvolutionBwdFilterAlgoPerf_t), crate::sys::cudnnStatus_t> {
    let mut out_6: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let mut out_7: std::mem::MaybeUninit<cudnnConvolutionBwdFilterAlgoPerf_t> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnGetConvolutionBackwardFilterAlgorithm_v7(handle, srcDesc, diffDesc, convDesc, gradDesc, requestedAlgoCount, out_6.as_mut_ptr() as *mut _, out_7.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok((out_6.assume_init() as ::core::ffi::c_int, out_7.assume_init() as cudnnConvolutionBwdFilterAlgoPerf_t)) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Get the minimum workspace size required for a backward filter convolution algorithm.\n\n# Arguments\n\n* `handle` [in]  -       The cuDNN handle.\n* `xDesc` [in]  -        Descriptor for the input tensor.\n* `dyDesc` [in]  -       Descriptor for the gradient output tensor.\n* `convDesc` [in]  -     The convolution descriptor.\n* `gradDesc` [in]  -     Descriptor for the filter gradient.\n* `algo` [in]  -         The backward filter convolution algorithm.\n* `sizeInBytes` [out]  -  Pointer to receive the required workspace size.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnGetConvolutionBackwardFilterWorkspaceSize(
    handle: cudnnHandle_t,
    xDesc: cudnnTensorDescriptor_t,
    dyDesc: cudnnTensorDescriptor_t,
    convDesc: cudnnConvolutionDescriptor_t,
    gradDesc: cudnnFilterDescriptor_t,
    algo: cudnnConvolutionBwdFilterAlgo_t,
) -> Result<usize, crate::sys::cudnnStatus_t> {
    let mut out_6: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnGetConvolutionBackwardFilterWorkspaceSize(handle, xDesc, dyDesc, convDesc, gradDesc, algo, out_6.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_6.assume_init() as usize) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Execute a backward filter convolution (compute gradient with respect to filter weights).\nComputes: dw = alpha * dconv(x, dy) + beta * dw\n\n# Arguments\n\n* `handle` [in]  -                The cuDNN handle.\n* `alpha` [in]  -                 Pointer to scaling factor for the convolution result.\n* `xDesc` [in]  -                 Descriptor for the input tensor.\n* `x` [in]  -                     Pointer to input data in device memory.\n* `dyDesc` [in]  -                Descriptor for the gradient output tensor.\n* `dy` [in]  -                    Pointer to gradient output data in device memory.\n* `convDesc` [in]  -              The convolution descriptor.\n* `algo` [in]  -                  The backward filter convolution algorithm.\n* `workSpace` [in]  -             Pointer to workspace in device memory.\n* `workSpaceSizeInBytes` [in]  -  Size of the workspace in bytes.\n* `beta` [in]  -                  Pointer to scaling factor for the prior filter gradient.\n* `dwDesc` [in]  -                Descriptor for the filter gradient.\n* `dw` [in,out]  -                    Pointer to filter gradient data in device memory.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnConvolutionBackwardFilter<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr, T4: types::CudaAsPtr, T5: types::CudaAsMutPtr>(
    handle: cudnnHandle_t,
    alpha: T0,
    xDesc: cudnnTensorDescriptor_t,
    x: T1,
    dyDesc: cudnnTensorDescriptor_t,
    dy: T2,
    convDesc: cudnnConvolutionDescriptor_t,
    algo: cudnnConvolutionBwdFilterAlgo_t,
    mut workSpace: T3,
    workSpaceSizeInBytes: usize,
    beta: T4,
    dwDesc: cudnnFilterDescriptor_t,
    mut dw: T5,
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
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Compute the bias gradient for batch convolution.\nComputes: db = alpha * sum(dy) + beta * db, where the sum is over spatial dimensions and batch.\n\n# Arguments\n\n* `handle` [in]  - The cuDNN handle.\n* `alpha` [in]  -  Pointer to scaling factor for the bias gradient result.\n* `dyDesc` [in]  - Descriptor for the gradient output tensor.\n* `dy` [in]  -     Pointer to gradient output data in device memory.\n* `beta` [in]  -   Pointer to scaling factor for the prior bias gradient.\n* `dbDesc` [in]  - Descriptor for the bias gradient tensor.\n* `db` [in,out]  -     Pointer to bias gradient data in device memory.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnConvolutionBackwardBias<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(
    handle: cudnnHandle_t,
    alpha: T0,
    dyDesc: cudnnTensorDescriptor_t,
    dy: T1,
    beta: T2,
    dbDesc: cudnnTensorDescriptor_t,
    mut db: T3,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnConvolutionBackwardBias(handle, alpha.as_const_ptr() as *const _, dyDesc, dy.as_const_ptr() as *const _, beta.as_const_ptr() as *const _, dbDesc, db.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Create a fused operations constant parameter pack.\n\n# Arguments\n\n* `constPack` [out]  - Pointer to receive the newly created constant parameter pack.\n* `ops` [in]  -       The fused operation type.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnCreateFusedOpsConstParamPack(ops: cudnnFusedOps_t) -> Result<cudnnFusedOpsConstParamPack_t, crate::sys::cudnnStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cudnnFusedOpsConstParamPack_t> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnCreateFusedOpsConstParamPack(out_0.as_mut_ptr() as *mut _, ops) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cudnnFusedOpsConstParamPack_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Destroy a fused operations constant parameter pack.\n\n# Arguments\n\n* `constPack` [in]  - The constant parameter pack to destroy.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnDestroyFusedOpsConstParamPack(constPack: cudnnFusedOpsConstParamPack_t) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnDestroyFusedOpsConstParamPack(constPack) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Set an attribute on a fused operations constant parameter pack.\n\n# Arguments\n\n* `constPack` [in,out]  -  The constant parameter pack.\n* `paramLabel` [in]  - The label identifying which parameter to set.\n* `param` [in]  -      Pointer to the parameter value.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnSetFusedOpsConstParamPackAttribute<T0: types::CudaAsPtr>(constPack: cudnnFusedOpsConstParamPack_t, paramLabel: cudnnFusedOpsConstParamLabel_t, param: T0) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnSetFusedOpsConstParamPackAttribute(constPack, paramLabel, param.as_const_ptr() as *const _) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Get an attribute from a fused operations constant parameter pack.\n\n# Arguments\n\n* `constPack` [in]  -  The constant parameter pack.\n* `paramLabel` [in]  - The label identifying which parameter to get.\n* `param` [out]  -      Pointer to receive the parameter value.\n* `isNULL` [out]  -     Pointer to receive whether the parameter is NULL.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnGetFusedOpsConstParamPackAttribute(constPack: cudnnFusedOpsConstParamPack_t, paramLabel: cudnnFusedOpsConstParamLabel_t, param: *mut ::core::ffi::c_void) -> Result<::core::ffi::c_int, crate::sys::cudnnStatus_t> {
    let mut out_3: std::mem::MaybeUninit<::core::ffi::c_int> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnGetFusedOpsConstParamPackAttribute(constPack, paramLabel, param, out_3.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_3.assume_init() as ::core::ffi::c_int) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Create a fused operations variant parameter pack.\n\n# Arguments\n\n* `varPack` [out]  - Pointer to receive the newly created variant parameter pack.\n* `ops` [in]  -     The fused operation type.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnCreateFusedOpsVariantParamPack(ops: cudnnFusedOps_t) -> Result<cudnnFusedOpsVariantParamPack_t, crate::sys::cudnnStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cudnnFusedOpsVariantParamPack_t> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnCreateFusedOpsVariantParamPack(out_0.as_mut_ptr() as *mut _, ops) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cudnnFusedOpsVariantParamPack_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Destroy a fused operations variant parameter pack.\n\n# Arguments\n\n* `varPack` [in]  - The variant parameter pack to destroy.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnDestroyFusedOpsVariantParamPack(varPack: cudnnFusedOpsVariantParamPack_t) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnDestroyFusedOpsVariantParamPack(varPack) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Set an attribute on a fused operations variant parameter pack.\n\n# Arguments\n\n* `varPack` [in,out]  -    The variant parameter pack.\n* `paramLabel` [in]  - The label identifying which parameter to set.\n* `ptr` [in]  -        Pointer to the parameter value.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnSetFusedOpsVariantParamPackAttribute<T0: types::CudaAsMutPtr>(varPack: cudnnFusedOpsVariantParamPack_t, paramLabel: cudnnFusedOpsVariantParamLabel_t, mut ptr: T0) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnSetFusedOpsVariantParamPackAttribute(varPack, paramLabel, ptr.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Get an attribute from a fused operations variant parameter pack.\n\n# Arguments\n\n* `varPack` [in]  -    The variant parameter pack.\n* `paramLabel` [in]  - The label identifying which parameter to get.\n* `ptr` [out]  -        Pointer to receive the parameter value.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnGetFusedOpsVariantParamPackAttribute<T0: types::CudaAsMutPtr>(varPack: cudnnFusedOpsVariantParamPack_t, paramLabel: cudnnFusedOpsVariantParamLabel_t, mut ptr: T0) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnGetFusedOpsVariantParamPackAttribute(varPack, paramLabel, ptr.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Create a fused operations execution plan.\n\n# Arguments\n\n* `plan` [out]  - Pointer to receive the newly created fused operations plan.\n* `ops` [in]  -  The fused operation type.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnCreateFusedOpsPlan(ops: cudnnFusedOps_t) -> Result<cudnnFusedOpsPlan_t, crate::sys::cudnnStatus_t> {
    let mut out_0: std::mem::MaybeUninit<cudnnFusedOpsPlan_t> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::cudnnCreateFusedOpsPlan(out_0.as_mut_ptr() as *mut _, ops) };
    if status as usize == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as cudnnFusedOpsPlan_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Destroy a fused operations execution plan.\n\n# Arguments\n\n* `plan` [in]  - The fused operations plan to destroy.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnDestroyFusedOpsPlan(plan: cudnnFusedOpsPlan_t) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnDestroyFusedOpsPlan(plan) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Build a fused operations execution plan from constant parameters.\nCompiles the plan and returns the required workspace size.\n\n# Arguments\n\n* `handle` [in]  -               The cuDNN handle.\n* `plan` [in,out]  -                 The fused operations plan to build.\n* `constPack` [in]  -            The constant parameter pack with descriptors.\n* `workspaceSizeInBytes` [out]  - Pointer to receive the required workspace size.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnMakeFusedOpsPlan<T0: types::CudaAsMutPtr>(handle: cudnnHandle_t, plan: cudnnFusedOpsPlan_t, constPack: cudnnFusedOpsConstParamPack_t, mut workspaceSizeInBytes: T0) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnMakeFusedOpsPlan(handle, plan, constPack, workspaceSizeInBytes.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Execute a fused operations plan.\n\n# Arguments\n\n* `handle` [in]  -  The cuDNN handle.\n* `plan` [in]  -    The fused operations plan to execute.\n* `varPack` [in]  - The variant parameter pack with data pointers and scalar values.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Deprecated** Since cuDNN 9.0.0. Use graph API instead.\n> **Since** cuDNN 9.0.0"]
pub unsafe fn cudnnFusedOpsExecute(handle: cudnnHandle_t, plan: cudnnFusedOpsPlan_t, varPack: cudnnFusedOpsVariantParamPack_t) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnFusedOpsExecute(handle, plan, varPack) };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Check the version of the cuDNN SubquadraticOps library.\nVerifies that the SubquadraticOps sub-library version matches the core cuDNN version.\n\n# Returns\n\ncudnnStatus_t indicating success or version mismatch.\n> **Since** cuDNN 9.22.0"]
pub unsafe fn cudnnSubquadraticOpsVersionCheck() -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe { crate::sys::cudnnSubquadraticOpsVersionCheck() };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Compute a causal (left-padded) depthwise 1D convolution with optional SiLU activation.\nComputes: y = Act( conv1d_causal(x, weight) + bias )\nCausal padding inserts (kernel_size - 1) zeros on the left and 0 on the right.\nThe convolution is depthwise: each channel is convolved independently with its\nown 1D filter.\n\n# Arguments\n\n* `stream` [in]  -      CUDA stream for kernel launch.\n* `x` [in]  -           Input tensor in device memory, layout (batch, dim, seq_len), contiguous.\n* `weight` [in]  -      Filter tensor in device memory, layout (dim, kernel_size), contiguous.\n* `bias` [in]  -        Bias tensor in device memory, layout (dim,), contiguous. Must be non-NULL.\n* `y` [out]  -           Output tensor in device memory, layout (batch, dim, seq_len), contiguous.\n* `batch` [in]  -       Batch size.\n* `dim` [in]  -         Number of channels (feature dimension).\n* `seqLen` [in]  -      Sequence length.\n* `kernelSize` [in]  -  Convolution kernel width. Supported: 2-8, 16, 32, 64, 128, 256.\n* `dataType` [in]  -    Element type for x, weight, bias, y. Supported: FLOAT, HALF, BFLOAT16.\n* `activation` [in]  -  Activation to apply after convolution + bias.\n> **Note** Not supported on Windows.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Since** cuDNN 9.22.0"]
pub unsafe fn cudnnCausalConv1dForward<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsMutPtr>(
    stream: cudaStream_t,
    x: T0,
    weight: T1,
    bias: T2,
    mut y: T3,
    batch: ::core::ffi::c_int,
    dim: ::core::ffi::c_int,
    seqLen: ::core::ffi::c_int,
    kernelSize: ::core::ffi::c_int,
    dataType: cudnnDataType_t,
    activation: cudnnCausalConv1dActivation_t,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnCausalConv1dForward(
            stream,
            x.as_const_ptr() as *const _,
            weight.as_const_ptr() as *const _,
            bias.as_const_ptr() as *const _,
            y.as_mut_ptr() as *mut _,
            batch,
            dim,
            seqLen,
            kernelSize,
            dataType,
            activation,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Compute gradients for causal depthwise 1D convolution.\nComputes:\n- dx      = dL/dx       (batch, dim, seq_len)\n- dweight = dL/dweight   (dim, kernel_size) — accumulated via atomicAdd\n- dbias   = dL/dbias     (dim,)             — accumulated via atomicAdd\nThe caller must zero-initialize dweight and dbias before calling this function\nif accumulation across multiple calls is not desired.\n\n# Arguments\n\n* `stream` [in]  -      CUDA stream for kernel launch.\n* `x` [in]  -           Original input tensor (needed for activation backward), device memory.\n* `weight` [in]  -      Original filter tensor in device memory.\n* `bias` [in]  -        Original bias tensor in device memory. Must be non-NULL.\n* `dy` [in]  -          Output gradient tensor in device memory, layout (batch, dim, seq_len).\n* `dx` [out]  -          Input gradient tensor in device memory, layout (batch, dim, seq_len).\n* `dweight` [in,out]  -     Filter gradient tensor (accumulated) in device memory, layout (dim, kernel_size).\n* `dbias` [in,out]  -       Bias gradient tensor (accumulated) in device memory, layout (dim,). Must be non-NULL.\n* `batch` [in]  -       Batch size.\n* `dim` [in]  -         Number of channels.\n* `seqLen` [in]  -      Sequence length.\n* `kernelSize` [in]  -  Convolution kernel width.\n* `dataType` [in]  -    Element type for x, weight, bias, dy, dx. Supported: FLOAT, HALF, BFLOAT16.\n* `dwDataType` [in]  -  Element type for dweight, dbias. Currently only FLOAT is supported.\n* `activation` [in]  -  Activation that was applied in forward (needed for backward recompute).\n> **Note** Not supported on Windows.\n\n# Returns\n\ncudnnStatus_t indicating success or failure.\n> **Since** cuDNN 9.22.0"]
pub unsafe fn cudnnCausalConv1dBackward<T0: types::CudaAsPtr, T1: types::CudaAsPtr, T2: types::CudaAsPtr, T3: types::CudaAsPtr, T4: types::CudaAsMutPtr, T5: types::CudaAsMutPtr, T6: types::CudaAsMutPtr>(
    stream: cudaStream_t,
    x: T0,
    weight: T1,
    bias: T2,
    dy: T3,
    mut dx: T4,
    mut dweight: T5,
    mut dbias: T6,
    batch: ::core::ffi::c_int,
    dim: ::core::ffi::c_int,
    seqLen: ::core::ffi::c_int,
    kernelSize: ::core::ffi::c_int,
    dataType: cudnnDataType_t,
    dwDataType: cudnnDataType_t,
    activation: cudnnCausalConv1dActivation_t,
) -> Result<(), crate::sys::cudnnStatus_t> {
    let status = unsafe {
        crate::sys::cudnnCausalConv1dBackward(
            stream,
            x.as_const_ptr() as *const _,
            weight.as_const_ptr() as *const _,
            bias.as_const_ptr() as *const _,
            dy.as_const_ptr() as *const _,
            dx.as_mut_ptr() as *mut _,
            dweight.as_mut_ptr() as *mut _,
            dbias.as_mut_ptr() as *mut _,
            batch,
            dim,
            seqLen,
            kernelSize,
            dataType,
            dwDataType,
            activation,
        )
    };
    if status == crate::sys::cudnnStatus_t::CUDNN_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
