pub use crate::sys::cudaError as CudaTargetStatus;
#[allow(unused_imports)]
use crate::sys::*;
#[allow(unused_imports)]
use crate::types;
#[cfg(feature = "runtime-link")]
impl crate::sys::dim3 {
    pub fn x(mut self, val: u32) -> Self {
        self.x = val as _;
        self
    }
    pub fn y(mut self, val: u32) -> Self {
        self.y = val as _;
        self
    }
    pub fn z(mut self, val: u32) -> Self {
        self.z = val as _;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaChannelFormatDesc {
    pub fn x(mut self, val: i32) -> Self {
        self.x = val as _;
        self
    }
    pub fn y(mut self, val: i32) -> Self {
        self.y = val as _;
        self
    }
    pub fn z(mut self, val: i32) -> Self {
        self.z = val as _;
        self
    }
    pub fn w(mut self, val: i32) -> Self {
        self.w = val as _;
        self
    }
    pub fn f(mut self, val: cudaChannelFormatKind) -> Self {
        self.f = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaArraySparseProperties {
    pub fn tileExtent(mut self, val: cudaArraySparseProperties__bindgen_ty_1) -> Self {
        self.tileExtent = val;
        self
    }
    pub fn miptailFirstLevel(mut self, val: u32) -> Self {
        self.miptailFirstLevel = val as _;
        self
    }
    pub fn miptailSize(mut self, val: u64) -> Self {
        self.miptailSize = val as _;
        self
    }
    pub fn flags(mut self, val: u32) -> Self {
        self.flags = val as _;
        self
    }
    pub fn reserved(mut self, val: [::std::os::raw::c_uint; 4usize]) -> Self {
        self.reserved = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaArraySparseProperties__bindgen_ty_1 {
    pub fn width(mut self, val: u32) -> Self {
        self.width = val as _;
        self
    }
    pub fn height(mut self, val: u32) -> Self {
        self.height = val as _;
        self
    }
    pub fn depth(mut self, val: u32) -> Self {
        self.depth = val as _;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaArrayMemoryRequirements {
    pub fn size(mut self, val: usize) -> Self {
        self.size = val;
        self
    }
    pub fn alignment(mut self, val: usize) -> Self {
        self.alignment = val;
        self
    }
    pub fn reserved(mut self, val: [::std::os::raw::c_uint; 4usize]) -> Self {
        self.reserved = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaPitchedPtr {
    pub fn ptr(mut self, val: *mut ::std::os::raw::c_void) -> Self {
        self.ptr = val;
        self
    }
    pub fn pitch(mut self, val: usize) -> Self {
        self.pitch = val;
        self
    }
    pub fn xsize(mut self, val: usize) -> Self {
        self.xsize = val;
        self
    }
    pub fn ysize(mut self, val: usize) -> Self {
        self.ysize = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaExtent {
    pub fn width(mut self, val: usize) -> Self {
        self.width = val;
        self
    }
    pub fn height(mut self, val: usize) -> Self {
        self.height = val;
        self
    }
    pub fn depth(mut self, val: usize) -> Self {
        self.depth = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaPos {
    pub fn x(mut self, val: usize) -> Self {
        self.x = val;
        self
    }
    pub fn y(mut self, val: usize) -> Self {
        self.y = val;
        self
    }
    pub fn z(mut self, val: usize) -> Self {
        self.z = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaMemcpy3DParms {
    pub fn srcArray(mut self, val: cudaArray_t) -> Self {
        self.srcArray = val;
        self
    }
    pub fn srcPos(mut self, val: cudaPos) -> Self {
        self.srcPos = val;
        self
    }
    pub fn srcPtr(mut self, val: cudaPitchedPtr) -> Self {
        self.srcPtr = val;
        self
    }
    pub fn dstArray(mut self, val: cudaArray_t) -> Self {
        self.dstArray = val;
        self
    }
    pub fn dstPos(mut self, val: cudaPos) -> Self {
        self.dstPos = val;
        self
    }
    pub fn dstPtr(mut self, val: cudaPitchedPtr) -> Self {
        self.dstPtr = val;
        self
    }
    pub fn extent(mut self, val: cudaExtent) -> Self {
        self.extent = val;
        self
    }
    pub fn kind(mut self, val: cudaMemcpyKind) -> Self {
        self.kind = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaMemcpyNodeParams {
    pub fn flags(mut self, val: i32) -> Self {
        self.flags = val as _;
        self
    }
    pub fn reserved(mut self, val: i32) -> Self {
        self.reserved = val as _;
        self
    }
    pub fn ctx(mut self, val: cudaExecutionContext_t) -> Self {
        self.ctx = val;
        self
    }
    pub fn copyParams(mut self, val: cudaMemcpy3DParms) -> Self {
        self.copyParams = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaMemcpy3DPeerParms {
    pub fn srcArray(mut self, val: cudaArray_t) -> Self {
        self.srcArray = val;
        self
    }
    pub fn srcPos(mut self, val: cudaPos) -> Self {
        self.srcPos = val;
        self
    }
    pub fn srcPtr(mut self, val: cudaPitchedPtr) -> Self {
        self.srcPtr = val;
        self
    }
    pub fn srcDevice(mut self, val: i32) -> Self {
        self.srcDevice = val as _;
        self
    }
    pub fn dstArray(mut self, val: cudaArray_t) -> Self {
        self.dstArray = val;
        self
    }
    pub fn dstPos(mut self, val: cudaPos) -> Self {
        self.dstPos = val;
        self
    }
    pub fn dstPtr(mut self, val: cudaPitchedPtr) -> Self {
        self.dstPtr = val;
        self
    }
    pub fn dstDevice(mut self, val: i32) -> Self {
        self.dstDevice = val as _;
        self
    }
    pub fn extent(mut self, val: cudaExtent) -> Self {
        self.extent = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaMemsetParams {
    pub fn dst(mut self, val: *mut ::std::os::raw::c_void) -> Self {
        self.dst = val;
        self
    }
    pub fn pitch(mut self, val: usize) -> Self {
        self.pitch = val;
        self
    }
    pub fn value(mut self, val: u32) -> Self {
        self.value = val as _;
        self
    }
    pub fn elementSize(mut self, val: u32) -> Self {
        self.elementSize = val as _;
        self
    }
    pub fn width(mut self, val: usize) -> Self {
        self.width = val;
        self
    }
    pub fn height(mut self, val: usize) -> Self {
        self.height = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaMemsetParamsV2 {
    pub fn dst(mut self, val: *mut ::std::os::raw::c_void) -> Self {
        self.dst = val;
        self
    }
    pub fn pitch(mut self, val: usize) -> Self {
        self.pitch = val;
        self
    }
    pub fn value(mut self, val: u32) -> Self {
        self.value = val as _;
        self
    }
    pub fn elementSize(mut self, val: u32) -> Self {
        self.elementSize = val as _;
        self
    }
    pub fn width(mut self, val: usize) -> Self {
        self.width = val;
        self
    }
    pub fn height(mut self, val: usize) -> Self {
        self.height = val;
        self
    }
    pub fn ctx(mut self, val: cudaExecutionContext_t) -> Self {
        self.ctx = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaAccessPolicyWindow {
    pub fn base_ptr(mut self, val: *mut ::std::os::raw::c_void) -> Self {
        self.base_ptr = val;
        self
    }
    pub fn num_bytes(mut self, val: usize) -> Self {
        self.num_bytes = val;
        self
    }
    pub fn hitRatio(mut self, val: f32) -> Self {
        self.hitRatio = val;
        self
    }
    pub fn hitProp(mut self, val: cudaAccessProperty) -> Self {
        self.hitProp = val;
        self
    }
    pub fn missProp(mut self, val: cudaAccessProperty) -> Self {
        self.missProp = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaHostNodeParams {
    pub fn fn_(mut self, val: cudaHostFn_t) -> Self {
        self.fn_ = val;
        self
    }
    pub fn userData(mut self, val: *mut ::std::os::raw::c_void) -> Self {
        self.userData = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaHostNodeParamsV2 {
    pub fn fn_(mut self, val: cudaHostFn_t) -> Self {
        self.fn_ = val;
        self
    }
    pub fn userData(mut self, val: *mut ::std::os::raw::c_void) -> Self {
        self.userData = val;
        self
    }
    pub fn syncMode(mut self, val: u32) -> Self {
        self.syncMode = val as _;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaResourceDesc {
    pub fn resType(mut self, val: cudaResourceType) -> Self {
        self.resType = val;
        self
    }
    pub fn res(mut self, val: cudaResourceDesc__bindgen_ty_1) -> Self {
        self.res = val;
        self
    }
    pub fn flags(mut self, val: u32) -> Self {
        self.flags = val as _;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaResourceDesc__bindgen_ty_1__bindgen_ty_1 {
    pub fn array(mut self, val: cudaArray_t) -> Self {
        self.array = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaResourceDesc__bindgen_ty_1__bindgen_ty_2 {
    pub fn mipmap(mut self, val: cudaMipmappedArray_t) -> Self {
        self.mipmap = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaResourceDesc__bindgen_ty_1__bindgen_ty_3 {
    pub fn devPtr(mut self, val: *mut ::std::os::raw::c_void) -> Self {
        self.devPtr = val;
        self
    }
    pub fn desc(mut self, val: cudaChannelFormatDesc) -> Self {
        self.desc = val;
        self
    }
    pub fn sizeInBytes(mut self, val: usize) -> Self {
        self.sizeInBytes = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaResourceDesc__bindgen_ty_1__bindgen_ty_4 {
    pub fn devPtr(mut self, val: *mut ::std::os::raw::c_void) -> Self {
        self.devPtr = val;
        self
    }
    pub fn desc(mut self, val: cudaChannelFormatDesc) -> Self {
        self.desc = val;
        self
    }
    pub fn width(mut self, val: usize) -> Self {
        self.width = val;
        self
    }
    pub fn height(mut self, val: usize) -> Self {
        self.height = val;
        self
    }
    pub fn pitchInBytes(mut self, val: usize) -> Self {
        self.pitchInBytes = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaResourceDesc__bindgen_ty_1__bindgen_ty_5 {
    pub fn reserved(mut self, val: [::std::os::raw::c_int; 32usize]) -> Self {
        self.reserved = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaResourceViewDesc {
    pub fn format(mut self, val: cudaResourceViewFormat) -> Self {
        self.format = val;
        self
    }
    pub fn width(mut self, val: usize) -> Self {
        self.width = val;
        self
    }
    pub fn height(mut self, val: usize) -> Self {
        self.height = val;
        self
    }
    pub fn depth(mut self, val: usize) -> Self {
        self.depth = val;
        self
    }
    pub fn firstMipmapLevel(mut self, val: u32) -> Self {
        self.firstMipmapLevel = val as _;
        self
    }
    pub fn lastMipmapLevel(mut self, val: u32) -> Self {
        self.lastMipmapLevel = val as _;
        self
    }
    pub fn firstLayer(mut self, val: u32) -> Self {
        self.firstLayer = val as _;
        self
    }
    pub fn lastLayer(mut self, val: u32) -> Self {
        self.lastLayer = val as _;
        self
    }
    pub fn reserved(mut self, val: [::std::os::raw::c_uint; 16usize]) -> Self {
        self.reserved = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaPointerAttributes {
    pub fn type_(mut self, val: cudaMemoryType) -> Self {
        self.type_ = val;
        self
    }
    pub fn device(mut self, val: i32) -> Self {
        self.device = val as _;
        self
    }
    pub fn devicePointer(mut self, val: *mut ::std::os::raw::c_void) -> Self {
        self.devicePointer = val;
        self
    }
    pub fn hostPointer(mut self, val: *mut ::std::os::raw::c_void) -> Self {
        self.hostPointer = val;
        self
    }
    pub fn reserved(mut self, val: [::std::os::raw::c_long; 8usize]) -> Self {
        self.reserved = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaFuncAttributes {
    pub fn sharedSizeBytes(mut self, val: usize) -> Self {
        self.sharedSizeBytes = val;
        self
    }
    pub fn constSizeBytes(mut self, val: usize) -> Self {
        self.constSizeBytes = val;
        self
    }
    pub fn localSizeBytes(mut self, val: usize) -> Self {
        self.localSizeBytes = val;
        self
    }
    pub fn maxThreadsPerBlock(mut self, val: i32) -> Self {
        self.maxThreadsPerBlock = val as _;
        self
    }
    pub fn numRegs(mut self, val: i32) -> Self {
        self.numRegs = val as _;
        self
    }
    pub fn ptxVersion(mut self, val: i32) -> Self {
        self.ptxVersion = val as _;
        self
    }
    pub fn binaryVersion(mut self, val: i32) -> Self {
        self.binaryVersion = val as _;
        self
    }
    pub fn cacheModeCA(mut self, val: i32) -> Self {
        self.cacheModeCA = val as _;
        self
    }
    pub fn maxDynamicSharedSizeBytes(mut self, val: i32) -> Self {
        self.maxDynamicSharedSizeBytes = val as _;
        self
    }
    pub fn preferredShmemCarveout(mut self, val: i32) -> Self {
        self.preferredShmemCarveout = val as _;
        self
    }
    pub fn clusterDimMustBeSet(mut self, val: i32) -> Self {
        self.clusterDimMustBeSet = val as _;
        self
    }
    pub fn requiredClusterWidth(mut self, val: i32) -> Self {
        self.requiredClusterWidth = val as _;
        self
    }
    pub fn requiredClusterHeight(mut self, val: i32) -> Self {
        self.requiredClusterHeight = val as _;
        self
    }
    pub fn requiredClusterDepth(mut self, val: i32) -> Self {
        self.requiredClusterDepth = val as _;
        self
    }
    pub fn clusterSchedulingPolicyPreference(mut self, val: i32) -> Self {
        self.clusterSchedulingPolicyPreference = val as _;
        self
    }
    pub fn nonPortableClusterSizeAllowed(mut self, val: i32) -> Self {
        self.nonPortableClusterSizeAllowed = val as _;
        self
    }
    pub fn reserved0(mut self, val: i32) -> Self {
        self.reserved0 = val as _;
        self
    }
    pub fn reserved(mut self, val: [::std::os::raw::c_int; 15usize]) -> Self {
        self.reserved = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaMemLocation {
    pub fn type_(mut self, val: cudaMemLocationType) -> Self {
        self.type_ = val;
        self
    }
    pub fn id(mut self, val: i32) -> Self {
        self.id = val as _;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaMemAccessDesc {
    pub fn location(mut self, val: cudaMemLocation) -> Self {
        self.location = val;
        self
    }
    pub fn flags(mut self, val: cudaMemAccessFlags) -> Self {
        self.flags = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaMemPoolProps {
    pub fn allocType(mut self, val: cudaMemAllocationType) -> Self {
        self.allocType = val;
        self
    }
    pub fn handleTypes(mut self, val: cudaMemAllocationHandleType) -> Self {
        self.handleTypes = val;
        self
    }
    pub fn location(mut self, val: cudaMemLocation) -> Self {
        self.location = val;
        self
    }
    pub fn win32SecurityAttributes(mut self, val: *mut ::std::os::raw::c_void) -> Self {
        self.win32SecurityAttributes = val;
        self
    }
    pub fn maxSize(mut self, val: usize) -> Self {
        self.maxSize = val;
        self
    }
    pub fn usage(mut self, val: u16) -> Self {
        self.usage = val as _;
        self
    }
    pub fn reserved(mut self, val: [::std::os::raw::c_uchar; 54usize]) -> Self {
        self.reserved = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaMemPoolPtrExportData {
    pub fn reserved(mut self, val: [::std::os::raw::c_uchar; 64usize]) -> Self {
        self.reserved = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaMemAllocNodeParams {
    pub fn poolProps(mut self, val: cudaMemPoolProps) -> Self {
        self.poolProps = val;
        self
    }
    pub fn accessDescs(mut self, val: *const cudaMemAccessDesc) -> Self {
        self.accessDescs = val;
        self
    }
    pub fn accessDescCount(mut self, val: usize) -> Self {
        self.accessDescCount = val;
        self
    }
    pub fn bytesize(mut self, val: usize) -> Self {
        self.bytesize = val;
        self
    }
    pub fn dptr(mut self, val: *mut ::std::os::raw::c_void) -> Self {
        self.dptr = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaMemAllocNodeParamsV2 {
    pub fn poolProps(mut self, val: cudaMemPoolProps) -> Self {
        self.poolProps = val;
        self
    }
    pub fn accessDescs(mut self, val: *const cudaMemAccessDesc) -> Self {
        self.accessDescs = val;
        self
    }
    pub fn accessDescCount(mut self, val: usize) -> Self {
        self.accessDescCount = val;
        self
    }
    pub fn bytesize(mut self, val: usize) -> Self {
        self.bytesize = val;
        self
    }
    pub fn dptr(mut self, val: *mut ::std::os::raw::c_void) -> Self {
        self.dptr = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaMemFreeNodeParams {
    pub fn dptr(mut self, val: *mut ::std::os::raw::c_void) -> Self {
        self.dptr = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaMemcpyAttributes {
    pub fn srcAccessOrder(mut self, val: cudaMemcpySrcAccessOrder) -> Self {
        self.srcAccessOrder = val;
        self
    }
    pub fn srcLocHint(mut self, val: cudaMemLocation) -> Self {
        self.srcLocHint = val;
        self
    }
    pub fn dstLocHint(mut self, val: cudaMemLocation) -> Self {
        self.dstLocHint = val;
        self
    }
    pub fn flags(mut self, val: u32) -> Self {
        self.flags = val as _;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaOffset3D {
    pub fn x(mut self, val: usize) -> Self {
        self.x = val;
        self
    }
    pub fn y(mut self, val: usize) -> Self {
        self.y = val;
        self
    }
    pub fn z(mut self, val: usize) -> Self {
        self.z = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaMemcpy3DOperand {
    pub fn type_(mut self, val: cudaMemcpy3DOperandType) -> Self {
        self.type_ = val;
        self
    }
    pub fn op(mut self, val: cudaMemcpy3DOperand__bindgen_ty_1) -> Self {
        self.op = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaMemcpy3DOperand__bindgen_ty_1__bindgen_ty_1 {
    pub fn ptr(mut self, val: *mut ::std::os::raw::c_void) -> Self {
        self.ptr = val;
        self
    }
    pub fn rowLength(mut self, val: usize) -> Self {
        self.rowLength = val;
        self
    }
    pub fn layerHeight(mut self, val: usize) -> Self {
        self.layerHeight = val;
        self
    }
    pub fn locHint(mut self, val: cudaMemLocation) -> Self {
        self.locHint = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaMemcpy3DOperand__bindgen_ty_1__bindgen_ty_2 {
    pub fn array(mut self, val: cudaArray_t) -> Self {
        self.array = val;
        self
    }
    pub fn offset(mut self, val: cudaOffset3D) -> Self {
        self.offset = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaMemcpy3DBatchOp {
    pub fn src(mut self, val: cudaMemcpy3DOperand) -> Self {
        self.src = val;
        self
    }
    pub fn dst(mut self, val: cudaMemcpy3DOperand) -> Self {
        self.dst = val;
        self
    }
    pub fn extent(mut self, val: cudaExtent) -> Self {
        self.extent = val;
        self
    }
    pub fn srcAccessOrder(mut self, val: cudaMemcpySrcAccessOrder) -> Self {
        self.srcAccessOrder = val;
        self
    }
    pub fn flags(mut self, val: u32) -> Self {
        self.flags = val as _;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::CUuuid_st {
    pub fn bytes(mut self, val: [::std::os::raw::c_char; 16usize]) -> Self {
        self.bytes = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaDeviceProp {
    pub fn name(mut self, val: [::std::os::raw::c_char; 256usize]) -> Self {
        self.name = val;
        self
    }
    pub fn uuid(mut self, val: cudaUUID_t) -> Self {
        self.uuid = val;
        self
    }
    pub fn luid(mut self, val: [::std::os::raw::c_char; 8usize]) -> Self {
        self.luid = val;
        self
    }
    pub fn luidDeviceNodeMask(mut self, val: u32) -> Self {
        self.luidDeviceNodeMask = val as _;
        self
    }
    pub fn totalGlobalMem(mut self, val: usize) -> Self {
        self.totalGlobalMem = val;
        self
    }
    pub fn sharedMemPerBlock(mut self, val: usize) -> Self {
        self.sharedMemPerBlock = val;
        self
    }
    pub fn regsPerBlock(mut self, val: i32) -> Self {
        self.regsPerBlock = val as _;
        self
    }
    pub fn warpSize(mut self, val: i32) -> Self {
        self.warpSize = val as _;
        self
    }
    pub fn memPitch(mut self, val: usize) -> Self {
        self.memPitch = val;
        self
    }
    pub fn maxThreadsPerBlock(mut self, val: i32) -> Self {
        self.maxThreadsPerBlock = val as _;
        self
    }
    pub fn maxThreadsDim(mut self, val: [::std::os::raw::c_int; 3usize]) -> Self {
        self.maxThreadsDim = val;
        self
    }
    pub fn maxGridSize(mut self, val: [::std::os::raw::c_int; 3usize]) -> Self {
        self.maxGridSize = val;
        self
    }
    pub fn totalConstMem(mut self, val: usize) -> Self {
        self.totalConstMem = val;
        self
    }
    pub fn major(mut self, val: i32) -> Self {
        self.major = val as _;
        self
    }
    pub fn minor(mut self, val: i32) -> Self {
        self.minor = val as _;
        self
    }
    pub fn textureAlignment(mut self, val: usize) -> Self {
        self.textureAlignment = val;
        self
    }
    pub fn texturePitchAlignment(mut self, val: usize) -> Self {
        self.texturePitchAlignment = val;
        self
    }
    pub fn multiProcessorCount(mut self, val: i32) -> Self {
        self.multiProcessorCount = val as _;
        self
    }
    pub fn integrated(mut self, val: i32) -> Self {
        self.integrated = val as _;
        self
    }
    pub fn canMapHostMemory(mut self, val: i32) -> Self {
        self.canMapHostMemory = val as _;
        self
    }
    pub fn maxTexture1D(mut self, val: i32) -> Self {
        self.maxTexture1D = val as _;
        self
    }
    pub fn maxTexture1DMipmap(mut self, val: i32) -> Self {
        self.maxTexture1DMipmap = val as _;
        self
    }
    pub fn maxTexture2D(mut self, val: [::std::os::raw::c_int; 2usize]) -> Self {
        self.maxTexture2D = val;
        self
    }
    pub fn maxTexture2DMipmap(mut self, val: [::std::os::raw::c_int; 2usize]) -> Self {
        self.maxTexture2DMipmap = val;
        self
    }
    pub fn maxTexture2DLinear(mut self, val: [::std::os::raw::c_int; 3usize]) -> Self {
        self.maxTexture2DLinear = val;
        self
    }
    pub fn maxTexture2DGather(mut self, val: [::std::os::raw::c_int; 2usize]) -> Self {
        self.maxTexture2DGather = val;
        self
    }
    pub fn maxTexture3D(mut self, val: [::std::os::raw::c_int; 3usize]) -> Self {
        self.maxTexture3D = val;
        self
    }
    pub fn maxTexture3DAlt(mut self, val: [::std::os::raw::c_int; 3usize]) -> Self {
        self.maxTexture3DAlt = val;
        self
    }
    pub fn maxTextureCubemap(mut self, val: i32) -> Self {
        self.maxTextureCubemap = val as _;
        self
    }
    pub fn maxTexture1DLayered(mut self, val: [::std::os::raw::c_int; 2usize]) -> Self {
        self.maxTexture1DLayered = val;
        self
    }
    pub fn maxTexture2DLayered(mut self, val: [::std::os::raw::c_int; 3usize]) -> Self {
        self.maxTexture2DLayered = val;
        self
    }
    pub fn maxTextureCubemapLayered(mut self, val: [::std::os::raw::c_int; 2usize]) -> Self {
        self.maxTextureCubemapLayered = val;
        self
    }
    pub fn maxSurface1D(mut self, val: i32) -> Self {
        self.maxSurface1D = val as _;
        self
    }
    pub fn maxSurface2D(mut self, val: [::std::os::raw::c_int; 2usize]) -> Self {
        self.maxSurface2D = val;
        self
    }
    pub fn maxSurface3D(mut self, val: [::std::os::raw::c_int; 3usize]) -> Self {
        self.maxSurface3D = val;
        self
    }
    pub fn maxSurface1DLayered(mut self, val: [::std::os::raw::c_int; 2usize]) -> Self {
        self.maxSurface1DLayered = val;
        self
    }
    pub fn maxSurface2DLayered(mut self, val: [::std::os::raw::c_int; 3usize]) -> Self {
        self.maxSurface2DLayered = val;
        self
    }
    pub fn maxSurfaceCubemap(mut self, val: i32) -> Self {
        self.maxSurfaceCubemap = val as _;
        self
    }
    pub fn maxSurfaceCubemapLayered(mut self, val: [::std::os::raw::c_int; 2usize]) -> Self {
        self.maxSurfaceCubemapLayered = val;
        self
    }
    pub fn surfaceAlignment(mut self, val: usize) -> Self {
        self.surfaceAlignment = val;
        self
    }
    pub fn concurrentKernels(mut self, val: i32) -> Self {
        self.concurrentKernels = val as _;
        self
    }
    pub fn ECCEnabled(mut self, val: i32) -> Self {
        self.ECCEnabled = val as _;
        self
    }
    pub fn pciBusID(mut self, val: i32) -> Self {
        self.pciBusID = val as _;
        self
    }
    pub fn pciDeviceID(mut self, val: i32) -> Self {
        self.pciDeviceID = val as _;
        self
    }
    pub fn pciDomainID(mut self, val: i32) -> Self {
        self.pciDomainID = val as _;
        self
    }
    pub fn tccDriver(mut self, val: i32) -> Self {
        self.tccDriver = val as _;
        self
    }
    pub fn asyncEngineCount(mut self, val: i32) -> Self {
        self.asyncEngineCount = val as _;
        self
    }
    pub fn unifiedAddressing(mut self, val: i32) -> Self {
        self.unifiedAddressing = val as _;
        self
    }
    pub fn memoryBusWidth(mut self, val: i32) -> Self {
        self.memoryBusWidth = val as _;
        self
    }
    pub fn l2CacheSize(mut self, val: i32) -> Self {
        self.l2CacheSize = val as _;
        self
    }
    pub fn persistingL2CacheMaxSize(mut self, val: i32) -> Self {
        self.persistingL2CacheMaxSize = val as _;
        self
    }
    pub fn maxThreadsPerMultiProcessor(mut self, val: i32) -> Self {
        self.maxThreadsPerMultiProcessor = val as _;
        self
    }
    pub fn streamPrioritiesSupported(mut self, val: i32) -> Self {
        self.streamPrioritiesSupported = val as _;
        self
    }
    pub fn globalL1CacheSupported(mut self, val: i32) -> Self {
        self.globalL1CacheSupported = val as _;
        self
    }
    pub fn localL1CacheSupported(mut self, val: i32) -> Self {
        self.localL1CacheSupported = val as _;
        self
    }
    pub fn sharedMemPerMultiprocessor(mut self, val: usize) -> Self {
        self.sharedMemPerMultiprocessor = val;
        self
    }
    pub fn regsPerMultiprocessor(mut self, val: i32) -> Self {
        self.regsPerMultiprocessor = val as _;
        self
    }
    pub fn managedMemory(mut self, val: i32) -> Self {
        self.managedMemory = val as _;
        self
    }
    pub fn isMultiGpuBoard(mut self, val: i32) -> Self {
        self.isMultiGpuBoard = val as _;
        self
    }
    pub fn multiGpuBoardGroupID(mut self, val: i32) -> Self {
        self.multiGpuBoardGroupID = val as _;
        self
    }
    pub fn hostNativeAtomicSupported(mut self, val: i32) -> Self {
        self.hostNativeAtomicSupported = val as _;
        self
    }
    pub fn pageableMemoryAccess(mut self, val: i32) -> Self {
        self.pageableMemoryAccess = val as _;
        self
    }
    pub fn concurrentManagedAccess(mut self, val: i32) -> Self {
        self.concurrentManagedAccess = val as _;
        self
    }
    pub fn computePreemptionSupported(mut self, val: i32) -> Self {
        self.computePreemptionSupported = val as _;
        self
    }
    pub fn canUseHostPointerForRegisteredMem(mut self, val: i32) -> Self {
        self.canUseHostPointerForRegisteredMem = val as _;
        self
    }
    pub fn cooperativeLaunch(mut self, val: i32) -> Self {
        self.cooperativeLaunch = val as _;
        self
    }
    pub fn sharedMemPerBlockOptin(mut self, val: usize) -> Self {
        self.sharedMemPerBlockOptin = val;
        self
    }
    pub fn pageableMemoryAccessUsesHostPageTables(mut self, val: i32) -> Self {
        self.pageableMemoryAccessUsesHostPageTables = val as _;
        self
    }
    pub fn directManagedMemAccessFromHost(mut self, val: i32) -> Self {
        self.directManagedMemAccessFromHost = val as _;
        self
    }
    pub fn maxBlocksPerMultiProcessor(mut self, val: i32) -> Self {
        self.maxBlocksPerMultiProcessor = val as _;
        self
    }
    pub fn accessPolicyMaxWindowSize(mut self, val: i32) -> Self {
        self.accessPolicyMaxWindowSize = val as _;
        self
    }
    pub fn reservedSharedMemPerBlock(mut self, val: usize) -> Self {
        self.reservedSharedMemPerBlock = val;
        self
    }
    pub fn hostRegisterSupported(mut self, val: i32) -> Self {
        self.hostRegisterSupported = val as _;
        self
    }
    pub fn sparseCudaArraySupported(mut self, val: i32) -> Self {
        self.sparseCudaArraySupported = val as _;
        self
    }
    pub fn hostRegisterReadOnlySupported(mut self, val: i32) -> Self {
        self.hostRegisterReadOnlySupported = val as _;
        self
    }
    pub fn timelineSemaphoreInteropSupported(mut self, val: i32) -> Self {
        self.timelineSemaphoreInteropSupported = val as _;
        self
    }
    pub fn memoryPoolsSupported(mut self, val: i32) -> Self {
        self.memoryPoolsSupported = val as _;
        self
    }
    pub fn gpuDirectRDMASupported(mut self, val: i32) -> Self {
        self.gpuDirectRDMASupported = val as _;
        self
    }
    pub fn gpuDirectRDMAFlushWritesOptions(mut self, val: u32) -> Self {
        self.gpuDirectRDMAFlushWritesOptions = val as _;
        self
    }
    pub fn gpuDirectRDMAWritesOrdering(mut self, val: i32) -> Self {
        self.gpuDirectRDMAWritesOrdering = val as _;
        self
    }
    pub fn memoryPoolSupportedHandleTypes(mut self, val: u32) -> Self {
        self.memoryPoolSupportedHandleTypes = val as _;
        self
    }
    pub fn deferredMappingCudaArraySupported(mut self, val: i32) -> Self {
        self.deferredMappingCudaArraySupported = val as _;
        self
    }
    pub fn ipcEventSupported(mut self, val: i32) -> Self {
        self.ipcEventSupported = val as _;
        self
    }
    pub fn clusterLaunch(mut self, val: i32) -> Self {
        self.clusterLaunch = val as _;
        self
    }
    pub fn unifiedFunctionPointers(mut self, val: i32) -> Self {
        self.unifiedFunctionPointers = val as _;
        self
    }
    pub fn deviceNumaConfig(mut self, val: i32) -> Self {
        self.deviceNumaConfig = val as _;
        self
    }
    pub fn deviceNumaId(mut self, val: i32) -> Self {
        self.deviceNumaId = val as _;
        self
    }
    pub fn mpsEnabled(mut self, val: i32) -> Self {
        self.mpsEnabled = val as _;
        self
    }
    pub fn hostNumaId(mut self, val: i32) -> Self {
        self.hostNumaId = val as _;
        self
    }
    pub fn gpuPciDeviceID(mut self, val: u32) -> Self {
        self.gpuPciDeviceID = val as _;
        self
    }
    pub fn gpuPciSubsystemID(mut self, val: u32) -> Self {
        self.gpuPciSubsystemID = val as _;
        self
    }
    pub fn hostNumaMultinodeIpcSupported(mut self, val: i32) -> Self {
        self.hostNumaMultinodeIpcSupported = val as _;
        self
    }
    pub fn reserved(mut self, val: [::std::os::raw::c_int; 56usize]) -> Self {
        self.reserved = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaIpcEventHandle_st {
    pub fn reserved(mut self, val: [::std::os::raw::c_char; 64usize]) -> Self {
        self.reserved = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaIpcMemHandle_st {
    pub fn reserved(mut self, val: [::std::os::raw::c_char; 64usize]) -> Self {
        self.reserved = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaMemFabricHandle_st {
    pub fn reserved(mut self, val: [::std::os::raw::c_char; 64usize]) -> Self {
        self.reserved = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaExternalMemoryHandleDesc {
    pub fn type_(mut self, val: cudaExternalMemoryHandleType) -> Self {
        self.type_ = val;
        self
    }
    pub fn handle(mut self, val: cudaExternalMemoryHandleDesc__bindgen_ty_1) -> Self {
        self.handle = val;
        self
    }
    pub fn size(mut self, val: u64) -> Self {
        self.size = val as _;
        self
    }
    pub fn flags(mut self, val: u32) -> Self {
        self.flags = val as _;
        self
    }
    pub fn reserved(mut self, val: [::std::os::raw::c_uint; 16usize]) -> Self {
        self.reserved = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaExternalMemoryHandleDesc__bindgen_ty_1__bindgen_ty_1 {
    pub fn handle(mut self, val: *mut ::std::os::raw::c_void) -> Self {
        self.handle = val;
        self
    }
    pub fn name(mut self, val: *const ::std::os::raw::c_void) -> Self {
        self.name = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaExternalMemoryBufferDesc {
    pub fn offset(mut self, val: u64) -> Self {
        self.offset = val as _;
        self
    }
    pub fn size(mut self, val: u64) -> Self {
        self.size = val as _;
        self
    }
    pub fn flags(mut self, val: u32) -> Self {
        self.flags = val as _;
        self
    }
    pub fn reserved(mut self, val: [::std::os::raw::c_uint; 16usize]) -> Self {
        self.reserved = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaExternalMemoryMipmappedArrayDesc {
    pub fn offset(mut self, val: u64) -> Self {
        self.offset = val as _;
        self
    }
    pub fn formatDesc(mut self, val: cudaChannelFormatDesc) -> Self {
        self.formatDesc = val;
        self
    }
    pub fn extent(mut self, val: cudaExtent) -> Self {
        self.extent = val;
        self
    }
    pub fn flags(mut self, val: u32) -> Self {
        self.flags = val as _;
        self
    }
    pub fn numLevels(mut self, val: u32) -> Self {
        self.numLevels = val as _;
        self
    }
    pub fn reserved(mut self, val: [::std::os::raw::c_uint; 16usize]) -> Self {
        self.reserved = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaExternalSemaphoreHandleDesc {
    pub fn type_(mut self, val: cudaExternalSemaphoreHandleType) -> Self {
        self.type_ = val;
        self
    }
    pub fn handle(mut self, val: cudaExternalSemaphoreHandleDesc__bindgen_ty_1) -> Self {
        self.handle = val;
        self
    }
    pub fn flags(mut self, val: u32) -> Self {
        self.flags = val as _;
        self
    }
    pub fn reserved(mut self, val: [::std::os::raw::c_uint; 16usize]) -> Self {
        self.reserved = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaExternalSemaphoreHandleDesc__bindgen_ty_1__bindgen_ty_1 {
    pub fn handle(mut self, val: *mut ::std::os::raw::c_void) -> Self {
        self.handle = val;
        self
    }
    pub fn name(mut self, val: *const ::std::os::raw::c_void) -> Self {
        self.name = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaExternalSemaphoreSignalParams {
    pub fn params(mut self, val: cudaExternalSemaphoreSignalParams__bindgen_ty_1) -> Self {
        self.params = val;
        self
    }
    pub fn flags(mut self, val: u32) -> Self {
        self.flags = val as _;
        self
    }
    pub fn reserved(mut self, val: [::std::os::raw::c_uint; 16usize]) -> Self {
        self.reserved = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaExternalSemaphoreSignalParams__bindgen_ty_1 {
    pub fn fence(mut self, val: cudaExternalSemaphoreSignalParams__bindgen_ty_1__bindgen_ty_1) -> Self {
        self.fence = val;
        self
    }
    pub fn nvSciSync(mut self, val: cudaExternalSemaphoreSignalParams__bindgen_ty_1__bindgen_ty_2) -> Self {
        self.nvSciSync = val;
        self
    }
    pub fn keyedMutex(mut self, val: cudaExternalSemaphoreSignalParams__bindgen_ty_1__bindgen_ty_3) -> Self {
        self.keyedMutex = val;
        self
    }
    pub fn reserved(mut self, val: [::std::os::raw::c_uint; 12usize]) -> Self {
        self.reserved = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaExternalSemaphoreSignalParams__bindgen_ty_1__bindgen_ty_1 {
    pub fn value(mut self, val: u64) -> Self {
        self.value = val as _;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaExternalSemaphoreSignalParams__bindgen_ty_1__bindgen_ty_3 {
    pub fn key(mut self, val: u64) -> Self {
        self.key = val as _;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaExternalSemaphoreWaitParams {
    pub fn params(mut self, val: cudaExternalSemaphoreWaitParams__bindgen_ty_1) -> Self {
        self.params = val;
        self
    }
    pub fn flags(mut self, val: u32) -> Self {
        self.flags = val as _;
        self
    }
    pub fn reserved(mut self, val: [::std::os::raw::c_uint; 16usize]) -> Self {
        self.reserved = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaExternalSemaphoreWaitParams__bindgen_ty_1 {
    pub fn fence(mut self, val: cudaExternalSemaphoreWaitParams__bindgen_ty_1__bindgen_ty_1) -> Self {
        self.fence = val;
        self
    }
    pub fn nvSciSync(mut self, val: cudaExternalSemaphoreWaitParams__bindgen_ty_1__bindgen_ty_2) -> Self {
        self.nvSciSync = val;
        self
    }
    pub fn keyedMutex(mut self, val: cudaExternalSemaphoreWaitParams__bindgen_ty_1__bindgen_ty_3) -> Self {
        self.keyedMutex = val;
        self
    }
    pub fn reserved(mut self, val: [::std::os::raw::c_uint; 10usize]) -> Self {
        self.reserved = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaExternalSemaphoreWaitParams__bindgen_ty_1__bindgen_ty_1 {
    pub fn value(mut self, val: u64) -> Self {
        self.value = val as _;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaExternalSemaphoreWaitParams__bindgen_ty_1__bindgen_ty_3 {
    pub fn key(mut self, val: u64) -> Self {
        self.key = val as _;
        self
    }
    pub fn timeoutMs(mut self, val: u32) -> Self {
        self.timeoutMs = val as _;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaDevSmResource {
    pub fn smCount(mut self, val: u32) -> Self {
        self.smCount = val as _;
        self
    }
    pub fn minSmPartitionSize(mut self, val: u32) -> Self {
        self.minSmPartitionSize = val as _;
        self
    }
    pub fn smCoscheduledAlignment(mut self, val: u32) -> Self {
        self.smCoscheduledAlignment = val as _;
        self
    }
    pub fn flags(mut self, val: u32) -> Self {
        self.flags = val as _;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaDevWorkqueueConfigResource {
    pub fn device(mut self, val: i32) -> Self {
        self.device = val as _;
        self
    }
    pub fn wqConcurrencyLimit(mut self, val: u32) -> Self {
        self.wqConcurrencyLimit = val as _;
        self
    }
    pub fn sharingScope(mut self, val: cudaDevWorkqueueConfigScope) -> Self {
        self.sharingScope = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaDevWorkqueueResource {
    pub fn reserved(mut self, val: [::std::os::raw::c_uchar; 40usize]) -> Self {
        self.reserved = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaDevSmResourceGroupParams_st {
    pub fn smCount(mut self, val: u32) -> Self {
        self.smCount = val as _;
        self
    }
    pub fn coscheduledSmCount(mut self, val: u32) -> Self {
        self.coscheduledSmCount = val as _;
        self
    }
    pub fn preferredCoscheduledSmCount(mut self, val: u32) -> Self {
        self.preferredCoscheduledSmCount = val as _;
        self
    }
    pub fn flags(mut self, val: u32) -> Self {
        self.flags = val as _;
        self
    }
    pub fn reserved(mut self, val: [::std::os::raw::c_uint; 12usize]) -> Self {
        self.reserved = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaDevResource_st {
    pub fn type_(mut self, val: cudaDevResourceType) -> Self {
        self.type_ = val;
        self
    }
    pub fn _internal_padding(mut self, val: [::std::os::raw::c_uchar; 92usize]) -> Self {
        self._internal_padding = val;
        self
    }
    pub fn __bindgen_anon_1(mut self, val: cudaDevResource_st__bindgen_ty_1) -> Self {
        self.__bindgen_anon_1 = val;
        self
    }
    pub fn nextResource(mut self, val: *mut cudaDevResource_st) -> Self {
        self.nextResource = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudalibraryHostUniversalFunctionAndDataTable {
    pub fn functionTable(mut self, val: *mut ::std::os::raw::c_void) -> Self {
        self.functionTable = val;
        self
    }
    pub fn functionWindowSize(mut self, val: usize) -> Self {
        self.functionWindowSize = val;
        self
    }
    pub fn dataTable(mut self, val: *mut ::std::os::raw::c_void) -> Self {
        self.dataTable = val;
        self
    }
    pub fn dataWindowSize(mut self, val: usize) -> Self {
        self.dataWindowSize = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaKernelNodeParams {
    pub fn func(mut self, val: *mut ::std::os::raw::c_void) -> Self {
        self.func = val;
        self
    }
    pub fn gridDim(mut self, val: dim3) -> Self {
        self.gridDim = val;
        self
    }
    pub fn blockDim(mut self, val: dim3) -> Self {
        self.blockDim = val;
        self
    }
    pub fn sharedMemBytes(mut self, val: u32) -> Self {
        self.sharedMemBytes = val as _;
        self
    }
    pub fn kernelParams(mut self, val: *mut *mut ::std::os::raw::c_void) -> Self {
        self.kernelParams = val;
        self
    }
    pub fn extra(mut self, val: *mut *mut ::std::os::raw::c_void) -> Self {
        self.extra = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaKernelNodeParamsV2 {
    pub fn __bindgen_anon_1(mut self, val: cudaKernelNodeParamsV2__bindgen_ty_1) -> Self {
        self.__bindgen_anon_1 = val;
        self
    }
    pub fn gridDim(mut self, val: dim3) -> Self {
        self.gridDim = val;
        self
    }
    pub fn blockDim(mut self, val: dim3) -> Self {
        self.blockDim = val;
        self
    }
    pub fn sharedMemBytes(mut self, val: u32) -> Self {
        self.sharedMemBytes = val as _;
        self
    }
    pub fn kernelParams(mut self, val: *mut *mut ::std::os::raw::c_void) -> Self {
        self.kernelParams = val;
        self
    }
    pub fn extra(mut self, val: *mut *mut ::std::os::raw::c_void) -> Self {
        self.extra = val;
        self
    }
    pub fn ctx(mut self, val: cudaExecutionContext_t) -> Self {
        self.ctx = val;
        self
    }
    pub fn functionType(mut self, val: cudaKernelFunctionType) -> Self {
        self.functionType = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaExternalSemaphoreSignalNodeParams {
    pub fn extSemArray(mut self, val: *mut cudaExternalSemaphore_t) -> Self {
        self.extSemArray = val;
        self
    }
    pub fn paramsArray(mut self, val: *const cudaExternalSemaphoreSignalParams) -> Self {
        self.paramsArray = val;
        self
    }
    pub fn numExtSems(mut self, val: u32) -> Self {
        self.numExtSems = val as _;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaExternalSemaphoreSignalNodeParamsV2 {
    pub fn extSemArray(mut self, val: *mut cudaExternalSemaphore_t) -> Self {
        self.extSemArray = val;
        self
    }
    pub fn paramsArray(mut self, val: *const cudaExternalSemaphoreSignalParams) -> Self {
        self.paramsArray = val;
        self
    }
    pub fn numExtSems(mut self, val: u32) -> Self {
        self.numExtSems = val as _;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaExternalSemaphoreWaitNodeParams {
    pub fn extSemArray(mut self, val: *mut cudaExternalSemaphore_t) -> Self {
        self.extSemArray = val;
        self
    }
    pub fn paramsArray(mut self, val: *const cudaExternalSemaphoreWaitParams) -> Self {
        self.paramsArray = val;
        self
    }
    pub fn numExtSems(mut self, val: u32) -> Self {
        self.numExtSems = val as _;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaExternalSemaphoreWaitNodeParamsV2 {
    pub fn extSemArray(mut self, val: *mut cudaExternalSemaphore_t) -> Self {
        self.extSemArray = val;
        self
    }
    pub fn paramsArray(mut self, val: *const cudaExternalSemaphoreWaitParams) -> Self {
        self.paramsArray = val;
        self
    }
    pub fn numExtSems(mut self, val: u32) -> Self {
        self.numExtSems = val as _;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaConditionalNodeParams {
    pub fn handle(mut self, val: cudaGraphConditionalHandle) -> Self {
        self.handle = val;
        self
    }
    pub fn type_(mut self, val: cudaGraphConditionalNodeType) -> Self {
        self.type_ = val;
        self
    }
    pub fn size(mut self, val: u32) -> Self {
        self.size = val as _;
        self
    }
    pub fn phGraph_out(mut self, val: *mut cudaGraph_t) -> Self {
        self.phGraph_out = val;
        self
    }
    pub fn ctx(mut self, val: cudaExecutionContext_t) -> Self {
        self.ctx = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaChildGraphNodeParams {
    pub fn graph(mut self, val: cudaGraph_t) -> Self {
        self.graph = val;
        self
    }
    pub fn ownership(mut self, val: cudaGraphChildGraphNodeOwnership) -> Self {
        self.ownership = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaEventRecordNodeParams {
    pub fn event(mut self, val: cudaEvent_t) -> Self {
        self.event = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaEventWaitNodeParams {
    pub fn event(mut self, val: cudaEvent_t) -> Self {
        self.event = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaGraphNodeParams {
    pub fn type_(mut self, val: cudaGraphNodeType) -> Self {
        self.type_ = val;
        self
    }
    pub fn reserved0(mut self, val: [::std::os::raw::c_int; 3usize]) -> Self {
        self.reserved0 = val;
        self
    }
    pub fn __bindgen_anon_1(mut self, val: cudaGraphNodeParams__bindgen_ty_1) -> Self {
        self.__bindgen_anon_1 = val;
        self
    }
    pub fn reserved2(mut self, val: i64) -> Self {
        self.reserved2 = val as _;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaGraphEdgeData_st {
    pub fn from_port(mut self, val: u8) -> Self {
        self.from_port = val as _;
        self
    }
    pub fn to_port(mut self, val: u8) -> Self {
        self.to_port = val as _;
        self
    }
    pub fn type_(mut self, val: u8) -> Self {
        self.type_ = val as _;
        self
    }
    pub fn reserved(mut self, val: [::std::os::raw::c_uchar; 5usize]) -> Self {
        self.reserved = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaGraphInstantiateParams_st {
    pub fn flags(mut self, val: u64) -> Self {
        self.flags = val as _;
        self
    }
    pub fn uploadStream(mut self, val: cudaStream_t) -> Self {
        self.uploadStream = val;
        self
    }
    pub fn errNode_out(mut self, val: cudaGraphNode_t) -> Self {
        self.errNode_out = val;
        self
    }
    pub fn result_out(mut self, val: cudaGraphInstantiateResult) -> Self {
        self.result_out = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaGraphExecUpdateResultInfo_st {
    pub fn result(mut self, val: cudaGraphExecUpdateResult) -> Self {
        self.result = val;
        self
    }
    pub fn errorNode(mut self, val: cudaGraphNode_t) -> Self {
        self.errorNode = val;
        self
    }
    pub fn errorFromNode(mut self, val: cudaGraphNode_t) -> Self {
        self.errorFromNode = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaGraphKernelNodeUpdate {
    pub fn node(mut self, val: cudaGraphDeviceNode_t) -> Self {
        self.node = val;
        self
    }
    pub fn field(mut self, val: cudaGraphKernelNodeField) -> Self {
        self.field = val;
        self
    }
    pub fn updateData(mut self, val: cudaGraphKernelNodeUpdate__bindgen_ty_1) -> Self {
        self.updateData = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaGraphKernelNodeUpdate__bindgen_ty_1__bindgen_ty_1 {
    pub fn pValue(mut self, val: *const ::std::os::raw::c_void) -> Self {
        self.pValue = val;
        self
    }
    pub fn offset(mut self, val: usize) -> Self {
        self.offset = val;
        self
    }
    pub fn size(mut self, val: usize) -> Self {
        self.size = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaLaunchMemSyncDomainMap_st {
    pub fn default_(mut self, val: u8) -> Self {
        self.default_ = val as _;
        self
    }
    pub fn remote(mut self, val: u8) -> Self {
        self.remote = val as _;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaLaunchAttributeValue__bindgen_ty_1 {
    pub fn x(mut self, val: u32) -> Self {
        self.x = val as _;
        self
    }
    pub fn y(mut self, val: u32) -> Self {
        self.y = val as _;
        self
    }
    pub fn z(mut self, val: u32) -> Self {
        self.z = val as _;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaLaunchAttributeValue__bindgen_ty_2 {
    pub fn event(mut self, val: cudaEvent_t) -> Self {
        self.event = val;
        self
    }
    pub fn flags(mut self, val: i32) -> Self {
        self.flags = val as _;
        self
    }
    pub fn triggerAtBlockStart(mut self, val: i32) -> Self {
        self.triggerAtBlockStart = val as _;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaLaunchAttributeValue__bindgen_ty_3 {
    pub fn x(mut self, val: u32) -> Self {
        self.x = val as _;
        self
    }
    pub fn y(mut self, val: u32) -> Self {
        self.y = val as _;
        self
    }
    pub fn z(mut self, val: u32) -> Self {
        self.z = val as _;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaLaunchAttributeValue__bindgen_ty_4 {
    pub fn event(mut self, val: cudaEvent_t) -> Self {
        self.event = val;
        self
    }
    pub fn flags(mut self, val: i32) -> Self {
        self.flags = val as _;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaLaunchAttributeValue__bindgen_ty_5 {
    pub fn deviceUpdatable(mut self, val: i32) -> Self {
        self.deviceUpdatable = val as _;
        self
    }
    pub fn devNode(mut self, val: cudaGraphDeviceNode_t) -> Self {
        self.devNode = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaLaunchAttribute_st {
    pub fn id(mut self, val: cudaLaunchAttributeID) -> Self {
        self.id = val;
        self
    }
    pub fn pad(mut self, val: [::std::os::raw::c_char; 4usize]) -> Self {
        self.pad = val;
        self
    }
    pub fn val(mut self, val: cudaLaunchAttributeValue) -> Self {
        self.val = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaLaunchConfig_st {
    pub fn gridDim(mut self, val: dim3) -> Self {
        self.gridDim = val;
        self
    }
    pub fn blockDim(mut self, val: dim3) -> Self {
        self.blockDim = val;
        self
    }
    pub fn dynamicSmemBytes(mut self, val: usize) -> Self {
        self.dynamicSmemBytes = val;
        self
    }
    pub fn stream(mut self, val: cudaStream_t) -> Self {
        self.stream = val;
        self
    }
    pub fn attrs(mut self, val: *mut cudaLaunchAttribute) -> Self {
        self.attrs = val;
        self
    }
    pub fn numAttrs(mut self, val: u32) -> Self {
        self.numAttrs = val as _;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaAsyncNotificationInfo {
    pub fn type_(mut self, val: cudaAsyncNotificationType) -> Self {
        self.type_ = val;
        self
    }
    pub fn info(mut self, val: cudaAsyncNotificationInfo__bindgen_ty_1) -> Self {
        self.info = val;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaAsyncNotificationInfo__bindgen_ty_1__bindgen_ty_1 {
    pub fn bytesOverBudget(mut self, val: u64) -> Self {
        self.bytesOverBudget = val as _;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::cudaTextureDesc {
    pub fn addressMode(mut self, val: [cudaTextureAddressMode; 3usize]) -> Self {
        self.addressMode = val;
        self
    }
    pub fn filterMode(mut self, val: cudaTextureFilterMode) -> Self {
        self.filterMode = val;
        self
    }
    pub fn readMode(mut self, val: cudaTextureReadMode) -> Self {
        self.readMode = val;
        self
    }
    pub fn sRGB(mut self, val: i32) -> Self {
        self.sRGB = val as _;
        self
    }
    pub fn borderColor(mut self, val: [f32; 4usize]) -> Self {
        self.borderColor = val;
        self
    }
    pub fn normalizedCoords(mut self, val: i32) -> Self {
        self.normalizedCoords = val as _;
        self
    }
    pub fn maxAnisotropy(mut self, val: u32) -> Self {
        self.maxAnisotropy = val as _;
        self
    }
    pub fn mipmapFilterMode(mut self, val: cudaTextureFilterMode) -> Self {
        self.mipmapFilterMode = val;
        self
    }
    pub fn mipmapLevelBias(mut self, val: f32) -> Self {
        self.mipmapLevelBias = val;
        self
    }
    pub fn minMipmapLevelClamp(mut self, val: f32) -> Self {
        self.minMipmapLevelClamp = val;
        self
    }
    pub fn maxMipmapLevelClamp(mut self, val: f32) -> Self {
        self.maxMipmapLevelClamp = val;
        self
    }
    pub fn disableTrilinearOptimization(mut self, val: i32) -> Self {
        self.disableTrilinearOptimization = val as _;
        self
    }
    pub fn seamlessCubemap(mut self, val: i32) -> Self {
        self.seamlessCubemap = val as _;
        self
    }
}
#[cfg(feature = "runtime-link")]
impl crate::sys::DynamicBindings {
    pub fn cudaDeviceReset(mut self, val: Option<unsafe extern "C" fn() -> cudaError_t>) -> Self {
        self.cudaDeviceReset = val;
        self
    }
    pub fn cudaDeviceSynchronize(mut self, val: Option<unsafe extern "C" fn() -> cudaError_t>) -> Self {
        self.cudaDeviceSynchronize = val;
        self
    }
    pub fn cudaDeviceSetLimit(
        mut self,
        val: Option<unsafe extern "C" fn(limit: cudaLimit, value: usize) -> cudaError_t>,
    ) -> Self {
        self.cudaDeviceSetLimit = val;
        self
    }
    pub fn cudaDeviceGetLimit(
        mut self,
        val: Option<unsafe extern "C" fn(pValue: *mut usize, limit: cudaLimit) -> cudaError_t>,
    ) -> Self {
        self.cudaDeviceGetLimit = val;
        self
    }
    pub fn cudaDeviceGetTexture1DLinearMaxWidth(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                maxWidthInElements: *mut usize,
                fmtDesc: *const cudaChannelFormatDesc,
                device: ::std::os::raw::c_int,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaDeviceGetTexture1DLinearMaxWidth = val;
        self
    }
    pub fn cudaDeviceGetCacheConfig(
        mut self,
        val: Option<unsafe extern "C" fn(pCacheConfig: *mut cudaFuncCache) -> cudaError_t>,
    ) -> Self {
        self.cudaDeviceGetCacheConfig = val;
        self
    }
    pub fn cudaDeviceGetStreamPriorityRange(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                leastPriority: *mut ::std::os::raw::c_int,
                greatestPriority: *mut ::std::os::raw::c_int,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaDeviceGetStreamPriorityRange = val;
        self
    }
    pub fn cudaDeviceSetCacheConfig(
        mut self,
        val: Option<unsafe extern "C" fn(cacheConfig: cudaFuncCache) -> cudaError_t>,
    ) -> Self {
        self.cudaDeviceSetCacheConfig = val;
        self
    }
    pub fn cudaDeviceGetByPCIBusId(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                device: *mut ::std::os::raw::c_int,
                pciBusId: *const ::std::os::raw::c_char,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaDeviceGetByPCIBusId = val;
        self
    }
    pub fn cudaDeviceGetPCIBusId(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                pciBusId: *mut ::std::os::raw::c_char,
                len: ::std::os::raw::c_int,
                device: ::std::os::raw::c_int,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaDeviceGetPCIBusId = val;
        self
    }
    pub fn cudaIpcGetEventHandle(
        mut self,
        val: Option<unsafe extern "C" fn(handle: *mut cudaIpcEventHandle_t, event: cudaEvent_t) -> cudaError_t>,
    ) -> Self {
        self.cudaIpcGetEventHandle = val;
        self
    }
    pub fn cudaIpcOpenEventHandle(
        mut self,
        val: Option<unsafe extern "C" fn(event: *mut cudaEvent_t, handle: cudaIpcEventHandle_t) -> cudaError_t>,
    ) -> Self {
        self.cudaIpcOpenEventHandle = val;
        self
    }
    pub fn cudaIpcGetMemHandle(
        mut self,
        val: Option<
            unsafe extern "C" fn(handle: *mut cudaIpcMemHandle_t, devPtr: *mut ::std::os::raw::c_void) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaIpcGetMemHandle = val;
        self
    }
    pub fn cudaIpcOpenMemHandle(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                devPtr: *mut *mut ::std::os::raw::c_void,
                handle: cudaIpcMemHandle_t,
                flags: ::std::os::raw::c_uint,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaIpcOpenMemHandle = val;
        self
    }
    pub fn cudaIpcCloseMemHandle(
        mut self,
        val: Option<unsafe extern "C" fn(devPtr: *mut ::std::os::raw::c_void) -> cudaError_t>,
    ) -> Self {
        self.cudaIpcCloseMemHandle = val;
        self
    }
    pub fn cudaDeviceFlushGPUDirectRDMAWrites(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                target: cudaFlushGPUDirectRDMAWritesTarget,
                scope: cudaFlushGPUDirectRDMAWritesScope,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaDeviceFlushGPUDirectRDMAWrites = val;
        self
    }
    pub fn cudaDeviceRegisterAsyncNotification(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                device: ::std::os::raw::c_int,
                callbackFunc: cudaAsyncCallback,
                userData: *mut ::std::os::raw::c_void,
                callback: *mut cudaAsyncCallbackHandle_t,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaDeviceRegisterAsyncNotification = val;
        self
    }
    pub fn cudaDeviceUnregisterAsyncNotification(
        mut self,
        val: Option<
            unsafe extern "C" fn(device: ::std::os::raw::c_int, callback: cudaAsyncCallbackHandle_t) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaDeviceUnregisterAsyncNotification = val;
        self
    }
    pub fn cudaDeviceGetSharedMemConfig(
        mut self,
        val: Option<unsafe extern "C" fn(pConfig: *mut cudaSharedMemConfig) -> cudaError_t>,
    ) -> Self {
        self.cudaDeviceGetSharedMemConfig = val;
        self
    }
    pub fn cudaDeviceSetSharedMemConfig(
        mut self,
        val: Option<unsafe extern "C" fn(config: cudaSharedMemConfig) -> cudaError_t>,
    ) -> Self {
        self.cudaDeviceSetSharedMemConfig = val;
        self
    }
    pub fn cudaGetLastError(mut self, val: Option<unsafe extern "C" fn() -> cudaError_t>) -> Self {
        self.cudaGetLastError = val;
        self
    }
    pub fn cudaPeekAtLastError(mut self, val: Option<unsafe extern "C" fn() -> cudaError_t>) -> Self {
        self.cudaPeekAtLastError = val;
        self
    }
    pub fn cudaGetErrorName(
        mut self,
        val: Option<unsafe extern "C" fn(error: cudaError_t) -> *const ::std::os::raw::c_char>,
    ) -> Self {
        self.cudaGetErrorName = val;
        self
    }
    pub fn cudaGetErrorString(
        mut self,
        val: Option<unsafe extern "C" fn(error: cudaError_t) -> *const ::std::os::raw::c_char>,
    ) -> Self {
        self.cudaGetErrorString = val;
        self
    }
    pub fn cudaGetDeviceCount(
        mut self,
        val: Option<unsafe extern "C" fn(count: *mut ::std::os::raw::c_int) -> cudaError_t>,
    ) -> Self {
        self.cudaGetDeviceCount = val;
        self
    }
    pub fn cudaGetDeviceProperties(
        mut self,
        val: Option<unsafe extern "C" fn(prop: *mut cudaDeviceProp, device: ::std::os::raw::c_int) -> cudaError_t>,
    ) -> Self {
        self.cudaGetDeviceProperties = val;
        self
    }
    pub fn cudaDeviceGetAttribute(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                value: *mut ::std::os::raw::c_int,
                attr: cudaDeviceAttr,
                device: ::std::os::raw::c_int,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaDeviceGetAttribute = val;
        self
    }
    pub fn cudaDeviceGetHostAtomicCapabilities(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                capabilities: *mut ::std::os::raw::c_uint,
                operations: *const cudaAtomicOperation,
                count: ::std::os::raw::c_uint,
                device: ::std::os::raw::c_int,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaDeviceGetHostAtomicCapabilities = val;
        self
    }
    pub fn cudaDeviceGetDefaultMemPool(
        mut self,
        val: Option<unsafe extern "C" fn(memPool: *mut cudaMemPool_t, device: ::std::os::raw::c_int) -> cudaError_t>,
    ) -> Self {
        self.cudaDeviceGetDefaultMemPool = val;
        self
    }
    pub fn cudaDeviceSetMemPool(
        mut self,
        val: Option<unsafe extern "C" fn(device: ::std::os::raw::c_int, memPool: cudaMemPool_t) -> cudaError_t>,
    ) -> Self {
        self.cudaDeviceSetMemPool = val;
        self
    }
    pub fn cudaDeviceGetMemPool(
        mut self,
        val: Option<unsafe extern "C" fn(memPool: *mut cudaMemPool_t, device: ::std::os::raw::c_int) -> cudaError_t>,
    ) -> Self {
        self.cudaDeviceGetMemPool = val;
        self
    }
    pub fn cudaDeviceGetNvSciSyncAttributes(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                nvSciSyncAttrList: *mut ::std::os::raw::c_void,
                device: ::std::os::raw::c_int,
                flags: ::std::os::raw::c_int,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaDeviceGetNvSciSyncAttributes = val;
        self
    }
    pub fn cudaDeviceGetP2PAttribute(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                value: *mut ::std::os::raw::c_int,
                attr: cudaDeviceP2PAttr,
                srcDevice: ::std::os::raw::c_int,
                dstDevice: ::std::os::raw::c_int,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaDeviceGetP2PAttribute = val;
        self
    }
    pub fn cudaDeviceGetP2PAtomicCapabilities(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                capabilities: *mut ::std::os::raw::c_uint,
                operations: *const cudaAtomicOperation,
                count: ::std::os::raw::c_uint,
                srcDevice: ::std::os::raw::c_int,
                dstDevice: ::std::os::raw::c_int,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaDeviceGetP2PAtomicCapabilities = val;
        self
    }
    pub fn cudaChooseDevice(
        mut self,
        val: Option<
            unsafe extern "C" fn(device: *mut ::std::os::raw::c_int, prop: *const cudaDeviceProp) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaChooseDevice = val;
        self
    }
    pub fn cudaInitDevice(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                device: ::std::os::raw::c_int,
                deviceFlags: ::std::os::raw::c_uint,
                flags: ::std::os::raw::c_uint,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaInitDevice = val;
        self
    }
    pub fn cudaSetDevice(
        mut self,
        val: Option<unsafe extern "C" fn(device: ::std::os::raw::c_int) -> cudaError_t>,
    ) -> Self {
        self.cudaSetDevice = val;
        self
    }
    pub fn cudaGetDevice(
        mut self,
        val: Option<unsafe extern "C" fn(device: *mut ::std::os::raw::c_int) -> cudaError_t>,
    ) -> Self {
        self.cudaGetDevice = val;
        self
    }
    pub fn cudaSetValidDevices(
        mut self,
        val: Option<
            unsafe extern "C" fn(device_arr: *mut ::std::os::raw::c_int, len: ::std::os::raw::c_int) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaSetValidDevices = val;
        self
    }
    pub fn cudaSetDeviceFlags(
        mut self,
        val: Option<unsafe extern "C" fn(flags: ::std::os::raw::c_uint) -> cudaError_t>,
    ) -> Self {
        self.cudaSetDeviceFlags = val;
        self
    }
    pub fn cudaGetDeviceFlags(
        mut self,
        val: Option<unsafe extern "C" fn(flags: *mut ::std::os::raw::c_uint) -> cudaError_t>,
    ) -> Self {
        self.cudaGetDeviceFlags = val;
        self
    }
    pub fn cudaStreamCreate(
        mut self,
        val: Option<unsafe extern "C" fn(pStream: *mut cudaStream_t) -> cudaError_t>,
    ) -> Self {
        self.cudaStreamCreate = val;
        self
    }
    pub fn cudaStreamCreateWithFlags(
        mut self,
        val: Option<unsafe extern "C" fn(pStream: *mut cudaStream_t, flags: ::std::os::raw::c_uint) -> cudaError_t>,
    ) -> Self {
        self.cudaStreamCreateWithFlags = val;
        self
    }
    pub fn cudaStreamCreateWithPriority(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                pStream: *mut cudaStream_t,
                flags: ::std::os::raw::c_uint,
                priority: ::std::os::raw::c_int,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaStreamCreateWithPriority = val;
        self
    }
    pub fn cudaStreamGetPriority(
        mut self,
        val: Option<unsafe extern "C" fn(hStream: cudaStream_t, priority: *mut ::std::os::raw::c_int) -> cudaError_t>,
    ) -> Self {
        self.cudaStreamGetPriority = val;
        self
    }
    pub fn cudaStreamGetFlags(
        mut self,
        val: Option<unsafe extern "C" fn(hStream: cudaStream_t, flags: *mut ::std::os::raw::c_uint) -> cudaError_t>,
    ) -> Self {
        self.cudaStreamGetFlags = val;
        self
    }
    pub fn cudaStreamGetId(
        mut self,
        val: Option<
            unsafe extern "C" fn(hStream: cudaStream_t, streamId: *mut ::std::os::raw::c_ulonglong) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaStreamGetId = val;
        self
    }
    pub fn cudaStreamGetDevice(
        mut self,
        val: Option<unsafe extern "C" fn(hStream: cudaStream_t, device: *mut ::std::os::raw::c_int) -> cudaError_t>,
    ) -> Self {
        self.cudaStreamGetDevice = val;
        self
    }
    pub fn cudaCtxResetPersistingL2Cache(mut self, val: Option<unsafe extern "C" fn() -> cudaError_t>) -> Self {
        self.cudaCtxResetPersistingL2Cache = val;
        self
    }
    pub fn cudaStreamCopyAttributes(
        mut self,
        val: Option<unsafe extern "C" fn(dst: cudaStream_t, src: cudaStream_t) -> cudaError_t>,
    ) -> Self {
        self.cudaStreamCopyAttributes = val;
        self
    }
    pub fn cudaStreamGetAttribute(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                hStream: cudaStream_t,
                attr: cudaLaunchAttributeID,
                value_out: *mut cudaLaunchAttributeValue,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaStreamGetAttribute = val;
        self
    }
    pub fn cudaStreamSetAttribute(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                hStream: cudaStream_t,
                attr: cudaLaunchAttributeID,
                value: *const cudaLaunchAttributeValue,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaStreamSetAttribute = val;
        self
    }
    pub fn cudaStreamDestroy(mut self, val: Option<unsafe extern "C" fn(stream: cudaStream_t) -> cudaError_t>) -> Self {
        self.cudaStreamDestroy = val;
        self
    }
    pub fn cudaStreamWaitEvent(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                stream: cudaStream_t,
                event: cudaEvent_t,
                flags: ::std::os::raw::c_uint,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaStreamWaitEvent = val;
        self
    }
    pub fn cudaStreamAddCallback(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                stream: cudaStream_t,
                callback: cudaStreamCallback_t,
                userData: *mut ::std::os::raw::c_void,
                flags: ::std::os::raw::c_uint,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaStreamAddCallback = val;
        self
    }
    pub fn cudaStreamSynchronize(
        mut self,
        val: Option<unsafe extern "C" fn(stream: cudaStream_t) -> cudaError_t>,
    ) -> Self {
        self.cudaStreamSynchronize = val;
        self
    }
    pub fn cudaStreamQuery(mut self, val: Option<unsafe extern "C" fn(stream: cudaStream_t) -> cudaError_t>) -> Self {
        self.cudaStreamQuery = val;
        self
    }
    pub fn cudaStreamAttachMemAsync(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                stream: cudaStream_t,
                devPtr: *mut ::std::os::raw::c_void,
                length: usize,
                flags: ::std::os::raw::c_uint,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaStreamAttachMemAsync = val;
        self
    }
    pub fn cudaStreamBeginCapture(
        mut self,
        val: Option<unsafe extern "C" fn(stream: cudaStream_t, mode: cudaStreamCaptureMode) -> cudaError_t>,
    ) -> Self {
        self.cudaStreamBeginCapture = val;
        self
    }
    pub fn cudaStreamBeginCaptureToGraph(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                stream: cudaStream_t,
                graph: cudaGraph_t,
                dependencies: *const cudaGraphNode_t,
                dependencyData: *const cudaGraphEdgeData,
                numDependencies: usize,
                mode: cudaStreamCaptureMode,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaStreamBeginCaptureToGraph = val;
        self
    }
    pub fn cudaThreadExchangeStreamCaptureMode(
        mut self,
        val: Option<unsafe extern "C" fn(mode: *mut cudaStreamCaptureMode) -> cudaError_t>,
    ) -> Self {
        self.cudaThreadExchangeStreamCaptureMode = val;
        self
    }
    pub fn cudaStreamEndCapture(
        mut self,
        val: Option<unsafe extern "C" fn(stream: cudaStream_t, pGraph: *mut cudaGraph_t) -> cudaError_t>,
    ) -> Self {
        self.cudaStreamEndCapture = val;
        self
    }
    pub fn cudaStreamIsCapturing(
        mut self,
        val: Option<
            unsafe extern "C" fn(stream: cudaStream_t, pCaptureStatus: *mut cudaStreamCaptureStatus) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaStreamIsCapturing = val;
        self
    }
    pub fn cudaStreamGetCaptureInfo(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                stream: cudaStream_t,
                captureStatus_out: *mut cudaStreamCaptureStatus,
                id_out: *mut ::std::os::raw::c_ulonglong,
                graph_out: *mut cudaGraph_t,
                dependencies_out: *mut *const cudaGraphNode_t,
                edgeData_out: *mut *const cudaGraphEdgeData,
                numDependencies_out: *mut usize,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaStreamGetCaptureInfo = val;
        self
    }
    pub fn cudaStreamUpdateCaptureDependencies(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                stream: cudaStream_t,
                dependencies: *mut cudaGraphNode_t,
                dependencyData: *const cudaGraphEdgeData,
                numDependencies: usize,
                flags: ::std::os::raw::c_uint,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaStreamUpdateCaptureDependencies = val;
        self
    }
    pub fn cudaEventCreate(
        mut self,
        val: Option<unsafe extern "C" fn(event: *mut cudaEvent_t) -> cudaError_t>,
    ) -> Self {
        self.cudaEventCreate = val;
        self
    }
    pub fn cudaEventCreateWithFlags(
        mut self,
        val: Option<unsafe extern "C" fn(event: *mut cudaEvent_t, flags: ::std::os::raw::c_uint) -> cudaError_t>,
    ) -> Self {
        self.cudaEventCreateWithFlags = val;
        self
    }
    pub fn cudaEventRecord(
        mut self,
        val: Option<unsafe extern "C" fn(event: cudaEvent_t, stream: cudaStream_t) -> cudaError_t>,
    ) -> Self {
        self.cudaEventRecord = val;
        self
    }
    pub fn cudaEventRecordWithFlags(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                event: cudaEvent_t,
                stream: cudaStream_t,
                flags: ::std::os::raw::c_uint,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaEventRecordWithFlags = val;
        self
    }
    pub fn cudaEventQuery(mut self, val: Option<unsafe extern "C" fn(event: cudaEvent_t) -> cudaError_t>) -> Self {
        self.cudaEventQuery = val;
        self
    }
    pub fn cudaEventSynchronize(
        mut self,
        val: Option<unsafe extern "C" fn(event: cudaEvent_t) -> cudaError_t>,
    ) -> Self {
        self.cudaEventSynchronize = val;
        self
    }
    pub fn cudaEventDestroy(mut self, val: Option<unsafe extern "C" fn(event: cudaEvent_t) -> cudaError_t>) -> Self {
        self.cudaEventDestroy = val;
        self
    }
    pub fn cudaEventElapsedTime(
        mut self,
        val: Option<unsafe extern "C" fn(ms: *mut f32, start: cudaEvent_t, end: cudaEvent_t) -> cudaError_t>,
    ) -> Self {
        self.cudaEventElapsedTime = val;
        self
    }
    pub fn cudaImportExternalMemory(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                extMem_out: *mut cudaExternalMemory_t,
                memHandleDesc: *const cudaExternalMemoryHandleDesc,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaImportExternalMemory = val;
        self
    }
    pub fn cudaExternalMemoryGetMappedBuffer(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                devPtr: *mut *mut ::std::os::raw::c_void,
                extMem: cudaExternalMemory_t,
                bufferDesc: *const cudaExternalMemoryBufferDesc,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaExternalMemoryGetMappedBuffer = val;
        self
    }
    pub fn cudaExternalMemoryGetMappedMipmappedArray(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                mipmap: *mut cudaMipmappedArray_t,
                extMem: cudaExternalMemory_t,
                mipmapDesc: *const cudaExternalMemoryMipmappedArrayDesc,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaExternalMemoryGetMappedMipmappedArray = val;
        self
    }
    pub fn cudaDestroyExternalMemory(
        mut self,
        val: Option<unsafe extern "C" fn(extMem: cudaExternalMemory_t) -> cudaError_t>,
    ) -> Self {
        self.cudaDestroyExternalMemory = val;
        self
    }
    pub fn cudaImportExternalSemaphore(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                extSem_out: *mut cudaExternalSemaphore_t,
                semHandleDesc: *const cudaExternalSemaphoreHandleDesc,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaImportExternalSemaphore = val;
        self
    }
    pub fn cudaSignalExternalSemaphoresAsync(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                extSemArray: *const cudaExternalSemaphore_t,
                paramsArray: *const cudaExternalSemaphoreSignalParams,
                numExtSems: ::std::os::raw::c_uint,
                stream: cudaStream_t,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaSignalExternalSemaphoresAsync = val;
        self
    }
    pub fn cudaWaitExternalSemaphoresAsync(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                extSemArray: *const cudaExternalSemaphore_t,
                paramsArray: *const cudaExternalSemaphoreWaitParams,
                numExtSems: ::std::os::raw::c_uint,
                stream: cudaStream_t,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaWaitExternalSemaphoresAsync = val;
        self
    }
    pub fn cudaDestroyExternalSemaphore(
        mut self,
        val: Option<unsafe extern "C" fn(extSem: cudaExternalSemaphore_t) -> cudaError_t>,
    ) -> Self {
        self.cudaDestroyExternalSemaphore = val;
        self
    }
    pub fn cudaLaunchKernel(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                func: *const ::std::os::raw::c_void,
                gridDim: dim3,
                blockDim: dim3,
                args: *mut *mut ::std::os::raw::c_void,
                sharedMem: usize,
                stream: cudaStream_t,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaLaunchKernel = val;
        self
    }
    pub fn cudaLaunchKernelExC(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                config: *const cudaLaunchConfig_t,
                func: *const ::std::os::raw::c_void,
                args: *mut *mut ::std::os::raw::c_void,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaLaunchKernelExC = val;
        self
    }
    pub fn cudaLaunchCooperativeKernel(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                func: *const ::std::os::raw::c_void,
                gridDim: dim3,
                blockDim: dim3,
                args: *mut *mut ::std::os::raw::c_void,
                sharedMem: usize,
                stream: cudaStream_t,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaLaunchCooperativeKernel = val;
        self
    }
    pub fn cudaFuncSetCacheConfig(
        mut self,
        val: Option<
            unsafe extern "C" fn(func: *const ::std::os::raw::c_void, cacheConfig: cudaFuncCache) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaFuncSetCacheConfig = val;
        self
    }
    pub fn cudaFuncGetAttributes(
        mut self,
        val: Option<
            unsafe extern "C" fn(attr: *mut cudaFuncAttributes, func: *const ::std::os::raw::c_void) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaFuncGetAttributes = val;
        self
    }
    pub fn cudaFuncSetAttribute(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                func: *const ::std::os::raw::c_void,
                attr: cudaFuncAttribute,
                value: ::std::os::raw::c_int,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaFuncSetAttribute = val;
        self
    }
    pub fn cudaFuncGetName(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                name: *mut *const ::std::os::raw::c_char,
                func: *const ::std::os::raw::c_void,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaFuncGetName = val;
        self
    }
    pub fn cudaFuncGetParamInfo(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                func: *const ::std::os::raw::c_void,
                paramIndex: usize,
                paramOffset: *mut usize,
                paramSize: *mut usize,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaFuncGetParamInfo = val;
        self
    }
    pub fn cudaFuncGetParamCount(
        mut self,
        val: Option<unsafe extern "C" fn(func: *const ::std::os::raw::c_void, paramCount: *mut usize) -> cudaError_t>,
    ) -> Self {
        self.cudaFuncGetParamCount = val;
        self
    }
    pub fn cudaLaunchHostFunc(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                stream: cudaStream_t,
                fn_: cudaHostFn_t,
                userData: *mut ::std::os::raw::c_void,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaLaunchHostFunc = val;
        self
    }
    pub fn cudaLaunchHostFunc_v2(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                stream: cudaStream_t,
                fn_: cudaHostFn_t,
                userData: *mut ::std::os::raw::c_void,
                syncMode: ::std::os::raw::c_uint,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaLaunchHostFunc_v2 = val;
        self
    }
    pub fn cudaFuncSetSharedMemConfig(
        mut self,
        val: Option<
            unsafe extern "C" fn(func: *const ::std::os::raw::c_void, config: cudaSharedMemConfig) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaFuncSetSharedMemConfig = val;
        self
    }
    pub fn cudaOccupancyMaxActiveBlocksPerMultiprocessor(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                numBlocks: *mut ::std::os::raw::c_int,
                func: *const ::std::os::raw::c_void,
                blockSize: ::std::os::raw::c_int,
                dynamicSMemSize: usize,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaOccupancyMaxActiveBlocksPerMultiprocessor = val;
        self
    }
    pub fn cudaOccupancyAvailableDynamicSMemPerBlock(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                dynamicSmemSize: *mut usize,
                func: *const ::std::os::raw::c_void,
                numBlocks: ::std::os::raw::c_int,
                blockSize: ::std::os::raw::c_int,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaOccupancyAvailableDynamicSMemPerBlock = val;
        self
    }
    pub fn cudaOccupancyMaxActiveBlocksPerMultiprocessorWithFlags(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                numBlocks: *mut ::std::os::raw::c_int,
                func: *const ::std::os::raw::c_void,
                blockSize: ::std::os::raw::c_int,
                dynamicSMemSize: usize,
                flags: ::std::os::raw::c_uint,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaOccupancyMaxActiveBlocksPerMultiprocessorWithFlags = val;
        self
    }
    pub fn cudaOccupancyMaxPotentialClusterSize(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                clusterSize: *mut ::std::os::raw::c_int,
                func: *const ::std::os::raw::c_void,
                launchConfig: *const cudaLaunchConfig_t,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaOccupancyMaxPotentialClusterSize = val;
        self
    }
    pub fn cudaOccupancyMaxActiveClusters(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                numClusters: *mut ::std::os::raw::c_int,
                func: *const ::std::os::raw::c_void,
                launchConfig: *const cudaLaunchConfig_t,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaOccupancyMaxActiveClusters = val;
        self
    }
    pub fn cudaMallocManaged(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                devPtr: *mut *mut ::std::os::raw::c_void,
                size: usize,
                flags: ::std::os::raw::c_uint,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaMallocManaged = val;
        self
    }
    pub fn cudaMalloc(
        mut self,
        val: Option<unsafe extern "C" fn(devPtr: *mut *mut ::std::os::raw::c_void, size: usize) -> cudaError_t>,
    ) -> Self {
        self.cudaMalloc = val;
        self
    }
    pub fn cudaMallocHost(
        mut self,
        val: Option<unsafe extern "C" fn(ptr: *mut *mut ::std::os::raw::c_void, size: usize) -> cudaError_t>,
    ) -> Self {
        self.cudaMallocHost = val;
        self
    }
    pub fn cudaMallocPitch(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                devPtr: *mut *mut ::std::os::raw::c_void,
                pitch: *mut usize,
                width: usize,
                height: usize,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaMallocPitch = val;
        self
    }
    pub fn cudaMallocArray(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                array: *mut cudaArray_t,
                desc: *const cudaChannelFormatDesc,
                width: usize,
                height: usize,
                flags: ::std::os::raw::c_uint,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaMallocArray = val;
        self
    }
    pub fn cudaFree(
        mut self,
        val: Option<unsafe extern "C" fn(devPtr: *mut ::std::os::raw::c_void) -> cudaError_t>,
    ) -> Self {
        self.cudaFree = val;
        self
    }
    pub fn cudaFreeHost(
        mut self,
        val: Option<unsafe extern "C" fn(ptr: *mut ::std::os::raw::c_void) -> cudaError_t>,
    ) -> Self {
        self.cudaFreeHost = val;
        self
    }
    pub fn cudaFreeArray(mut self, val: Option<unsafe extern "C" fn(array: cudaArray_t) -> cudaError_t>) -> Self {
        self.cudaFreeArray = val;
        self
    }
    pub fn cudaFreeMipmappedArray(
        mut self,
        val: Option<unsafe extern "C" fn(mipmappedArray: cudaMipmappedArray_t) -> cudaError_t>,
    ) -> Self {
        self.cudaFreeMipmappedArray = val;
        self
    }
    pub fn cudaHostAlloc(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                pHost: *mut *mut ::std::os::raw::c_void,
                size: usize,
                flags: ::std::os::raw::c_uint,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaHostAlloc = val;
        self
    }
    pub fn cudaHostRegister(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                ptr: *mut ::std::os::raw::c_void,
                size: usize,
                flags: ::std::os::raw::c_uint,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaHostRegister = val;
        self
    }
    pub fn cudaHostUnregister(
        mut self,
        val: Option<unsafe extern "C" fn(ptr: *mut ::std::os::raw::c_void) -> cudaError_t>,
    ) -> Self {
        self.cudaHostUnregister = val;
        self
    }
    pub fn cudaHostGetDevicePointer(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                pDevice: *mut *mut ::std::os::raw::c_void,
                pHost: *mut ::std::os::raw::c_void,
                flags: ::std::os::raw::c_uint,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaHostGetDevicePointer = val;
        self
    }
    pub fn cudaHostGetFlags(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                pFlags: *mut ::std::os::raw::c_uint,
                pHost: *mut ::std::os::raw::c_void,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaHostGetFlags = val;
        self
    }
    pub fn cudaMalloc3D(
        mut self,
        val: Option<unsafe extern "C" fn(pitchedDevPtr: *mut cudaPitchedPtr, extent: cudaExtent) -> cudaError_t>,
    ) -> Self {
        self.cudaMalloc3D = val;
        self
    }
    pub fn cudaMalloc3DArray(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                array: *mut cudaArray_t,
                desc: *const cudaChannelFormatDesc,
                extent: cudaExtent,
                flags: ::std::os::raw::c_uint,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaMalloc3DArray = val;
        self
    }
    pub fn cudaMallocMipmappedArray(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                mipmappedArray: *mut cudaMipmappedArray_t,
                desc: *const cudaChannelFormatDesc,
                extent: cudaExtent,
                numLevels: ::std::os::raw::c_uint,
                flags: ::std::os::raw::c_uint,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaMallocMipmappedArray = val;
        self
    }
    pub fn cudaGetMipmappedArrayLevel(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                levelArray: *mut cudaArray_t,
                mipmappedArray: cudaMipmappedArray_const_t,
                level: ::std::os::raw::c_uint,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGetMipmappedArrayLevel = val;
        self
    }
    pub fn cudaMemcpy3D(
        mut self,
        val: Option<unsafe extern "C" fn(p: *const cudaMemcpy3DParms) -> cudaError_t>,
    ) -> Self {
        self.cudaMemcpy3D = val;
        self
    }
    pub fn cudaMemcpy3DPeer(
        mut self,
        val: Option<unsafe extern "C" fn(p: *const cudaMemcpy3DPeerParms) -> cudaError_t>,
    ) -> Self {
        self.cudaMemcpy3DPeer = val;
        self
    }
    pub fn cudaMemcpy3DAsync(
        mut self,
        val: Option<unsafe extern "C" fn(p: *const cudaMemcpy3DParms, stream: cudaStream_t) -> cudaError_t>,
    ) -> Self {
        self.cudaMemcpy3DAsync = val;
        self
    }
    pub fn cudaMemcpy3DPeerAsync(
        mut self,
        val: Option<unsafe extern "C" fn(p: *const cudaMemcpy3DPeerParms, stream: cudaStream_t) -> cudaError_t>,
    ) -> Self {
        self.cudaMemcpy3DPeerAsync = val;
        self
    }
    pub fn cudaMemGetInfo(
        mut self,
        val: Option<unsafe extern "C" fn(free: *mut usize, total: *mut usize) -> cudaError_t>,
    ) -> Self {
        self.cudaMemGetInfo = val;
        self
    }
    pub fn cudaArrayGetInfo(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                desc: *mut cudaChannelFormatDesc,
                extent: *mut cudaExtent,
                flags: *mut ::std::os::raw::c_uint,
                array: cudaArray_t,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaArrayGetInfo = val;
        self
    }
    pub fn cudaArrayGetPlane(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                pPlaneArray: *mut cudaArray_t,
                hArray: cudaArray_t,
                planeIdx: ::std::os::raw::c_uint,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaArrayGetPlane = val;
        self
    }
    pub fn cudaArrayGetMemoryRequirements(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                memoryRequirements: *mut cudaArrayMemoryRequirements,
                array: cudaArray_t,
                device: ::std::os::raw::c_int,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaArrayGetMemoryRequirements = val;
        self
    }
    pub fn cudaMipmappedArrayGetMemoryRequirements(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                memoryRequirements: *mut cudaArrayMemoryRequirements,
                mipmap: cudaMipmappedArray_t,
                device: ::std::os::raw::c_int,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaMipmappedArrayGetMemoryRequirements = val;
        self
    }
    pub fn cudaArrayGetSparseProperties(
        mut self,
        val: Option<
            unsafe extern "C" fn(sparseProperties: *mut cudaArraySparseProperties, array: cudaArray_t) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaArrayGetSparseProperties = val;
        self
    }
    pub fn cudaMipmappedArrayGetSparseProperties(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                sparseProperties: *mut cudaArraySparseProperties,
                mipmap: cudaMipmappedArray_t,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaMipmappedArrayGetSparseProperties = val;
        self
    }
    pub fn cudaMemcpy(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                dst: *mut ::std::os::raw::c_void,
                src: *const ::std::os::raw::c_void,
                count: usize,
                kind: cudaMemcpyKind,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaMemcpy = val;
        self
    }
    pub fn cudaMemcpyPeer(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                dst: *mut ::std::os::raw::c_void,
                dstDevice: ::std::os::raw::c_int,
                src: *const ::std::os::raw::c_void,
                srcDevice: ::std::os::raw::c_int,
                count: usize,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaMemcpyPeer = val;
        self
    }
    pub fn cudaMemcpy2D(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                dst: *mut ::std::os::raw::c_void,
                dpitch: usize,
                src: *const ::std::os::raw::c_void,
                spitch: usize,
                width: usize,
                height: usize,
                kind: cudaMemcpyKind,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaMemcpy2D = val;
        self
    }
    pub fn cudaMemcpy2DToArray(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                dst: cudaArray_t,
                wOffset: usize,
                hOffset: usize,
                src: *const ::std::os::raw::c_void,
                spitch: usize,
                width: usize,
                height: usize,
                kind: cudaMemcpyKind,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaMemcpy2DToArray = val;
        self
    }
    pub fn cudaMemcpy2DFromArray(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                dst: *mut ::std::os::raw::c_void,
                dpitch: usize,
                src: cudaArray_const_t,
                wOffset: usize,
                hOffset: usize,
                width: usize,
                height: usize,
                kind: cudaMemcpyKind,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaMemcpy2DFromArray = val;
        self
    }
    pub fn cudaMemcpy2DArrayToArray(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                dst: cudaArray_t,
                wOffsetDst: usize,
                hOffsetDst: usize,
                src: cudaArray_const_t,
                wOffsetSrc: usize,
                hOffsetSrc: usize,
                width: usize,
                height: usize,
                kind: cudaMemcpyKind,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaMemcpy2DArrayToArray = val;
        self
    }
    pub fn cudaMemcpyToSymbol(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                symbol: *const ::std::os::raw::c_void,
                src: *const ::std::os::raw::c_void,
                count: usize,
                offset: usize,
                kind: cudaMemcpyKind,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaMemcpyToSymbol = val;
        self
    }
    pub fn cudaMemcpyFromSymbol(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                dst: *mut ::std::os::raw::c_void,
                symbol: *const ::std::os::raw::c_void,
                count: usize,
                offset: usize,
                kind: cudaMemcpyKind,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaMemcpyFromSymbol = val;
        self
    }
    pub fn cudaMemcpyAsync(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                dst: *mut ::std::os::raw::c_void,
                src: *const ::std::os::raw::c_void,
                count: usize,
                kind: cudaMemcpyKind,
                stream: cudaStream_t,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaMemcpyAsync = val;
        self
    }
    pub fn cudaMemcpyPeerAsync(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                dst: *mut ::std::os::raw::c_void,
                dstDevice: ::std::os::raw::c_int,
                src: *const ::std::os::raw::c_void,
                srcDevice: ::std::os::raw::c_int,
                count: usize,
                stream: cudaStream_t,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaMemcpyPeerAsync = val;
        self
    }
    pub fn cudaMemcpyBatchAsync(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                dsts: *const *mut ::std::os::raw::c_void,
                srcs: *const *const ::std::os::raw::c_void,
                sizes: *const usize,
                count: usize,
                attrs: *mut cudaMemcpyAttributes,
                attrsIdxs: *mut usize,
                numAttrs: usize,
                stream: cudaStream_t,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaMemcpyBatchAsync = val;
        self
    }
    pub fn cudaMemcpy3DBatchAsync(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                numOps: usize,
                opList: *mut cudaMemcpy3DBatchOp,
                flags: ::std::os::raw::c_ulonglong,
                stream: cudaStream_t,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaMemcpy3DBatchAsync = val;
        self
    }
    pub fn cudaMemcpyWithAttributesAsync(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                dst: *mut ::std::os::raw::c_void,
                src: *const ::std::os::raw::c_void,
                size: usize,
                attr: *mut cudaMemcpyAttributes,
                stream: cudaStream_t,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaMemcpyWithAttributesAsync = val;
        self
    }
    pub fn cudaMemcpy3DWithAttributesAsync(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                op: *mut cudaMemcpy3DBatchOp,
                flags: ::std::os::raw::c_ulonglong,
                stream: cudaStream_t,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaMemcpy3DWithAttributesAsync = val;
        self
    }
    pub fn cudaMemcpy2DAsync(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                dst: *mut ::std::os::raw::c_void,
                dpitch: usize,
                src: *const ::std::os::raw::c_void,
                spitch: usize,
                width: usize,
                height: usize,
                kind: cudaMemcpyKind,
                stream: cudaStream_t,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaMemcpy2DAsync = val;
        self
    }
    pub fn cudaMemcpy2DToArrayAsync(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                dst: cudaArray_t,
                wOffset: usize,
                hOffset: usize,
                src: *const ::std::os::raw::c_void,
                spitch: usize,
                width: usize,
                height: usize,
                kind: cudaMemcpyKind,
                stream: cudaStream_t,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaMemcpy2DToArrayAsync = val;
        self
    }
    pub fn cudaMemcpy2DFromArrayAsync(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                dst: *mut ::std::os::raw::c_void,
                dpitch: usize,
                src: cudaArray_const_t,
                wOffset: usize,
                hOffset: usize,
                width: usize,
                height: usize,
                kind: cudaMemcpyKind,
                stream: cudaStream_t,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaMemcpy2DFromArrayAsync = val;
        self
    }
    pub fn cudaMemcpyToSymbolAsync(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                symbol: *const ::std::os::raw::c_void,
                src: *const ::std::os::raw::c_void,
                count: usize,
                offset: usize,
                kind: cudaMemcpyKind,
                stream: cudaStream_t,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaMemcpyToSymbolAsync = val;
        self
    }
    pub fn cudaMemcpyFromSymbolAsync(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                dst: *mut ::std::os::raw::c_void,
                symbol: *const ::std::os::raw::c_void,
                count: usize,
                offset: usize,
                kind: cudaMemcpyKind,
                stream: cudaStream_t,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaMemcpyFromSymbolAsync = val;
        self
    }
    pub fn cudaMemset(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                devPtr: *mut ::std::os::raw::c_void,
                value: ::std::os::raw::c_int,
                count: usize,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaMemset = val;
        self
    }
    pub fn cudaMemset2D(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                devPtr: *mut ::std::os::raw::c_void,
                pitch: usize,
                value: ::std::os::raw::c_int,
                width: usize,
                height: usize,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaMemset2D = val;
        self
    }
    pub fn cudaMemset3D(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                pitchedDevPtr: cudaPitchedPtr,
                value: ::std::os::raw::c_int,
                extent: cudaExtent,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaMemset3D = val;
        self
    }
    pub fn cudaMemsetAsync(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                devPtr: *mut ::std::os::raw::c_void,
                value: ::std::os::raw::c_int,
                count: usize,
                stream: cudaStream_t,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaMemsetAsync = val;
        self
    }
    pub fn cudaMemset2DAsync(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                devPtr: *mut ::std::os::raw::c_void,
                pitch: usize,
                value: ::std::os::raw::c_int,
                width: usize,
                height: usize,
                stream: cudaStream_t,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaMemset2DAsync = val;
        self
    }
    pub fn cudaMemset3DAsync(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                pitchedDevPtr: cudaPitchedPtr,
                value: ::std::os::raw::c_int,
                extent: cudaExtent,
                stream: cudaStream_t,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaMemset3DAsync = val;
        self
    }
    pub fn cudaGetSymbolAddress(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                devPtr: *mut *mut ::std::os::raw::c_void,
                symbol: *const ::std::os::raw::c_void,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGetSymbolAddress = val;
        self
    }
    pub fn cudaGetSymbolSize(
        mut self,
        val: Option<unsafe extern "C" fn(size: *mut usize, symbol: *const ::std::os::raw::c_void) -> cudaError_t>,
    ) -> Self {
        self.cudaGetSymbolSize = val;
        self
    }
    pub fn cudaMemPrefetchAsync(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                devPtr: *const ::std::os::raw::c_void,
                count: usize,
                location: cudaMemLocation,
                flags: ::std::os::raw::c_uint,
                stream: cudaStream_t,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaMemPrefetchAsync = val;
        self
    }
    pub fn cudaMemPrefetchBatchAsync(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                dptrs: *mut *mut ::std::os::raw::c_void,
                sizes: *mut usize,
                count: usize,
                prefetchLocs: *mut cudaMemLocation,
                prefetchLocIdxs: *mut usize,
                numPrefetchLocs: usize,
                flags: ::std::os::raw::c_ulonglong,
                stream: cudaStream_t,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaMemPrefetchBatchAsync = val;
        self
    }
    pub fn cudaMemDiscardBatchAsync(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                dptrs: *mut *mut ::std::os::raw::c_void,
                sizes: *mut usize,
                count: usize,
                flags: ::std::os::raw::c_ulonglong,
                stream: cudaStream_t,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaMemDiscardBatchAsync = val;
        self
    }
    pub fn cudaMemDiscardAndPrefetchBatchAsync(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                dptrs: *mut *mut ::std::os::raw::c_void,
                sizes: *mut usize,
                count: usize,
                prefetchLocs: *mut cudaMemLocation,
                prefetchLocIdxs: *mut usize,
                numPrefetchLocs: usize,
                flags: ::std::os::raw::c_ulonglong,
                stream: cudaStream_t,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaMemDiscardAndPrefetchBatchAsync = val;
        self
    }
    pub fn cudaMemAdvise(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                devPtr: *const ::std::os::raw::c_void,
                count: usize,
                advice: cudaMemoryAdvise,
                location: cudaMemLocation,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaMemAdvise = val;
        self
    }
    pub fn cudaMemRangeGetAttribute(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                data: *mut ::std::os::raw::c_void,
                dataSize: usize,
                attribute: cudaMemRangeAttribute,
                devPtr: *const ::std::os::raw::c_void,
                count: usize,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaMemRangeGetAttribute = val;
        self
    }
    pub fn cudaMemRangeGetAttributes(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                data: *mut *mut ::std::os::raw::c_void,
                dataSizes: *mut usize,
                attributes: *mut cudaMemRangeAttribute,
                numAttributes: usize,
                devPtr: *const ::std::os::raw::c_void,
                count: usize,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaMemRangeGetAttributes = val;
        self
    }
    pub fn cudaMemcpyToArray(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                dst: cudaArray_t,
                wOffset: usize,
                hOffset: usize,
                src: *const ::std::os::raw::c_void,
                count: usize,
                kind: cudaMemcpyKind,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaMemcpyToArray = val;
        self
    }
    pub fn cudaMemcpyFromArray(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                dst: *mut ::std::os::raw::c_void,
                src: cudaArray_const_t,
                wOffset: usize,
                hOffset: usize,
                count: usize,
                kind: cudaMemcpyKind,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaMemcpyFromArray = val;
        self
    }
    pub fn cudaMemcpyArrayToArray(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                dst: cudaArray_t,
                wOffsetDst: usize,
                hOffsetDst: usize,
                src: cudaArray_const_t,
                wOffsetSrc: usize,
                hOffsetSrc: usize,
                count: usize,
                kind: cudaMemcpyKind,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaMemcpyArrayToArray = val;
        self
    }
    pub fn cudaMemcpyToArrayAsync(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                dst: cudaArray_t,
                wOffset: usize,
                hOffset: usize,
                src: *const ::std::os::raw::c_void,
                count: usize,
                kind: cudaMemcpyKind,
                stream: cudaStream_t,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaMemcpyToArrayAsync = val;
        self
    }
    pub fn cudaMemcpyFromArrayAsync(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                dst: *mut ::std::os::raw::c_void,
                src: cudaArray_const_t,
                wOffset: usize,
                hOffset: usize,
                count: usize,
                kind: cudaMemcpyKind,
                stream: cudaStream_t,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaMemcpyFromArrayAsync = val;
        self
    }
    pub fn cudaMallocAsync(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                devPtr: *mut *mut ::std::os::raw::c_void,
                size: usize,
                hStream: cudaStream_t,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaMallocAsync = val;
        self
    }
    pub fn cudaFreeAsync(
        mut self,
        val: Option<unsafe extern "C" fn(devPtr: *mut ::std::os::raw::c_void, hStream: cudaStream_t) -> cudaError_t>,
    ) -> Self {
        self.cudaFreeAsync = val;
        self
    }
    pub fn cudaMemPoolTrimTo(
        mut self,
        val: Option<unsafe extern "C" fn(memPool: cudaMemPool_t, minBytesToKeep: usize) -> cudaError_t>,
    ) -> Self {
        self.cudaMemPoolTrimTo = val;
        self
    }
    pub fn cudaMemPoolSetAttribute(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                memPool: cudaMemPool_t,
                attr: cudaMemPoolAttr,
                value: *mut ::std::os::raw::c_void,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaMemPoolSetAttribute = val;
        self
    }
    pub fn cudaMemPoolGetAttribute(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                memPool: cudaMemPool_t,
                attr: cudaMemPoolAttr,
                value: *mut ::std::os::raw::c_void,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaMemPoolGetAttribute = val;
        self
    }
    pub fn cudaMemPoolSetAccess(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                memPool: cudaMemPool_t,
                descList: *const cudaMemAccessDesc,
                count: usize,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaMemPoolSetAccess = val;
        self
    }
    pub fn cudaMemPoolGetAccess(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                flags: *mut cudaMemAccessFlags,
                memPool: cudaMemPool_t,
                location: *mut cudaMemLocation,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaMemPoolGetAccess = val;
        self
    }
    pub fn cudaMemPoolCreate(
        mut self,
        val: Option<
            unsafe extern "C" fn(memPool: *mut cudaMemPool_t, poolProps: *const cudaMemPoolProps) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaMemPoolCreate = val;
        self
    }
    pub fn cudaMemPoolDestroy(
        mut self,
        val: Option<unsafe extern "C" fn(memPool: cudaMemPool_t) -> cudaError_t>,
    ) -> Self {
        self.cudaMemPoolDestroy = val;
        self
    }
    pub fn cudaMemGetDefaultMemPool(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                memPool: *mut cudaMemPool_t,
                location: *mut cudaMemLocation,
                type_: cudaMemAllocationType,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaMemGetDefaultMemPool = val;
        self
    }
    pub fn cudaMemGetMemPool(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                memPool: *mut cudaMemPool_t,
                location: *mut cudaMemLocation,
                type_: cudaMemAllocationType,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaMemGetMemPool = val;
        self
    }
    pub fn cudaMemSetMemPool(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                location: *mut cudaMemLocation,
                type_: cudaMemAllocationType,
                memPool: cudaMemPool_t,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaMemSetMemPool = val;
        self
    }
    pub fn cudaMallocFromPoolAsync(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                ptr: *mut *mut ::std::os::raw::c_void,
                size: usize,
                memPool: cudaMemPool_t,
                stream: cudaStream_t,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaMallocFromPoolAsync = val;
        self
    }
    pub fn cudaMemPoolExportToShareableHandle(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                shareableHandle: *mut ::std::os::raw::c_void,
                memPool: cudaMemPool_t,
                handleType: cudaMemAllocationHandleType,
                flags: ::std::os::raw::c_uint,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaMemPoolExportToShareableHandle = val;
        self
    }
    pub fn cudaMemPoolImportFromShareableHandle(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                memPool: *mut cudaMemPool_t,
                shareableHandle: *mut ::std::os::raw::c_void,
                handleType: cudaMemAllocationHandleType,
                flags: ::std::os::raw::c_uint,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaMemPoolImportFromShareableHandle = val;
        self
    }
    pub fn cudaMemPoolExportPointer(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                exportData: *mut cudaMemPoolPtrExportData,
                ptr: *mut ::std::os::raw::c_void,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaMemPoolExportPointer = val;
        self
    }
    pub fn cudaMemPoolImportPointer(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                ptr: *mut *mut ::std::os::raw::c_void,
                memPool: cudaMemPool_t,
                exportData: *mut cudaMemPoolPtrExportData,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaMemPoolImportPointer = val;
        self
    }
    pub fn cudaPointerGetAttributes(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                attributes: *mut cudaPointerAttributes,
                ptr: *const ::std::os::raw::c_void,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaPointerGetAttributes = val;
        self
    }
    pub fn cudaDeviceCanAccessPeer(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                canAccessPeer: *mut ::std::os::raw::c_int,
                device: ::std::os::raw::c_int,
                peerDevice: ::std::os::raw::c_int,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaDeviceCanAccessPeer = val;
        self
    }
    pub fn cudaDeviceEnablePeerAccess(
        mut self,
        val: Option<
            unsafe extern "C" fn(peerDevice: ::std::os::raw::c_int, flags: ::std::os::raw::c_uint) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaDeviceEnablePeerAccess = val;
        self
    }
    pub fn cudaDeviceDisablePeerAccess(
        mut self,
        val: Option<unsafe extern "C" fn(peerDevice: ::std::os::raw::c_int) -> cudaError_t>,
    ) -> Self {
        self.cudaDeviceDisablePeerAccess = val;
        self
    }
    pub fn cudaGraphicsUnregisterResource(
        mut self,
        val: Option<unsafe extern "C" fn(resource: cudaGraphicsResource_t) -> cudaError_t>,
    ) -> Self {
        self.cudaGraphicsUnregisterResource = val;
        self
    }
    pub fn cudaGraphicsResourceSetMapFlags(
        mut self,
        val: Option<
            unsafe extern "C" fn(resource: cudaGraphicsResource_t, flags: ::std::os::raw::c_uint) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGraphicsResourceSetMapFlags = val;
        self
    }
    pub fn cudaGraphicsMapResources(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                count: ::std::os::raw::c_int,
                resources: *mut cudaGraphicsResource_t,
                stream: cudaStream_t,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGraphicsMapResources = val;
        self
    }
    pub fn cudaGraphicsUnmapResources(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                count: ::std::os::raw::c_int,
                resources: *mut cudaGraphicsResource_t,
                stream: cudaStream_t,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGraphicsUnmapResources = val;
        self
    }
    pub fn cudaGraphicsResourceGetMappedPointer(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                devPtr: *mut *mut ::std::os::raw::c_void,
                size: *mut usize,
                resource: cudaGraphicsResource_t,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGraphicsResourceGetMappedPointer = val;
        self
    }
    pub fn cudaGraphicsSubResourceGetMappedArray(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                array: *mut cudaArray_t,
                resource: cudaGraphicsResource_t,
                arrayIndex: ::std::os::raw::c_uint,
                mipLevel: ::std::os::raw::c_uint,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGraphicsSubResourceGetMappedArray = val;
        self
    }
    pub fn cudaGraphicsResourceGetMappedMipmappedArray(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                mipmappedArray: *mut cudaMipmappedArray_t,
                resource: cudaGraphicsResource_t,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGraphicsResourceGetMappedMipmappedArray = val;
        self
    }
    pub fn cudaGetChannelDesc(
        mut self,
        val: Option<unsafe extern "C" fn(desc: *mut cudaChannelFormatDesc, array: cudaArray_const_t) -> cudaError_t>,
    ) -> Self {
        self.cudaGetChannelDesc = val;
        self
    }
    pub fn cudaCreateChannelDesc(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                x: ::std::os::raw::c_int,
                y: ::std::os::raw::c_int,
                z: ::std::os::raw::c_int,
                w: ::std::os::raw::c_int,
                f: cudaChannelFormatKind,
            ) -> cudaChannelFormatDesc,
        >,
    ) -> Self {
        self.cudaCreateChannelDesc = val;
        self
    }
    pub fn cudaCreateTextureObject(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                pTexObject: *mut cudaTextureObject_t,
                pResDesc: *const cudaResourceDesc,
                pTexDesc: *const cudaTextureDesc,
                pResViewDesc: *const cudaResourceViewDesc,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaCreateTextureObject = val;
        self
    }
    pub fn cudaDestroyTextureObject(
        mut self,
        val: Option<unsafe extern "C" fn(texObject: cudaTextureObject_t) -> cudaError_t>,
    ) -> Self {
        self.cudaDestroyTextureObject = val;
        self
    }
    pub fn cudaGetTextureObjectResourceDesc(
        mut self,
        val: Option<
            unsafe extern "C" fn(pResDesc: *mut cudaResourceDesc, texObject: cudaTextureObject_t) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGetTextureObjectResourceDesc = val;
        self
    }
    pub fn cudaGetTextureObjectTextureDesc(
        mut self,
        val: Option<
            unsafe extern "C" fn(pTexDesc: *mut cudaTextureDesc, texObject: cudaTextureObject_t) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGetTextureObjectTextureDesc = val;
        self
    }
    pub fn cudaGetTextureObjectResourceViewDesc(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                pResViewDesc: *mut cudaResourceViewDesc,
                texObject: cudaTextureObject_t,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGetTextureObjectResourceViewDesc = val;
        self
    }
    pub fn cudaCreateSurfaceObject(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                pSurfObject: *mut cudaSurfaceObject_t,
                pResDesc: *const cudaResourceDesc,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaCreateSurfaceObject = val;
        self
    }
    pub fn cudaDestroySurfaceObject(
        mut self,
        val: Option<unsafe extern "C" fn(surfObject: cudaSurfaceObject_t) -> cudaError_t>,
    ) -> Self {
        self.cudaDestroySurfaceObject = val;
        self
    }
    pub fn cudaGetSurfaceObjectResourceDesc(
        mut self,
        val: Option<
            unsafe extern "C" fn(pResDesc: *mut cudaResourceDesc, surfObject: cudaSurfaceObject_t) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGetSurfaceObjectResourceDesc = val;
        self
    }
    pub fn cudaDriverGetVersion(
        mut self,
        val: Option<unsafe extern "C" fn(driverVersion: *mut ::std::os::raw::c_int) -> cudaError_t>,
    ) -> Self {
        self.cudaDriverGetVersion = val;
        self
    }
    pub fn cudaRuntimeGetVersion(
        mut self,
        val: Option<unsafe extern "C" fn(runtimeVersion: *mut ::std::os::raw::c_int) -> cudaError_t>,
    ) -> Self {
        self.cudaRuntimeGetVersion = val;
        self
    }
    pub fn cudaLogsRegisterCallback(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                callbackFunc: cudaLogsCallback_t,
                userData: *mut ::std::os::raw::c_void,
                callback_out: *mut cudaLogsCallbackHandle,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaLogsRegisterCallback = val;
        self
    }
    pub fn cudaLogsUnregisterCallback(
        mut self,
        val: Option<unsafe extern "C" fn(callback: cudaLogsCallbackHandle) -> cudaError_t>,
    ) -> Self {
        self.cudaLogsUnregisterCallback = val;
        self
    }
    pub fn cudaLogsCurrent(
        mut self,
        val: Option<
            unsafe extern "C" fn(iterator_out: *mut cudaLogIterator, flags: ::std::os::raw::c_uint) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaLogsCurrent = val;
        self
    }
    pub fn cudaLogsDumpToFile(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                iterator: *mut cudaLogIterator,
                pathToFile: *const ::std::os::raw::c_char,
                flags: ::std::os::raw::c_uint,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaLogsDumpToFile = val;
        self
    }
    pub fn cudaLogsDumpToMemory(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                iterator: *mut cudaLogIterator,
                buffer: *mut ::std::os::raw::c_char,
                size: *mut usize,
                flags: ::std::os::raw::c_uint,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaLogsDumpToMemory = val;
        self
    }
    pub fn cudaGraphCreate(
        mut self,
        val: Option<unsafe extern "C" fn(pGraph: *mut cudaGraph_t, flags: ::std::os::raw::c_uint) -> cudaError_t>,
    ) -> Self {
        self.cudaGraphCreate = val;
        self
    }
    pub fn cudaGraphAddKernelNode(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                pGraphNode: *mut cudaGraphNode_t,
                graph: cudaGraph_t,
                pDependencies: *const cudaGraphNode_t,
                numDependencies: usize,
                pNodeParams: *const cudaKernelNodeParams,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGraphAddKernelNode = val;
        self
    }
    pub fn cudaGraphKernelNodeGetParams(
        mut self,
        val: Option<unsafe extern "C" fn(node: cudaGraphNode_t, pNodeParams: *mut cudaKernelNodeParams) -> cudaError_t>,
    ) -> Self {
        self.cudaGraphKernelNodeGetParams = val;
        self
    }
    pub fn cudaGraphKernelNodeSetParams(
        mut self,
        val: Option<
            unsafe extern "C" fn(node: cudaGraphNode_t, pNodeParams: *const cudaKernelNodeParams) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGraphKernelNodeSetParams = val;
        self
    }
    pub fn cudaGraphKernelNodeCopyAttributes(
        mut self,
        val: Option<unsafe extern "C" fn(hDst: cudaGraphNode_t, hSrc: cudaGraphNode_t) -> cudaError_t>,
    ) -> Self {
        self.cudaGraphKernelNodeCopyAttributes = val;
        self
    }
    pub fn cudaGraphKernelNodeGetAttribute(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                hNode: cudaGraphNode_t,
                attr: cudaLaunchAttributeID,
                value_out: *mut cudaLaunchAttributeValue,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGraphKernelNodeGetAttribute = val;
        self
    }
    pub fn cudaGraphKernelNodeSetAttribute(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                hNode: cudaGraphNode_t,
                attr: cudaLaunchAttributeID,
                value: *const cudaLaunchAttributeValue,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGraphKernelNodeSetAttribute = val;
        self
    }
    pub fn cudaGraphAddMemcpyNode(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                pGraphNode: *mut cudaGraphNode_t,
                graph: cudaGraph_t,
                pDependencies: *const cudaGraphNode_t,
                numDependencies: usize,
                pCopyParams: *const cudaMemcpy3DParms,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGraphAddMemcpyNode = val;
        self
    }
    pub fn cudaGraphAddMemcpyNodeToSymbol(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                pGraphNode: *mut cudaGraphNode_t,
                graph: cudaGraph_t,
                pDependencies: *const cudaGraphNode_t,
                numDependencies: usize,
                symbol: *const ::std::os::raw::c_void,
                src: *const ::std::os::raw::c_void,
                count: usize,
                offset: usize,
                kind: cudaMemcpyKind,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGraphAddMemcpyNodeToSymbol = val;
        self
    }
    pub fn cudaGraphAddMemcpyNodeFromSymbol(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                pGraphNode: *mut cudaGraphNode_t,
                graph: cudaGraph_t,
                pDependencies: *const cudaGraphNode_t,
                numDependencies: usize,
                dst: *mut ::std::os::raw::c_void,
                symbol: *const ::std::os::raw::c_void,
                count: usize,
                offset: usize,
                kind: cudaMemcpyKind,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGraphAddMemcpyNodeFromSymbol = val;
        self
    }
    pub fn cudaGraphAddMemcpyNode1D(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                pGraphNode: *mut cudaGraphNode_t,
                graph: cudaGraph_t,
                pDependencies: *const cudaGraphNode_t,
                numDependencies: usize,
                dst: *mut ::std::os::raw::c_void,
                src: *const ::std::os::raw::c_void,
                count: usize,
                kind: cudaMemcpyKind,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGraphAddMemcpyNode1D = val;
        self
    }
    pub fn cudaGraphMemcpyNodeGetParams(
        mut self,
        val: Option<unsafe extern "C" fn(node: cudaGraphNode_t, pNodeParams: *mut cudaMemcpy3DParms) -> cudaError_t>,
    ) -> Self {
        self.cudaGraphMemcpyNodeGetParams = val;
        self
    }
    pub fn cudaGraphMemcpyNodeSetParams(
        mut self,
        val: Option<unsafe extern "C" fn(node: cudaGraphNode_t, pNodeParams: *const cudaMemcpy3DParms) -> cudaError_t>,
    ) -> Self {
        self.cudaGraphMemcpyNodeSetParams = val;
        self
    }
    pub fn cudaGraphMemcpyNodeSetParamsToSymbol(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                node: cudaGraphNode_t,
                symbol: *const ::std::os::raw::c_void,
                src: *const ::std::os::raw::c_void,
                count: usize,
                offset: usize,
                kind: cudaMemcpyKind,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGraphMemcpyNodeSetParamsToSymbol = val;
        self
    }
    pub fn cudaGraphMemcpyNodeSetParamsFromSymbol(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                node: cudaGraphNode_t,
                dst: *mut ::std::os::raw::c_void,
                symbol: *const ::std::os::raw::c_void,
                count: usize,
                offset: usize,
                kind: cudaMemcpyKind,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGraphMemcpyNodeSetParamsFromSymbol = val;
        self
    }
    pub fn cudaGraphMemcpyNodeSetParams1D(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                node: cudaGraphNode_t,
                dst: *mut ::std::os::raw::c_void,
                src: *const ::std::os::raw::c_void,
                count: usize,
                kind: cudaMemcpyKind,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGraphMemcpyNodeSetParams1D = val;
        self
    }
    pub fn cudaGraphAddMemsetNode(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                pGraphNode: *mut cudaGraphNode_t,
                graph: cudaGraph_t,
                pDependencies: *const cudaGraphNode_t,
                numDependencies: usize,
                pMemsetParams: *const cudaMemsetParams,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGraphAddMemsetNode = val;
        self
    }
    pub fn cudaGraphMemsetNodeGetParams(
        mut self,
        val: Option<unsafe extern "C" fn(node: cudaGraphNode_t, pNodeParams: *mut cudaMemsetParams) -> cudaError_t>,
    ) -> Self {
        self.cudaGraphMemsetNodeGetParams = val;
        self
    }
    pub fn cudaGraphMemsetNodeSetParams(
        mut self,
        val: Option<unsafe extern "C" fn(node: cudaGraphNode_t, pNodeParams: *const cudaMemsetParams) -> cudaError_t>,
    ) -> Self {
        self.cudaGraphMemsetNodeSetParams = val;
        self
    }
    pub fn cudaGraphAddHostNode(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                pGraphNode: *mut cudaGraphNode_t,
                graph: cudaGraph_t,
                pDependencies: *const cudaGraphNode_t,
                numDependencies: usize,
                pNodeParams: *const cudaHostNodeParams,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGraphAddHostNode = val;
        self
    }
    pub fn cudaGraphHostNodeGetParams(
        mut self,
        val: Option<unsafe extern "C" fn(node: cudaGraphNode_t, pNodeParams: *mut cudaHostNodeParams) -> cudaError_t>,
    ) -> Self {
        self.cudaGraphHostNodeGetParams = val;
        self
    }
    pub fn cudaGraphHostNodeSetParams(
        mut self,
        val: Option<unsafe extern "C" fn(node: cudaGraphNode_t, pNodeParams: *const cudaHostNodeParams) -> cudaError_t>,
    ) -> Self {
        self.cudaGraphHostNodeSetParams = val;
        self
    }
    pub fn cudaGraphAddChildGraphNode(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                pGraphNode: *mut cudaGraphNode_t,
                graph: cudaGraph_t,
                pDependencies: *const cudaGraphNode_t,
                numDependencies: usize,
                childGraph: cudaGraph_t,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGraphAddChildGraphNode = val;
        self
    }
    pub fn cudaGraphChildGraphNodeGetGraph(
        mut self,
        val: Option<unsafe extern "C" fn(node: cudaGraphNode_t, pGraph: *mut cudaGraph_t) -> cudaError_t>,
    ) -> Self {
        self.cudaGraphChildGraphNodeGetGraph = val;
        self
    }
    pub fn cudaGraphAddEmptyNode(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                pGraphNode: *mut cudaGraphNode_t,
                graph: cudaGraph_t,
                pDependencies: *const cudaGraphNode_t,
                numDependencies: usize,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGraphAddEmptyNode = val;
        self
    }
    pub fn cudaGraphAddEventRecordNode(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                pGraphNode: *mut cudaGraphNode_t,
                graph: cudaGraph_t,
                pDependencies: *const cudaGraphNode_t,
                numDependencies: usize,
                event: cudaEvent_t,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGraphAddEventRecordNode = val;
        self
    }
    pub fn cudaGraphEventRecordNodeGetEvent(
        mut self,
        val: Option<unsafe extern "C" fn(node: cudaGraphNode_t, event_out: *mut cudaEvent_t) -> cudaError_t>,
    ) -> Self {
        self.cudaGraphEventRecordNodeGetEvent = val;
        self
    }
    pub fn cudaGraphEventRecordNodeSetEvent(
        mut self,
        val: Option<unsafe extern "C" fn(node: cudaGraphNode_t, event: cudaEvent_t) -> cudaError_t>,
    ) -> Self {
        self.cudaGraphEventRecordNodeSetEvent = val;
        self
    }
    pub fn cudaGraphAddEventWaitNode(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                pGraphNode: *mut cudaGraphNode_t,
                graph: cudaGraph_t,
                pDependencies: *const cudaGraphNode_t,
                numDependencies: usize,
                event: cudaEvent_t,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGraphAddEventWaitNode = val;
        self
    }
    pub fn cudaGraphEventWaitNodeGetEvent(
        mut self,
        val: Option<unsafe extern "C" fn(node: cudaGraphNode_t, event_out: *mut cudaEvent_t) -> cudaError_t>,
    ) -> Self {
        self.cudaGraphEventWaitNodeGetEvent = val;
        self
    }
    pub fn cudaGraphEventWaitNodeSetEvent(
        mut self,
        val: Option<unsafe extern "C" fn(node: cudaGraphNode_t, event: cudaEvent_t) -> cudaError_t>,
    ) -> Self {
        self.cudaGraphEventWaitNodeSetEvent = val;
        self
    }
    pub fn cudaGraphAddExternalSemaphoresSignalNode(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                pGraphNode: *mut cudaGraphNode_t,
                graph: cudaGraph_t,
                pDependencies: *const cudaGraphNode_t,
                numDependencies: usize,
                nodeParams: *const cudaExternalSemaphoreSignalNodeParams,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGraphAddExternalSemaphoresSignalNode = val;
        self
    }
    pub fn cudaGraphExternalSemaphoresSignalNodeGetParams(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                hNode: cudaGraphNode_t,
                params_out: *mut cudaExternalSemaphoreSignalNodeParams,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGraphExternalSemaphoresSignalNodeGetParams = val;
        self
    }
    pub fn cudaGraphExternalSemaphoresSignalNodeSetParams(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                hNode: cudaGraphNode_t,
                nodeParams: *const cudaExternalSemaphoreSignalNodeParams,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGraphExternalSemaphoresSignalNodeSetParams = val;
        self
    }
    pub fn cudaGraphAddExternalSemaphoresWaitNode(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                pGraphNode: *mut cudaGraphNode_t,
                graph: cudaGraph_t,
                pDependencies: *const cudaGraphNode_t,
                numDependencies: usize,
                nodeParams: *const cudaExternalSemaphoreWaitNodeParams,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGraphAddExternalSemaphoresWaitNode = val;
        self
    }
    pub fn cudaGraphExternalSemaphoresWaitNodeGetParams(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                hNode: cudaGraphNode_t,
                params_out: *mut cudaExternalSemaphoreWaitNodeParams,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGraphExternalSemaphoresWaitNodeGetParams = val;
        self
    }
    pub fn cudaGraphExternalSemaphoresWaitNodeSetParams(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                hNode: cudaGraphNode_t,
                nodeParams: *const cudaExternalSemaphoreWaitNodeParams,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGraphExternalSemaphoresWaitNodeSetParams = val;
        self
    }
    pub fn cudaGraphAddMemAllocNode(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                pGraphNode: *mut cudaGraphNode_t,
                graph: cudaGraph_t,
                pDependencies: *const cudaGraphNode_t,
                numDependencies: usize,
                nodeParams: *mut cudaMemAllocNodeParams,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGraphAddMemAllocNode = val;
        self
    }
    pub fn cudaGraphMemAllocNodeGetParams(
        mut self,
        val: Option<
            unsafe extern "C" fn(node: cudaGraphNode_t, params_out: *mut cudaMemAllocNodeParams) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGraphMemAllocNodeGetParams = val;
        self
    }
    pub fn cudaGraphAddMemFreeNode(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                pGraphNode: *mut cudaGraphNode_t,
                graph: cudaGraph_t,
                pDependencies: *const cudaGraphNode_t,
                numDependencies: usize,
                dptr: *mut ::std::os::raw::c_void,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGraphAddMemFreeNode = val;
        self
    }
    pub fn cudaGraphMemFreeNodeGetParams(
        mut self,
        val: Option<unsafe extern "C" fn(node: cudaGraphNode_t, dptr_out: *mut ::std::os::raw::c_void) -> cudaError_t>,
    ) -> Self {
        self.cudaGraphMemFreeNodeGetParams = val;
        self
    }
    pub fn cudaDeviceGraphMemTrim(
        mut self,
        val: Option<unsafe extern "C" fn(device: ::std::os::raw::c_int) -> cudaError_t>,
    ) -> Self {
        self.cudaDeviceGraphMemTrim = val;
        self
    }
    pub fn cudaDeviceGetGraphMemAttribute(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                device: ::std::os::raw::c_int,
                attr: cudaGraphMemAttributeType,
                value: *mut ::std::os::raw::c_void,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaDeviceGetGraphMemAttribute = val;
        self
    }
    pub fn cudaDeviceSetGraphMemAttribute(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                device: ::std::os::raw::c_int,
                attr: cudaGraphMemAttributeType,
                value: *mut ::std::os::raw::c_void,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaDeviceSetGraphMemAttribute = val;
        self
    }
    pub fn cudaGraphClone(
        mut self,
        val: Option<unsafe extern "C" fn(pGraphClone: *mut cudaGraph_t, originalGraph: cudaGraph_t) -> cudaError_t>,
    ) -> Self {
        self.cudaGraphClone = val;
        self
    }
    pub fn cudaGraphNodeFindInClone(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                pNode: *mut cudaGraphNode_t,
                originalNode: cudaGraphNode_t,
                clonedGraph: cudaGraph_t,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGraphNodeFindInClone = val;
        self
    }
    pub fn cudaGraphNodeGetType(
        mut self,
        val: Option<unsafe extern "C" fn(node: cudaGraphNode_t, pType: *mut cudaGraphNodeType) -> cudaError_t>,
    ) -> Self {
        self.cudaGraphNodeGetType = val;
        self
    }
    pub fn cudaGraphNodeGetContainingGraph(
        mut self,
        val: Option<unsafe extern "C" fn(hNode: cudaGraphNode_t, phGraph: *mut cudaGraph_t) -> cudaError_t>,
    ) -> Self {
        self.cudaGraphNodeGetContainingGraph = val;
        self
    }
    pub fn cudaGraphNodeGetLocalId(
        mut self,
        val: Option<unsafe extern "C" fn(hNode: cudaGraphNode_t, nodeId: *mut ::std::os::raw::c_uint) -> cudaError_t>,
    ) -> Self {
        self.cudaGraphNodeGetLocalId = val;
        self
    }
    pub fn cudaGraphNodeGetToolsId(
        mut self,
        val: Option<
            unsafe extern "C" fn(hNode: cudaGraphNode_t, toolsNodeId: *mut ::std::os::raw::c_ulonglong) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGraphNodeGetToolsId = val;
        self
    }
    pub fn cudaGraphGetId(
        mut self,
        val: Option<unsafe extern "C" fn(hGraph: cudaGraph_t, graphID: *mut ::std::os::raw::c_uint) -> cudaError_t>,
    ) -> Self {
        self.cudaGraphGetId = val;
        self
    }
    pub fn cudaGraphExecGetId(
        mut self,
        val: Option<
            unsafe extern "C" fn(hGraphExec: cudaGraphExec_t, graphID: *mut ::std::os::raw::c_uint) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGraphExecGetId = val;
        self
    }
    pub fn cudaGraphGetNodes(
        mut self,
        val: Option<
            unsafe extern "C" fn(graph: cudaGraph_t, nodes: *mut cudaGraphNode_t, numNodes: *mut usize) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGraphGetNodes = val;
        self
    }
    pub fn cudaGraphGetRootNodes(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                graph: cudaGraph_t,
                pRootNodes: *mut cudaGraphNode_t,
                pNumRootNodes: *mut usize,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGraphGetRootNodes = val;
        self
    }
    pub fn cudaGraphGetEdges(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                graph: cudaGraph_t,
                from: *mut cudaGraphNode_t,
                to: *mut cudaGraphNode_t,
                edgeData: *mut cudaGraphEdgeData,
                numEdges: *mut usize,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGraphGetEdges = val;
        self
    }
    pub fn cudaGraphNodeGetDependencies(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                node: cudaGraphNode_t,
                pDependencies: *mut cudaGraphNode_t,
                edgeData: *mut cudaGraphEdgeData,
                pNumDependencies: *mut usize,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGraphNodeGetDependencies = val;
        self
    }
    pub fn cudaGraphNodeGetDependentNodes(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                node: cudaGraphNode_t,
                pDependentNodes: *mut cudaGraphNode_t,
                edgeData: *mut cudaGraphEdgeData,
                pNumDependentNodes: *mut usize,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGraphNodeGetDependentNodes = val;
        self
    }
    pub fn cudaGraphAddDependencies(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                graph: cudaGraph_t,
                from: *const cudaGraphNode_t,
                to: *const cudaGraphNode_t,
                edgeData: *const cudaGraphEdgeData,
                numDependencies: usize,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGraphAddDependencies = val;
        self
    }
    pub fn cudaGraphRemoveDependencies(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                graph: cudaGraph_t,
                from: *const cudaGraphNode_t,
                to: *const cudaGraphNode_t,
                edgeData: *const cudaGraphEdgeData,
                numDependencies: usize,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGraphRemoveDependencies = val;
        self
    }
    pub fn cudaGraphDestroyNode(
        mut self,
        val: Option<unsafe extern "C" fn(node: cudaGraphNode_t) -> cudaError_t>,
    ) -> Self {
        self.cudaGraphDestroyNode = val;
        self
    }
    pub fn cudaGraphInstantiate(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                pGraphExec: *mut cudaGraphExec_t,
                graph: cudaGraph_t,
                flags: ::std::os::raw::c_ulonglong,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGraphInstantiate = val;
        self
    }
    pub fn cudaGraphInstantiateWithFlags(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                pGraphExec: *mut cudaGraphExec_t,
                graph: cudaGraph_t,
                flags: ::std::os::raw::c_ulonglong,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGraphInstantiateWithFlags = val;
        self
    }
    pub fn cudaGraphInstantiateWithParams(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                pGraphExec: *mut cudaGraphExec_t,
                graph: cudaGraph_t,
                instantiateParams: *mut cudaGraphInstantiateParams,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGraphInstantiateWithParams = val;
        self
    }
    pub fn cudaGraphExecGetFlags(
        mut self,
        val: Option<
            unsafe extern "C" fn(graphExec: cudaGraphExec_t, flags: *mut ::std::os::raw::c_ulonglong) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGraphExecGetFlags = val;
        self
    }
    pub fn cudaGraphExecKernelNodeSetParams(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                hGraphExec: cudaGraphExec_t,
                node: cudaGraphNode_t,
                pNodeParams: *const cudaKernelNodeParams,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGraphExecKernelNodeSetParams = val;
        self
    }
    pub fn cudaGraphExecMemcpyNodeSetParams(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                hGraphExec: cudaGraphExec_t,
                node: cudaGraphNode_t,
                pNodeParams: *const cudaMemcpy3DParms,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGraphExecMemcpyNodeSetParams = val;
        self
    }
    pub fn cudaGraphExecMemcpyNodeSetParamsToSymbol(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                hGraphExec: cudaGraphExec_t,
                node: cudaGraphNode_t,
                symbol: *const ::std::os::raw::c_void,
                src: *const ::std::os::raw::c_void,
                count: usize,
                offset: usize,
                kind: cudaMemcpyKind,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGraphExecMemcpyNodeSetParamsToSymbol = val;
        self
    }
    pub fn cudaGraphExecMemcpyNodeSetParamsFromSymbol(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                hGraphExec: cudaGraphExec_t,
                node: cudaGraphNode_t,
                dst: *mut ::std::os::raw::c_void,
                symbol: *const ::std::os::raw::c_void,
                count: usize,
                offset: usize,
                kind: cudaMemcpyKind,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGraphExecMemcpyNodeSetParamsFromSymbol = val;
        self
    }
    pub fn cudaGraphExecMemcpyNodeSetParams1D(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                hGraphExec: cudaGraphExec_t,
                node: cudaGraphNode_t,
                dst: *mut ::std::os::raw::c_void,
                src: *const ::std::os::raw::c_void,
                count: usize,
                kind: cudaMemcpyKind,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGraphExecMemcpyNodeSetParams1D = val;
        self
    }
    pub fn cudaGraphExecMemsetNodeSetParams(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                hGraphExec: cudaGraphExec_t,
                node: cudaGraphNode_t,
                pNodeParams: *const cudaMemsetParams,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGraphExecMemsetNodeSetParams = val;
        self
    }
    pub fn cudaGraphExecHostNodeSetParams(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                hGraphExec: cudaGraphExec_t,
                node: cudaGraphNode_t,
                pNodeParams: *const cudaHostNodeParams,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGraphExecHostNodeSetParams = val;
        self
    }
    pub fn cudaGraphExecChildGraphNodeSetParams(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                hGraphExec: cudaGraphExec_t,
                node: cudaGraphNode_t,
                childGraph: cudaGraph_t,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGraphExecChildGraphNodeSetParams = val;
        self
    }
    pub fn cudaGraphExecEventRecordNodeSetEvent(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                hGraphExec: cudaGraphExec_t,
                hNode: cudaGraphNode_t,
                event: cudaEvent_t,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGraphExecEventRecordNodeSetEvent = val;
        self
    }
    pub fn cudaGraphExecEventWaitNodeSetEvent(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                hGraphExec: cudaGraphExec_t,
                hNode: cudaGraphNode_t,
                event: cudaEvent_t,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGraphExecEventWaitNodeSetEvent = val;
        self
    }
    pub fn cudaGraphExecExternalSemaphoresSignalNodeSetParams(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                hGraphExec: cudaGraphExec_t,
                hNode: cudaGraphNode_t,
                nodeParams: *const cudaExternalSemaphoreSignalNodeParams,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGraphExecExternalSemaphoresSignalNodeSetParams = val;
        self
    }
    pub fn cudaGraphExecExternalSemaphoresWaitNodeSetParams(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                hGraphExec: cudaGraphExec_t,
                hNode: cudaGraphNode_t,
                nodeParams: *const cudaExternalSemaphoreWaitNodeParams,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGraphExecExternalSemaphoresWaitNodeSetParams = val;
        self
    }
    pub fn cudaGraphNodeSetEnabled(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                hGraphExec: cudaGraphExec_t,
                hNode: cudaGraphNode_t,
                isEnabled: ::std::os::raw::c_uint,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGraphNodeSetEnabled = val;
        self
    }
    pub fn cudaGraphNodeGetEnabled(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                hGraphExec: cudaGraphExec_t,
                hNode: cudaGraphNode_t,
                isEnabled: *mut ::std::os::raw::c_uint,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGraphNodeGetEnabled = val;
        self
    }
    pub fn cudaGraphExecUpdate(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                hGraphExec: cudaGraphExec_t,
                hGraph: cudaGraph_t,
                resultInfo: *mut cudaGraphExecUpdateResultInfo,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGraphExecUpdate = val;
        self
    }
    pub fn cudaGraphUpload(
        mut self,
        val: Option<unsafe extern "C" fn(graphExec: cudaGraphExec_t, stream: cudaStream_t) -> cudaError_t>,
    ) -> Self {
        self.cudaGraphUpload = val;
        self
    }
    pub fn cudaGraphLaunch(
        mut self,
        val: Option<unsafe extern "C" fn(graphExec: cudaGraphExec_t, stream: cudaStream_t) -> cudaError_t>,
    ) -> Self {
        self.cudaGraphLaunch = val;
        self
    }
    pub fn cudaGraphExecDestroy(
        mut self,
        val: Option<unsafe extern "C" fn(graphExec: cudaGraphExec_t) -> cudaError_t>,
    ) -> Self {
        self.cudaGraphExecDestroy = val;
        self
    }
    pub fn cudaGraphDestroy(mut self, val: Option<unsafe extern "C" fn(graph: cudaGraph_t) -> cudaError_t>) -> Self {
        self.cudaGraphDestroy = val;
        self
    }
    pub fn cudaGraphDebugDotPrint(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                graph: cudaGraph_t,
                path: *const ::std::os::raw::c_char,
                flags: ::std::os::raw::c_uint,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGraphDebugDotPrint = val;
        self
    }
    pub fn cudaUserObjectCreate(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                object_out: *mut cudaUserObject_t,
                ptr: *mut ::std::os::raw::c_void,
                destroy: cudaHostFn_t,
                initialRefcount: ::std::os::raw::c_uint,
                flags: ::std::os::raw::c_uint,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaUserObjectCreate = val;
        self
    }
    pub fn cudaUserObjectRetain(
        mut self,
        val: Option<unsafe extern "C" fn(object: cudaUserObject_t, count: ::std::os::raw::c_uint) -> cudaError_t>,
    ) -> Self {
        self.cudaUserObjectRetain = val;
        self
    }
    pub fn cudaUserObjectRelease(
        mut self,
        val: Option<unsafe extern "C" fn(object: cudaUserObject_t, count: ::std::os::raw::c_uint) -> cudaError_t>,
    ) -> Self {
        self.cudaUserObjectRelease = val;
        self
    }
    pub fn cudaGraphRetainUserObject(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                graph: cudaGraph_t,
                object: cudaUserObject_t,
                count: ::std::os::raw::c_uint,
                flags: ::std::os::raw::c_uint,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGraphRetainUserObject = val;
        self
    }
    pub fn cudaGraphReleaseUserObject(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                graph: cudaGraph_t,
                object: cudaUserObject_t,
                count: ::std::os::raw::c_uint,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGraphReleaseUserObject = val;
        self
    }
    pub fn cudaGraphAddNode(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                pGraphNode: *mut cudaGraphNode_t,
                graph: cudaGraph_t,
                pDependencies: *const cudaGraphNode_t,
                dependencyData: *const cudaGraphEdgeData,
                numDependencies: usize,
                nodeParams: *mut cudaGraphNodeParams,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGraphAddNode = val;
        self
    }
    pub fn cudaGraphNodeSetParams(
        mut self,
        val: Option<unsafe extern "C" fn(node: cudaGraphNode_t, nodeParams: *mut cudaGraphNodeParams) -> cudaError_t>,
    ) -> Self {
        self.cudaGraphNodeSetParams = val;
        self
    }
    pub fn cudaGraphNodeGetParams(
        mut self,
        val: Option<unsafe extern "C" fn(node: cudaGraphNode_t, nodeParams: *mut cudaGraphNodeParams) -> cudaError_t>,
    ) -> Self {
        self.cudaGraphNodeGetParams = val;
        self
    }
    pub fn cudaGraphExecNodeSetParams(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                graphExec: cudaGraphExec_t,
                node: cudaGraphNode_t,
                nodeParams: *mut cudaGraphNodeParams,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGraphExecNodeSetParams = val;
        self
    }
    pub fn cudaGraphConditionalHandleCreate(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                pHandle_out: *mut cudaGraphConditionalHandle,
                graph: cudaGraph_t,
                defaultLaunchValue: ::std::os::raw::c_uint,
                flags: ::std::os::raw::c_uint,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGraphConditionalHandleCreate = val;
        self
    }
    pub fn cudaGraphConditionalHandleCreate_v2(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                pHandle_out: *mut cudaGraphConditionalHandle,
                graph: cudaGraph_t,
                ctx: cudaExecutionContext_t,
                defaultLaunchValue: ::std::os::raw::c_uint,
                flags: ::std::os::raw::c_uint,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGraphConditionalHandleCreate_v2 = val;
        self
    }
    pub fn cudaGetDriverEntryPoint(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                symbol: *const ::std::os::raw::c_char,
                funcPtr: *mut *mut ::std::os::raw::c_void,
                flags: ::std::os::raw::c_ulonglong,
                driverStatus: *mut cudaDriverEntryPointQueryResult,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGetDriverEntryPoint = val;
        self
    }
    pub fn cudaGetDriverEntryPointByVersion(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                symbol: *const ::std::os::raw::c_char,
                funcPtr: *mut *mut ::std::os::raw::c_void,
                cudaVersion: ::std::os::raw::c_uint,
                flags: ::std::os::raw::c_ulonglong,
                driverStatus: *mut cudaDriverEntryPointQueryResult,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGetDriverEntryPointByVersion = val;
        self
    }
    pub fn cudaLibraryLoadData(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                library: *mut cudaLibrary_t,
                code: *const ::std::os::raw::c_void,
                jitOptions: *mut cudaJitOption,
                jitOptionsValues: *mut *mut ::std::os::raw::c_void,
                numJitOptions: ::std::os::raw::c_uint,
                libraryOptions: *mut cudaLibraryOption,
                libraryOptionValues: *mut *mut ::std::os::raw::c_void,
                numLibraryOptions: ::std::os::raw::c_uint,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaLibraryLoadData = val;
        self
    }
    pub fn cudaLibraryLoadFromFile(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                library: *mut cudaLibrary_t,
                fileName: *const ::std::os::raw::c_char,
                jitOptions: *mut cudaJitOption,
                jitOptionsValues: *mut *mut ::std::os::raw::c_void,
                numJitOptions: ::std::os::raw::c_uint,
                libraryOptions: *mut cudaLibraryOption,
                libraryOptionValues: *mut *mut ::std::os::raw::c_void,
                numLibraryOptions: ::std::os::raw::c_uint,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaLibraryLoadFromFile = val;
        self
    }
    pub fn cudaLibraryUnload(
        mut self,
        val: Option<unsafe extern "C" fn(library: cudaLibrary_t) -> cudaError_t>,
    ) -> Self {
        self.cudaLibraryUnload = val;
        self
    }
    pub fn cudaLibraryGetKernel(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                pKernel: *mut cudaKernel_t,
                library: cudaLibrary_t,
                name: *const ::std::os::raw::c_char,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaLibraryGetKernel = val;
        self
    }
    pub fn cudaLibraryGetGlobal(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                dptr: *mut *mut ::std::os::raw::c_void,
                bytes: *mut usize,
                library: cudaLibrary_t,
                name: *const ::std::os::raw::c_char,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaLibraryGetGlobal = val;
        self
    }
    pub fn cudaLibraryGetManaged(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                dptr: *mut *mut ::std::os::raw::c_void,
                bytes: *mut usize,
                library: cudaLibrary_t,
                name: *const ::std::os::raw::c_char,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaLibraryGetManaged = val;
        self
    }
    pub fn cudaLibraryGetUnifiedFunction(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                fptr: *mut *mut ::std::os::raw::c_void,
                library: cudaLibrary_t,
                symbol: *const ::std::os::raw::c_char,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaLibraryGetUnifiedFunction = val;
        self
    }
    pub fn cudaLibraryGetKernelCount(
        mut self,
        val: Option<unsafe extern "C" fn(count: *mut ::std::os::raw::c_uint, lib: cudaLibrary_t) -> cudaError_t>,
    ) -> Self {
        self.cudaLibraryGetKernelCount = val;
        self
    }
    pub fn cudaLibraryEnumerateKernels(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                kernels: *mut cudaKernel_t,
                numKernels: ::std::os::raw::c_uint,
                lib: cudaLibrary_t,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaLibraryEnumerateKernels = val;
        self
    }
    pub fn cudaKernelSetAttributeForDevice(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                kernel: cudaKernel_t,
                attr: cudaFuncAttribute,
                value: ::std::os::raw::c_int,
                device: ::std::os::raw::c_int,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaKernelSetAttributeForDevice = val;
        self
    }
    pub fn cudaDeviceGetDevResource(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                device: ::std::os::raw::c_int,
                resource: *mut cudaDevResource,
                type_: cudaDevResourceType,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaDeviceGetDevResource = val;
        self
    }
    pub fn cudaDevSmResourceSplitByCount(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                result: *mut cudaDevResource,
                nbGroups: *mut ::std::os::raw::c_uint,
                input: *const cudaDevResource,
                remaining: *mut cudaDevResource,
                flags: ::std::os::raw::c_uint,
                minCount: ::std::os::raw::c_uint,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaDevSmResourceSplitByCount = val;
        self
    }
    pub fn cudaDevSmResourceSplit(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                result: *mut cudaDevResource,
                nbGroups: ::std::os::raw::c_uint,
                input: *const cudaDevResource,
                remainder: *mut cudaDevResource,
                flags: ::std::os::raw::c_uint,
                groupParams: *mut cudaDevSmResourceGroupParams,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaDevSmResourceSplit = val;
        self
    }
    pub fn cudaDevResourceGenerateDesc(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                phDesc: *mut cudaDevResourceDesc_t,
                resources: *mut cudaDevResource,
                nbResources: ::std::os::raw::c_uint,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaDevResourceGenerateDesc = val;
        self
    }
    pub fn cudaGreenCtxCreate(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                phCtx: *mut cudaExecutionContext_t,
                desc: cudaDevResourceDesc_t,
                device: ::std::os::raw::c_int,
                flags: ::std::os::raw::c_uint,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGreenCtxCreate = val;
        self
    }
    pub fn cudaExecutionCtxDestroy(
        mut self,
        val: Option<unsafe extern "C" fn(ctx: cudaExecutionContext_t) -> cudaError_t>,
    ) -> Self {
        self.cudaExecutionCtxDestroy = val;
        self
    }
    pub fn cudaExecutionCtxGetDevResource(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                ctx: cudaExecutionContext_t,
                resource: *mut cudaDevResource,
                type_: cudaDevResourceType,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaExecutionCtxGetDevResource = val;
        self
    }
    pub fn cudaExecutionCtxGetDevice(
        mut self,
        val: Option<
            unsafe extern "C" fn(device: *mut ::std::os::raw::c_int, ctx: cudaExecutionContext_t) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaExecutionCtxGetDevice = val;
        self
    }
    pub fn cudaExecutionCtxGetId(
        mut self,
        val: Option<
            unsafe extern "C" fn(ctx: cudaExecutionContext_t, ctxId: *mut ::std::os::raw::c_ulonglong) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaExecutionCtxGetId = val;
        self
    }
    pub fn cudaExecutionCtxStreamCreate(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                phStream: *mut cudaStream_t,
                ctx: cudaExecutionContext_t,
                flags: ::std::os::raw::c_uint,
                priority: ::std::os::raw::c_int,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaExecutionCtxStreamCreate = val;
        self
    }
    pub fn cudaExecutionCtxSynchronize(
        mut self,
        val: Option<unsafe extern "C" fn(ctx: cudaExecutionContext_t) -> cudaError_t>,
    ) -> Self {
        self.cudaExecutionCtxSynchronize = val;
        self
    }
    pub fn cudaStreamGetDevResource(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                hStream: cudaStream_t,
                resource: *mut cudaDevResource,
                type_: cudaDevResourceType,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaStreamGetDevResource = val;
        self
    }
    pub fn cudaExecutionCtxRecordEvent(
        mut self,
        val: Option<unsafe extern "C" fn(ctx: cudaExecutionContext_t, event: cudaEvent_t) -> cudaError_t>,
    ) -> Self {
        self.cudaExecutionCtxRecordEvent = val;
        self
    }
    pub fn cudaExecutionCtxWaitEvent(
        mut self,
        val: Option<unsafe extern "C" fn(ctx: cudaExecutionContext_t, event: cudaEvent_t) -> cudaError_t>,
    ) -> Self {
        self.cudaExecutionCtxWaitEvent = val;
        self
    }
    pub fn cudaDeviceGetExecutionCtx(
        mut self,
        val: Option<
            unsafe extern "C" fn(ctx: *mut cudaExecutionContext_t, device: ::std::os::raw::c_int) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaDeviceGetExecutionCtx = val;
        self
    }
    pub fn cudaGetExportTable(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                ppExportTable: *mut *const ::std::os::raw::c_void,
                pExportTableId: *const cudaUUID_t,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGetExportTable = val;
        self
    }
    pub fn cudaGetFuncBySymbol(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                functionPtr: *mut cudaFunction_t,
                symbolPtr: *const ::std::os::raw::c_void,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGetFuncBySymbol = val;
        self
    }
    pub fn cudaGetKernel(
        mut self,
        val: Option<
            unsafe extern "C" fn(
                kernelPtr: *mut cudaKernel_t,
                entryFuncAddr: *const ::std::os::raw::c_void,
            ) -> cudaError_t,
        >,
    ) -> Self {
        self.cudaGetKernel = val;
        self
    }
}
pub struct CudaExecutionContext(pub(crate) crate::sys::cudaExecutionContext_t);
impl CudaExecutionContext {
    #[doc = "Get context resources\nGet the `type` resources available to context represented by `ctx.`\nNote: The API is not supported on 32-bit platforms.\n\n# Arguments\n\n* `ctx` - - Execution context to get resource for (required parameter, see note below)\n* `resource` - - Output pointer to a cudaDevResource structure\n* `type` - - Type of resource to retrieve\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorNotSupported,\n::cudaErrorNotPermitted,\n::cudaErrorCudartUnloading,\n::cudaErrorInitializationError\n\\notefnerr \\note_callback \\note_cudaExecutionContext_t_required_param # See also\n\n> [`::cudaDeviceGetDevResource,`]\n::cudaDevSmResourceSplit,\n::cudaDevResourceGenerateDesc,\n::cudaGreenCtxCreate"]
    pub unsafe fn cudaExecutionCtxGetDevResource(
        &self,
        type_: cudaDevResourceType,
    ) -> Result<cudaDevResource, crate::sys::cudaError> {
        let mut out_1: std::mem::MaybeUninit<cudaDevResource> = std::mem::MaybeUninit::uninit();
        let status = unsafe { crate::sys::cudaExecutionCtxGetDevResource(self.0, out_1.as_mut_ptr() as *mut _, type_) };
        if status as usize == crate::sys::cudaError::cudaSuccess as usize {
            unsafe { Ok(out_1.assume_init() as cudaDevResource) }
        } else {
            Err(unsafe { std::mem::transmute(status) })
        }
    }
    #[doc = "Returns the unique Id associated with the execution context supplied\nReturns in `ctxId` the unique Id which is associated with a given context.\nThe Id is unique for the life of the program for this instance of CUDA.\nThe execution context should not be NULL.\n\n# Arguments\n\n* `ctx` - - Context for which to obtain the Id (required parameter, see note below)\n* `ctxId` - - Pointer to store the Id of the context\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorCudartUnloading,\n::cudaErrorInitializationError,\n::cudaErrorInvalidValue,\n::cudaErrorNotPermitted\n\\notefnerr \\note_callback \\note_cudaExecutionContext_t_required_param # See also\n\n> [`::cudaGreenCtxCreate,`]\n::cudaExecutionCtxDestroy,\n::cudaExecutionCtxGetDevice,\n::cuCtxGetId"]
    pub unsafe fn cudaExecutionCtxGetId(&self) -> Result<u64, crate::sys::cudaError> {
        let mut out_1: std::mem::MaybeUninit<::std::os::raw::c_ulonglong> = std::mem::MaybeUninit::uninit();
        let status = unsafe { crate::sys::cudaExecutionCtxGetId(self.0, out_1.as_mut_ptr() as *mut _) };
        if status as usize == crate::sys::cudaError::cudaSuccess as usize {
            unsafe { Ok(out_1.assume_init() as u64) }
        } else {
            Err(unsafe { std::mem::transmute(status) })
        }
    }
    #[doc = "Block for the specified execution context's tasks to complete\nBlocks until the specified execution context has completed all preceding requested tasks.\nIf the specified execution context is the device (primary) context obtained via ::cudaDeviceGetExecutionCtx,\ngreen contexts that have been created on the device will also be synchronized.\nThe API returns an error if one of the preceding tasks failed.\n\n# Arguments\n\n* `ctx` - - Execution context to synchronize (required parameter, see note below)\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorCudartUnloading,\n::cudaErrorDeviceUninitialized,\n::cudaErrorInvalidValue\n\\notefnerr \\note_callback \\note_cudaExecutionContext_t_required_param # See also\n\n> [`::cudaGreenCtxCreate,`]\n::cudaExecutionCtxDestroy,\n::cudaDeviceSynchronize,\n::cuCtxSynchronize_v2"]
    pub unsafe fn cudaExecutionCtxSynchronize(&self) -> Result<(), crate::sys::cudaError> {
        let status = unsafe { crate::sys::cudaExecutionCtxSynchronize(self.0) };
        if status == crate::sys::cudaError::cudaSuccess {
            Ok(())
        } else {
            Err(status)
        }
    }
    #[doc = "Records an event for the specified execution context\nCaptures in `event` all the activities of the execution context `ctx`\nat the time of this call. `event` and `ctx` must be from the same\nCUDA device, otherwise ::cudaErrorInvalidHandle will be returned.\nCalls such as ::cudaEventQuery() or ::cudaExecutionCtxWaitEvent() will then examine\nor wait for completion of the work that was captured.\nUses of `ctx` after this call do not modify `event.`\nIf the execution context passed to `ctx` is the device (primary) context obtained via\n::cudaDeviceGetExecutionCtx(), `event` will capture all the activities of the green\ncontexts created on the device as well.\n> **Note** The API will return ::cudaErrorStreamCaptureUnsupported if the\nspecified execution context `ctx` has a stream in the capture mode. In such a case,\nthe call will invalidate all the conflicting captures.\n\n# Arguments\n\n* `ctx` - - Execution context to record event for (required parameter, see note below)\n* `event` - - Event to record\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorCudartUnloading,\n::cudaErrorInitializationError,\n::cudaErrorInvalidHandle,\n::cudaErrorStreamCaptureUnsupported\n\\notefnerr \\note_callback \\note_cudaExecutionContext_t_required_param # See also\n\n> [`::cudaEventRecord,`]\n::cudaExecutionCtxWaitEvent,\n::cuCtxRecordEvent,\n::cuGreenCtxRecordEvent"]
    pub unsafe fn cudaExecutionCtxRecordEvent(&self, event: cudaEvent_t) -> Result<(), crate::sys::cudaError> {
        let status = unsafe { crate::sys::cudaExecutionCtxRecordEvent(self.0, event) };
        if status == crate::sys::cudaError::cudaSuccess {
            Ok(())
        } else {
            Err(status)
        }
    }
    #[doc = "Make an execution context wait on an event\nMakes all future work submitted to execution context `ctx` wait for all work\ncaptured in `event.` The synchronization will be performed on the device\nand will not block the calling CPU thread. See ::cudaExecutionCtxRecordEvent()\nfor details on what is captured by an event.\nIf the execution context passed to `ctx` is the device (primary) context obtained via\n::cudaDeviceGetExecutionCtx(), all green contexts created on the device will wait for\n`event` as well.\n> **Note** `event` may be from a different execution context or device than `ctx.`\n> **Note** The API will return ::cudaErrorStreamCaptureUnsupported and\ninvalidate the capture if the specified event `event` is part of an ongoing\ncapture sequence or if the specified execution context `ctx` has a stream in the capture mode.\n\n# Arguments\n\n* `ctx` -    - Execution context to wait for (required parameter, see note below)\n* `event` -  - Event to wait on\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorCudartUnloading,\n::cudaErrorInitializationError,\n::cudaErrorInvalidHandle,\n::cudaErrorStreamCaptureUnsupported\n\\notefnerr \\note_callback \\note_cudaExecutionContext_t_required_param # See also\n\n> [`::cudaExecutionCtxRecordEvent,`]\n::cudaStreamWaitEvent,\n::cuCtxWaitEvent,\n::cuGreenCtxWaitEvent"]
    pub unsafe fn cudaExecutionCtxWaitEvent(&self, event: cudaEvent_t) -> Result<(), crate::sys::cudaError> {
        let status = unsafe { crate::sys::cudaExecutionCtxWaitEvent(self.0, event) };
        if status == crate::sys::cudaError::cudaSuccess {
            Ok(())
        } else {
            Err(status)
        }
    }
}
#[doc = "Destroy all allocations and reset all state on the current device\nin the current process.\nExplicitly destroys and cleans up all resources associated with the current\ndevice in the current process. It is the caller's responsibility to ensure\nthat the resources are not accessed or passed in subsequent API calls and\ndoing so will result in undefined behavior. These resources include CUDA types\n::cudaStream_t, ::cudaEvent_t, ::cudaArray_t, ::cudaMipmappedArray_t, ::cudaPitchedPtr,\n::cudaTextureObject_t, ::cudaSurfaceObject_t, ::textureReference, ::surfaceReference,\n::cudaExternalMemory_t, ::cudaExternalSemaphore_t and ::cudaGraphicsResource_t.\nThese resources also include memory allocations by ::cudaMalloc, ::cudaMallocHost,\n::cudaMallocManaged and ::cudaMallocPitch.\nAny subsequent API call to this device will reinitialize the device.\nNote that this function will reset the device immediately.  It is the caller's\nresponsibility to ensure that the device is not being accessed by any\nother host threads from the process when this function is called.\n> **Note** ::cudaDeviceReset() will not destroy memory allocations by ::cudaMallocAsync() and\n::cudaMallocFromPoolAsync(). These memory allocations need to be destroyed explicitly.\n> **Note** If a non-primary ::CUcontext is current to the thread, ::cudaDeviceReset()\nwill destroy only the internal CUDA RT state for that ::CUcontext.\n\n# Returns\n\n::cudaSuccess\n\\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaDeviceSynchronize`]"]
pub unsafe fn cudaDeviceReset() -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaDeviceReset() };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Wait for compute device to finish\nBlocks until the device has completed all preceding requested tasks.\n::cudaDeviceSynchronize() returns an error if one of the preceding tasks\nhas failed. If the ::cudaDeviceScheduleBlockingSync flag was set for\nthis device, the host thread will block until the device has finished\nits work.\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorStreamCaptureUnsupported\n\\note_device_sync_deprecated \\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaDeviceReset,`]\n::cuCtxSynchronize"]
pub unsafe fn cudaDeviceSynchronize() -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaDeviceSynchronize() };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Set resource limits\nSetting `limit` to `value` is a request by the application to update\nthe current limit maintained by the device.  The driver is free to\nmodify the requested value to meet h/w requirements (this could be\nclamping to minimum or maximum values, rounding up to nearest element\nsize, etc).  The application can use ::cudaDeviceGetLimit() to find out\nexactly what the limit has been set to.\nSetting each ::cudaLimit has its own specific restrictions, so each is\ndiscussed here.\n- ::cudaLimitStackSize controls the stack size in bytes of each GPU thread.\n- ::cudaLimitPrintfFifoSize controls the size in bytes of the shared FIFO\nused by the ::printf() device system call. Setting\n::cudaLimitPrintfFifoSize must not be performed after launching any kernel\nthat uses the ::printf() device system call - in such case\n::cudaErrorInvalidValue will be returned.\n- ::cudaLimitMallocHeapSize controls the size in bytes of the heap used by\nthe ::malloc() and ::free() device system calls. Setting\n::cudaLimitMallocHeapSize must not be performed after launching any kernel\nthat uses the ::malloc() or ::free() device system calls - in such case\n::cudaErrorInvalidValue will be returned.\n- ::cudaLimitDevRuntimeSyncDepth controls the maximum nesting depth of a\ngrid at which a thread can safely call ::cudaDeviceSynchronize(). Setting\nthis limit must be performed before any launch of a kernel that uses the\ndevice runtime and calls ::cudaDeviceSynchronize() above the default sync\ndepth, two levels of grids. Calls to ::cudaDeviceSynchronize() will fail\nwith error code ::cudaErrorSyncDepthExceeded if the limitation is\nviolated. This limit can be set smaller than the default or up the maximum\nlaunch depth of 24. When setting this limit, keep in mind that additional\nlevels of sync depth require the runtime to reserve large amounts of\ndevice memory which can no longer be used for user allocations. If these\nreservations of device memory fail, ::cudaDeviceSetLimit will return\n::cudaErrorMemoryAllocation, and the limit can be reset to a lower value.\nThis limit is only applicable to devices of compute capability < 9.0.\nAttempting to set this limit on devices of other compute capability will\nresults in error ::cudaErrorUnsupportedLimit being returned.\n- ::cudaLimitDevRuntimePendingLaunchCount controls the maximum number of\noutstanding device runtime launches that can be made from the current\ndevice. A grid is outstanding from the point of launch up until the grid\nis known to have been completed. Device runtime launches which violate\nthis limitation fail and return ::cudaErrorLaunchPendingCountExceeded when\n::cudaGetLastError() is called after launch. If more pending launches than\nthe default (2048 launches) are needed for a module using the device\nruntime, this limit can be increased. Keep in mind that being able to\nsustain additional pending launches will require the runtime to reserve\nlarger amounts of device memory upfront which can no longer be used for\nallocations. If these reservations fail, ::cudaDeviceSetLimit will return\n::cudaErrorMemoryAllocation, and the limit can be reset to a lower value.\nThis limit is only applicable to devices of compute capability 3.5 and\nhigher. Attempting to set this limit on devices of compute capability less\nthan 3.5 will result in the error ::cudaErrorUnsupportedLimit being\nreturned.\n- ::cudaLimitMaxL2FetchGranularity controls the L2 cache fetch granularity.\nValues can range from 0B to 128B. This is purely a performance hint and\nit can be ignored or clamped depending on the platform.\n- ::cudaLimitPersistingL2CacheSize controls size in bytes available\nfor persisting L2 cache. This is purely a performance hint and it\ncan be ignored or clamped depending on the platform.\n\n# Arguments\n\n* `limit` - - Limit to set\n* `value` - - Size of limit\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorUnsupportedLimit,\n::cudaErrorInvalidValue,\n::cudaErrorMemoryAllocation\n\\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaDeviceGetLimit,`]\n::cuCtxSetLimit"]
pub unsafe fn cudaDeviceSetLimit(limit: cudaLimit, value: usize) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaDeviceSetLimit(limit, value) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Return resource limits\nReturns in `*pValue` the current size of `limit.` The following ::cudaLimit values are supported.\n- ::cudaLimitStackSize is the stack size in bytes of each GPU thread.\n- ::cudaLimitPrintfFifoSize is the size in bytes of the shared FIFO used by the\n::printf() device system call.\n- ::cudaLimitMallocHeapSize is the size in bytes of the heap used by the\n::malloc() and ::free() device system calls.\n- ::cudaLimitDevRuntimeSyncDepth is the maximum grid depth at which a\nthread can isssue the device runtime call ::cudaDeviceSynchronize()\nto wait on child grid launches to complete. This functionality is removed\nfor devices of compute capability >= 9.0, and hence will return error\n::cudaErrorUnsupportedLimit on such devices.\n- ::cudaLimitDevRuntimePendingLaunchCount is the maximum number of outstanding\ndevice runtime launches.\n- ::cudaLimitMaxL2FetchGranularity is the L2 cache fetch granularity.\n- ::cudaLimitPersistingL2CacheSize is the persisting L2 cache size in bytes.\n\n# Arguments\n\n* `limit` -  - Limit to query\n* `pValue` - - Returned size of the limit\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorUnsupportedLimit,\n::cudaErrorInvalidValue\n\\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaDeviceSetLimit,`]\n::cuCtxGetLimit"]
pub unsafe fn cudaDeviceGetLimit(limit: cudaLimit) -> Result<usize, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaDeviceGetLimit(out_0.as_mut_ptr() as *mut _, limit) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_0.assume_init() as usize) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudaDeviceGetTexture1DLinearMaxWidth(
    fmtDesc: *const cudaChannelFormatDesc,
    device: i32,
) -> Result<usize, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let status =
        unsafe { crate::sys::cudaDeviceGetTexture1DLinearMaxWidth(out_0.as_mut_ptr() as *mut _, fmtDesc, device as _) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_0.assume_init() as usize) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Returns the preferred cache configuration for the current device.\nOn devices where the L1 cache and shared memory use the same hardware\nresources, this returns through `pCacheConfig` the preferred cache\nconfiguration for the current device. This is only a preference. The\nruntime will use the requested configuration if possible, but it is free to\nchoose a different configuration if required to execute functions.\nThis will return a `pCacheConfig` of ::cudaFuncCachePreferNone on devices\nwhere the size of the L1 cache and shared memory are fixed.\nThe supported cache configurations are:\n- ::cudaFuncCachePreferNone: no preference for shared memory or L1 (default)\n- ::cudaFuncCachePreferShared: prefer larger shared memory and smaller L1 cache\n- ::cudaFuncCachePreferL1: prefer larger L1 cache and smaller shared memory\n- ::cudaFuncCachePreferEqual: prefer equal size L1 cache and shared memory\n\n# Arguments\n\n* `pCacheConfig` - - Returned cache configuration\n\n# Returns\n\n::cudaSuccess\n\\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaDeviceSetCacheConfig,`]\n[`::cudaFuncSetCacheConfig(const`] void*, enum cudaFuncCache) \"cudaFuncSetCacheConfig (C API)\",\n[`::cudaFuncSetCacheConfig(T*,`] enum cudaFuncCache) \"cudaFuncSetCacheConfig (C++ API)\",\n::cuCtxGetCacheConfig"]
pub unsafe fn cudaDeviceGetCacheConfig() -> Result<cudaFuncCache, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<cudaFuncCache> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaDeviceGetCacheConfig(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_0.assume_init() as cudaFuncCache) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Returns numerical values that correspond to the least and\ngreatest stream priorities.\nReturns in `*leastPriority` and `*greatestPriority` the numerical values that correspond\nto the least and greatest stream priorities respectively. Stream priorities\nfollow a convention where lower numbers imply greater priorities. The range of\nmeaningful stream priorities is given by [`*greatestPriority,` `*leastPriority].`\nIf the user attempts to create a stream with a priority value that is\noutside the the meaningful range as specified by this API, the priority is\nautomatically clamped down or up to either `*leastPriority` or `*greatestPriority`\nrespectively. See ::cudaStreamCreateWithPriority for details on creating a\npriority stream.\nA NULL may be passed in for `*leastPriority` or `*greatestPriority` if the value\nis not desired.\nThis function will return '0' in both `*leastPriority` and `*greatestPriority` if\nthe current context's device does not support stream priorities\n(see ::cudaDeviceGetAttribute).\n\n# Arguments\n\n* `leastPriority` -    - Pointer to an int in which the numerical value for least\nstream priority is returned\n* `greatestPriority` - - Pointer to an int in which the numerical value for greatest\nstream priority is returned\n\n# Returns\n\n::cudaSuccess\n\\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaStreamCreateWithPriority,`]\n::cudaStreamGetPriority,\n::cuCtxGetStreamPriorityRange"]
pub unsafe fn cudaDeviceGetStreamPriorityRange() -> Result<(i32, i32), crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_1: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudaDeviceGetStreamPriorityRange(out_0.as_mut_ptr() as *mut _, out_1.as_mut_ptr() as *mut _)
    };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok((out_0.assume_init() as i32, out_1.assume_init() as i32)) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Sets the preferred cache configuration for the current device.\nOn devices where the L1 cache and shared memory use the same hardware\nresources, this sets through `cacheConfig` the preferred cache\nconfiguration for the current device. This is only a preference. The\nruntime will use the requested configuration if possible, but it is free to\nchoose a different configuration if required to execute the function. Any\nfunction preference set via\n[`::cudaFuncSetCacheConfig(const`] void*, enum cudaFuncCache) \"cudaFuncSetCacheConfig (C API)\"\nor\n[`::cudaFuncSetCacheConfig(T*,`] enum cudaFuncCache) \"cudaFuncSetCacheConfig (C++ API)\"\nwill be preferred over this device-wide setting. Setting the device-wide\ncache configuration to ::cudaFuncCachePreferNone will cause subsequent\nkernel launches to prefer to not change the cache configuration unless\nrequired to launch the kernel.\nThis setting does nothing on devices where the size of the L1 cache and\nshared memory are fixed.\nLaunching a kernel with a different preference than the most recent\npreference setting may insert a device-side synchronization point.\nThe supported cache configurations are:\n- ::cudaFuncCachePreferNone: no preference for shared memory or L1 (default)\n- ::cudaFuncCachePreferShared: prefer larger shared memory and smaller L1 cache\n- ::cudaFuncCachePreferL1: prefer larger L1 cache and smaller shared memory\n- ::cudaFuncCachePreferEqual: prefer equal size L1 cache and shared memory\n\n# Arguments\n\n* `cacheConfig` - - Requested cache configuration\n\n# Returns\n\n::cudaSuccess\n\\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaDeviceGetCacheConfig,`]\n[`::cudaFuncSetCacheConfig(const`] void*, enum cudaFuncCache) \"cudaFuncSetCacheConfig (C API)\",\n[`::cudaFuncSetCacheConfig(T*,`] enum cudaFuncCache) \"cudaFuncSetCacheConfig (C++ API)\",\n::cuCtxSetCacheConfig"]
pub unsafe fn cudaDeviceSetCacheConfig(cacheConfig: cudaFuncCache) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaDeviceSetCacheConfig(cacheConfig) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Returns a handle to a compute device\nReturns in `*device` a device ordinal given a PCI bus ID string.\n\n# Arguments\n\n* `device` -   - Returned device ordinal\n* `pciBusId` - - String in one of the following forms:\n[domain]:[bus]:[device].[function]\n[domain]:[bus]:[device]\n[bus]:[device].[function]\nwhere `domain,` `bus,` `device,` and `function` are all hexadecimal values\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorInvalidDevice\n\\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaDeviceGetPCIBusId,`]\n::cuDeviceGetByPCIBusId"]
pub unsafe fn cudaDeviceGetByPCIBusId(pciBusId: *const ::std::os::raw::c_char) -> Result<i32, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaDeviceGetByPCIBusId(out_0.as_mut_ptr() as *mut _, pciBusId) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_0.assume_init() as i32) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Returns a PCI Bus Id string for the device\nReturns an ASCII string identifying the device `dev` in the NULL-terminated\nstring pointed to by `pciBusId.` `len` specifies the maximum length of the\nstring that may be returned.\n\n# Arguments\n\n* `pciBusId` - - Returned identifier string for the device in the following format\n[domain]:[bus]:[device].[function]\nwhere `domain,` `bus,` `device,` and `function` are all hexadecimal values.\npciBusId should be large enough to store 13 characters including the NULL-terminator.\n* `len` -      - Maximum length of string to store in `name`\n* `device` -   - Device to get identifier string for\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorInvalidDevice\n\\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaDeviceGetByPCIBusId,`]\n::cuDeviceGetPCIBusId"]
pub unsafe fn cudaDeviceGetPCIBusId(len: i32, device: i32) -> Result<i8, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<::std::os::raw::c_char> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaDeviceGetPCIBusId(out_0.as_mut_ptr() as *mut _, len as _, device as _) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_0.assume_init() as i8) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Gets an interprocess handle for a previously allocated event\nTakes as input a previously allocated event. This event must have been\ncreated with the ::cudaEventInterprocess and ::cudaEventDisableTiming\nflags set. This opaque handle may be copied into other processes and\nopened with ::cudaIpcOpenEventHandle to allow efficient hardware\nsynchronization between GPU work in different processes.\nAfter the event has been been opened in the importing process,\n::cudaEventRecord, ::cudaEventSynchronize, ::cudaStreamWaitEvent and\n::cudaEventQuery may be used in either process. Performing operations\non the imported event after the exported event has been freed\nwith ::cudaEventDestroy will result in undefined behavior.\nIPC functionality is restricted to devices with support for unified\naddressing on Linux and Windows operating systems.\nIPC functionality on Windows is supported for compatibility purposes\nbut not recommended as it comes with performance cost.\nUsers can test their device for IPC functionality by calling\n::cudaDeviceGetAttribute with ::cudaDevAttrIpcEventSupport\n\n# Arguments\n\n* `handle` - - Pointer to a user allocated cudaIpcEventHandle\nin which to return the opaque event handle\n* `event` -   - Event allocated with ::cudaEventInterprocess and\n::cudaEventDisableTiming flags.\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidResourceHandle,\n::cudaErrorMemoryAllocation,\n::cudaErrorMapBufferObjectFailed,\n::cudaErrorNotSupported,\n::cudaErrorInvalidValue\n\\note_init_rt \\note_callback # See also\n\n> [`::cudaEventCreate,`]\n::cudaEventDestroy,\n::cudaEventSynchronize,\n::cudaEventQuery,\n::cudaStreamWaitEvent,\n::cudaIpcOpenEventHandle,\n::cudaIpcGetMemHandle,\n::cudaIpcOpenMemHandle,\n::cudaIpcCloseMemHandle,\n::cuIpcGetEventHandle"]
pub unsafe fn cudaIpcGetEventHandle(event: cudaEvent_t) -> Result<cudaIpcEventHandle_t, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<cudaIpcEventHandle_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaIpcGetEventHandle(out_0.as_mut_ptr() as *mut _, event) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_0.assume_init() as cudaIpcEventHandle_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Opens an interprocess event handle for use in the current process\nOpens an interprocess event handle exported from another process with\n::cudaIpcGetEventHandle. This function returns a ::cudaEvent_t that behaves like\na locally created event with the ::cudaEventDisableTiming flag specified.\nThis event must be freed with ::cudaEventDestroy.\nPerforming operations on the imported event after the exported event has\nbeen freed with ::cudaEventDestroy will result in undefined behavior.\nIPC functionality is restricted to devices with support for unified\naddressing on Linux and Windows operating systems.\nIPC functionality on Windows is supported for compatibility purposes\nbut not recommended as it comes with performance cost.\nUsers can test their device for IPC functionality by calling\n::cudaDeviceGetAttribute with ::cudaDevAttrIpcEventSupport\n\n# Arguments\n\n* `event` - - Returns the imported event\n* `handle` -  - Interprocess handle to open\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorMapBufferObjectFailed,\n::cudaErrorNotSupported,\n::cudaErrorInvalidValue,\n::cudaErrorDeviceUninitialized\n\\note_init_rt \\note_callback # See also\n\n> [`::cudaEventCreate,`]\n::cudaEventDestroy,\n::cudaEventSynchronize,\n::cudaEventQuery,\n::cudaStreamWaitEvent,\n::cudaIpcGetEventHandle,\n::cudaIpcGetMemHandle,\n::cudaIpcOpenMemHandle,\n::cudaIpcCloseMemHandle,\n::cuIpcOpenEventHandle"]
pub unsafe fn cudaIpcOpenEventHandle<T: types::CudaAsPtr>(
    mut event: T,
    handle: cudaIpcEventHandle_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaIpcOpenEventHandle(event.as_mut_ptr() as *mut _, handle) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Gets an interprocess memory handle for an existing device memory\nallocation\nTakes a pointer to the base of an existing device memory allocation created\nwith ::cudaMalloc and exports it for use in another process. This is a\nlightweight operation and may be called multiple times on an allocation\nwithout adverse effects.\nIf a region of memory is freed with ::cudaFree and a subsequent call\nto ::cudaMalloc returns memory with the same device address,\n::cudaIpcGetMemHandle will return a unique handle for the\nnew memory.\nIPC functionality is restricted to devices with support for unified\naddressing on Linux and Windows operating systems.\nIPC functionality on Windows is supported for compatibility purposes\nbut not recommended as it comes with performance cost.\nUsers can test their device for IPC functionality by calling\n::cudaDeviceGetAttribute with ::cudaDevAttrIpcEventSupport\n\n# Arguments\n\n* `handle` - - Pointer to user allocated ::cudaIpcMemHandle to return\nthe handle in.\n* `devPtr` - - Base pointer to previously allocated device memory\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorMemoryAllocation,\n::cudaErrorMapBufferObjectFailed,\n::cudaErrorNotSupported,\n::cudaErrorInvalidValue\n\\note_init_rt \\note_callback # See also\n\n> [`::cudaMalloc,`]\n::cudaFree,\n::cudaIpcGetEventHandle,\n::cudaIpcOpenEventHandle,\n::cudaIpcOpenMemHandle,\n::cudaIpcCloseMemHandle,\n::cuIpcGetMemHandle"]
pub unsafe fn cudaIpcGetMemHandle(
    devPtr: *mut ::std::os::raw::c_void,
) -> Result<cudaIpcMemHandle_t, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<cudaIpcMemHandle_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaIpcGetMemHandle(out_0.as_mut_ptr() as *mut _, devPtr) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_0.assume_init() as cudaIpcMemHandle_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Opens an interprocess memory handle exported from another process\nand returns a device pointer usable in the local process.\nMaps memory exported from another process with ::cudaIpcGetMemHandle into\nthe current device address space. For contexts on different devices\n::cudaIpcOpenMemHandle can attempt to enable peer access between the\ndevices as if the user called ::cudaDeviceEnablePeerAccess. This behavior is\ncontrolled by the ::cudaIpcMemLazyEnablePeerAccess flag.\n::cudaDeviceCanAccessPeer can determine if a mapping is possible.\n::cudaIpcOpenMemHandle can open handles to devices that may not be visible\nin the process calling the API.\nContexts that may open ::cudaIpcMemHandles are restricted in the following way.\n::cudaIpcMemHandles from each device in a given process may only be opened\nby one context per device per other process.\nIf the memory handle has already been opened by the current context, the\nreference count on the handle is incremented by 1 and the existing device pointer\nis returned.\nMemory returned from ::cudaIpcOpenMemHandle must be freed with\n::cudaIpcCloseMemHandle.\nCalling ::cudaFree on an exported memory region before calling\n::cudaIpcCloseMemHandle in the importing context will result in undefined\nbehavior.\nIPC functionality is restricted to devices with support for unified\naddressing on Linux and Windows operating systems.\nIPC functionality on Windows is supported for compatibility purposes\nbut not recommended as it comes with performance cost.\nUsers can test their device for IPC functionality by calling\n::cudaDeviceGetAttribute with ::cudaDevAttrIpcEventSupport\n\n# Arguments\n\n* `devPtr` - - Returned device pointer\n* `handle` - - ::cudaIpcMemHandle to open\n* `flags` -  - Flags for this operation. Must be specified as ::cudaIpcMemLazyEnablePeerAccess\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorMapBufferObjectFailed,\n::cudaErrorInvalidResourceHandle,\n::cudaErrorDeviceUninitialized,\n::cudaErrorTooManyPeers,\n::cudaErrorNotSupported,\n::cudaErrorInvalidValue\n\\note_init_rt \\note_callback > **Note** No guarantees are made about the address returned in `*devPtr.`\nIn particular, multiple processes may not receive the same address for the same `handle.`\n\n# See also\n\n> [`::cudaMalloc,`]\n::cudaFree,\n::cudaIpcGetEventHandle,\n::cudaIpcOpenEventHandle,\n::cudaIpcGetMemHandle,\n::cudaIpcCloseMemHandle,\n::cudaDeviceEnablePeerAccess,\n::cudaDeviceCanAccessPeer,\n::cuIpcOpenMemHandle"]
pub unsafe fn cudaIpcOpenMemHandle<T: types::CudaAsPtr>(
    mut devPtr: T,
    handle: cudaIpcMemHandle_t,
    flags: u32,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaIpcOpenMemHandle(devPtr.as_mut_ptr() as *mut _, handle, flags as _) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Attempts to close memory mapped with cudaIpcOpenMemHandle\nDecrements the reference count of the memory returnd by ::cudaIpcOpenMemHandle by 1.\nWhen the reference count reaches 0, this API unmaps the memory. The original allocation\nin the exporting process as well as imported mappings in other processes\nwill be unaffected.\nAny resources used to enable peer access will be freed if this is the\nlast mapping using them.\nIPC functionality is restricted to devices with support for unified\naddressing on Linux and Windows operating systems.\nIPC functionality on Windows is supported for compatibility purposes\nbut not recommended as it comes with performance cost.\nUsers can test their device for IPC functionality by calling\n::cudaDeviceGetAttribute with ::cudaDevAttrIpcEventSupport\n\n# Arguments\n\n* `devPtr` - - Device pointer returned by ::cudaIpcOpenMemHandle\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorMapBufferObjectFailed,\n::cudaErrorNotSupported,\n::cudaErrorInvalidValue\n\\note_init_rt \\note_callback # See also\n\n> [`::cudaMalloc,`]\n::cudaFree,\n::cudaIpcGetEventHandle,\n::cudaIpcOpenEventHandle,\n::cudaIpcGetMemHandle,\n::cudaIpcOpenMemHandle,\n::cuIpcCloseMemHandle"]
pub unsafe fn cudaIpcCloseMemHandle<T: types::CudaAsPtr>(mut devPtr: T) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaIpcCloseMemHandle(devPtr.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudaDeviceFlushGPUDirectRDMAWrites(
    target: cudaFlushGPUDirectRDMAWritesTarget,
    scope: cudaFlushGPUDirectRDMAWritesScope,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaDeviceFlushGPUDirectRDMAWrites(target, scope) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Registers a callback function to receive async notifications\nRegisters `callbackFunc` to receive async notifications.\nThe `userData` parameter is passed to the callback function at async notification time.\nLikewise, `callback` is also passed to the callback function to distinguish between\nmultiple registered callbacks.\nThe callback function being registered should be designed to return quickly (~10ms).\nAny long running tasks should be queued for execution on an application thread.\nCallbacks may not call cudaDeviceRegisterAsyncNotification or cudaDeviceUnregisterAsyncNotification.\nDoing so will result in ::cudaErrorNotPermitted. Async notification callbacks execute\nin an undefined order and may be serialized.\nReturns in `*callback` a handle representing the registered callback instance.\n\n# Arguments\n\n* `device` - - The device on which to register the callback\n* `callbackFunc` - - The function to register as a callback\n* `userData` - - A generic pointer to user data. This is passed into the callback function.\n* `callback` - - A handle representing the registered callback instance\n\n# Returns\n\n::cudaSuccess\n::cudaErrorNotSupported\n::cudaErrorInvalidDevice\n::cudaErrorInvalidValue\n::cudaErrorNotPermitted\n::cudaErrorUnknown\n\\notefnerr # See also\n\n> [`::cudaDeviceUnregisterAsyncNotification`]"]
pub unsafe fn cudaDeviceRegisterAsyncNotification<T: types::CudaAsPtr>(
    device: i32,
    callbackFunc: cudaAsyncCallback,
    mut userData: T,
    callback: *mut cudaAsyncCallbackHandle_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaDeviceRegisterAsyncNotification(
            device as _,
            callbackFunc,
            userData.as_mut_ptr() as *mut _,
            callback,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Unregisters an async notification callback\nUnregisters `callback` so that the corresponding callback function will stop receiving\nasync notifications.\n\n# Arguments\n\n* `device` - - The device from which to remove `callback.`\n* `callback` - - The callback instance to unregister from receiving async notifications.\n\n# Returns\n\n::cudaSuccess\n::cudaErrorNotSupported\n::cudaErrorInvalidDevice\n::cudaErrorInvalidValue\n::cudaErrorNotPermitted\n::cudaErrorUnknown\n\\notefnerr # See also\n\n> [`::cudaDeviceRegisterAsyncNotification`]"]
pub unsafe fn cudaDeviceUnregisterAsyncNotification(
    device: i32,
    callback: cudaAsyncCallbackHandle_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaDeviceUnregisterAsyncNotification(device as _, callback) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Returns the shared memory configuration for the current device.\n> **Deprecated** This function will return in `pConfig` the current size of shared memory banks\non the current device. On devices with configurable shared memory banks,\n::cudaDeviceSetSharedMemConfig can be used to change this setting, so that all\nsubsequent kernel launches will by default use the new bank size. When\n::cudaDeviceGetSharedMemConfig is called on devices without configurable shared\nmemory, it will return the fixed bank size of the hardware.\nThe returned bank configurations can be either:\n- ::cudaSharedMemBankSizeFourByte - shared memory bank width is four bytes.\n- ::cudaSharedMemBankSizeEightByte - shared memory bank width is eight bytes.\n\n# Arguments\n\n* `pConfig` - - Returned cache configuration\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue\n\\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaDeviceSetCacheConfig,`]\n::cudaDeviceGetCacheConfig,\n::cudaDeviceSetSharedMemConfig,\n::cudaFuncSetCacheConfig,\n::cuCtxGetSharedMemConfig"]
pub unsafe fn cudaDeviceGetSharedMemConfig() -> Result<cudaSharedMemConfig, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<cudaSharedMemConfig> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaDeviceGetSharedMemConfig(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_0.assume_init() as cudaSharedMemConfig) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Sets the shared memory configuration for the current device.\n> **Deprecated** On devices with configurable shared memory banks, this function will set\nthe shared memory bank size which is used for all subsequent kernel launches.\nAny per-function setting of shared memory set via ::cudaFuncSetSharedMemConfig\nwill override the device wide setting.\nChanging the shared memory configuration between launches may introduce\na device side synchronization point.\nChanging the shared memory bank size will not increase shared memory usage\nor affect occupancy of kernels, but may have major effects on performance.\nLarger bank sizes will allow for greater potential bandwidth to shared memory,\nbut will change what kinds of accesses to shared memory will result in bank\nconflicts.\nThis function will do nothing on devices with fixed shared memory bank size.\nThe supported bank configurations are:\n- ::cudaSharedMemBankSizeDefault: set bank width the device default (currently,\nfour bytes)\n- ::cudaSharedMemBankSizeFourByte: set shared memory bank width to be four bytes\nnatively.\n- ::cudaSharedMemBankSizeEightByte: set shared memory bank width to be eight\nbytes natively.\n\n# Arguments\n\n* `config` - - Requested cache configuration\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue\n\\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaDeviceSetCacheConfig,`]\n::cudaDeviceGetCacheConfig,\n::cudaDeviceGetSharedMemConfig,\n::cudaFuncSetCacheConfig,\n::cuCtxSetSharedMemConfig"]
pub unsafe fn cudaDeviceSetSharedMemConfig(config: cudaSharedMemConfig) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaDeviceSetSharedMemConfig(config) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Returns the last error from a runtime call\nReturns the last error that has been produced by any of the runtime calls\nin the same instance of the CUDA Runtime library in the host thread and\nresets it to ::cudaSuccess.\nNote: Multiple instances of the CUDA Runtime library can be present in an\napplication when using a library that statically links the CUDA Runtime.\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorMissingConfiguration,\n::cudaErrorMemoryAllocation,\n::cudaErrorInitializationError,\n::cudaErrorLaunchFailure,\n::cudaErrorLaunchTimeout,\n::cudaErrorLaunchOutOfResources,\n::cudaErrorInvalidDeviceFunction,\n::cudaErrorInvalidConfiguration,\n::cudaErrorInvalidDevice,\n::cudaErrorInvalidValue,\n::cudaErrorInvalidPitchValue,\n::cudaErrorInvalidSymbol,\n::cudaErrorUnmapBufferObjectFailed,\n::cudaErrorInvalidDevicePointer,\n::cudaErrorInvalidTexture,\n::cudaErrorInvalidTextureBinding,\n::cudaErrorInvalidChannelDescriptor,\n::cudaErrorInvalidMemcpyDirection,\n::cudaErrorInvalidFilterSetting,\n::cudaErrorInvalidNormSetting,\n::cudaErrorUnknown,\n::cudaErrorInvalidResourceHandle,\n::cudaErrorInsufficientDriver,\n::cudaErrorNoDevice,\n::cudaErrorSetOnActiveProcess,\n::cudaErrorStartupFailure,\n::cudaErrorInvalidPtx,\n::cudaErrorUnsupportedPtxVersion,\n::cudaErrorNoKernelImageForDevice,\n::cudaErrorJitCompilerNotFound,\n::cudaErrorJitCompilationDisabled\n\\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaPeekAtLastError,`] ::cudaGetErrorName, ::cudaGetErrorString, ::cudaError"]
pub unsafe fn cudaGetLastError() -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaGetLastError() };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Returns the last error from a runtime call\nReturns the last error that has been produced by any of the runtime calls\nin the same instance of the CUDA Runtime library in the host thread. This\ncall does not reset the error to ::cudaSuccess like ::cudaGetLastError().\nNote: Multiple instances of the CUDA Runtime library can be present in an\napplication when using a library that statically links the CUDA Runtime.\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorMissingConfiguration,\n::cudaErrorMemoryAllocation,\n::cudaErrorInitializationError,\n::cudaErrorLaunchFailure,\n::cudaErrorLaunchTimeout,\n::cudaErrorLaunchOutOfResources,\n::cudaErrorInvalidDeviceFunction,\n::cudaErrorInvalidConfiguration,\n::cudaErrorInvalidDevice,\n::cudaErrorInvalidValue,\n::cudaErrorInvalidPitchValue,\n::cudaErrorInvalidSymbol,\n::cudaErrorUnmapBufferObjectFailed,\n::cudaErrorInvalidDevicePointer,\n::cudaErrorInvalidTexture,\n::cudaErrorInvalidTextureBinding,\n::cudaErrorInvalidChannelDescriptor,\n::cudaErrorInvalidMemcpyDirection,\n::cudaErrorInvalidFilterSetting,\n::cudaErrorInvalidNormSetting,\n::cudaErrorUnknown,\n::cudaErrorInvalidResourceHandle,\n::cudaErrorInsufficientDriver,\n::cudaErrorNoDevice,\n::cudaErrorSetOnActiveProcess,\n::cudaErrorStartupFailure,\n::cudaErrorInvalidPtx,\n::cudaErrorUnsupportedPtxVersion,\n::cudaErrorNoKernelImageForDevice,\n::cudaErrorJitCompilerNotFound,\n::cudaErrorJitCompilationDisabled\n\\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaGetLastError,`] ::cudaGetErrorName, ::cudaGetErrorString, ::cudaError"]
pub unsafe fn cudaPeekAtLastError() -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaPeekAtLastError() };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Returns the string representation of an error code enum name\nReturns a string containing the name of an error code in the enum.  If the error\ncode is not recognized, \"unrecognized error code\" is returned.\n\n# Arguments\n\n* `error` - - Error code to convert to string\n\n# Returns\n\n`char*` pointer to a NULL-terminated string\n\n# See also\n\n> [`::cudaGetErrorString,`] ::cudaGetLastError, ::cudaPeekAtLastError, ::cudaError,\n::cuGetErrorName"]
pub unsafe fn cudaGetErrorName(error: cudaError_t) -> *const ::std::os::raw::c_char {
    unsafe { crate::sys::cudaGetErrorName(error) }
}
#[doc = "Returns the description string for an error code\nReturns the description string for an error code.  If the error\ncode is not recognized, \"unrecognized error code\" is returned.\n\n# Arguments\n\n* `error` - - Error code to convert to string\n\n# Returns\n\n`char*` pointer to a NULL-terminated string\n\n# See also\n\n> [`::cudaGetErrorName,`] ::cudaGetLastError, ::cudaPeekAtLastError, ::cudaError,\n::cuGetErrorString"]
pub unsafe fn cudaGetErrorString(error: cudaError_t) -> *const ::std::os::raw::c_char {
    unsafe { crate::sys::cudaGetErrorString(error) }
}
#[doc = "Returns the number of compute-capable devices\nReturns in `*count` the number of devices with compute capability greater\nor equal to 2.0 that are available for execution.\n\n# Arguments\n\n* `count` - - Returns the number of devices with compute capability\ngreater or equal to 2.0\n\n# Returns\n\n::cudaSuccess\n\\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaGetDevice,`] ::cudaSetDevice, ::cudaGetDeviceProperties,\n::cudaChooseDevice,\n::cudaInitDevice,\n::cuDeviceGetCount"]
pub unsafe fn cudaGetDeviceCount() -> Result<i32, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaGetDeviceCount(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_0.assume_init() as i32) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Returns information about the compute-device\nReturns in `*prop` the properties of device `dev.`\n\n# Arguments\n\n* `prop` -   - Properties for the specified device\n* `device` - - Device number to get properties for\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidDevice\n\\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaGetDeviceCount,`] ::cudaGetDevice, ::cudaSetDevice, ::cudaChooseDevice,\n::cudaDeviceGetAttribute,\n::cudaInitDevice,\n::cuDeviceGetAttribute,\n::cuDeviceGetName"]
pub unsafe fn cudaGetDeviceProperties(device: i32) -> Result<cudaDeviceProp, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<cudaDeviceProp> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaGetDeviceProperties(out_0.as_mut_ptr() as *mut _, device as _) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_0.assume_init() as cudaDeviceProp) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Returns information about the device\nReturns in `*value` the integer value of the attribute `attr` on device\n`device.`\n\n# Arguments\n\n* `value` -  - Returned device attribute value\n* `attr` -   - Device attribute to query\n* `device` - - Device number to query\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidDevice,\n::cudaErrorInvalidValue\n\\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaGetDeviceCount,`] ::cudaGetDevice, ::cudaSetDevice, ::cudaChooseDevice,\n::cudaGetDeviceProperties,\n::cudaInitDevice,\n::cuDeviceGetAttribute"]
pub unsafe fn cudaDeviceGetAttribute(attr: cudaDeviceAttr, device: i32) -> Result<i32, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaDeviceGetAttribute(out_0.as_mut_ptr() as *mut _, attr, device as _) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_0.assume_init() as i32) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Queries details about atomic operations supported between the device and host.\nReturns in `*capabilities` the details about requested atomic `*operations` over the\nthe link between `dev` and the host. The allocated size of `*operations` and\n`*capabilities` must be `count.`\nFor each ::cudaAtomicOperation in `*operations,` the corresponding result in `*capabilities`\nwill be a bitmask indicating which of ::cudaAtomicOperationCapability the link supports natively.\nReturns ::cudaErrorInvalidDevice if `dev` is not valid.\nReturns ::cudaErrorInvalidValue if `*capabilities` or `*operations` is NULL, if `count` is 0,\nor if any of `*operations` is not valid.\n\n# Arguments\n\n* `capabilities` -          - Returned capability details of each requested operation\n* `operations` -            - Requested operations\n* `count` -                 - Count of requested operations and size of capabilities\n* `dev` -                   - Device handle\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidDevice,\n::cudaErrorInvalidValue\n\\notefnerr # See also\n\n> [`::cudaDeviceGetAttribute,`]\n::cudaDeviceGetP2PAtomicCapabilities,\n::cuDeviceGeHostAtomicCapabilities"]
pub unsafe fn cudaDeviceGetHostAtomicCapabilities(
    operations: *const cudaAtomicOperation,
    count: u32,
    device: i32,
) -> Result<u32, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<::std::os::raw::c_uint> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudaDeviceGetHostAtomicCapabilities(
            out_0.as_mut_ptr() as *mut _,
            operations,
            count as _,
            device as _,
        )
    };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_0.assume_init() as u32) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Returns the default mempool of a device\nThe default mempool of a device contains device memory from that device.\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidDevice,\n::cudaErrorInvalidValue\n::cudaErrorNotSupported\n\\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cuDeviceGetDefaultMemPool,`] ::cudaMallocAsync, ::cudaMemPoolTrimTo, ::cudaMemPoolGetAttribute, ::cudaDeviceSetMemPool, ::cudaMemPoolSetAttribute, ::cudaMemPoolSetAccess"]
pub unsafe fn cudaDeviceGetDefaultMemPool(device: i32) -> Result<cudaMemPool_t, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<cudaMemPool_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaDeviceGetDefaultMemPool(out_0.as_mut_ptr() as *mut _, device as _) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_0.assume_init() as cudaMemPool_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Sets the current memory pool of a device\nThe memory pool must be local to the specified device.\nUnless a mempool is specified in the ::cudaMallocAsync call,\n::cudaMallocAsync allocates from the current mempool of the provided stream's device.\nBy default, a device's current memory pool is its default memory pool.\n> **Note** Use ::cudaMallocFromPoolAsync to specify asynchronous allocations from a device different\nthan the one the stream runs on.\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue\n::cudaErrorInvalidDevice\n::cudaErrorNotSupported\n\\notefnerr \\note_callback # See also\n\n> [`::cuDeviceSetMemPool,`] ::cudaDeviceGetMemPool, ::cudaDeviceGetDefaultMemPool, ::cudaMemPoolCreate, ::cudaMemPoolDestroy, ::cudaMallocFromPoolAsync"]
pub unsafe fn cudaDeviceSetMemPool(device: i32, memPool: cudaMemPool_t) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaDeviceSetMemPool(device as _, memPool) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Gets the current mempool for a device\nReturns the last pool provided to ::cudaDeviceSetMemPool for this device\nor the device's default memory pool if ::cudaDeviceSetMemPool has never been called.\nBy default the current mempool is the default mempool for a device,\notherwise the returned pool must have been set with ::cuDeviceSetMemPool or ::cudaDeviceSetMemPool.\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue\n::cudaErrorNotSupported\n\\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cuDeviceGetMemPool,`] ::cudaDeviceGetDefaultMemPool, ::cudaDeviceSetMemPool"]
pub unsafe fn cudaDeviceGetMemPool(device: i32) -> Result<cudaMemPool_t, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<cudaMemPool_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaDeviceGetMemPool(out_0.as_mut_ptr() as *mut _, device as _) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_0.assume_init() as cudaMemPool_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Return NvSciSync attributes that this device can support.\nReturns in `nvSciSyncAttrList,` the properties of NvSciSync that\nthis CUDA device, `dev` can support. The returned `nvSciSyncAttrList`\ncan be used to create an NvSciSync that matches this device's capabilities.\nIf NvSciSyncAttrKey_RequiredPerm field in `nvSciSyncAttrList` is\nalready set this API will return ::cudaErrorInvalidValue.\nThe applications should set `nvSciSyncAttrList` to a valid\nNvSciSyncAttrList failing which this API will return\n::cudaErrorInvalidHandle.\nThe `flags` controls how applications intends to use\nthe NvSciSync created from the `nvSciSyncAttrList.` The valid flags are:\n- ::cudaNvSciSyncAttrSignal, specifies that the applications intends to\nsignal an NvSciSync on this CUDA device.\n- ::cudaNvSciSyncAttrWait, specifies that the applications intends to\nwait on an NvSciSync on this CUDA device.\nAt least one of these flags must be set, failing which the API\nreturns ::cudaErrorInvalidValue. Both the flags are orthogonal\nto one another: a developer may set both these flags that allows to\nset both wait and signal specific attributes in the same `nvSciSyncAttrList.`\nNote that this API updates the input `nvSciSyncAttrList` with values equivalent\nto the following public attribute key-values:\nNvSciSyncAttrKey_RequiredPerm is set to\n- NvSciSyncAccessPerm_SignalOnly if ::cudaNvSciSyncAttrSignal is set in `flags.`\n- NvSciSyncAccessPerm_WaitOnly if ::cudaNvSciSyncAttrWait is set in `flags.`\n- NvSciSyncAccessPerm_WaitSignal if both ::cudaNvSciSyncAttrWait and\n::cudaNvSciSyncAttrSignal are set in `flags.`\nNvSciSyncAttrKey_PrimitiveInfo is set to\n- NvSciSyncAttrValPrimitiveType_SysmemSemaphore on any valid `device.`\n- NvSciSyncAttrValPrimitiveType_Syncpoint if `device` is a Tegra device.\n- NvSciSyncAttrValPrimitiveType_SysmemSemaphorePayload64b if `device` is GA10X+.\nNvSciSyncAttrKey_GpuId is set to the same UUID that is returned in\n`cudaDeviceProp.uuid` from ::cudaDeviceGetProperties for this `device.`\n\n# Arguments\n\n* `nvSciSyncAttrList` -     - Return NvSciSync attributes supported.\n* `device` -                - Valid Cuda Device to get NvSciSync attributes for.\n* `flags` -                 - flags describing NvSciSync usage.\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorDeviceUninitialized,\n::cudaErrorInvalidValue,\n::cudaErrorInvalidHandle,\n::cudaErrorInvalidDevice,\n::cudaErrorNotSupported,\n::cudaErrorMemoryAllocation\n\n# See also\n\n> [`::cudaImportExternalSemaphore,`]\n::cudaDestroyExternalSemaphore,\n::cudaSignalExternalSemaphoresAsync,\n::cudaWaitExternalSemaphoresAsync"]
pub unsafe fn cudaDeviceGetNvSciSyncAttributes<T: types::CudaAsPtr>(
    mut nvSciSyncAttrList: T,
    device: i32,
    flags: i32,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaDeviceGetNvSciSyncAttributes(nvSciSyncAttrList.as_mut_ptr() as *mut _, device as _, flags as _)
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Queries attributes of the link between two devices.\nReturns in `*value` the value of the requested attribute `attrib` of the\nlink between `srcDevice` and `dstDevice.` The supported attributes are:\n- ::cudaDevP2PAttrPerformanceRank: A relative value indicating the\nperformance of the link between two devices. Lower value means better\nperformance (0 being the value used for most performant link).\n- ::cudaDevP2PAttrAccessSupported: 1 if peer access is enabled.\n- ::cudaDevP2PAttrNativeAtomicSupported: 1 if all native atomic operations\nover the link are supported.\n- ::cudaDevP2PAttrCudaArrayAccessSupported: 1 if accessing CUDA arrays over\nthe link is supported.\n- ::cudaDevP2PAttrOnlyPartialNativeAtomicSupported: 1 if some\nCUDA-valid atomic operations over the link are supported. Information about\nspecific operations can be retrieved with ::cudaDeviceGetP2PAtomicCapabilities.\nReturns ::cudaErrorInvalidDevice if `srcDevice` or `dstDevice` are not valid\nor if they represent the same device.\nReturns ::cudaErrorInvalidValue if `attrib` is not valid or if `value` is\na null pointer.\n\n# Arguments\n\n* `value` -         - Returned value of the requested attribute\n* `attrib` -        - The requested attribute of the link between `srcDevice` and `dstDevice.`\n* `srcDevice` -     - The source device of the target link.\n* `dstDevice` -     - The destination device of the target link.\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidDevice,\n::cudaErrorInvalidValue\n\\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaDeviceEnablePeerAccess,`]\n::cudaDeviceDisablePeerAccess,\n::cudaDeviceCanAccessPeer,\n::cuDeviceGetP2PAttribute\n::cudaDeviceGetP2PAtomicCapabilities"]
pub unsafe fn cudaDeviceGetP2PAttribute(
    attr: cudaDeviceP2PAttr,
    srcDevice: i32,
    dstDevice: i32,
) -> Result<i32, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudaDeviceGetP2PAttribute(out_0.as_mut_ptr() as *mut _, attr, srcDevice as _, dstDevice as _)
    };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_0.assume_init() as i32) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Queries details about atomic operations supported between two devices\nReturns in `*capabilities` the details about requested atomic `*operations` over the\nthe link between `srcDevice` and `dstDevice.` The allocated size of `*operations` and\n`*capabilities` must be `count.`\nFor each ::cudaAtomicOperation in `*operations,` the corresponding result in `*capabilities`\nwill be a bitmask indicating which of ::cudaAtomicOperationCapability the link supports natively.\nReturns ::cudaErrorInvalidDevice if `srcDevice` or `dstDevice` are not valid\nor if they represent the same device.\nReturns ::cudaErrorInvalidValue if `*capabilities` or `*operations` is NULL, if `count` is 0,\nor if any of `*operations` is not valid.\n\n# Arguments\n\n* `capabilities` -          - Returned capability details of each requested operation\n* `operations` -            - Requested operations\n* `count` -                 - Count of requested operations and size of capabilities\n* `srcDevice` -             - The source device of the target link\n* `dstDevice` -             - The destination device of the target link\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidDevice,\n::cudaErrorInvalidValue\n\\notefnerr # See also\n\n> [`::cudaDeviceGetP2PAttribute,`]\n::cuDeviceGetP2PAttribute,\n::cuDeviceGetP2PAtomicCapabilities"]
pub unsafe fn cudaDeviceGetP2PAtomicCapabilities(
    operations: *const cudaAtomicOperation,
    count: u32,
    srcDevice: i32,
    dstDevice: i32,
) -> Result<u32, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<::std::os::raw::c_uint> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudaDeviceGetP2PAtomicCapabilities(
            out_0.as_mut_ptr() as *mut _,
            operations,
            count as _,
            srcDevice as _,
            dstDevice as _,
        )
    };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_0.assume_init() as u32) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Select compute-device which best matches criteria\nReturns in `*device` the device which has properties that best match\n`*prop.`\n\n# Arguments\n\n* `device` - - Device with best match\n* `prop` -   - Desired device properties\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue\n\\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaGetDeviceCount,`] ::cudaGetDevice, ::cudaSetDevice,\n::cudaGetDeviceProperties,\n::cudaInitDevice"]
pub unsafe fn cudaChooseDevice<T: types::CudaAsPtr>(
    mut device: T,
    prop: *const cudaDeviceProp,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaChooseDevice(device.as_mut_ptr() as *mut _, prop) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Initialize device to be used for GPU executions\nThis function will initialize the CUDA Runtime structures and primary context on `device` when called,\nbut the context will not be made current to `device.`\nWhen ::cudaInitDeviceFlagsAreValid is set in `flags,` deviceFlags are applied to the requested device.\nThe values of deviceFlags match those of the flags parameters in ::cudaSetDeviceFlags.\nThe effect may be verified by ::cudaGetDeviceFlags.\nThis function will return an error if the device is in ::cudaComputeModeExclusiveProcess\nand is occupied by another process or if the device is in ::cudaComputeModeProhibited.\n\n# Arguments\n\n* `device` - - Device on which the runtime will initialize itself.\n* `deviceFlags` - - Parameters for device operation.\n* `flags` - - Flags for controlling the device initialization.\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidDevice,\n\\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaGetDeviceCount,`] ::cudaGetDevice, ::cudaGetDeviceProperties,\n::cudaChooseDevice, ::cudaSetDevice\n::cuCtxSetCurrent"]
pub unsafe fn cudaInitDevice(device: i32, deviceFlags: u32, flags: u32) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaInitDevice(device as _, deviceFlags as _, flags as _) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Set device to be used for GPU executions\nSets `device` as the current device for the calling host thread.\nValid device id's are 0 to (::cudaGetDeviceCount() - 1).\nAny device memory subsequently allocated from this host thread\nusing ::cudaMalloc(), ::cudaMallocPitch() or ::cudaMallocArray()\nwill be physically resident on `device.`  Any host memory allocated\nfrom this host thread using ::cudaMallocHost() or ::cudaHostAlloc()\nor ::cudaHostRegister() will have its lifetime associated  with\n`device.`  Any streams or events created from this host thread will\nbe associated with `device.`  Any kernels launched from this host\nthread using the <<<>>> operator or ::cudaLaunchKernel() will be executed\non `device.`\nThis call may be made from any host thread, to any device, and at\nany time.  This function will do no synchronization with the previous\nor new device,\nand should only take significant time when it initializes the runtime's context state.\nThis call will bind the primary context of the specified device to the calling thread and all the\nsubsequent memory allocations, stream and event creations, and kernel launches\nwill be associated with the primary context.\nThis function will also immediately initialize the runtime state on the primary context,\nand the context will be current on `device` immediately. This function will return an\nerror if the device is in ::cudaComputeModeExclusiveProcess and is occupied by another\nprocess or if the device is in ::cudaComputeModeProhibited.\nIt is not required to call ::cudaInitDevice before using this function.\n\n# Arguments\n\n* `device` - - Device on which the active host thread should execute the\ndevice code.\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidDevice,\n::cudaErrorDeviceUnavailable,\n\\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaGetDeviceCount,`] ::cudaGetDevice, ::cudaGetDeviceProperties,\n::cudaChooseDevice,\n::cudaInitDevice,\n::cuCtxSetCurrent"]
pub unsafe fn cudaSetDevice(device: i32) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaSetDevice(device as _) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Returns which device is currently being used\nReturns in `*device` the current device for the calling host thread.\n\n# Arguments\n\n* `device` - - Returns the device on which the active host thread\nexecutes the device code.\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorDeviceUnavailable,\n\\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaGetDeviceCount,`] ::cudaSetDevice, ::cudaGetDeviceProperties,\n::cudaChooseDevice,\n::cuCtxGetCurrent"]
pub unsafe fn cudaGetDevice() -> Result<i32, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaGetDevice(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_0.assume_init() as i32) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Set a list of devices that can be used for CUDA\nSets a list of devices for CUDA execution in priority order using\n`device_arr.` The parameter `len` specifies the number of elements in the\nlist.  CUDA will try devices from the list sequentially until it finds one\nthat works.  If this function is not called, or if it is called with a `len`\nof 0, then CUDA will go back to its default behavior of trying devices\nsequentially from a default list containing all of the available CUDA\ndevices in the system. If a specified device ID in the list does not exist,\nthis function will return ::cudaErrorInvalidDevice. If `len` is not 0 and\n`device_arr` is NULL or if `len` exceeds the number of devices in\nthe system, then ::cudaErrorInvalidValue is returned.\n\n# Arguments\n\n* `device_arr` - - List of devices to try\n* `len` -        - Number of devices in specified list\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorInvalidDevice\n\\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaGetDeviceCount,`] ::cudaSetDevice, ::cudaGetDeviceProperties,\n::cudaSetDeviceFlags,\n::cudaChooseDevice"]
pub unsafe fn cudaSetValidDevices<T: types::CudaAsPtr>(
    mut device_arr: T,
    len: i32,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaSetValidDevices(device_arr.as_mut_ptr() as *mut _, len as _) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Sets flags to be used for device executions\nRecords `flags` as the flags for the current device. If the current device\nhas been set and that device has already been initialized, the previous flags\nare overwritten. If the current device has not been initialized, it is\ninitialized with the provided flags. If no device has been made current to\nthe calling thread, a default device is selected and initialized with the\nprovided flags.\nThe three LSBs of the `flags` parameter can be used to control how the CPU\nthread interacts with the OS scheduler when waiting for results from the\ndevice.\n- ::cudaDeviceScheduleAuto: The default value if the `flags` parameter is\nzero, uses a heuristic based on the number of active CUDA contexts in the\nprocess `C` and the number of logical processors in the system `P.` If\n`C` \\> `P,` then CUDA will yield to other OS threads when waiting for the\ndevice, otherwise CUDA will not yield while waiting for results and\nactively spin on the processor. Additionally, on Tegra devices,\n::cudaDeviceScheduleAuto uses a heuristic based on the power profile of\nthe platform and may choose ::cudaDeviceScheduleBlockingSync for low-powered\ndevices.\n- ::cudaDeviceScheduleSpin: Instruct CUDA to actively spin when waiting for\nresults from the device. This can decrease latency when waiting for the\ndevice, but may lower the performance of CPU threads if they are performing\nwork in parallel with the CUDA thread.\n- ::cudaDeviceScheduleYield: Instruct CUDA to yield its thread when waiting\nfor results from the device. This can increase latency when waiting for the\ndevice, but can increase the performance of CPU threads performing work in\nparallel with the device.\n- ::cudaDeviceScheduleBlockingSync: Instruct CUDA to block the CPU thread\non a synchronization primitive when waiting for the device to finish work.\n- ::cudaDeviceBlockingSync: Instruct CUDA to block the CPU thread on a\nsynchronization primitive when waiting for the device to finish work. <br>\n[`deprecated`] \"Deprecated:\" This flag was deprecated as of CUDA 4.0 and\nreplaced with ::cudaDeviceScheduleBlockingSync.\n- ::cudaDeviceMapHost: This flag enables allocating pinned\nhost memory that is accessible to the device. It is implicit for the\nruntime but may be absent if a context is created using the driver API.\nIf this flag is not set, ::cudaHostGetDevicePointer() will always return\na failure code.\n- ::cudaDeviceLmemResizeToMax: Instruct CUDA to not reduce local memory\nafter resizing local memory for a kernel. This can prevent thrashing by\nlocal memory allocations when launching many kernels with high local\nmemory usage at the cost of potentially increased memory usage. <br>\n[`deprecated`] \"Deprecated:\" This flag is deprecated and the behavior enabled\nby this flag is now the default and cannot be disabled.\n- ::cudaDeviceSyncMemops: Ensures that synchronous memory operations initiated\non this context will always synchronize. See further documentation in the\nsection titled \"API Synchronization behavior\" to learn more about cases when\nsynchronous memory operations can exhibit asynchronous behavior.\n\n# Arguments\n\n* `flags` - - Parameters for device operation\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue\n\\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaGetDeviceFlags,`] ::cudaGetDeviceCount, ::cudaGetDevice, ::cudaGetDeviceProperties,\n::cudaSetDevice, ::cudaSetValidDevices,\n::cudaInitDevice,\n::cudaChooseDevice,\n::cuDevicePrimaryCtxSetFlags"]
pub unsafe fn cudaSetDeviceFlags(flags: u32) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaSetDeviceFlags(flags as _) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Gets the flags for the current device\nReturns in `flags` the flags for the current device. If there is a current\ndevice for the calling thread, the flags for the device are returned. If\nthere is no current device, the flags for the first device are returned,\nwhich may be the default flags.  Compare to the behavior of\n::cudaSetDeviceFlags.\nTypically, the flags returned should match the behavior that will be seen\nif the calling thread uses a device after this call, without any change to\nthe flags or current device inbetween by this or another thread.  Note that\nif the device is not initialized, it is possible for another thread to\nchange the flags for the current device before it is initialized.\nAdditionally, when using exclusive mode, if this thread has not requested a\nspecific device, it may use a device other than the first device, contrary\nto the assumption made by this function.\nIf a context has been created via the driver API and is current to the\ncalling thread, the flags for that context are always returned.\nFlags returned by this function may specifically include ::cudaDeviceMapHost\neven though it is not accepted by ::cudaSetDeviceFlags because it is\nimplicit in runtime API flags.  The reason for this is that the current\ncontext may have been created via the driver API in which case the flag is\nnot implicit and may be unset.\n\n# Arguments\n\n* `flags` - - Pointer to store the device flags\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidDevice\n\\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaGetDevice,`] ::cudaGetDeviceProperties,\n::cudaSetDevice, ::cudaSetDeviceFlags,\n::cudaInitDevice,\n::cuCtxGetFlags,\n::cuDevicePrimaryCtxGetState"]
pub unsafe fn cudaGetDeviceFlags() -> Result<u32, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<::std::os::raw::c_uint> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaGetDeviceFlags(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_0.assume_init() as u32) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Query the priority of a stream\nQuery the priority of a stream. The priority is returned in in `priority.`\nNote that if the stream was created with a priority outside the meaningful\nnumerical range returned by ::cudaDeviceGetStreamPriorityRange,\nthis function returns the clamped priority.\nSee ::cudaStreamCreateWithPriority for details about priority clamping.\n\n# Arguments\n\n* `hStream` -    - Handle to the stream to be queried\n* `priority` -   - Pointer to a signed integer in which the stream's priority is returned\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorInvalidResourceHandle\n\\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaStreamCreateWithPriority,`]\n::cudaDeviceGetStreamPriorityRange,\n::cudaStreamGetFlags,\n::cudaStreamGetDevice,\n::cudaStreamGetDevResource,\n::cuStreamGetPriority"]
pub unsafe fn cudaStreamGetPriority(hStream: cudaStream_t) -> Result<i32, crate::sys::cudaError> {
    let mut out_1: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaStreamGetPriority(hStream, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_1.assume_init() as i32) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Query the flags of a stream\nQuery the flags of a stream. The flags are returned in `flags.`\nSee ::cudaStreamCreateWithFlags for a list of valid flags.\n\n# Arguments\n\n* `hStream` - - Handle to the stream to be queried\n* `flags` -   - Pointer to an unsigned integer in which the stream's flags are returned\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorInvalidResourceHandle\n\\note_null_stream \\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaStreamCreateWithPriority,`]\n::cudaStreamCreateWithFlags,\n::cudaStreamGetPriority,\n::cudaStreamGetDevice,\n::cuStreamGetFlags"]
pub unsafe fn cudaStreamGetFlags(hStream: cudaStream_t) -> Result<u32, crate::sys::cudaError> {
    let mut out_1: std::mem::MaybeUninit<::std::os::raw::c_uint> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaStreamGetFlags(hStream, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_1.assume_init() as u32) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Query the Id of a stream\nQuery the Id of a stream. The Id is returned in `streamId.`\nThe Id is unique for the life of the program.\nThe stream handle `hStream` can refer to any of the following:\n<ul>\n<li>a stream created via any of the CUDA runtime APIs such as ::cudaStreamCreate,\n::cudaStreamCreateWithFlags and ::cudaStreamCreateWithPriority, or their driver\nAPI equivalents such as ::cuStreamCreate or ::cuStreamCreateWithPriority.\nPassing an invalid handle will result in undefined behavior.</li>\n<li>any of the special streams such as the NULL stream, ::cudaStreamLegacy\nand ::cudaStreamPerThread respectively.  The driver API equivalents of these\nare also accepted which are NULL, ::CU_STREAM_LEGACY and ::CU_STREAM_PER_THREAD.</li>\n</ul>\n\n# Arguments\n\n* `hStream` -    - Handle to the stream to be queried\n* `streamId` -   - Pointer to an unsigned long long in which the stream Id is returned\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorInvalidResourceHandle\n\\note_null_stream \\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaStreamCreateWithPriority,`]\n::cudaStreamCreateWithFlags,\n::cudaStreamGetPriority,\n::cudaStreamGetFlags,\n::cuStreamGetId"]
pub unsafe fn cudaStreamGetId(hStream: cudaStream_t) -> Result<u64, crate::sys::cudaError> {
    let mut out_1: std::mem::MaybeUninit<::std::os::raw::c_ulonglong> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaStreamGetId(hStream, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_1.assume_init() as u64) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Query the device of a stream\nReturns in `*device` the device of the stream.\n\n# Arguments\n\n* `hStream` - - Handle to the stream to be queried\n* `device` - - Returns the device to which the stream belongs\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorDeviceUnavailable,\n\\note_null_stream \\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaSetDevice,`]\n::cudaGetDevice,\n::cudaStreamCreate,\n::cudaStreamGetPriority,\n::cudaStreamGetFlags,\n::cuStreamGetId"]
pub unsafe fn cudaStreamGetDevice(hStream: cudaStream_t) -> Result<i32, crate::sys::cudaError> {
    let mut out_1: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaStreamGetDevice(hStream, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_1.assume_init() as i32) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Resets all persisting lines in cache to normal status.\nResets all persisting lines in cache to normal status.\nTakes effect on function return.\n\n# Returns\n\n::cudaSuccess,\n\\notefnerr # See also\n\n> [`::cudaAccessPolicyWindow`]"]
pub unsafe fn cudaCtxResetPersistingL2Cache() -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaCtxResetPersistingL2Cache() };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Copies attributes from source stream to destination stream.\nCopies attributes from source stream `src` to destination stream `dst.`\nBoth streams must have the same context.\n\n# Arguments\n\n* `dst` [out]  - Destination stream\n* `src` [in]  - Source stream\nFor attributes see ::cudaStreamAttrID\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorNotSupported\n\\notefnerr # See also\n\n> [`::cudaAccessPolicyWindow`]"]
pub unsafe fn cudaStreamCopyAttributes(dst: cudaStream_t, src: cudaStream_t) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaStreamCopyAttributes(dst, src) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Queries stream attribute.\nQueries attribute `attr` from `hStream` and stores it in corresponding\nmember of `value_out.`\n\n# Arguments\n\n* `hStream` [in]  -\n* `attr` [in]  -\n* `value_out` [out]  -\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorInvalidResourceHandle\n\\notefnerr # See also\n\n> [`::cudaAccessPolicyWindow`]"]
pub unsafe fn cudaStreamGetAttribute(
    hStream: cudaStream_t,
    attr: cudaLaunchAttributeID,
) -> Result<cudaLaunchAttributeValue, crate::sys::cudaError> {
    let mut out_2: std::mem::MaybeUninit<cudaLaunchAttributeValue> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaStreamGetAttribute(hStream, attr, out_2.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_2.assume_init() as cudaLaunchAttributeValue) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Sets stream attribute.\nSets attribute `attr` on `hStream` from corresponding attribute of\n`value.` The updated attribute will be applied to subsequent work\nsubmitted to the stream. It will not affect previously submitted work.\n\n# Arguments\n\n* `hStream` [out]  -\n* `attr` [in]  -\n* `value` [in]  -\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorInvalidResourceHandle\n\\notefnerr # See also\n\n> [`::cudaAccessPolicyWindow`]"]
pub unsafe fn cudaStreamSetAttribute<T: types::CudaAsPtr>(
    hStream: cudaStream_t,
    attr: cudaLaunchAttributeID,
    value: T,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaStreamSetAttribute(hStream, attr, value.as_const_ptr() as *const _) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Make a compute stream wait on an event\nMakes all future work submitted to `stream` wait for all work captured in\n`event.`  See ::cudaEventRecord() for details on what is captured by an event.\nThe synchronization will be performed efficiently on the device when applicable.\n`event` may be from a different device than `stream.`\nflags include:\n- ::cudaEventWaitDefault: Default event creation flag.\n- ::cudaEventWaitExternal: Event is captured in the graph as an external\nevent node when performing stream capture.\n\n# Arguments\n\n* `stream` - - Stream to wait\n* `event` -  - Event to wait on\n* `flags` -  - Parameters for the operation(See above)\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorInvalidResourceHandle\n\\note_null_stream \\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaStreamCreate,`] ::cudaStreamCreateWithFlags, ::cudaStreamQuery, ::cudaStreamSynchronize, ::cudaStreamAddCallback, ::cudaStreamDestroy,\n::cuStreamWaitEvent"]
pub unsafe fn cudaStreamWaitEvent(
    stream: cudaStream_t,
    event: cudaEvent_t,
    flags: u32,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaStreamWaitEvent(stream, event, flags as _) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Add a callback to a compute stream\n> **Note** This function is slated for eventual deprecation and removal. If\nyou do not require the callback to execute in case of a device error,\nconsider using ::cudaLaunchHostFunc. Additionally, this function is not\nsupported with ::cudaStreamBeginCapture and ::cudaStreamEndCapture, unlike\n::cudaLaunchHostFunc.\nAdds a callback to be called on the host after all currently enqueued\nitems in the stream have completed.  For each\ncudaStreamAddCallback call, a callback will be executed exactly once.\nThe callback will block later work in the stream until it is finished.\nThe callback may be passed ::cudaSuccess or an error code.  In the event\nof a device error, all subsequently executed callbacks will receive an\nappropriate ::cudaError_t.\nCallbacks must not make any CUDA API calls.  Attempting to use CUDA APIs\nmay result in ::cudaErrorNotPermitted.  Callbacks must not perform any\nsynchronization that may depend on outstanding device work or other callbacks\nthat are not mandated to run earlier.  Callbacks without a mandated order\n(in independent streams) execute in undefined order and may be serialized.\nFor the purposes of Unified Memory, callback execution makes a number of\nguarantees:\n<ul>\n<li>The callback stream is considered idle for the duration of the\ncallback.  Thus, for example, a callback may always use memory attached\nto the callback stream.</li>\n<li>The start of execution of a callback has the same effect as\nsynchronizing an event recorded in the same stream immediately prior to\nthe callback.  It thus synchronizes streams which have been \"joined\"\nprior to the callback.</li>\n<li>Adding device work to any stream does not have the effect of making\nthe stream active until all preceding callbacks have executed.  Thus, for\nexample, a callback might use global attached memory even if work has\nbeen added to another stream, if it has been properly ordered with an\nevent.</li>\n<li>Completion of a callback does not cause a stream to become\nactive except as described above.  The callback stream will remain idle\nif no device work follows the callback, and will remain idle across\nconsecutive callbacks without device work in between.  Thus, for example,\nstream synchronization can be done by signaling from a callback at the\nend of the stream.</li>\n</ul>\n\n# Arguments\n\n* `stream` -   - Stream to add callback to\n* `callback` - - The function to call once preceding stream operations are complete\n* `userData` - - User specified data to be passed to the callback function\n* `flags` -    - Reserved for future use, must be 0\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidResourceHandle,\n::cudaErrorInvalidValue,\n::cudaErrorNotSupported\n\\note_null_stream \\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaStreamCreate,`] ::cudaStreamCreateWithFlags, ::cudaStreamQuery, ::cudaStreamSynchronize, ::cudaStreamWaitEvent, ::cudaStreamDestroy, ::cudaMallocManaged, ::cudaStreamAttachMemAsync,\n::cudaLaunchHostFunc, ::cuStreamAddCallback"]
pub unsafe fn cudaStreamAddCallback<T: types::CudaAsPtr>(
    stream: cudaStream_t,
    callback: cudaStreamCallback_t,
    mut userData: T,
    flags: u32,
) -> Result<(), crate::sys::cudaError> {
    let status =
        unsafe { crate::sys::cudaStreamAddCallback(stream, callback, userData.as_mut_ptr() as *mut _, flags as _) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Waits for stream tasks to complete\nBlocks until `stream` has completed all operations. If the\n::cudaDeviceScheduleBlockingSync flag was set for this device,\nthe host thread will block until the stream is finished with\nall of its tasks.\n\n# Arguments\n\n* `stream` - - Stream identifier\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidResourceHandle\n\\note_null_stream \\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaStreamCreate,`] ::cudaStreamCreateWithFlags, ::cudaStreamQuery, ::cudaStreamWaitEvent, ::cudaStreamAddCallback, ::cudaStreamDestroy,\n::cuStreamSynchronize"]
pub unsafe fn cudaStreamSynchronize(stream: cudaStream_t) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaStreamSynchronize(stream) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Queries an asynchronous stream for completion status\nReturns ::cudaSuccess if all operations in `stream` have\ncompleted, or ::cudaErrorNotReady if not.\nFor the purposes of Unified Memory, a return value of ::cudaSuccess\nis equivalent to having called ::cudaStreamSynchronize().\n\n# Arguments\n\n* `stream` - - Stream identifier\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorNotReady,\n::cudaErrorInvalidResourceHandle\n\\note_null_stream \\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaStreamCreate,`] ::cudaStreamCreateWithFlags, ::cudaStreamWaitEvent, ::cudaStreamSynchronize, ::cudaStreamAddCallback, ::cudaStreamDestroy,\n::cuStreamQuery"]
pub unsafe fn cudaStreamQuery(stream: cudaStream_t) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaStreamQuery(stream) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudaStreamAttachMemAsync<T: types::CudaAsPtr>(
    stream: cudaStream_t,
    mut devPtr: T,
    length: usize,
    flags: u32,
) -> Result<(), crate::sys::cudaError> {
    let status =
        unsafe { crate::sys::cudaStreamAttachMemAsync(stream, devPtr.as_mut_ptr() as *mut _, length, flags as _) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Begins graph capture on a stream\nBegin graph capture on `stream.` When a stream is in capture mode, all operations\npushed into the stream will not be executed, but will instead be captured into\na graph, which will be returned via ::cudaStreamEndCapture. Capture may not be initiated\nif `stream` is ::cudaStreamLegacy. Capture must be ended on the same stream in which\nit was initiated, and it may only be initiated if the stream is not already in capture\nmode. The capture mode may be queried via ::cudaStreamIsCapturing. A unique id\nrepresenting the capture sequence may be queried via ::cudaStreamGetCaptureInfo.\nIf `mode` is not ::cudaStreamCaptureModeRelaxed, ::cudaStreamEndCapture must be\ncalled on this stream from the same thread.\n> **Note** Kernels captured using this API must not use texture and surface references.\nReading or writing through any texture or surface reference is undefined\nbehavior. This restriction does not apply to texture and surface objects.\n\n# Arguments\n\n* `stream` - - Stream in which to initiate capture\n* `mode` -    - Controls the interaction of this capture sequence with other API\ncalls that are potentially unsafe. For more details see\n::cudaThreadExchangeStreamCaptureMode.\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue\n\\notefnerr # See also\n\n> [`::cudaStreamCreate,`]\n::cudaStreamIsCapturing,\n::cudaStreamEndCapture,\n::cudaThreadExchangeStreamCaptureMode"]
pub unsafe fn cudaStreamBeginCapture(
    stream: cudaStream_t,
    mode: cudaStreamCaptureMode,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaStreamBeginCapture(stream, mode) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Begins graph capture on a stream to an existing graph\nBegin graph capture on `stream.` When a stream is in capture mode, all operations\npushed into the stream will not be executed, but will instead be captured into\n`graph,` which will be returned via ::cudaStreamEndCapture.\nCapture may not be initiated if `stream` is ::cudaStreamLegacy. Capture must be ended on the\nsame stream in which it was initiated, and it may only be initiated if the stream is not\nalready in capture mode. The capture mode may be queried via ::cudaStreamIsCapturing. A unique id\nrepresenting the capture sequence may be queried via ::cudaStreamGetCaptureInfo.\nIf `mode` is not ::cudaStreamCaptureModeRelaxed, ::cudaStreamEndCapture must be\ncalled on this stream from the same thread.\n> **Note** Kernels captured using this API must not use texture and surface references.\nReading or writing through any texture or surface reference is undefined\nbehavior. This restriction does not apply to texture and surface objects.\n\n# Arguments\n\n* `stream` -          - Stream in which to initiate capture.\n* `graph` -           - Graph to capture into.\n* `dependencies` -    - Dependencies of the first node captured in the stream.  Can be NULL if numDependencies is 0.\n* `dependencyData` -  - Optional array of data associated with each dependency.\n* `numDependencies` - - Number of dependencies.\n* `mode` -            - Controls the interaction of this capture sequence with other API\ncalls that are potentially unsafe. For more details see\n::cudaThreadExchangeStreamCaptureMode.\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue\n\\notefnerr # See also\n\n> [`::cudaStreamCreate,`]\n::cudaStreamIsCapturing,\n::cudaStreamEndCapture,\n::cudaThreadExchangeStreamCaptureMode"]
pub unsafe fn cudaStreamBeginCaptureToGraph<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    stream: cudaStream_t,
    graph: cudaGraph_t,
    dependencies: T,
    dependencyData: U,
    numDependencies: usize,
    mode: cudaStreamCaptureMode,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaStreamBeginCaptureToGraph(
            stream,
            graph,
            dependencies.as_const_ptr() as *const _,
            dependencyData.as_const_ptr() as *const _,
            numDependencies,
            mode,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Swaps the stream capture interaction mode for a thread\nSets the calling thread's stream capture interaction mode to the value contained\nin `*mode,` and overwrites `*mode` with the previous mode for the thread. To\nfacilitate deterministic behavior across function or module boundaries, callers\nare encouraged to use this API in a push-pop fashion: \\code cudaStreamCaptureMode mode = desiredMode;\ncudaThreadExchangeStreamCaptureMode(&mode);\n...\ncudaThreadExchangeStreamCaptureMode(&mode); // restore previous mode\n\\endcode During stream capture (see ::cudaStreamBeginCapture), some actions, such as a call\nto ::cudaMalloc, may be unsafe. In the case of ::cudaMalloc, the operation is\nnot enqueued asynchronously to a stream, and is not observed by stream capture.\nTherefore, if the sequence of operations captured via ::cudaStreamBeginCapture\ndepended on the allocation being replayed whenever the graph is launched, the\ncaptured graph would be invalid.\nTherefore, stream capture places restrictions on API calls that can be made within\nor concurrently to a ::cudaStreamBeginCapture-::cudaStreamEndCapture sequence. This\nbehavior can be controlled via this API and flags to ::cudaStreamBeginCapture.\nA thread's mode is one of the following:\n- `cudaStreamCaptureModeGlobal:` This is the default mode. If the local thread has\nan ongoing capture sequence that was not initiated with\n`cudaStreamCaptureModeRelaxed` at `cuStreamBeginCapture,` or if any other thread\nhas a concurrent capture sequence initiated with `cudaStreamCaptureModeGlobal,`\nthis thread is prohibited from potentially unsafe API calls.\n- `cudaStreamCaptureModeThreadLocal:` If the local thread has an ongoing capture\nsequence not initiated with `cudaStreamCaptureModeRelaxed,` it is prohibited\nfrom potentially unsafe API calls. Concurrent capture sequences in other threads\nare ignored.\n- `cudaStreamCaptureModeRelaxed:` The local thread is not prohibited from potentially\nunsafe API calls. Note that the thread is still prohibited from API calls which\nnecessarily conflict with stream capture, for example, attempting ::cudaEventQuery\non an event that was last recorded inside a capture sequence.\n\n# Arguments\n\n* `mode` - - Pointer to mode value to swap with the current mode\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue\n\\notefnerr # See also\n\n> [`::cudaStreamBeginCapture`]"]
pub unsafe fn cudaThreadExchangeStreamCaptureMode(
    mode: *mut cudaStreamCaptureMode,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaThreadExchangeStreamCaptureMode(mode) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Ends capture on a stream, returning the captured graph\nEnd capture on `stream,` returning the captured graph via `pGraph.`\nCapture must have been initiated on `stream` via a call to ::cudaStreamBeginCapture.\nIf capture was invalidated, due to a violation of the rules of stream capture, then\na NULL graph will be returned.\nIf the `mode` argument to ::cudaStreamBeginCapture was not\n::cudaStreamCaptureModeRelaxed, this call must be from the same thread as\n::cudaStreamBeginCapture.\n\n# Arguments\n\n* `stream` - - Stream to query\n* `pGraph` - - The captured graph\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorStreamCaptureWrongThread\n\\notefnerr # See also\n\n> [`::cudaStreamCreate,`]\n::cudaStreamBeginCapture,\n::cudaStreamIsCapturing,\n::cudaGraphDestroy"]
pub unsafe fn cudaStreamEndCapture<T: types::CudaAsPtr>(
    stream: cudaStream_t,
    mut pGraph: T,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaStreamEndCapture(stream, pGraph.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Returns a stream's capture status\nReturn the capture status of `stream` via `pCaptureStatus.` After a successful\ncall, `*pCaptureStatus` will contain one of the following:\n- ::cudaStreamCaptureStatusNone: The stream is not capturing.\n- ::cudaStreamCaptureStatusActive: The stream is capturing.\n- ::cudaStreamCaptureStatusInvalidated: The stream was capturing but an error\nhas invalidated the capture sequence. The capture sequence must be terminated\nwith ::cudaStreamEndCapture on the stream where it was initiated in order to\ncontinue using `stream.`\nNote that, if this is called on ::cudaStreamLegacy (the \"null stream\") while\na blocking stream on the same device is capturing, it will return\n::cudaErrorStreamCaptureImplicit and `*pCaptureStatus` is unspecified\nafter the call. The blocking stream capture is not invalidated.\nWhen a blocking stream is capturing, the legacy stream is in an\nunusable state until the blocking stream capture is terminated. The legacy\nstream is not supported for stream capture, but attempted use would have an\nimplicit dependency on the capturing stream(s).\n\n# Arguments\n\n* `stream` -         - Stream to query\n* `pCaptureStatus` - - Returns the stream's capture status\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorStreamCaptureImplicit\n\\notefnerr # See also\n\n> [`::cudaStreamCreate,`]\n::cudaStreamBeginCapture,\n::cudaStreamEndCapture"]
pub unsafe fn cudaStreamIsCapturing(
    stream: cudaStream_t,
    pCaptureStatus: *mut cudaStreamCaptureStatus,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaStreamIsCapturing(stream, pCaptureStatus) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Query a stream's capture state\nQuery stream state related to stream capture.\nIf called on ::cudaStreamLegacy (the \"null stream\") while a stream not created\nwith ::cudaStreamNonBlocking is capturing, returns ::cudaErrorStreamCaptureImplicit.\nValid data (other than capture status) is returned only if both of the following are true:\n- the call returns cudaSuccess\n- the returned capture status is ::cudaStreamCaptureStatusActive\nIf `edgeData_out` is non-NULL then `dependencies_out` must be as well. If\n`dependencies_out` is non-NULL and `edgeData_out` is NULL, but there is non-zero edge\ndata for one or more of the current stream dependencies, the call will return\n::cudaErrorLossyQuery.\n\n# Arguments\n\n* `stream` - - The stream to query\n* `captureStatus_out` - - Location to return the capture status of the stream; required\n* `id_out` - - Optional location to return an id for the capture sequence, which is\nunique over the lifetime of the process\n* `graph_out` - - Optional location to return the graph being captured into. All\noperations other than destroy and node removal are permitted on the graph\nwhile the capture sequence is in progress. This API does not transfer\nownership of the graph, which is transferred or destroyed at\n::cudaStreamEndCapture. Note that the graph handle may be invalidated before\nend of capture for certain errors. Nodes that are or become\nunreachable from the original stream at ::cudaStreamEndCapture due to direct\nactions on the graph do not trigger ::cudaErrorStreamCaptureUnjoined.\n* `dependencies_out` - - Optional location to store a pointer to an array of nodes.\nThe next node to be captured in the stream will depend on this set of nodes,\nabsent operations such as event wait which modify this set. The array pointer\nis valid until the next API call which operates on the stream or until the\ncapture is terminated. The node handles may be copied out and are valid until\nthey or the graph is destroyed. The driver-owned array may also be passed\ndirectly to APIs that operate on the graph (not the stream) without copying.\n* `edgeData_out` - - Optional location to store a pointer to an array of graph edge\ndata. This array parallels `dependencies_out;` the next node to be added\nhas an edge to `dependencies_out`[i] with annotation `edgeData_out`[i] for\neach `i.` The array pointer is valid until the next API call which operates\non the stream or until the capture is terminated.\n* `numDependencies_out` - - Optional location to store the size of the array\nreturned in dependencies_out.\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorStreamCaptureImplicit,\n::cudaErrorLossyQuery\n\\note_graph_thread_safety \\notefnerr # See also\n\n> [`::cudaStreamBeginCapture,`]\n::cudaStreamIsCapturing,\n::cudaStreamUpdateCaptureDependencies"]
pub unsafe fn cudaStreamGetCaptureInfo(
    stream: cudaStream_t,
) -> Result<
    (
        cudaStreamCaptureStatus,
        u64,
        cudaGraph_t,
        *const cudaGraphNode_t,
        *const cudaGraphEdgeData,
        usize,
    ),
    crate::sys::cudaError,
> {
    let mut out_1: std::mem::MaybeUninit<cudaStreamCaptureStatus> = std::mem::MaybeUninit::uninit();
    let mut out_2: std::mem::MaybeUninit<::std::os::raw::c_ulonglong> = std::mem::MaybeUninit::uninit();
    let mut out_3: std::mem::MaybeUninit<cudaGraph_t> = std::mem::MaybeUninit::uninit();
    let mut out_4: std::mem::MaybeUninit<*const cudaGraphNode_t> = std::mem::MaybeUninit::uninit();
    let mut out_5: std::mem::MaybeUninit<*const cudaGraphEdgeData> = std::mem::MaybeUninit::uninit();
    let mut out_6: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudaStreamGetCaptureInfo(
            stream,
            out_1.as_mut_ptr() as *mut _,
            out_2.as_mut_ptr() as *mut _,
            out_3.as_mut_ptr() as *mut _,
            out_4.as_mut_ptr() as *mut _,
            out_5.as_mut_ptr() as *mut _,
            out_6.as_mut_ptr() as *mut _,
        )
    };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe {
            Ok((
                out_1.assume_init() as cudaStreamCaptureStatus,
                out_2.assume_init() as u64,
                out_3.assume_init() as cudaGraph_t,
                out_4.assume_init() as *const cudaGraphNode_t,
                out_5.assume_init() as *const cudaGraphEdgeData,
                out_6.assume_init() as usize,
            ))
        }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Update the set of dependencies in a capturing stream\nModifies the dependency set of a capturing stream. The dependency set is the set\nof nodes that the next captured node in the stream will depend on.\nValid flags are ::cudaStreamAddCaptureDependencies and\n::cudaStreamSetCaptureDependencies. These control whether the set passed to\nthe API is added to the existing set or replaces it. A flags value of 0 defaults\nto ::cudaStreamAddCaptureDependencies.\nNodes that are removed from the dependency set via this API do not result in\n::cudaErrorStreamCaptureUnjoined if they are unreachable from the stream at\n::cudaStreamEndCapture.\nReturns ::cudaErrorIllegalState if the stream is not capturing.\n\n# Arguments\n\n* `stream` - - The stream to update\n* `dependencies` - - The set of dependencies to add\n* `dependencyData` - - Optional array of data associated with each dependency.\n* `numDependencies` - - The size of the dependencies array\n* `flags` - - See above\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorIllegalState\n\\notefnerr # See also\n\n> [`::cudaStreamBeginCapture,`]\n::cudaStreamGetCaptureInfo,"]
pub unsafe fn cudaStreamUpdateCaptureDependencies<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    stream: cudaStream_t,
    mut dependencies: T,
    dependencyData: U,
    numDependencies: usize,
    flags: u32,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaStreamUpdateCaptureDependencies(
            stream,
            dependencies.as_mut_ptr() as *mut _,
            dependencyData.as_const_ptr() as *const _,
            numDependencies,
            flags as _,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Records an event\nCaptures in `event` the contents of `stream` at the time of this call.\n`event` and `stream` must be on the same CUDA context.\nCalls such as ::cudaEventQuery() or ::cudaStreamWaitEvent() will then\nexamine or wait for completion of the work that was captured. Uses of\n`stream` after this call do not modify `event.` See note on default\nstream behavior for what is captured in the default case.\n::cudaEventRecord() can be called multiple times on the same event and\nwill overwrite the previously captured state. Other APIs such as\n::cudaStreamWaitEvent() use the most recently captured state at the time\nof the API call, and are not affected by later calls to\n::cudaEventRecord(). Before the first call to ::cudaEventRecord(), an\nevent represents an empty set of work, so for example ::cudaEventQuery()\nwould return ::cudaSuccess.\n\n# Arguments\n\n* `event` -  - Event to record\n* `stream` - - Stream in which to record event\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorInvalidResourceHandle,\n::cudaErrorLaunchFailure\n\\note_null_stream \\notefnerr \\note_init_rt \\note_callback \\note_null_event # See also\n\n> [`\\ref`] ::cudaEventCreate(cudaEvent_t*) \"cudaEventCreate (C API)\",\n::cudaEventCreateWithFlags, ::cudaEventQuery,\n::cudaEventSynchronize, ::cudaEventDestroy, ::cudaEventElapsedTime,\n::cudaStreamWaitEvent,\n::cudaEventRecordWithFlags,\n::cuEventRecord"]
pub unsafe fn cudaEventRecord(event: cudaEvent_t, stream: cudaStream_t) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaEventRecord(event, stream) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudaEventRecordWithFlags(
    event: cudaEvent_t,
    stream: cudaStream_t,
    flags: u32,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaEventRecordWithFlags(event, stream, flags as _) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Queries an event's status\nQueries the status of all work currently captured by `event.` See\n::cudaEventRecord() for details on what is captured by an event.\nReturns ::cudaSuccess if all captured work has been completed, or\n::cudaErrorNotReady if any captured work is incomplete.\nFor the purposes of Unified Memory, a return value of ::cudaSuccess\nis equivalent to having called ::cudaEventSynchronize().\n\n# Arguments\n\n* `event` - - Event to query\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorNotReady,\n::cudaErrorInvalidValue,\n::cudaErrorInvalidResourceHandle,\n::cudaErrorLaunchFailure\n\\notefnerr \\note_init_rt \\note_callback \\note_null_event # See also\n\n> [`\\ref`] ::cudaEventCreate(cudaEvent_t*) \"cudaEventCreate (C API)\",\n::cudaEventCreateWithFlags, ::cudaEventRecord,\n::cudaEventSynchronize, ::cudaEventDestroy, ::cudaEventElapsedTime,\n::cuEventQuery"]
pub unsafe fn cudaEventQuery(event: cudaEvent_t) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaEventQuery(event) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Waits for an event to complete\nWaits until the completion of all work currently captured in `event.`\nSee ::cudaEventRecord() for details on what is captured by an event.\nWaiting for an event that was created with the ::cudaEventBlockingSync\nflag will cause the calling CPU thread to block until the event has\nbeen completed by the device.  If the ::cudaEventBlockingSync flag has\nnot been set, then the CPU thread will busy-wait until the event has\nbeen completed by the device.\n\n# Arguments\n\n* `event` - - Event to wait for\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorInvalidResourceHandle,\n::cudaErrorLaunchFailure\n\\notefnerr \\note_init_rt \\note_callback \\note_null_event # See also\n\n> [`\\ref`] ::cudaEventCreate(cudaEvent_t*) \"cudaEventCreate (C API)\",\n::cudaEventCreateWithFlags, ::cudaEventRecord,\n::cudaEventQuery, ::cudaEventDestroy, ::cudaEventElapsedTime,\n::cuEventSynchronize"]
pub unsafe fn cudaEventSynchronize(event: cudaEvent_t) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaEventSynchronize(event) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Computes the elapsed time between events\nComputes the elapsed time between two events (in milliseconds with a\nresolution of around 0.5 microseconds). Note this API is not guaranteed\nto return the latest errors for pending work. As such this API is intended to\nserve as a elapsed time calculation only and polling for completion on the\nevents to be compared should be done with ::cudaEventQuery instead.\nIf either event was last recorded in a non-NULL stream, the resulting time\nmay be greater than expected (even if both used the same stream handle). This\nhappens because the ::cudaEventRecord() operation takes place asynchronously\nand there is no guarantee that the measured latency is actually just between\nthe two events. Any number of other different stream operations could execute\nin between the two measured events, thus altering the timing in a significant\nway.\nIf ::cudaEventRecord() has not been called on either event, then\n::cudaErrorInvalidResourceHandle is returned. If ::cudaEventRecord() has been\ncalled on both events but one or both of them has not yet been completed\n(that is, ::cudaEventQuery() would return ::cudaErrorNotReady on at least one\nof the events), ::cudaErrorNotReady is returned. If either event was created\nwith the ::cudaEventDisableTiming flag, then this function will return\n::cudaErrorInvalidResourceHandle.\n\n# Arguments\n\n* `ms` -    - Time between `start` and `end` in ms\n* `start` - - Starting event\n* `end` -   - Ending event\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorNotReady,\n::cudaErrorInvalidValue,\n::cudaErrorInvalidResourceHandle,\n::cudaErrorLaunchFailure,\n::cudaErrorUnknown\n\\notefnerr \\note_init_rt \\note_callback \\note_null_event # See also\n\n> [`\\ref`] ::cudaEventCreate(cudaEvent_t*) \"cudaEventCreate (C API)\",\n::cudaEventCreateWithFlags, ::cudaEventQuery,\n::cudaEventSynchronize, ::cudaEventDestroy, ::cudaEventRecord,\n::cuEventElapsedTime"]
pub unsafe fn cudaEventElapsedTime<T: types::CudaAsPtr>(
    mut ms: T,
    start: cudaEvent_t,
    end: cudaEvent_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaEventElapsedTime(ms.as_mut_ptr() as *mut _, start, end) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Imports an external memory object\nImports an externally allocated memory object and returns\na handle to that in `extMem_out.`\nThe properties of the handle being imported must be described in\n`memHandleDesc.` The ::cudaExternalMemoryHandleDesc structure\nis defined as follows:\n\\code typedef struct cudaExternalMemoryHandleDesc_st {\ncudaExternalMemoryHandleType type;\nunion {\nint fd;\nstruct {\nvoid *handle;\nconst void *name;\n} win32;\nconst void *nvSciBufObject;\n} handle;\nunsigned long long size;\nunsigned int flags;\n} cudaExternalMemoryHandleDesc;\n\\endcode where ::cudaExternalMemoryHandleDesc::type specifies the type\nof handle being imported. ::cudaExternalMemoryHandleType is\ndefined as:\n\\code typedef enum cudaExternalMemoryHandleType_enum {\ncudaExternalMemoryHandleTypeOpaqueFd         = 1,\ncudaExternalMemoryHandleTypeOpaqueWin32      = 2,\ncudaExternalMemoryHandleTypeOpaqueWin32Kmt   = 3,\ncudaExternalMemoryHandleTypeD3D12Heap        = 4,\ncudaExternalMemoryHandleTypeD3D12Resource    = 5,\ncudaExternalMemoryHandleTypeD3D11Resource    = 6,\ncudaExternalMemoryHandleTypeD3D11ResourceKmt = 7,\ncudaExternalMemoryHandleTypeNvSciBuf         = 8\n} cudaExternalMemoryHandleType;\n\\endcode If ::cudaExternalMemoryHandleDesc::type is\n::cudaExternalMemoryHandleTypeOpaqueFd, then\n::cudaExternalMemoryHandleDesc::handle::fd must be a valid\nfile descriptor referencing a memory object. Ownership of\nthe file descriptor is transferred to the CUDA driver when the\nhandle is imported successfully. Performing any operations on the\nfile descriptor after it is imported results in undefined behavior.\nIf ::cudaExternalMemoryHandleDesc::type is\n::cudaExternalMemoryHandleTypeOpaqueWin32, then exactly one\nof ::cudaExternalMemoryHandleDesc::handle::win32::handle and\n::cudaExternalMemoryHandleDesc::handle::win32::name must not be\nNULL. If ::cudaExternalMemoryHandleDesc::handle::win32::handle\nis not NULL, then it must represent a valid shared NT handle that\nreferences a memory object. Ownership of this handle is\nnot transferred to CUDA after the import operation, so the\napplication must release the handle using the appropriate system\ncall. If ::cudaExternalMemoryHandleDesc::handle::win32::name\nis not NULL, then it must point to a NULL-terminated array of\nUTF-16 characters that refers to a memory object.\nIf ::cudaExternalMemoryHandleDesc::type is\n::cudaExternalMemoryHandleTypeOpaqueWin32Kmt, then\n::cudaExternalMemoryHandleDesc::handle::win32::handle must\nbe non-NULL and\n::cudaExternalMemoryHandleDesc::handle::win32::name\nmust be NULL. The handle specified must be a globally shared KMT\nhandle. This handle does not hold a reference to the underlying\nobject, and thus will be invalid when all references to the\nmemory object are destroyed.\nIf ::cudaExternalMemoryHandleDesc::type is\n::cudaExternalMemoryHandleTypeD3D12Heap, then exactly one\nof ::cudaExternalMemoryHandleDesc::handle::win32::handle and\n::cudaExternalMemoryHandleDesc::handle::win32::name must not be\nNULL. If ::cudaExternalMemoryHandleDesc::handle::win32::handle\nis not NULL, then it must represent a valid shared NT handle that\nis returned by ID3D12Device::CreateSharedHandle when referring to a\nID3D12Heap object. This handle holds a reference to the underlying\nobject. If ::cudaExternalMemoryHandleDesc::handle::win32::name\nis not NULL, then it must point to a NULL-terminated array of\nUTF-16 characters that refers to a ID3D12Heap object.\nIf ::cudaExternalMemoryHandleDesc::type is\n::cudaExternalMemoryHandleTypeD3D12Resource, then exactly one\nof ::cudaExternalMemoryHandleDesc::handle::win32::handle and\n::cudaExternalMemoryHandleDesc::handle::win32::name must not be\nNULL. If ::cudaExternalMemoryHandleDesc::handle::win32::handle\nis not NULL, then it must represent a valid shared NT handle that\nis returned by ID3D12Device::CreateSharedHandle when referring to a\nID3D12Resource object. This handle holds a reference to the\nunderlying object. If\n::cudaExternalMemoryHandleDesc::handle::win32::name\nis not NULL, then it must point to a NULL-terminated array of\nUTF-16 characters that refers to a ID3D12Resource object.\nIf ::cudaExternalMemoryHandleDesc::type is\n::cudaExternalMemoryHandleTypeD3D11Resource,then exactly one\nof ::cudaExternalMemoryHandleDesc::handle::win32::handle and\n::cudaExternalMemoryHandleDesc::handle::win32::name must not be\nNULL. If ::cudaExternalMemoryHandleDesc::handle::win32::handle is\nnot NULL, then it must represent a valid shared NT handle that is\nreturned by  IDXGIResource1::CreateSharedHandle when referring to a\nID3D11Resource object. If\n::cudaExternalMemoryHandleDesc::handle::win32::name\nis not NULL, then it must point to a NULL-terminated array of\nUTF-16 characters that refers to a ID3D11Resource object.\nIf ::cudaExternalMemoryHandleDesc::type is\n::cudaExternalMemoryHandleTypeD3D11ResourceKmt, then\n::cudaExternalMemoryHandleDesc::handle::win32::handle must\nbe non-NULL and ::cudaExternalMemoryHandleDesc::handle::win32::name\nmust be NULL. The handle specified must be a valid shared KMT\nhandle that is returned by IDXGIResource::GetSharedHandle when\nreferring to a ID3D11Resource object.\nIf ::cudaExternalMemoryHandleDesc::type is\n::cudaExternalMemoryHandleTypeNvSciBuf, then\n::cudaExternalMemoryHandleDesc::handle::nvSciBufObject must be NON-NULL\nand reference a valid NvSciBuf object.\nIf the NvSciBuf object imported into CUDA is also mapped by other drivers, then the\napplication must use ::cudaWaitExternalSemaphoresAsync or ::cudaSignalExternalSemaphoresAsync\nas approprriate barriers to maintain coherence between CUDA and the other drivers.\nSee ::cudaExternalSemaphoreWaitSkipNvSciBufMemSync and ::cudaExternalSemaphoreSignalSkipNvSciBufMemSync\nfor memory synchronization.\nThe size of the memory object must be specified in\n::cudaExternalMemoryHandleDesc::size.\nSpecifying the flag ::cudaExternalMemoryDedicated in\n::cudaExternalMemoryHandleDesc::flags indicates that the\nresource is a dedicated resource. The definition of what a\ndedicated resource is outside the scope of this extension.\nThis flag must be set if ::cudaExternalMemoryHandleDesc::type\nis one of the following:\n::cudaExternalMemoryHandleTypeD3D12Resource\n::cudaExternalMemoryHandleTypeD3D11Resource\n::cudaExternalMemoryHandleTypeD3D11ResourceKmt\n\n# Arguments\n\n* `extMem_out` -    - Returned handle to an external memory object\n* `memHandleDesc` - - Memory import handle descriptor\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorInvalidResourceHandle,\n::cudaErrorOperatingSystem\n\\notefnerr \\note_init_rt \\note_callback > **Note** If the Vulkan memory imported into CUDA is mapped on the CPU then the\napplication must use vkInvalidateMappedMemoryRanges/vkFlushMappedMemoryRanges\nas well as appropriate Vulkan pipeline barriers to maintain coherence between\nCPU and GPU. For more information on these APIs, please refer to \"Synchronization\nand Cache Control\" chapter from Vulkan specification.\n\n# See also\n\n> [`::cudaDestroyExternalMemory,`]\n::cudaExternalMemoryGetMappedBuffer,\n::cudaExternalMemoryGetMappedMipmappedArray"]
pub unsafe fn cudaImportExternalMemory<T: types::CudaAsPtr>(
    mut extMem_out: T,
    memHandleDesc: *const cudaExternalMemoryHandleDesc,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaImportExternalMemory(extMem_out.as_mut_ptr() as *mut _, memHandleDesc) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Maps a buffer onto an imported memory object\nMaps a buffer onto an imported memory object and returns a device\npointer in `devPtr.`\nThe properties of the buffer being mapped must be described in\n`bufferDesc.` The ::cudaExternalMemoryBufferDesc structure is\ndefined as follows:\n\\code typedef struct cudaExternalMemoryBufferDesc_st {\nunsigned long long offset;\nunsigned long long size;\nunsigned int flags;\n} cudaExternalMemoryBufferDesc;\n\\endcode where ::cudaExternalMemoryBufferDesc::offset is the offset in\nthe memory object where the buffer's base address is.\n::cudaExternalMemoryBufferDesc::size is the size of the buffer.\n::cudaExternalMemoryBufferDesc::flags must be zero.\nThe offset and size have to be suitably aligned to match the\nrequirements of the external API. Mapping two buffers whose ranges\noverlap may or may not result in the same virtual address being\nreturned for the overlapped portion. In such cases, the application\nmust ensure that all accesses to that region from the GPU are\nvolatile. Otherwise writes made via one address are not guaranteed\nto be visible via the other address, even if they're issued by the\nsame thread. It is recommended that applications map the combined\nrange instead of mapping separate buffers and then apply the\nappropriate offsets to the returned pointer to derive the\nindividual buffers.\nThe returned pointer `devPtr` must be freed using ::cudaFree.\n\n# Arguments\n\n* `devPtr` -     - Returned device pointer to buffer\n* `extMem` -     - Handle to external memory object\n* `bufferDesc` - - Buffer descriptor\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorInvalidResourceHandle\n\\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaImportExternalMemory,`]\n::cudaDestroyExternalMemory,\n::cudaExternalMemoryGetMappedMipmappedArray"]
pub unsafe fn cudaExternalMemoryGetMappedBuffer<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    mut devPtr: T,
    extMem: cudaExternalMemory_t,
    bufferDesc: U,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaExternalMemoryGetMappedBuffer(
            devPtr.as_mut_ptr() as *mut _,
            extMem,
            bufferDesc.as_const_ptr() as *const _,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Maps a CUDA mipmapped array onto an external memory object\nMaps a CUDA mipmapped array onto an external object and returns a\nhandle to it in `mipmap.`\nThe properties of the CUDA mipmapped array being mapped must be\ndescribed in `mipmapDesc.` The structure\n::cudaExternalMemoryMipmappedArrayDesc is defined as follows:\n\\code typedef struct cudaExternalMemoryMipmappedArrayDesc_st {\nunsigned long long offset;\ncudaChannelFormatDesc formatDesc;\ncudaExtent extent;\nunsigned int flags;\nunsigned int numLevels;\n} cudaExternalMemoryMipmappedArrayDesc;\n\\endcode where ::cudaExternalMemoryMipmappedArrayDesc::offset is the\noffset in the memory object where the base level of the mipmap\nchain is.\n::cudaExternalMemoryMipmappedArrayDesc::formatDesc describes the\nformat of the data.\n::cudaExternalMemoryMipmappedArrayDesc::extent specifies the\ndimensions of the base level of the mipmap chain.\n::cudaExternalMemoryMipmappedArrayDesc::flags are flags associated\nwith CUDA mipmapped arrays. For further details, please refer to\nthe documentation for ::cudaMalloc3DArray. Note that if the mipmapped\narray is bound as a color target in the graphics API, then the flag\n::cudaArrayColorAttachment must be specified in\n::cudaExternalMemoryMipmappedArrayDesc::flags.\n::cudaExternalMemoryMipmappedArrayDesc::numLevels specifies\nthe total number of levels in the mipmap chain.\nThe returned CUDA mipmapped array must be freed using ::cudaFreeMipmappedArray.\n\n# Arguments\n\n* `mipmap` -     - Returned CUDA mipmapped array\n* `extMem` -     - Handle to external memory object\n* `mipmapDesc` - - CUDA array descriptor\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorInvalidResourceHandle\n\\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaImportExternalMemory,`]\n::cudaDestroyExternalMemory,\n::cudaExternalMemoryGetMappedBuffer\n> **Note** If ::cudaExternalMemoryHandleDesc::type is\n::cudaExternalMemoryHandleTypeNvSciBuf, then\n::cudaExternalMemoryMipmappedArrayDesc::numLevels must not be greater than 1."]
pub unsafe fn cudaExternalMemoryGetMappedMipmappedArray(
    extMem: cudaExternalMemory_t,
    mipmapDesc: *const cudaExternalMemoryMipmappedArrayDesc,
) -> Result<cudaMipmappedArray_t, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<cudaMipmappedArray_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudaExternalMemoryGetMappedMipmappedArray(out_0.as_mut_ptr() as *mut _, extMem, mipmapDesc)
    };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_0.assume_init() as cudaMipmappedArray_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Imports an external semaphore\nImports an externally allocated synchronization object and returns\na handle to that in `extSem_out.`\nThe properties of the handle being imported must be described in\n`semHandleDesc.` The ::cudaExternalSemaphoreHandleDesc is defined\nas follows:\n\\code typedef struct cudaExternalSemaphoreHandleDesc_st {\ncudaExternalSemaphoreHandleType type;\nunion {\nint fd;\nstruct {\nvoid *handle;\nconst void *name;\n} win32;\nconst void* NvSciSyncObj;\n} handle;\nunsigned int flags;\n} cudaExternalSemaphoreHandleDesc;\n\\endcode where ::cudaExternalSemaphoreHandleDesc::type specifies the type of\nhandle being imported. ::cudaExternalSemaphoreHandleType is defined\nas:\n\\code typedef enum cudaExternalSemaphoreHandleType_enum {\ncudaExternalSemaphoreHandleTypeOpaqueFd                = 1,\ncudaExternalSemaphoreHandleTypeOpaqueWin32             = 2,\ncudaExternalSemaphoreHandleTypeOpaqueWin32Kmt          = 3,\ncudaExternalSemaphoreHandleTypeD3D12Fence              = 4,\ncudaExternalSemaphoreHandleTypeD3D11Fence              = 5,\ncudaExternalSemaphoreHandleTypeNvSciSync               = 6,\ncudaExternalSemaphoreHandleTypeKeyedMutex              = 7,\ncudaExternalSemaphoreHandleTypeKeyedMutexKmt           = 8,\ncudaExternalSemaphoreHandleTypeTimelineSemaphoreFd     = 9,\ncudaExternalSemaphoreHandleTypeTimelineSemaphoreWin32  = 10\n} cudaExternalSemaphoreHandleType;\n\\endcode If ::cudaExternalSemaphoreHandleDesc::type is\n::cudaExternalSemaphoreHandleTypeOpaqueFd, then\n::cudaExternalSemaphoreHandleDesc::handle::fd must be a valid file\ndescriptor referencing a synchronization object. Ownership of the\nfile descriptor is transferred to the CUDA driver when the handle\nis imported successfully. Performing any operations on the file\ndescriptor after it is imported results in undefined behavior.\nIf ::cudaExternalSemaphoreHandleDesc::type is\n::cudaExternalSemaphoreHandleTypeOpaqueWin32, then exactly one of\n::cudaExternalSemaphoreHandleDesc::handle::win32::handle and\n::cudaExternalSemaphoreHandleDesc::handle::win32::name must not be\nNULL. If ::cudaExternalSemaphoreHandleDesc::handle::win32::handle\nis not NULL, then it must represent a valid shared NT handle that\nreferences a synchronization object. Ownership of this handle is\nnot transferred to CUDA after the import operation, so the\napplication must release the handle using the appropriate system\ncall. If ::cudaExternalSemaphoreHandleDesc::handle::win32::name is\nnot NULL, then it must name a valid synchronization object.\nIf ::cudaExternalSemaphoreHandleDesc::type is\n::cudaExternalSemaphoreHandleTypeOpaqueWin32Kmt, then\n::cudaExternalSemaphoreHandleDesc::handle::win32::handle must be\nnon-NULL and ::cudaExternalSemaphoreHandleDesc::handle::win32::name\nmust be NULL. The handle specified must be a globally shared KMT\nhandle. This handle does not hold a reference to the underlying\nobject, and thus will be invalid when all references to the\nsynchronization object are destroyed.\nIf ::cudaExternalSemaphoreHandleDesc::type is\n::cudaExternalSemaphoreHandleTypeD3D12Fence, then exactly one of\n::cudaExternalSemaphoreHandleDesc::handle::win32::handle and\n::cudaExternalSemaphoreHandleDesc::handle::win32::name must not be\nNULL. If ::cudaExternalSemaphoreHandleDesc::handle::win32::handle\nis not NULL, then it must represent a valid shared NT handle that\nis returned by ID3D12Device::CreateSharedHandle when referring to a\nID3D12Fence object. This handle holds a reference to the underlying\nobject. If ::cudaExternalSemaphoreHandleDesc::handle::win32::name\nis not NULL, then it must name a valid synchronization object that\nrefers to a valid ID3D12Fence object.\nIf ::cudaExternalSemaphoreHandleDesc::type is\n::cudaExternalSemaphoreHandleTypeD3D11Fence, then exactly one of\n::cudaExternalSemaphoreHandleDesc::handle::win32::handle and\n::cudaExternalSemaphoreHandleDesc::handle::win32::name must not be\nNULL. If ::cudaExternalSemaphoreHandleDesc::handle::win32::handle\nis not NULL, then it must represent a valid shared NT handle that\nis returned by ID3D11Fence::CreateSharedHandle. If\n::cudaExternalSemaphoreHandleDesc::handle::win32::name\nis not NULL, then it must name a valid synchronization object that\nrefers to a valid ID3D11Fence object.\nIf ::cudaExternalSemaphoreHandleDesc::type is\n::cudaExternalSemaphoreHandleTypeNvSciSync, then\n::cudaExternalSemaphoreHandleDesc::handle::nvSciSyncObj\nrepresents a valid NvSciSyncObj.\n::cudaExternalSemaphoreHandleTypeKeyedMutex, then exactly one of\n::cudaExternalSemaphoreHandleDesc::handle::win32::handle and\n::cudaExternalSemaphoreHandleDesc::handle::win32::name must not be\nNULL. If ::cudaExternalSemaphoreHandleDesc::handle::win32::handle\nis not NULL, then it represent a valid shared NT handle that\nis returned by IDXGIResource1::CreateSharedHandle when referring to\na IDXGIKeyedMutex object.\nIf ::cudaExternalSemaphoreHandleDesc::type is\n::cudaExternalSemaphoreHandleTypeKeyedMutexKmt, then\n::cudaExternalSemaphoreHandleDesc::handle::win32::handle must be\nnon-NULL and ::cudaExternalSemaphoreHandleDesc::handle::win32::name\nmust be NULL. The handle specified must represent a valid KMT\nhandle that is returned by IDXGIResource::GetSharedHandle when\nreferring to a IDXGIKeyedMutex object.\nIf ::cudaExternalSemaphoreHandleDesc::type is\n::cudaExternalSemaphoreHandleTypeTimelineSemaphoreFd, then\n::cudaExternalSemaphoreHandleDesc::handle::fd must be a valid file\ndescriptor referencing a synchronization object. Ownership of the\nfile descriptor is transferred to the CUDA driver when the handle\nis imported successfully. Performing any operations on the file\ndescriptor after it is imported results in undefined behavior.\nIf ::cudaExternalSemaphoreHandleDesc::type is\n::cudaExternalSemaphoreHandleTypeTimelineSemaphoreWin32, then exactly one of\n::cudaExternalSemaphoreHandleDesc::handle::win32::handle and\n::cudaExternalSemaphoreHandleDesc::handle::win32::name must not be\nNULL. If ::cudaExternalSemaphoreHandleDesc::handle::win32::handle\nis not NULL, then it must represent a valid shared NT handle that\nreferences a synchronization object. Ownership of this handle is\nnot transferred to CUDA after the import operation, so the\napplication must release the handle using the appropriate system\ncall. If ::cudaExternalSemaphoreHandleDesc::handle::win32::name is\nnot NULL, then it must name a valid synchronization object.\n\n# Arguments\n\n* `extSem_out` -    - Returned handle to an external semaphore\n* `semHandleDesc` - - Semaphore import handle descriptor\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidResourceHandle,\n::cudaErrorOperatingSystem\n\\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaDestroyExternalSemaphore,`]\n::cudaSignalExternalSemaphoresAsync,\n::cudaWaitExternalSemaphoresAsync"]
pub unsafe fn cudaImportExternalSemaphore<T: types::CudaAsPtr>(
    mut extSem_out: T,
    semHandleDesc: *const cudaExternalSemaphoreHandleDesc,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaImportExternalSemaphore(extSem_out.as_mut_ptr() as *mut _, semHandleDesc) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Signals a set of external semaphore objects\nEnqueues a signal operation on a set of externally allocated\nsemaphore object in the specified stream. The operations will be\nexecuted when all prior operations in the stream complete.\nThe exact semantics of signaling a semaphore depends on the type of\nthe object.\nIf the semaphore object is any one of the following types:\n::cudaExternalSemaphoreHandleTypeOpaqueFd,\n::cudaExternalSemaphoreHandleTypeOpaqueWin32,\n::cudaExternalSemaphoreHandleTypeOpaqueWin32Kmt\nthen signaling the semaphore will set it to the signaled state.\nIf the semaphore object is any one of the following types:\n::cudaExternalSemaphoreHandleTypeD3D12Fence,\n::cudaExternalSemaphoreHandleTypeD3D11Fence,\n::cudaExternalSemaphoreHandleTypeTimelineSemaphoreFd,\n::cudaExternalSemaphoreHandleTypeTimelineSemaphoreWin32\nthen the semaphore will be set to the value specified in\n::cudaExternalSemaphoreSignalParams::params::fence::value.\nIf the semaphore object is of the type ::cudaExternalSemaphoreHandleTypeNvSciSync\nthis API sets ::cudaExternalSemaphoreSignalParams::params::nvSciSync::fence to a\nvalue that can be used by subsequent waiters of the same NvSciSync object to\norder operations with those currently submitted in `stream.` Such an update\nwill overwrite previous contents of\n::cudaExternalSemaphoreSignalParams::params::nvSciSync::fence. By default,\nsignaling such an external semaphore object causes appropriate memory synchronization\noperations to be performed over all the external memory objects that are imported as\n::cudaExternalMemoryHandleTypeNvSciBuf. This ensures that any subsequent accesses\nmade by other importers of the same set of NvSciBuf memory object(s) are coherent.\nThese operations can be skipped by specifying the flag\n::cudaExternalSemaphoreSignalSkipNvSciBufMemSync, which can be used as a\nperformance optimization when data coherency is not required. But specifying this\nflag in scenarios where data coherency is required results in undefined behavior.\nAlso, for semaphore object of the type ::cudaExternalSemaphoreHandleTypeNvSciSync,\nif the NvSciSyncAttrList used to create the NvSciSyncObj had not set the flags in\n::cudaDeviceGetNvSciSyncAttributes to cudaNvSciSyncAttrSignal, this API will return\ncudaErrorNotSupported.\n::cudaExternalSemaphoreSignalParams::params::nvSciSync::fence associated with\nsemaphore object of the type ::cudaExternalSemaphoreHandleTypeNvSciSync can be\ndeterministic. For this the NvSciSyncAttrList used to create the semaphore object\nmust have value of NvSciSyncAttrKey_RequireDeterministicFences key set to true.\nDeterministic fences allow users to enqueue a wait over the semaphore object even\nbefore corresponding signal is enqueued. For such a semaphore object, CUDA guarantees\nthat each signal operation will increment the fence value by '1'. Users are expected\nto track count of signals enqueued on the semaphore object and insert waits accordingly.\nWhen such a semaphore object is signaled from multiple streams, due to concurrent\nstream execution, it is possible that the order in which the semaphore gets signaled\nis indeterministic. This could lead to waiters of the semaphore getting unblocked\nincorrectly. Users are expected to handle such situations, either by not using the\nsame semaphore object with deterministic fence support enabled in different streams\nor by adding explicit dependency amongst such streams so that the semaphore is\nsignaled in order.\n::cudaExternalSemaphoreSignalParams::params::nvSciSync::fence associated with\nsemaphore object of the type ::cudaExternalSemaphoreHandleTypeNvSciSync can be\ntimestamp enabled. For this the NvSciSyncAttrList used to create the object must\nhave the value of NvSciSyncAttrKey_WaiterRequireTimestamps key set to true. Timestamps\nare emitted asynchronously by the GPU and CUDA saves the GPU timestamp in the\ncorresponding NvSciSyncFence at the time of signal on GPU. Users are expected to\nconvert GPU clocks to CPU clocks using appropriate scaling functions. Users are\nexpected to wait for the completion of the fence before extracting timestamp using\nappropriate NvSciSync APIs. Users are expected to ensure that there is only one\noutstanding timestamp enabled fence per Cuda-NvSciSync object at any point of time,\nfailing which leads to undefined behavior. Extracting the timestamp before the\ncorresponding fence is signalled could lead to undefined behaviour. Timestamp\nextracted via appropriate NvSciSync API would be in microseconds.\nIf the semaphore object is any one of the following types:\n::cudaExternalSemaphoreHandleTypeKeyedMutex,\n::cudaExternalSemaphoreHandleTypeKeyedMutexKmt,\nthen the keyed mutex will be released with the key specified in\n::cudaExternalSemaphoreSignalParams::params::keyedmutex::key.\n\n# Arguments\n\n* `extSemArray` - - Set of external semaphores to be signaled\n* `paramsArray` - - Array of semaphore parameters\n* `numExtSems` -  - Number of semaphores to signal\n* `stream` -     - Stream to enqueue the signal operations in\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidResourceHandle\n\\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaImportExternalSemaphore,`]\n::cudaDestroyExternalSemaphore,\n::cudaWaitExternalSemaphoresAsync"]
pub unsafe fn cudaSignalExternalSemaphoresAsync<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    extSemArray: T,
    paramsArray: U,
    numExtSems: u32,
    stream: cudaStream_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaSignalExternalSemaphoresAsync(
            extSemArray.as_const_ptr() as *const _,
            paramsArray.as_const_ptr() as *const _,
            numExtSems as _,
            stream,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Waits on a set of external semaphore objects\nEnqueues a wait operation on a set of externally allocated\nsemaphore object in the specified stream. The operations will be\nexecuted when all prior operations in the stream complete.\nThe exact semantics of waiting on a semaphore depends on the type\nof the object.\nIf the semaphore object is any one of the following types:\n::cudaExternalSemaphoreHandleTypeOpaqueFd,\n::cudaExternalSemaphoreHandleTypeOpaqueWin32,\n::cudaExternalSemaphoreHandleTypeOpaqueWin32Kmt\nthen waiting on the semaphore will wait until the semaphore reaches\nthe signaled state. The semaphore will then be reset to the\nunsignaled state. Therefore for every signal operation, there can\nonly be one wait operation.\nIf the semaphore object is any one of the following types:\n::cudaExternalSemaphoreHandleTypeD3D12Fence,\n::cudaExternalSemaphoreHandleTypeD3D11Fence,\n::cudaExternalSemaphoreHandleTypeTimelineSemaphoreFd,\n::cudaExternalSemaphoreHandleTypeTimelineSemaphoreWin32\nthen waiting on the semaphore will wait until the value of the\nsemaphore is greater than or equal to\n::cudaExternalSemaphoreWaitParams::params::fence::value.\nIf the semaphore object is of the type ::cudaExternalSemaphoreHandleTypeNvSciSync\nthen, waiting on the semaphore will wait until the\n::cudaExternalSemaphoreSignalParams::params::nvSciSync::fence is signaled by the\nsignaler of the NvSciSyncObj that was associated with this semaphore object.\nBy default, waiting on such an external semaphore object causes appropriate\nmemory synchronization operations to be performed over all external memory objects\nthat are imported as ::cudaExternalMemoryHandleTypeNvSciBuf. This ensures that\nany subsequent accesses made by other importers of the same set of NvSciBuf memory\nobject(s) are coherent. These operations can be skipped by specifying the flag\n::cudaExternalSemaphoreWaitSkipNvSciBufMemSync, which can be used as a\nperformance optimization when data coherency is not required. But specifying this\nflag in scenarios where data coherency is required results in undefined behavior.\nAlso, for semaphore object of the type ::cudaExternalSemaphoreHandleTypeNvSciSync,\nif the NvSciSyncAttrList used to create the NvSciSyncObj had not set the flags in\n::cudaDeviceGetNvSciSyncAttributes to cudaNvSciSyncAttrWait, this API will return\ncudaErrorNotSupported.\nIf the semaphore object is any one of the following types:\n::cudaExternalSemaphoreHandleTypeKeyedMutex,\n::cudaExternalSemaphoreHandleTypeKeyedMutexKmt,\nthen the keyed mutex will be acquired when it is released with the key specified\nin ::cudaExternalSemaphoreSignalParams::params::keyedmutex::key or\nuntil the timeout specified by\n::cudaExternalSemaphoreSignalParams::params::keyedmutex::timeoutMs\nhas lapsed. The timeout interval can either be a finite value\nspecified in milliseconds or an infinite value. In case an infinite\nvalue is specified the timeout never elapses. The windows INFINITE\nmacro must be used to specify infinite timeout\n\n# Arguments\n\n* `extSemArray` - - External semaphores to be waited on\n* `paramsArray` - - Array of semaphore parameters\n* `numExtSems` -  - Number of semaphores to wait on\n* `stream` -      - Stream to enqueue the wait operations in\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidResourceHandle\n::cudaErrorTimeout\n\\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaImportExternalSemaphore,`]\n::cudaDestroyExternalSemaphore,\n::cudaSignalExternalSemaphoresAsync"]
pub unsafe fn cudaWaitExternalSemaphoresAsync<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    extSemArray: T,
    paramsArray: U,
    numExtSems: u32,
    stream: cudaStream_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaWaitExternalSemaphoresAsync(
            extSemArray.as_const_ptr() as *const _,
            paramsArray.as_const_ptr() as *const _,
            numExtSems as _,
            stream,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Launches a device function\nThe function invokes kernel `func` on `gridDim` (`gridDim.x` &times; `gridDim.y`\n&times; `gridDim.z)` grid of blocks. Each block contains `blockDim` (`blockDim.x` &times;\n`blockDim.y` &times; `blockDim.z)` threads.\nIf the kernel has N parameters the `args` should point to array of N pointers.\nEach pointer, from <tt>args[0]</tt> to <tt>args[N - 1]</tt>, point to the region\nof memory from which the actual parameter will be copied.\nFor templated functions, pass the function symbol as follows:\nfunc_name<template_arg_0,...,template_arg_N>\n`sharedMem` sets the amount of dynamic shared memory that will be available to\neach thread block.\n`stream` specifies a stream the invocation is associated to.\n\n# Arguments\n\n* `func` -        - Device function symbol\n* `gridDim` -     - Grid dimentions\n* `blockDim` -    - Block dimentions\n* `args` -        - Arguments\n* `sharedMem` -   - Shared memory\n* `stream` -      - Stream identifier\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidDeviceFunction,\n::cudaErrorInvalidConfiguration,\n::cudaErrorLaunchFailure,\n::cudaErrorLaunchTimeout,\n::cudaErrorLaunchOutOfResources,\n::cudaErrorSharedObjectInitFailed,\n::cudaErrorInvalidPtx,\n::cudaErrorUnsupportedPtxVersion,\n::cudaErrorNoKernelImageForDevice,\n::cudaErrorJitCompilerNotFound,\n::cudaErrorJitCompilationDisabled\n\\note_null_stream \\notefnerr \\note_init_rt \\note_callback \\note_cudaKernel_t # See also\n\n> [`\\ref`] ::cudaLaunchKernel(const T *func, dim3 gridDim, dim3 blockDim, void **args, size_t sharedMem, cudaStream_t stream) \"cudaLaunchKernel (C++ API)\",\n::cuLaunchKernel"]
pub unsafe fn cudaLaunchKernel<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    func: T,
    gridDim: dim3,
    blockDim: dim3,
    mut args: U,
    sharedMem: usize,
    stream: cudaStream_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaLaunchKernel(
            func.as_const_ptr() as *const _,
            gridDim,
            blockDim,
            args.as_mut_ptr() as *mut _,
            sharedMem,
            stream,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Launches a CUDA function with launch-time configuration\nNote that the functionally equivalent variadic template ::cudaLaunchKernelEx\nis available for C++11 and newer.\nInvokes the kernel `func` on `config->gridDim` (`config->gridDim.x`\n&times; `config->gridDim.y` &times; `config->gridDim.z)` grid of blocks.\nEach block contains `config->blockDim` (`config->blockDim.x` &times;\n`config->blockDim.y` &times; `config->blockDim.z)` threads.\n`config->dynamicSmemBytes` sets the amount of dynamic shared memory that\nwill be available to each thread block.\n`config->stream` specifies a stream the invocation is associated to.\nConfiguration beyond grid and block dimensions, dynamic shared memory size,\nand stream can be provided with the following two fields of `config:`\n`config->attrs` is an array of `config->numAttrs` contiguous\n::cudaLaunchAttribute elements. The value of this pointer is not considered\nif `config->numAttrs` is zero. However, in that case, it is recommended to\nset the pointer to NULL.\n`config->numAttrs` is the number of attributes populating the first\n`config->numAttrs` positions of the `config->attrs` array.\nIf the kernel has N parameters the `args` should point to array of N\npointers. Each pointer, from <tt>args[0]</tt> to <tt>args[N - 1]</tt>, point\nto the region of memory from which the actual parameter will be copied.\nN.B. This function is so named to avoid unintentionally invoking the\ntemplated version, `cudaLaunchKernelEx,` for kernels taking a single\nvoid** or void* parameter.\n\n# Arguments\n\n* `config` - - Launch configuration\n* `func` -   - Kernel to launch\n* `args` -   - Array of pointers to kernel parameters\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidDeviceFunction,\n::cudaErrorInvalidConfiguration,\n::cudaErrorLaunchFailure,\n::cudaErrorLaunchTimeout,\n::cudaErrorLaunchOutOfResources,\n::cudaErrorSharedObjectInitFailed,\n::cudaErrorInvalidPtx,\n::cudaErrorUnsupportedPtxVersion,\n::cudaErrorNoKernelImageForDevice,\n::cudaErrorJitCompilerNotFound,\n::cudaErrorJitCompilationDisabled\n\\note_null_stream \\notefnerr \\note_init_rt \\note_callback \\note_cudaKernel_t # See also\n\n> [`\\ref`] ::cudaLaunchKernelEx(const cudaLaunchConfig_t *config, void (*kernel)(ExpTypes...), ActTypes &&... args) \"cudaLaunchKernelEx (C++ API)\",\n::cuLaunchKernelEx"]
pub unsafe fn cudaLaunchKernelExC<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    config: T,
    func: U,
    mut args: V,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaLaunchKernelExC(
            config.as_const_ptr() as *const _,
            func.as_const_ptr() as *const _,
            args.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Launches a device function where thread blocks can cooperate and synchronize as they execute\nThe function invokes kernel `func` on `gridDim` (`gridDim.x` &times; `gridDim.y`\n&times; `gridDim.z)` grid of blocks. Each block contains `blockDim` (`blockDim.x` &times;\n`blockDim.y` &times; `blockDim.z)` threads.\nThe device on which this kernel is invoked must have a non-zero value for\nthe device attribute ::cudaDevAttrCooperativeLaunch.\nThe total number of blocks launched cannot exceed the maximum number of blocks per\nmultiprocessor as returned by ::cudaOccupancyMaxActiveBlocksPerMultiprocessor (or\n::cudaOccupancyMaxActiveBlocksPerMultiprocessorWithFlags) times the number of multiprocessors\nas specified by the device attribute ::cudaDevAttrMultiProcessorCount.\nThe kernel cannot make use of CUDA dynamic parallelism.\nIf the kernel has N parameters the `args` should point to array of N pointers.\nEach pointer, from <tt>args[0]</tt> to <tt>args[N - 1]</tt>, point to the region\nof memory from which the actual parameter will be copied.\nFor templated functions, pass the function symbol as follows:\nfunc_name<template_arg_0,...,template_arg_N>\n`sharedMem` sets the amount of dynamic shared memory that will be available to\neach thread block.\n`stream` specifies a stream the invocation is associated to.\n\n# Arguments\n\n* `func` -        - Device function symbol\n* `gridDim` -     - Grid dimentions\n* `blockDim` -    - Block dimentions\n* `args` -        - Arguments\n* `sharedMem` -   - Shared memory\n* `stream` -      - Stream identifier\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidDeviceFunction,\n::cudaErrorInvalidConfiguration,\n::cudaErrorLaunchFailure,\n::cudaErrorLaunchTimeout,\n::cudaErrorLaunchOutOfResources,\n::cudaErrorCooperativeLaunchTooLarge,\n::cudaErrorSharedObjectInitFailed\n\\note_null_stream \\notefnerr \\note_init_rt \\note_callback \\note_cudaKernel_t # See also\n\n> [`\\ref`] ::cudaLaunchCooperativeKernel(const T *func, dim3 gridDim, dim3 blockDim, void **args, size_t sharedMem, cudaStream_t stream) \"cudaLaunchCooperativeKernel (C++ API)\",\n::cuLaunchCooperativeKernel"]
pub unsafe fn cudaLaunchCooperativeKernel<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    func: T,
    gridDim: dim3,
    blockDim: dim3,
    mut args: U,
    sharedMem: usize,
    stream: cudaStream_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaLaunchCooperativeKernel(
            func.as_const_ptr() as *const _,
            gridDim,
            blockDim,
            args.as_mut_ptr() as *mut _,
            sharedMem,
            stream,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Sets the preferred cache configuration for a device function\nOn devices where the L1 cache and shared memory use the same hardware\nresources, this sets through `cacheConfig` the preferred cache configuration\nfor the function specified via `func.` This is only a preference. The\nruntime will use the requested configuration if possible, but it is free to\nchoose a different configuration if required to execute `func.`\n`func` is a device function symbol and must be declared as a\n`__global__` function. If the specified function does not exist,\nthen ::cudaErrorInvalidDeviceFunction is returned. For templated functions,\npass the function symbol as follows: func_name<template_arg_0,...,template_arg_N>\nThis setting does nothing on devices where the size of the L1 cache and\nshared memory are fixed.\nLaunching a kernel with a different preference than the most recent\npreference setting may insert a device-side synchronization point.\nThe supported cache configurations are:\n- ::cudaFuncCachePreferNone: no preference for shared memory or L1 (default)\n- ::cudaFuncCachePreferShared: prefer larger shared memory and smaller L1 cache\n- ::cudaFuncCachePreferL1: prefer larger L1 cache and smaller shared memory\n- ::cudaFuncCachePreferEqual: prefer equal size L1 cache and shared memory\n\n# Arguments\n\n* `func` -        - Device function symbol\n* `cacheConfig` - - Requested cache configuration\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidDeviceFunction\n\\notefnerr \\note_string_api_deprecation2 \\note_init_rt \\note_callback > **Note** This API does not accept a ::cudaKernel_t casted as void*. If cache config modification\nis required for a ::cudaKernel_t (or a __global__ function), it can be replaced with a call to\n::cudaFuncSetAttributes with the attribute ::cudaFuncAttributePreferredSharedMemoryCarveout\nto specify a more granular L1 cache and shared memory split configuration.\n\n# See also\n\n> [`\\ref`] ::cudaFuncSetCacheConfig(T*, enum cudaFuncCache) \"cudaFuncSetCacheConfig (C++ API)\",\n[`::cudaFuncGetAttributes(struct`] cudaFuncAttributes*, const void*) \"cudaFuncGetAttributes (C API)\",\n[`::cudaLaunchKernel(const`] void *func, dim3 gridDim, dim3 blockDim, void **args, size_t sharedMem, cudaStream_t stream) \"cudaLaunchKernel (C API)\",\n::cuFuncSetCacheConfig"]
pub unsafe fn cudaFuncSetCacheConfig<T: types::CudaAsPtr>(
    func: T,
    cacheConfig: cudaFuncCache,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaFuncSetCacheConfig(func.as_const_ptr() as *const _, cacheConfig) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Find out attributes for a given function\nThis function obtains the attributes of a function specified via `func.`\n`func` is a device function symbol and must be declared as a\n`__global__` function. The fetched attributes are placed in `attr.`\nIf the specified function does not exist, then it is assumed to\nbe a ::cudaKernel_t and used as is.\nFor templated functions, pass the function symbol as follows:\nfunc_name<template_arg_0,...,template_arg_N>\nNote that some function attributes such as\n[`::cudaFuncAttributes::maxThreadsPerBlock`] \"maxThreadsPerBlock\"\nmay vary based on the device that is currently being used.\n\n# Arguments\n\n* `attr` - - Return pointer to function's attributes\n* `func` - - Device function symbol\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidDeviceFunction\n\\notefnerr \\note_string_api_deprecation2 \\note_init_rt \\note_callback \\note_cudaKernel_t # See also\n\n> [`\\ref`] ::cudaFuncSetCacheConfig(const void*, enum cudaFuncCache) \"cudaFuncSetCacheConfig (C API)\",\n[`::cudaFuncGetAttributes(struct`] cudaFuncAttributes*, T*) \"cudaFuncGetAttributes (C++ API)\",\n[`::cudaLaunchKernel(const`] void *func, dim3 gridDim, dim3 blockDim, void **args, size_t sharedMem, cudaStream_t stream) \"cudaLaunchKernel (C API)\",\n::cuFuncGetAttribute"]
pub unsafe fn cudaFuncGetAttributes(
    func: *const ::std::os::raw::c_void,
) -> Result<cudaFuncAttributes, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<cudaFuncAttributes> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaFuncGetAttributes(out_0.as_mut_ptr() as *mut _, func) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_0.assume_init() as cudaFuncAttributes) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Set attributes for a given function\nThis function sets the attributes of a function specified via `func.`\nThe parameter `func` must be a pointer to a function that executes\non the device. The parameter specified by `func` must be declared as a `__global__`\nfunction. The enumeration defined by `attr` is set to the value defined by `value.`\nIf the specified function does not exist, then it is assumed to\nbe a ::cudaKernel_t and used as is.\nIf the specified attribute cannot be written, or if the value is incorrect,\nthen ::cudaErrorInvalidValue is returned.\nValid values for `attr` are:\n- ::cudaFuncAttributeMaxDynamicSharedMemorySize - The requested maximum size in bytes of dynamically-allocated shared memory. The sum of this value and the function attribute ::sharedSizeBytes\ncannot exceed the device attribute ::cudaDevAttrMaxSharedMemoryPerBlockOptin. The maximal size of requestable dynamic shared memory may differ by GPU architecture.\n- ::cudaFuncAttributePreferredSharedMemoryCarveout - On devices where the L1 cache and shared memory use the same hardware resources,\nthis sets the shared memory carveout preference, in percent of the total shared memory. See ::cudaDevAttrMaxSharedMemoryPerMultiprocessor.\nThis is only a hint, and the driver can choose a different ratio if required to execute the function.\n- ::cudaFuncAttributeRequiredClusterWidth: The required cluster width in\nblocks. The width, height, and depth values must either all be 0 or all be\npositive. The validity of the cluster dimensions is checked at launch time.\nIf the value is set during compile time, it cannot be set at runtime.\nSetting it at runtime will return cudaErrorNotPermitted.\n- ::cudaFuncAttributeRequiredClusterHeight: The required cluster height in\nblocks. The width, height, and depth values must either all be 0 or all be\npositive. The validity of the cluster dimensions is checked at launch time.\nIf the value is set during compile time, it cannot be set at runtime.\nSetting it at runtime will return cudaErrorNotPermitted.\n- ::cudaFuncAttributeRequiredClusterDepth: The required cluster depth in\nblocks. The width, height, and depth values must either all be 0 or all be\npositive. The validity of the cluster dimensions is checked at launch time.\nIf the value is set during compile time, it cannot be set at runtime.\nSetting it at runtime will return cudaErrorNotPermitted.\n- ::cudaFuncAttributeNonPortableClusterSizeAllowed: Indicates whether the\nfunction can be launched with non-portable cluster size. 1 is allowed, 0 is\ndisallowed.\n- ::cudaFuncAttributeClusterSchedulingPolicyPreference: The block\nscheduling policy of a function. The value type is cudaClusterSchedulingPolicy.\n\n# Arguments\n\n* `func` -  - Function to get attributes of\n* `attr` -  - Attribute to set\n* `value` - - Value to set\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidDeviceFunction,\n::cudaErrorInvalidValue\n\\notefnerr \\note_init_rt \\note_callback \\note_cudaKernel_t [`::cudaLaunchKernel(const`] T *func, dim3 gridDim, dim3 blockDim, void **args, size_t sharedMem, cudaStream_t stream) \"cudaLaunchKernel (C++ API)\",\n[`::cudaFuncSetCacheConfig(T*,`] enum cudaFuncCache) \"cudaFuncSetCacheConfig (C++ API)\",\n[`::cudaFuncGetAttributes(struct`] cudaFuncAttributes*, const void*) \"cudaFuncGetAttributes (C API)\","]
pub unsafe fn cudaFuncSetAttribute<T: types::CudaAsPtr>(
    func: T,
    attr: cudaFuncAttribute,
    value: i32,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaFuncSetAttribute(func.as_const_ptr() as *const _, attr, value as _) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Returns the function name for a device entry function pointer.\nReturns in `**name` the function name associated with the symbol `func` .\nThe function name is returned as a null-terminated string. This API may\nreturn a mangled name if the function is not declared as having C linkage.\nIf `**name` is NULL, ::cudaErrorInvalidValue is returned.\nIf `func` is not a device entry function, then it is assumed to\nbe a ::cudaKernel_t and used as is.\n\n# Arguments\n\n* `name` - - The returned name of the function\n* `func` - - The function pointer to retrieve name for\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorInvalidDeviceFunction\n\\notefnerr \\note_init_rt \\note_callback \\note_cudaKernel_t [`::cudaFuncGetName(const`] char **name, const T *func) \"cudaFuncGetName (C++ API)\""]
pub unsafe fn cudaFuncGetName<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    mut name: T,
    func: U,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaFuncGetName(name.as_mut_ptr() as *mut _, func.as_const_ptr() as *const _) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Returns the offset and size of a kernel parameter in the device-side parameter layout.\nQueries the kernel parameter at `paramIndex` in `func's` list of parameters and returns\nparameter information via `paramOffset` and `paramSize.` `paramOffset` returns the\noffset of the parameter in the device-side parameter layout. `paramSize` returns the size\nin bytes of the parameter. This information can be used to update kernel node parameters\nfrom the device via ::cudaGraphKernelNodeSetParam() and ::cudaGraphKernelNodeUpdatesApply().\n`paramIndex` must be less than the number of parameters that `func` takes.\n\n# Arguments\n\n* `func` -        - The function to query\n* `paramIndex` -  - The parameter index to query\n* `paramOffset` - - The offset into the device-side parameter layout at which the parameter resides\n* `paramSize` -   - The size of the parameter in the device-side parameter layout\n\n# Returns\n\n::CUDA_SUCCESS,\n::CUDA_ERROR_INVALID_VALUE,\n\\notefnerr \\note_cudaKernel_t "]
pub unsafe fn cudaFuncGetParamInfo(
    func: *const ::std::os::raw::c_void,
    paramIndex: usize,
) -> Result<(usize, usize), crate::sys::cudaError> {
    let mut out_2: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let mut out_3: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudaFuncGetParamInfo(
            func,
            paramIndex,
            out_2.as_mut_ptr() as *mut _,
            out_3.as_mut_ptr() as *mut _,
        )
    };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok((out_2.assume_init() as usize, out_3.assume_init() as usize)) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Returns the number of parameters used by the function\nQueries the number of kernel parameters used by `func` and returns it in `paramCount.`\n\n# Arguments\n\n* `func` -        - The function to query\n* `paramCount` -  - Returns the number of parameters used by the function\n\n# Returns\n\n::CUDA_SUCCESS,\n::CUDA_ERROR_INVALID_VALUE,\n\\notefnerr \\note_cudaKernel_t "]
pub unsafe fn cudaFuncGetParamCount(func: *const ::std::os::raw::c_void) -> Result<usize, crate::sys::cudaError> {
    let mut out_1: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaFuncGetParamCount(func, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_1.assume_init() as usize) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Enqueues a host function call in a stream\nEnqueues a host function to run in a stream.  The function will be called\nafter currently enqueued work and will block work added after it.\nThe host function must not make any CUDA API calls.  Attempting to use a\nCUDA API may result in ::cudaErrorNotPermitted, but this is not required.\nThe host function must not perform any synchronization that may depend on\noutstanding CUDA work not mandated to run earlier.  Host functions without a\nmandated order (such as in independent streams) execute in undefined order\nand may be serialized.\nFor the purposes of Unified Memory, execution makes a number of guarantees:\n<ul>\n<li>The stream is considered idle for the duration of the function's\nexecution.  Thus, for example, the function may always use memory attached\nto the stream it was enqueued in.</li>\n<li>The start of execution of the function has the same effect as\nsynchronizing an event recorded in the same stream immediately prior to\nthe function.  It thus synchronizes streams which have been \"joined\"\nprior to the function.</li>\n<li>Adding device work to any stream does not have the effect of making\nthe stream active until all preceding host functions and stream callbacks\nhave executed.  Thus, for\nexample, a function might use global attached memory even if work has\nbeen added to another stream, if the work has been ordered behind the\nfunction call with an event.</li>\n<li>Completion of the function does not cause a stream to become\nactive except as described above.  The stream will remain idle\nif no device work follows the function, and will remain idle across\nconsecutive host functions or stream callbacks without device work in\nbetween.  Thus, for example,\nstream synchronization can be done by signaling from a host function at the\nend of the stream.</li>\n</ul>\nNote that, in constrast to ::cuStreamAddCallback, the function will not be\ncalled in the event of an error in the CUDA context.\n\n# Arguments\n\n* `hStream` -  - Stream to enqueue function call in\n* `fn` -       - The function to call once preceding stream operations are complete\n* `userData` - - User-specified data to be passed to the function\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidResourceHandle,\n::cudaErrorInvalidValue,\n::cudaErrorNotSupported\n\\note_null_stream \\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaStreamCreate,`]\n::cudaStreamQuery,\n::cudaStreamSynchronize,\n::cudaStreamWaitEvent,\n::cudaStreamDestroy,\n::cudaMallocManaged,\n::cudaStreamAttachMemAsync,\n::cudaStreamAddCallback,\n::cuLaunchHostFunc"]
pub unsafe fn cudaLaunchHostFunc<T: types::CudaAsPtr>(
    stream: cudaStream_t,
    fn_: cudaHostFn_t,
    mut userData: T,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaLaunchHostFunc(stream, fn_, userData.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Enqueues a host function call in a stream\nEnqueues a host function to run in a stream.  The function will be called\nafter currently enqueued work and will block work added after it.\nThe host function must not make any CUDA API calls.  Attempting to use a\nCUDA API may result in ::cudaErrorNotPermitted, but this is not required.\nThe host function must not perform any synchronization that may depend on\noutstanding CUDA work not mandated to run earlier.  Host functions without a\nmandated order (such as in independent streams) execute in undefined order\nand may be serialized.\nFor the purposes of Unified Memory, execution makes a number of guarantees:\n<ul>\n<li>The stream is considered idle for the duration of the function's\nexecution.  Thus, for example, the function may always use memory attached\nto the stream it was enqueued in.</li>\n<li>The start of execution of the function has the same effect as\nsynchronizing an event recorded in the same stream immediately prior to\nthe function.  It thus synchronizes streams which have been \"joined\"\nprior to the function.</li>\n<li>Adding device work to any stream does not have the effect of making\nthe stream active until all preceding host functions and stream callbacks\nhave executed.  Thus, for\nexample, a function might use global attached memory even if work has\nbeen added to another stream, if the work has been ordered behind the\nfunction call with an event.</li>\n<li>Completion of the function does not cause a stream to become\nactive except as described above.  The stream will remain idle\nif no device work follows the function, and will remain idle across\nconsecutive host functions or stream callbacks without device work in\nbetween.  Thus, for example,\nstream synchronization can be done by signaling from a host function at the\nend of the stream.</li>\n</ul>\nNote that, in constrast to ::cuStreamAddCallback, the function will not be\ncalled in the event of an error in the CUDA context.\n\n# Arguments\n\n* `hStream` -  - Stream to enqueue function call in\n* `fn` -       - The function to call once preceding stream operations are complete\n* `userData` - - User-specified data to be passed to the function\n* `syncMode` - - Sync mode for the host function\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidResourceHandle,\n::cudaErrorInvalidValue,\n::cudaErrorNotSupported\n\\note_null_stream \\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaStreamCreate,`]\n::cudaStreamQuery,\n::cudaStreamSynchronize,\n::cudaStreamWaitEvent,\n::cudaStreamDestroy,\n::cudaMallocManaged,\n::cudaStreamAttachMemAsync,\n::cudaStreamAddCallback,\n::cuLaunchHostFunc"]
pub unsafe fn cudaLaunchHostFunc_v2<T: types::CudaAsPtr>(
    stream: cudaStream_t,
    fn_: cudaHostFn_t,
    mut userData: T,
    syncMode: u32,
) -> Result<(), crate::sys::cudaError> {
    let status =
        unsafe { crate::sys::cudaLaunchHostFunc_v2(stream, fn_, userData.as_mut_ptr() as *mut _, syncMode as _) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Sets the shared memory configuration for a device function\n> **Deprecated** On devices with configurable shared memory banks, this function will\nforce all subsequent launches of the specified device function to have\nthe given shared memory bank size configuration. On any given launch of the\nfunction, the shared memory configuration of the device will be temporarily\nchanged if needed to suit the function's preferred configuration. Changes in\nshared memory configuration between subsequent launches of functions,\nmay introduce a device side synchronization point.\nAny per-function setting of shared memory bank size set via\n::cudaFuncSetSharedMemConfig will override the device wide setting set by\n::cudaDeviceSetSharedMemConfig.\nChanging the shared memory bank size will not increase shared memory usage\nor affect occupancy of kernels, but may have major effects on performance.\nLarger bank sizes will allow for greater potential bandwidth to shared memory,\nbut will change what kinds of accesses to shared memory will result in bank\nconflicts.\nThis function will do nothing on devices with fixed shared memory bank size.\nFor templated functions, pass the function symbol as follows:\nfunc_name<template_arg_0,...,template_arg_N>\nThe supported bank configurations are:\n- ::cudaSharedMemBankSizeDefault: use the device's shared memory configuration\nwhen launching this function.\n- ::cudaSharedMemBankSizeFourByte: set shared memory bank width to be\nfour bytes natively when launching this function.\n- ::cudaSharedMemBankSizeEightByte: set shared memory bank width to be eight\nbytes natively when launching this function.\n\n# Arguments\n\n* `func` -   - Device function symbol\n* `config` - - Requested shared memory configuration\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidDeviceFunction,\n::cudaErrorInvalidValue,\n\\notefnerr \\note_string_api_deprecation2 \\note_init_rt \\note_callback # See also\n\n> [`::cudaDeviceSetSharedMemConfig,`]\n::cudaDeviceGetSharedMemConfig,\n::cudaDeviceSetCacheConfig,\n::cudaDeviceGetCacheConfig,\n::cudaFuncSetCacheConfig,\n::cuFuncSetSharedMemConfig"]
pub unsafe fn cudaFuncSetSharedMemConfig<T: types::CudaAsPtr>(
    func: T,
    config: cudaSharedMemConfig,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaFuncSetSharedMemConfig(func.as_const_ptr() as *const _, config) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Returns occupancy for a device function\nReturns in `*numBlocks` the maximum number of active blocks per\nstreaming multiprocessor for the device function.\n\n# Arguments\n\n* `numBlocks` -       - Returned occupancy\n* `func` -            - Kernel function for which occupancy is calculated\n* `blockSize` -       - Block size the kernel is intended to be launched with\n* `dynamicSMemSize` - - Per-block dynamic shared memory usage intended, in bytes\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidDevice,\n::cudaErrorInvalidDeviceFunction,\n::cudaErrorInvalidValue,\n::cudaErrorUnknown,\n\\notefnerr \\note_init_rt \\note_callback \\note_cudaKernel_t # See also\n\n> [`::cudaOccupancyMaxActiveBlocksPerMultiprocessorWithFlags,`]\n[`::cudaOccupancyMaxPotentialBlockSize(int*,`] int*, T, size_t, int) \"cudaOccupancyMaxPotentialBlockSize (C++ API)\",\n[`::cudaOccupancyMaxPotentialBlockSizeWithFlags(int*,`] int*, T, size_t, int, unsigned int) \"cudaOccupancyMaxPotentialBlockSizeWithFlags (C++ API)\",\n[`::cudaOccupancyMaxPotentialBlockSizeVariableSMem(int*,`] int*, T, UnaryFunction, int) \"cudaOccupancyMaxPotentialBlockSizeVariableSMem (C++ API)\",\n[`::cudaOccupancyMaxPotentialBlockSizeVariableSMemWithFlags(int*,`] int*, T, UnaryFunction, int, unsigned int) \"cudaOccupancyMaxPotentialBlockSizeVariableSMemWithFlags (C++ API)\",\n[`::cudaOccupancyAvailableDynamicSMemPerBlock(size_t*,`] T, int, int) \"cudaOccupancyAvailableDynamicSMemPerBlock (C++ API)\",\n::cuOccupancyMaxActiveBlocksPerMultiprocessor"]
pub unsafe fn cudaOccupancyMaxActiveBlocksPerMultiprocessor<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    mut numBlocks: T,
    func: U,
    blockSize: i32,
    dynamicSMemSize: usize,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaOccupancyMaxActiveBlocksPerMultiprocessor(
            numBlocks.as_mut_ptr() as *mut _,
            func.as_const_ptr() as *const _,
            blockSize as _,
            dynamicSMemSize,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Returns dynamic shared memory available per block when launching `numBlocks` blocks on SM.\nReturns in `*dynamicSmemSize` the maximum size of dynamic shared memory to allow `numBlocks` blocks per SM.\n\n# Arguments\n\n* `dynamicSmemSize` - - Returned maximum dynamic shared memory\n* `func` -            - Kernel function for which occupancy is calculated\n* `numBlocks` -       - Number of blocks to fit on SM\n* `blockSize` -       - Size of the block\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidDevice,\n::cudaErrorInvalidDeviceFunction,\n::cudaErrorInvalidValue,\n::cudaErrorUnknown,\n\\notefnerr \\note_init_rt \\note_callback \\note_cudaKernel_t # See also\n\n> [`::cudaOccupancyMaxActiveBlocksPerMultiprocessorWithFlags,`]\n[`::cudaOccupancyMaxPotentialBlockSize(int*,`] int*, T, size_t, int) \"cudaOccupancyMaxPotentialBlockSize (C++ API)\",\n[`::cudaOccupancyMaxPotentialBlockSizeWithFlags(int*,`] int*, T, size_t, int, unsigned int) \"cudaOccupancyMaxPotentialBlockSizeWithFlags (C++ API)\",\n[`::cudaOccupancyMaxPotentialBlockSizeVariableSMem(int*,`] int*, T, UnaryFunction, int) \"cudaOccupancyMaxPotentialBlockSizeVariableSMem (C++ API)\",\n[`::cudaOccupancyMaxPotentialBlockSizeVariableSMemWithFlags(int*,`] int*, T, UnaryFunction, int, unsigned int) \"cudaOccupancyMaxPotentialBlockSizeVariableSMemWithFlags (C++ API)\",\n::cudaOccupancyAvailableDynamicSMemPerBlock"]
pub unsafe fn cudaOccupancyAvailableDynamicSMemPerBlock<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    mut dynamicSmemSize: T,
    func: U,
    numBlocks: i32,
    blockSize: i32,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaOccupancyAvailableDynamicSMemPerBlock(
            dynamicSmemSize.as_mut_ptr() as *mut _,
            func.as_const_ptr() as *const _,
            numBlocks as _,
            blockSize as _,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Returns occupancy for a device function with the specified flags\nReturns in `*numBlocks` the maximum number of active blocks per\nstreaming multiprocessor for the device function.\nThe `flags` parameter controls how special cases are handled. Valid flags include:\n- ::cudaOccupancyDefault: keeps the default behavior as\n::cudaOccupancyMaxActiveBlocksPerMultiprocessor\n- ::cudaOccupancyDisableCachingOverride: This flag suppresses the default behavior\non platform where global caching affects occupancy. On such platforms, if caching\nis enabled, but per-block SM resource usage would result in zero occupancy, the\noccupancy calculator will calculate the occupancy as if caching is disabled.\nSetting this flag makes the occupancy calculator to return 0 in such cases.\nMore information can be found about this feature in the \"Unified L1/Texture Cache\"\nsection of the Maxwell tuning guide.\n\n# Arguments\n\n* `numBlocks` -       - Returned occupancy\n* `func` -            - Kernel function for which occupancy is calculated\n* `blockSize` -       - Block size the kernel is intended to be launched with\n* `dynamicSMemSize` - - Per-block dynamic shared memory usage intended, in bytes\n* `flags` -           - Requested behavior for the occupancy calculator\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidDevice,\n::cudaErrorInvalidDeviceFunction,\n::cudaErrorInvalidValue,\n::cudaErrorUnknown,\n\\notefnerr \\note_init_rt \\note_callback \\note_cudaKernel_t # See also\n\n> [`::cudaOccupancyMaxActiveBlocksPerMultiprocessor,`]\n[`::cudaOccupancyMaxPotentialBlockSize(int*,`] int*, T, size_t, int) \"cudaOccupancyMaxPotentialBlockSize (C++ API)\",\n[`::cudaOccupancyMaxPotentialBlockSizeWithFlags(int*,`] int*, T, size_t, int, unsigned int) \"cudaOccupancyMaxPotentialBlockSizeWithFlags (C++ API)\",\n[`::cudaOccupancyMaxPotentialBlockSizeVariableSMem(int*,`] int*, T, UnaryFunction, int) \"cudaOccupancyMaxPotentialBlockSizeVariableSMem (C++ API)\",\n[`::cudaOccupancyMaxPotentialBlockSizeVariableSMemWithFlags(int*,`] int*, T, UnaryFunction, int, unsigned int) \"cudaOccupancyMaxPotentialBlockSizeVariableSMemWithFlags (C++ API)\",\n[`::cudaOccupancyAvailableDynamicSMemPerBlock(size_t*,`] T, int, int) \"cudaOccupancyAvailableDynamicSMemPerBlock (C++ API)\",\n::cuOccupancyMaxActiveBlocksPerMultiprocessorWithFlags"]
pub unsafe fn cudaOccupancyMaxActiveBlocksPerMultiprocessorWithFlags<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    mut numBlocks: T,
    func: U,
    blockSize: i32,
    dynamicSMemSize: usize,
    flags: u32,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaOccupancyMaxActiveBlocksPerMultiprocessorWithFlags(
            numBlocks.as_mut_ptr() as *mut _,
            func.as_const_ptr() as *const _,
            blockSize as _,
            dynamicSMemSize,
            flags as _,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Given the kernel function (`func)` and launch configuration\n(`config),` return the maximum cluster size in `*clusterSize.`\nThe cluster dimensions in `config` are ignored. If func has a required\ncluster size set (see ::cudaFuncGetAttributes),`*clusterSize` will reflect\nthe required cluster size.\nBy default this function will always return a value that's portable on\nfuture hardware. A higher value may be returned if the kernel function\nallows non-portable cluster sizes.\nThis function will respect the compile time launch bounds.\n\n# Arguments\n\n* `clusterSize` - - Returned maximum cluster size that can be launched\nfor the given kernel function and launch configuration\n* `func` -        - Kernel function for which maximum cluster\nsize is calculated\n* `config` -      - Launch configuration for the given kernel function\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidDeviceFunction,\n::cudaErrorInvalidValue,\n::cudaErrorUnknown,\n\\notefnerr \\note_init_rt \\note_callback \\note_cudaKernel_t # See also\n\n> [`::cudaFuncGetAttributes`]\n[`::cudaOccupancyMaxPotentialClusterSize(int*,`] T, const cudaLaunchConfig_t*) \"cudaOccupancyMaxPotentialClusterSize (C++ API)\",\n::cuOccupancyMaxPotentialClusterSize"]
pub unsafe fn cudaOccupancyMaxPotentialClusterSize<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    mut clusterSize: T,
    func: U,
    launchConfig: V,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaOccupancyMaxPotentialClusterSize(
            clusterSize.as_mut_ptr() as *mut _,
            func.as_const_ptr() as *const _,
            launchConfig.as_const_ptr() as *const _,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Given the kernel function (`func)` and launch configuration\n(`config),` return the maximum number of clusters that could co-exist\non the target device in `*numClusters.`\nIf the function has required cluster size already set (see\n::cudaFuncGetAttributes), the cluster size from config must either be\nunspecified or match the required size.\nWithout required sizes, the cluster size must be specified in config,\nelse the function will return an error.\nNote that various attributes of the kernel function may affect occupancy\ncalculation. Runtime environment may affect how the hardware schedules\nthe clusters, so the calculated occupancy is not guaranteed to be achievable.\n\n# Arguments\n\n* `numClusters` - - Returned maximum number of clusters that\ncould co-exist on the target device\n* `func` -        - Kernel function for which maximum number\nof clusters are calculated\n* `config` -      - Launch configuration for the given kernel function\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidDeviceFunction,\n::cudaErrorInvalidValue,\n::cudaErrorInvalidClusterSize,\n::cudaErrorUnknown,\n\\notefnerr \\note_init_rt \\note_callback \\note_cudaKernel_t # See also\n\n> [`::cudaFuncGetAttributes`]\n[`::cudaOccupancyMaxActiveClusters(int*,`] T, const cudaLaunchConfig_t*) \"cudaOccupancyMaxActiveClusters (C++ API)\",\n::cuOccupancyMaxActiveClusters"]
pub unsafe fn cudaOccupancyMaxActiveClusters<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    mut numClusters: T,
    func: U,
    launchConfig: V,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaOccupancyMaxActiveClusters(
            numClusters.as_mut_ptr() as *mut _,
            func.as_const_ptr() as *const _,
            launchConfig.as_const_ptr() as *const _,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudaMallocManaged<T>(
    size: usize,
    flags: u32,
) -> Result<::cuda_libs_cudart::types::cuDeviceAllocation<T>, crate::sys::cudaError> {
    let mut dev_ptr = std::ptr::null_mut();
    let status = unsafe { crate::sys::cudaMallocManaged(&mut dev_ptr as *mut _ as *mut _, size, flags as _) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(::cuda_libs_cudart::types::cuDeviceAllocation(dev_ptr as *mut T))
    } else {
        Err(status)
    }
}
#[doc = "Allocate memory on the device\nAllocates `size` bytes of linear memory on the device and returns in\n`*devPtr` a pointer to the allocated memory. The allocated memory is\nsuitably aligned for any kind of variable. The memory is not cleared.\n::cudaMalloc() returns ::cudaErrorMemoryAllocation in case of failure.\nThe device version of ::cudaFree cannot be used with a `*devPtr`\nallocated using the host API, and vice versa.\n\n# Arguments\n\n* `devPtr` - - Pointer to allocated device memory\n* `size` -   - Requested allocation size in bytes\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorMemoryAllocation,\n::cudaErrorExternalDevice\n\\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaMallocPitch,`] ::cudaFree, ::cudaMallocArray, ::cudaFreeArray,\n::cudaMalloc3D, ::cudaMalloc3DArray,\n[`::cudaMallocHost(void**,`] size_t) \"cudaMallocHost (C API)\",\n::cudaFreeHost, ::cudaHostAlloc,\n::cuMemAlloc"]
pub unsafe fn cudaMalloc<T>(
    size: usize,
) -> Result<::cuda_libs_cudart::types::cuDeviceAllocation<T>, crate::sys::cudaError> {
    let mut dev_ptr = std::ptr::null_mut();
    let status = unsafe { crate::sys::cudaMalloc(&mut dev_ptr as *mut _ as *mut _, size) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(::cuda_libs_cudart::types::cuDeviceAllocation(dev_ptr as *mut T))
    } else {
        Err(status)
    }
}
#[doc = "Allocates page-locked memory on the host\nAllocates `size` bytes of host memory that is page-locked and accessible\nto the device. The driver tracks the virtual memory ranges allocated with\nthis function and automatically accelerates calls to functions such as\n::cudaMemcpy*(). Since the memory can be accessed directly by the device,\nit can be read or written with much higher bandwidth than pageable memory\nobtained with functions such as ::malloc().\nOn systems where ::pageableMemoryAccessUsesHostPageTables\nis true, ::cudaMallocHost may not page-lock the allocated memory.\nPage-locking excessive amounts of memory with ::cudaMallocHost() may degrade\nsystem performance, since it reduces the amount of memory available to the\nsystem for paging. As a result, this function is best used sparingly to allocate\nstaging areas for data exchange between host and device.\n\n# Arguments\n\n* `ptr` -  - Pointer to allocated host memory\n* `size` - - Requested allocation size in bytes\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorMemoryAllocation,\n::cudaErrorExternalDevice\n\\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaMalloc,`] ::cudaMallocPitch, ::cudaMallocArray, ::cudaMalloc3D,\n::cudaMalloc3DArray, ::cudaHostAlloc, ::cudaFree, ::cudaFreeArray,\n[`::cudaMallocHost(void**,`] size_t, unsigned int) \"cudaMallocHost (C++ API)\",\n::cudaFreeHost, ::cudaHostAlloc,\n::cuMemAllocHost"]
pub unsafe fn cudaMallocHost<T>(size: usize) -> Result<*mut T, crate::sys::cudaError> {
    let mut dev_ptr = std::ptr::null_mut();
    let status = unsafe { crate::sys::cudaMallocHost(&mut dev_ptr as *mut _ as *mut _, size) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(dev_ptr as *mut T)
    } else {
        Err(status)
    }
}
#[doc = "Allocates pitched memory on the device\nAllocates at least `width` (in bytes) * `height` bytes of linear memory\non the device and returns in `*devPtr` a pointer to the allocated memory.\nThe function may pad the allocation to ensure that corresponding pointers\nin any given row will continue to meet the alignment requirements for\ncoalescing as the address is updated from row to row. The pitch returned in\n`*pitch` by ::cudaMallocPitch() is the width in bytes of the allocation.\nThe intended usage of `pitch` is as a separate parameter of the allocation,\nused to compute addresses within the 2D array. Given the row and column of\nan array element of type `T,` the address is computed as:\n\\code T* pElement = (T*)((char*)BaseAddress + Row * pitch) + Column;\n\\endcode For allocations of 2D arrays, it is recommended that programmers consider\nperforming pitch allocations using ::cudaMallocPitch(). Due to pitch\nalignment restrictions in the hardware, this is especially true if the\napplication will be performing 2D memory copies between different regions\nof device memory (whether linear memory or CUDA arrays).\n\n# Arguments\n\n* `devPtr` - - Pointer to allocated pitched device memory\n* `pitch` -  - Pitch for allocation\n* `width` -  - Requested pitched allocation width (in bytes)\n* `height` - - Requested pitched allocation height\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorMemoryAllocation\n\\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaMalloc,`] ::cudaFree, ::cudaMallocArray, ::cudaFreeArray,\n[`::cudaMallocHost(void**,`] size_t) \"cudaMallocHost (C API)\",\n::cudaFreeHost, ::cudaMalloc3D, ::cudaMalloc3DArray,\n::cudaHostAlloc,\n::cuMemAllocPitch"]
pub unsafe fn cudaMallocPitch<T>(
    pitch: *mut usize,
    width: usize,
    height: usize,
) -> Result<::cuda_libs_cudart::types::cuDeviceAllocation<T>, crate::sys::cudaError> {
    let mut dev_ptr = std::ptr::null_mut();
    let status = unsafe { crate::sys::cudaMallocPitch(&mut dev_ptr as *mut _ as *mut _, pitch, width, height) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(::cuda_libs_cudart::types::cuDeviceAllocation(dev_ptr as *mut T))
    } else {
        Err(status)
    }
}
#[doc = "Allocate an array on the device\nAllocates a CUDA array according to the ::cudaChannelFormatDesc structure\n`desc` and returns a handle to the new CUDA array in `*array.`\nThe ::cudaChannelFormatDesc is defined as:\n\\code struct cudaChannelFormatDesc {\nint x, y, z, w;\nenum cudaChannelFormatKind f;\n};\n\\endcode where ::cudaChannelFormatKind is one of ::cudaChannelFormatKindSigned,\n::cudaChannelFormatKindUnsigned, or ::cudaChannelFormatKindFloat.\nThe `flags` parameter enables different options to be specified that affect\nthe allocation, as follows.\n- ::cudaArrayDefault: This flag's value is defined to be 0 and provides default array allocation\n- ::cudaArraySurfaceLoadStore: Allocates an array that can be read from or written to using a surface reference\n- ::cudaArrayTextureGather: This flag indicates that texture gather operations will be performed on the array.\n- ::cudaArraySparse: Allocates a CUDA array without physical backing memory. The subregions within this sparse array\ncan later be mapped onto a physical memory allocation by calling ::cuMemMapArrayAsync.\nThe physical backing memory must be allocated via ::cuMemCreate.\n- ::cudaArrayDeferredMapping: Allocates a CUDA array without physical backing memory. The entire array can\nlater be mapped onto a physical memory allocation by calling ::cuMemMapArrayAsync.\nThe physical backing memory must be allocated via ::cuMemCreate.\n`width` and `height` must meet certain size requirements. See ::cudaMalloc3DArray() for more details.\n\n# Arguments\n\n* `array` -  - Pointer to allocated array in device memory\n* `desc` -   - Requested channel format\n* `width` -  - Requested array allocation width\n* `height` - - Requested array allocation height\n* `flags` -  - Requested properties of allocated array\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorMemoryAllocation\n\\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaMalloc,`] ::cudaMallocPitch, ::cudaFree, ::cudaFreeArray,\n[`::cudaMallocHost(void**,`] size_t) \"cudaMallocHost (C API)\",\n::cudaFreeHost, ::cudaMalloc3D, ::cudaMalloc3DArray,\n::cudaHostAlloc,\n::cuArrayCreate"]
pub unsafe fn cudaMallocArray(
    desc: *const cudaChannelFormatDesc,
    width: usize,
    height: usize,
    flags: u32,
) -> Result<cudaArray_t, crate::sys::cudaError> {
    let mut dev_ptr: cudaArray_t = unsafe { std::mem::zeroed() };
    let status =
        unsafe { crate::sys::cudaMallocArray(&mut dev_ptr as *mut _ as *mut _, desc, width, height, flags as _) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(dev_ptr)
    } else {
        Err(status)
    }
}
#[doc = "Frees memory on the device\nFrees the memory space pointed to by `devPtr,` which must have been\nreturned by a previous call to one of the following memory allocation APIs -\n::cudaMalloc(), ::cudaMallocPitch(), ::cudaMallocManaged(), ::cudaMallocAsync(),\n::cudaMallocFromPoolAsync().\nNote - This API will not perform any implicit synchronization when the pointer was\nallocated with ::cudaMallocAsync or ::cudaMallocFromPoolAsync. Callers must ensure\nthat all accesses to these pointer have completed before invoking ::cudaFree. For\nbest performance and memory reuse, users should use ::cudaFreeAsync to free memory\nallocated via the stream ordered memory allocator.\nFor all other pointers, this API may perform implicit synchronization.\nIf ::cudaFree(`devPtr)` has already been called before,\nan error is returned. If `devPtr` is 0, no operation is performed.\n::cudaFree() returns ::cudaErrorValue in case of failure.\nThe device version of ::cudaFree cannot be used with a `*devPtr`\nallocated using the host API, and vice versa.\n\n# Arguments\n\n* `devPtr` - - Device pointer to memory to free\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue\n\\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaMalloc,`] ::cudaMallocPitch, ::cudaMallocManaged, ::cudaMallocArray, ::cudaFreeArray, ::cudaMallocAsync, ::cudaMallocFromPoolAsync\n[`::cudaMallocHost(void**,`] size_t) \"cudaMallocHost (C API)\",\n::cudaFreeHost, ::cudaMalloc3D, ::cudaMalloc3DArray, ::cudaFreeAsync\n::cudaHostAlloc,\n::cuMemFree"]
pub unsafe fn cudaFree<T>(
    devPtr: ::cuda_libs_cudart::types::cuDeviceAllocation<T>,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaFree(devPtr.0 as *mut _) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Frees page-locked memory\nFrees the memory space pointed to by `hostPtr,` which must have been\nreturned by a previous call to ::cudaMallocHost() or ::cudaHostAlloc().\n\n# Arguments\n\n* `ptr` - - Pointer to memory to free\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue\n\\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaMalloc,`] ::cudaMallocPitch, ::cudaFree, ::cudaMallocArray,\n::cudaFreeArray,\n[`::cudaMallocHost(void**,`] size_t) \"cudaMallocHost (C API)\",\n::cudaMalloc3D, ::cudaMalloc3DArray, ::cudaHostAlloc,\n::cuMemFreeHost"]
pub unsafe fn cudaFreeHost<T>(ptr: *mut T) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaFreeHost(ptr as *mut _) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Frees an array on the device\nFrees the CUDA array `array,` which must have been returned by a\nprevious call to ::cudaMallocArray(). If `devPtr` is 0,\nno operation is performed.\n\n# Arguments\n\n* `array` - - Pointer to array to free\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue\n\\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaMalloc,`] ::cudaMallocPitch, ::cudaFree, ::cudaMallocArray,\n[`::cudaMallocHost(void**,`] size_t) \"cudaMallocHost (C API)\",\n::cudaFreeHost, ::cudaHostAlloc,\n::cuArrayDestroy"]
pub unsafe fn cudaFreeArray(array: cudaArray_t) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaFreeArray(array) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Frees a mipmapped array on the device\nFrees the CUDA mipmapped array `mipmappedArray,` which must have been\nreturned by a previous call to ::cudaMallocMipmappedArray(). If `devPtr`\nis 0, no operation is performed.\n\n# Arguments\n\n* `mipmappedArray` - - Pointer to mipmapped array to free\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue\n\\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaMalloc,`] ::cudaMallocPitch, ::cudaFree, ::cudaMallocArray,\n[`::cudaMallocHost(void**,`] size_t) \"cudaMallocHost (C API)\",\n::cudaFreeHost, ::cudaHostAlloc,\n::cuMipmappedArrayDestroy"]
pub unsafe fn cudaFreeMipmappedArray(mipmappedArray: cudaMipmappedArray_t) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaFreeMipmappedArray(mipmappedArray) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Allocates page-locked memory on the host\nAllocates `size` bytes of host memory that is page-locked and accessible\nto the device. The driver tracks the virtual memory ranges allocated with\nthis function and automatically accelerates calls to functions such as\n::cudaMemcpy(). Since the memory can be accessed directly by the device, it\ncan be read or written with much higher bandwidth than pageable memory\nobtained with functions such as ::malloc(). Allocating excessive amounts of\npinned memory may degrade system performance, since it reduces the amount\nof memory available to the system for paging. As a result, this function is\nbest used sparingly to allocate staging areas for data exchange between host\nand device.\nThe `flags` parameter enables different options to be specified that affect\nthe allocation, as follows.\n- ::cudaHostAllocDefault: This flag's value is defined to be 0 and causes\n::cudaHostAlloc() to emulate ::cudaMallocHost().\n- ::cudaHostAllocPortable: The memory returned by this call will be\nconsidered as pinned memory by all CUDA contexts, not just the one that\nperformed the allocation.\n- ::cudaHostAllocMapped: Maps the allocation into the CUDA address space.\nThe device pointer to the memory may be obtained by calling\n::cudaHostGetDevicePointer().\n- ::cudaHostAllocWriteCombined: Allocates the memory as write-combined (WC).\nWC memory can be transferred across the PCI Express bus more quickly on some\nsystem configurations, but cannot be read efficiently by most CPUs.  WC\nmemory is a good option for buffers that will be written by the CPU and read\nby the device via mapped pinned memory or host->device transfers.\nAll of these flags are orthogonal to one another: a developer may allocate\nmemory that is portable, mapped and/or write-combined with no restrictions.\nIn order for the ::cudaHostAllocMapped flag to have any effect, the CUDA context\nmust support the ::cudaDeviceMapHost flag, which can be checked via\n::cudaGetDeviceFlags(). The ::cudaDeviceMapHost flag is implicitly set for\ncontexts created via the runtime API.\nThe ::cudaHostAllocMapped flag may be specified on CUDA contexts for devices\nthat do not support mapped pinned memory. The failure is deferred to\n::cudaHostGetDevicePointer() because the memory may be mapped into other\nCUDA contexts via the ::cudaHostAllocPortable flag.\nMemory allocated by this function must be freed with ::cudaFreeHost().\n\n# Arguments\n\n* `pHost` - - Device pointer to allocated memory\n* `size` -  - Requested allocation size in bytes\n* `flags` - - Requested properties of allocated memory\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorMemoryAllocation,\n::cudaErrorExternalDevice\n\\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaSetDeviceFlags,`]\n[`::cudaMallocHost(void**,`] size_t) \"cudaMallocHost (C API)\",\n::cudaFreeHost,\n::cudaGetDeviceFlags,\n::cuMemHostAlloc"]
pub unsafe fn cudaHostAlloc<T: types::CudaAsPtr>(
    mut pHost: T,
    size: usize,
    flags: u32,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaHostAlloc(pHost.as_mut_ptr() as *mut _, size, flags as _) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Registers an existing host memory range for use by CUDA\nPage-locks the memory range specified by `ptr` and `size` and maps it\nfor the device(s) as specified by `flags.` This memory range also is added\nto the same tracking mechanism as ::cudaHostAlloc() to automatically accelerate\ncalls to functions such as ::cudaMemcpy(). Since the memory can be accessed\ndirectly by the device, it can be read or written with much higher bandwidth\nthan pageable memory that has not been registered.  Page-locking excessive\namounts of memory may degrade system performance, since it reduces the amount\nof memory available to the system for paging. As a result, this function is\nbest used sparingly to register staging areas for data exchange between\nhost and device.\nOn systems where ::pageableMemoryAccessUsesHostPageTables is true, ::cudaHostRegister\nwill not page-lock the memory range specified by `ptr` but only populate\nunpopulated pages.\n::cudaHostRegister is supported only on I/O coherent devices that have a non-zero\nvalue for the device attribute ::cudaDevAttrHostRegisterSupported.\nThe `flags` parameter enables different options to be specified that\naffect the allocation, as follows.\n- ::cudaHostRegisterDefault: On a system with unified virtual addressing,\nthe memory will be both mapped and portable.  On a system with no unified\nvirtual addressing, the memory will be neither mapped nor portable.\n- ::cudaHostRegisterPortable: The memory returned by this call will be\nconsidered as pinned memory by all CUDA contexts, not just the one that\nperformed the allocation.\n- ::cudaHostRegisterMapped: Maps the allocation into the CUDA address\nspace. The device pointer to the memory may be obtained by calling\n::cudaHostGetDevicePointer().\n- ::cudaHostRegisterIoMemory: The passed memory pointer is treated as\npointing to some memory-mapped I/O space, e.g. belonging to a\nthird-party PCIe device, and it will marked as non cache-coherent and\ncontiguous.\n- ::cudaHostRegisterReadOnly: The passed memory pointer is treated as\npointing to memory that is considered read-only by the device.  On\nplatforms without ::cudaDevAttrPageableMemoryAccessUsesHostPageTables, this\nflag is required in order to register memory mapped to the CPU as\nread-only.  Support for the use of this flag can be queried from the device\nattribute ::cudaDevAttrHostRegisterReadOnlySupported.  Using this flag with\na current context associated with a device that does not have this attribute\nset will cause ::cudaHostRegister to error with cudaErrorNotSupported.\nAll of these flags are orthogonal to one another: a developer may page-lock\nmemory that is portable or mapped with no restrictions.\nThe CUDA context must have been created with the ::cudaMapHost flag in\norder for the ::cudaHostRegisterMapped flag to have any effect.\nThe ::cudaHostRegisterMapped flag may be specified on CUDA contexts for\ndevices that do not support mapped pinned memory. The failure is deferred\nto ::cudaHostGetDevicePointer() because the memory may be mapped into\nother CUDA contexts via the ::cudaHostRegisterPortable flag.\nFor devices that have a non-zero value for the device attribute\n::cudaDevAttrCanUseHostPointerForRegisteredMem, the memory\ncan also be accessed from the device using the host pointer `ptr.`\nThe device pointer returned by ::cudaHostGetDevicePointer() may or may not\nmatch the original host pointer `ptr` and depends on the devices visible to the\napplication. If all devices visible to the application have a non-zero value for the\ndevice attribute, the device pointer returned by ::cudaHostGetDevicePointer()\nwill match the original pointer `ptr.` If any device visible to the application\nhas a zero value for the device attribute, the device pointer returned by\n::cudaHostGetDevicePointer() will not match the original host pointer `ptr,`\nbut it will be suitable for use on all devices provided Unified Virtual Addressing\nis enabled. In such systems, it is valid to access the memory using either pointer\non devices that have a non-zero value for the device attribute. Note however that\nsuch devices should access the memory using only of the two pointers and not both.\nThe memory page-locked by this function must be unregistered with ::cudaHostUnregister().\n\n# Arguments\n\n* `ptr` -   - Host pointer to memory to page-lock\n* `size` -  - Size in bytes of the address range to page-lock in bytes\n* `flags` - - Flags for allocation request\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorMemoryAllocation,\n::cudaErrorHostMemoryAlreadyRegistered,\n::cudaErrorNotSupported,\n::cudaErrorExternalDevice\n\\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaHostUnregister,`] ::cudaHostGetFlags, ::cudaHostGetDevicePointer,\n::cuMemHostRegister"]
pub unsafe fn cudaHostRegister<T: types::CudaAsPtr>(
    mut ptr: T,
    size: usize,
    flags: u32,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaHostRegister(ptr.as_mut_ptr() as *mut _, size, flags as _) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Unregisters a memory range that was registered with cudaHostRegister\nUnmaps the memory range whose base address is specified by `ptr,` and makes\nit pageable again.\nThe base address must be the same one specified to ::cudaHostRegister().\n\n# Arguments\n\n* `ptr` - - Host pointer to memory to unregister\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorHostMemoryNotRegistered\n\\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaHostUnregister,`]\n::cuMemHostUnregister"]
pub unsafe fn cudaHostUnregister<T: types::CudaAsPtr>(mut ptr: T) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaHostUnregister(ptr.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Passes back device pointer of mapped host memory allocated by\ncudaHostAlloc or registered by cudaHostRegister\nPasses back the device pointer corresponding to the mapped, pinned host\nbuffer allocated by ::cudaHostAlloc() or registered by ::cudaHostRegister().\n::cudaHostGetDevicePointer() will fail if the ::cudaDeviceMapHost flag was\nnot specified before deferred context creation occurred, or if called on a\ndevice that does not support mapped, pinned memory.\nFor devices that have a non-zero value for the device attribute\n::cudaDevAttrCanUseHostPointerForRegisteredMem, the memory\ncan also be accessed from the device using the host pointer `pHost.`\nThe device pointer returned by ::cudaHostGetDevicePointer() may or may not\nmatch the original host pointer `pHost` and depends on the devices visible to the\napplication. If all devices visible to the application have a non-zero value for the\ndevice attribute, the device pointer returned by ::cudaHostGetDevicePointer()\nwill match the original pointer `pHost.` If any device visible to the application\nhas a zero value for the device attribute, the device pointer returned by\n::cudaHostGetDevicePointer() will not match the original host pointer `pHost,`\nbut it will be suitable for use on all devices provided Unified Virtual Addressing\nis enabled. In such systems, it is valid to access the memory using either pointer\non devices that have a non-zero value for the device attribute. Note however that\nsuch devices should access the memory using only of the two pointers and not both.\n`flags` provides for future releases.  For now, it must be set to 0.\n\n# Arguments\n\n* `pDevice` - - Returned device pointer for mapped memory\n* `pHost` -   - Requested host pointer mapping\n* `flags` -   - Flags for extensions (must be 0 for now)\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorMemoryAllocation\n\\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaSetDeviceFlags,`] ::cudaHostAlloc,\n::cuMemHostGetDevicePointer"]
pub unsafe fn cudaHostGetDevicePointer<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    mut pDevice: T,
    mut pHost: U,
    flags: u32,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaHostGetDevicePointer(pDevice.as_mut_ptr() as *mut _, pHost.as_mut_ptr() as *mut _, flags as _)
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Passes back flags used to allocate pinned host memory allocated by\ncudaHostAlloc\n::cudaHostGetFlags() will fail if the input pointer does not\nreside in an address range allocated by ::cudaHostAlloc().\n\n# Arguments\n\n* `pFlags` - - Returned flags word\n* `pHost` - - Host pointer\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue\n\\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaHostAlloc,`]\n::cuMemHostGetFlags"]
pub unsafe fn cudaHostGetFlags(pHost: *mut ::std::os::raw::c_void) -> Result<u32, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<::std::os::raw::c_uint> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaHostGetFlags(out_0.as_mut_ptr() as *mut _, pHost) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_0.assume_init() as u32) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Allocates logical 1D, 2D, or 3D memory objects on the device\nAllocates at least `width` * `height` * `depth` bytes of linear memory\non the device and returns a ::cudaPitchedPtr in which `ptr` is a pointer\nto the allocated memory. The function may pad the allocation to ensure\nhardware alignment requirements are met. The pitch returned in the `pitch`\nfield of `pitchedDevPtr` is the width in bytes of the allocation.\nThe returned ::cudaPitchedPtr contains additional fields `xsize` and\n`ysize,` the logical width and height of the allocation, which are\nequivalent to the `width` and `height` `extent` parameters provided by\nthe programmer during allocation.\nFor allocations of 2D and 3D objects, it is highly recommended that\nprogrammers perform allocations using ::cudaMalloc3D() or\n::cudaMallocPitch(). Due to alignment restrictions in the hardware, this is\nespecially true if the application will be performing memory copies\ninvolving 2D or 3D objects (whether linear memory or CUDA arrays).\n\n# Arguments\n\n* `pitchedDevPtr` -  - Pointer to allocated pitched device memory\n* `extent` -         - Requested allocation size (`width` field in bytes)\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorMemoryAllocation\n\\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaMallocPitch,`] ::cudaFree, ::cudaMemcpy3D, ::cudaMemset3D,\n::cudaMalloc3DArray, ::cudaMallocArray, ::cudaFreeArray,\n[`::cudaMallocHost(void**,`] size_t) \"cudaMallocHost (C API)\",\n::cudaFreeHost, ::cudaHostAlloc, ::make_cudaPitchedPtr, ::make_cudaExtent,\n::cuMemAllocPitch"]
pub unsafe fn cudaMalloc3D(extent: cudaExtent) -> Result<cudaPitchedPtr, crate::sys::cudaError> {
    let mut dev_ptr: cudaPitchedPtr = unsafe { std::mem::zeroed() };
    let status = unsafe { crate::sys::cudaMalloc3D(&mut dev_ptr as *mut _ as *mut _, extent) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(dev_ptr)
    } else {
        Err(status)
    }
}
#[doc = "Allocate an array on the device\nAllocates a CUDA array according to the ::cudaChannelFormatDesc structure\n`desc` and returns a handle to the new CUDA array in `*array.`\nThe ::cudaChannelFormatDesc is defined as:\n\\code struct cudaChannelFormatDesc {\nint x, y, z, w;\nenum cudaChannelFormatKind f;\n};\n\\endcode where ::cudaChannelFormatKind is one of ::cudaChannelFormatKindSigned,\n::cudaChannelFormatKindUnsigned, or ::cudaChannelFormatKindFloat.\n::cudaMalloc3DArray() can allocate the following:\n- A 1D array is allocated if the height and depth extents are both zero.\n- A 2D array is allocated if only the depth extent is zero.\n- A 3D array is allocated if all three extents are non-zero.\n- A 1D layered CUDA array is allocated if only the height extent is zero and\nthe cudaArrayLayered flag is set. Each layer is a 1D array. The number of layers is\ndetermined by the depth extent.\n- A 2D layered CUDA array is allocated if all three extents are non-zero and\nthe cudaArrayLayered flag is set. Each layer is a 2D array. The number of layers is\ndetermined by the depth extent.\n- A cubemap CUDA array is allocated if all three extents are non-zero and the\ncudaArrayCubemap flag is set. Width must be equal to height, and depth must be six. A cubemap is\na special type of 2D layered CUDA array, where the six layers represent the six faces of a cube.\nThe order of the six layers in memory is the same as that listed in ::cudaGraphicsCubeFace.\n- A cubemap layered CUDA array is allocated if all three extents are non-zero, and both,\ncudaArrayCubemap and cudaArrayLayered flags are set. Width must be equal to height, and depth must be\na multiple of six. A cubemap layered CUDA array is a special type of 2D layered CUDA array that consists\nof a collection of cubemaps. The first six layers represent the first cubemap, the next six layers form\nthe second cubemap, and so on.\nThe `flags` parameter enables different options to be specified that affect\nthe allocation, as follows.\n- ::cudaArrayDefault: This flag's value is defined to be 0 and provides default array allocation\n- ::cudaArrayLayered: Allocates a layered CUDA array, with the depth extent indicating the number of layers\n- ::cudaArrayCubemap: Allocates a cubemap CUDA array. Width must be equal to height, and depth must be six.\nIf the cudaArrayLayered flag is also set, depth must be a multiple of six.\n- ::cudaArraySurfaceLoadStore: Allocates a CUDA array that could be read from or written to using a surface\nreference.\n- ::cudaArrayTextureGather: This flag indicates that texture gather operations will be performed on the CUDA\narray. Texture gather can only be performed on 2D CUDA arrays.\n- ::cudaArraySparse: Allocates a CUDA array without physical backing memory. The subregions within this sparse array\ncan later be mapped onto a physical memory allocation by calling ::cuMemMapArrayAsync. This flag can only be used for\ncreating 2D, 3D or 2D layered sparse CUDA arrays. The physical backing memory must be allocated via ::cuMemCreate.\n- ::cudaArrayDeferredMapping: Allocates a CUDA array without physical backing memory. The entire array can\nlater be mapped onto a physical memory allocation by calling ::cuMemMapArrayAsync. The physical backing memory must be allocated\nvia ::cuMemCreate.\nThe width, height and depth extents must meet certain size requirements as listed in the following table.\nAll values are specified in elements.\nNote that 2D CUDA arrays have different size requirements if the ::cudaArrayTextureGather flag is set. In that\ncase, the valid range for (width, height, depth) is ((1,maxTexture2DGather[0]), (1,maxTexture2DGather[1]), 0).\n\\xmlonly <table outputclass=\"xmlonly\">\n<tgroup cols=\"3\" colsep=\"1\" rowsep=\"1\">\n<colspec colname=\"c1\" colwidth=\"1.0*\"/>\n<colspec colname=\"c2\" colwidth=\"3.0*\"/>\n<colspec colname=\"c3\" colwidth=\"3.0*\"/>\n<thead>\n<row>\n<entry>CUDA array type</entry>\n<entry>Valid extents that must always be met {(width range in elements),\n(height range), (depth range)}</entry>\n<entry>Valid extents with cudaArraySurfaceLoadStore set {(width range in\nelements), (height range), (depth range)}</entry>\n</row>\n</thead>\n<tbody>\n<row>\n<entry>1D</entry>\n<entry>{ (1,maxTexture1D), 0, 0 }</entry>\n<entry>{ (1,maxSurface1D), 0, 0 }</entry>\n</row>\n<row>\n<entry>2D</entry>\n<entry>{ (1,maxTexture2D[0]), (1,maxTexture2D[1]), 0 }</entry>\n<entry>{ (1,maxSurface2D[0]), (1,maxSurface2D[1]), 0 }</entry>\n</row>\n<row>\n<entry>3D</entry>\n<entry>{ (1,maxTexture3D[0]), (1,maxTexture3D[1]), (1,maxTexture3D[2]) }\nOR { (1,maxTexture3DAlt[0]), (1,maxTexture3DAlt[1]),\n(1,maxTexture3DAlt[2]) }</entry>\n<entry>{ (1,maxSurface3D[0]), (1,maxSurface3D[1]), (1,maxSurface3D[2]) }</entry>\n</row>\n<row>\n<entry>1D Layered</entry>\n<entry>{ (1,maxTexture1DLayered[0]), 0, (1,maxTexture1DLayered[1]) }</entry>\n<entry>{ (1,maxSurface1DLayered[0]), 0, (1,maxSurface1DLayered[1]) }</entry>\n</row>\n<row>\n<entry>2D Layered</entry>\n<entry>{ (1,maxTexture2DLayered[0]), (1,maxTexture2DLayered[1]),\n(1,maxTexture2DLayered[2]) }</entry>\n<entry>{ (1,maxSurface2DLayered[0]), (1,maxSurface2DLayered[1]),\n(1,maxSurface2DLayered[2]) }</entry>\n</row>\n<row>\n<entry>Cubemap</entry>\n<entry>{ (1,maxTextureCubemap), (1,maxTextureCubemap), 6 }</entry>\n<entry>{ (1,maxSurfaceCubemap), (1,maxSurfaceCubemap), 6 }</entry>\n</row>\n<row>\n<entry>Cubemap Layered</entry>\n<entry>{ (1,maxTextureCubemapLayered[0]), (1,maxTextureCubemapLayered[0]),\n(1,maxTextureCubemapLayered[1]) }</entry>\n<entry>{ (1,maxSurfaceCubemapLayered[0]), (1,maxSurfaceCubemapLayered[0]),\n(1,maxSurfaceCubemapLayered[1]) }</entry>\n</row>\n</tbody>\n</tgroup>\n</table>\n\\endxmlonly # Arguments\n\n* `array` -  - Pointer to allocated array in device memory\n* `desc` -   - Requested channel format\n* `extent` - - Requested allocation size (`width` field in elements)\n* `flags` -  - Flags for extensions\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorMemoryAllocation\n\\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaMalloc3D,`] ::cudaMalloc, ::cudaMallocPitch, ::cudaFree,\n::cudaFreeArray,\n[`::cudaMallocHost(void**,`] size_t) \"cudaMallocHost (C API)\",\n::cudaFreeHost, ::cudaHostAlloc,\n::make_cudaExtent,\n::cuArray3DCreate"]
pub unsafe fn cudaMalloc3DArray(
    desc: *const cudaChannelFormatDesc,
    extent: cudaExtent,
    flags: u32,
) -> Result<cudaArray_t, crate::sys::cudaError> {
    let mut dev_ptr: cudaArray_t = unsafe { std::mem::zeroed() };
    let status = unsafe { crate::sys::cudaMalloc3DArray(&mut dev_ptr as *mut _ as *mut _, desc, extent, flags as _) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(dev_ptr)
    } else {
        Err(status)
    }
}
#[doc = "Allocate a mipmapped array on the device\nAllocates a CUDA mipmapped array according to the ::cudaChannelFormatDesc structure\n`desc` and returns a handle to the new CUDA mipmapped array in `*mipmappedArray.`\n`numLevels` specifies the number of mipmap levels to be allocated. This value is\nclamped to the range [1, 1 + floor(log2(max(width, height, depth)))].\nThe ::cudaChannelFormatDesc is defined as:\n\\code struct cudaChannelFormatDesc {\nint x, y, z, w;\nenum cudaChannelFormatKind f;\n};\n\\endcode where ::cudaChannelFormatKind is one of ::cudaChannelFormatKindSigned,\n::cudaChannelFormatKindUnsigned, or ::cudaChannelFormatKindFloat.\n::cudaMallocMipmappedArray() can allocate the following:\n- A 1D mipmapped array is allocated if the height and depth extents are both zero.\n- A 2D mipmapped array is allocated if only the depth extent is zero.\n- A 3D mipmapped array is allocated if all three extents are non-zero.\n- A 1D layered CUDA mipmapped array is allocated if only the height extent is zero and\nthe cudaArrayLayered flag is set. Each layer is a 1D mipmapped array. The number of layers is\ndetermined by the depth extent.\n- A 2D layered CUDA mipmapped array is allocated if all three extents are non-zero and\nthe cudaArrayLayered flag is set. Each layer is a 2D mipmapped array. The number of layers is\ndetermined by the depth extent.\n- A cubemap CUDA mipmapped array is allocated if all three extents are non-zero and the\ncudaArrayCubemap flag is set. Width must be equal to height, and depth must be six.\nThe order of the six layers in memory is the same as that listed in ::cudaGraphicsCubeFace.\n- A cubemap layered CUDA mipmapped array is allocated if all three extents are non-zero, and both,\ncudaArrayCubemap and cudaArrayLayered flags are set. Width must be equal to height, and depth must be\na multiple of six. A cubemap layered CUDA mipmapped array is a special type of 2D layered CUDA mipmapped\narray that consists of a collection of cubemap mipmapped arrays. The first six layers represent the\nfirst cubemap mipmapped array, the next six layers form the second cubemap mipmapped array, and so on.\nThe `flags` parameter enables different options to be specified that affect\nthe allocation, as follows.\n- ::cudaArrayDefault: This flag's value is defined to be 0 and provides default mipmapped array allocation\n- ::cudaArrayLayered: Allocates a layered CUDA mipmapped array, with the depth extent indicating the number of layers\n- ::cudaArrayCubemap: Allocates a cubemap CUDA mipmapped array. Width must be equal to height, and depth must be six.\nIf the cudaArrayLayered flag is also set, depth must be a multiple of six.\n- ::cudaArraySurfaceLoadStore: This flag indicates that individual mipmap levels of the CUDA mipmapped array\nwill be read from or written to using a surface reference.\n- ::cudaArrayTextureGather: This flag indicates that texture gather operations will be performed on the CUDA\narray. Texture gather can only be performed on 2D CUDA mipmapped arrays, and the gather operations are\nperformed only on the most detailed mipmap level.\n- ::cudaArraySparse: Allocates a CUDA mipmapped array without physical backing memory. The subregions within this sparse array\ncan later be mapped onto a physical memory allocation by calling ::cuMemMapArrayAsync. This flag can only be used for creating\n2D, 3D or 2D layered sparse CUDA mipmapped arrays. The physical backing memory must be allocated via ::cuMemCreate.\n- ::cudaArrayDeferredMapping: Allocates a CUDA mipmapped array without physical backing memory. The entire array can\nlater be mapped onto a physical memory allocation by calling ::cuMemMapArrayAsync. The physical backing memory must be allocated\nvia ::cuMemCreate.\nThe width, height and depth extents must meet certain size requirements as listed in the following table.\nAll values are specified in elements.\n\\xmlonly <table outputclass=\"xmlonly\">\n<tgroup cols=\"3\" colsep=\"1\" rowsep=\"1\">\n<colspec colname=\"c1\" colwidth=\"1.0*\"/>\n<colspec colname=\"c2\" colwidth=\"3.0*\"/>\n<colspec colname=\"c3\" colwidth=\"3.0*\"/>\n<thead>\n<row>\n<entry>CUDA array type</entry>\n<entry>Valid extents that must always be met {(width range in elements),\n(height range), (depth range)}</entry>\n<entry>Valid extents with cudaArraySurfaceLoadStore set {(width range in\nelements), (height range), (depth range)}</entry>\n</row>\n</thead>\n<tbody>\n<row>\n<entry>1D</entry>\n<entry>{ (1,maxTexture1DMipmap), 0, 0 }</entry>\n<entry>{ (1,maxSurface1D), 0, 0 }</entry>\n</row>\n<row>\n<entry>2D</entry>\n<entry>{ (1,maxTexture2DMipmap[0]), (1,maxTexture2DMipmap[1]), 0 }</entry>\n<entry>{ (1,maxSurface2D[0]), (1,maxSurface2D[1]), 0 }</entry>\n</row>\n<row>\n<entry>3D</entry>\n<entry>{ (1,maxTexture3D[0]), (1,maxTexture3D[1]), (1,maxTexture3D[2]) }\nOR { (1,maxTexture3DAlt[0]), (1,maxTexture3DAlt[1]),\n(1,maxTexture3DAlt[2]) }</entry>\n<entry>{ (1,maxSurface3D[0]), (1,maxSurface3D[1]), (1,maxSurface3D[2]) }</entry>\n</row>\n<row>\n<entry>1D Layered</entry>\n<entry>{ (1,maxTexture1DLayered[0]), 0, (1,maxTexture1DLayered[1]) }</entry>\n<entry>{ (1,maxSurface1DLayered[0]), 0, (1,maxSurface1DLayered[1]) }</entry>\n</row>\n<row>\n<entry>2D Layered</entry>\n<entry>{ (1,maxTexture2DLayered[0]), (1,maxTexture2DLayered[1]),\n(1,maxTexture2DLayered[2]) }</entry>\n<entry>{ (1,maxSurface2DLayered[0]), (1,maxSurface2DLayered[1]),\n(1,maxSurface2DLayered[2]) }</entry>\n</row>\n<row>\n<entry>Cubemap</entry>\n<entry>{ (1,maxTextureCubemap), (1,maxTextureCubemap), 6 }</entry>\n<entry>{ (1,maxSurfaceCubemap), (1,maxSurfaceCubemap), 6 }</entry>\n</row>\n<row>\n<entry>Cubemap Layered</entry>\n<entry>{ (1,maxTextureCubemapLayered[0]), (1,maxTextureCubemapLayered[0]),\n(1,maxTextureCubemapLayered[1]) }</entry>\n<entry>{ (1,maxSurfaceCubemapLayered[0]), (1,maxSurfaceCubemapLayered[0]),\n(1,maxSurfaceCubemapLayered[1]) }</entry>\n</row>\n</tbody>\n</tgroup>\n</table>\n\\endxmlonly # Arguments\n\n* `mipmappedArray` -  - Pointer to allocated mipmapped array in device memory\n* `desc` -            - Requested channel format\n* `extent` -          - Requested allocation size (`width` field in elements)\n* `numLevels` -       - Number of mipmap levels to allocate\n* `flags` -           - Flags for extensions\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorMemoryAllocation\n\\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaMalloc3D,`] ::cudaMalloc, ::cudaMallocPitch, ::cudaFree,\n::cudaFreeArray,\n[`::cudaMallocHost(void**,`] size_t) \"cudaMallocHost (C API)\",\n::cudaFreeHost, ::cudaHostAlloc,\n::make_cudaExtent,\n::cuMipmappedArrayCreate"]
pub unsafe fn cudaMallocMipmappedArray(
    desc: *const cudaChannelFormatDesc,
    extent: cudaExtent,
    numLevels: u32,
    flags: u32,
) -> Result<cudaMipmappedArray_t, crate::sys::cudaError> {
    let mut dev_ptr: cudaMipmappedArray_t = unsafe { std::mem::zeroed() };
    let status = unsafe {
        crate::sys::cudaMallocMipmappedArray(
            &mut dev_ptr as *mut _ as *mut _,
            desc,
            extent,
            numLevels as _,
            flags as _,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(dev_ptr)
    } else {
        Err(status)
    }
}
#[doc = "Gets a mipmap level of a CUDA mipmapped array\nReturns in `*levelArray` a CUDA array that represents a single mipmap level\nof the CUDA mipmapped array `mipmappedArray.`\nIf `level` is greater than the maximum number of levels in this mipmapped array,\n::cudaErrorInvalidValue is returned.\nIf `mipmappedArray` is NULL,\n::cudaErrorInvalidResourceHandle is returned.\n\n# Arguments\n\n* `levelArray` -     - Returned mipmap level CUDA array\n* `mipmappedArray` - - CUDA mipmapped array\n* `level` -          - Mipmap level\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue\n::cudaErrorInvalidResourceHandle\n\\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaMalloc3D,`] ::cudaMalloc, ::cudaMallocPitch, ::cudaFree,\n::cudaFreeArray,\n[`::cudaMallocHost(void**,`] size_t) \"cudaMallocHost (C API)\",\n::cudaFreeHost, ::cudaHostAlloc,\n::make_cudaExtent,\n::cuMipmappedArrayGetLevel"]
pub unsafe fn cudaGetMipmappedArrayLevel<T: types::CudaAsPtr>(
    mut levelArray: T,
    mipmappedArray: cudaMipmappedArray_const_t,
    level: u32,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGetMipmappedArrayLevel(levelArray.as_mut_ptr() as *mut _, mipmappedArray, level as _)
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Copies data between 3D objects\n\\code struct cudaExtent {\nsize_t width;\nsize_t height;\nsize_t depth;\n};\nstruct cudaExtent make_cudaExtent(size_t w, size_t h, size_t d);\nstruct cudaPos {\nsize_t x;\nsize_t y;\nsize_t z;\n};\nstruct cudaPos make_cudaPos(size_t x, size_t y, size_t z);\nstruct cudaMemcpy3DParms {\ncudaArray_t           srcArray;\nstruct cudaPos        srcPos;\nstruct cudaPitchedPtr srcPtr;\ncudaArray_t           dstArray;\nstruct cudaPos        dstPos;\nstruct cudaPitchedPtr dstPtr;\nstruct cudaExtent     extent;\nenum cudaMemcpyKind   kind;\n};\n\\endcode ::cudaMemcpy3D() copies data betwen two 3D objects. The source and\ndestination objects may be in either host memory, device memory, or a CUDA\narray. The source, destination, extent, and kind of copy performed is\nspecified by the ::cudaMemcpy3DParms struct which should be initialized to\nzero before use:\n\\code cudaMemcpy3DParms myParms = {0};\n\\endcode The struct passed to ::cudaMemcpy3D() must specify one of `srcArray` or\n`srcPtr` and one of `dstArray` or `dstPtr.` Passing more than one\nnon-zero source or destination will cause ::cudaMemcpy3D() to return an\nerror.\nThe `srcPos` and `dstPos` fields are optional offsets into the source and\ndestination objects and are defined in units of each object's elements. The\nelement for a host or device pointer is assumed to be <b>unsigned char</b>.\nThe `extent` field defines the dimensions of the transferred area in\nelements. If a CUDA array is participating in the copy, the extent is\ndefined in terms of that array's elements. If no CUDA array is\nparticipating in the copy then the extents are defined in elements of\n<b>unsigned char</b>.\nThe `kind` field defines the direction of the copy. It must be one of\n::cudaMemcpyHostToHost, ::cudaMemcpyHostToDevice, ::cudaMemcpyDeviceToHost,\n::cudaMemcpyDeviceToDevice, or ::cudaMemcpyDefault. Passing\n::cudaMemcpyDefault is recommended, in which case the type of transfer is\ninferred from the pointer values. However, ::cudaMemcpyDefault is only\nallowed on systems that support unified virtual addressing.\nFor ::cudaMemcpyHostToHost or ::cudaMemcpyHostToDevice or ::cudaMemcpyDeviceToHost\npassed as kind and cudaArray type passed as source or destination, if the kind\nimplies cudaArray type to be present on the host, ::cudaMemcpy3D() will\ndisregard that implication and silently correct the kind based on the fact that\ncudaArray type can only be present on the device.\nIf the source and destination are both arrays, ::cudaMemcpy3D() will return\nan error if they do not have the same element size.\nThe source and destination object may not overlap. If overlapping source\nand destination objects are specified, undefined behavior will result.\nThe source object must entirely contain the region defined by `srcPos`\nand `extent.` The destination object must entirely contain the region\ndefined by `dstPos` and `extent.`\n::cudaMemcpy3D() returns an error if the pitch of `srcPtr` or `dstPtr`\nexceeds the maximum allowed. The pitch of a ::cudaPitchedPtr allocated\nwith ::cudaMalloc3D() will always be valid.\n\n# Arguments\n\n* `p` - - 3D memory copy parameters\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorInvalidPitchValue,\n::cudaErrorInvalidMemcpyDirection\n\\notefnerr \\note_sync \\note_init_rt \\note_callback # See also\n\n> [`::cudaMalloc3D,`] ::cudaMalloc3DArray, ::cudaMemset3D, ::cudaMemcpy3DAsync,\n::cudaMemcpy, ::cudaMemcpy2D,\n::cudaMemcpy2DToArray, ::cudaMemcpy2DFromArray,\n::cudaMemcpy2DArrayToArray, ::cudaMemcpyToSymbol,\n::cudaMemcpyFromSymbol, ::cudaMemcpyAsync, ::cudaMemcpy2DAsync,\n::cudaMemcpy2DToArrayAsync,\n::cudaMemcpy2DFromArrayAsync,\n::cudaMemcpyToSymbolAsync, ::cudaMemcpyFromSymbolAsync,\n::make_cudaExtent, ::make_cudaPos,\n::cuMemcpy3D"]
pub unsafe fn cudaMemcpy3D<T: types::CudaAsPtr>(p: T) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaMemcpy3D(p.as_const_ptr() as *const _) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Copies memory between devices\nPerform a 3D memory copy according to the parameters specified in\n`p.`  See the definition of the ::cudaMemcpy3DPeerParms structure\nfor documentation of its parameters.\nNote that this function is synchronous with respect to the host only if\nthe source or destination of the transfer is host memory.  Note also\nthat this copy is serialized with respect to all pending and future\nasynchronous work in to the current device, the copy's source device,\nand the copy's destination device (use ::cudaMemcpy3DPeerAsync to avoid\nthis synchronization).\n\n# Arguments\n\n* `p` - - Parameters for the memory copy\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorInvalidDevice,\n::cudaErrorInvalidPitchValue\n\\notefnerr \\note_sync \\note_init_rt \\note_callback # See also\n\n> [`::cudaMemcpy,`] ::cudaMemcpyPeer, ::cudaMemcpyAsync, ::cudaMemcpyPeerAsync,\n::cudaMemcpy3DPeerAsync,\n::cuMemcpy3DPeer"]
pub unsafe fn cudaMemcpy3DPeer<T: types::CudaAsPtr>(p: T) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaMemcpy3DPeer(p.as_const_ptr() as *const _) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Copies data between 3D objects\n\\code struct cudaExtent {\nsize_t width;\nsize_t height;\nsize_t depth;\n};\nstruct cudaExtent make_cudaExtent(size_t w, size_t h, size_t d);\nstruct cudaPos {\nsize_t x;\nsize_t y;\nsize_t z;\n};\nstruct cudaPos make_cudaPos(size_t x, size_t y, size_t z);\nstruct cudaMemcpy3DParms {\ncudaArray_t           srcArray;\nstruct cudaPos        srcPos;\nstruct cudaPitchedPtr srcPtr;\ncudaArray_t           dstArray;\nstruct cudaPos        dstPos;\nstruct cudaPitchedPtr dstPtr;\nstruct cudaExtent     extent;\nenum cudaMemcpyKind   kind;\n};\n\\endcode ::cudaMemcpy3DAsync() copies data betwen two 3D objects. The source and\ndestination objects may be in either host memory, device memory, or a CUDA\narray. The source, destination, extent, and kind of copy performed is\nspecified by the ::cudaMemcpy3DParms struct which should be initialized to\nzero before use:\n\\code cudaMemcpy3DParms myParms = {0};\n\\endcode The struct passed to ::cudaMemcpy3DAsync() must specify one of `srcArray`\nor `srcPtr` and one of `dstArray` or `dstPtr.` Passing more than one\nnon-zero source or destination will cause ::cudaMemcpy3DAsync() to return an\nerror.\nThe `srcPos` and `dstPos` fields are optional offsets into the source and\ndestination objects and are defined in units of each object's elements. The\nelement for a host or device pointer is assumed to be <b>unsigned char</b>.\nFor CUDA arrays, positions must be in the range [0, 2048) for any\ndimension.\nThe `extent` field defines the dimensions of the transferred area in\nelements. If a CUDA array is participating in the copy, the extent is\ndefined in terms of that array's elements. If no CUDA array is\nparticipating in the copy then the extents are defined in elements of\n<b>unsigned char</b>.\nThe `kind` field defines the direction of the copy. It must be one of\n::cudaMemcpyHostToHost, ::cudaMemcpyHostToDevice, ::cudaMemcpyDeviceToHost,\n::cudaMemcpyDeviceToDevice, or ::cudaMemcpyDefault. Passing\n::cudaMemcpyDefault is recommended, in which case the type of transfer is\ninferred from the pointer values. However, ::cudaMemcpyDefault is only\nallowed on systems that support unified virtual addressing.\nFor ::cudaMemcpyHostToHost or ::cudaMemcpyHostToDevice or ::cudaMemcpyDeviceToHost\npassed as kind and cudaArray type passed as source or destination, if the kind\nimplies cudaArray type to be present on the host, ::cudaMemcpy3DAsync() will\ndisregard that implication and silently correct the kind based on the fact that\ncudaArray type can only be present on the device.\nIf the source and destination are both arrays, ::cudaMemcpy3DAsync() will\nreturn an error if they do not have the same element size.\nThe source and destination object may not overlap. If overlapping source\nand destination objects are specified, undefined behavior will result.\nThe source object must lie entirely within the region defined by `srcPos`\nand `extent.` The destination object must lie entirely within the region\ndefined by `dstPos` and `extent.`\n::cudaMemcpy3DAsync() returns an error if the pitch of `srcPtr` or\n`dstPtr` exceeds the maximum allowed. The pitch of a\n::cudaPitchedPtr allocated with ::cudaMalloc3D() will always be valid.\n::cudaMemcpy3DAsync() is asynchronous with respect to the host, so\nthe call may return before the copy is complete. The copy can optionally\nbe associated to a stream by passing a non-zero `stream` argument. If\n`kind` is ::cudaMemcpyHostToDevice or ::cudaMemcpyDeviceToHost and `stream`\nis non-zero, the copy may overlap with operations in other streams.\nThe device version of this function only handles device to device copies and\ncannot be given local or shared pointers.\n\n# Arguments\n\n* `p` -      - 3D memory copy parameters\n* `stream` - - Stream identifier\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorInvalidPitchValue,\n::cudaErrorInvalidMemcpyDirection\n\\notefnerr \\note_async \\note_null_stream \\note_init_rt \\note_callback # See also\n\n> [`::cudaMalloc3D,`] ::cudaMalloc3DArray, ::cudaMemset3D, ::cudaMemcpy3D,\n::cudaMemcpy, ::cudaMemcpy2D,\n::cudaMemcpy2DToArray, :::cudaMemcpy2DFromArray,\n::cudaMemcpy2DArrayToArray, ::cudaMemcpyToSymbol,\n::cudaMemcpyFromSymbol, ::cudaMemcpyAsync, ::cudaMemcpy2DAsync,\n::cudaMemcpy2DToArrayAsync,\n::cudaMemcpy2DFromArrayAsync,\n::cudaMemcpyToSymbolAsync, ::cudaMemcpyFromSymbolAsync,\n::make_cudaExtent, ::make_cudaPos,\n::cuMemcpy3DAsync"]
pub unsafe fn cudaMemcpy3DAsync<T: types::CudaAsPtr>(p: T, stream: cudaStream_t) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaMemcpy3DAsync(p.as_const_ptr() as *const _, stream) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Copies memory between devices asynchronously.\nPerform a 3D memory copy according to the parameters specified in\n`p.`  See the definition of the ::cudaMemcpy3DPeerParms structure\nfor documentation of its parameters.\n\n# Arguments\n\n* `p` -      - Parameters for the memory copy\n* `stream` - - Stream identifier\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorInvalidDevice,\n::cudaErrorInvalidPitchValue\n\\notefnerr \\note_async \\note_null_stream \\note_init_rt \\note_callback # See also\n\n> [`::cudaMemcpy,`] ::cudaMemcpyPeer, ::cudaMemcpyAsync, ::cudaMemcpyPeerAsync,\n::cudaMemcpy3DPeerAsync,\n::cuMemcpy3DPeerAsync"]
pub unsafe fn cudaMemcpy3DPeerAsync<T: types::CudaAsPtr>(
    p: T,
    stream: cudaStream_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaMemcpy3DPeerAsync(p.as_const_ptr() as *const _, stream) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Gets free and total device memory\nReturns in `*total` the total amount of memory available to the the current context.\nReturns in `*free` the amount of memory on the device that is free according to the OS.\nCUDA is not guaranteed to be able to allocate all of the memory that the OS reports as free.\nIn a multi-tenet situation, free estimate returned is prone to race condition where\na new allocation/free done by a different process or a different thread in the same\nprocess between the time when free memory was estimated and reported, will result in\ndeviation in free value reported and actual free memory.\nThe integrated GPU on Tegra shares memory with CPU and other component\nof the SoC. The free and total values returned by the API excludes\nthe SWAP memory space maintained by the OS on some platforms.\nThe OS may move some of the memory pages into swap area as the GPU or\nCPU allocate or access memory. See Tegra app note on how to calculate\ntotal and free memory on Tegra.\n\n# Arguments\n\n* `free` -  - Returned free memory in bytes\n* `total` - - Returned total memory in bytes\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorLaunchFailure\n\\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cuMemGetInfo`]"]
pub unsafe fn cudaMemGetInfo() -> Result<(usize, usize), crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let mut out_1: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaMemGetInfo(out_0.as_mut_ptr() as *mut _, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok((out_0.assume_init() as usize, out_1.assume_init() as usize)) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Gets info about the specified cudaArray\nReturns in `*desc,` `*extent` and `*flags` respectively, the type, shape\nand flags of `array.`\nAny of `*desc,` `*extent` and `*flags` may be specified as NULL.\n\n# Arguments\n\n* `desc` -   - Returned array type\n* `extent` - - Returned array shape. 2D arrays will have depth of zero\n* `flags` -  - Returned array flags\n* `array` -  - The ::cudaArray to get info for\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue\n\\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cuArrayGetDescriptor,`]\n::cuArray3DGetDescriptor"]
pub unsafe fn cudaArrayGetInfo(
    array: cudaArray_t,
) -> Result<(cudaChannelFormatDesc, cudaExtent, u32), crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<cudaChannelFormatDesc> = std::mem::MaybeUninit::uninit();
    let mut out_1: std::mem::MaybeUninit<cudaExtent> = std::mem::MaybeUninit::uninit();
    let mut out_2: std::mem::MaybeUninit<::std::os::raw::c_uint> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudaArrayGetInfo(
            out_0.as_mut_ptr() as *mut _,
            out_1.as_mut_ptr() as *mut _,
            out_2.as_mut_ptr() as *mut _,
            array,
        )
    };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe {
            Ok((
                out_0.assume_init() as cudaChannelFormatDesc,
                out_1.assume_init() as cudaExtent,
                out_2.assume_init() as u32,
            ))
        }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Gets a CUDA array plane from a CUDA array\nReturns in `pPlaneArray` a CUDA array that represents a single format plane\nof the CUDA array `hArray.`\nIf `planeIdx` is greater than the maximum number of planes in this array or if the array does\nnot have a multi-planar format e.g: ::cudaChannelFormatKindNV12, then ::cudaErrorInvalidValue is returned.\nNote that if the `hArray` has format ::cudaChannelFormatKindNV12, then passing in 0 for `planeIdx` returns\na CUDA array of the same size as `hArray` but with one 8-bit channel and ::cudaChannelFormatKindUnsigned as its format kind.\nIf 1 is passed for `planeIdx,` then the returned CUDA array has half the height and width\nof `hArray` with two 8-bit channels and ::cudaChannelFormatKindUnsigned as its format kind.\n\n# Arguments\n\n* `pPlaneArray` -   - Returned CUDA array referenced by the `planeIdx`\n* `hArray` -        - CUDA array\n* `planeIdx` -      - Plane index\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue\n::cudaErrorInvalidResourceHandle\n\\notefnerr # See also\n\n> [`::cuArrayGetPlane`]"]
pub unsafe fn cudaArrayGetPlane<T: types::CudaAsPtr>(
    mut pPlaneArray: T,
    hArray: cudaArray_t,
    planeIdx: u32,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaArrayGetPlane(pPlaneArray.as_mut_ptr() as *mut _, hArray, planeIdx as _) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Returns the memory requirements of a CUDA array\nReturns the memory requirements of a CUDA array in `memoryRequirements`\nIf the CUDA array is not allocated with flag ::cudaArrayDeferredMapping\n::cudaErrorInvalidValue will be returned.\nThe returned value in ::cudaArrayMemoryRequirements::size\nrepresents the total size of the CUDA array.\nThe returned value in ::cudaArrayMemoryRequirements::alignment\nrepresents the alignment necessary for mapping the CUDA array.\n\n# Returns\n\n::cudaSuccess\n::cudaErrorInvalidValue\n\n# Arguments\n\n* `memoryRequirements` [out]  - - Pointer to ::cudaArrayMemoryRequirements\n* `array` [in]  - - CUDA array to get the memory requirements of\n* `device` [in]  - - Device to get the memory requirements for\n\n# See also\n\n> [`::cudaMipmappedArrayGetMemoryRequirements`]"]
pub unsafe fn cudaArrayGetMemoryRequirements(
    array: cudaArray_t,
    device: i32,
) -> Result<cudaArrayMemoryRequirements, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<cudaArrayMemoryRequirements> = std::mem::MaybeUninit::uninit();
    let status =
        unsafe { crate::sys::cudaArrayGetMemoryRequirements(out_0.as_mut_ptr() as *mut _, array, device as _) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_0.assume_init() as cudaArrayMemoryRequirements) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Returns the memory requirements of a CUDA mipmapped array\nReturns the memory requirements of a CUDA mipmapped array in `memoryRequirements`\nIf the CUDA mipmapped array is not allocated with flag ::cudaArrayDeferredMapping\n::cudaErrorInvalidValue will be returned.\nThe returned value in ::cudaArrayMemoryRequirements::size\nrepresents the total size of the CUDA mipmapped array.\nThe returned value in ::cudaArrayMemoryRequirements::alignment\nrepresents the alignment necessary for mapping the CUDA mipmapped\narray.\n\n# Returns\n\n::cudaSuccess\n::cudaErrorInvalidValue\n\n# Arguments\n\n* `memoryRequirements` [out]  - - Pointer to ::cudaArrayMemoryRequirements\n* `mipmap` [in]  - - CUDA mipmapped array to get the memory requirements of\n* `device` [in]  - - Device to get the memory requirements for\n\n# See also\n\n> [`::cudaArrayGetMemoryRequirements`]"]
pub unsafe fn cudaMipmappedArrayGetMemoryRequirements(
    mipmap: cudaMipmappedArray_t,
    device: i32,
) -> Result<cudaArrayMemoryRequirements, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<cudaArrayMemoryRequirements> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudaMipmappedArrayGetMemoryRequirements(out_0.as_mut_ptr() as *mut _, mipmap, device as _)
    };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_0.assume_init() as cudaArrayMemoryRequirements) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudaArrayGetSparseProperties(
    array: cudaArray_t,
) -> Result<cudaArraySparseProperties, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<cudaArraySparseProperties> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaArrayGetSparseProperties(out_0.as_mut_ptr() as *mut _, array) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_0.assume_init() as cudaArraySparseProperties) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudaMipmappedArrayGetSparseProperties(
    mipmap: cudaMipmappedArray_t,
) -> Result<cudaArraySparseProperties, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<cudaArraySparseProperties> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaMipmappedArrayGetSparseProperties(out_0.as_mut_ptr() as *mut _, mipmap) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_0.assume_init() as cudaArraySparseProperties) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Copies data between host and device\nCopies `count` bytes from the memory area pointed to by `src` to the\nmemory area pointed to by `dst,` where `kind` specifies the direction\nof the copy, and must be one of ::cudaMemcpyHostToHost,\n::cudaMemcpyHostToDevice, ::cudaMemcpyDeviceToHost,\n::cudaMemcpyDeviceToDevice, or ::cudaMemcpyDefault. Passing\n::cudaMemcpyDefault is recommended, in which case the type of transfer is\ninferred from the pointer values. However, ::cudaMemcpyDefault is only\nallowed on systems that support unified virtual addressing. Calling\n::cudaMemcpy() with dst and src pointers that do not match the direction of\nthe copy results in an undefined behavior.\n\n# Arguments\n\n* `dst` -   - Destination memory address\n* `src` -   - Source memory address\n* `count` - - Size in bytes to copy\n* `kind` -  - Type of transfer\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorInvalidMemcpyDirection\n\\notefnerr \\note_init_rt \\note_callback \\note_sync \\note_memcpy # See also\n\n> [`::cudaMemcpy2D,`]\n::cudaMemcpy2DToArray, ::cudaMemcpy2DFromArray,\n::cudaMemcpy2DArrayToArray, ::cudaMemcpyToSymbol,\n::cudaMemcpyFromSymbol, ::cudaMemcpyAsync, ::cudaMemcpy2DAsync,\n::cudaMemcpy2DToArrayAsync,\n::cudaMemcpy2DFromArrayAsync,\n::cudaMemcpyToSymbolAsync, ::cudaMemcpyFromSymbolAsync,\n::cuMemcpyDtoH,\n::cuMemcpyHtoD,\n::cuMemcpyDtoD,\n::cuMemcpy"]
pub unsafe fn cudaMemcpy<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    mut dst: T,
    src: U,
    count: usize,
    kind: cudaMemcpyKind,
) -> Result<(), crate::sys::cudaError> {
    let status =
        unsafe { crate::sys::cudaMemcpy(dst.as_mut_ptr() as *mut _, src.as_const_ptr() as *const _, count, kind) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Copies memory between two devices\nCopies memory from one device to memory on another device.  `dst` is the\nbase device pointer of the destination memory and `dstDevice` is the\ndestination device.  `src` is the base device pointer of the source memory\nand `srcDevice` is the source device.  `count` specifies the number of bytes\nto copy.\nNote that this function is asynchronous with respect to the host, but\nserialized with respect all pending and future asynchronous work in to the\ncurrent device, `srcDevice,` and `dstDevice` (use ::cudaMemcpyPeerAsync\nto avoid this synchronization).\n\n# Arguments\n\n* `dst` -       - Destination device pointer\n* `dstDevice` - - Destination device\n* `src` -       - Source device pointer\n* `srcDevice` - - Source device\n* `count` -     - Size of memory copy in bytes\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorInvalidDevice\n\\notefnerr \\note_sync \\note_init_rt \\note_callback # See also\n\n> [`::cudaMemcpy,`] ::cudaMemcpyAsync, ::cudaMemcpyPeerAsync,\n::cudaMemcpy3DPeerAsync,\n::cuMemcpyPeer"]
pub unsafe fn cudaMemcpyPeer<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    mut dst: T,
    dstDevice: i32,
    src: U,
    srcDevice: i32,
    count: usize,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaMemcpyPeer(
            dst.as_mut_ptr() as *mut _,
            dstDevice as _,
            src.as_const_ptr() as *const _,
            srcDevice as _,
            count,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Copies data between host and device\nCopies a matrix (`height` rows of `width` bytes each) from the memory\narea pointed to by `src` to the memory area pointed to by `dst,` where\n`kind` specifies the direction of the copy, and must be one of\n::cudaMemcpyHostToHost, ::cudaMemcpyHostToDevice, ::cudaMemcpyDeviceToHost,\n::cudaMemcpyDeviceToDevice, or ::cudaMemcpyDefault. Passing\n::cudaMemcpyDefault is recommended, in which case the type of transfer is\ninferred from the pointer values. However, ::cudaMemcpyDefault is only\nallowed on systems that support unified virtual addressing. `dpitch` and\n`spitch` are the widths in memory in bytes of the 2D arrays pointed to by\n`dst` and `src,` including any padding added to the end of each row. The\nmemory areas may not overlap. `width` must not exceed either `dpitch` or\n`spitch.` Calling ::cudaMemcpy2D() with `dst` and `src` pointers that do\nnot match the direction of the copy results in an undefined behavior.\n::cudaMemcpy2D() returns an error if `dpitch` or `spitch` exceeds\nthe maximum allowed.\n\n# Arguments\n\n* `dst` -    - Destination memory address\n* `dpitch` - - Pitch of destination memory\n* `src` -    - Source memory address\n* `spitch` - - Pitch of source memory\n* `width` -  - Width of matrix transfer (columns in bytes)\n* `height` - - Height of matrix transfer (rows)\n* `kind` -   - Type of transfer\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorInvalidPitchValue,\n::cudaErrorInvalidMemcpyDirection\n\\notefnerr \\note_init_rt \\note_callback \\note_memcpy # See also\n\n> [`::cudaMemcpy,`]\n::cudaMemcpy2DToArray, ::cudaMemcpy2DFromArray,\n::cudaMemcpy2DArrayToArray, ::cudaMemcpyToSymbol,\n::cudaMemcpyFromSymbol, ::cudaMemcpyAsync, ::cudaMemcpy2DAsync,\n::cudaMemcpy2DToArrayAsync,\n::cudaMemcpy2DFromArrayAsync,\n::cudaMemcpyToSymbolAsync, ::cudaMemcpyFromSymbolAsync,\n::cuMemcpy2D,\n::cuMemcpy2DUnaligned"]
pub unsafe fn cudaMemcpy2D<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    mut dst: T,
    dpitch: usize,
    src: U,
    spitch: usize,
    width: usize,
    height: usize,
    kind: cudaMemcpyKind,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaMemcpy2D(
            dst.as_mut_ptr() as *mut _,
            dpitch,
            src.as_const_ptr() as *const _,
            spitch,
            width,
            height,
            kind,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Copies data between host and device\nCopies a matrix (`height` rows of `width` bytes each) from the memory\narea pointed to by `src` to the CUDA array `dst` starting at\n`hOffset` rows and `wOffset` bytes from the upper left corner,\nwhere `kind` specifies the direction of the copy, and must be one\nof ::cudaMemcpyHostToHost, ::cudaMemcpyHostToDevice, ::cudaMemcpyDeviceToHost,\n::cudaMemcpyDeviceToDevice, or ::cudaMemcpyDefault. Passing\n::cudaMemcpyDefault is recommended, in which case the type of transfer is\ninferred from the pointer values. However, ::cudaMemcpyDefault is only\nallowed on systems that support unified virtual addressing.\n`spitch` is the width in memory in bytes of the 2D array pointed to by\n`src,` including any padding added to the end of each row. `wOffset` +\n`width` must not exceed the width of the CUDA array `dst.` `width` must\nnot exceed `spitch.` ::cudaMemcpy2DToArray() returns an error if `spitch`\nexceeds the maximum allowed.\n\n# Arguments\n\n* `dst` -     - Destination memory address\n* `wOffset` - - Destination starting X offset (columns in bytes)\n* `hOffset` - - Destination starting Y offset (rows)\n* `src` -     - Source memory address\n* `spitch` -  - Pitch of source memory\n* `width` -   - Width of matrix transfer (columns in bytes)\n* `height` -  - Height of matrix transfer (rows)\n* `kind` -    - Type of transfer\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorInvalidPitchValue,\n::cudaErrorInvalidMemcpyDirection\n\\notefnerr \\note_sync \\note_init_rt \\note_callback \\note_memcpy # See also\n\n> [`::cudaMemcpy,`] ::cudaMemcpy2D,\n::cudaMemcpy2DFromArray,\n::cudaMemcpy2DArrayToArray, ::cudaMemcpyToSymbol,\n::cudaMemcpyFromSymbol, ::cudaMemcpyAsync, ::cudaMemcpy2DAsync,\n::cudaMemcpy2DToArrayAsync,\n::cudaMemcpy2DFromArrayAsync,\n::cudaMemcpyToSymbolAsync, ::cudaMemcpyFromSymbolAsync,\n::cuMemcpy2D,\n::cuMemcpy2DUnaligned"]
pub unsafe fn cudaMemcpy2DToArray<T: types::CudaAsPtr>(
    dst: cudaArray_t,
    wOffset: usize,
    hOffset: usize,
    src: T,
    spitch: usize,
    width: usize,
    height: usize,
    kind: cudaMemcpyKind,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaMemcpy2DToArray(
            dst,
            wOffset,
            hOffset,
            src.as_const_ptr() as *const _,
            spitch,
            width,
            height,
            kind,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Copies data between host and device\nCopies a matrix (`height` rows of `width` bytes each) from the CUDA\narray `src` starting at `hOffset` rows and `wOffset` bytes from the\nupper left corner to the memory area pointed to by `dst,` where\n`kind` specifies the direction of the copy, and must be one of\n::cudaMemcpyHostToHost, ::cudaMemcpyHostToDevice, ::cudaMemcpyDeviceToHost,\n::cudaMemcpyDeviceToDevice, or ::cudaMemcpyDefault. Passing\n::cudaMemcpyDefault is recommended, in which case the type of transfer is\ninferred from the pointer values. However, ::cudaMemcpyDefault is only\nallowed on systems that support unified virtual addressing. `dpitch` is the\nwidth in memory in bytes of the 2D array pointed to by `dst,` including any\npadding added to the end of each row. `wOffset` + `width` must not exceed\nthe width of the CUDA array `src.` `width` must not exceed `dpitch.`\n::cudaMemcpy2DFromArray() returns an error if `dpitch` exceeds the maximum\nallowed.\n\n# Arguments\n\n* `dst` -     - Destination memory address\n* `dpitch` -  - Pitch of destination memory\n* `src` -     - Source memory address\n* `wOffset` - - Source starting X offset (columns in bytes)\n* `hOffset` - - Source starting Y offset (rows)\n* `width` -   - Width of matrix transfer (columns in bytes)\n* `height` -  - Height of matrix transfer (rows)\n* `kind` -    - Type of transfer\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorInvalidPitchValue,\n::cudaErrorInvalidMemcpyDirection\n\\notefnerr \\note_sync \\note_init_rt \\note_callback \\note_memcpy # See also\n\n> [`::cudaMemcpy,`] ::cudaMemcpy2D,\n::cudaMemcpy2DToArray,\n::cudaMemcpy2DArrayToArray, ::cudaMemcpyToSymbol,\n::cudaMemcpyFromSymbol, ::cudaMemcpyAsync, ::cudaMemcpy2DAsync,\n::cudaMemcpy2DToArrayAsync,\n::cudaMemcpy2DFromArrayAsync,\n::cudaMemcpyToSymbolAsync, ::cudaMemcpyFromSymbolAsync,\n::cuMemcpy2D,\n::cuMemcpy2DUnaligned"]
pub unsafe fn cudaMemcpy2DFromArray<T: types::CudaAsPtr>(
    mut dst: T,
    dpitch: usize,
    src: cudaArray_const_t,
    wOffset: usize,
    hOffset: usize,
    width: usize,
    height: usize,
    kind: cudaMemcpyKind,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaMemcpy2DFromArray(
            dst.as_mut_ptr() as *mut _,
            dpitch,
            src,
            wOffset,
            hOffset,
            width,
            height,
            kind,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Copies data between host and device\nCopies a matrix (`height` rows of `width` bytes each) from the CUDA\narray `src` starting at `hOffsetSrc` rows and `wOffsetSrc` bytes from the\nupper left corner to the CUDA array `dst` starting at `hOffsetDst` rows\nand `wOffsetDst` bytes from the upper left corner, where `kind`\nspecifies the direction of the copy, and must be one of\n::cudaMemcpyHostToHost, ::cudaMemcpyHostToDevice, ::cudaMemcpyDeviceToHost,\n::cudaMemcpyDeviceToDevice, or ::cudaMemcpyDefault. Passing\n::cudaMemcpyDefault is recommended, in which case the type of transfer is\ninferred from the pointer values. However, ::cudaMemcpyDefault is only\nallowed on systems that support unified virtual addressing.\n`wOffsetDst` + `width` must not exceed the width of the CUDA array `dst.`\n`wOffsetSrc` + `width` must not exceed the width of the CUDA array `src.`\n\n# Arguments\n\n* `dst` -        - Destination memory address\n* `wOffsetDst` - - Destination starting X offset (columns in bytes)\n* `hOffsetDst` - - Destination starting Y offset (rows)\n* `src` -        - Source memory address\n* `wOffsetSrc` - - Source starting X offset (columns in bytes)\n* `hOffsetSrc` - - Source starting Y offset (rows)\n* `width` -      - Width of matrix transfer (columns in bytes)\n* `height` -     - Height of matrix transfer (rows)\n* `kind` -       - Type of transfer\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorInvalidMemcpyDirection\n\\notefnerr \\note_sync \\note_init_rt \\note_callback # See also\n\n> [`::cudaMemcpy,`] ::cudaMemcpy2D,\n::cudaMemcpy2DToArray, ::cudaMemcpy2DFromArray,\n::cudaMemcpyToSymbol,\n::cudaMemcpyFromSymbol, ::cudaMemcpyAsync, ::cudaMemcpy2DAsync,\n::cudaMemcpy2DToArrayAsync,\n::cudaMemcpy2DFromArrayAsync,\n::cudaMemcpyToSymbolAsync, ::cudaMemcpyFromSymbolAsync,\n::cuMemcpy2D,\n::cuMemcpy2DUnaligned"]
pub unsafe fn cudaMemcpy2DArrayToArray(
    dst: cudaArray_t,
    wOffsetDst: usize,
    hOffsetDst: usize,
    src: cudaArray_const_t,
    wOffsetSrc: usize,
    hOffsetSrc: usize,
    width: usize,
    height: usize,
    kind: cudaMemcpyKind,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaMemcpy2DArrayToArray(
            dst, wOffsetDst, hOffsetDst, src, wOffsetSrc, hOffsetSrc, width, height, kind,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Copies data to the given symbol on the device\nCopies `count` bytes from the memory area pointed to by `src`\nto the memory area pointed to by `offset` bytes from the start of symbol\n`symbol.` The memory areas may not overlap. `symbol` is a variable that\nresides in global or constant memory space. `kind` can be either\n::cudaMemcpyHostToDevice, ::cudaMemcpyDeviceToDevice, or ::cudaMemcpyDefault.\nPassing ::cudaMemcpyDefault is recommended, in which case the type of\ntransfer is inferred from the pointer values. However, ::cudaMemcpyDefault\nis only allowed on systems that support unified virtual addressing.\n\n# Arguments\n\n* `symbol` - - Device symbol address\n* `src` -    - Source memory address\n* `count` -  - Size in bytes to copy\n* `offset` - - Offset from start of symbol in bytes\n* `kind` -   - Type of transfer\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorInvalidSymbol,\n::cudaErrorInvalidMemcpyDirection,\n::cudaErrorNoKernelImageForDevice\n\\notefnerr \\note_sync \\note_string_api_deprecation \\note_init_rt \\note_callback # See also\n\n> [`::cudaMemcpy,`] ::cudaMemcpy2D,\n::cudaMemcpy2DToArray,  ::cudaMemcpy2DFromArray,\n::cudaMemcpy2DArrayToArray,\n::cudaMemcpyFromSymbol, ::cudaMemcpyAsync, ::cudaMemcpy2DAsync,\n::cudaMemcpy2DToArrayAsync,\n::cudaMemcpy2DFromArrayAsync,\n::cudaMemcpyToSymbolAsync, ::cudaMemcpyFromSymbolAsync,\n::cuMemcpy,\n::cuMemcpyHtoD,\n::cuMemcpyDtoD"]
pub unsafe fn cudaMemcpyToSymbol<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    symbol: T,
    src: U,
    count: usize,
    offset: usize,
    kind: cudaMemcpyKind,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaMemcpyToSymbol(
            symbol.as_const_ptr() as *const _,
            src.as_const_ptr() as *const _,
            count,
            offset,
            kind,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Copies data from the given symbol on the device\nCopies `count` bytes from the memory area pointed to by `offset` bytes\nfrom the start of symbol `symbol` to the memory area pointed to by `dst.`\nThe memory areas may not overlap. `symbol` is a variable that\nresides in global or constant memory space. `kind` can be either\n::cudaMemcpyDeviceToHost, ::cudaMemcpyDeviceToDevice, or ::cudaMemcpyDefault.\nPassing ::cudaMemcpyDefault is recommended, in which case the type of\ntransfer is inferred from the pointer values. However, ::cudaMemcpyDefault\nis only allowed on systems that support unified virtual addressing.\n\n# Arguments\n\n* `dst` -    - Destination memory address\n* `symbol` - - Device symbol address\n* `count` -  - Size in bytes to copy\n* `offset` - - Offset from start of symbol in bytes\n* `kind` -   - Type of transfer\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorInvalidSymbol,\n::cudaErrorInvalidMemcpyDirection,\n::cudaErrorNoKernelImageForDevice\n\\notefnerr \\note_sync \\note_string_api_deprecation \\note_init_rt \\note_callback # See also\n\n> [`::cudaMemcpy,`] ::cudaMemcpy2D,\n::cudaMemcpy2DToArray, ::cudaMemcpy2DFromArray,\n::cudaMemcpy2DArrayToArray, ::cudaMemcpyToSymbol,\n::cudaMemcpyAsync, ::cudaMemcpy2DAsync,\n::cudaMemcpy2DToArrayAsync,\n::cudaMemcpy2DFromArrayAsync,\n::cudaMemcpyToSymbolAsync, ::cudaMemcpyFromSymbolAsync,\n::cuMemcpy,\n::cuMemcpyDtoH,\n::cuMemcpyDtoD"]
pub unsafe fn cudaMemcpyFromSymbol<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    mut dst: T,
    symbol: U,
    count: usize,
    offset: usize,
    kind: cudaMemcpyKind,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaMemcpyFromSymbol(
            dst.as_mut_ptr() as *mut _,
            symbol.as_const_ptr() as *const _,
            count,
            offset,
            kind,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Copies data between host and device\nCopies `count` bytes from the memory area pointed to by `src` to the\nmemory area pointed to by `dst,` where `kind` specifies the\ndirection of the copy, and must be one of ::cudaMemcpyHostToHost,\n::cudaMemcpyHostToDevice, ::cudaMemcpyDeviceToHost,\n::cudaMemcpyDeviceToDevice, or ::cudaMemcpyDefault. Passing\n::cudaMemcpyDefault is recommended, in which case the type of transfer is\ninferred from the pointer values. However, ::cudaMemcpyDefault is only\nallowed on systems that support unified virtual addressing.\nThe memory areas may not overlap. Calling ::cudaMemcpyAsync() with `dst` and\n`src` pointers that do not match the direction of the copy results in an\nundefined behavior.\n::cudaMemcpyAsync() is asynchronous with respect to the host, so the call\nmay return before the copy is complete. The copy can optionally be\nassociated to a stream by passing a non-zero `stream` argument. If `kind`\nis ::cudaMemcpyHostToDevice or ::cudaMemcpyDeviceToHost and the `stream` is\nnon-zero, the copy may overlap with operations in other streams.\nThe device version of this function only handles device to device copies and\ncannot be given local or shared pointers.\n\n# Arguments\n\n* `dst` -    - Destination memory address\n* `src` -    - Source memory address\n* `count` -  - Size in bytes to copy\n* `kind` -   - Type of transfer\n* `stream` - - Stream identifier\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorInvalidMemcpyDirection\n\\notefnerr \\note_async \\note_null_stream \\note_init_rt \\note_callback \\note_memcpy # See also\n\n> [`::cudaMemcpy,`] ::cudaMemcpy2D,\n::cudaMemcpy2DToArray, ::cudaMemcpy2DFromArray,\n::cudaMemcpy2DArrayToArray, ::cudaMemcpyToSymbol,\n::cudaMemcpyFromSymbol, ::cudaMemcpy2DAsync,\n::cudaMemcpy2DToArrayAsync,\n::cudaMemcpy2DFromArrayAsync,\n::cudaMemcpyToSymbolAsync, ::cudaMemcpyFromSymbolAsync,\n::cuMemcpyAsync,\n::cuMemcpyDtoHAsync,\n::cuMemcpyHtoDAsync,\n::cuMemcpyDtoDAsync"]
pub unsafe fn cudaMemcpyAsync<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    mut dst: T,
    src: U,
    count: usize,
    kind: cudaMemcpyKind,
    stream: cudaStream_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaMemcpyAsync(
            dst.as_mut_ptr() as *mut _,
            src.as_const_ptr() as *const _,
            count,
            kind,
            stream,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Copies memory between two devices asynchronously.\nCopies memory from one device to memory on another device.  `dst` is the\nbase device pointer of the destination memory and `dstDevice` is the\ndestination device.  `src` is the base device pointer of the source memory\nand `srcDevice` is the source device.  `count` specifies the number of bytes\nto copy.\nNote that this function is asynchronous with respect to the host and all work\non other devices.\n\n# Arguments\n\n* `dst` -       - Destination device pointer\n* `dstDevice` - - Destination device\n* `src` -       - Source device pointer\n* `srcDevice` - - Source device\n* `count` -     - Size of memory copy in bytes\n* `stream` -    - Stream identifier\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorInvalidDevice\n\\notefnerr \\note_async \\note_null_stream \\note_init_rt \\note_callback # See also\n\n> [`::cudaMemcpy,`] ::cudaMemcpyPeer, ::cudaMemcpyAsync,\n::cudaMemcpy3DPeerAsync,\n::cuMemcpyPeerAsync"]
pub unsafe fn cudaMemcpyPeerAsync<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    mut dst: T,
    dstDevice: i32,
    src: U,
    srcDevice: i32,
    count: usize,
    stream: cudaStream_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaMemcpyPeerAsync(
            dst.as_mut_ptr() as *mut _,
            dstDevice as _,
            src.as_const_ptr() as *const _,
            srcDevice as _,
            count,
            stream,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Performs a batch of memory copies asynchronously.\nPerforms a batch of memory copies. The batch as a whole executes in stream order but copies within a\nbatch are not guaranteed to execute in any specific order. This API only supports pointer-to-pointer copies.\nFor copies involving CUDA arrays, please see ::cudaMemcpy3DBatchAsync.\nPerforms memory copies from source buffers specified in `srcs` to destination buffers specified in `dsts.`\nThe size of each copy is specified in `sizes.` All three arrays must be of the same length as specified\nby `count.` Since there are no ordering guarantees for copies within a batch, specifying any dependent copies\nwithin a batch will result in undefined behavior.\nEvery copy in the batch has to be associated with a set of attributes specified in the `attrs` array.\nEach entry in this array can apply to more than one copy. This can be done by specifying in the `attrsIdxs` array,\nthe index of the first copy that the corresponding entry in the `attrs` array applies to. Both `attrs` and\n`attrsIdxs` must be of the same length as specified by `numAttrs.` For example, if a batch has 10 copies listed\nin dst/src/sizes, the first 6 of which have one set of attributes and the remaining 4 another, then `numAttrs`\nwill be 2, `attrsIdxs` will be {0, 6} and `attrs` will contains the two sets of attributes. Note that the first entry\nin `attrsIdxs` must always be 0. Also, each entry must be greater than the previous entry and the last entry should be\nless than `count.` Furthermore, `numAttrs` must be lesser than or equal to `count.`\nThe ::cudaMemcpyAttributes::srcAccessOrder indicates the source access ordering to be observed for copies associated\nwith the attribute. If the source access order is set to ::cudaMemcpySrcAccessOrderStream, then the source will\nbe accessed in stream order. If the source access order is set to ::cudaMemcpySrcAccessOrderDuringApiCall then\nit indicates that access to the source pointer can be out of stream order and all accesses must be complete before the\nAPI call returns. This flag is suited for ephemeral sources (ex., stack variables) when it's known that no prior\noperations in the stream can be accessing the memory and also that the lifetime of the memory is limited to the scope\nthat the source variable was declared in. Specifying this flag allows the driver to optimize the copy and removes the\nneed for the user to synchronize the stream after the API call. If the source access order is set to\n::cudaMemcpySrcAccessOrderAny then it indicates that access to the source pointer can be out of stream order and the\naccesses can happen even after the API call returns. This flag is suited for host pointers allocated\noutside CUDA (ex., via malloc) when it's known that no prior operations in the stream can be accessing the memory.\nSpecifying this flag allows the driver to optimize the copy on certain platforms. Each memcpy operation in the batch\nmust have a valid ::cudaMemcpyAttributes corresponding to it including the appropriate srcAccessOrder setting,\notherwise the API will return ::cudaErrorInvalidValue.\nThe ::cudaMemcpyAttributes::srcLocHint and ::cudaMemcpyAttributes::dstLocHint allows applications to specify hint locations\nfor operands of a copy when the operand doesn't have a fixed location. That is, these hints are\nonly applicable for managed memory pointers on devices where ::cudaDevAttrConcurrentManagedAccess is true or\nsystem-allocated pageable memory on devices where ::cudaDevAttrPageableMemoryAccess is true.\nFor other cases, these hints are ignored.\nThe ::cudaMemcpyAttributes::flags field can be used to specify certain flags for copies. Setting the\n::cudaMemcpyFlagPreferOverlapWithCompute flag indicates that the associated copies should preferably overlap with\nany compute work. Note that this flag is a hint and can be ignored depending on the platform and other parameters of the copy.\n\n# Arguments\n\n* `dsts` -          - Array of destination pointers.\n* `srcs` -          - Array of memcpy source pointers.\n* `sizes` -         - Array of sizes for memcpy operations.\n* `count` -         - Size of `dsts,` `srcs` and `sizes` arrays\n* `attrs` -         - Array of memcpy attributes.\n* `attrsIdxs` -     - Array of indices to specify which copies each entry in the `attrs` array applies to.\nThe attributes specified in attrs[k] will be applied to copies starting from attrsIdxs[k]\nthrough attrsIdxs[k+1] - 1. Also attrs[numAttrs-1] will apply to copies starting from\nattrsIdxs[numAttrs-1] through count - 1.\n* `numAttrs` -      - Size of `attrs` and `attrsIdxs` arrays.\n* `hStream` -       - The stream to enqueue the operations in. Must not be legacy NULL stream.\n\n# Returns\n\n::cudaSuccess\n::cudaErrorInvalidValue\n\\notefnerr \\note_async \\note_memcpy "]
pub unsafe fn cudaMemcpyBatchAsync<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
>(
    dsts: T,
    srcs: U,
    sizes: V,
    count: usize,
    mut attrs: W,
    mut attrsIdxs: X,
    numAttrs: usize,
    stream: cudaStream_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaMemcpyBatchAsync(
            dsts.as_const_ptr() as *const _,
            srcs.as_const_ptr() as *const _,
            sizes.as_const_ptr() as *const _,
            count,
            attrs.as_mut_ptr() as *mut _,
            attrsIdxs.as_mut_ptr() as *mut _,
            numAttrs,
            stream,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Performs a batch of 3D memory copies asynchronously.\nPerforms a batch of memory copies. The batch as a whole executes in stream order but copies within a\nbatch are not guaranteed to execute in any specific order. Note that this means specifying any dependent\ncopies within a batch will result in undefined behavior.\nPerforms memory copies as specified in the `opList` array. The length of this array is specified in `numOps.`\nEach entry in this array describes a copy operation. This includes among other things, the source and destination\noperands for the copy as specified in ::cudaMemcpy3DBatchOp::src and ::cudaMemcpy3DBatchOp::dst respectively.\nThe source and destination operands of a copy can either be a pointer or a CUDA array. The width, height and depth\nof a copy is specified in ::cudaMemcpy3DBatchOp::extent. The width, height and depth of a copy are specified in\nelements and must not be zero. For pointer-to-pointer copies, the element size is considered to be 1. For pointer\nto CUDA array or vice versa copies, the element size is determined by the CUDA array. For CUDA array to CUDA array copies,\nthe element size of the two CUDA arrays must match.\nFor a given operand, if ::cudaMemcpy3DOperand::type is specified as ::cudaMemcpyOperandTypePointer, then\n::cudaMemcpy3DOperand::op::ptr will be used. The ::cudaMemcpy3DOperand::op::ptr::ptr field must contain the pointer where\nthe copy should begin. The ::cudaMemcpy3DOperand::op::ptr::rowLength field specifies the length of each row in elements and\nmust either be zero or be greater than or equal to the width of the copy specified in ::cudaMemcpy3DBatchOp::extent::width.\nThe ::cudaMemcpy3DOperand::op::ptr::layerHeight field specifies the height of each layer and must either be\nzero or be greater than or equal to the height of the copy specified in ::cudaMemcpy3DBatchOp::extent::height.\nWhen either of these values is zero, that aspect of the operand is considered to be tightly packed according to the copy extent.\nFor managed memory pointers on devices where ::cudaDevAttrConcurrentManagedAccess is true or system-allocated pageable memory\non devices where ::cudaDevAttrPageableMemoryAccess is true, the ::cudaMemcpy3DOperand::op::ptr::locHint field can be used to hint\nthe location of the operand.\nIf an operand's type is specified as ::cudaMemcpyOperandTypeArray, then ::cudaMemcpy3DOperand::op::array will be used.\nThe ::cudaMemcpy3DOperand::op::array::array field specifies the CUDA array and ::cudaMemcpy3DOperand::op::array::offset specifies\nthe 3D offset into that array where the copy begins.\nThe ::cudaMemcpyAttributes::srcAccessOrder indicates the source access ordering to be observed for copies associated\nwith the attribute. If the source access order is set to ::cudaMemcpySrcAccessOrderStream, then the source will\nbe accessed in stream order. If the source access order is set to ::cudaMemcpySrcAccessOrderDuringApiCall then\nit indicates that access to the source pointer can be out of stream order and all accesses must be complete before the\nAPI call returns. This flag is suited for ephemeral sources (ex., stack variables) when it's known that no prior\noperations in the stream can be accessing the memory and also that the lifetime of the memory is limited to the scope\nthat the source variable was declared in. Specifying this flag allows the driver to optimize the copy and removes the\nneed for the user to synchronize the stream after the API call. If the source access order is set to\n::cudaMemcpySrcAccessOrderAny then it indicates that access to the source pointer can be out of stream order and the\naccesses can happen even after the API call returns. This flag is suited for host pointers allocated\noutside CUDA (ex., via malloc) when it's known that no prior operations in the stream can be accessing the memory.\nSpecifying this flag allows the driver to optimize the copy on certain platforms. Each memcopy operation in `opList`\nmust have a valid srcAccessOrder setting, otherwise this API will return ::cudaErrorInvalidValue.\nThe ::cudaMemcpyAttributes::flags field can be used to specify certain flags for copies. Setting the\n::cudaMemcpyFlagPreferOverlapWithCompute flag indicates that the associated copies should preferably overlap with\nany compute work. Note that this flag is a hint and can be ignored depending on the platform and other parameters of the copy.\n\n# Arguments\n\n* `numOps` -     - Total number of memcpy operations.\n* `opList` -     - Array of size `numOps` containing the actual memcpy operations.\n* `flags` -      - Flags for future use, must be zero now.\n* `hStream` -    - The stream to enqueue the operations in. Must not be default NULL stream.\n\n# Returns\n\n::cudaSuccess\n::cudaErrorInvalidValue\n\\notefnerr \\note_async \\note_memcpy "]
pub unsafe fn cudaMemcpy3DBatchAsync<T: types::CudaAsPtr>(
    numOps: usize,
    mut opList: T,
    flags: u64,
    stream: cudaStream_t,
) -> Result<(), crate::sys::cudaError> {
    let status =
        unsafe { crate::sys::cudaMemcpy3DBatchAsync(numOps, opList.as_mut_ptr() as *mut _, flags as _, stream) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Performs asynchronous memory copy operation with the specified attributes.\nPerforms asynchronous memory copy operation where `dst` and `src` are\nthe destination and source pointers respectively. `size` specifies\nthe number of bytes to copy. `attr` specifies the attributes for the copy and\n`hStream` specifies the stream to enqueue the operation in.\nFor more information regarding the attributes, please refer to ::cudaMemcpyAttributes and it's usage desciption in::cudaMemcpyBatchAsync\n\n# Arguments\n\n* `dst` - - Destination device pointer\n* `src` - - Source device pointer\n* `size` - - Number of bytes to copy\n* `attr` - - Attributes for the copy\n* `hStream` - - Stream to enqueue the operation in\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue\n\\notefnerr \\note_async \\note_memcpy # See also\n\n> [`::cudaMemcpyBatchAsync`]"]
pub unsafe fn cudaMemcpyWithAttributesAsync<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    mut dst: T,
    src: U,
    size: usize,
    mut attr: V,
    stream: cudaStream_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaMemcpyWithAttributesAsync(
            dst.as_mut_ptr() as *mut _,
            src.as_const_ptr() as *const _,
            size,
            attr.as_mut_ptr() as *mut _,
            stream,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Performs 3D asynchronous memory copy with the specified attributes.\nPerforms the copy operation specified in `op.`\n`flags` specifies the flags for the copy and `hStream` specifies the stream to enqueue the operation in.\nFor more information regarding the operation, please refer to ::cudaMemcpy3DBatchOp and it's usage desciption in::cudaMemcpy3DBatchAsync\n\n# Arguments\n\n* `op` - - Operation to perform\n* `flags` - - Flags for the copy, must be zero now.\n* `hStream` - - Stream to enqueue the operation in\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue\n\\notefnerr \\note_async \\note_memcpy # See also\n\n> [`::cudaMemcpy3DBatchAsync`]"]
pub unsafe fn cudaMemcpy3DWithAttributesAsync<T: types::CudaAsPtr>(
    mut op: T,
    flags: u64,
    stream: cudaStream_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaMemcpy3DWithAttributesAsync(op.as_mut_ptr() as *mut _, flags as _, stream) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Copies data between host and device\nCopies a matrix (`height` rows of `width` bytes each) from the memory\narea pointed to by `src` to the memory area pointed to by `dst,` where\n`kind` specifies the direction of the copy, and must be one of\n::cudaMemcpyHostToHost, ::cudaMemcpyHostToDevice, ::cudaMemcpyDeviceToHost,\n::cudaMemcpyDeviceToDevice, or ::cudaMemcpyDefault. Passing\n::cudaMemcpyDefault is recommended, in which case the type of transfer is\ninferred from the pointer values. However, ::cudaMemcpyDefault is only\nallowed on systems that support unified virtual addressing.\n`dpitch` and `spitch` are the widths in memory in bytes of the 2D arrays\npointed to by `dst` and `src,` including any padding added to the end of\neach row. The memory areas may not overlap. `width` must not exceed either\n`dpitch` or `spitch.`\nCalling ::cudaMemcpy2DAsync() with `dst` and `src` pointers that do not\nmatch the direction of the copy results in an undefined behavior.\n::cudaMemcpy2DAsync() returns an error if `dpitch` or `spitch` is greater\nthan the maximum allowed.\n::cudaMemcpy2DAsync() is asynchronous with respect to the host, so\nthe call may return before the copy is complete. The copy can optionally\nbe associated to a stream by passing a non-zero `stream` argument. If\n`kind` is ::cudaMemcpyHostToDevice or ::cudaMemcpyDeviceToHost and\n`stream` is non-zero, the copy may overlap with operations in other\nstreams.\nThe device version of this function only handles device to device copies and\ncannot be given local or shared pointers.\n\n# Arguments\n\n* `dst` -    - Destination memory address\n* `dpitch` - - Pitch of destination memory\n* `src` -    - Source memory address\n* `spitch` - - Pitch of source memory\n* `width` -  - Width of matrix transfer (columns in bytes)\n* `height` - - Height of matrix transfer (rows)\n* `kind` -   - Type of transfer\n* `stream` - - Stream identifier\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorInvalidPitchValue,\n::cudaErrorInvalidMemcpyDirection\n\\notefnerr \\note_async \\note_null_stream \\note_init_rt \\note_callback \\note_memcpy # See also\n\n> [`::cudaMemcpy,`] ::cudaMemcpy2D,\n::cudaMemcpy2DToArray, ::cudaMemcpy2DFromArray,\n::cudaMemcpy2DArrayToArray, ::cudaMemcpyToSymbol,\n::cudaMemcpyFromSymbol, ::cudaMemcpyAsync,\n::cudaMemcpy2DToArrayAsync,\n::cudaMemcpy2DFromArrayAsync,\n::cudaMemcpyToSymbolAsync, ::cudaMemcpyFromSymbolAsync,\n::cuMemcpy2DAsync"]
pub unsafe fn cudaMemcpy2DAsync<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    mut dst: T,
    dpitch: usize,
    src: U,
    spitch: usize,
    width: usize,
    height: usize,
    kind: cudaMemcpyKind,
    stream: cudaStream_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaMemcpy2DAsync(
            dst.as_mut_ptr() as *mut _,
            dpitch,
            src.as_const_ptr() as *const _,
            spitch,
            width,
            height,
            kind,
            stream,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Copies data between host and device\nCopies a matrix (`height` rows of `width` bytes each) from the memory\narea pointed to by `src` to the CUDA array `dst` starting at `hOffset`\nrows and `wOffset` bytes from the upper left corner, where `kind` specifies\nthe direction of the copy, and must be one of ::cudaMemcpyHostToHost,\n::cudaMemcpyHostToDevice, ::cudaMemcpyDeviceToHost,\n::cudaMemcpyDeviceToDevice, or ::cudaMemcpyDefault. Passing\n::cudaMemcpyDefault is recommended, in which case the type of transfer is\ninferred from the pointer values. However, ::cudaMemcpyDefault is only\nallowed on systems that support unified virtual addressing.\n`spitch` is the width in memory in bytes of the 2D array pointed to by\n`src,` including any padding added to the end of each row. `wOffset` +\n`width` must not exceed the width of the CUDA array `dst.` `width` must\nnot exceed `spitch.` ::cudaMemcpy2DToArrayAsync() returns an error if\n`spitch` exceeds the maximum allowed.\n::cudaMemcpy2DToArrayAsync() is asynchronous with respect to the host, so\nthe call may return before the copy is complete. The copy can optionally\nbe associated to a stream by passing a non-zero `stream` argument. If\n`kind` is ::cudaMemcpyHostToDevice or ::cudaMemcpyDeviceToHost and\n`stream` is non-zero, the copy may overlap with operations in other\nstreams.\n\n# Arguments\n\n* `dst` -     - Destination memory address\n* `wOffset` - - Destination starting X offset (columns in bytes)\n* `hOffset` - - Destination starting Y offset (rows)\n* `src` -     - Source memory address\n* `spitch` -  - Pitch of source memory\n* `width` -   - Width of matrix transfer (columns in bytes)\n* `height` -  - Height of matrix transfer (rows)\n* `kind` -    - Type of transfer\n* `stream` -  - Stream identifier\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorInvalidPitchValue,\n::cudaErrorInvalidMemcpyDirection\n\\notefnerr \\note_async \\note_null_stream \\note_init_rt \\note_callback \\note_memcpy # See also\n\n> [`::cudaMemcpy,`] ::cudaMemcpy2D,\n::cudaMemcpy2DToArray, ::cudaMemcpy2DFromArray,\n::cudaMemcpy2DArrayToArray, ::cudaMemcpyToSymbol,\n::cudaMemcpyFromSymbol, ::cudaMemcpyAsync, ::cudaMemcpy2DAsync,\n::cudaMemcpy2DFromArrayAsync,\n::cudaMemcpyToSymbolAsync, ::cudaMemcpyFromSymbolAsync,\n::cuMemcpy2DAsync"]
pub unsafe fn cudaMemcpy2DToArrayAsync<T: types::CudaAsPtr>(
    dst: cudaArray_t,
    wOffset: usize,
    hOffset: usize,
    src: T,
    spitch: usize,
    width: usize,
    height: usize,
    kind: cudaMemcpyKind,
    stream: cudaStream_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaMemcpy2DToArrayAsync(
            dst,
            wOffset,
            hOffset,
            src.as_const_ptr() as *const _,
            spitch,
            width,
            height,
            kind,
            stream,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Copies data between host and device\nCopies a matrix (`height` rows of `width` bytes each) from the CUDA\narray `src` starting at `hOffset` rows and `wOffset` bytes from the\nupper left corner to the memory area pointed to by `dst,`\nwhere `kind` specifies the direction of the copy, and must be one of\n::cudaMemcpyHostToHost, ::cudaMemcpyHostToDevice, ::cudaMemcpyDeviceToHost,\n::cudaMemcpyDeviceToDevice, or ::cudaMemcpyDefault. Passing\n::cudaMemcpyDefault is recommended, in which case the type of transfer is\ninferred from the pointer values. However, ::cudaMemcpyDefault is only\nallowed on systems that support unified virtual addressing.\n`dpitch` is the width in memory in bytes of the 2D\narray pointed to by `dst,` including any padding added to the end of each\nrow. `wOffset` + `width` must not exceed the width of the CUDA array\n`src.` `width` must not exceed `dpitch.` ::cudaMemcpy2DFromArrayAsync()\nreturns an error if `dpitch` exceeds the maximum allowed.\n::cudaMemcpy2DFromArrayAsync() is asynchronous with respect to the host, so\nthe call may return before the copy is complete. The copy can optionally be\nassociated to a stream by passing a non-zero `stream` argument. If `kind`\nis ::cudaMemcpyHostToDevice or ::cudaMemcpyDeviceToHost and `stream` is\nnon-zero, the copy may overlap with operations in other streams.\n\n# Arguments\n\n* `dst` -     - Destination memory address\n* `dpitch` -  - Pitch of destination memory\n* `src` -     - Source memory address\n* `wOffset` - - Source starting X offset (columns in bytes)\n* `hOffset` - - Source starting Y offset (rows)\n* `width` -   - Width of matrix transfer (columns in bytes)\n* `height` -  - Height of matrix transfer (rows)\n* `kind` -    - Type of transfer\n* `stream` -  - Stream identifier\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorInvalidPitchValue,\n::cudaErrorInvalidMemcpyDirection\n\\notefnerr \\note_async \\note_null_stream \\note_init_rt \\note_callback \\note_memcpy # See also\n\n> [`::cudaMemcpy,`] ::cudaMemcpy2D,\n::cudaMemcpy2DToArray, ::cudaMemcpy2DFromArray,\n::cudaMemcpy2DArrayToArray, ::cudaMemcpyToSymbol,\n::cudaMemcpyFromSymbol, ::cudaMemcpyAsync, ::cudaMemcpy2DAsync,\n::cudaMemcpy2DToArrayAsync,\n::cudaMemcpyToSymbolAsync, ::cudaMemcpyFromSymbolAsync,\n::cuMemcpy2DAsync"]
pub unsafe fn cudaMemcpy2DFromArrayAsync<T: types::CudaAsPtr>(
    mut dst: T,
    dpitch: usize,
    src: cudaArray_const_t,
    wOffset: usize,
    hOffset: usize,
    width: usize,
    height: usize,
    kind: cudaMemcpyKind,
    stream: cudaStream_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaMemcpy2DFromArrayAsync(
            dst.as_mut_ptr() as *mut _,
            dpitch,
            src,
            wOffset,
            hOffset,
            width,
            height,
            kind,
            stream,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Copies data to the given symbol on the device\nCopies `count` bytes from the memory area pointed to by `src`\nto the memory area pointed to by `offset` bytes from the start of symbol\n`symbol.` The memory areas may not overlap. `symbol` is a variable that\nresides in global or constant memory space. `kind` can be either\n::cudaMemcpyHostToDevice, ::cudaMemcpyDeviceToDevice, or ::cudaMemcpyDefault.\nPassing ::cudaMemcpyDefault is recommended, in which case the type of transfer\nis inferred from the pointer values. However, ::cudaMemcpyDefault is only\nallowed on systems that support unified virtual addressing.\n::cudaMemcpyToSymbolAsync() is asynchronous with respect to the host, so\nthe call may return before the copy is complete. The copy can optionally\nbe associated to a stream by passing a non-zero `stream` argument. If\n`kind` is ::cudaMemcpyHostToDevice and `stream` is non-zero, the copy\nmay overlap with operations in other streams.\n\n# Arguments\n\n* `symbol` - - Device symbol address\n* `src` -    - Source memory address\n* `count` -  - Size in bytes to copy\n* `offset` - - Offset from start of symbol in bytes\n* `kind` -   - Type of transfer\n* `stream` - - Stream identifier\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorInvalidSymbol,\n::cudaErrorInvalidMemcpyDirection,\n::cudaErrorNoKernelImageForDevice\n\\notefnerr \\note_async \\note_null_stream \\note_string_api_deprecation \\note_init_rt \\note_callback # See also\n\n> [`::cudaMemcpy,`] ::cudaMemcpy2D,\n::cudaMemcpy2DToArray, ::cudaMemcpy2DFromArray,\n::cudaMemcpy2DArrayToArray, ::cudaMemcpyToSymbol,\n::cudaMemcpyFromSymbol, ::cudaMemcpyAsync, ::cudaMemcpy2DAsync,\n::cudaMemcpy2DToArrayAsync,\n::cudaMemcpy2DFromArrayAsync,\n::cudaMemcpyFromSymbolAsync,\n::cuMemcpyAsync,\n::cuMemcpyHtoDAsync,\n::cuMemcpyDtoDAsync"]
pub unsafe fn cudaMemcpyToSymbolAsync<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    symbol: T,
    src: U,
    count: usize,
    offset: usize,
    kind: cudaMemcpyKind,
    stream: cudaStream_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaMemcpyToSymbolAsync(
            symbol.as_const_ptr() as *const _,
            src.as_const_ptr() as *const _,
            count,
            offset,
            kind,
            stream,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Copies data from the given symbol on the device\nCopies `count` bytes from the memory area pointed to by `offset` bytes\nfrom the start of symbol `symbol` to the memory area pointed to by `dst.`\nThe memory areas may not overlap. `symbol` is a variable that resides in\nglobal or constant memory space. `kind` can be either\n::cudaMemcpyDeviceToHost, ::cudaMemcpyDeviceToDevice, or ::cudaMemcpyDefault.\nPassing ::cudaMemcpyDefault is recommended, in which case the type of transfer\nis inferred from the pointer values. However, ::cudaMemcpyDefault is only\nallowed on systems that support unified virtual addressing.\n::cudaMemcpyFromSymbolAsync() is asynchronous with respect to the host, so\nthe call may return before the copy is complete. The copy can optionally be\nassociated to a stream by passing a non-zero `stream` argument. If `kind`\nis ::cudaMemcpyDeviceToHost and `stream` is non-zero, the copy may overlap\nwith operations in other streams.\n\n# Arguments\n\n* `dst` -    - Destination memory address\n* `symbol` - - Device symbol address\n* `count` -  - Size in bytes to copy\n* `offset` - - Offset from start of symbol in bytes\n* `kind` -   - Type of transfer\n* `stream` - - Stream identifier\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorInvalidSymbol,\n::cudaErrorInvalidMemcpyDirection,\n::cudaErrorNoKernelImageForDevice\n\\notefnerr \\note_async \\note_null_stream \\note_string_api_deprecation \\note_init_rt \\note_callback # See also\n\n> [`::cudaMemcpy,`] ::cudaMemcpy2D,\n::cudaMemcpy2DToArray, ::cudaMemcpy2DFromArray,\n::cudaMemcpy2DArrayToArray, ::cudaMemcpyToSymbol,\n::cudaMemcpyFromSymbol, ::cudaMemcpyAsync, ::cudaMemcpy2DAsync,\n::cudaMemcpy2DToArrayAsync,\n::cudaMemcpy2DFromArrayAsync,\n::cudaMemcpyToSymbolAsync,\n::cuMemcpyAsync,\n::cuMemcpyDtoHAsync,\n::cuMemcpyDtoDAsync"]
pub unsafe fn cudaMemcpyFromSymbolAsync<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    mut dst: T,
    symbol: U,
    count: usize,
    offset: usize,
    kind: cudaMemcpyKind,
    stream: cudaStream_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaMemcpyFromSymbolAsync(
            dst.as_mut_ptr() as *mut _,
            symbol.as_const_ptr() as *const _,
            count,
            offset,
            kind,
            stream,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Initializes or sets device memory to a value\nFills the first `count` bytes of the memory area pointed to by `devPtr`\nwith the constant byte value `value.`\nNote that this function is asynchronous with respect to the host unless\n`devPtr` refers to pinned host memory.\n\n# Arguments\n\n* `devPtr` - - Pointer to device memory\n* `value` -  - Value to set for each byte of specified memory\n* `count` -  - Size in bytes to set\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n\\notefnerr \\note_memset \\note_init_rt \\note_callback # See also\n\n> [`::cuMemsetD8,`]\n::cuMemsetD16,\n::cuMemsetD32"]
pub unsafe fn cudaMemset<T: types::CudaAsPtr>(
    mut devPtr: T,
    value: i32,
    count: usize,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaMemset(devPtr.as_mut_ptr() as *mut _, value as _, count) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Initializes or sets device memory to a value\nSets to the specified value `value` a matrix (`height` rows of `width`\nbytes each) pointed to by `dstPtr.` `pitch` is the width in bytes of the\n2D array pointed to by `dstPtr,` including any padding added to the end\nof each row. This function performs fastest when the pitch is one that has\nbeen passed back by ::cudaMallocPitch().\nNote that this function is asynchronous with respect to the host unless\n`devPtr` refers to pinned host memory.\n\n# Arguments\n\n* `devPtr` - - Pointer to 2D device memory\n* `pitch` -  - Pitch in bytes of 2D device memory(Unused if `height` is 1)\n* `value` -  - Value to set for each byte of specified memory\n* `width` -  - Width of matrix set (columns in bytes)\n* `height` - - Height of matrix set (rows)\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n\\notefnerr \\note_memset \\note_init_rt \\note_callback # See also\n\n> [`::cudaMemset,`] ::cudaMemset3D, ::cudaMemsetAsync,\n::cudaMemset2DAsync, ::cudaMemset3DAsync,\n::cuMemsetD2D8,\n::cuMemsetD2D16,\n::cuMemsetD2D32"]
pub unsafe fn cudaMemset2D<T: types::CudaAsPtr>(
    mut devPtr: T,
    pitch: usize,
    value: i32,
    width: usize,
    height: usize,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaMemset2D(devPtr.as_mut_ptr() as *mut _, pitch, value as _, width, height) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Initializes or sets device memory to a value\nInitializes each element of a 3D array to the specified value `value.`\nThe object to initialize is defined by `pitchedDevPtr.` The `pitch` field\nof `pitchedDevPtr` is the width in memory in bytes of the 3D array pointed\nto by `pitchedDevPtr,` including any padding added to the end of each row.\nThe `xsize` field specifies the logical width of each row in bytes, while\nthe `ysize` field specifies the height of each 2D slice in rows.\nThe `pitch` field of `pitchedDevPtr` is ignored when `height` and `depth`\nare both equal to 1.\nThe extents of the initialized region are specified as a `width` in bytes,\na `height` in rows, and a `depth` in slices.\nExtents with `width` greater than or equal to the `xsize` of\n`pitchedDevPtr` may perform significantly faster than extents narrower\nthan the `xsize.` Secondarily, extents with `height` equal to the\n`ysize` of `pitchedDevPtr` will perform faster than when the `height` is\nshorter than the `ysize.`\nThis function performs fastest when the `pitchedDevPtr` has been allocated\nby ::cudaMalloc3D().\nNote that this function is asynchronous with respect to the host unless\n`pitchedDevPtr` refers to pinned host memory.\n\n# Arguments\n\n* `pitchedDevPtr` - - Pointer to pitched device memory\n* `value` -         - Value to set for each byte of specified memory\n* `extent` -        - Size parameters for where to set device memory (`width` field in bytes)\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n\\notefnerr \\note_memset \\note_init_rt \\note_callback # See also\n\n> [`::cudaMemset,`] ::cudaMemset2D,\n::cudaMemsetAsync, ::cudaMemset2DAsync, ::cudaMemset3DAsync,\n::cudaMalloc3D, ::make_cudaPitchedPtr,\n::make_cudaExtent"]
pub unsafe fn cudaMemset3D(
    pitchedDevPtr: cudaPitchedPtr,
    value: i32,
    extent: cudaExtent,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaMemset3D(pitchedDevPtr, value as _, extent) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Initializes or sets device memory to a value\nFills the first `count` bytes of the memory area pointed to by `devPtr`\nwith the constant byte value `value.`\n::cudaMemsetAsync() is asynchronous with respect to the host, so\nthe call may return before the memset is complete. The operation can optionally\nbe associated to a stream by passing a non-zero `stream` argument.\nIf `stream` is non-zero, the operation may overlap with operations in other streams.\nThe device version of this function only handles device to device copies and\ncannot be given local or shared pointers.\n\n# Arguments\n\n* `devPtr` - - Pointer to device memory\n* `value` -  - Value to set for each byte of specified memory\n* `count` -  - Size in bytes to set\n* `stream` - - Stream identifier\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n\\notefnerr \\note_memset \\note_null_stream \\note_init_rt \\note_callback # See also\n\n> [`::cudaMemset,`] ::cudaMemset2D, ::cudaMemset3D,\n::cudaMemset2DAsync, ::cudaMemset3DAsync,\n::cuMemsetD8Async,\n::cuMemsetD16Async,\n::cuMemsetD32Async"]
pub unsafe fn cudaMemsetAsync<T: types::CudaAsPtr>(
    mut devPtr: T,
    value: i32,
    count: usize,
    stream: cudaStream_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaMemsetAsync(devPtr.as_mut_ptr() as *mut _, value as _, count, stream) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Initializes or sets device memory to a value\nSets to the specified value `value` a matrix (`height` rows of `width`\nbytes each) pointed to by `dstPtr.` `pitch` is the width in bytes of the\n2D array pointed to by `dstPtr,` including any padding added to the end\nof each row. This function performs fastest when the pitch is one that has\nbeen passed back by ::cudaMallocPitch().\n::cudaMemset2DAsync() is asynchronous with respect to the host, so\nthe call may return before the memset is complete. The operation can optionally\nbe associated to a stream by passing a non-zero `stream` argument.\nIf `stream` is non-zero, the operation may overlap with operations in other streams.\nThe device version of this function only handles device to device copies and\ncannot be given local or shared pointers.\n\n# Arguments\n\n* `devPtr` - - Pointer to 2D device memory\n* `pitch` -  - Pitch in bytes of 2D device memory(Unused if `height` is 1)\n* `value` -  - Value to set for each byte of specified memory\n* `width` -  - Width of matrix set (columns in bytes)\n* `height` - - Height of matrix set (rows)\n* `stream` - - Stream identifier\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n\\notefnerr \\note_memset \\note_null_stream \\note_init_rt \\note_callback # See also\n\n> [`::cudaMemset,`] ::cudaMemset2D, ::cudaMemset3D,\n::cudaMemsetAsync, ::cudaMemset3DAsync,\n::cuMemsetD2D8Async,\n::cuMemsetD2D16Async,\n::cuMemsetD2D32Async"]
pub unsafe fn cudaMemset2DAsync<T: types::CudaAsPtr>(
    mut devPtr: T,
    pitch: usize,
    value: i32,
    width: usize,
    height: usize,
    stream: cudaStream_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaMemset2DAsync(devPtr.as_mut_ptr() as *mut _, pitch, value as _, width, height, stream)
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Initializes or sets device memory to a value\nInitializes each element of a 3D array to the specified value `value.`\nThe object to initialize is defined by `pitchedDevPtr.` The `pitch` field\nof `pitchedDevPtr` is the width in memory in bytes of the 3D array pointed\nto by `pitchedDevPtr,` including any padding added to the end of each row.\nThe `xsize` field specifies the logical width of each row in bytes, while\nthe `ysize` field specifies the height of each 2D slice in rows.\nThe `pitch` field of `pitchedDevPtr` is ignored when `height` and `depth`\nare both equal to 1.\nThe extents of the initialized region are specified as a `width` in bytes,\na `height` in rows, and a `depth` in slices.\nExtents with `width` greater than or equal to the `xsize` of\n`pitchedDevPtr` may perform significantly faster than extents narrower\nthan the `xsize.` Secondarily, extents with `height` equal to the\n`ysize` of `pitchedDevPtr` will perform faster than when the `height` is\nshorter than the `ysize.`\nThis function performs fastest when the `pitchedDevPtr` has been allocated\nby ::cudaMalloc3D().\n::cudaMemset3DAsync() is asynchronous with respect to the host, so\nthe call may return before the memset is complete. The operation can optionally\nbe associated to a stream by passing a non-zero `stream` argument.\nIf `stream` is non-zero, the operation may overlap with operations in other streams.\nThe device version of this function only handles device to device copies and\ncannot be given local or shared pointers.\n\n# Arguments\n\n* `pitchedDevPtr` - - Pointer to pitched device memory\n* `value` -         - Value to set for each byte of specified memory\n* `extent` -        - Size parameters for where to set device memory (`width` field in bytes)\n* `stream` - - Stream identifier\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n\\notefnerr \\note_memset \\note_null_stream \\note_init_rt \\note_callback # See also\n\n> [`::cudaMemset,`] ::cudaMemset2D, ::cudaMemset3D,\n::cudaMemsetAsync, ::cudaMemset2DAsync,\n::cudaMalloc3D, ::make_cudaPitchedPtr,\n::make_cudaExtent"]
pub unsafe fn cudaMemset3DAsync(
    pitchedDevPtr: cudaPitchedPtr,
    value: i32,
    extent: cudaExtent,
    stream: cudaStream_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaMemset3DAsync(pitchedDevPtr, value as _, extent, stream) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Finds the address associated with a CUDA symbol\nReturns in `*devPtr` the address of symbol `symbol` on the device.\n`symbol` is a variable that resides in global or constant memory space.\nIf `symbol` cannot be found, or if `symbol` is not declared in the\nglobal or constant memory space, `*devPtr` is unchanged and the error\n::cudaErrorInvalidSymbol is returned.\n\n# Arguments\n\n* `devPtr` - - Return device pointer associated with symbol\n* `symbol` - - Device symbol address\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidSymbol,\n::cudaErrorNoKernelImageForDevice\n\\notefnerr \\note_string_api_deprecation \\note_init_rt \\note_callback # See also\n\n> [`\\ref`] ::cudaGetSymbolAddress(void**, const T&) \"cudaGetSymbolAddress (C++ API)\",\n[`::cudaGetSymbolSize(size_t*,`] const void*) \"cudaGetSymbolSize (C API)\",\n::cuModuleGetGlobal"]
pub unsafe fn cudaGetSymbolAddress<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    mut devPtr: T,
    symbol: U,
) -> Result<(), crate::sys::cudaError> {
    let status =
        unsafe { crate::sys::cudaGetSymbolAddress(devPtr.as_mut_ptr() as *mut _, symbol.as_const_ptr() as *const _) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Finds the size of the object associated with a CUDA symbol\nReturns in `*size` the size of symbol `symbol.` `symbol` is a variable that\nresides in global or constant memory space. If `symbol` cannot be found, or\nif `symbol` is not declared in global or constant memory space, `*size` is\nunchanged and the error ::cudaErrorInvalidSymbol is returned.\n\n# Arguments\n\n* `size` -   - Size of object associated with symbol\n* `symbol` - - Device symbol address\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidSymbol,\n::cudaErrorNoKernelImageForDevice\n\\notefnerr \\note_string_api_deprecation \\note_init_rt \\note_callback # See also\n\n> [`\\ref`] ::cudaGetSymbolAddress(void**, const void*) \"cudaGetSymbolAddress (C API)\",\n[`::cudaGetSymbolSize(size_t*,`] const T&) \"cudaGetSymbolSize (C++ API)\",\n::cuModuleGetGlobal"]
pub unsafe fn cudaGetSymbolSize(symbol: *const ::std::os::raw::c_void) -> Result<usize, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaGetSymbolSize(out_0.as_mut_ptr() as *mut _, symbol) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_0.assume_init() as usize) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Prefetches memory to the specified destination location\nPrefetches memory to the specified destination location.  `devPtr` is the\nbase device pointer of the memory to be prefetched and `location` specifies the\ndestination location. `count` specifies the number of bytes to copy. `stream`\nis the stream in which the operation is enqueued. The memory range must refer\nto managed memory allocated via ::cudaMallocManaged or declared via __managed__ variables,\nor it may also refer to memory allocated from a managed memory pool,\nor it may also refer to system-allocated memory on systems with non-zero\ncudaDevAttrPageableMemoryAccess.\nSpecifying ::cudaMemLocationTypeDevice for ::cudaMemLocation::type will prefetch memory to GPU\nspecified by device ordinal ::cudaMemLocation::id which must have non-zero value for the device attribute\n::concurrentManagedAccess. Additionally, `stream` must be associated with a device\nthat has a non-zero value for the device attribute ::concurrentManagedAccess.\nSpecifying ::cudaMemLocationTypeHost as ::cudaMemLocation::type will prefetch data to host memory.\nApplications can request prefetching memory to a specific host NUMA node by specifying\n::cudaMemLocationTypeHostNuma for ::cudaMemLocation::type and a valid host NUMA node id in ::cudaMemLocation::id\nUsers can also request prefetching memory to the host NUMA node closest to the current thread's CPU by specifying\n::cudaMemLocationTypeHostNumaCurrent for ::cudaMemLocation::type. Note when ::cudaMemLocation::type is etiher\n::cudaMemLocationTypeHost OR ::cudaMemLocationTypeHostNumaCurrent, ::cudaMemLocation::id will be ignored.\nThe start address and end address of the memory range will be rounded down and rounded up\nrespectively to be aligned to CPU page size before the prefetch operation is enqueued\nin the stream.\nIf no physical memory has been allocated for this region, then this memory region\nwill be populated and mapped on the destination device. If there's insufficient\nmemory to prefetch the desired region, the Unified Memory driver may evict pages from other\n::cudaMallocManaged allocations to host memory in order to make room. Device memory\nallocated using ::cudaMalloc or ::cudaMallocArray will not be evicted.\nBy default, any mappings to the previous location of the migrated pages are removed and\nmappings for the new location are only setup on the destination location. The exact behavior however\nalso depends on the settings applied to this memory range via ::cuMemAdvise as described\nbelow:\nIf ::cudaMemAdviseSetReadMostly was set on any subset of this memory range,\nthen that subset will create a read-only copy of the pages on destination location.\nIf however the destination location is a host NUMA node, then any pages of that subset\nthat are already in another host NUMA node will be transferred to the destination.\nIf ::cudaMemAdviseSetPreferredLocation was called on any subset of this memory\nrange, then the pages will be migrated to `location` even if `location` is not the\npreferred location of any pages in the memory range.\nIf ::cudaMemAdviseSetAccessedBy was called on any subset of this memory range,\nthen mappings to those pages from all the appropriate processors are updated to\nrefer to the new location if establishing such a mapping is possible. Otherwise,\nthose mappings are cleared.\nNote that this API is not required for functionality and only serves to improve performance\nby allowing the application to migrate data to a suitable location before it is accessed.\nMemory accesses to this range are always coherent and are allowed even when the data is\nactively being migrated.\nNote that this function is asynchronous with respect to the host and all work\non other devices.\n\n# Arguments\n\n* `devPtr` -    - Pointer to be prefetched\n* `count` -     - Size in bytes\n* `location` -  - location to prefetch to\n* `flags` -     - flags for future use, must be zero now.\n* `stream` -    - Stream to enqueue prefetch operation\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorInvalidDevice\n\\notefnerr \\note_async \\note_null_stream \\note_init_rt \\note_callback # See also\n\n> [`::cudaMemcpy,`] ::cudaMemcpyPeer, ::cudaMemcpyAsync,\n::cudaMemcpy3DPeerAsync, ::cudaMemAdvise, ::cuMemPrefetchAsync"]
pub unsafe fn cudaMemPrefetchAsync<T: types::CudaAsPtr>(
    devPtr: T,
    count: usize,
    location: cudaMemLocation,
    flags: u32,
    stream: cudaStream_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaMemPrefetchAsync(devPtr.as_const_ptr() as *const _, count, location, flags as _, stream)
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Performs a batch of memory prefetches asynchronously\nPerforms a batch of memory prefetches. The batch as a whole executes in stream order\nbut operations within a batch are not guaranteed to execute in any specific order.\nAll devices in the system must have a non-zero value for the device attribute\n::cudaDevAttrConcurrentManagedAccess otherwise the API will return an error.\nThe semantics of the individual prefetch operations are as described in ::cudaMemPrefetchAsync.\nPerforms memory prefetch on address ranges specified in `dptrs` and `sizes.`\nBoth arrays must be of the same length as specified by `count.` Each memory range specified\nmust refer to managed memory allocated via ::cudaMallocManaged or declared via\n__managed__ variables or it may also refer to system-allocated memory when all devices\nhave a non-zero value for ::cudaDevAttrPageableMemoryAccess. The prefetch location for\nevery operation in the batch is specified in the `prefetchLocs` array. Each entry in\nthis array can apply to more than one operation. This can be done by specifying in the\n`prefetchLocIdxs` array, the index of the first prefetch operation that the corresponding entry\nin the `prefetchLocs` array applies to. Both `prefetchLocs` and `prefetchLocIdxs` must be of\nthe same length as specified by `numPrefetchLocs.` For example, if a batch has 10 prefetches listed\nin dptrs/sizes, the first 4 of which are to be prefetched to one location and the remaining 6 are to be prefetched\nto another, then `numPrefetchLocs` will be 2, `prefetchLocIdxs` will be {0, 4} and `prefetchLocs`\nwill contain the two locations. Note the first entry in `prefetchLocIdxs` must always be 0.\nAlso, each entry must be greater than the previous entry and the last entry should be less than `count.`\nFurthermore, `numPrefetchLocs` must be lesser than or equal to `count.`\n\n# Arguments\n\n* `dptrs` -           - Array of pointers to be prefetched\n* `sizes` -           - Array of sizes for memory prefetch operations.\n* `count` -           - Size of `dptrs` and `sizes` arrays.\n* `prefetchLocs` -    - Array of locations to prefetch to.\n* `prefetchLocIdxs` - - Array of indices to specify which operands each entry in the `prefetchLocs` array applies to.\nThe locations specified in prefetchLocs[k] will be applied to copies starting from  prefetchLocIdxs[k]\nthrough  prefetchLocIdxs[k+1] - 1. Also prefetchLocs[numPrefetchLocs - 1] will apply to prefetches starting from\nprefetchLocIdxs[numPrefetchLocs - 1] through count - 1.\n* `numPrefetchLocs` - - Size of `prefetchLocs` and `prefetchLocIdxs` arrays.\n* `flags` -           - Flags reserved for future use. Must be zero.\n* `hStream` -         - The stream to enqueue the operations in. Must not be legacy NULL stream.\n"]
pub unsafe fn cudaMemPrefetchBatchAsync<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    mut dptrs: T,
    mut sizes: U,
    count: usize,
    mut prefetchLocs: V,
    mut prefetchLocIdxs: W,
    numPrefetchLocs: usize,
    flags: u64,
    stream: cudaStream_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaMemPrefetchBatchAsync(
            dptrs.as_mut_ptr() as *mut _,
            sizes.as_mut_ptr() as *mut _,
            count,
            prefetchLocs.as_mut_ptr() as *mut _,
            prefetchLocIdxs.as_mut_ptr() as *mut _,
            numPrefetchLocs,
            flags as _,
            stream,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Performs a batch of memory discards asynchronously\nPerforms a batch of memory discards. The batch as a whole executes in stream order\nbut operations within a batch are not guaranteed to execute in any specific order.\nAll devices in the system must have a non-zero value for the device attribute\n::cudaDevAttrConcurrentManagedAccess otherwise the API will return an error.\nDiscarding a memory range informs the driver that the contents of that range are no longer useful.\nDiscarding memory ranges allows the driver to optimize certain data migrations and can also help\nreduce memory pressure. This operation can be undone on any part of the range by either writing to it\nor prefetching it via ::cudaMemPrefetchAsync or ::cudaMemPrefetchBatchAsync. Reading from a discarded range,\nwithout a subsequent write or prefetch to that part of the range, will return an indeterminate value.\nNote that any reads, writes or prefetches to any part of the memory range that occur simultaneously with\nthe discard operation result in undefined behavior.\nPerforms memory discard on address ranges specified in `dptrs` and `sizes.`\nBoth arrays must be of the same length as specified by `count.` Each memory range\nspecified must refer to managed memory allocated via ::cudaMallocManaged or declared\nvia __managed__ variables or it may also refer to system-allocated memory when all devices\nhave a non-zero value for ::cudaDevAttrPageableMemoryAccess.\n\n# Arguments\n\n* `dptrs` -   - Array of pointers to be discarded\n* `sizes` -   - Array of sizes for memory discard operations.\n* `count` -   - Size of `dptrs` and `sizes` arrays.\n* `flags` -   - Flags reserved for future use. Must be zero.\n* `hStream` - - The stream to enqueue the operations in. Must not be legacy NULL stream.\n"]
pub unsafe fn cudaMemDiscardBatchAsync<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    mut dptrs: T,
    mut sizes: U,
    count: usize,
    flags: u64,
    stream: cudaStream_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaMemDiscardBatchAsync(
            dptrs.as_mut_ptr() as *mut _,
            sizes.as_mut_ptr() as *mut _,
            count,
            flags as _,
            stream,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Performs a batch of memory discards and prefetches asynchronously\nPerforms a batch of memory discards followed by prefetches. The batch as a whole executes\nin stream order but operations within a batch are not guaranteed to execute in any specific order.\nAll devices in the system must have a non-zero value for the device attribute\n::cudaDevAttrConcurrentManagedAccess otherwise the API will return an error.\nCalling ::cudaMemDiscardAndPrefetchBatchAsync is semantically equivalent to calling\n::cudaMemDiscardBatchAsync followed by ::cudaMemPrefetchBatchAsync, but is more optimal.\nFor more details on what discarding and prefetching imply, please refer to ::cudaMemDiscardBatchAsync\nand ::cudaMemPrefetchBatchAsync respectively. Note that any reads, writes or prefetches to any part\nof the memory range that occur simultaneously with this combined discard+prefetch operation\nresult in undefined behavior.\nPerforms memory discard and prefetch on address ranges specified in `dptrs` and `sizes.`\nBoth arrays must be of the same length as specified by `count.` Each memory range specified\nmust refer to managed memory allocated via ::cudaMallocManaged or declared via\n__managed__ variables or it may also refer to system-allocated memory when all devices\nhave a non-zero value for ::cudaDevAttrPageableMemoryAccess. Every operation in the batch\nhas to be associated with a valid location to prefetch the address range to and specified in\nthe `prefetchLocs` array. Each entry in this array can apply to more than one operation.\nThis can be done by specifying in the `prefetchLocIdxs` array, the index of the first\noperation that the corresponding entry in the `prefetchLocs` array applies to.\nBoth `prefetchLocs` and `prefetchLocIdxs` must be of the same length as specified by\n`numPrefetchLocs.` For example, if a batch has 10 operations listed in dptrs/sizes,\nthe first 6 of which are to be prefetched to one location and the remaining 4 are to be\nprefetched to another, then `numPrefetchLocs` will be 2, `prefetchLocIdxs` will be {0, 6}\nand `prefetchLocs` will contain the two set of locations. Note the first entry in\n`prefetchLocIdxs` must always be 0. Also, each entry must be greater than the previous\nentry and the last entry should be less than `count.` Furthermore, `numPrefetchLocs`\nmust be lesser than or equal to `count.`\n\n# Arguments\n\n* `dptrs` -           - Array of pointers to be discarded\n* `sizes` -           - Array of sizes for memory discard operations.\n* `count` -           - Size of `dptrs` and `sizes` arrays.\n* `prefetchLocs` -    - Array of locations to prefetch to.\n* `prefetchLocIdxs` - - Array of indices to specify which operands each entry in the `prefetchLocs` array applies to.\nThe locations specified in prefetchLocs[k] will be applied to operations starting from  prefetchLocIdxs[k]\nthrough prefetchLocIdxs[k+1] - 1. Also prefetchLocs[numPrefetchLocs - 1] will apply to copies starting from\nprefetchLocIdxs[numPrefetchLocs - 1] through count - 1.\n* `numPrefetchLocs` - - Size of `prefetchLocs` and `prefetchLocIdxs` arrays.\n* `flags` -           - Flags reserved for future use. Must be zero.\n* `hStream` -         - The stream to enqueue the operations in. Must not be legacy NULL stream.\n"]
pub unsafe fn cudaMemDiscardAndPrefetchBatchAsync<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    mut dptrs: T,
    mut sizes: U,
    count: usize,
    mut prefetchLocs: V,
    mut prefetchLocIdxs: W,
    numPrefetchLocs: usize,
    flags: u64,
    stream: cudaStream_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaMemDiscardAndPrefetchBatchAsync(
            dptrs.as_mut_ptr() as *mut _,
            sizes.as_mut_ptr() as *mut _,
            count,
            prefetchLocs.as_mut_ptr() as *mut _,
            prefetchLocIdxs.as_mut_ptr() as *mut _,
            numPrefetchLocs,
            flags as _,
            stream,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Advise about the usage of a given memory range\nAdvise the Unified Memory subsystem about the usage pattern for the memory range\nstarting at `devPtr` with a size of `count` bytes. The start address and end address of the memory\nrange will be rounded down and rounded up respectively to be aligned to CPU page size before the\nadvice is applied. The memory range must refer to managed memory allocated via ::cudaMallocManaged\nor declared via __managed__ variables. The memory range could also refer to system-allocated pageable\nmemory provided it represents a valid, host-accessible region of memory and all additional constraints\nimposed by `advice` as outlined below are also satisfied. Specifying an invalid system-allocated pageable\nmemory range results in an error being returned.\nThe `advice` parameter can take the following values:\n- ::cudaMemAdviseSetReadMostly: This implies that the data is mostly going to be read\nfrom and only occasionally written to. Any read accesses from any processor to this region will create a\nread-only copy of at least the accessed pages in that processor's memory. Additionally, if ::cudaMemPrefetchAsync\nor ::cudaMemPrefetchAsync is called on this region, it will create a read-only copy of the data on the destination processor.\nIf the target location for ::cudaMemPrefetchAsync is a host NUMA node and a read-only copy already exists on\nanother host NUMA node, that copy will be migrated to the targeted host NUMA node.\nIf any processor writes to this region, all copies of the corresponding page will be invalidated\nexcept for the one where the write occurred. If the writing processor is the CPU and the preferred location of\nthe page is a host NUMA node, then the page will also be migrated to that host NUMA node. The `location` argument is ignored for this advice.\nNote that for a page to be read-duplicated, the accessing processor must either be the CPU or a GPU\nthat has a non-zero value for the device attribute ::cudaDevAttrConcurrentManagedAccess.\nAlso, if a context is created on a device that does not have the device attribute\n::cudaDevAttrConcurrentManagedAccess set, then read-duplication will not occur until\nall such contexts are destroyed.\nIf the memory region refers to valid system-allocated pageable memory, then the accessing device must\nhave a non-zero value for the device attribute ::cudaDevAttrPageableMemoryAccess for a read-only\ncopy to be created on that device. Note however that if the accessing device also has a non-zero value for the\ndevice attribute ::cudaDevAttrPageableMemoryAccessUsesHostPageTables, then setting this advice\nwill not create a read-only copy when that device accesses this memory region.\n- ::cudaMemAdviceUnsetReadMostly:  Undoes the effect of ::cudaMemAdviseSetReadMostly and also prevents the\nUnified Memory driver from attempting heuristic read-duplication on the memory range. Any read-duplicated\ncopies of the data will be collapsed into a single copy. The location for the collapsed\ncopy will be the preferred location if the page has a preferred location and one of the read-duplicated\ncopies was resident at that location. Otherwise, the location chosen is arbitrary.\nNote: The `location` argument is ignored for this advice.\n- ::cudaMemAdviseSetPreferredLocation: This advice sets the preferred location for the\ndata to be the memory belonging to `location.` When ::cudaMemLocation::type is ::cudaMemLocationTypeHost,\n::cudaMemLocation::id is ignored and the preferred location is set to be host memory. To set the preferred location\nto a specific host NUMA node, applications must set ::cudaMemLocation::type to ::cudaMemLocationTypeHostNuma and\n::cudaMemLocation::id must specify the NUMA ID of the host NUMA node. If ::cudaMemLocation::type is set to ::cudaMemLocationTypeHostNumaCurrent,\n::cudaMemLocation::id will be ignored and the host NUMA node closest to the calling thread's CPU will be used as the preferred location.\nIf ::cudaMemLocation::type is a ::cudaMemLocationTypeDevice, then ::cudaMemLocation::id must be a valid device ordinal\nand the device must have a non-zero value for the device attribute ::cudaDevAttrConcurrentManagedAccess.\nSetting the preferred location does not cause data to migrate to that location immediately. Instead, it guides the migration policy\nwhen a fault occurs on that memory region. If the data is already in its preferred location and the\nfaulting processor can establish a mapping without requiring the data to be migrated, then\ndata migration will be avoided. On the other hand, if the data is not in its preferred location\nor if a direct mapping cannot be established, then it will be migrated to the processor accessing\nit. It is important to note that setting the preferred location does not prevent data prefetching\ndone using ::cudaMemPrefetchAsync.\nHaving a preferred location can override the page thrash detection and resolution logic in the Unified\nMemory driver. Normally, if a page is detected to be constantly thrashing between for example host and device\nmemory, the page may eventually be pinned to host memory by the Unified Memory driver. But\nif the preferred location is set as device memory, then the page will continue to thrash indefinitely.\nIf ::cudaMemAdviseSetReadMostly is also set on this memory region or any subset of it, then the\npolicies associated with that advice will override the policies of this advice, unless read accesses from\n`location` will not result in a read-only copy being created on that procesor as outlined in description for\nthe advice ::cudaMemAdviseSetReadMostly.\nIf the memory region refers to valid system-allocated pageable memory, and ::cudaMemLocation::type is ::cudaMemLocationTypeDevice\nthen ::cudaMemLocation::id must be a valid device that has a non-zero alue for the device attribute ::cudaDevAttrPageableMemoryAccess.\n- ::cudaMemAdviseUnsetPreferredLocation: Undoes the effect of ::cudaMemAdviseSetPreferredLocation\nand changes the preferred location to none. The `location` argument is ignored for this advice.\n- ::cudaMemAdviseSetAccessedBy: This advice implies that the data will be accessed by processor `location.`\nThe ::cudaMemLocation::type must be either ::cudaMemLocationTypeDevice with ::cudaMemLocation::id representing a valid device\nordinal or ::cudaMemLocationTypeHost and ::cudaMemLocation::id will be ignored. All other location types are invalid.\nIf ::cudaMemLocation::id is a GPU, then the device attribute ::cudaDevAttrConcurrentManagedAccess must be non-zero.\nThis advice does not cause data migration and has no impact on the location of the data per se. Instead,\nit causes the data to always be mapped in the specified processor's page tables, as long as the\nlocation of the data permits a mapping to be established. If the data gets migrated for any reason,\nthe mappings are updated accordingly.\nThis advice is recommended in scenarios where data locality is not important, but avoiding faults is.\nConsider for example a system containing multiple GPUs with peer-to-peer access enabled, where the\ndata located on one GPU is occasionally accessed by peer GPUs. In such scenarios, migrating data\nover to the other GPUs is not as important because the accesses are infrequent and the overhead of\nmigration may be too high. But preventing faults can still help improve performance, and so having\na mapping set up in advance is useful. Note that on CPU access of this data, the data may be migrated\nto host memory because the CPU typically cannot access device memory directly. Any GPU that had the\n::cudaMemAdviseSetAccessedBy flag set for this data will now have its mapping updated to point to the\npage in host memory.\nIf ::cudaMemAdviseSetReadMostly is also set on this memory region or any subset of it, then the\npolicies associated with that advice will override the policies of this advice. Additionally, if the\npreferred location of this memory region or any subset of it is also `location,` then the policies\nassociated with ::CU_MEM_ADVISE_SET_PREFERRED_LOCATION will override the policies of this advice.\nIf the memory region refers to valid system-allocated pageable memory, and ::cudaMemLocation::type is ::cudaMemLocationTypeDevice\nthen device in ::cudaMemLocation::id must have a non-zero value for the device attribute ::cudaDevAttrPageableMemoryAccess.\nAdditionally, if ::cudaMemLocation::id has a non-zero value for the device attribute ::cudaDevAttrPageableMemoryAccessUsesHostPageTables,\nthen this call has no effect.\n- ::CU_MEM_ADVISE_UNSET_ACCESSED_BY: Undoes the effect of ::cudaMemAdviseSetAccessedBy. Any mappings to\nthe data from `location` may be removed at any time causing accesses to result in non-fatal page faults.\nIf the memory region refers to valid system-allocated pageable memory, and ::cudaMemLocation::type is ::cudaMemLocationTypeDevice\nthen device in ::cudaMemLocation::id must have a non-zero value for the device attribute ::cudaDevAttrPageableMemoryAccess.\nAdditionally, if ::cudaMemLocation::id has a non-zero value for the device attribute ::cudaDevAttrPageableMemoryAccessUsesHostPageTables,\nthen this call has no effect.\n\n# Arguments\n\n* `devPtr` -   - Pointer to memory to set the advice for\n* `count` -    - Size in bytes of the memory range\n* `advice` -   - Advice to be applied for the specified memory range\n* `location` - - location to apply the advice for\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorInvalidDevice\n\\notefnerr \\note_async \\note_null_stream \\note_init_rt \\note_callback # See also\n\n> [`::cudaMemcpy,`] ::cudaMemcpyPeer, ::cudaMemcpyAsync,\n::cudaMemcpy3DPeerAsync, ::cudaMemPrefetchAsync,\n::cuMemAdvise"]
pub unsafe fn cudaMemAdvise<T: types::CudaAsPtr>(
    devPtr: T,
    count: usize,
    advice: cudaMemoryAdvise,
    location: cudaMemLocation,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaMemAdvise(devPtr.as_const_ptr() as *const _, count, advice, location) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Query an attribute of a given memory range\nQuery an attribute about the memory range starting at `devPtr` with a size of `count` bytes. The\nmemory range must refer to managed memory allocated via ::cudaMallocManaged or declared via\n__managed__ variables.\nThe `attribute` parameter can take the following values:\n- ::cudaMemRangeAttributeReadMostly: If this attribute is specified, `data` will be interpreted\nas a 32-bit integer, and `dataSize` must be 4. The result returned will be 1 if all pages in the given\nmemory range have read-duplication enabled, or 0 otherwise.\n- ::cudaMemRangeAttributePreferredLocation: If this attribute is specified, `data` will be\ninterpreted as a 32-bit integer, and `dataSize` must be 4. The result returned will be a GPU device\nid if all pages in the memory range have that GPU as their preferred location, or it will be cudaCpuDeviceId\nif all pages in the memory range have the CPU as their preferred location, or it will be cudaInvalidDeviceId\nif either all the pages don't have the same preferred location or some of the pages don't have a\npreferred location at all. Note that the actual location of the pages in the memory range at the time of\nthe query may be different from the preferred location.\n- ::cudaMemRangeAttributeAccessedBy: If this attribute is specified, `data` will be interpreted\nas an array of 32-bit integers, and `dataSize` must be a non-zero multiple of 4. The result returned\nwill be a list of device ids that had ::cudaMemAdviceSetAccessedBy set for that entire memory range.\nIf any device does not have that advice set for the entire memory range, that device will not be included.\nIf `data` is larger than the number of devices that have that advice set for that memory range,\ncudaInvalidDeviceId will be returned in all the extra space provided. For ex., if `dataSize` is 12\n(i.e. `data` has 3 elements) and only device 0 has the advice set, then the result returned will be\n{ 0, cudaInvalidDeviceId, cudaInvalidDeviceId }. If `data` is smaller than the number of devices that have\nthat advice set, then only as many devices will be returned as can fit in the array. There is no\nguarantee on which specific devices will be returned, however.\n- ::cudaMemRangeAttributeLastPrefetchLocation: If this attribute is specified, `data` will be\ninterpreted as a 32-bit integer, and `dataSize` must be 4. The result returned will be the last location\nto which all pages in the memory range were prefetched explicitly via ::cudaMemPrefetchAsync. This will either be\na GPU id or cudaCpuDeviceId depending on whether the last location for prefetch was a GPU or the CPU\nrespectively. If any page in the memory range was never explicitly prefetched or if all pages were not\nprefetched to the same location, cudaInvalidDeviceId will be returned. Note that this simply returns the\nlast location that the applicaton requested to prefetch the memory range to. It gives no indication as to\nwhether the prefetch operation to that location has completed or even begun.\n- ::cudaMemRangeAttributePreferredLocationType: If this attribute is specified, `data` will be\ninterpreted as a ::cudaMemLocationType, and `dataSize` must be sizeof(cudaMemLocationType). The ::cudaMemLocationType returned will be\n::cudaMemLocationTypeDevice if all pages in the memory range have the same GPU as their preferred location, or ::cudaMemLocationType\nwill be ::cudaMemLocationTypeHost if all pages in the memory range have the CPU as their preferred location, or or it will be ::cudaMemLocationTypeHostNuma\nif all the pages in the memory range have the same host NUMA node ID as their preferred location or it will be ::cudaMemLocationTypeInvalid\nif either all the pages don't have the same preferred location or some of the pages don't have a preferred location at all.\nNote that the actual location type of the pages in the memory range at the time of the query may be different from the preferred location type.\n- ::cudaMemRangeAttributePreferredLocationId: If this attribute is specified, `data` will be\ninterpreted as a 32-bit integer, and `dataSize` must be 4. If the ::cudaMemRangeAttributePreferredLocationType query for the same address range\nreturns ::cudaMemLocationTypeDevice, it will be a valid device ordinal or if it returns ::cudaMemLocationTypeHostNuma, it will be a valid host NUMA node ID\nor if it returns any other location type, the id should be ignored.\n- ::cudaMemRangeAttributeLastPrefetchLocationType: If this attribute is specified, `data` will be\ninterpreted as a ::cudaMemLocationType, and `dataSize` must be sizeof(cudaMemLocationType). The result returned will be the last location type\nto which all pages in the memory range were prefetched explicitly via ::cuMemPrefetchAsync. The ::cudaMemLocationType returned\nwill be ::cudaMemLocationTypeDevice if the last prefetch location was the GPU or ::cudaMemLocationTypeHost if it was the CPU or ::cudaMemLocationTypeHostNuma if\nthe last prefetch location was a specific host NUMA node. If any page in the memory range was never explicitly prefetched or if all pages were not\nprefetched to the same location, ::CUmemLocationType will be ::cudaMemLocationTypeInvalid.\nNote that this simply returns the last location type that the application requested to prefetch the memory range to. It gives no indication as to\nwhether the prefetch operation to that location has completed or even begun.\n- ::cudaMemRangeAttributeLastPrefetchLocationId: If this attribute is specified, `data` will be\ninterpreted as a 32-bit integer, and `dataSize` must be 4. If the ::cudaMemRangeAttributeLastPrefetchLocationType query for the same address range\nreturns ::cudaMemLocationTypeDevice, it will be a valid device ordinal or if it returns ::cudaMemLocationTypeHostNuma, it will be a valid host NUMA node ID\nor if it returns any other location type, the id should be ignored.\n\n# Arguments\n\n* `data` -      - A pointers to a memory location where the result\nof each attribute query will be written to.\n* `dataSize` -  - Array containing the size of data\n* `attribute` - - The attribute to query\n* `devPtr` -    - Start of the range to query\n* `count` -     - Size of the range to query\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue\n\\notefnerr \\note_async \\note_null_stream \\note_init_rt \\note_callback # See also\n\n> [`::cudaMemRangeGetAttributes,`] ::cudaMemPrefetchAsync,\n::cudaMemAdvise,\n::cuMemRangeGetAttribute"]
pub unsafe fn cudaMemRangeGetAttribute<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    mut data: T,
    dataSize: usize,
    attribute: cudaMemRangeAttribute,
    devPtr: U,
    count: usize,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaMemRangeGetAttribute(
            data.as_mut_ptr() as *mut _,
            dataSize,
            attribute,
            devPtr.as_const_ptr() as *const _,
            count,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Query attributes of a given memory range.\nQuery attributes of the memory range starting at `devPtr` with a size of `count` bytes. The\nmemory range must refer to managed memory allocated via ::cudaMallocManaged or declared via\n__managed__ variables. The `attributes` array will be interpreted to have `numAttributes`\nentries. The `dataSizes` array will also be interpreted to have `numAttributes` entries.\nThe results of the query will be stored in `data.`\nThe list of supported attributes are given below. Please refer to ::cudaMemRangeGetAttribute for\nattribute descriptions and restrictions.\n- ::cudaMemRangeAttributeReadMostly\n- ::cudaMemRangeAttributePreferredLocation\n- ::cudaMemRangeAttributeAccessedBy\n- ::cudaMemRangeAttributeLastPrefetchLocation\n- :: cudaMemRangeAttributePreferredLocationType\n- :: cudaMemRangeAttributePreferredLocationId\n- :: cudaMemRangeAttributeLastPrefetchLocationType\n- :: cudaMemRangeAttributeLastPrefetchLocationId\n\n# Arguments\n\n* `data` -          - A two-dimensional array containing pointers to memory\nlocations where the result of each attribute query will be written to.\n* `dataSizes` -     - Array containing the sizes of each result\n* `attributes` -    - An array of attributes to query\n(numAttributes and the number of attributes in this array should match)\n* `numAttributes` - - Number of attributes to query\n* `devPtr` -        - Start of the range to query\n* `count` -         - Size of the range to query\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue\n\\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaMemRangeGetAttribute,`] ::cudaMemAdvise,\n::cudaMemPrefetchAsync,\n::cuMemRangeGetAttributes"]
pub unsafe fn cudaMemRangeGetAttributes(
    data: *mut *mut ::std::os::raw::c_void,
    numAttributes: usize,
    devPtr: *const ::std::os::raw::c_void,
    count: usize,
) -> Result<(usize, cudaMemRangeAttribute), crate::sys::cudaError> {
    let mut out_1: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let mut out_2: std::mem::MaybeUninit<cudaMemRangeAttribute> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudaMemRangeGetAttributes(
            data,
            out_1.as_mut_ptr() as *mut _,
            out_2.as_mut_ptr() as *mut _,
            numAttributes,
            devPtr,
            count,
        )
    };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe {
            Ok((
                out_1.assume_init() as usize,
                out_2.assume_init() as cudaMemRangeAttribute,
            ))
        }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Copies data between host and device\n> **Deprecated** Copies `count` bytes from the memory area pointed to by `src` to the\nCUDA array `dst` starting at `hOffset` rows and `wOffset` bytes from\nthe upper left corner, where `kind` specifies the direction\nof the copy, and must be one of ::cudaMemcpyHostToHost,\n::cudaMemcpyHostToDevice, ::cudaMemcpyDeviceToHost,\n::cudaMemcpyDeviceToDevice, or ::cudaMemcpyDefault. Passing\n::cudaMemcpyDefault is recommended, in which case the type of transfer is\ninferred from the pointer values. However, ::cudaMemcpyDefault is only\nallowed on systems that support unified virtual addressing.\n\n# Arguments\n\n* `dst` -     - Destination memory address\n* `wOffset` - - Destination starting X offset (columns in bytes)\n* `hOffset` - - Destination starting Y offset (rows)\n* `src` -     - Source memory address\n* `count` -   - Size in bytes to copy\n* `kind` -    - Type of transfer\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorInvalidMemcpyDirection\n\\notefnerr \\note_sync \\note_init_rt \\note_callback # See also\n\n> [`::cudaMemcpy,`] ::cudaMemcpy2D,\n::cudaMemcpy2DToArray, ::cudaMemcpyFromArray, ::cudaMemcpy2DFromArray,\n::cudaMemcpyArrayToArray, ::cudaMemcpy2DArrayToArray, ::cudaMemcpyToSymbol,\n::cudaMemcpyFromSymbol, ::cudaMemcpyAsync, ::cudaMemcpy2DAsync,\n::cudaMemcpyToArrayAsync, ::cudaMemcpy2DToArrayAsync,\n::cudaMemcpyFromArrayAsync, ::cudaMemcpy2DFromArrayAsync,\n::cudaMemcpyToSymbolAsync, ::cudaMemcpyFromSymbolAsync,\n::cuMemcpyHtoA,\n::cuMemcpyDtoA"]
pub unsafe fn cudaMemcpyToArray<T: types::CudaAsPtr>(
    dst: cudaArray_t,
    wOffset: usize,
    hOffset: usize,
    src: T,
    count: usize,
    kind: cudaMemcpyKind,
) -> Result<(), crate::sys::cudaError> {
    let status =
        unsafe { crate::sys::cudaMemcpyToArray(dst, wOffset, hOffset, src.as_const_ptr() as *const _, count, kind) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Copies data between host and device\n> **Deprecated** Copies `count` bytes from the CUDA array `src` starting at `hOffset` rows\nand `wOffset` bytes from the upper left corner to the memory area pointed to\nby `dst,` where `kind` specifies the direction of the copy, and must be one of\n::cudaMemcpyHostToHost, ::cudaMemcpyHostToDevice, ::cudaMemcpyDeviceToHost,\n::cudaMemcpyDeviceToDevice, or ::cudaMemcpyDefault. Passing\n::cudaMemcpyDefault is recommended, in which case the type of transfer is\ninferred from the pointer values. However, ::cudaMemcpyDefault is only\nallowed on systems that support unified virtual addressing.\n\n# Arguments\n\n* `dst` -     - Destination memory address\n* `src` -     - Source memory address\n* `wOffset` - - Source starting X offset (columns in bytes)\n* `hOffset` - - Source starting Y offset (rows)\n* `count` -   - Size in bytes to copy\n* `kind` -    - Type of transfer\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorInvalidMemcpyDirection\n\\notefnerr \\note_sync \\note_init_rt \\note_callback # See also\n\n> [`::cudaMemcpy,`] ::cudaMemcpy2D, ::cudaMemcpyToArray,\n::cudaMemcpy2DToArray, ::cudaMemcpy2DFromArray,\n::cudaMemcpyArrayToArray, ::cudaMemcpy2DArrayToArray, ::cudaMemcpyToSymbol,\n::cudaMemcpyFromSymbol, ::cudaMemcpyAsync, ::cudaMemcpy2DAsync,\n::cudaMemcpyToArrayAsync, ::cudaMemcpy2DToArrayAsync,\n::cudaMemcpyFromArrayAsync, ::cudaMemcpy2DFromArrayAsync,\n::cudaMemcpyToSymbolAsync, ::cudaMemcpyFromSymbolAsync,\n::cuMemcpyAtoH,\n::cuMemcpyAtoD"]
pub unsafe fn cudaMemcpyFromArray<T: types::CudaAsPtr>(
    mut dst: T,
    src: cudaArray_const_t,
    wOffset: usize,
    hOffset: usize,
    count: usize,
    kind: cudaMemcpyKind,
) -> Result<(), crate::sys::cudaError> {
    let status =
        unsafe { crate::sys::cudaMemcpyFromArray(dst.as_mut_ptr() as *mut _, src, wOffset, hOffset, count, kind) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Copies data between host and device\n> **Deprecated** Copies `count` bytes from the CUDA array `src` starting at `hOffsetSrc`\nrows and `wOffsetSrc` bytes from the upper left corner to the CUDA array\n`dst` starting at `hOffsetDst` rows and `wOffsetDst` bytes from the upper\nleft corner, where `kind` specifies the direction of the copy, and must be one of\n::cudaMemcpyHostToHost, ::cudaMemcpyHostToDevice, ::cudaMemcpyDeviceToHost,\n::cudaMemcpyDeviceToDevice, or ::cudaMemcpyDefault. Passing\n::cudaMemcpyDefault is recommended, in which case the type of transfer is\ninferred from the pointer values. However, ::cudaMemcpyDefault is only\nallowed on systems that support unified virtual addressing.\n\n# Arguments\n\n* `dst` -        - Destination memory address\n* `wOffsetDst` - - Destination starting X offset (columns in bytes)\n* `hOffsetDst` - - Destination starting Y offset (rows)\n* `src` -        - Source memory address\n* `wOffsetSrc` - - Source starting X offset (columns in bytes)\n* `hOffsetSrc` - - Source starting Y offset (rows)\n* `count` -      - Size in bytes to copy\n* `kind` -       - Type of transfer\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorInvalidMemcpyDirection\n\\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaMemcpy,`] ::cudaMemcpy2D, ::cudaMemcpyToArray,\n::cudaMemcpy2DToArray, ::cudaMemcpyFromArray, ::cudaMemcpy2DFromArray,\n::cudaMemcpy2DArrayToArray, ::cudaMemcpyToSymbol,\n::cudaMemcpyFromSymbol, ::cudaMemcpyAsync, ::cudaMemcpy2DAsync,\n::cudaMemcpyToArrayAsync, ::cudaMemcpy2DToArrayAsync,\n::cudaMemcpyFromArrayAsync, ::cudaMemcpy2DFromArrayAsync,\n::cudaMemcpyToSymbolAsync, ::cudaMemcpyFromSymbolAsync,\n::cuMemcpyAtoA"]
pub unsafe fn cudaMemcpyArrayToArray(
    dst: cudaArray_t,
    wOffsetDst: usize,
    hOffsetDst: usize,
    src: cudaArray_const_t,
    wOffsetSrc: usize,
    hOffsetSrc: usize,
    count: usize,
    kind: cudaMemcpyKind,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaMemcpyArrayToArray(dst, wOffsetDst, hOffsetDst, src, wOffsetSrc, hOffsetSrc, count, kind)
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Copies data between host and device\n> **Deprecated** Copies `count` bytes from the memory area pointed to by `src` to the\nCUDA array `dst` starting at `hOffset` rows and `wOffset` bytes from\nthe upper left corner, where `kind` specifies the\ndirection of the copy, and must be one of ::cudaMemcpyHostToHost,\n::cudaMemcpyHostToDevice, ::cudaMemcpyDeviceToHost,\n::cudaMemcpyDeviceToDevice, or ::cudaMemcpyDefault. Passing\n::cudaMemcpyDefault is recommended, in which case the type of transfer is\ninferred from the pointer values. However, ::cudaMemcpyDefault is only\nallowed on systems that support unified virtual addressing.\n::cudaMemcpyToArrayAsync() is asynchronous with respect to the host, so\nthe call may return before the copy is complete. The copy can optionally\nbe associated to a stream by passing a non-zero `stream` argument. If `kind` is ::cudaMemcpyHostToDevice or ::cudaMemcpyDeviceToHost and `stream`\nis non-zero, the copy may overlap with operations in other streams.\n\n# Arguments\n\n* `dst` -     - Destination memory address\n* `wOffset` - - Destination starting X offset (columns in bytes)\n* `hOffset` - - Destination starting Y offset (rows)\n* `src` -     - Source memory address\n* `count` -   - Size in bytes to copy\n* `kind` -    - Type of transfer\n* `stream` -  - Stream identifier\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorInvalidMemcpyDirection\n\\notefnerr \\note_async \\note_null_stream \\note_init_rt \\note_callback # See also\n\n> [`::cudaMemcpy,`] ::cudaMemcpy2D, ::cudaMemcpyToArray,\n::cudaMemcpy2DToArray, ::cudaMemcpyFromArray, ::cudaMemcpy2DFromArray,\n::cudaMemcpyArrayToArray, ::cudaMemcpy2DArrayToArray, ::cudaMemcpyToSymbol,\n::cudaMemcpyFromSymbol, ::cudaMemcpyAsync, ::cudaMemcpy2DAsync,\n::cudaMemcpy2DToArrayAsync,\n::cudaMemcpyFromArrayAsync, ::cudaMemcpy2DFromArrayAsync,\n::cudaMemcpyToSymbolAsync, ::cudaMemcpyFromSymbolAsync,\n::cuMemcpyHtoAAsync,\n::cuMemcpy2DAsync"]
pub unsafe fn cudaMemcpyToArrayAsync<T: types::CudaAsPtr>(
    dst: cudaArray_t,
    wOffset: usize,
    hOffset: usize,
    src: T,
    count: usize,
    kind: cudaMemcpyKind,
    stream: cudaStream_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaMemcpyToArrayAsync(
            dst,
            wOffset,
            hOffset,
            src.as_const_ptr() as *const _,
            count,
            kind,
            stream,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Copies data between host and device\n> **Deprecated** Copies `count` bytes from the CUDA array `src` starting at `hOffset` rows\nand `wOffset` bytes from the upper left corner to the memory area pointed to\nby `dst,` where `kind` specifies the direction of the copy, and must be one of\n::cudaMemcpyHostToHost, ::cudaMemcpyHostToDevice, ::cudaMemcpyDeviceToHost,\n::cudaMemcpyDeviceToDevice, or ::cudaMemcpyDefault. Passing\n::cudaMemcpyDefault is recommended, in which case the type of transfer is\ninferred from the pointer values. However, ::cudaMemcpyDefault is only\nallowed on systems that support unified virtual addressing.\n::cudaMemcpyFromArrayAsync() is asynchronous with respect to the host, so\nthe call may return before the copy is complete. The copy can optionally\nbe associated to a stream by passing a non-zero `stream` argument. If `kind` is ::cudaMemcpyHostToDevice or ::cudaMemcpyDeviceToHost and `stream`\nis non-zero, the copy may overlap with operations in other streams.\n\n# Arguments\n\n* `dst` -     - Destination memory address\n* `src` -     - Source memory address\n* `wOffset` - - Source starting X offset (columns in bytes)\n* `hOffset` - - Source starting Y offset (rows)\n* `count` -   - Size in bytes to copy\n* `kind` -    - Type of transfer\n* `stream` -  - Stream identifier\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorInvalidMemcpyDirection\n\\notefnerr \\note_async \\note_null_stream \\note_init_rt \\note_callback # See also\n\n> [`::cudaMemcpy,`] ::cudaMemcpy2D, ::cudaMemcpyToArray,\n::cudaMemcpy2DToArray, ::cudaMemcpyFromArray, ::cudaMemcpy2DFromArray,\n::cudaMemcpyArrayToArray, ::cudaMemcpy2DArrayToArray, ::cudaMemcpyToSymbol,\n::cudaMemcpyFromSymbol, ::cudaMemcpyAsync, ::cudaMemcpy2DAsync,\n::cudaMemcpyToArrayAsync, ::cudaMemcpy2DToArrayAsync,\n::cudaMemcpy2DFromArrayAsync,\n::cudaMemcpyToSymbolAsync, ::cudaMemcpyFromSymbolAsync,\n::cuMemcpyAtoHAsync,\n::cuMemcpy2DAsync"]
pub unsafe fn cudaMemcpyFromArrayAsync<T: types::CudaAsPtr>(
    mut dst: T,
    src: cudaArray_const_t,
    wOffset: usize,
    hOffset: usize,
    count: usize,
    kind: cudaMemcpyKind,
    stream: cudaStream_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaMemcpyFromArrayAsync(dst.as_mut_ptr() as *mut _, src, wOffset, hOffset, count, kind, stream)
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Allocates memory with stream ordered semantics\nInserts an allocation operation into `hStream.`\nA pointer to the allocated memory is returned immediately in *dptr.\nThe allocation must not be accessed until the the allocation operation completes.\nThe allocation comes from the memory pool associated with the stream's device.\n> **Note** The default memory pool of a device contains device memory from that device.\n> **Note** Basic stream ordering allows future work submitted into the same stream to use the allocation.\nStream query, stream synchronize, and CUDA events can be used to guarantee that the allocation\noperation completes before work submitted in a separate stream runs.\n> **Note** During stream capture, this function results in the creation of an allocation node.  In this case,\nthe allocation is owned by the graph instead of the memory pool. The memory pool's properties\nare used to set the node's creation parameters.\n\n# Arguments\n\n* `devPtr` [out]  -  - Returned device pointer\n* `size` [in]  -     - Number of bytes to allocate\n* `hStream` [in]  -  - The stream establishing the stream ordering contract and the memory pool to allocate from\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorNotSupported,\n::cudaErrorOutOfMemory,\n\\notefnerr \\note_null_stream \\note_init_rt \\note_callback # See also\n\n> [`::cuMemAllocAsync,`]\n[`::cudaMallocAsync(void**`] ptr, size_t size, cudaMemPool_t memPool, cudaStream_t stream)  \"cudaMallocAsync (C++ API)\",\n::cudaMallocFromPoolAsync, ::cudaFreeAsync, ::cudaDeviceSetMemPool, ::cudaDeviceGetDefaultMemPool, ::cudaDeviceGetMemPool, ::cudaMemPoolSetAccess, ::cudaMemPoolSetAttribute, ::cudaMemPoolGetAttribute"]
pub unsafe fn cudaMallocAsync<T>(
    size: usize,
    hStream: cudaStream_t,
) -> Result<::cuda_libs_cudart::types::cuDeviceAllocation<T>, crate::sys::cudaError> {
    let mut dev_ptr = std::ptr::null_mut();
    let status = unsafe { crate::sys::cudaMallocAsync(&mut dev_ptr as *mut _ as *mut _, size, hStream) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(::cuda_libs_cudart::types::cuDeviceAllocation(dev_ptr as *mut T))
    } else {
        Err(status)
    }
}
#[doc = "Frees memory with stream ordered semantics\nInserts a free operation into `hStream.`\nThe allocation must not be accessed after stream execution reaches the free.\nAfter this API returns, accessing the memory from any subsequent work launched on the GPU\nor querying its pointer attributes results in undefined behavior.\n> **Note** During stream capture, this function results in the creation of a free node and\nmust therefore be passed the address of a graph allocation.\n\n# Arguments\n\n* `dptr` - - memory to free\n* `hStream` - - The stream establishing the stream ordering promise\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorNotSupported\n\\notefnerr \\note_null_stream \\note_init_rt \\note_callback # See also\n\n> [`::cuMemFreeAsync,`] ::cudaMallocAsync"]
pub unsafe fn cudaFreeAsync<T>(
    devPtr: ::cuda_libs_cudart::types::cuDeviceAllocation<T>,
    hStream: cudaStream_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaFreeAsync(devPtr.0 as *mut _, hStream) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Tries to release memory back to the OS\nReleases memory back to the OS until the pool contains fewer than minBytesToKeep\nreserved bytes, or there is no more memory that the allocator can safely release.\nThe allocator cannot release OS allocations that back outstanding asynchronous allocations.\nThe OS allocations may happen at different granularity from the user allocations.\n\\note: Allocations that have not been freed count as outstanding.\n\\note: Allocations that have been asynchronously freed but whose completion has\nnot been observed on the host (eg. by a synchronize) can count as outstanding.\n\n# Arguments\n\n* `pool` [in]  -           - The memory pool to trim\n* `minBytesToKeep` [in]  - - If the pool has less than minBytesToKeep reserved,\nthe TrimTo operation is a no-op.  Otherwise the pool will be guaranteed to have\nat least minBytesToKeep bytes reserved after the operation.\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue\n\\note_callback # See also\n\n> [`::cuMemPoolTrimTo,`] ::cudaMallocAsync, ::cudaFreeAsync, ::cudaDeviceGetDefaultMemPool, ::cudaDeviceGetMemPool, ::cudaMemPoolCreate"]
pub unsafe fn cudaMemPoolTrimTo(memPool: cudaMemPool_t, minBytesToKeep: usize) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaMemPoolTrimTo(memPool, minBytesToKeep) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Sets attributes of a memory pool\nSupported attributes are:\n- ::cudaMemPoolAttrReleaseThreshold: (value type = cuuint64_t)\nAmount of reserved memory in bytes to hold onto before trying\nto release memory back to the OS. When more than the release\nthreshold bytes of memory are held by the memory pool, the\nallocator will try to release memory back to the OS on the\nnext call to stream, event or context synchronize. (default 0)\n- ::cudaMemPoolReuseFollowEventDependencies: (value type = int)\nAllow ::cudaMallocAsync to use memory asynchronously freed\nin another stream as long as a stream ordering dependency\nof the allocating stream on the free action exists.\nCuda events and null stream interactions can create the required\nstream ordered dependencies. (default enabled)\n- ::cudaMemPoolReuseAllowOpportunistic: (value type = int)\nAllow reuse of already completed frees when there is no dependency\nbetween the free and allocation. (default enabled)\n- ::cudaMemPoolReuseAllowInternalDependencies: (value type = int)\nAllow ::cudaMallocAsync to insert new stream dependencies\nin order to establish the stream ordering required to reuse\na piece of memory released by ::cudaFreeAsync (default enabled).\n- ::cudaMemPoolAttrReservedMemHigh: (value type = cuuint64_t)\nReset the high watermark that tracks the amount of backing memory that was\nallocated for the memory pool. It is illegal to set this attribute to a non-zero value.\n- ::cudaMemPoolAttrUsedMemHigh: (value type = cuuint64_t)\nReset the high watermark that tracks the amount of used memory that was\nallocated for the memory pool. It is illegal to set this attribute to a non-zero value.\n\n# Arguments\n\n* `pool` [in]  -  - The memory pool to modify\n* `attr` [in]  -  - The attribute to modify\n* `value` [in]  - - Pointer to the value to assign\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue\n\\note_callback # See also\n\n> [`::cuMemPoolSetAttribute,`] ::cudaMallocAsync, ::cudaFreeAsync, ::cudaDeviceGetDefaultMemPool, ::cudaDeviceGetMemPool, ::cudaMemPoolCreate\n"]
pub unsafe fn cudaMemPoolSetAttribute<T: types::CudaAsPtr>(
    memPool: cudaMemPool_t,
    attr: cudaMemPoolAttr,
    mut value: T,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaMemPoolSetAttribute(memPool, attr, value.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Gets attributes of a memory pool\nSupported attributes are:\n- ::cudaMemPoolAttrReleaseThreshold: (value type = cuuint64_t)\nAmount of reserved memory in bytes to hold onto before trying\nto release memory back to the OS. When more than the release\nthreshold bytes of memory are held by the memory pool, the\nallocator will try to release memory back to the OS on the\nnext call to stream, event or context synchronize. (default 0)\n- ::cudaMemPoolReuseFollowEventDependencies: (value type = int)\nAllow ::cudaMallocAsync to use memory asynchronously freed\nin another stream as long as a stream ordering dependency\nof the allocating stream on the free action exists.\nCuda events and null stream interactions can create the required\nstream ordered dependencies. (default enabled)\n- ::cudaMemPoolReuseAllowOpportunistic: (value type = int)\nAllow reuse of already completed frees when there is no dependency\nbetween the free and allocation. (default enabled)\n- ::cudaMemPoolReuseAllowInternalDependencies: (value type = int)\nAllow ::cudaMallocAsync to insert new stream dependencies\nin order to establish the stream ordering required to reuse\na piece of memory released by ::cudaFreeAsync (default enabled).\n- ::cudaMemPoolAttrReservedMemCurrent: (value type = cuuint64_t)\nAmount of backing memory currently allocated for the mempool.\n- ::cudaMemPoolAttrReservedMemHigh: (value type = cuuint64_t)\nHigh watermark of backing memory allocated for the mempool since\nthe last time it was reset.\n- ::cudaMemPoolAttrUsedMemCurrent: (value type = cuuint64_t)\nAmount of memory from the pool that is currently in use by the application.\n- ::cudaMemPoolAttrUsedMemHigh: (value type = cuuint64_t)\nHigh watermark of the amount of memory from the pool that was in use by the\napplication since the last time it was reset.\nThe following properties can be also be queried on imported and default pools:\n- ::cudaMemPoolAttrAllocationType: (value type = cudaMemAllocationType)\nThe allocation type of the mempool\n- ::cudaMemPoolAttrExportHandleTypes: (value type = cudaMemAllocationHandleType)\nAvailable export handle types for the mempool. For imported pools this\nvalue is always cudaMemHandleTypeNone as an imported pool cannot be\nre-exported\n- ::cudaMemPoolAttrLocationId: (value type = int)\nThe location id for the mempool. If the location type for this pool is\ncudaMemLocationTypeInvisible then ID will be cudaInvalidDeviceId.\n- ::cudaMemPoolAttrLocationType: (value type = cudaMemLocationType)\nThe location type for the mempool. For imported memory pools where the\ndevice is not directly visible to the importing process or pools imported\nvia fabric handles across nodes this will be\ncudaMemlocataionTypeInvisible.\n- ::cudaMemPoolAttrMaxPoolSize: (value type = cuuint64_t)\nMaximum size of the pool in bytes, this value may be higher than what was\ninitially passed to cuMemPoolCreate due to alignment requirements. A\nvalue of 0 indicates no maximum size. For cudaMemAllocationTypeManaged\nand IPC imported pools this value will be system dependent.\n- ::cudaMemPoolAttrHwDecompressEnabled: (value type = int)\nIndicates whether the pool has hardware compresssion enabled\n\n# Arguments\n\n* `pool` [in]  -  - The memory pool to get attributes of\n* `attr` [in]  -  - The attribute to get\n* `value` [in]  - - Retrieved value\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue\n\\note_callback # See also\n\n> [`::cuMemPoolGetAttribute,`] ::cudaMallocAsync, ::cudaFreeAsync, ::cudaDeviceGetDefaultMemPool, ::cudaDeviceGetMemPool, ::cudaMemPoolCreate\n"]
pub unsafe fn cudaMemPoolGetAttribute<T: types::CudaAsPtr>(
    memPool: cudaMemPool_t,
    attr: cudaMemPoolAttr,
    mut value: T,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaMemPoolGetAttribute(memPool, attr, value.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Controls visibility of pools between devices\n\n# Arguments\n\n* `pool` [in]  -  - The pool being modified\n* `map` [in]  -   - Array of access descriptors. Each descriptor instructs the access to enable for a single gpu\n* `count` [in]  - - Number of descriptors in the map array.\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue\n\n# See also\n\n> [`::cuMemPoolSetAccess,`] ::cudaMemPoolGetAccess, ::cudaMallocAsync, cudaFreeAsync"]
pub unsafe fn cudaMemPoolSetAccess<T: types::CudaAsPtr>(
    memPool: cudaMemPool_t,
    descList: T,
    count: usize,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaMemPoolSetAccess(memPool, descList.as_const_ptr() as *const _, count) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Returns the accessibility of a pool from a device\nReturns the accessibility of the pool's memory from the specified location.\n\n# Arguments\n\n* `flags` [out]  -   - the accessibility of the pool from the specified location\n* `memPool` [in]  -  - the pool being queried\n* `location` [in]  - - the location accessing the pool\n\n# See also\n\n> [`::cuMemPoolGetAccess,`] ::cudaMemPoolSetAccess"]
pub unsafe fn cudaMemPoolGetAccess(
    memPool: cudaMemPool_t,
) -> Result<(cudaMemAccessFlags, cudaMemLocation), crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<cudaMemAccessFlags> = std::mem::MaybeUninit::uninit();
    let mut out_2: std::mem::MaybeUninit<cudaMemLocation> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudaMemPoolGetAccess(out_0.as_mut_ptr() as *mut _, memPool, out_2.as_mut_ptr() as *mut _)
    };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe {
            Ok((
                out_0.assume_init() as cudaMemAccessFlags,
                out_2.assume_init() as cudaMemLocation,
            ))
        }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Returns the default memory pool for a given location and allocation type\nThe memory location can be of one of ::cudaMemLocationTypeDevice, ::cudaMemLocationTypeHost or\n::cudaMemLocationTypeHostNuma. The allocation type can be one of ::cudaMemAllocationTypePinned or\n::cudaMemAllocationTypeManaged. When the allocation type is ::cudaMemAllocationTypeManaged,\nthe location type can also be ::cudaMemLocationTypeNone to indicate no preferred location\nfor the managed memory pool. In all other cases, the call return ::cudaErrorInvalidValue\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorNotSupported,\n\n# See also\n\n> [`::cuMemAllocAsync,`] ::cuMemPoolTrimTo, ::cuMemPoolGetAttribute, ::cuMemPoolSetAttribute, cuMemPoolSetAccess, ::cuMemGetMemPool, ::cuMemPoolCreate"]
pub unsafe fn cudaMemGetDefaultMemPool(
    type_: cudaMemAllocationType,
) -> Result<(cudaMemPool_t, cudaMemLocation), crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<cudaMemPool_t> = std::mem::MaybeUninit::uninit();
    let mut out_1: std::mem::MaybeUninit<cudaMemLocation> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudaMemGetDefaultMemPool(out_0.as_mut_ptr() as *mut _, out_1.as_mut_ptr() as *mut _, type_)
    };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe {
            Ok((
                out_0.assume_init() as cudaMemPool_t,
                out_1.assume_init() as cudaMemLocation,
            ))
        }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Gets the current memory pool for a given memory location and allocation type\nThe memory location can be of one of ::cudaMemLocationTypeDevice, ::cudaMemLocationTypeHost or\n::cudaMemLocationTypeHostNuma. The allocation type can be one of ::cudaMemAllocationTypePinned or\n::cudaMemAllocationTypeManaged. When the allocation type is ::cudaMemAllocationTypeManaged,\nthe location type can also be ::cudaMemLocationTypeNone to indicate no preferred location\nfor the managed memory pool. In all other cases, the call return ::cudaErrorInvalidValue\nReturns the last pool provided to ::cudaMemSetMemPool or ::cudaDeviceSetMemPool for this location and allocation type\nor the location's default memory pool if ::cudaMemSetMemPool or ::cudaDeviceSetMemPool for that allocType and location\nhas never been called.\nBy default the current mempool of a location is the default mempool for a device that can be obtained via cudaMemGetDefaultMemPool\nOtherwise the returned pool must have been set with ::cudaDeviceSetMemPool.\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue\n\n# See also\n\n> [`::cuDeviceGetDefaultMemPool,`] ::cuMemPoolCreate, ::cuDeviceSetMemPool, ::cuMemSetMemPool"]
pub unsafe fn cudaMemGetMemPool(
    type_: cudaMemAllocationType,
) -> Result<(cudaMemPool_t, cudaMemLocation), crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<cudaMemPool_t> = std::mem::MaybeUninit::uninit();
    let mut out_1: std::mem::MaybeUninit<cudaMemLocation> = std::mem::MaybeUninit::uninit();
    let status =
        unsafe { crate::sys::cudaMemGetMemPool(out_0.as_mut_ptr() as *mut _, out_1.as_mut_ptr() as *mut _, type_) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe {
            Ok((
                out_0.assume_init() as cudaMemPool_t,
                out_1.assume_init() as cudaMemLocation,
            ))
        }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Sets the current memory pool for a memory location and allocation type\nThe memory location can be of one of ::cudaMemLocationTypeDevice, ::cudaMemLocationTypeHost or\n::cudaMemLocationTypeHostNuma. The allocation type can be one of ::cudaMemAllocationTypePinned or\n::cudaMemAllocationTypeManaged. When the allocation type is ::cudaMemAllocationTypeManaged,\nthe location type can also be ::cudaMemLocationTypeNone to indicate no preferred location\nfor the managed memory pool. In all other cases, the call return ::cudaErrorInvalidValue\nWhen a memory pool is set as the current memory pool, the location parameter should be the same as the location of the pool.\nIf the location type or index don't match, the call returns ::cudaErrorInvalidValue.\nThe type of memory pool should also match the parameter allocType. Else the call returns ::cudaErrorInvalidValue.\nBy default, a memory location's current memory pool is its default memory pool.\nIf the location type is ::cudaMemLocationTypeDevice and the allocation type is ::cudaMemAllocationTypePinned, then\nthis API is the equivalent of calling ::cudaDeviceSetMemPool with the location id as the device.\nFor further details on the implications, please refer to the documentation for ::cudaDeviceSetMemPool.\n> **Note** Use ::cudaMallocFromPoolAsync to specify asynchronous allocations from a device different\nthan the one the stream runs on.\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue\n\n# See also\n\n> [`::cuDeviceGetDefaultMemPool,`] ::cuDeviceGetMemPool, ::cuMemGetMemPool, ::cuMemPoolCreate, ::cuMemPoolDestroy, ::cuMemAllocFromPoolAsync"]
pub unsafe fn cudaMemSetMemPool<T: types::CudaAsPtr>(
    mut location: T,
    type_: cudaMemAllocationType,
    memPool: cudaMemPool_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaMemSetMemPool(location.as_mut_ptr() as *mut _, type_, memPool) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Allocates memory from a specified pool with stream ordered semantics.\nInserts an allocation operation into `hStream.`\nA pointer to the allocated memory is returned immediately in *dptr.\nThe allocation must not be accessed until the the allocation operation completes.\nThe allocation comes from the specified memory pool.\n> **Note** -  The specified memory pool may be from a device different than that of the specified `hStream.`\n-  Basic stream ordering allows future work submitted into the same stream to use the allocation.\nStream query, stream synchronize, and CUDA events can be used to guarantee that the allocation\noperation completes before work submitted in a separate stream runs.\n> **Note** During stream capture, this function results in the creation of an allocation node.  In this case,\nthe allocation is owned by the graph instead of the memory pool. The memory pool's properties\nare used to set the node's creation parameters.\n\n# Arguments\n\n* `ptr` [out]  -     - Returned device pointer\n* `bytesize` [in]  - - Number of bytes to allocate\n* `memPool` [in]  -  - The pool to allocate from\n* `stream` [in]  -   - The stream establishing the stream ordering semantic\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorNotSupported,\n::cudaErrorOutOfMemory\n\n# See also\n\n> [`::cuMemAllocFromPoolAsync,`]\n[`::cudaMallocAsync(void**`] ptr, size_t size, cudaMemPool_t memPool, cudaStream_t stream)  \"cudaMallocAsync (C++ API)\",\n::cudaMallocAsync, ::cudaFreeAsync, ::cudaDeviceGetDefaultMemPool, ::cudaMemPoolCreate, ::cudaMemPoolSetAccess, ::cudaMemPoolSetAttribute"]
pub unsafe fn cudaMallocFromPoolAsync<T>(
    size: usize,
    memPool: cudaMemPool_t,
    stream: cudaStream_t,
) -> Result<::cuda_libs_cudart::types::cuDeviceAllocation<T>, crate::sys::cudaError> {
    let mut dev_ptr = std::ptr::null_mut();
    let status =
        unsafe { crate::sys::cudaMallocFromPoolAsync(&mut dev_ptr as *mut _ as *mut _, size, memPool, stream) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(::cuda_libs_cudart::types::cuDeviceAllocation(dev_ptr as *mut T))
    } else {
        Err(status)
    }
}
#[doc = "Exports a memory pool to the requested handle type.\nGiven an IPC capable mempool, create an OS handle to share the pool with another process.\nA recipient process can convert the shareable handle into a mempool with ::cudaMemPoolImportFromShareableHandle.\nIndividual pointers can then be shared with the ::cudaMemPoolExportPointer and ::cudaMemPoolImportPointer APIs.\nThe implementation of what the shareable handle is and how it can be transferred is defined by the requested\nhandle type.\n\\note: To create an IPC capable mempool, create a mempool with a CUmemAllocationHandleType other than cudaMemHandleTypeNone.\n\n# Arguments\n\n* `handle_out` [out]  -  - pointer to the location in which to store the requested handle\n* `pool` [in]  -         - pool to export\n* `handleType` [in]  -   - the type of handle to create\n* `flags` [in]  -        - must be 0\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorOutOfMemory\n\n# See also\n\n> [`::cuMemPoolExportToShareableHandle,`] ::cudaMemPoolImportFromShareableHandle, ::cudaMemPoolExportPointer, ::cudaMemPoolImportPointer"]
pub unsafe fn cudaMemPoolExportToShareableHandle<T: types::CudaAsPtr>(
    mut shareableHandle: T,
    memPool: cudaMemPool_t,
    handleType: cudaMemAllocationHandleType,
    flags: u32,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaMemPoolExportToShareableHandle(
            shareableHandle.as_mut_ptr() as *mut _,
            memPool,
            handleType,
            flags as _,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "imports a memory pool from a shared handle.\nSpecific allocations can be imported from the imported pool with ::cudaMemPoolImportPointer.\n> **Note** Imported memory pools do not support creating new allocations.\nAs such imported memory pools may not be used in ::cudaDeviceSetMemPool\nor ::cudaMallocFromPoolAsync calls.\n\n# Arguments\n\n* `pool_out` [out]  -    - Returned memory pool\n* `handle` [in]  -       - OS handle of the pool to open\n* `handleType` [in]  -   - The type of handle being imported\n* `flags` [in]  -        - must be 0\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorOutOfMemory\n\n# See also\n\n> [`::cuMemPoolImportFromShareableHandle,`] ::cudaMemPoolExportToShareableHandle, ::cudaMemPoolExportPointer, ::cudaMemPoolImportPointer"]
pub unsafe fn cudaMemPoolImportFromShareableHandle<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    mut memPool: T,
    mut shareableHandle: U,
    handleType: cudaMemAllocationHandleType,
    flags: u32,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaMemPoolImportFromShareableHandle(
            memPool.as_mut_ptr() as *mut _,
            shareableHandle.as_mut_ptr() as *mut _,
            handleType,
            flags as _,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Export data to share a memory pool allocation between processes.\nConstructs `shareData_out` for sharing a specific allocation from an already shared memory pool.\nThe recipient process can import the allocation with the ::cudaMemPoolImportPointer api.\nThe data is not a handle and may be shared through any IPC mechanism.\n\n# Arguments\n\n* `shareData_out` [out]  - - Returned export data\n* `ptr` [in]  -            - pointer to memory being exported\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorOutOfMemory\n\n# See also\n\n> [`::cuMemPoolExportPointer,`] ::cudaMemPoolExportToShareableHandle, ::cudaMemPoolImportFromShareableHandle, ::cudaMemPoolImportPointer"]
pub unsafe fn cudaMemPoolExportPointer<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    mut exportData: T,
    mut ptr: U,
) -> Result<(), crate::sys::cudaError> {
    let status =
        unsafe { crate::sys::cudaMemPoolExportPointer(exportData.as_mut_ptr() as *mut _, ptr.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Import a memory pool allocation from another process.\nReturns in `ptr_out` a pointer to the imported memory.\nThe imported memory must not be accessed before the allocation operation completes\nin the exporting process. The imported memory must be freed from all importing processes before\nbeing freed in the exporting process. The pointer may be freed with cudaFree\nor cudaFreeAsync.  If ::cudaFreeAsync is used, the free must be completed\non the importing process before the free operation on the exporting process.\n> **Note** The ::cudaFreeAsync api may be used in the exporting process before\nthe ::cudaFreeAsync operation completes in its stream as long as the\n::cudaFreeAsync in the exporting process specifies a stream with\na stream dependency on the importing process's ::cudaFreeAsync.\n\n# Arguments\n\n* `ptr_out` [out]  -  - pointer to imported memory\n* `pool` [in]  -      - pool from which to import\n* `shareData` [in]  - - data specifying the memory to import\n\n# Returns\n\n::CUDA_SUCCESS,\n::CUDA_ERROR_INVALID_VALUE,\n::CUDA_ERROR_NOT_INITIALIZED,\n::CUDA_ERROR_OUT_OF_MEMORY\n\n# See also\n\n> [`::cuMemPoolImportPointer,`] ::cudaMemPoolExportToShareableHandle, ::cudaMemPoolImportFromShareableHandle, ::cudaMemPoolExportPointer"]
pub unsafe fn cudaMemPoolImportPointer<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    mut ptr: T,
    memPool: cudaMemPool_t,
    mut exportData: U,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaMemPoolImportPointer(ptr.as_mut_ptr() as *mut _, memPool, exportData.as_mut_ptr() as *mut _)
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Returns attributes about a specified pointer\nReturns in `*attributes` the attributes of the pointer `ptr.`\nIf pointer was not allocated in, mapped by or registered with context\nsupporting unified addressing ::cudaErrorInvalidValue is returned.\n> **Note** In CUDA 11.0 forward passing host pointer will return ::cudaMemoryTypeUnregistered\nin ::cudaPointerAttributes::type and call will return ::cudaSuccess.\nThe ::cudaPointerAttributes structure is defined as:\n\\code struct cudaPointerAttributes {\nenum cudaMemoryType type;\nint device;\nvoid *devicePointer;\nvoid *hostPointer;\n}\n\\endcode In this structure, the individual fields mean\n- [`::cudaPointerAttributes::type`] identifies type of memory. It can be\n::cudaMemoryTypeUnregistered for unregistered host memory,\n::cudaMemoryTypeHost for registered host memory, ::cudaMemoryTypeDevice for device\nmemory or  ::cudaMemoryTypeManaged for managed memory.\n- [`::cudaPointerAttributes::device`] \"device\" is the device against which\n`ptr` was allocated.  If `ptr` has memory type ::cudaMemoryTypeDevice\nthen this identifies the device on which the memory referred to by `ptr`\nphysically resides.  If `ptr` has memory type ::cudaMemoryTypeHost then this\nidentifies the device which was current when the allocation was made\n(and if that device is deinitialized then this allocation will vanish\nwith that device's state).\n- [`::cudaPointerAttributes::devicePointer`] \"devicePointer\" is\nthe device pointer alias through which the memory referred to by `ptr`\nmay be accessed on the current device.\nIf the memory referred to by `ptr` cannot be accessed directly by the\ncurrent device then this is NULL.\n- [`::cudaPointerAttributes::hostPointer`] \"hostPointer\" is\nthe host pointer alias through which the memory referred to by `ptr`\nmay be accessed on the host.\nIf the memory referred to by `ptr` cannot be accessed directly by the\nhost then this is NULL.\n\n# Arguments\n\n* `attributes` - - Attributes for the specified pointer\n* `ptr` -        - Pointer to get attributes for\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidDevice,\n::cudaErrorInvalidValue\n\\note_init_rt \\note_callback # See also\n\n> [`::cudaGetDeviceCount,`] ::cudaGetDevice, ::cudaSetDevice,\n::cudaChooseDevice,\n::cudaInitDevice,\n::cuPointerGetAttributes"]
pub unsafe fn cudaPointerGetAttributes(
    ptr: *const ::std::os::raw::c_void,
) -> Result<cudaPointerAttributes, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<cudaPointerAttributes> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaPointerGetAttributes(out_0.as_mut_ptr() as *mut _, ptr) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_0.assume_init() as cudaPointerAttributes) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Queries if a device may directly access a peer device's memory.\nReturns in `*canAccessPeer` a value of 1 if device `device` is capable of\ndirectly accessing memory from `peerDevice` and 0 otherwise.  If direct\naccess of `peerDevice` from `device` is possible, then access may be\nenabled by calling ::cudaDeviceEnablePeerAccess().\n\n# Arguments\n\n* `canAccessPeer` - - Returned access capability\n* `device` -        - Device from which allocations on `peerDevice` are to\nbe directly accessed.\n* `peerDevice` -    - Device on which the allocations to be directly accessed\nby `device` reside.\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidDevice\n\\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaDeviceEnablePeerAccess,`]\n::cudaDeviceDisablePeerAccess,\n::cuDeviceCanAccessPeer"]
pub unsafe fn cudaDeviceCanAccessPeer<T: types::CudaAsPtr>(
    mut canAccessPeer: T,
    device: i32,
    peerDevice: i32,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaDeviceCanAccessPeer(canAccessPeer.as_mut_ptr() as *mut _, device as _, peerDevice as _)
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Enables direct access to memory allocations on a peer device.\nOn success, all allocations from `peerDevice` will immediately be accessible by\nthe current device.  They will remain accessible until access is explicitly\ndisabled using ::cudaDeviceDisablePeerAccess() or either device is reset using\n::cudaDeviceReset().\nNote that access granted by this call is unidirectional and that in order to access\nmemory on the current device from `peerDevice,` a separate symmetric call\nto ::cudaDeviceEnablePeerAccess() is required.\nNote that there are both device-wide and system-wide limitations per system\nconfiguration, as noted in the CUDA Programming Guide under the section\n\"Peer-to-Peer Memory Access\".\nReturns ::cudaErrorInvalidDevice if ::cudaDeviceCanAccessPeer() indicates\nthat the current device cannot directly access memory from `peerDevice.`\nReturns ::cudaErrorPeerAccessAlreadyEnabled if direct access of\n`peerDevice` from the current device has already been enabled.\nReturns ::cudaErrorInvalidValue if `flags` is not 0.\n\n# Arguments\n\n* `peerDevice` -  - Peer device to enable direct access to from the current device\n* `flags` -       - Reserved for future use and must be set to 0\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidDevice,\n::cudaErrorPeerAccessAlreadyEnabled,\n::cudaErrorInvalidValue\n\\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaDeviceCanAccessPeer,`]\n::cudaDeviceDisablePeerAccess,\n::cuCtxEnablePeerAccess"]
pub unsafe fn cudaDeviceEnablePeerAccess(peerDevice: i32, flags: u32) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaDeviceEnablePeerAccess(peerDevice as _, flags as _) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Disables direct access to memory allocations on a peer device.\nReturns ::cudaErrorPeerAccessNotEnabled if direct access to memory on\n`peerDevice` has not yet been enabled from the current device.\n\n# Arguments\n\n* `peerDevice` - - Peer device to disable direct access to\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorPeerAccessNotEnabled,\n::cudaErrorInvalidDevice\n\\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaDeviceCanAccessPeer,`]\n::cudaDeviceEnablePeerAccess,\n::cuCtxDisablePeerAccess"]
pub unsafe fn cudaDeviceDisablePeerAccess(peerDevice: i32) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaDeviceDisablePeerAccess(peerDevice as _) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Unregisters a graphics resource for access by CUDA\nUnregisters the graphics resource `resource` so it is not accessible by\nCUDA unless registered again.\nIf `resource` is invalid then ::cudaErrorInvalidResourceHandle is\nreturned.\n\n# Arguments\n\n* `resource` - - Resource to unregister\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidResourceHandle,\n::cudaErrorUnknown\n\\notefnerr \\note_init_rt \\note_callback \\note_destroy_ub # See also\n\n> [`::cudaGraphicsD3D9RegisterResource,`]\n::cudaGraphicsD3D10RegisterResource,\n::cudaGraphicsD3D11RegisterResource,\n::cudaGraphicsGLRegisterBuffer,\n::cudaGraphicsGLRegisterImage,\n::cuGraphicsUnregisterResource"]
pub unsafe fn cudaGraphicsUnregisterResource(resource: cudaGraphicsResource_t) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaGraphicsUnregisterResource(resource) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Set usage flags for mapping a graphics resource\nSet `flags` for mapping the graphics resource `resource.`\nChanges to `flags` will take effect the next time `resource` is mapped.\nThe `flags` argument may be any of the following:\n- ::cudaGraphicsMapFlagsNone: Specifies no hints about how `resource` will\nbe used. It is therefore assumed that CUDA may read from or write to `resource.`\n- ::cudaGraphicsMapFlagsReadOnly: Specifies that CUDA will not write to `resource.`\n- ::cudaGraphicsMapFlagsWriteDiscard: Specifies CUDA will not read from `resource` and will\nwrite over the entire contents of `resource,` so none of the data\npreviously stored in `resource` will be preserved.\nIf `resource` is presently mapped for access by CUDA then ::cudaErrorUnknown is returned.\nIf `flags` is not one of the above values then ::cudaErrorInvalidValue is returned.\n\n# Arguments\n\n* `resource` - - Registered resource to set flags for\n* `flags` -    - Parameters for resource mapping\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorInvalidResourceHandle,\n::cudaErrorUnknown,\n\\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaGraphicsMapResources,`]\n::cuGraphicsResourceSetMapFlags"]
pub unsafe fn cudaGraphicsResourceSetMapFlags(
    resource: cudaGraphicsResource_t,
    flags: u32,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaGraphicsResourceSetMapFlags(resource, flags as _) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Map graphics resources for access by CUDA\nMaps the `count` graphics resources in `resources` for access by CUDA.\nThe resources in `resources` may be accessed by CUDA until they\nare unmapped. The graphics API from which `resources` were registered\nshould not access any resources while they are mapped by CUDA. If an\napplication does so, the results are undefined.\nThis function provides the synchronization guarantee that any graphics calls\nissued before ::cudaGraphicsMapResources() will complete before any subsequent CUDA\nwork issued in `stream` begins.\nIf `resources` contains any duplicate entries then ::cudaErrorInvalidResourceHandle\nis returned. If any of `resources` are presently mapped for access by\nCUDA then ::cudaErrorUnknown is returned.\n\n# Arguments\n\n* `count` -     - Number of resources to map\n* `resources` - - Resources to map for CUDA\n* `stream` -    - Stream for synchronization\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidResourceHandle,\n::cudaErrorUnknown\n\\note_null_stream \\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaGraphicsResourceGetMappedPointer,`]\n::cudaGraphicsSubResourceGetMappedArray,\n::cudaGraphicsUnmapResources,\n::cuGraphicsMapResources"]
pub unsafe fn cudaGraphicsMapResources<T: types::CudaAsPtr>(
    count: i32,
    mut resources: T,
    stream: cudaStream_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaGraphicsMapResources(count as _, resources.as_mut_ptr() as *mut _, stream) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Unmap graphics resources.\nUnmaps the `count` graphics resources in `resources.`\nOnce unmapped, the resources in `resources` may not be accessed by CUDA\nuntil they are mapped again.\nThis function provides the synchronization guarantee that any CUDA work issued\nin `stream` before ::cudaGraphicsUnmapResources() will complete before any\nsubsequently issued graphics work begins.\nIf `resources` contains any duplicate entries then ::cudaErrorInvalidResourceHandle\nis returned. If any of `resources` are not presently mapped for access by\nCUDA then ::cudaErrorUnknown is returned.\n\n# Arguments\n\n* `count` -     - Number of resources to unmap\n* `resources` - - Resources to unmap\n* `stream` -    - Stream for synchronization\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidResourceHandle,\n::cudaErrorUnknown\n\\note_null_stream \\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaGraphicsMapResources,`]\n::cuGraphicsUnmapResources"]
pub unsafe fn cudaGraphicsUnmapResources<T: types::CudaAsPtr>(
    count: i32,
    mut resources: T,
    stream: cudaStream_t,
) -> Result<(), crate::sys::cudaError> {
    let status =
        unsafe { crate::sys::cudaGraphicsUnmapResources(count as _, resources.as_mut_ptr() as *mut _, stream) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Get an device pointer through which to access a mapped graphics resource.\nReturns in `*devPtr` a pointer through which the mapped graphics resource\n`resource` may be accessed.\nReturns in `*size` the size of the memory in bytes which may be accessed from that pointer.\nThe value set in `devPtr` may change every time that `resource` is mapped.\nIf `resource` is not a buffer then it cannot be accessed via a pointer and\n::cudaErrorUnknown is returned.\nIf `resource` is not mapped then ::cudaErrorUnknown is returned.\n*\n\n# Arguments\n\n* `devPtr` -     - Returned pointer through which `resource` may be accessed\n* `size` -       - Returned size of the buffer accessible starting at `*devPtr`\n* `resource` -   - Mapped resource to access\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorInvalidResourceHandle,\n::cudaErrorUnknown\n\\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaGraphicsMapResources,`]\n::cudaGraphicsSubResourceGetMappedArray,\n::cuGraphicsResourceGetMappedPointer"]
pub unsafe fn cudaGraphicsResourceGetMappedPointer(
    devPtr: *mut *mut ::std::os::raw::c_void,
    resource: cudaGraphicsResource_t,
) -> Result<usize, crate::sys::cudaError> {
    let mut out_1: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let status =
        unsafe { crate::sys::cudaGraphicsResourceGetMappedPointer(devPtr, out_1.as_mut_ptr() as *mut _, resource) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_1.assume_init() as usize) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Get an array through which to access a subresource of a mapped graphics resource.\nReturns in `*array` an array through which the subresource of the mapped\ngraphics resource `resource` which corresponds to array index `arrayIndex`\nand mipmap level `mipLevel` may be accessed.  The value set in `array` may\nchange every time that `resource` is mapped.\nIf `resource` is not a texture then it cannot be accessed via an array and\n::cudaErrorUnknown is returned.\nIf `arrayIndex` is not a valid array index for `resource` then\n::cudaErrorInvalidValue is returned.\nIf `mipLevel` is not a valid mipmap level for `resource` then\n::cudaErrorInvalidValue is returned.\nIf `resource` is not mapped then ::cudaErrorUnknown is returned.\n\n# Arguments\n\n* `array` -       - Returned array through which a subresource of `resource` may be accessed\n* `resource` -    - Mapped resource to access\n* `arrayIndex` -  - Array index for array textures or cubemap face\nindex as defined by ::cudaGraphicsCubeFace for\ncubemap textures for the subresource to access\n* `mipLevel` -    - Mipmap level for the subresource to access\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorInvalidResourceHandle,\n::cudaErrorUnknown\n\\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaGraphicsResourceGetMappedPointer,`]\n::cuGraphicsSubResourceGetMappedArray"]
pub unsafe fn cudaGraphicsSubResourceGetMappedArray(
    resource: cudaGraphicsResource_t,
    arrayIndex: u32,
    mipLevel: u32,
) -> Result<cudaArray_t, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<cudaArray_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudaGraphicsSubResourceGetMappedArray(
            out_0.as_mut_ptr() as *mut _,
            resource,
            arrayIndex as _,
            mipLevel as _,
        )
    };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_0.assume_init() as cudaArray_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Get a mipmapped array through which to access a mapped graphics resource.\nReturns in `*mipmappedArray` a mipmapped array through which the mapped\ngraphics resource `resource` may be accessed. The value set in `mipmappedArray` may\nchange every time that `resource` is mapped.\nIf `resource` is not a texture then it cannot be accessed via an array and\n::cudaErrorUnknown is returned.\nIf `resource` is not mapped then ::cudaErrorUnknown is returned.\n\n# Arguments\n\n* `mipmappedArray` - - Returned mipmapped array through which `resource` may be accessed\n* `resource` -       - Mapped resource to access\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorInvalidResourceHandle,\n::cudaErrorUnknown\n\\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaGraphicsResourceGetMappedPointer,`]\n::cuGraphicsResourceGetMappedMipmappedArray"]
pub unsafe fn cudaGraphicsResourceGetMappedMipmappedArray<T: types::CudaAsPtr>(
    mut mipmappedArray: T,
    resource: cudaGraphicsResource_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphicsResourceGetMappedMipmappedArray(mipmappedArray.as_mut_ptr() as *mut _, resource)
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Get the channel descriptor of an array\nReturns in `*desc` the channel descriptor of the CUDA array `array.`\n\n# Arguments\n\n* `desc` -  - Channel format\n* `array` - - Memory array on device\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue\n\\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`\\ref`] ::cudaCreateChannelDesc(int, int, int, int, cudaChannelFormatKind) \"cudaCreateChannelDesc (C API)\",\n::cudaCreateTextureObject, ::cudaCreateSurfaceObject"]
pub unsafe fn cudaGetChannelDesc(array: cudaArray_const_t) -> Result<cudaChannelFormatDesc, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<cudaChannelFormatDesc> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaGetChannelDesc(out_0.as_mut_ptr() as *mut _, array) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_0.assume_init() as cudaChannelFormatDesc) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Returns a texture object's resource descriptor\nReturns the resource descriptor for the texture object specified by `texObject.`\n\n# Arguments\n\n* `pResDesc` -  - Resource descriptor\n* `texObject` - - Texture object\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue\n\\note_init_rt \\note_callback # See also\n\n> [`::cudaCreateTextureObject,`]\n::cuTexObjectGetResourceDesc"]
pub unsafe fn cudaGetTextureObjectResourceDesc(
    texObject: cudaTextureObject_t,
) -> Result<cudaResourceDesc, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<cudaResourceDesc> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaGetTextureObjectResourceDesc(out_0.as_mut_ptr() as *mut _, texObject) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_0.assume_init() as cudaResourceDesc) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Returns a texture object's texture descriptor\nReturns the texture descriptor for the texture object specified by `texObject.`\n\n# Arguments\n\n* `pTexDesc` -  - Texture descriptor\n* `texObject` - - Texture object\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue\n\\note_init_rt \\note_callback # See also\n\n> [`::cudaCreateTextureObject,`]\n::cuTexObjectGetTextureDesc"]
pub unsafe fn cudaGetTextureObjectTextureDesc(
    texObject: cudaTextureObject_t,
) -> Result<cudaTextureDesc, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<cudaTextureDesc> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaGetTextureObjectTextureDesc(out_0.as_mut_ptr() as *mut _, texObject) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_0.assume_init() as cudaTextureDesc) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Returns a texture object's resource view descriptor\nReturns the resource view descriptor for the texture object specified by `texObject.`\nIf no resource view was specified, ::cudaErrorInvalidValue is returned.\n\n# Arguments\n\n* `pResViewDesc` - - Resource view descriptor\n* `texObject` -    - Texture object\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue\n\\note_init_rt \\note_callback # See also\n\n> [`::cudaCreateTextureObject,`]\n::cuTexObjectGetResourceViewDesc"]
pub unsafe fn cudaGetTextureObjectResourceViewDesc(
    texObject: cudaTextureObject_t,
) -> Result<cudaResourceViewDesc, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<cudaResourceViewDesc> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaGetTextureObjectResourceViewDesc(out_0.as_mut_ptr() as *mut _, texObject) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_0.assume_init() as cudaResourceViewDesc) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Returns a surface object's resource descriptor\nReturns the resource descriptor for the surface object specified by `surfObject.`\n\n# Arguments\n\n* `pResDesc` -   - Resource descriptor\n* `surfObject` - - Surface object\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue\n\\note_init_rt \\note_callback # See also\n\n> [`::cudaCreateSurfaceObject,`]\n::cuSurfObjectGetResourceDesc"]
pub unsafe fn cudaGetSurfaceObjectResourceDesc(
    surfObject: cudaSurfaceObject_t,
) -> Result<cudaResourceDesc, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<cudaResourceDesc> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaGetSurfaceObjectResourceDesc(out_0.as_mut_ptr() as *mut _, surfObject) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_0.assume_init() as cudaResourceDesc) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Returns the latest version of CUDA supported by the driver\nReturns in `*driverVersion` the latest version of CUDA supported by\nthe driver. The version is returned as (1000 &times; major + 10 &times; minor).\nFor example, CUDA 9.2 would be represented by 9020. If no driver is installed,\nthen 0 is returned as the driver version.\nThis function automatically returns ::cudaErrorInvalidValue\nif `driverVersion` is NULL.\n\n# Arguments\n\n* `driverVersion` - - Returns the CUDA driver version.\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue\n\\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaRuntimeGetVersion,`]\n::cuDriverGetVersion"]
pub unsafe fn cudaDriverGetVersion() -> Result<i32, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaDriverGetVersion(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_0.assume_init() as i32) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Returns the CUDA Runtime version\nReturns in `*runtimeVersion` the version number of the current CUDA\nRuntime instance. The version is returned as\n(1000 &times; major + 10 &times; minor). For example,\nCUDA 9.2 would be represented by 9020.\nAs of CUDA 12.0, this function no longer initializes CUDA. The purpose\nof this API is solely to return a compile-time constant stating the\nCUDA Toolkit version in the above format.\nThis function automatically returns ::cudaErrorInvalidValue if\nthe `runtimeVersion` argument is NULL.\n\n# Arguments\n\n* `runtimeVersion` - - Returns the CUDA Runtime version.\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue\n\\note_init_rt \\note_callback # See also\n\n> [`::cudaDriverGetVersion,`]\n::cuDriverGetVersion"]
pub unsafe fn cudaRuntimeGetVersion() -> Result<i32, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaRuntimeGetVersion(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_0.assume_init() as i32) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Register a callback function to receive error log messages\n\n# Arguments\n\n* `callbackFunc` -  - The function to register as a callback\n* `userData` -      - A generic pointer to user data. This is passed into the callback function.\n* `callback_out` -  - Optional location to store the callback handle after it is registered\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,"]
pub unsafe fn cudaLogsRegisterCallback<T: types::CudaAsPtr>(
    callbackFunc: cudaLogsCallback_t,
    mut userData: T,
    callback_out: *mut cudaLogsCallbackHandle,
) -> Result<(), crate::sys::cudaError> {
    let status =
        unsafe { crate::sys::cudaLogsRegisterCallback(callbackFunc, userData.as_mut_ptr() as *mut _, callback_out) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Unregister a log message callback\n\n# Arguments\n\n* `callback` -  - The callback instance to unregister from receiving log messages\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,"]
pub unsafe fn cudaLogsUnregisterCallback(callback: cudaLogsCallbackHandle) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaLogsUnregisterCallback(callback) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Sets log iterator to point to the end of log buffer, where the next message would be written.\n\n# Arguments\n\n* `iterator_out` - - Location to store an iterator to the current tail of the logs\n* `flags` -        - Reserved for future use, must be 0\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,"]
pub unsafe fn cudaLogsCurrent<T: types::CudaAsPtr>(
    mut iterator_out: T,
    flags: u32,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaLogsCurrent(iterator_out.as_mut_ptr() as *mut _, flags as _) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Dump accumulated driver logs into a file\nLogs generated by the driver are stored in an internal buffer and can be copied out using this API.\nThis API dumps all driver logs starting from `iterator` into `pathToFile` provided.\n> **Note** `iterator` is auto-advancing. Dumping logs will update the value of\n`iterator` to receive the next generated log.\n> **Note** The driver reserves limited memory for storing logs.\nThe oldest logs may be overwritten and become unrecoverable. An indication will appear in the\ndestination outupt if the logs have been truncated. Call dump after each failed API to mitigate this\nrisk.\n\n# Arguments\n\n* `iterator` -   - Optional auto-advancing iterator specifying the starting log to read. NULL value dumps all logs.\n* `pathToFile` - - Path to output file for dumping logs\n* `flags` -      - Reserved for future use, must be 0\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,"]
pub unsafe fn cudaLogsDumpToFile<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    mut iterator: T,
    pathToFile: U,
    flags: u32,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaLogsDumpToFile(
            iterator.as_mut_ptr() as *mut _,
            pathToFile.as_const_ptr() as *const _,
            flags as _,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Dump accumulated driver logs into a buffer\nLogs generated by the driver are stored in an internal buffer and can be copied out using this API.\nThis API dumps driver logs from `iterator` into `buffer` up to the size specified in `*size.`\nThe driver will always null terminate the buffer but there will not be a null character between log\nentries, only a newline \\\\n. The driver will then return the actual number of bytes written in\n`*size,` excluding the null terminator. If there are no messages to dump, `*size` will be set to 0\nand the function will return ::CUDA_SUCCESS.\nIf the provided `buffer` is not large enough to hold any messages, `*size` will be set to 0 and\nthe function will return ::CUDA_ERROR_INVALID_VALUE.\n> **Note** `iterator` is auto-advancing. Dumping logs will update the value of\n`iterator` to receive the next generated log.\n> **Note** The driver reserves limited memory for storing logs. The maximum size of the buffer is 25600 bytes.\nThe oldest logs may be overwritten and become unrecoverable. An indication will appear in the\ndestination outupt if the logs have been truncated. Call dump after each failed API to mitigate this\nrisk.\n> **Note** If the provided value in `*size` is not large enough to hold all buffered messages, a message will\nbe added at the head of the buffer indicating this. The driver then computes the number of messages\nit is able to store in `buffer` and writes it out. The final message in `buffer` will always be\nthe most recent log message as of when the API is called.\n\n# Arguments\n\n* `iterator` -  - Optional auto-advancing iterator specifying the starting log to read. NULL value dumps all logs.\n* `buffer` -    - Pointer to dump logs\n* `size` -      - See description\n* `flags` -     - Reserved for future use, must be 0\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,"]
pub unsafe fn cudaLogsDumpToMemory<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    mut iterator: T,
    mut buffer: U,
    mut size: V,
    flags: u32,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaLogsDumpToMemory(
            iterator.as_mut_ptr() as *mut _,
            buffer.as_mut_ptr() as *mut _,
            size.as_mut_ptr() as *mut _,
            flags as _,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Creates a kernel execution node and adds it to a graph\nCreates a new kernel execution node and adds it to `graph` with `numDependencies`\ndependencies specified via `pDependencies` and arguments specified in `pNodeParams.`\nIt is possible for `numDependencies` to be 0, in which case the node will be placed\nat the root of the graph. `pDependencies` may not have any duplicate entries.\nA handle to the new node will be returned in `pGraphNode.`\nThe cudaKernelNodeParams structure is defined as:\n\\code struct cudaKernelNodeParams\n{\nvoid* func;\ndim3 gridDim;\ndim3 blockDim;\nunsigned int sharedMemBytes;\nvoid **kernelParams;\nvoid **extra;\n};\n\\endcode When the graph is launched, the node will invoke kernel `func` on a (`gridDim.x` x\n`gridDim.y` x `gridDim.z)` grid of blocks. Each block contains\n(`blockDim.x` x `blockDim.y` x `blockDim.z)` threads.\n`sharedMem` sets the amount of dynamic shared memory that will be\navailable to each thread block.\nKernel parameters to `func` can be specified in one of two ways:\n1) Kernel parameters can be specified via `kernelParams.` If the kernel has N\nparameters, then `kernelParams` needs to be an array of N pointers. Each pointer,\nfrom `kernelParams`[0] to `kernelParams`[N-1], points to the region of memory from which the actual\nparameter will be copied. The number of kernel parameters and their offsets and sizes do not need\nto be specified as that information is retrieved directly from the kernel's image.\n2) Kernel parameters can also be packaged by the application into a single buffer that is passed in\nvia `extra.` This places the burden on the application of knowing each kernel\nparameter's size and alignment/padding within the buffer. The `extra` parameter exists\nto allow this function to take additional less commonly used arguments. `extra` specifies\na list of names of extra settings and their corresponding values. Each extra setting name is\nimmediately followed by the corresponding value. The list must be terminated with either NULL or\nCU_LAUNCH_PARAM_END.\n- ::CU_LAUNCH_PARAM_END, which indicates the end of the `extra`\narray;\n- ::CU_LAUNCH_PARAM_BUFFER_POINTER, which specifies that the next\nvalue in `extra` will be a pointer to a buffer\ncontaining all the kernel parameters for launching kernel\n`func;`\n- ::CU_LAUNCH_PARAM_BUFFER_SIZE, which specifies that the next\nvalue in `extra` will be a pointer to a size_t\ncontaining the size of the buffer specified with\n::CU_LAUNCH_PARAM_BUFFER_POINTER;\nThe error ::cudaErrorInvalidValue will be returned if kernel parameters are specified with both\n`kernelParams` and `extra` (i.e. both `kernelParams` and\n`extra` are non-NULL).\nThe `kernelParams` or `extra` array, as well as the argument values it points to,\nare copied during this call.\n> **Note** Kernels launched using graphs must not use texture and surface references. Reading or\nwriting through any texture or surface reference is undefined behavior.\nThis restriction does not apply to texture and surface objects.\n\n# Arguments\n\n* `pGraphNode` -     - Returns newly created node\n* `graph` -          - Graph to which to add the node\n* `pDependencies` -    - Dependencies of the node\n* `numDependencies` - - Number of dependencies\n* `pNodeParams` -      - Parameters for the GPU execution node\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorInvalidDeviceFunction\n\\note_graph_thread_safety \\notefnerr \\note_init_rt \\note_callback \\note_cudaKernel_t # See also\n\n> [`::cudaGraphAddNode,`]\n::cudaLaunchKernel,\n::cudaGraphKernelNodeGetParams,\n::cudaGraphKernelNodeSetParams,\n::cudaGraphCreate,\n::cudaGraphDestroyNode,\n::cudaGraphAddChildGraphNode,\n::cudaGraphAddEmptyNode,\n::cudaGraphAddHostNode,\n::cudaGraphAddMemcpyNode,\n::cudaGraphAddMemsetNode"]
pub unsafe fn cudaGraphAddKernelNode<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    mut pGraphNode: T,
    graph: cudaGraph_t,
    pDependencies: U,
    numDependencies: usize,
    pNodeParams: V,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphAddKernelNode(
            pGraphNode.as_mut_ptr() as *mut _,
            graph,
            pDependencies.as_const_ptr() as *const _,
            numDependencies,
            pNodeParams.as_const_ptr() as *const _,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Returns a kernel node's parameters\nReturns the parameters of kernel node `node` in `pNodeParams.`\nThe `kernelParams` or `extra` array returned in `pNodeParams,`\nas well as the argument values it points to, are owned by the node.\nThis memory remains valid until the node is destroyed or its\nparameters are modified, and should not be modified\ndirectly. Use ::cudaGraphKernelNodeSetParams to update the\nparameters of this node.\nThe params will contain either `kernelParams` or `extra,`\naccording to which of these was most recently set on the node.\n\n# Arguments\n\n* `node` -        - Node to get the parameters for\n* `pNodeParams` - - Pointer to return the parameters\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorInvalidDeviceFunction\n\\note_graph_thread_safety \\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaGraphNodeGetParams,`]\n::cudaLaunchKernel,\n::cudaGraphAddKernelNode,\n::cudaGraphKernelNodeSetParams"]
pub unsafe fn cudaGraphKernelNodeGetParams(
    node: cudaGraphNode_t,
) -> Result<cudaKernelNodeParams, crate::sys::cudaError> {
    let mut out_1: std::mem::MaybeUninit<cudaKernelNodeParams> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaGraphKernelNodeGetParams(node, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_1.assume_init() as cudaKernelNodeParams) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Sets a kernel node's parameters\nSets the parameters of kernel node `node` to `pNodeParams.`\n\n# Arguments\n\n* `node` -        - Node to set the parameters for\n* `pNodeParams` - - Parameters to copy\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorInvalidResourceHandle,\n::cudaErrorMemoryAllocation\n\\note_graph_thread_safety \\notefnerr \\note_init_rt \\note_callback \\note_cudaKernel_t # See also\n\n> [`::cudaGraphNodeSetParams,`]\n::cudaLaunchKernel,\n::cudaGraphAddKernelNode,\n::cudaGraphKernelNodeGetParams"]
pub unsafe fn cudaGraphKernelNodeSetParams<T: types::CudaAsPtr>(
    node: cudaGraphNode_t,
    pNodeParams: T,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaGraphKernelNodeSetParams(node, pNodeParams.as_const_ptr() as *const _) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Copies attributes from source node to destination node.\nCopies attributes from source node `hSrc` to destination node `hDst.`\nBoth node must have the same context.\n\n# Arguments\n\n* `hDst` [out]  - Destination node\n* `hSrc` [in]  - Source node\nFor list of attributes see ::cudaKernelNodeAttrID\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidContext\n\\notefnerr # See also\n\n> [`::cudaAccessPolicyWindow`]"]
pub unsafe fn cudaGraphKernelNodeCopyAttributes(
    hDst: cudaGraphNode_t,
    hSrc: cudaGraphNode_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaGraphKernelNodeCopyAttributes(hDst, hSrc) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Queries node attribute.\nQueries attribute `attr` from node `hNode` and stores it in corresponding\nmember of `value_out.`\n\n# Arguments\n\n* `hNode` [in]  -\n* `attr` [in]  -\n* `value_out` [out]  -\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorInvalidResourceHandle\n\\notefnerr # See also\n\n> [`::cudaAccessPolicyWindow`]"]
pub unsafe fn cudaGraphKernelNodeGetAttribute(
    hNode: cudaGraphNode_t,
    attr: cudaLaunchAttributeID,
) -> Result<cudaLaunchAttributeValue, crate::sys::cudaError> {
    let mut out_2: std::mem::MaybeUninit<cudaLaunchAttributeValue> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaGraphKernelNodeGetAttribute(hNode, attr, out_2.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_2.assume_init() as cudaLaunchAttributeValue) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Sets node attribute.\nSets attribute `attr` on node `hNode` from corresponding attribute of\n`value.`\n\n# Arguments\n\n* `hNode` [out]  -\n* `attr` [in]  -\n* `value` [out]  -\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorInvalidResourceHandle\n\\notefnerr # See also\n\n> [`::cudaAccessPolicyWindow`]"]
pub unsafe fn cudaGraphKernelNodeSetAttribute<T: types::CudaAsPtr>(
    hNode: cudaGraphNode_t,
    attr: cudaLaunchAttributeID,
    value: T,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaGraphKernelNodeSetAttribute(hNode, attr, value.as_const_ptr() as *const _) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Creates a memcpy node and adds it to a graph\nCreates a new memcpy node and adds it to `graph` with `numDependencies`\ndependencies specified via `pDependencies.`\nIt is possible for `numDependencies` to be 0, in which case the node will be placed\nat the root of the graph. `pDependencies` may not have any duplicate entries.\nA handle to the new node will be returned in `pGraphNode.`\nWhen the graph is launched, the node will perform the memcpy described by `pCopyParams.`\nSee ::cudaMemcpy3D() for a description of the structure and its restrictions.\nMemcpy nodes have some additional restrictions with regards to managed memory, if the\nsystem contains at least one device which has a zero value for the device attribute\n::cudaDevAttrConcurrentManagedAccess.\n\n# Arguments\n\n* `pGraphNode` -     - Returns newly created node\n* `graph` -          - Graph to which to add the node\n* `pDependencies` -    - Dependencies of the node\n* `numDependencies` - - Number of dependencies\n* `pCopyParams` -      - Parameters for the memory copy\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue\n\\note_graph_thread_safety \\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaGraphAddNode,`]\n::cudaMemcpy3D,\n::cudaGraphAddMemcpyNodeToSymbol,\n::cudaGraphAddMemcpyNodeFromSymbol,\n::cudaGraphAddMemcpyNode1D,\n::cudaGraphMemcpyNodeGetParams,\n::cudaGraphMemcpyNodeSetParams,\n::cudaGraphCreate,\n::cudaGraphDestroyNode,\n::cudaGraphAddChildGraphNode,\n::cudaGraphAddEmptyNode,\n::cudaGraphAddKernelNode,\n::cudaGraphAddHostNode,\n::cudaGraphAddMemsetNode"]
pub unsafe fn cudaGraphAddMemcpyNode<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    mut pGraphNode: T,
    graph: cudaGraph_t,
    pDependencies: U,
    numDependencies: usize,
    pCopyParams: V,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphAddMemcpyNode(
            pGraphNode.as_mut_ptr() as *mut _,
            graph,
            pDependencies.as_const_ptr() as *const _,
            numDependencies,
            pCopyParams.as_const_ptr() as *const _,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudaGraphAddMemcpyNodeToSymbol<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    mut pGraphNode: T,
    graph: cudaGraph_t,
    pDependencies: U,
    numDependencies: usize,
    symbol: V,
    src: W,
    count: usize,
    offset: usize,
    kind: cudaMemcpyKind,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphAddMemcpyNodeToSymbol(
            pGraphNode.as_mut_ptr() as *mut _,
            graph,
            pDependencies.as_const_ptr() as *const _,
            numDependencies,
            symbol.as_const_ptr() as *const _,
            src.as_const_ptr() as *const _,
            count,
            offset,
            kind,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudaGraphAddMemcpyNodeFromSymbol<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    mut pGraphNode: T,
    graph: cudaGraph_t,
    pDependencies: U,
    numDependencies: usize,
    mut dst: V,
    symbol: W,
    count: usize,
    offset: usize,
    kind: cudaMemcpyKind,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphAddMemcpyNodeFromSymbol(
            pGraphNode.as_mut_ptr() as *mut _,
            graph,
            pDependencies.as_const_ptr() as *const _,
            numDependencies,
            dst.as_mut_ptr() as *mut _,
            symbol.as_const_ptr() as *const _,
            count,
            offset,
            kind,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudaGraphAddMemcpyNode1D<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    mut pGraphNode: T,
    graph: cudaGraph_t,
    pDependencies: U,
    numDependencies: usize,
    mut dst: V,
    src: W,
    count: usize,
    kind: cudaMemcpyKind,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphAddMemcpyNode1D(
            pGraphNode.as_mut_ptr() as *mut _,
            graph,
            pDependencies.as_const_ptr() as *const _,
            numDependencies,
            dst.as_mut_ptr() as *mut _,
            src.as_const_ptr() as *const _,
            count,
            kind,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Returns a memcpy node's parameters\nReturns the parameters of memcpy node `node` in `pNodeParams.`\n\n# Arguments\n\n* `node` -        - Node to get the parameters for\n* `pNodeParams` - - Pointer to return the parameters\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue\n\\note_graph_thread_safety \\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaGraphNodeGetParams,`]\n::cudaMemcpy3D,\n::cudaGraphAddMemcpyNode,\n::cudaGraphMemcpyNodeSetParams"]
pub unsafe fn cudaGraphMemcpyNodeGetParams(node: cudaGraphNode_t) -> Result<cudaMemcpy3DParms, crate::sys::cudaError> {
    let mut out_1: std::mem::MaybeUninit<cudaMemcpy3DParms> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaGraphMemcpyNodeGetParams(node, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_1.assume_init() as cudaMemcpy3DParms) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Sets a memcpy node's parameters\nSets the parameters of memcpy node `node` to `pNodeParams.`\n\n# Arguments\n\n* `node` -        - Node to set the parameters for\n* `pNodeParams` - - Parameters to copy\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n\\note_graph_thread_safety \\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaGraphNodeSetParams,`]\n::cudaMemcpy3D,\n::cudaGraphMemcpyNodeSetParamsToSymbol,\n::cudaGraphMemcpyNodeSetParamsFromSymbol,\n::cudaGraphMemcpyNodeSetParams1D,\n::cudaGraphAddMemcpyNode,\n::cudaGraphMemcpyNodeGetParams"]
pub unsafe fn cudaGraphMemcpyNodeSetParams<T: types::CudaAsPtr>(
    node: cudaGraphNode_t,
    pNodeParams: T,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaGraphMemcpyNodeSetParams(node, pNodeParams.as_const_ptr() as *const _) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudaGraphMemcpyNodeSetParamsToSymbol<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    node: cudaGraphNode_t,
    symbol: T,
    src: U,
    count: usize,
    offset: usize,
    kind: cudaMemcpyKind,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphMemcpyNodeSetParamsToSymbol(
            node,
            symbol.as_const_ptr() as *const _,
            src.as_const_ptr() as *const _,
            count,
            offset,
            kind,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudaGraphMemcpyNodeSetParamsFromSymbol<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    node: cudaGraphNode_t,
    mut dst: T,
    symbol: U,
    count: usize,
    offset: usize,
    kind: cudaMemcpyKind,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphMemcpyNodeSetParamsFromSymbol(
            node,
            dst.as_mut_ptr() as *mut _,
            symbol.as_const_ptr() as *const _,
            count,
            offset,
            kind,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudaGraphMemcpyNodeSetParams1D<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    node: cudaGraphNode_t,
    mut dst: T,
    src: U,
    count: usize,
    kind: cudaMemcpyKind,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphMemcpyNodeSetParams1D(
            node,
            dst.as_mut_ptr() as *mut _,
            src.as_const_ptr() as *const _,
            count,
            kind,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Creates a memset node and adds it to a graph\nCreates a new memset node and adds it to `graph` with `numDependencies`\ndependencies specified via `pDependencies.`\nIt is possible for `numDependencies` to be 0, in which case the node will be placed\nat the root of the graph. `pDependencies` may not have any duplicate entries.\nA handle to the new node will be returned in `pGraphNode.`\nThe element size must be 1, 2, or 4 bytes.\nWhen the graph is launched, the node will perform the memset described by `pMemsetParams.`\n\n# Arguments\n\n* `pGraphNode` -     - Returns newly created node\n* `graph` -          - Graph to which to add the node\n* `pDependencies` -    - Dependencies of the node\n* `numDependencies` - - Number of dependencies\n* `pMemsetParams` -    - Parameters for the memory set\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorInvalidDevice\n\\note_graph_thread_safety \\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaGraphAddNode,`]\n::cudaMemset2D,\n::cudaGraphMemsetNodeGetParams,\n::cudaGraphMemsetNodeSetParams,\n::cudaGraphCreate,\n::cudaGraphDestroyNode,\n::cudaGraphAddChildGraphNode,\n::cudaGraphAddEmptyNode,\n::cudaGraphAddKernelNode,\n::cudaGraphAddHostNode,\n::cudaGraphAddMemcpyNode"]
pub unsafe fn cudaGraphAddMemsetNode<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    mut pGraphNode: T,
    graph: cudaGraph_t,
    pDependencies: U,
    numDependencies: usize,
    pMemsetParams: V,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphAddMemsetNode(
            pGraphNode.as_mut_ptr() as *mut _,
            graph,
            pDependencies.as_const_ptr() as *const _,
            numDependencies,
            pMemsetParams.as_const_ptr() as *const _,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Returns a memset node's parameters\nReturns the parameters of memset node `node` in `pNodeParams.`\n\n# Arguments\n\n* `node` -        - Node to get the parameters for\n* `pNodeParams` - - Pointer to return the parameters\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue\n\\note_graph_thread_safety \\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaGraphNodeGetParams,`]\n::cudaMemset2D,\n::cudaGraphAddMemsetNode,\n::cudaGraphMemsetNodeSetParams"]
pub unsafe fn cudaGraphMemsetNodeGetParams(node: cudaGraphNode_t) -> Result<cudaMemsetParams, crate::sys::cudaError> {
    let mut out_1: std::mem::MaybeUninit<cudaMemsetParams> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaGraphMemsetNodeGetParams(node, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_1.assume_init() as cudaMemsetParams) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Sets a memset node's parameters\nSets the parameters of memset node `node` to `pNodeParams.`\n\n# Arguments\n\n* `node` -        - Node to set the parameters for\n* `pNodeParams` - - Parameters to copy\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue\n\\note_graph_thread_safety \\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaGraphNodeSetParams,`]\n::cudaMemset2D,\n::cudaGraphAddMemsetNode,\n::cudaGraphMemsetNodeGetParams"]
pub unsafe fn cudaGraphMemsetNodeSetParams<T: types::CudaAsPtr>(
    node: cudaGraphNode_t,
    pNodeParams: T,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaGraphMemsetNodeSetParams(node, pNodeParams.as_const_ptr() as *const _) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Creates a host execution node and adds it to a graph\nCreates a new CPU execution node and adds it to `graph` with `numDependencies`\ndependencies specified via `pDependencies` and arguments specified in `pNodeParams.`\nIt is possible for `numDependencies` to be 0, in which case the node will be placed\nat the root of the graph. `pDependencies` may not have any duplicate entries.\nA handle to the new node will be returned in `pGraphNode.`\nWhen the graph is launched, the node will invoke the specified CPU function.\nHost nodes are not supported under MPS with pre-Volta GPUs.\n\n# Arguments\n\n* `pGraphNode` -     - Returns newly created node\n* `graph` -          - Graph to which to add the node\n* `pDependencies` -    - Dependencies of the node\n* `numDependencies` - - Number of dependencies\n* `pNodeParams` -      - Parameters for the host node\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorNotSupported,\n::cudaErrorInvalidValue\n\\note_graph_thread_safety \\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaGraphAddNode,`]\n::cudaLaunchHostFunc,\n::cudaGraphHostNodeGetParams,\n::cudaGraphHostNodeSetParams,\n::cudaGraphCreate,\n::cudaGraphDestroyNode,\n::cudaGraphAddChildGraphNode,\n::cudaGraphAddEmptyNode,\n::cudaGraphAddKernelNode,\n::cudaGraphAddMemcpyNode,\n::cudaGraphAddMemsetNode"]
pub unsafe fn cudaGraphAddHostNode<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    mut pGraphNode: T,
    graph: cudaGraph_t,
    pDependencies: U,
    numDependencies: usize,
    pNodeParams: V,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphAddHostNode(
            pGraphNode.as_mut_ptr() as *mut _,
            graph,
            pDependencies.as_const_ptr() as *const _,
            numDependencies,
            pNodeParams.as_const_ptr() as *const _,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Returns a host node's parameters\nReturns the parameters of host node `node` in `pNodeParams.`\n\n# Arguments\n\n* `node` -        - Node to get the parameters for\n* `pNodeParams` - - Pointer to return the parameters\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue\n\\note_graph_thread_safety \\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaGraphNodeGetParams,`]\n::cudaLaunchHostFunc,\n::cudaGraphAddHostNode,\n::cudaGraphHostNodeSetParams"]
pub unsafe fn cudaGraphHostNodeGetParams(node: cudaGraphNode_t) -> Result<cudaHostNodeParams, crate::sys::cudaError> {
    let mut out_1: std::mem::MaybeUninit<cudaHostNodeParams> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaGraphHostNodeGetParams(node, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_1.assume_init() as cudaHostNodeParams) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Sets a host node's parameters\nSets the parameters of host node `node` to `nodeParams.`\n\n# Arguments\n\n* `node` -        - Node to set the parameters for\n* `pNodeParams` - - Parameters to copy\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue\n\\note_graph_thread_safety \\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaGraphNodeSetParams,`]\n::cudaLaunchHostFunc,\n::cudaGraphAddHostNode,\n::cudaGraphHostNodeGetParams"]
pub unsafe fn cudaGraphHostNodeSetParams<T: types::CudaAsPtr>(
    node: cudaGraphNode_t,
    pNodeParams: T,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaGraphHostNodeSetParams(node, pNodeParams.as_const_ptr() as *const _) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Creates a child graph node and adds it to a graph\nCreates a new node which executes an embedded graph, and adds it to `graph` with\n`numDependencies` dependencies specified via `pDependencies.`\nIt is possible for `numDependencies` to be 0, in which case the node will be placed\nat the root of the graph. `pDependencies` may not have any duplicate entries.\nA handle to the new node will be returned in `pGraphNode.`\nIf `childGraph` contains allocation nodes, free nodes, or conditional nodes, this call will\nreturn an error.\nThe node executes an embedded child graph. The child graph is cloned in this call.\n\n# Arguments\n\n* `pGraphNode` -     - Returns newly created node\n* `graph` -          - Graph to which to add the node\n* `pDependencies` -    - Dependencies of the node\n* `numDependencies` - - Number of dependencies\n* `childGraph` -      - The graph to clone into this node\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue\n\\note_graph_thread_safety \\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaGraphAddNode,`]\n::cudaGraphChildGraphNodeGetGraph,\n::cudaGraphCreate,\n::cudaGraphDestroyNode,\n::cudaGraphAddEmptyNode,\n::cudaGraphAddKernelNode,\n::cudaGraphAddHostNode,\n::cudaGraphAddMemcpyNode,\n::cudaGraphAddMemsetNode,\n::cudaGraphClone"]
pub unsafe fn cudaGraphAddChildGraphNode<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    mut pGraphNode: T,
    graph: cudaGraph_t,
    pDependencies: U,
    numDependencies: usize,
    childGraph: cudaGraph_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphAddChildGraphNode(
            pGraphNode.as_mut_ptr() as *mut _,
            graph,
            pDependencies.as_const_ptr() as *const _,
            numDependencies,
            childGraph,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Gets a handle to the embedded graph of a child graph node\nGets a handle to the embedded graph in a child graph node. This call\ndoes not clone the graph. Changes to the graph will be reflected in\nthe node, and the node retains ownership of the graph.\nAllocation and free nodes cannot be added to the returned graph.\nAttempting to do so will return an error.\n\n# Arguments\n\n* `node` -   - Node to get the embedded graph for\n* `pGraph` - - Location to store a handle to the graph\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue\n\\note_graph_thread_safety \\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaGraphAddChildGraphNode,`]\n::cudaGraphNodeFindInClone"]
pub unsafe fn cudaGraphChildGraphNodeGetGraph(node: cudaGraphNode_t) -> Result<cudaGraph_t, crate::sys::cudaError> {
    let mut out_1: std::mem::MaybeUninit<cudaGraph_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaGraphChildGraphNodeGetGraph(node, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_1.assume_init() as cudaGraph_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Creates an empty node and adds it to a graph\nCreates a new node which performs no operation, and adds it to `graph` with\n`numDependencies` dependencies specified via `pDependencies.`\nIt is possible for `numDependencies` to be 0, in which case the node will be placed\nat the root of the graph. `pDependencies` may not have any duplicate entries.\nA handle to the new node will be returned in `pGraphNode.`\nAn empty node performs no operation during execution, but can be used for\ntransitive ordering. For example, a phased execution graph with 2 groups of n\nnodes with a barrier between them can be represented using an empty node and\n2*n dependency edges, rather than no empty node and n^2 dependency edges.\n\n# Arguments\n\n* `pGraphNode` -     - Returns newly created node\n* `graph` -          - Graph to which to add the node\n* `pDependencies` -    - Dependencies of the node\n* `numDependencies` - - Number of dependencies\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue\n\\note_graph_thread_safety \\note_init_rt \\note_callback # See also\n\n> [`::cudaGraphAddNode,`]\n::cudaGraphCreate,\n::cudaGraphDestroyNode,\n::cudaGraphAddChildGraphNode,\n::cudaGraphAddKernelNode,\n::cudaGraphAddHostNode,\n::cudaGraphAddMemcpyNode,\n::cudaGraphAddMemsetNode"]
pub unsafe fn cudaGraphAddEmptyNode<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    mut pGraphNode: T,
    graph: cudaGraph_t,
    pDependencies: U,
    numDependencies: usize,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphAddEmptyNode(
            pGraphNode.as_mut_ptr() as *mut _,
            graph,
            pDependencies.as_const_ptr() as *const _,
            numDependencies,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudaGraphAddEventRecordNode<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    mut pGraphNode: T,
    graph: cudaGraph_t,
    pDependencies: U,
    numDependencies: usize,
    event: cudaEvent_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphAddEventRecordNode(
            pGraphNode.as_mut_ptr() as *mut _,
            graph,
            pDependencies.as_const_ptr() as *const _,
            numDependencies,
            event,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudaGraphEventRecordNodeGetEvent(node: cudaGraphNode_t) -> Result<cudaEvent_t, crate::sys::cudaError> {
    let mut out_1: std::mem::MaybeUninit<cudaEvent_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaGraphEventRecordNodeGetEvent(node, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_1.assume_init() as cudaEvent_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudaGraphEventRecordNodeSetEvent(
    node: cudaGraphNode_t,
    event: cudaEvent_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaGraphEventRecordNodeSetEvent(node, event) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudaGraphAddEventWaitNode<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    mut pGraphNode: T,
    graph: cudaGraph_t,
    pDependencies: U,
    numDependencies: usize,
    event: cudaEvent_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphAddEventWaitNode(
            pGraphNode.as_mut_ptr() as *mut _,
            graph,
            pDependencies.as_const_ptr() as *const _,
            numDependencies,
            event,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudaGraphEventWaitNodeGetEvent(node: cudaGraphNode_t) -> Result<cudaEvent_t, crate::sys::cudaError> {
    let mut out_1: std::mem::MaybeUninit<cudaEvent_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaGraphEventWaitNodeGetEvent(node, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_1.assume_init() as cudaEvent_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudaGraphEventWaitNodeSetEvent(
    node: cudaGraphNode_t,
    event: cudaEvent_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaGraphEventWaitNodeSetEvent(node, event) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudaGraphAddExternalSemaphoresSignalNode<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
>(
    mut pGraphNode: T,
    graph: cudaGraph_t,
    pDependencies: U,
    numDependencies: usize,
    nodeParams: V,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphAddExternalSemaphoresSignalNode(
            pGraphNode.as_mut_ptr() as *mut _,
            graph,
            pDependencies.as_const_ptr() as *const _,
            numDependencies,
            nodeParams.as_const_ptr() as *const _,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudaGraphExternalSemaphoresSignalNodeGetParams(
    hNode: cudaGraphNode_t,
) -> Result<cudaExternalSemaphoreSignalNodeParams, crate::sys::cudaError> {
    let mut out_1: std::mem::MaybeUninit<cudaExternalSemaphoreSignalNodeParams> = std::mem::MaybeUninit::uninit();
    let status =
        unsafe { crate::sys::cudaGraphExternalSemaphoresSignalNodeGetParams(hNode, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_1.assume_init() as cudaExternalSemaphoreSignalNodeParams) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudaGraphExternalSemaphoresSignalNodeSetParams<T: types::CudaAsPtr>(
    hNode: cudaGraphNode_t,
    nodeParams: T,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphExternalSemaphoresSignalNodeSetParams(hNode, nodeParams.as_const_ptr() as *const _)
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudaGraphAddExternalSemaphoresWaitNode<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    mut pGraphNode: T,
    graph: cudaGraph_t,
    pDependencies: U,
    numDependencies: usize,
    nodeParams: V,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphAddExternalSemaphoresWaitNode(
            pGraphNode.as_mut_ptr() as *mut _,
            graph,
            pDependencies.as_const_ptr() as *const _,
            numDependencies,
            nodeParams.as_const_ptr() as *const _,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudaGraphExternalSemaphoresWaitNodeGetParams(
    hNode: cudaGraphNode_t,
) -> Result<cudaExternalSemaphoreWaitNodeParams, crate::sys::cudaError> {
    let mut out_1: std::mem::MaybeUninit<cudaExternalSemaphoreWaitNodeParams> = std::mem::MaybeUninit::uninit();
    let status =
        unsafe { crate::sys::cudaGraphExternalSemaphoresWaitNodeGetParams(hNode, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_1.assume_init() as cudaExternalSemaphoreWaitNodeParams) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudaGraphExternalSemaphoresWaitNodeSetParams<T: types::CudaAsPtr>(
    hNode: cudaGraphNode_t,
    nodeParams: T,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphExternalSemaphoresWaitNodeSetParams(hNode, nodeParams.as_const_ptr() as *const _)
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudaGraphAddMemAllocNode<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    mut pGraphNode: T,
    graph: cudaGraph_t,
    pDependencies: U,
    numDependencies: usize,
    mut nodeParams: V,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphAddMemAllocNode(
            pGraphNode.as_mut_ptr() as *mut _,
            graph,
            pDependencies.as_const_ptr() as *const _,
            numDependencies,
            nodeParams.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudaGraphMemAllocNodeGetParams(
    node: cudaGraphNode_t,
) -> Result<cudaMemAllocNodeParams, crate::sys::cudaError> {
    let mut out_1: std::mem::MaybeUninit<cudaMemAllocNodeParams> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaGraphMemAllocNodeGetParams(node, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_1.assume_init() as cudaMemAllocNodeParams) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudaGraphAddMemFreeNode<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    mut pGraphNode: T,
    graph: cudaGraph_t,
    pDependencies: U,
    numDependencies: usize,
    mut dptr: V,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphAddMemFreeNode(
            pGraphNode.as_mut_ptr() as *mut _,
            graph,
            pDependencies.as_const_ptr() as *const _,
            numDependencies,
            dptr.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudaGraphMemFreeNodeGetParams<T: types::CudaAsPtr>(
    node: cudaGraphNode_t,
    mut dptr_out: T,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaGraphMemFreeNodeGetParams(node, dptr_out.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudaDeviceGraphMemTrim(device: i32) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaDeviceGraphMemTrim(device as _) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudaDeviceGetGraphMemAttribute<T: types::CudaAsPtr>(
    device: i32,
    attr: cudaGraphMemAttributeType,
    mut value: T,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaDeviceGetGraphMemAttribute(device as _, attr, value.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudaDeviceSetGraphMemAttribute<T: types::CudaAsPtr>(
    device: i32,
    attr: cudaGraphMemAttributeType,
    mut value: T,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaDeviceSetGraphMemAttribute(device as _, attr, value.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Clones a graph\nThis function creates a copy of `originalGraph` and returns it in `pGraphClone.`\nAll parameters are copied into the cloned graph. The original graph may be modified\nafter this call without affecting the clone.\nChild graph nodes in the original graph are recursively copied into the clone.\n\\note: Cloning is not supported for graphs which contain memory allocation nodes,\nmemory free nodes, or conditional nodes.\n\n# Arguments\n\n* `pGraphClone` -  - Returns newly created cloned graph\n* `originalGraph` - - Graph to clone\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorMemoryAllocation\n\\note_graph_thread_safety \\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaGraphCreate,`]\n::cudaGraphNodeFindInClone"]
pub unsafe fn cudaGraphClone<T: types::CudaAsPtr>(
    mut pGraphClone: T,
    originalGraph: cudaGraph_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaGraphClone(pGraphClone.as_mut_ptr() as *mut _, originalGraph) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Finds a cloned version of a node\nThis function returns the node in `clonedGraph` corresponding to `originalNode`\nin the original graph.\n`clonedGraph` must have been cloned from `originalGraph` via ::cudaGraphClone.\n`originalNode` must have been in `originalGraph` at the time of the call to\n::cudaGraphClone, and the corresponding cloned node in `clonedGraph` must not have\nbeen removed. The cloned node is then returned via `pClonedNode.`\n\n# Arguments\n\n* `pNode` -  - Returns handle to the cloned node\n* `originalNode` - - Handle to the original node\n* `clonedGraph` - - Cloned graph to query\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue\n\\note_graph_thread_safety \\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaGraphClone`]"]
pub unsafe fn cudaGraphNodeFindInClone<T: types::CudaAsPtr>(
    mut pNode: T,
    originalNode: cudaGraphNode_t,
    clonedGraph: cudaGraph_t,
) -> Result<(), crate::sys::cudaError> {
    let status =
        unsafe { crate::sys::cudaGraphNodeFindInClone(pNode.as_mut_ptr() as *mut _, originalNode, clonedGraph) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Returns a node's type\nReturns the node type of `node` in `pType.`\n\n# Arguments\n\n* `node` - - Node to query\n* `pType` -  - Pointer to return the node type\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue\n\\note_graph_thread_safety \\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaGraphGetNodes,`]\n::cudaGraphGetRootNodes,\n::cudaGraphChildGraphNodeGetGraph,\n::cudaGraphKernelNodeGetParams,\n::cudaGraphKernelNodeSetParams,\n::cudaGraphHostNodeGetParams,\n::cudaGraphHostNodeSetParams,\n::cudaGraphMemcpyNodeGetParams,\n::cudaGraphMemcpyNodeSetParams,\n::cudaGraphMemsetNodeGetParams,\n::cudaGraphMemsetNodeSetParams"]
pub unsafe fn cudaGraphNodeGetType(node: cudaGraphNode_t) -> Result<cudaGraphNodeType, crate::sys::cudaError> {
    let mut out_1: std::mem::MaybeUninit<cudaGraphNodeType> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaGraphNodeGetType(node, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_1.assume_init() as cudaGraphNodeType) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Returns the graph that contains a given graph node\nReturns the graph that contains `hNode` in `*phGraph.`\nIf hNode is in a child graph, the child graph it is in is returned.\n\n# Arguments\n\n* `hNode` - - Node to query\n* `phGraph` - - Pointer to return the containing graph\n\n# Returns\n\n::cudaSuccess\n::cudaErrorInvalidValue\n\n# See also\n\n> [`::cudaGraphGetNodes,`]\n::cudaGraphDebugDotPrint\n::cudaGraphNodeGetLocalId\n::cudaGraphNodeGetToolsId\n::cudaGraphGetId\n::cudaGraphExecGetId"]
pub unsafe fn cudaGraphNodeGetContainingGraph(hNode: cudaGraphNode_t) -> Result<cudaGraph_t, crate::sys::cudaError> {
    let mut out_1: std::mem::MaybeUninit<cudaGraph_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaGraphNodeGetContainingGraph(hNode, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_1.assume_init() as cudaGraph_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Returns the node id of a given graph node\nReturns the node id of `hNode` in `*nodeId.`\nThe nodeId matches that referenced by ::cudaGraphDebugDotPrint.\nThe local nodeId and graphId together can uniquely identify the node.\n\n# Arguments\n\n* `hNode` - - Node to query\n* `nodeId` - - Pointer to return the nodeId\n\n# Returns\n\n::cudaSuccess\n::cudaErrorInvalidValue\n\n# See also\n\n> [`::cudaGraphGetNodes,`]\n::cudaGraphDebugDotPrint\n::cudaGraphNodeGetContainingGraph\n::cudaGraphNodeGetToolsId\n::cudaGraphGetId\n::cudaGraphExecGetId"]
pub unsafe fn cudaGraphNodeGetLocalId(hNode: cudaGraphNode_t) -> Result<u32, crate::sys::cudaError> {
    let mut out_1: std::mem::MaybeUninit<::std::os::raw::c_uint> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaGraphNodeGetLocalId(hNode, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_1.assume_init() as u32) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Returns an id used by tools to identify a given node\n\n# Arguments\n\n* `hNode` - - Node to query\n* `*toolsNodeId` - - Pointer to return the id used by tools\n\n# Returns\n\n::CUDA_SUCCESS\n::cudaErrorInvalidValue\n\n# See also\n\n> [`::cudaGraphGetNodes,`]\n::cudaGraphDebugDotPrint\n::cudaGraphNodeGetContainingGraph\n::cudaGraphNodeGetLocalId\n::cudaGraphGetId\n::cudaGraphExecGetId\n"]
pub unsafe fn cudaGraphNodeGetToolsId(hNode: cudaGraphNode_t) -> Result<u64, crate::sys::cudaError> {
    let mut out_1: std::mem::MaybeUninit<::std::os::raw::c_ulonglong> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaGraphNodeGetToolsId(hNode, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_1.assume_init() as u64) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Returns the id of a given graph\nReturns the id of `hGraph` in `*graphId.`\nThe value in `*graphId` matches that referenced by ::cudaGraphDebugDotPrint.\n\n# Arguments\n\n* `hGraph` - - Graph to query\n* `graphId` - - Pointer to return the graphId\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue\n\n# See also\n\n> [`::cudaGraphGetNodes,`]\n::cudaGraphDebugDotPrint\n::cudaGraphNodeGetContainingGraph\n::cudaGraphNodeGetLocalId\n::cudaGraphNodeGetToolsId\n::cudaGraphExecGetId"]
pub unsafe fn cudaGraphGetId(hGraph: cudaGraph_t) -> Result<u32, crate::sys::cudaError> {
    let mut out_1: std::mem::MaybeUninit<::std::os::raw::c_uint> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaGraphGetId(hGraph, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_1.assume_init() as u32) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Returns the id of a given graph exec\nReturns the id of `hGraphExec` in `*graphId.`\nThe value in `*graphId` matches that referenced by ::cudaGraphDebugDotPrint.\n\n# Arguments\n\n* `hGraphExec` - - Graph to query\n* `graphId` - - Pointer to return the graphId\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue\n\n# See also\n\n> [`::cudaGraphGetNodes,`]\n::cudaGraphDebugDotPrint\n::cudaGraphNodeGetContainingGraph\n::cudaGraphNodeGetLocalId\n::cudaGraphNodeGetToolsId\n::cudaGraphGetId"]
pub unsafe fn cudaGraphExecGetId(hGraphExec: cudaGraphExec_t) -> Result<u32, crate::sys::cudaError> {
    let mut out_1: std::mem::MaybeUninit<::std::os::raw::c_uint> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaGraphExecGetId(hGraphExec, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_1.assume_init() as u32) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Returns a graph's nodes\nReturns a list of `graph's` nodes. `nodes` may be NULL, in which case this\nfunction will return the number of nodes in `numNodes.` Otherwise,\n`numNodes` entries will be filled in. If `numNodes` is higher than the actual\nnumber of nodes, the remaining entries in `nodes` will be set to NULL, and the\nnumber of nodes actually obtained will be returned in `numNodes.`\n\n# Arguments\n\n* `graph` -    - Graph to query\n* `nodes` -    - Pointer to return the nodes\n* `numNodes` - - See description\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue\n\\note_graph_thread_safety \\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaGraphCreate,`]\n::cudaGraphGetRootNodes,\n::cudaGraphGetEdges,\n::cudaGraphNodeGetType,\n::cudaGraphNodeGetDependencies,\n::cudaGraphNodeGetDependentNodes"]
pub unsafe fn cudaGraphGetNodes(graph: cudaGraph_t) -> Result<(cudaGraphNode_t, usize), crate::sys::cudaError> {
    let mut out_1: std::mem::MaybeUninit<cudaGraphNode_t> = std::mem::MaybeUninit::uninit();
    let mut out_2: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let status =
        unsafe { crate::sys::cudaGraphGetNodes(graph, out_1.as_mut_ptr() as *mut _, out_2.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok((out_1.assume_init() as cudaGraphNode_t, out_2.assume_init() as usize)) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Returns a graph's root nodes\nReturns a list of `graph's` root nodes. `pRootNodes` may be NULL, in which case this\nfunction will return the number of root nodes in `pNumRootNodes.` Otherwise,\n`pNumRootNodes` entries will be filled in. If `pNumRootNodes` is higher than the actual\nnumber of root nodes, the remaining entries in `pRootNodes` will be set to NULL, and the\nnumber of nodes actually obtained will be returned in `pNumRootNodes.`\n\n# Arguments\n\n* `graph` -       - Graph to query\n* `pRootNodes` -    - Pointer to return the root nodes\n* `pNumRootNodes` - - See description\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue\n\\note_graph_thread_safety \\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaGraphCreate,`]\n::cudaGraphGetNodes,\n::cudaGraphGetEdges,\n::cudaGraphNodeGetType,\n::cudaGraphNodeGetDependencies,\n::cudaGraphNodeGetDependentNodes"]
pub unsafe fn cudaGraphGetRootNodes(graph: cudaGraph_t) -> Result<(cudaGraphNode_t, usize), crate::sys::cudaError> {
    let mut out_1: std::mem::MaybeUninit<cudaGraphNode_t> = std::mem::MaybeUninit::uninit();
    let mut out_2: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let status =
        unsafe { crate::sys::cudaGraphGetRootNodes(graph, out_1.as_mut_ptr() as *mut _, out_2.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok((out_1.assume_init() as cudaGraphNode_t, out_2.assume_init() as usize)) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Returns a graph's dependency edges\nReturns a list of `graph's` dependency edges. Edges are returned via corresponding\nindices in `from,` `to` and `edgeData;` that is, the node in `to`[i] has a\ndependency on the node in `from`[i] with data `edgeData`[i]. `from` and `to` may\nboth be NULL, in which case this function only returns the number of edges in\n`numEdges.` Otherwise, `numEdges` entries will be filled in. If `numEdges` is higher\nthan the actual number of edges, the remaining entries in `from` and `to` will be\nset to NULL, and the number of edges actually returned will be written to `numEdges.`\n`edgeData` may alone be NULL, in which case the edges must all have default (zeroed)\nedge data. Attempting a losst query via NULL `edgeData` will result in\n::cudaErrorLossyQuery. If `edgeData` is non-NULL then `from` and `to` must be as\nwell.\n\n# Arguments\n\n* `graph` -    - Graph to get the edges from\n* `from` -     - Location to return edge endpoints\n* `to` -       - Location to return edge endpoints\n* `edgeData` - - Optional location to return edge data\n* `numEdges` - - See description\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorLossyQuery,\n::cudaErrorInvalidValue\n\\note_graph_thread_safety \\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaGraphGetNodes,`]\n::cudaGraphGetRootNodes,\n::cudaGraphAddDependencies,\n::cudaGraphRemoveDependencies,\n::cudaGraphNodeGetDependencies,\n::cudaGraphNodeGetDependentNodes"]
pub unsafe fn cudaGraphGetEdges(
    graph: cudaGraph_t,
) -> Result<(cudaGraphNode_t, cudaGraphNode_t, cudaGraphEdgeData, usize), crate::sys::cudaError> {
    let mut out_1: std::mem::MaybeUninit<cudaGraphNode_t> = std::mem::MaybeUninit::uninit();
    let mut out_2: std::mem::MaybeUninit<cudaGraphNode_t> = std::mem::MaybeUninit::uninit();
    let mut out_3: std::mem::MaybeUninit<cudaGraphEdgeData> = std::mem::MaybeUninit::uninit();
    let mut out_4: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudaGraphGetEdges(
            graph,
            out_1.as_mut_ptr() as *mut _,
            out_2.as_mut_ptr() as *mut _,
            out_3.as_mut_ptr() as *mut _,
            out_4.as_mut_ptr() as *mut _,
        )
    };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe {
            Ok((
                out_1.assume_init() as cudaGraphNode_t,
                out_2.assume_init() as cudaGraphNode_t,
                out_3.assume_init() as cudaGraphEdgeData,
                out_4.assume_init() as usize,
            ))
        }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Returns a node's dependencies\nReturns a list of `node's` dependencies. `pDependencies` may be NULL, in which case this\nfunction will return the number of dependencies in `pNumDependencies.` Otherwise,\n`pNumDependencies` entries will be filled in. If `pNumDependencies` is higher than the actual\nnumber of dependencies, the remaining entries in `pDependencies` will be set to NULL, and the\nnumber of nodes actually obtained will be returned in `pNumDependencies.`\nNote that if an edge has non-zero (non-default) edge data and `edgeData` is NULL,\nthis API will return ::cudaErrorLossyQuery. If `edgeData` is non-NULL, then\n`pDependencies` must be as well.\n\n# Arguments\n\n* `node` -             - Node to query\n* `pDependencies` -    - Pointer to return the dependencies\n* `edgeData` -         - Optional array to return edge data for each dependency\n* `pNumDependencies` - - See description\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorLossyQuery,\n::cudaErrorInvalidValue\n\\note_graph_thread_safety \\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaGraphNodeGetDependentNodes,`]\n::cudaGraphGetNodes,\n::cudaGraphGetRootNodes,\n::cudaGraphGetEdges,\n::cudaGraphAddDependencies,\n::cudaGraphRemoveDependencies"]
pub unsafe fn cudaGraphNodeGetDependencies(
    node: cudaGraphNode_t,
) -> Result<(cudaGraphNode_t, cudaGraphEdgeData, usize), crate::sys::cudaError> {
    let mut out_1: std::mem::MaybeUninit<cudaGraphNode_t> = std::mem::MaybeUninit::uninit();
    let mut out_2: std::mem::MaybeUninit<cudaGraphEdgeData> = std::mem::MaybeUninit::uninit();
    let mut out_3: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudaGraphNodeGetDependencies(
            node,
            out_1.as_mut_ptr() as *mut _,
            out_2.as_mut_ptr() as *mut _,
            out_3.as_mut_ptr() as *mut _,
        )
    };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe {
            Ok((
                out_1.assume_init() as cudaGraphNode_t,
                out_2.assume_init() as cudaGraphEdgeData,
                out_3.assume_init() as usize,
            ))
        }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Returns a node's dependent nodes\nReturns a list of `node's` dependent nodes. `pDependentNodes` may be NULL, in which\ncase this function will return the number of dependent nodes in `pNumDependentNodes.`\nOtherwise, `pNumDependentNodes` entries will be filled in. If `pNumDependentNodes` is\nhigher than the actual number of dependent nodes, the remaining entries in\n`pDependentNodes` will be set to NULL, and the number of nodes actually obtained will\nbe returned in `pNumDependentNodes.`\nNote that if an edge has non-zero (non-default) edge data and `edgeData` is NULL,\nthis API will return ::cudaErrorLossyQuery. If `edgeData` is non-NULL, then\n`pDependentNodes` must be as well.\n\n# Arguments\n\n* `node` -               - Node to query\n* `pDependentNodes` -    - Pointer to return the dependent nodes\n* `edgeData` -           - Optional pointer to return edge data for dependent nodes\n* `pNumDependentNodes` - - See description\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorLossyQuery,\n::cudaErrorInvalidValue\n\\note_graph_thread_safety \\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaGraphNodeGetDependencies,`]\n::cudaGraphGetNodes,\n::cudaGraphGetRootNodes,\n::cudaGraphGetEdges,\n::cudaGraphAddDependencies,\n::cudaGraphRemoveDependencies"]
pub unsafe fn cudaGraphNodeGetDependentNodes(
    node: cudaGraphNode_t,
) -> Result<(cudaGraphNode_t, cudaGraphEdgeData, usize), crate::sys::cudaError> {
    let mut out_1: std::mem::MaybeUninit<cudaGraphNode_t> = std::mem::MaybeUninit::uninit();
    let mut out_2: std::mem::MaybeUninit<cudaGraphEdgeData> = std::mem::MaybeUninit::uninit();
    let mut out_3: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudaGraphNodeGetDependentNodes(
            node,
            out_1.as_mut_ptr() as *mut _,
            out_2.as_mut_ptr() as *mut _,
            out_3.as_mut_ptr() as *mut _,
        )
    };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe {
            Ok((
                out_1.assume_init() as cudaGraphNode_t,
                out_2.assume_init() as cudaGraphEdgeData,
                out_3.assume_init() as usize,
            ))
        }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Adds dependency edges to a graph.\nThe number of dependencies to be added is defined by `numDependencies`\nElements in `pFrom` and `pTo` at corresponding indices define a dependency.\nEach node in `pFrom` and `pTo` must belong to `graph.`\nIf `numDependencies` is 0, elements in `pFrom` and `pTo` will be ignored.\nSpecifying an existing dependency will return an error.\n\n# Arguments\n\n* `graph` - - Graph to which dependencies are added\n* `from` - - Array of nodes that provide the dependencies\n* `to` - - Array of dependent nodes\n* `edgeData` - - Optional array of edge data. If NULL, default (zeroed) edge data is assumed.\n* `numDependencies` - - Number of dependencies to be added\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue\n\\note_graph_thread_safety \\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaGraphRemoveDependencies,`]\n::cudaGraphGetEdges,\n::cudaGraphNodeGetDependencies,\n::cudaGraphNodeGetDependentNodes"]
pub unsafe fn cudaGraphAddDependencies<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    graph: cudaGraph_t,
    from: T,
    to: U,
    edgeData: V,
    numDependencies: usize,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphAddDependencies(
            graph,
            from.as_const_ptr() as *const _,
            to.as_const_ptr() as *const _,
            edgeData.as_const_ptr() as *const _,
            numDependencies,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Removes dependency edges from a graph.\nThe number of `pDependencies` to be removed is defined by `numDependencies.`\nElements in `pFrom` and `pTo` at corresponding indices define a dependency.\nEach node in `pFrom` and `pTo` must belong to `graph.`\nIf `numDependencies` is 0, elements in `pFrom` and `pTo` will be ignored.\nSpecifying an edge that does not exist in the graph, with data matching\n`edgeData,` results in an error. `edgeData` is nullable, which is equivalent\nto passing default (zeroed) data for each edge.\n\n# Arguments\n\n* `graph` - - Graph from which to remove dependencies\n* `from` - - Array of nodes that provide the dependencies\n* `to` - - Array of dependent nodes\n* `edgeData` - - Optional array of edge data. If NULL, edge data is assumed to\nbe default (zeroed).\n* `numDependencies` - - Number of dependencies to be removed\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue\n\\note_graph_thread_safety \\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaGraphAddDependencies,`]\n::cudaGraphGetEdges,\n::cudaGraphNodeGetDependencies,\n::cudaGraphNodeGetDependentNodes"]
pub unsafe fn cudaGraphRemoveDependencies<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr>(
    graph: cudaGraph_t,
    from: T,
    to: U,
    edgeData: V,
    numDependencies: usize,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphRemoveDependencies(
            graph,
            from.as_const_ptr() as *const _,
            to.as_const_ptr() as *const _,
            edgeData.as_const_ptr() as *const _,
            numDependencies,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Creates an executable graph from a graph\nInstantiates `graph` as an executable graph. The graph is validated for any\nstructural constraints or intra-node constraints which were not previously\nvalidated. If instantiation is successful, a handle to the instantiated graph\nis returned in `pGraphExec.`\nThe `flags` parameter controls the behavior of instantiation and subsequent\ngraph launches.  Valid flags are:\n- ::cudaGraphInstantiateFlagAutoFreeOnLaunch, which configures a\ngraph containing memory allocation nodes to automatically free any\nunfreed memory allocations before the graph is relaunched.\n- ::cudaGraphInstantiateFlagDeviceLaunch, which configures the graph for launch\nfrom the device. If this flag is passed, the executable graph handle returned can be\nused to launch the graph from both the host and device. This flag cannot be used in\nconjunction with ::cudaGraphInstantiateFlagAutoFreeOnLaunch.\n- ::cudaGraphInstantiateFlagUseNodePriority, which causes the graph\nto use the priorities from the per-node attributes rather than the priority\nof the launch stream during execution. Note that priorities are only available\non kernel nodes, and are copied from stream priority during stream capture.\nIf `graph` contains any allocation or free nodes, there can be at most one\nexecutable graph in existence for that graph at a time. An attempt to\ninstantiate a second executable graph before destroying the first with\n::cudaGraphExecDestroy will result in an error.\nThe same also applies if `graph` contains any device-updatable kernel nodes.\nGraphs instantiated for launch on the device have additional restrictions which do not\napply to host graphs:\n- The graph's nodes must reside on a single device.\n- The graph can only contain kernel nodes, memcpy nodes, memset nodes, and child graph nodes.\n- The graph cannot be empty and must contain at least one kernel, memcpy, or memset node.\nOperation-specific restrictions are outlined below.\n- Kernel nodes:\n- Use of CUDA Dynamic Parallelism is not permitted.\n- Cooperative launches are permitted as long as MPS is not in use.\n- Memcpy nodes:\n- Only copies involving device memory and/or pinned device-mapped host memory are permitted.\n- Copies involving CUDA arrays are not permitted.\n- Both operands must be accessible from the current device, and the current device must\nmatch the device of other nodes in the graph.\nIf `graph` is not instantiated for launch on the device but contains kernels which\ncall device-side cudaGraphLaunch() from multiple devices, this will result in an error.\n\n# Arguments\n\n* `pGraphExec` - - Returns instantiated graph\n* `graph` -      - Graph to instantiate\n* `flags` -      - Flags to control instantiation.  See ::CUgraphInstantiate_flags.\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue\n\\note_graph_thread_safety \\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaGraphInstantiateWithFlags,`]\n::cudaGraphCreate,\n::cudaGraphUpload,\n::cudaGraphLaunch,\n::cudaGraphExecDestroy"]
pub unsafe fn cudaGraphInstantiate<T: types::CudaAsPtr>(
    mut pGraphExec: T,
    graph: cudaGraph_t,
    flags: u64,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaGraphInstantiate(pGraphExec.as_mut_ptr() as *mut _, graph, flags as _) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudaGraphInstantiateWithFlags<T: types::CudaAsPtr>(
    mut pGraphExec: T,
    graph: cudaGraph_t,
    flags: u64,
) -> Result<(), crate::sys::cudaError> {
    let status =
        unsafe { crate::sys::cudaGraphInstantiateWithFlags(pGraphExec.as_mut_ptr() as *mut _, graph, flags as _) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Creates an executable graph from a graph\nInstantiates `graph` as an executable graph according to the `instantiateParams` structure.\nThe graph is validated for any structural constraints or intra-node constraints\nwhich were not previously validated. If instantiation is successful, a handle to\nthe instantiated graph is returned in `pGraphExec.`\n`instantiateParams` controls the behavior of instantiation and subsequent\ngraph launches, as well as returning more detailed information in the event of an error.\n::cudaGraphInstantiateParams is defined as:\n\\code typedef struct {\nunsigned long long flags;\ncudaStream_t uploadStream;\ncudaGraphNode_t errNode_out;\ncudaGraphInstantiateResult result_out;\n} cudaGraphInstantiateParams;\n\\endcode The `flags` field controls the behavior of instantiation and subsequent\ngraph launches. Valid flags are:\n- ::cudaGraphInstantiateFlagAutoFreeOnLaunch, which configures a\ngraph containing memory allocation nodes to automatically free any\nunfreed memory allocations before the graph is relaunched.\n- ::cudaGraphInstantiateFlagUpload, which will perform an upload of the graph\ninto `uploadStream` once the graph has been instantiated.\n- ::cudaGraphInstantiateFlagDeviceLaunch, which configures the graph for launch\nfrom the device. If this flag is passed, the executable graph handle returned can be\nused to launch the graph from both the host and device. This flag can only be used\non platforms which support unified addressing. This flag cannot be used in\nconjunction with ::cudaGraphInstantiateFlagAutoFreeOnLaunch.\n- ::cudaGraphInstantiateFlagUseNodePriority, which causes the graph\nto use the priorities from the per-node attributes rather than the priority\nof the launch stream during execution. Note that priorities are only available\non kernel nodes, and are copied from stream priority during stream capture.\nIf `graph` contains any allocation or free nodes, there can be at most one\nexecutable graph in existence for that graph at a time. An attempt to instantiate a\nsecond executable graph before destroying the first with ::cudaGraphExecDestroy will\nresult in an error.\nThe same also applies if `graph` contains any device-updatable kernel nodes.\nIf `graph` contains kernels which call device-side cudaGraphLaunch() from multiple\ndevices, this will result in an error.\nGraphs instantiated for launch on the device have additional restrictions which do not\napply to host graphs:\n- The graph's nodes must reside on a single device.\n- The graph can only contain kernel nodes, memcpy nodes, memset nodes, and child graph nodes.\n- The graph cannot be empty and must contain at least one kernel, memcpy, or memset node.\nOperation-specific restrictions are outlined below.\n- Kernel nodes:\n- Use of CUDA Dynamic Parallelism is not permitted.\n- Cooperative launches are permitted as long as MPS is not in use.\n- Memcpy nodes:\n- Only copies involving device memory and/or pinned device-mapped host memory are permitted.\n- Copies involving CUDA arrays are not permitted.\n- Both operands must be accessible from the current device, and the current device must\nmatch the device of other nodes in the graph.\nIn the event of an error, the `result_out` and `errNode_out` fields will contain more\ninformation about the nature of the error. Possible error reporting includes:\n- ::cudaGraphInstantiateError, if passed an invalid value or if an unexpected error occurred\nwhich is described by the return value of the function. `errNode_out` will be set to NULL.\n- ::cudaGraphInstantiateInvalidStructure, if the graph structure is invalid. `errNode_out`\nwill be set to one of the offending nodes.\n- ::cudaGraphInstantiateNodeOperationNotSupported, if the graph is instantiated for device\nlaunch but contains a node of an unsupported node type, or a node which performs unsupported\noperations, such as use of CUDA dynamic parallelism within a kernel node. `errNode_out` will\nbe set to this node.\n- ::cudaGraphInstantiateMultipleDevicesNotSupported, if the graph is instantiated for device\nlaunch but a node’s device differs from that of another node. This error can also be returned\nif a graph is not instantiated for device launch and it contains kernels which call device-side\ncudaGraphLaunch() from multiple devices. `errNode_out` will be set to this node.\nIf instantiation is successful, `result_out` will be set to ::cudaGraphInstantiateSuccess,\nand `hErrNode_out` will be set to NULL.\n\n# Arguments\n\n* `pGraphExec` -       - Returns instantiated graph\n* `graph` -            - Graph to instantiate\n* `instantiateParams` - - Instantiation parameters\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue\n\\note_graph_thread_safety \\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaGraphCreate,`]\n::cudaGraphInstantiate,\n::cudaGraphInstantiateWithFlags,\n::cudaGraphExecDestroy"]
pub unsafe fn cudaGraphInstantiateWithParams<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    mut pGraphExec: T,
    graph: cudaGraph_t,
    mut instantiateParams: U,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphInstantiateWithParams(
            pGraphExec.as_mut_ptr() as *mut _,
            graph,
            instantiateParams.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Query the instantiation flags of an executable graph\nReturns the flags that were passed to instantiation for the given executable graph.\n::cudaGraphInstantiateFlagUpload will not be returned by this API as it does\nnot affect the resulting executable graph.\n\n# Arguments\n\n* `graphExec` - - The executable graph to query\n* `flags` -     - Returns the instantiation flags\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue\n\\note_graph_thread_safety \\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaGraphInstantiate,`]\n::cudaGraphInstantiateWithFlags,\n::cudaGraphInstantiateWithParams"]
pub unsafe fn cudaGraphExecGetFlags(graphExec: cudaGraphExec_t) -> Result<u64, crate::sys::cudaError> {
    let mut out_1: std::mem::MaybeUninit<::std::os::raw::c_ulonglong> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaGraphExecGetFlags(graphExec, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_1.assume_init() as u64) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Sets the parameters for a kernel node in the given graphExec\nSets the parameters of a kernel node in an executable graph `hGraphExec.`\nThe node is identified by the corresponding node `node` in the\nnon-executable graph, from which the executable graph was instantiated.\n`node` must not have been removed from the original graph. All `nodeParams`\nfields may change, but the following restrictions apply to `func` updates:\n- The owning device of the function cannot change.\n- A node whose function originally did not use CUDA dynamic parallelism cannot be updated\nto a function which uses CDP\n- A node whose function originally did not make device-side update calls cannot be updated\nto a function which makes device-side update calls.\n- If `hGraphExec` was not instantiated for device launch, a node whose function originally\ndid not use device-side cudaGraphLaunch() cannot be updated to a function which uses\ndevice-side cudaGraphLaunch() unless the node resides on the same device as nodes which\ncontained such calls at instantiate-time. If no such calls were present at instantiation,\nthese updates cannot be performed at all.\nThe modifications only affect future launches of `hGraphExec.` Already\nenqueued or running launches of `hGraphExec` are not affected by this call.\n`node` is also not modified by this call.\nIf `node` is a device-updatable kernel node, the next upload/launch of `hGraphExec`\nwill overwrite any previous device-side updates. Additionally, applying host updates to a\ndevice-updatable kernel node while it is being updated from the device will result in\nundefined behavior.\n\n# Arguments\n\n* `hGraphExec` -  - The executable graph in which to set the specified node\n* `node` -        - kernel node from the graph from which graphExec was instantiated\n* `pNodeParams` - - Updated Parameters to set\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n\\note_graph_thread_safety \\notefnerr \\note_init_rt \\note_callback \\note_cudaKernel_t # See also\n\n> [`::cudaGraphExecNodeSetParams,`]\n::cudaGraphAddKernelNode,\n::cudaGraphKernelNodeSetParams,\n::cudaGraphExecMemcpyNodeSetParams,\n::cudaGraphExecMemsetNodeSetParams,\n::cudaGraphExecHostNodeSetParams,\n::cudaGraphExecChildGraphNodeSetParams,\n::cudaGraphExecEventRecordNodeSetEvent,\n::cudaGraphExecEventWaitNodeSetEvent,\n::cudaGraphExecExternalSemaphoresSignalNodeSetParams,\n::cudaGraphExecExternalSemaphoresWaitNodeSetParams,\n::cudaGraphExecUpdate,\n::cudaGraphInstantiate"]
pub unsafe fn cudaGraphExecKernelNodeSetParams<T: types::CudaAsPtr>(
    hGraphExec: cudaGraphExec_t,
    node: cudaGraphNode_t,
    pNodeParams: T,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphExecKernelNodeSetParams(hGraphExec, node, pNodeParams.as_const_ptr() as *const _)
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Sets the parameters for a memcpy node in the given graphExec.\nUpdates the work represented by `node` in `hGraphExec` as though `node` had\ncontained `pNodeParams` at instantiation.  `node` must remain in the graph which was\nused to instantiate `hGraphExec.`  Changed edges to and from `node` are ignored.\nThe source and destination memory in `pNodeParams` must be allocated from the same\ncontexts as the original source and destination memory.  Both the instantiation-time\nmemory operands and the memory operands in `pNodeParams` must be 1-dimensional.\nZero-length operations are not supported.\nThe modifications only affect future launches of `hGraphExec.`  Already enqueued\nor running launches of `hGraphExec` are not affected by this call.  `node` is also\nnot modified by this call.\nReturns ::cudaErrorInvalidValue if the memory operands' mappings changed or\neither the original or new memory operands are multidimensional.\n\n# Arguments\n\n* `hGraphExec` -  - The executable graph in which to set the specified node\n* `node` -        - Memcpy node from the graph which was used to instantiate graphExec\n* `pNodeParams` - - Updated Parameters to set\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n\\note_graph_thread_safety \\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaGraphExecNodeSetParams,`]\n::cudaGraphAddMemcpyNode,\n::cudaGraphMemcpyNodeSetParams,\n::cudaGraphExecMemcpyNodeSetParamsToSymbol,\n::cudaGraphExecMemcpyNodeSetParamsFromSymbol,\n::cudaGraphExecMemcpyNodeSetParams1D,\n::cudaGraphExecKernelNodeSetParams,\n::cudaGraphExecMemsetNodeSetParams,\n::cudaGraphExecHostNodeSetParams,\n::cudaGraphExecChildGraphNodeSetParams,\n::cudaGraphExecEventRecordNodeSetEvent,\n::cudaGraphExecEventWaitNodeSetEvent,\n::cudaGraphExecExternalSemaphoresSignalNodeSetParams,\n::cudaGraphExecExternalSemaphoresWaitNodeSetParams,\n::cudaGraphExecUpdate,\n::cudaGraphInstantiate"]
pub unsafe fn cudaGraphExecMemcpyNodeSetParams<T: types::CudaAsPtr>(
    hGraphExec: cudaGraphExec_t,
    node: cudaGraphNode_t,
    pNodeParams: T,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphExecMemcpyNodeSetParams(hGraphExec, node, pNodeParams.as_const_ptr() as *const _)
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudaGraphExecMemcpyNodeSetParamsToSymbol<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    hGraphExec: cudaGraphExec_t,
    node: cudaGraphNode_t,
    symbol: T,
    src: U,
    count: usize,
    offset: usize,
    kind: cudaMemcpyKind,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphExecMemcpyNodeSetParamsToSymbol(
            hGraphExec,
            node,
            symbol.as_const_ptr() as *const _,
            src.as_const_ptr() as *const _,
            count,
            offset,
            kind,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudaGraphExecMemcpyNodeSetParamsFromSymbol<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    hGraphExec: cudaGraphExec_t,
    node: cudaGraphNode_t,
    mut dst: T,
    symbol: U,
    count: usize,
    offset: usize,
    kind: cudaMemcpyKind,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphExecMemcpyNodeSetParamsFromSymbol(
            hGraphExec,
            node,
            dst.as_mut_ptr() as *mut _,
            symbol.as_const_ptr() as *const _,
            count,
            offset,
            kind,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudaGraphExecMemcpyNodeSetParams1D<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    hGraphExec: cudaGraphExec_t,
    node: cudaGraphNode_t,
    mut dst: T,
    src: U,
    count: usize,
    kind: cudaMemcpyKind,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphExecMemcpyNodeSetParams1D(
            hGraphExec,
            node,
            dst.as_mut_ptr() as *mut _,
            src.as_const_ptr() as *const _,
            count,
            kind,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Sets the parameters for a memset node in the given graphExec.\nUpdates the work represented by `node` in `hGraphExec` as though `node` had\ncontained `pNodeParams` at instantiation.  `node` must remain in the graph which was\nused to instantiate `hGraphExec.`  Changed edges to and from `node` are ignored.\nZero sized operations are not supported.\nThe new destination pointer in `pNodeParams` must be to the same kind of allocation\nas the original destination pointer and have the same context association and device mapping\nas the original destination pointer.\nBoth the value and pointer address may be updated.\nChanging other aspects of the memset (width, height, element size or pitch) may cause the update to be rejected.\nSpecifically, for 2d memsets, all dimension changes are rejected.\nFor 1d memsets, changes in height are explicitly rejected and other changes are opportunistically allowed\nif the resulting work maps onto the work resources already allocated for the node.\nThe modifications only affect future launches of `hGraphExec.`  Already enqueued\nor running launches of `hGraphExec` are not affected by this call.  `node` is also\nnot modified by this call.\n\n# Arguments\n\n* `hGraphExec` -  - The executable graph in which to set the specified node\n* `node` -        - Memset node from the graph which was used to instantiate graphExec\n* `pNodeParams` - - Updated Parameters to set\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n\\note_graph_thread_safety \\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaGraphExecNodeSetParams,`]\n::cudaGraphAddMemsetNode,\n::cudaGraphMemsetNodeSetParams,\n::cudaGraphExecKernelNodeSetParams,\n::cudaGraphExecMemcpyNodeSetParams,\n::cudaGraphExecHostNodeSetParams,\n::cudaGraphExecChildGraphNodeSetParams,\n::cudaGraphExecEventRecordNodeSetEvent,\n::cudaGraphExecEventWaitNodeSetEvent,\n::cudaGraphExecExternalSemaphoresSignalNodeSetParams,\n::cudaGraphExecExternalSemaphoresWaitNodeSetParams,\n::cudaGraphExecUpdate,\n::cudaGraphInstantiate"]
pub unsafe fn cudaGraphExecMemsetNodeSetParams<T: types::CudaAsPtr>(
    hGraphExec: cudaGraphExec_t,
    node: cudaGraphNode_t,
    pNodeParams: T,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphExecMemsetNodeSetParams(hGraphExec, node, pNodeParams.as_const_ptr() as *const _)
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Sets the parameters for a host node in the given graphExec.\nUpdates the work represented by `node` in `hGraphExec` as though `node` had\ncontained `pNodeParams` at instantiation.  `node` must remain in the graph which was\nused to instantiate `hGraphExec.`  Changed edges to and from `node` are ignored.\nThe modifications only affect future launches of `hGraphExec.`  Already enqueued\nor running launches of `hGraphExec` are not affected by this call.  `node` is also\nnot modified by this call.\n\n# Arguments\n\n* `hGraphExec` -  - The executable graph in which to set the specified node\n* `node` -        - Host node from the graph which was used to instantiate graphExec\n* `pNodeParams` - - Updated Parameters to set\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n\\note_graph_thread_safety \\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaGraphExecNodeSetParams,`]\n::cudaGraphAddHostNode,\n::cudaGraphHostNodeSetParams,\n::cudaGraphExecKernelNodeSetParams,\n::cudaGraphExecMemcpyNodeSetParams,\n::cudaGraphExecMemsetNodeSetParams,\n::cudaGraphExecChildGraphNodeSetParams,\n::cudaGraphExecEventRecordNodeSetEvent,\n::cudaGraphExecEventWaitNodeSetEvent,\n::cudaGraphExecExternalSemaphoresSignalNodeSetParams,\n::cudaGraphExecExternalSemaphoresWaitNodeSetParams,\n::cudaGraphExecUpdate,\n::cudaGraphInstantiate"]
pub unsafe fn cudaGraphExecHostNodeSetParams<T: types::CudaAsPtr>(
    hGraphExec: cudaGraphExec_t,
    node: cudaGraphNode_t,
    pNodeParams: T,
) -> Result<(), crate::sys::cudaError> {
    let status =
        unsafe { crate::sys::cudaGraphExecHostNodeSetParams(hGraphExec, node, pNodeParams.as_const_ptr() as *const _) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudaGraphExecChildGraphNodeSetParams(
    hGraphExec: cudaGraphExec_t,
    node: cudaGraphNode_t,
    childGraph: cudaGraph_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaGraphExecChildGraphNodeSetParams(hGraphExec, node, childGraph) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudaGraphExecEventRecordNodeSetEvent(
    hGraphExec: cudaGraphExec_t,
    hNode: cudaGraphNode_t,
    event: cudaEvent_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaGraphExecEventRecordNodeSetEvent(hGraphExec, hNode, event) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudaGraphExecEventWaitNodeSetEvent(
    hGraphExec: cudaGraphExec_t,
    hNode: cudaGraphNode_t,
    event: cudaEvent_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaGraphExecEventWaitNodeSetEvent(hGraphExec, hNode, event) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudaGraphExecExternalSemaphoresSignalNodeSetParams<T: types::CudaAsPtr>(
    hGraphExec: cudaGraphExec_t,
    hNode: cudaGraphNode_t,
    nodeParams: T,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphExecExternalSemaphoresSignalNodeSetParams(
            hGraphExec,
            hNode,
            nodeParams.as_const_ptr() as *const _,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudaGraphExecExternalSemaphoresWaitNodeSetParams<T: types::CudaAsPtr>(
    hGraphExec: cudaGraphExec_t,
    hNode: cudaGraphNode_t,
    nodeParams: T,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphExecExternalSemaphoresWaitNodeSetParams(
            hGraphExec,
            hNode,
            nodeParams.as_const_ptr() as *const _,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudaGraphNodeSetEnabled(
    hGraphExec: cudaGraphExec_t,
    hNode: cudaGraphNode_t,
    isEnabled: u32,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaGraphNodeSetEnabled(hGraphExec, hNode, isEnabled as _) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudaGraphNodeGetEnabled(
    hGraphExec: cudaGraphExec_t,
    hNode: cudaGraphNode_t,
) -> Result<u32, crate::sys::cudaError> {
    let mut out_2: std::mem::MaybeUninit<::std::os::raw::c_uint> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaGraphNodeGetEnabled(hGraphExec, hNode, out_2.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_2.assume_init() as u32) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Check whether an executable graph can be updated with a graph and perform the update if possible\nUpdates the node parameters in the instantiated graph specified by `hGraphExec` with the\nnode parameters in a topologically identical graph specified by `hGraph.`\nLimitations:\n- Kernel nodes:\n- The owning context of the function cannot change.\n- A node whose function originally did not use CUDA dynamic parallelism cannot be updated\nto a function which uses CDP.\n- A node whose function originally did not make device-side update calls cannot be updated\nto a function which makes device-side update calls.\n- A cooperative node cannot be updated to a non-cooperative node, and vice-versa.\n- If the graph was instantiated with cudaGraphInstantiateFlagUseNodePriority, the\npriority attribute cannot change. Equality is checked on the originally requested\npriority values, before they are clamped to the device's supported range.\n- If `hGraphExec` was not instantiated for device launch, a node whose function originally\ndid not use device-side cudaGraphLaunch() cannot be updated to a function which uses\ndevice-side cudaGraphLaunch() unless the node resides on the same device as nodes which\ncontained such calls at instantiate-time. If no such calls were present at instantiation,\nthese updates cannot be performed at all.\n- Neither `hGraph` nor `hGraphExec` may contain device-updatable kernel nodes.\n- Memset and memcpy nodes:\n- The CUDA device(s) to which the operand(s) was allocated/mapped cannot change.\n- The source/destination memory must be allocated from the same contexts as the original\nsource/destination memory.\n- For 2d memsets, only address and assigned value may be updated.\n- For 1d memsets, updating dimensions is also allowed, but may fail if the resulting operation doesn't\nmap onto the work resources already allocated for the node.\n- Additional memcpy node restrictions:\n- Changing either the source or destination memory type(i.e. CU_MEMORYTYPE_DEVICE,\nCU_MEMORYTYPE_ARRAY, etc.) is not supported.\n- Conditional nodes:\n- Changing node parameters is not supported.\n- Changing parameters of nodes within the conditional body graph is subject to the rules above.\n- Conditional handle flags and default values are updated as part of the graph update.\nNote:  The API may add further restrictions in future releases.  The return code should always be checked.\ncudaGraphExecUpdate sets the result member of `resultInfo` to cudaGraphExecUpdateErrorTopologyChanged\nunder the following conditions:\n- The count of nodes directly in `hGraphExec` and `hGraph` differ, in which case resultInfo->errorNode\nis set to NULL.\n- `hGraph` has more exit nodes than `hGraph,` in which case resultInfo->errorNode is set to one of\nthe exit nodes in hGraph.\n- A node in `hGraph` has a different number of dependencies than the node from `hGraphExec` it is paired with,\nin which case resultInfo->errorNode is set to the node from `hGraph.`\n- A node in `hGraph` has a dependency that does not match with the corresponding dependency of the paired node\nfrom `hGraphExec.` resultInfo->errorNode will be set to the node from `hGraph.` resultInfo->errorFromNode\nwill be set to the mismatched dependency. The dependencies are paired based on edge order and a dependency\ndoes not match when the nodes are already paired based on other edges examined in the graph.\ncudaGraphExecUpdate sets `the` result member of `resultInfo` to:\n- cudaGraphExecUpdateError if passed an invalid value.\n- cudaGraphExecUpdateErrorTopologyChanged if the graph topology changed\n- cudaGraphExecUpdateErrorNodeTypeChanged if the type of a node changed, in which case\n`hErrorNode_out` is set to the node from `hGraph.`\n- cudaGraphExecUpdateErrorFunctionChanged if the function of a kernel node changed (CUDA driver < 11.2)\n- cudaGraphExecUpdateErrorUnsupportedFunctionChange if the func field of a kernel changed in an\nunsupported way(see note above), in which case `hErrorNode_out` is set to the node from `hGraph`\n- cudaGraphExecUpdateErrorParametersChanged if any parameters to a node changed in a way\nthat is not supported, in which case `hErrorNode_out` is set to the node from `hGraph`\n- cudaGraphExecUpdateErrorAttributesChanged if any attributes of a node changed in a way\nthat is not supported, in which case `hErrorNode_out` is set to the node from `hGraph`\n- cudaGraphExecUpdateErrorNotSupported if something about a node is unsupported, like\nthe node's type or configuration, in which case `hErrorNode_out` is set to the node from `hGraph`\nIf the update fails for a reason not listed above, the result member of `resultInfo` will be set\nto cudaGraphExecUpdateError. If the update succeeds, the result member will be set to cudaGraphExecUpdateSuccess.\ncudaGraphExecUpdate returns cudaSuccess when the updated was performed successfully.  It returns\ncudaErrorGraphExecUpdateFailure if the graph update was not performed because it included\nchanges which violated constraints specific to instantiated graph update.\n\n# Arguments\n\n* `hGraphExec` - The instantiated graph to be updated\n* `hGraph` - The graph containing the updated parameters\n* `resultInfo` - the error info structure\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorGraphExecUpdateFailure,\n\\note_graph_thread_safety \\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaGraphInstantiate`]"]
pub unsafe fn cudaGraphExecUpdate<T: types::CudaAsPtr>(
    hGraphExec: cudaGraphExec_t,
    hGraph: cudaGraph_t,
    mut resultInfo: T,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaGraphExecUpdate(hGraphExec, hGraph, resultInfo.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudaGraphUpload(graphExec: cudaGraphExec_t, stream: cudaStream_t) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaGraphUpload(graphExec, stream) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Launches an executable graph in a stream\nExecutes `graphExec` in `stream.` Only one instance of `graphExec` may be executing\nat a time. Each launch is ordered behind both any previous work in `stream`\nand any previous launches of `graphExec.` To execute a graph concurrently, it must be\ninstantiated multiple times into multiple executable graphs.\nIf any allocations created by `graphExec` remain unfreed (from a previous launch) and\n`graphExec` was not instantiated with ::cudaGraphInstantiateFlagAutoFreeOnLaunch,\nthe launch will fail with ::cudaErrorInvalidValue.\n\n# Arguments\n\n* `graphExec` - - Executable graph to launch\n* `stream` -    - Stream in which to launch the graph\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue\n\\note_graph_thread_safety \\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaGraphInstantiate,`]\n::cudaGraphUpload,\n::cudaGraphExecDestroy"]
pub unsafe fn cudaGraphLaunch(graphExec: cudaGraphExec_t, stream: cudaStream_t) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaGraphLaunch(graphExec, stream) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Write a DOT file describing graph structure\nUsing the provided `graph,` write to `path` a DOT formatted description of the graph.\nBy default this includes the graph topology, node types, node id, kernel names and memcpy direction.\n`flags` can be specified to write more detailed information about each node type such as\nparameter values, kernel attributes, node and function handles.\n\n# Arguments\n\n* `graph` - - The graph to create a DOT file from\n* `path` -  - The path to write the DOT file to\n* `flags` - - Flags from cudaGraphDebugDotFlags for specifying which additional node information to write\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorOperatingSystem"]
pub unsafe fn cudaGraphDebugDotPrint<T: types::CudaAsPtr>(
    graph: cudaGraph_t,
    path: T,
    flags: u32,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaGraphDebugDotPrint(graph, path.as_const_ptr() as *const _, flags as _) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Retain a reference to a user object\nRetains new references to a user object. The new references are owned by the caller.\nSee CUDA User Objects in the CUDA C++ Programming Guide for more information on user objects.\n\n# Arguments\n\n* `object` - - The object to retain\n* `count` -  - The number of references to retain, typically 1. Must be nonzero\nand not larger than INT_MAX.\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue\n\n# See also\n\n> [`::cudaUserObjectCreate,`]\n::cudaUserObjectRelease,\n::cudaGraphRetainUserObject,\n::cudaGraphReleaseUserObject,\n::cudaGraphCreate"]
pub unsafe fn cudaUserObjectRetain(object: cudaUserObject_t, count: u32) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaUserObjectRetain(object, count as _) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Release a reference to a user object\nReleases user object references owned by the caller. The object's destructor is invoked if\nthe reference count reaches zero.\nIt is undefined behavior to release references not owned by the caller, or to use a user\nobject handle after all references are released.\nSee CUDA User Objects in the CUDA C++ Programming Guide for more information on user objects.\n\n# Arguments\n\n* `object` - - The object to release\n* `count` -  - The number of references to release, typically 1. Must be nonzero\nand not larger than INT_MAX.\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue\n\n# See also\n\n> [`::cudaUserObjectCreate,`]\n::cudaUserObjectRetain,\n::cudaGraphRetainUserObject,\n::cudaGraphReleaseUserObject,\n::cudaGraphCreate"]
pub unsafe fn cudaUserObjectRelease(object: cudaUserObject_t, count: u32) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaUserObjectRelease(object, count as _) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Retain a reference to a user object from a graph\nCreates or moves user object references that will be owned by a CUDA graph.\nSee CUDA User Objects in the CUDA C++ Programming Guide for more information on user objects.\n\n# Arguments\n\n* `graph` -  - The graph to associate the reference with\n* `object` - - The user object to retain a reference for\n* `count` -  - The number of references to add to the graph, typically 1. Must be\nnonzero and not larger than INT_MAX.\n* `flags` -  - The optional flag ::cudaGraphUserObjectMove transfers references\nfrom the calling thread, rather than create new references. Pass 0\nto create new references.\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue\n\n# See also\n\n> [`::cudaUserObjectCreate`]\n::cudaUserObjectRetain,\n::cudaUserObjectRelease,\n::cudaGraphReleaseUserObject,\n::cudaGraphCreate"]
pub unsafe fn cudaGraphRetainUserObject(
    graph: cudaGraph_t,
    object: cudaUserObject_t,
    count: u32,
    flags: u32,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaGraphRetainUserObject(graph, object, count as _, flags as _) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Release a user object reference from a graph\nReleases user object references owned by a graph.\nSee CUDA User Objects in the CUDA C++ Programming Guide for more information on user objects.\n\n# Arguments\n\n* `graph` -  - The graph that will release the reference\n* `object` - - The user object to release a reference for\n* `count` -  - The number of references to release, typically 1. Must be nonzero\nand not larger than INT_MAX.\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue\n\n# See also\n\n> [`::cudaUserObjectCreate`]\n::cudaUserObjectRetain,\n::cudaUserObjectRelease,\n::cudaGraphRetainUserObject,\n::cudaGraphCreate"]
pub unsafe fn cudaGraphReleaseUserObject(
    graph: cudaGraph_t,
    object: cudaUserObject_t,
    count: u32,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaGraphReleaseUserObject(graph, object, count as _) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Adds a node of arbitrary type to a graph\nCreates a new node in `graph` described by `nodeParams` with `numDependencies`\ndependencies specified via `pDependencies.` `numDependencies` may be 0.\n`pDependencies` may be null if `numDependencies` is 0. `pDependencies` may not have\nany duplicate entries.\n`nodeParams` is a tagged union. The node type should be specified in the `type` field,\nand type-specific parameters in the corresponding union member. All unused bytes - that\nis, `reserved0` and all bytes past the utilized union member - must be set to zero.\nIt is recommended to use brace initialization or memset to ensure all bytes are\ninitialized.\nNote that for some node types, `nodeParams` may contain \"out parameters\" which are\nmodified during the call, such as `nodeParams->alloc.dptr.`\nA handle to the new node will be returned in `phGraphNode.`\n\n# Arguments\n\n* `pGraphNode` -      - Returns newly created node\n* `graph` -           - Graph to which to add the node\n* `pDependencies` -   - Dependencies of the node\n* `dependencyData` -  - Optional edge data for the dependencies. If NULL, the data is\nassumed to be default (zeroed) for all dependencies.\n* `numDependencies` - - Number of dependencies\n* `nodeParams` -      - Specification of the node\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorInvalidDeviceFunction,\n::cudaErrorNotSupported\n\\note_graph_thread_safety \\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaGraphCreate,`]\n::cudaGraphNodeSetParams,\n::cudaGraphExecNodeSetParams"]
pub unsafe fn cudaGraphAddNode<T: types::CudaAsPtr, U: types::CudaAsPtr, V: types::CudaAsPtr, W: types::CudaAsPtr>(
    mut pGraphNode: T,
    graph: cudaGraph_t,
    pDependencies: U,
    dependencyData: V,
    numDependencies: usize,
    mut nodeParams: W,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphAddNode(
            pGraphNode.as_mut_ptr() as *mut _,
            graph,
            pDependencies.as_const_ptr() as *const _,
            dependencyData.as_const_ptr() as *const _,
            numDependencies,
            nodeParams.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Update a graph node's parameters\nSets the parameters of graph node `node` to `nodeParams.` The node type specified by\n`nodeParams->type` must match the type of `node.` `nodeParams` must be fully\ninitialized and all unused bytes (reserved, padding) zeroed.\nModifying parameters is not supported for node types cudaGraphNodeTypeMemAlloc and\ncudaGraphNodeTypeMemFree.\n\n# Arguments\n\n* `node` -       - Node to set the parameters for\n* `nodeParams` - - Parameters to copy\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorInvalidDeviceFunction,\n::cudaErrorNotSupported\n\\note_graph_thread_safety \\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaGraphNodeGetParams,`]\n::cudaGraphAddNode,\n::cudaGraphExecNodeSetParams"]
pub unsafe fn cudaGraphNodeSetParams<T: types::CudaAsPtr>(
    node: cudaGraphNode_t,
    mut nodeParams: T,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaGraphNodeSetParams(node, nodeParams.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Returns a graph node's parameters\nReturns the parameters of graph node `node` in `*nodeParams.`\nAny pointers returned in `*nodeParams` point to driver-owned memory associated\nwith the node. This memory remains valid until the node is destroyed.  Any memory\npointed to from `*nodeParams` must not be modified.\nThe returned parameters are a description of the node, but may not be identical to the\nstruct provided at creation and may not be suitable for direct creation of identical\nnodes. This is because parameters may be partially unspecified and filled in by the\ndriver at creation, may reference non-copyable handles, or may describe ownership\nsemantics or other parameters that govern behavior of node creation but are not part\nof the final functional descriptor.\n\n# Arguments\n\n* `node` -       - Node to get the parameters for\n* `nodeParams` - - Pointer to return the parameters\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorNotSupported\n\\note_graph_thread_safety \\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaGraphNodeGetParams,`]\n::cudaGraphNodeSetParams,\n::cudaGraphAddNode,\n::cudaGraphExecNodeSetParams"]
pub unsafe fn cudaGraphNodeGetParams(node: cudaGraphNode_t) -> Result<cudaGraphNodeParams, crate::sys::cudaError> {
    let mut out_1: std::mem::MaybeUninit<cudaGraphNodeParams> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaGraphNodeGetParams(node, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_1.assume_init() as cudaGraphNodeParams) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Update a graph node's parameters in an instantiated graph\nSets the parameters of a node in an executable graph `graphExec.` The node is identified\nby the corresponding node `node` in the non-executable graph from which the executable\ngraph was instantiated. `node` must not have been removed from the original graph.\nThe modifications only affect future launches of `graphExec.` Already\nenqueued or running launches of `graphExec` are not affected by this call.\n`node` is also not modified by this call.\nAllowed changes to parameters on executable graphs are as follows:\n<table>\n<tr><th>Node type<th>Allowed changes\n<tr><td>kernel<td>See ::cudaGraphExecKernelNodeSetParams\n<tr><td>memcpy<td>Addresses for 1-dimensional copies if allocated in same context; see ::cudaGraphExecMemcpyNodeSetParams\n<tr><td>memset<td>Addresses for 1-dimensional memsets if allocated in same context; see ::cudaGraphExecMemsetNodeSetParams\n<tr><td>host<td>Unrestricted\n<tr><td>child graph<td>Topology must match and restrictions apply recursively; see ::cudaGraphExecUpdate\n<tr><td>event wait<td>Unrestricted\n<tr><td>event record<td>Unrestricted\n<tr><td>external semaphore signal<td>Number of semaphore operations cannot change\n<tr><td>external semaphore wait<td>Number of semaphore operations cannot change\n<tr><td>memory allocation<td>API unsupported\n<tr><td>memory free<td>API unsupported\n</table>\n\n# Arguments\n\n* `graphExec` -  - The executable graph in which to update the specified node\n* `node` -       - Corresponding node from the graph from which graphExec was instantiated\n* `nodeParams` - - Updated Parameters to set\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorInvalidDeviceFunction,\n::cudaErrorNotSupported\n\\note_graph_thread_safety \\notefnerr \\note_init_rt \\note_callback # See also\n\n> [`::cudaGraphAddNode,`]\n::cudaGraphNodeSetParams\n::cudaGraphExecUpdate,\n::cudaGraphInstantiate"]
pub unsafe fn cudaGraphExecNodeSetParams<T: types::CudaAsPtr>(
    graphExec: cudaGraphExec_t,
    node: cudaGraphNode_t,
    mut nodeParams: T,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaGraphExecNodeSetParams(graphExec, node, nodeParams.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudaGetDriverEntryPoint(
    symbol: *const ::std::os::raw::c_char,
    funcPtr: *mut *mut ::std::os::raw::c_void,
    flags: u64,
) -> Result<cudaDriverEntryPointQueryResult, crate::sys::cudaError> {
    let mut out_3: std::mem::MaybeUninit<cudaDriverEntryPointQueryResult> = std::mem::MaybeUninit::uninit();
    let status =
        unsafe { crate::sys::cudaGetDriverEntryPoint(symbol, funcPtr, flags as _, out_3.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_3.assume_init() as cudaDriverEntryPointQueryResult) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
pub unsafe fn cudaGetDriverEntryPointByVersion(
    symbol: *const ::std::os::raw::c_char,
    funcPtr: *mut *mut ::std::os::raw::c_void,
    cudaVersion: u32,
    flags: u64,
) -> Result<cudaDriverEntryPointQueryResult, crate::sys::cudaError> {
    let mut out_4: std::mem::MaybeUninit<cudaDriverEntryPointQueryResult> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudaGetDriverEntryPointByVersion(
            symbol,
            funcPtr,
            cudaVersion as _,
            flags as _,
            out_4.as_mut_ptr() as *mut _,
        )
    };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_4.assume_init() as cudaDriverEntryPointQueryResult) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Load a library with specified code and options\nTakes a pointer `code` and loads the corresponding library `library` based on\nthe application defined library loading mode:\n- If module loading is set to EAGER, via the environment variables described in \"Module loading\",\n`library` is loaded eagerly into all contexts at the time of the call and future contexts\nat the time of creation until the library is unloaded with ::cudaLibraryUnload().\n- If the environment variables are set to LAZY, `library`\nis not immediately loaded onto all existent contexts and will only be\nloaded when a function is needed for that context, such as a kernel launch.\nThese environment variables are described in the CUDA programming guide under the\n\"CUDA environment variables\" section.\nThe `code` may be a _cubin_ or _fatbin_ as output by **nvcc,**\nor a NULL-terminated _PTX,_ either as output by **nvcc**\nor hand-written, or _Tile_ IR data.\nA fatbin should also contain relocatable code when doing separate compilation.\nPlease also see the documentation for nvrtc (https://docs.nvidia.com/cuda/nvrtc/index.html),\nnvjitlink (https://docs.nvidia.com/cuda/nvjitlink/index.html), and nvfatbin\n(https://docs.nvidia.com/cuda/nvfatbin/index.html) for more information on generating\nloadable code at runtime.\nOptions are passed as an array via `jitOptions` and any corresponding parameters are passed in\n`jitOptionsValues.` The number of total JIT options is supplied via `numJitOptions.`\nAny outputs will be returned via `jitOptionsValues.`\nLibrary load options are passed as an array via `libraryOptions` and any corresponding parameters are passed in\n`libraryOptionValues.` The number of total library load options is supplied via `numLibraryOptions.`\n\n# Arguments\n\n* `library` -             - Returned library\n* `code` -                - Code to load\n* `jitOptions` -          - Options for JIT\n* `jitOptionsValues` -    - Option values for JIT\n* `numJitOptions` -       - Number of options\n* `libraryOptions` -      - Options for loading\n* `libraryOptionValues` - - Option values for loading\n* `numLibraryOptions` -   - Number of options for loading\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorMemoryAllocation,\n::cudaErrorInitializationError,\n::cudaErrorCudartUnloading,\n::cudaErrorInvalidPtx,\n::cudaErrorUnsupportedPtxVersion,\n::cudaErrorNoKernelImageForDevice,\n::cudaErrorSharedObjectSymbolNotFound,\n::cudaErrorSharedObjectInitFailed,\n::cudaErrorJitCompilerNotFound\n\n# See also\n\n> [`::cudaLibraryLoadFromFile,`]\n::cudaLibraryUnload,\n::cuLibraryLoadData"]
pub unsafe fn cudaLibraryLoadData<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    mut library: T,
    code: U,
    mut jitOptions: V,
    mut jitOptionsValues: W,
    numJitOptions: u32,
    mut libraryOptions: X,
    mut libraryOptionValues: Y,
    numLibraryOptions: u32,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaLibraryLoadData(
            library.as_mut_ptr() as *mut _,
            code.as_const_ptr() as *const _,
            jitOptions.as_mut_ptr() as *mut _,
            jitOptionsValues.as_mut_ptr() as *mut _,
            numJitOptions as _,
            libraryOptions.as_mut_ptr() as *mut _,
            libraryOptionValues.as_mut_ptr() as *mut _,
            numLibraryOptions as _,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Load a library with specified file and options\nTakes a pointer `code` and loads the corresponding library `library` based on\nthe application defined library loading mode:\n- If module loading is set to EAGER, via the environment variables described in \"Module loading\",\n`library` is loaded eagerly into all contexts at the time of the call and future contexts\nat the time of creation until the library is unloaded with ::cudaLibraryUnload().\n- If the environment variables are set to LAZY, `library`\nis not immediately loaded onto all existent contexts and will only be\nloaded when a function is needed for that context, such as a kernel launch.\nThese environment variables are described in the CUDA programming guide under the\n\"CUDA environment variables\" section.\nThe file should be a _cubin_ file as output by **nvcc,** or a _PTX_ file either\nas output by **nvcc** or handwritten, or a _fatbin_ file as output by **nvcc**\nor hand-written, or _Tile_ IR file.\nA fatbin should also contain relocatable code when doing separate compilation.\nPlease also see the documentation for nvrtc (https://docs.nvidia.com/cuda/nvrtc/index.html),\nnvjitlink (https://docs.nvidia.com/cuda/nvjitlink/index.html), and nvfatbin\n(https://docs.nvidia.com/cuda/nvfatbin/index.html) for more information on generating\nloadable code at runtime.\nOptions are passed as an array via `jitOptions` and any corresponding parameters are\npassed in `jitOptionsValues.` The number of total options is supplied via `numJitOptions.`\nAny outputs will be returned via `jitOptionsValues.`\nLibrary load options are passed as an array via `libraryOptions` and any corresponding parameters are passed in\n`libraryOptionValues.` The number of total library load options is supplied via `numLibraryOptions.`\n\n# Arguments\n\n* `library` -             - Returned library\n* `fileName` -            - File to load from\n* `jitOptions` -          - Options for JIT\n* `jitOptionsValues` -    - Option values for JIT\n* `numJitOptions` -       - Number of options\n* `libraryOptions` -      - Options for loading\n* `libraryOptionValues` - - Option values for loading\n* `numLibraryOptions` -   - Number of options for loading\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorMemoryAllocation,\n::cudaErrorInitializationError,\n::cudaErrorCudartUnloading,\n::cudaErrorInvalidPtx,\n::cudaErrorUnsupportedPtxVersion,\n::cudaErrorNoKernelImageForDevice,\n::cudaErrorSharedObjectSymbolNotFound,\n::cudaErrorSharedObjectInitFailed,\n::cudaErrorJitCompilerNotFound\n\n# See also\n\n> [`::cudaLibraryLoadData,`]\n::cudaLibraryUnload,\n::cuLibraryLoadFromFile"]
pub unsafe fn cudaLibraryLoadFromFile<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
    X: types::CudaAsPtr,
    Y: types::CudaAsPtr,
>(
    mut library: T,
    fileName: U,
    mut jitOptions: V,
    mut jitOptionsValues: W,
    numJitOptions: u32,
    mut libraryOptions: X,
    mut libraryOptionValues: Y,
    numLibraryOptions: u32,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaLibraryLoadFromFile(
            library.as_mut_ptr() as *mut _,
            fileName.as_const_ptr() as *const _,
            jitOptions.as_mut_ptr() as *mut _,
            jitOptionsValues.as_mut_ptr() as *mut _,
            numJitOptions as _,
            libraryOptions.as_mut_ptr() as *mut _,
            libraryOptionValues.as_mut_ptr() as *mut _,
            numLibraryOptions as _,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Unloads a library\nUnloads the library specified with `library`\n\n# Arguments\n\n* `library` - - Library to unload\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorCudartUnloading,\n::cudaErrorInitializationError,\n::cudaErrorInvalidValue\n\n# See also\n\n> [`::cudaLibraryLoadData,`]\n::cudaLibraryLoadFromFile,\n::cuLibraryUnload"]
pub unsafe fn cudaLibraryUnload(library: cudaLibrary_t) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaLibraryUnload(library) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Returns a kernel handle\nReturns in `pKernel` the handle of the kernel with name `name` located in library `library.`\nIf kernel handle is not found, the call returns ::cudaErrorSymbolNotFound.\n\n# Arguments\n\n* `pKernel` - - Returned kernel handle\n* `library` - - Library to retrieve kernel from\n* `name` - - Name of kernel to retrieve\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorCudartUnloading,\n::cudaErrorInitializationError,\n::cudaErrorInvalidValue,\n::cudaErrorInvalidResourceHandle,\n::cudaErrorSymbolNotFound\n\n# See also\n\n> [`::cudaLibraryLoadData,`]\n::cudaLibraryLoadFromFile,\n::cudaLibraryUnload,\n::cuLibraryGetKernel"]
pub unsafe fn cudaLibraryGetKernel(
    library: cudaLibrary_t,
    name: *const ::std::os::raw::c_char,
) -> Result<cudaKernel_t, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<cudaKernel_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaLibraryGetKernel(out_0.as_mut_ptr() as *mut _, library, name) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_0.assume_init() as cudaKernel_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Returns a global device pointer\nReturns in `*dptr` and `*bytes` the base pointer and size of the global with\nname `name` for the requested library `library` and the current device.\nIf no global for the requested name `name` exists, the call returns ::cudaErrorSymbolNotFound.\nOne of the parameters `dptr` or `bytes` (not both) can be NULL in which\ncase it is ignored. The returned `dptr` cannot be passed to the Symbol APIs\nsuch as ::cudaMemcpyToSymbol, ::cudaMemcpyFromSymbol, ::cudaGetSymbolAddress, or\n::cudaGetSymbolSize.\n\n# Arguments\n\n* `dptr` - - Returned global device pointer for the requested library\n* `bytes` - - Returned global size in bytes\n* `library` - - Library to retrieve global from\n* `name` - - Name of global to retrieve\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorCudartUnloading,\n::cudaErrorInitializationError,\n::cudaErrorInvalidValue,\n::cudaErrorInvalidResourceHandle,\n::cudaErrorSymbolNotFound\n::cudaErrorDeviceUninitialized,\n::cudaErrorContextIsDestroyed\n\n# See also\n\n> [`::cudaLibraryLoadData,`]\n::cudaLibraryLoadFromFile,\n::cudaLibraryUnload,\n::cudaLibraryGetManaged,\n::cuLibraryGetGlobal"]
pub unsafe fn cudaLibraryGetGlobal(
    dptr: *mut *mut ::std::os::raw::c_void,
    library: cudaLibrary_t,
    name: *const ::std::os::raw::c_char,
) -> Result<usize, crate::sys::cudaError> {
    let mut out_1: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaLibraryGetGlobal(dptr, out_1.as_mut_ptr() as *mut _, library, name) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_1.assume_init() as usize) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Returns a pointer to managed memory\nReturns in `*dptr` and `*bytes` the base pointer and size of the managed memory with\nname `name` for the requested library `library.` If no managed memory with the\nrequested name `name` exists, the call returns ::cudaErrorSymbolNotFound. One of the parameters\n`dptr` or `bytes` (not both) can be NULL in which case it is ignored.\nNote that managed memory for library `library` is shared across devices and is registered\nwhen the library is loaded. The returned `dptr` cannot be passed to the Symbol APIs\nsuch as ::cudaMemcpyToSymbol, ::cudaMemcpyFromSymbol, ::cudaGetSymbolAddress, or\n::cudaGetSymbolSize.\n\n# Arguments\n\n* `dptr` - - Returned pointer to the managed memory\n* `bytes` - - Returned memory size in bytes\n* `library` - - Library to retrieve managed memory from\n* `name` - - Name of managed memory to retrieve\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorCudartUnloading,\n::cudaErrorInitializationError,\n::cudaErrorInvalidValue,\n::cudaErrorInvalidResourceHandle,\n::cudaErrorSymbolNotFound\n\n# See also\n\n> [`::cudaLibraryLoadData,`]\n::cudaLibraryLoadFromFile,\n::cudaLibraryUnload,\n::cudaLibraryGetGlobal,\n::cuLibraryGetManaged"]
pub unsafe fn cudaLibraryGetManaged(
    dptr: *mut *mut ::std::os::raw::c_void,
    library: cudaLibrary_t,
    name: *const ::std::os::raw::c_char,
) -> Result<usize, crate::sys::cudaError> {
    let mut out_1: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaLibraryGetManaged(dptr, out_1.as_mut_ptr() as *mut _, library, name) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_1.assume_init() as usize) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Returns a pointer to a unified function\nReturns in `*fptr` the function pointer to a unified function denoted by `symbol.`\nIf no unified function with name `symbol` exists, the call returns ::cudaErrorSymbolNotFound.\nIf there is no device with attribute ::cudaDeviceProp::unifiedFunctionPointers present in the system,\nthe call may return ::cudaErrorSymbolNotFound.\n\n# Arguments\n\n* `fptr` - - Returned pointer to a unified function\n* `library` - - Library to retrieve function pointer memory from\n* `symbol` - - Name of function pointer to retrieve\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorCudartUnloading,\n::cudaErrorInitializationError,\n::cudaErrorInvalidValue,\n::cudaErrorInvalidResourceHandle,\n::cudaErrorSymbolNotFound\n\n# See also\n\n> [`::cudaLibraryLoadData,`]\n::cudaLibraryLoadFromFile,\n::cudaLibraryUnload,\n::cuLibraryGetUnifiedFunction"]
pub unsafe fn cudaLibraryGetUnifiedFunction<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    mut fptr: T,
    library: cudaLibrary_t,
    symbol: U,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaLibraryGetUnifiedFunction(
            fptr.as_mut_ptr() as *mut _,
            library,
            symbol.as_const_ptr() as *const _,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Returns the number of kernels within a library\nReturns in `count` the number of kernels in `lib.`\n\n# Arguments\n\n* `count` - - Number of kernels found within the library\n* `lib` - - Library to query\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorCudartUnloading,\n::cudaErrorInitializationError,\n::cudaErrorInvalidValue,\n::cudaErrorInvalidResourceHandle\n\n# See also\n\n> [`::cudaLibraryEnumerateKernels,`]\n::cudaLibraryLoadFromFile,\n::cudaLibraryLoadData,\n::cuLibraryGetKernelCount"]
pub unsafe fn cudaLibraryGetKernelCount(lib: cudaLibrary_t) -> Result<u32, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<::std::os::raw::c_uint> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaLibraryGetKernelCount(out_0.as_mut_ptr() as *mut _, lib) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_0.assume_init() as u32) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Retrieve the kernel handles within a library.\nReturns in `kernels` a maximum number of `numKernels` kernel handles within `lib.`\nThe returned kernel handle becomes invalid when the library is unloaded.\n\n# Arguments\n\n* `kernels` - - Buffer where the kernel handles are returned to\n* `numKernels` - - Maximum number of kernel handles may be returned to the buffer\n* `lib` - - Library to query from\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorCudartUnloading,\n::cudaErrorInitializationError,\n::cudaErrorInvalidValue,\n::cudaErrorInvalidResourceHandle\n\n# See also\n\n> [`::cudaLibraryGetKernelCount,`]\n::cuLibraryEnumerateKernels"]
pub unsafe fn cudaLibraryEnumerateKernels<T: types::CudaAsPtr>(
    mut kernels: T,
    numKernels: u32,
    lib: cudaLibrary_t,
) -> Result<(), crate::sys::cudaError> {
    let status =
        unsafe { crate::sys::cudaLibraryEnumerateKernels(kernels.as_mut_ptr() as *mut _, numKernels as _, lib) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Sets information about a kernel\nThis call sets the value of a specified attribute `attr` on the kernel `kernel`\nfor the requested device `device` to an integer value specified by `value.`\nThis function returns ::cudaSuccess if the new value of the attribute could be\nsuccessfully set. If the set fails, this call will return an error.\nNot all attributes can have values set. Attempting to set a value on a read-only\nattribute will result in an error (::cudaErrorInvalidValue)\nNote that attributes set using ::cudaFuncSetAttribute() will override the attribute\nset by this API irrespective of whether the call to ::cudaFuncSetAttribute() is made\nbefore or after this API call. Because of this and the stricter locking requirements\nmentioned below it is suggested that this call be used during the initialization path\nand not on each thread accessing `kernel` such as on kernel launches or on the\ncritical path.\nValid values for `attr` are:\n- ::cudaFuncAttributeMaxDynamicSharedMemorySize - The requested maximum size in bytes of dynamically-allocated shared memory. The sum of this value and the function attribute ::sharedSizeBytes\ncannot exceed the device attribute ::cudaDevAttrMaxSharedMemoryPerBlockOptin. The maximal size of requestable dynamic shared memory may differ by GPU architecture.\n- ::cudaFuncAttributePreferredSharedMemoryCarveout - On devices where the L1 cache and shared memory use the same hardware resources,\nthis sets the shared memory carveout preference, in percent of the total shared memory. See ::cudaDevAttrMaxSharedMemoryPerMultiprocessor.\nThis is only a hint, and the driver can choose a different ratio if required to execute the function.\n- ::cudaFuncAttributeRequiredClusterWidth: The required cluster width in\nblocks. The width, height, and depth values must either all be 0 or all be\npositive. The validity of the cluster dimensions is checked at launch time.\nIf the value is set during compile time, it cannot be set at runtime.\nSetting it at runtime will return cudaErrorNotPermitted.\n- ::cudaFuncAttributeRequiredClusterHeight: The required cluster height in\nblocks. The width, height, and depth values must either all be 0 or all be\npositive. The validity of the cluster dimensions is checked at launch time.\nIf the value is set during compile time, it cannot be set at runtime.\nSetting it at runtime will return cudaErrorNotPermitted.\n- ::cudaFuncAttributeRequiredClusterDepth: The required cluster depth in\nblocks. The width, height, and depth values must either all be 0 or all be\npositive. The validity of the cluster dimensions is checked at launch time.\nIf the value is set during compile time, it cannot be set at runtime.\nSetting it at runtime will return cudaErrorNotPermitted.\n- ::cudaFuncAttributeNonPortableClusterSizeAllowed: Indicates whether the\nfunction can be launched with non-portable cluster size. 1 is allowed, 0 is\ndisallowed.\n- ::cudaFuncAttributeClusterSchedulingPolicyPreference: The block\nscheduling policy of a function. The value type is cudaClusterSchedulingPolicy.\n> **Note** The API has stricter locking requirements in comparison to its legacy counterpart\n::cudaFuncSetAttribute() due to device-wide semantics. If multiple threads are trying to\nset the same attribute on the same device simultaneously, the attribute setting will depend\non the interleavings chosen by the OS scheduler and memory consistency.\n\n# Arguments\n\n* `kernel` -  - Kernel to set attribute of\n* `attr` - - Attribute requested\n* `value` - - Value to set\n* `device` - - Device to set attribute of\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidDeviceFunction,\n::cudaErrorInvalidValue\n\n# See also\n\n> [`::cudaLibraryLoadData,`]\n::cudaLibraryLoadFromFile,\n::cudaLibraryUnload,\n::cudaLibraryGetKernel,\n::cudaLaunchKernel,\n::cudaFuncSetAttribute,\n::cuKernelSetAttribute"]
pub unsafe fn cudaKernelSetAttributeForDevice(
    kernel: cudaKernel_t,
    attr: cudaFuncAttribute,
    value: i32,
    device: i32,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaKernelSetAttributeForDevice(kernel, attr, value as _, device as _) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Get device resources\nGet the `type` resources available to the `device.`\nThis may often be the starting point for further partitioning or configuring\nof resources.\nNote: The API is not supported on 32-bit platforms.\n\n# Arguments\n\n* `device` - - Device to get resource for\n* `resource` - - Output pointer to a cudaDevResource structure\n* `type` - - Type of resource to retrieve\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorNotPermitted,\n::cudaErrorInvalidDevice,\n::cudaErrorInvalidResourceType,\n::cudaErrorNotSupported,\n::cudaErrorCudartUnloading,\n::cudaErrorInitializationError\n\\note_callback # See also\n\n> [`::cuDeviceGetDevResource,`]\n::cudaExecutionCtxGetDevResource,\n::cudaDevSmResourceSplit,\n::cudaDevResourceGenerateDesc"]
pub unsafe fn cudaDeviceGetDevResource(
    device: i32,
    type_: cudaDevResourceType,
) -> Result<cudaDevResource, crate::sys::cudaError> {
    let mut out_1: std::mem::MaybeUninit<cudaDevResource> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaDeviceGetDevResource(device as _, out_1.as_mut_ptr() as *mut _, type_) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_1.assume_init() as cudaDevResource) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Splits `cudaDevResourceTypeSm` resources.\nSplits `cudaDevResourceTypeSm` resources into `nbGroups,` adhering to the\nminimum SM count specified in `minCount` and the usage flags in `flags.`\nIf `result` is NULL, the API simulates a split and provides the amount of groups that\nwould be created in `nbGroups.` Otherwise, `nbGroups` must point to the amount of elements\nin `result` and on return, the API will overwrite `nbGroups` with the amount actually created.\nThe groups are written to the array in `result.`\n`nbGroups` can be less than the total amount if a smaller number of groups is needed.\nThis API is used to spatially partition the input resource. The input resource needs to come\nfrom one of ::cudaDeviceGetDevResource, or ::cudaExecutionCtxGetDevResource.\nA limitation of the API is that the output results cannot be split again without\nfirst creating a descriptor and a green context with that descriptor.\nWhen creating the groups, the API will take into account the performance and functional\ncharacteristics of the input resource, and guarantee a split that will create a disjoint\nset of symmetrical partitions. This may lead to fewer groups created than purely dividing\nthe total SM count by the `minCount` due to cluster requirements or alignment and granularity\nrequirements for the minCount.\nThese requirements can be queried with ::cudaDeviceGetDevResource, or ::cudaExecutionCtxGetDevResource\nfor ::cudaDevResourceTypeSm, using the `minSmPartitionSize` and `smCoscheduledAlignment` fields\nto determine minimum partition size and alignment granularity, respectively.\nThe `remainder` set does not have the same functional or performance guarantees as the groups\nin `result.` Its use should be carefully planned and future partitions of the `remainder` set\nare discouraged.\nThe following flags are supported:\n- `cudaDevSmResourceSplitIgnoreSmCoscheduling` : Lower the minimum SM count and alignment, and\ntreat each SM independent of its hierarchy. This allows more fine grained partitions but at the\ncost of advanced features (such as large clusters on compute capability 9.0+).\n- `cudaDevSmResourceSplitMaxPotentialClusterSize` : Compute Capability 9.0+ only. Attempt to\ncreate groups that may allow for maximally sized thread clusters. This can be queried post\ngreen context creation using ::cudaOccupancyMaxPotentialClusterSize.\nA successful API call must either have:\n- A valid array of `result` pointers of size passed in `nbGroups,` with `input` of type\n`cudaDevResourceTypeSm.` Value of `minCount` must be between 0 and the SM count specified\nin `input.` `remaining` may be NULL.\n- NULL passed in for `result,` with a valid integer pointer in `nbGroups` and `input` of\ntype `cudaDevResourceTypeSm.` Value of `minCount` must be between 0 and the SM count\nspecified in `input.` `remaining` may be NULL. This queries the number of groups that\nwould be created by the API.\nNote: The API is not supported on 32-bit platforms.\n\n# Arguments\n\n* `result` - - Output array of `cudaDevResource` resources. Can be NULL to query the\nnumber of groups.\n* `nbGroups` - - This is a pointer, specifying the number of groups that would be or\nshould be created as described below.\n* `input` - - Input SM resource to be split. Must be a valid `cudaDevSmResource` resource.\n* `remaining` - - If the input resource cannot be cleanly split among `nbGroups,`\nthe remaining is placed in here. Can be ommitted (NULL) if the user does not need the remaining set.\n* `flags` - - Flags specifying how these partitions are used or which constraints to abide by\nwhen splitting the input. Zero is valid for default behavior.\n* `minCount` - - Minimum number of SMs required\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorNotPermitted,\n::cudaErrorInvalidResourceType,\n::cudaErrorInvalidResourceConfiguration,\n::cudaErrorNotSupported,\n::cudaErrorCudartUnloading,\n::cudaErrorInitializationError\n\\note_callback # See also\n\n> [`::cuDevSmResourceSplitByCount,`]\n::cudaDeviceGetDevResource,\n::cudaExecutionCtxGetDevResource,\n::cudaDevResourceGenerateDesc"]
pub unsafe fn cudaDevSmResourceSplitByCount<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    mut result: T,
    mut nbGroups: U,
    input: V,
    mut remaining: W,
    flags: u32,
    minCount: u32,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaDevSmResourceSplitByCount(
            result.as_mut_ptr() as *mut _,
            nbGroups.as_mut_ptr() as *mut _,
            input.as_const_ptr() as *const _,
            remaining.as_mut_ptr() as *mut _,
            flags as _,
            minCount as _,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Splits a `cudaDevResourceTypeSm` resource into structured groups.\nThis API will split a resource of ::cudaDevResourceTypeSm into `nbGroups` structured device resource groups (the `result` array),\nas well as an optional `remainder,` according to a set of requirements specified in the `groupParams` array. The term “structured”\nis a trait that specifies the `result` has SMs that are co-scheduled together. This co-scheduling can be specified via the `coscheduledSmCount`\nfield of the `groupParams` structure, while the `smCount` will specify how many SMs are required in total for that result.\nThe remainder is always “unstructured”, it does not have any set guarantees with respect to co-scheduling and those properties will need to\neither be queried via the occupancy set of APIs or further split into structured groups by this API.\nThe API has a discovery mode for use cases where it is difficult to know ahead of time what the SM count should be.\nDiscovery happens when the `smCount` field of a given `groupParams` array entry is set to 0 - the smCount will be filled in by the API\nwith the derived SM count according to the provided `groupParams` fields and constraints. Discovery can be used with both a valid result\narray and with a NULL `result` pointer value. The latter is useful in situations where the smCount will end up being zero, which is an invalid\nvalue to create a result entry with, but allowed for discovery purposes when the `result` is NULL.\nThe `groupParams` array is evaluated from index 0 to `nbGroups` - 1. For each index in the `groupParams` array,\nthe API will evaluate which SMs may be a good fit based on constraints and assign those SMs to `result.`\nThis evaluation order is important to consider when using discovery mode, as it helps discover the remaining SMs.\nFor a valid call:\n- `result` should point to a `cudaDevResource` array of size `nbGroups,` or alternatively, may be NULL, if the developer wishes for only the groupParams entries to be updated\n- `input` should be a valid ::cudaDevResourceTypeSm resource that originates from querying the execution context, or device.\n- The `remainder` group may be NULL.\n- There are no API `flags` at this time, so the value passed in should be 0.\n- A ::cudaDevSmResourceGroupParams array of size `nbGroups.` Each entry must be zero-initialized.\n- `smCount:` must be either 0 or in the range of [2,inputSmCount] where inputSmCount is the amount of SMs the `input` resource has.\n`smCount` must be a multiple of 2, as well as a multiple of `coscheduledSmCount.` When assigning SMs to a group (and if results are\nexpected by having the `result` parameter set), `smCount` cannot end up with 0 or a value less than `coscheduledSmCount`\notherwise ::cudaErrorInvalidResourceConfiguration will be returned.\n- `coscheduledSmCount:` allows grouping SMs together in order to be able to launch clusters on Compute Architecture 9.0+.\nThe default value may be queried from the device’s ::cudaDevResourceTypeSm resource (8 on Compute Architecture 9.0+ and 2 otherwise).\nThe maximum is 32 on Compute Architecture 9.0+ and 2 otherwise.\n- `preferredCoscheduledSmCount:` Attempts to merge `coscheduledSmCount` groups into larger groups,\nin order to make use of `preferredClusterDimensions` on Compute Architecture 10.0+. The default value is set to `coscheduledSmCount.`\n- `flags:`\n- `cudaDevSmResourceGroupBackfill:` lets `smCount` be a non-multiple of `coscheduledSmCount,` filling the difference between SM count\nand already assigned co-scheduled groupings with other SMs. This lets any resulting group behave similar to the `remainder` group for example.\n<b>Example params and their effect:</b>\nA groupParams array element is defined in the following order:\n\\code { .smCount, .coscheduledSmCount, .preferredCoscheduledSmCount, .flags, \\/\\* .reserved \\*\\/ }\n\\endcode \\code // Example 1\n// Will discover how many SMs there are, that are co-scheduled in groups of smCoscheduledAlignment.\n// The rest is placed in the optional remainder.\ncudaDevSmResourceGroupParams params { 0, 0, 0, 0 };\n\\endcode \\code // Example 2\n// Assuming the device has 10+ SMs, the result will have 10 SMs that are co-scheduled in groups of 2 SMs.\n// The rest is placed in the optional remainder.\ncudaDevSmResourceGroupParams params { 10, 2, 0, 0};\n// Setting the coscheduledSmCount to 2 guarantees that we can always have a valid result\n// as long as the SM count is less than or equal to the input resource SM count.\n\\endcode \\code // Example 3\n// A single piece is split-off, but instead of assigning the rest to the remainder, a second group contains everything else\n// This assumes the device has 10+ SMs (8 of which are coscheduled in groups of 4),\n// otherwise the second group could end up with 0 SMs, which is not allowed.\ncudaDevSmResourceGroupParams params { {8, 4, 0, 0}, {0, 2, 0, cudaDevSmResourceGroupBackfill } }\n\\endcode The difference between a catch-all param group as the last entry and the remainder is in two aspects:\n- The remainder may be NULL / _TYPE_INVALID (if there are no SMs remaining), while a result group must always be valid.\n- The remainder does not have a structure, while the result group will always need to adhere to a structure\nof coscheduledSmCount (even if its just 2), and therefore must always have enough coscheduled SMs to cover\nthat requirement (even with the `cudaDevSmResourceGroupBackfill` flag enabled).\nSplitting an input into N groups, can be accomplished by repeatedly splitting off 1 group and re-splitting\nthe remainder (a bisect operation). However, it's recommended to accomplish this with a single call wherever possible.\n\n# Arguments\n\n* `result` - - Output array of `cudaDevResource` resources. Can be NULL, alongside an smCount of 0, for discovery purpose.\n* `nbGroups` - - Specifies the number of groups in `result` and `groupParams`\n* `input` - - Input SM resource to be split. Must be a valid `cudaDevResourceTypeSm` resource.\n* `remainder` - - If splitting the input resource leaves any SMs, the remainder is placed in here.\n* `flags` - - Flags specifying how the API should behave. The value should be 0 for now.\n* `groupParams` - - Description of how the SMs should be split and assigned to the corresponding result entry.\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorNotPermitted,\n::cudaErrorInvalidResourceType,\n::cudaErrorInvalidResourceConfiguration,\n::cudaErrorNotSupported,\n::cudaErrorCudartUnloading,\n::cudaErrorInitializationError\n\\note_callback # See also\n\n> [`::cuDevSmResourceSplit,`]\n::cudaDeviceGetDevResource,\n::cudaExecutionCtxGetDevResource,\n::cudaDevResourceGenerateDesc"]
pub unsafe fn cudaDevSmResourceSplit<
    T: types::CudaAsPtr,
    U: types::CudaAsPtr,
    V: types::CudaAsPtr,
    W: types::CudaAsPtr,
>(
    mut result: T,
    nbGroups: u32,
    input: U,
    mut remainder: V,
    flags: u32,
    mut groupParams: W,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaDevSmResourceSplit(
            result.as_mut_ptr() as *mut _,
            nbGroups as _,
            input.as_const_ptr() as *const _,
            remainder.as_mut_ptr() as *mut _,
            flags as _,
            groupParams.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Generate a resource descriptor\nGenerates a single resource descriptor with the set of resources specified in `resources.`\nThe generated resource descriptor is necessary for the creation of green contexts via the\n::cudaGreenCtxCreate API. Resources of the same type can be passed in, provided they meet\nthe requirements as noted below.\nA successful API call must have:\n- A valid output pointer for the `phDesc` descriptor as well as a valid array of `resources` pointers,\nwith the array size passed in `nbResources.`\nIf multiple resources are provided in `resources,` the device they came from must be the same,\notherwise ::cudaErrorInvalidResourceConfiguration is returned.\nIf multiple resources are provided in `resources` and they are of type ::cudaDevResourceTypeSm,\nthey must be outputs (whether `result` or `remaining)` from the same split API instance and have\nthe same smCoscheduledAlignment values, otherwise ::cudaErrorInvalidResourceConfiguration is returned.\nNote: The API is not supported on 32-bit platforms.\n\n# Arguments\n\n* `phDesc` - - Output descriptor\n* `resources` - - Array of resources to be included in the descriptor\n* `nbResources` - - Number of resources passed in `resources`\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorNotPermitted,\n::cudaErrorInvalidResourceType,\n::cudaErrorInvalidResourceConfiguration,\n::cudaErrorNotSupported,\n::cudaErrorOutOfMemory,\n::cudaErrorCudartUnloading,\n::cudaErrorInitializationError\n\\note_callback # See also\n\n> [`::cuDevResourceGenerateDesc,`]\n::cudaDeviceGetDevResource,\n::cudaExecutionCtxGetDevResource,\n::cudaDevSmResourceSplit,\n::cudaGreenCtxCreate"]
pub unsafe fn cudaDevResourceGenerateDesc<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    mut phDesc: T,
    mut resources: U,
    nbResources: u32,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaDevResourceGenerateDesc(
            phDesc.as_mut_ptr() as *mut _,
            resources.as_mut_ptr() as *mut _,
            nbResources as _,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Returns the device handle for the execution context\nReturns in `*device` the handle of the specified execution context's device.\nThe execution context should not be NULL.\n\n# Arguments\n\n* `device` - - Returned device handle for the specified execution context\n* `ctx` - - Execution context for which to obtain the device (required parameter, see note below)\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorCudartUnloading,\n::cudaErrorInitializationError,\n::cudaErrorInvalidValue,\n::cudaErrorNotPermitted\n\\notefnerr \\note_callback \\note_cudaExecutionContext_t_required_param # See also\n\n> [`::cudaGreenCtxCreate,`]\n::cudaExecutionCtxDestroy,\n::cuCtxGetDevice"]
pub unsafe fn cudaExecutionCtxGetDevice(ctx: cudaExecutionContext_t) -> Result<i32, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaExecutionCtxGetDevice(out_0.as_mut_ptr() as *mut _, ctx) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_0.assume_init() as i32) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Get stream resources\nGet the `type` resources available to the `hStream` and store them in `resource.`\nNote: The API will return ::cudaErrorInvalidResourceType is `type` is\n`cudaDevResourceTypeWorkqueueConfig` or `cudaDevResourceTypeWorkqueue.`\n\n# Arguments\n\n* `hStream` - - Stream to get resource for\n* `resource` - - Output pointer to a cudaDevResource structure\n* `type` - - Type of resource to retrieve\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorCudartUnloading,\n::cudaErrorInitializationError,\n::cudaErrorDeviceUninitialized,\n::cudaErrorInvalidResourceType,\n::cudaErrorInvalidValue,\n::cudaErrorInvalidHandle,\n::cudaErrorNotPermitted,\n::cudaErrorCallRequiresNewerDriver,\n\\notefnerr \\note_callback # See also\n\n> [`::cudaGreenCtxCreate,`]\n::cudaExecutionCtxStreamCreate,\n::cudaStreamCreate,\n::cudaDevSmResourceSplit,\n::cudaDevResourceGenerateDesc,\n::cuStreamGetDevResource"]
pub unsafe fn cudaStreamGetDevResource(
    hStream: cudaStream_t,
    type_: cudaDevResourceType,
) -> Result<cudaDevResource, crate::sys::cudaError> {
    let mut out_1: std::mem::MaybeUninit<cudaDevResource> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaStreamGetDevResource(hStream, out_1.as_mut_ptr() as *mut _, type_) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_1.assume_init() as cudaDevResource) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Returns the execution context for a device\nReturns in `ctx` the execution context for the specified device. This is the device's primary context.\nThe returned context can then be passed to APIs that take in a cudaExecutionContext_t enabling explicit\ncontext-based programming without relying on thread-local state.\nPassing the returned execution context to ::cudaExecutionCtxDestroy() is not allowed and will result in undefined behavior.\n\n# Arguments\n\n* `ctx` - - Returns the device execution context\n* `device` - - Device to get the execution context for\n\n# Returns\n\n::cudaSuccess,\n::cudaErrorInvalidValue,\n::cudaErrorInvalidDevice\n\n# See also\n\n> [`cudaExecutionCtxGetDevice,`]\ncudaExecutionCtxGetId"]
pub unsafe fn cudaDeviceGetExecutionCtx(device: i32) -> Result<cudaExecutionContext_t, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<cudaExecutionContext_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaDeviceGetExecutionCtx(out_0.as_mut_ptr() as *mut _, device as _) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_0.assume_init() as cudaExecutionContext_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "\\cond impl_private"]
pub unsafe fn cudaGetExportTable<T: types::CudaAsPtr, U: types::CudaAsPtr>(
    mut ppExportTable: T,
    pExportTableId: U,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGetExportTable(
            ppExportTable.as_mut_ptr() as *mut _,
            pExportTableId.as_const_ptr() as *const _,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = "Get pointer to device entry function that matches entry function `symbolPtr`\nReturns in `functionPtr` the device entry function corresponding to the symbol `symbolPtr.`\n\n# Arguments\n\n* `functionPtr` -     - Returns the device entry function\n* `symbolPtr` -       - Pointer to device entry function to search for\n\n# Returns\n\n::cudaSuccess\n"]
pub unsafe fn cudaGetFuncBySymbol(
    symbolPtr: *const ::std::os::raw::c_void,
) -> Result<cudaFunction_t, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<cudaFunction_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaGetFuncBySymbol(out_0.as_mut_ptr() as *mut _, symbolPtr) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_0.assume_init() as cudaFunction_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Get pointer to device kernel that matches entry function `entryFuncAddr`\nReturns in `kernelPtr` the device kernel corresponding to the entry function `entryFuncAddr.`\nNote that it is possible that there are multiple symbols belonging to different\ntranslation units with the same `entryFuncAddr` registered with this CUDA Runtime\nand so the order which the translation units are loaded and registered with the\nCUDA Runtime can lead to differing return pointers in `kernelPtr` .\nSuggested methods of ensuring uniqueness are to limit visibility of __global__\ndevice functions by using static or hidden visibility attribute in the\nrespective translation units.\n\n# Arguments\n\n* `kernelPtr` -          - Returns the device kernel\n* `entryFuncAddr` -      - Address of device entry function to search kernel for\n\n# Returns\n\n::cudaSuccess\n\n# See also\n\n> [`\\ref`] ::cudaGetKernel(cudaKernel_t *kernelPtr, const T *entryFuncAddr) \"cudaGetKernel (C++ API)\""]
pub unsafe fn cudaGetKernel(
    entryFuncAddr: *const ::std::os::raw::c_void,
) -> Result<cudaKernel_t, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<cudaKernel_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaGetKernel(out_0.as_mut_ptr() as *mut _, entryFuncAddr) };
    if status as usize == crate::sys::cudaError::cudaSuccess as usize {
        unsafe { Ok(out_0.assume_init() as cudaKernel_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[allow(non_upper_case_globals)]
pub use crate::sys::cudaError as CudaStatusEnum;
impl CudaExecutionContext {
    pub fn cudaGreenCtxCreate(
        desc: crate::sys::cudaDevResourceDesc_t,
        device: i32,
        flags: u32,
    ) -> Result<Self, crate::sys::cudaError> {
        unsafe {
            let mut handle = std::ptr::null_mut();
            let status = crate::sys::cudaGreenCtxCreate(&mut handle, desc, device as _, flags as _);
            if status == crate::sys::cudaError::cudaSuccess {
                Ok(Self(handle))
            } else {
                Err(status)
            }
        }
    }
    pub fn cudaDeviceGetExecutionCtx(device: i32) -> Result<Self, crate::sys::cudaError> {
        unsafe {
            let mut handle = std::ptr::null_mut();
            let status = crate::sys::cudaDeviceGetExecutionCtx(&mut handle, device as _);
            if status == crate::sys::cudaError::cudaSuccess {
                Ok(Self(handle))
            } else {
                Err(status)
            }
        }
    }
}
impl Drop for CudaExecutionContext {
    fn drop(&mut self) {
        unsafe {
            let _ = crate::sys::cudaExecutionCtxDestroy(self.0);
        }
    }
}
