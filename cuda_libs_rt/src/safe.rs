pub use crate::sys::cudaError as CudaStatusEnum;
#[allow(unused_imports)]
use crate::sys::*;
impl crate::sys::dim3 {
    pub fn x(mut self, val: ::std::os::raw::c_uint) -> Self {
        self.x = val;
        self
    }
    pub fn y(mut self, val: ::std::os::raw::c_uint) -> Self {
        self.y = val;
        self
    }
    pub fn z(mut self, val: ::std::os::raw::c_uint) -> Self {
        self.z = val;
        self
    }
}
impl crate::sys::cudaChannelFormatDesc {
    pub fn x(mut self, val: ::std::os::raw::c_int) -> Self {
        self.x = val;
        self
    }
    pub fn y(mut self, val: ::std::os::raw::c_int) -> Self {
        self.y = val;
        self
    }
    pub fn z(mut self, val: ::std::os::raw::c_int) -> Self {
        self.z = val;
        self
    }
    pub fn w(mut self, val: ::std::os::raw::c_int) -> Self {
        self.w = val;
        self
    }
    pub fn f(mut self, val: cudaChannelFormatKind) -> Self {
        self.f = val;
        self
    }
}
impl crate::sys::cudaArraySparseProperties {
    pub fn tileExtent(mut self, val: cudaArraySparseProperties__bindgen_ty_1) -> Self {
        self.tileExtent = val;
        self
    }
    pub fn miptailFirstLevel(mut self, val: ::std::os::raw::c_uint) -> Self {
        self.miptailFirstLevel = val;
        self
    }
    pub fn miptailSize(mut self, val: ::std::os::raw::c_ulonglong) -> Self {
        self.miptailSize = val;
        self
    }
    pub fn flags(mut self, val: ::std::os::raw::c_uint) -> Self {
        self.flags = val;
        self
    }
    pub fn reserved(mut self, val: [::std::os::raw::c_uint; 4usize]) -> Self {
        self.reserved = val;
        self
    }
}
impl crate::sys::cudaArraySparseProperties__bindgen_ty_1 {
    pub fn width(mut self, val: ::std::os::raw::c_uint) -> Self {
        self.width = val;
        self
    }
    pub fn height(mut self, val: ::std::os::raw::c_uint) -> Self {
        self.height = val;
        self
    }
    pub fn depth(mut self, val: ::std::os::raw::c_uint) -> Self {
        self.depth = val;
        self
    }
}
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
impl crate::sys::cudaMemcpyNodeParams {
    pub fn flags(mut self, val: ::std::os::raw::c_int) -> Self {
        self.flags = val;
        self
    }
    pub fn reserved(mut self, val: ::std::os::raw::c_int) -> Self {
        self.reserved = val;
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
    pub fn srcDevice(mut self, val: ::std::os::raw::c_int) -> Self {
        self.srcDevice = val;
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
    pub fn dstDevice(mut self, val: ::std::os::raw::c_int) -> Self {
        self.dstDevice = val;
        self
    }
    pub fn extent(mut self, val: cudaExtent) -> Self {
        self.extent = val;
        self
    }
}
impl crate::sys::cudaMemsetParams {
    pub fn dst(mut self, val: *mut ::std::os::raw::c_void) -> Self {
        self.dst = val;
        self
    }
    pub fn pitch(mut self, val: usize) -> Self {
        self.pitch = val;
        self
    }
    pub fn value(mut self, val: ::std::os::raw::c_uint) -> Self {
        self.value = val;
        self
    }
    pub fn elementSize(mut self, val: ::std::os::raw::c_uint) -> Self {
        self.elementSize = val;
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
impl crate::sys::cudaMemsetParamsV2 {
    pub fn dst(mut self, val: *mut ::std::os::raw::c_void) -> Self {
        self.dst = val;
        self
    }
    pub fn pitch(mut self, val: usize) -> Self {
        self.pitch = val;
        self
    }
    pub fn value(mut self, val: ::std::os::raw::c_uint) -> Self {
        self.value = val;
        self
    }
    pub fn elementSize(mut self, val: ::std::os::raw::c_uint) -> Self {
        self.elementSize = val;
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
impl crate::sys::cudaHostNodeParamsV2 {
    pub fn fn_(mut self, val: cudaHostFn_t) -> Self {
        self.fn_ = val;
        self
    }
    pub fn userData(mut self, val: *mut ::std::os::raw::c_void) -> Self {
        self.userData = val;
        self
    }
    pub fn syncMode(mut self, val: ::std::os::raw::c_uint) -> Self {
        self.syncMode = val;
        self
    }
}
impl crate::sys::cudaResourceDesc {
    pub fn resType(mut self, val: cudaResourceType) -> Self {
        self.resType = val;
        self
    }
    pub fn res(mut self, val: cudaResourceDesc__bindgen_ty_1) -> Self {
        self.res = val;
        self
    }
    pub fn flags(mut self, val: ::std::os::raw::c_uint) -> Self {
        self.flags = val;
        self
    }
}
impl crate::sys::cudaResourceDesc__bindgen_ty_1__bindgen_ty_1 {
    pub fn array(mut self, val: cudaArray_t) -> Self {
        self.array = val;
        self
    }
}
impl crate::sys::cudaResourceDesc__bindgen_ty_1__bindgen_ty_2 {
    pub fn mipmap(mut self, val: cudaMipmappedArray_t) -> Self {
        self.mipmap = val;
        self
    }
}
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
impl crate::sys::cudaResourceDesc__bindgen_ty_1__bindgen_ty_5 {
    pub fn reserved(mut self, val: [::std::os::raw::c_int; 32usize]) -> Self {
        self.reserved = val;
        self
    }
}
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
    pub fn firstMipmapLevel(mut self, val: ::std::os::raw::c_uint) -> Self {
        self.firstMipmapLevel = val;
        self
    }
    pub fn lastMipmapLevel(mut self, val: ::std::os::raw::c_uint) -> Self {
        self.lastMipmapLevel = val;
        self
    }
    pub fn firstLayer(mut self, val: ::std::os::raw::c_uint) -> Self {
        self.firstLayer = val;
        self
    }
    pub fn lastLayer(mut self, val: ::std::os::raw::c_uint) -> Self {
        self.lastLayer = val;
        self
    }
    pub fn reserved(mut self, val: [::std::os::raw::c_uint; 16usize]) -> Self {
        self.reserved = val;
        self
    }
}
impl crate::sys::cudaPointerAttributes {
    pub fn type_(mut self, val: cudaMemoryType) -> Self {
        self.type_ = val;
        self
    }
    pub fn device(mut self, val: ::std::os::raw::c_int) -> Self {
        self.device = val;
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
    pub fn maxThreadsPerBlock(mut self, val: ::std::os::raw::c_int) -> Self {
        self.maxThreadsPerBlock = val;
        self
    }
    pub fn numRegs(mut self, val: ::std::os::raw::c_int) -> Self {
        self.numRegs = val;
        self
    }
    pub fn ptxVersion(mut self, val: ::std::os::raw::c_int) -> Self {
        self.ptxVersion = val;
        self
    }
    pub fn binaryVersion(mut self, val: ::std::os::raw::c_int) -> Self {
        self.binaryVersion = val;
        self
    }
    pub fn cacheModeCA(mut self, val: ::std::os::raw::c_int) -> Self {
        self.cacheModeCA = val;
        self
    }
    pub fn maxDynamicSharedSizeBytes(mut self, val: ::std::os::raw::c_int) -> Self {
        self.maxDynamicSharedSizeBytes = val;
        self
    }
    pub fn preferredShmemCarveout(mut self, val: ::std::os::raw::c_int) -> Self {
        self.preferredShmemCarveout = val;
        self
    }
    pub fn clusterDimMustBeSet(mut self, val: ::std::os::raw::c_int) -> Self {
        self.clusterDimMustBeSet = val;
        self
    }
    pub fn requiredClusterWidth(mut self, val: ::std::os::raw::c_int) -> Self {
        self.requiredClusterWidth = val;
        self
    }
    pub fn requiredClusterHeight(mut self, val: ::std::os::raw::c_int) -> Self {
        self.requiredClusterHeight = val;
        self
    }
    pub fn requiredClusterDepth(mut self, val: ::std::os::raw::c_int) -> Self {
        self.requiredClusterDepth = val;
        self
    }
    pub fn clusterSchedulingPolicyPreference(mut self, val: ::std::os::raw::c_int) -> Self {
        self.clusterSchedulingPolicyPreference = val;
        self
    }
    pub fn nonPortableClusterSizeAllowed(mut self, val: ::std::os::raw::c_int) -> Self {
        self.nonPortableClusterSizeAllowed = val;
        self
    }
    pub fn reserved0(mut self, val: ::std::os::raw::c_int) -> Self {
        self.reserved0 = val;
        self
    }
    pub fn reserved(mut self, val: [::std::os::raw::c_int; 15usize]) -> Self {
        self.reserved = val;
        self
    }
}
impl crate::sys::cudaMemLocation {
    pub fn type_(mut self, val: cudaMemLocationType) -> Self {
        self.type_ = val;
        self
    }
    pub fn id(mut self, val: ::std::os::raw::c_int) -> Self {
        self.id = val;
        self
    }
}
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
    pub fn usage(mut self, val: ::std::os::raw::c_ushort) -> Self {
        self.usage = val;
        self
    }
    pub fn reserved(mut self, val: [::std::os::raw::c_uchar; 54usize]) -> Self {
        self.reserved = val;
        self
    }
}
impl crate::sys::cudaMemPoolPtrExportData {
    pub fn reserved(mut self, val: [::std::os::raw::c_uchar; 64usize]) -> Self {
        self.reserved = val;
        self
    }
}
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
impl crate::sys::cudaMemFreeNodeParams {
    pub fn dptr(mut self, val: *mut ::std::os::raw::c_void) -> Self {
        self.dptr = val;
        self
    }
}
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
    pub fn flags(mut self, val: ::std::os::raw::c_uint) -> Self {
        self.flags = val;
        self
    }
}
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
    pub fn flags(mut self, val: ::std::os::raw::c_uint) -> Self {
        self.flags = val;
        self
    }
}
impl crate::sys::CUuuid_st {
    pub fn bytes(mut self, val: [::std::os::raw::c_char; 16usize]) -> Self {
        self.bytes = val;
        self
    }
}
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
    pub fn luidDeviceNodeMask(mut self, val: ::std::os::raw::c_uint) -> Self {
        self.luidDeviceNodeMask = val;
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
    pub fn regsPerBlock(mut self, val: ::std::os::raw::c_int) -> Self {
        self.regsPerBlock = val;
        self
    }
    pub fn warpSize(mut self, val: ::std::os::raw::c_int) -> Self {
        self.warpSize = val;
        self
    }
    pub fn memPitch(mut self, val: usize) -> Self {
        self.memPitch = val;
        self
    }
    pub fn maxThreadsPerBlock(mut self, val: ::std::os::raw::c_int) -> Self {
        self.maxThreadsPerBlock = val;
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
    pub fn major(mut self, val: ::std::os::raw::c_int) -> Self {
        self.major = val;
        self
    }
    pub fn minor(mut self, val: ::std::os::raw::c_int) -> Self {
        self.minor = val;
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
    pub fn multiProcessorCount(mut self, val: ::std::os::raw::c_int) -> Self {
        self.multiProcessorCount = val;
        self
    }
    pub fn integrated(mut self, val: ::std::os::raw::c_int) -> Self {
        self.integrated = val;
        self
    }
    pub fn canMapHostMemory(mut self, val: ::std::os::raw::c_int) -> Self {
        self.canMapHostMemory = val;
        self
    }
    pub fn maxTexture1D(mut self, val: ::std::os::raw::c_int) -> Self {
        self.maxTexture1D = val;
        self
    }
    pub fn maxTexture1DMipmap(mut self, val: ::std::os::raw::c_int) -> Self {
        self.maxTexture1DMipmap = val;
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
    pub fn maxTextureCubemap(mut self, val: ::std::os::raw::c_int) -> Self {
        self.maxTextureCubemap = val;
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
    pub fn maxSurface1D(mut self, val: ::std::os::raw::c_int) -> Self {
        self.maxSurface1D = val;
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
    pub fn maxSurfaceCubemap(mut self, val: ::std::os::raw::c_int) -> Self {
        self.maxSurfaceCubemap = val;
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
    pub fn concurrentKernels(mut self, val: ::std::os::raw::c_int) -> Self {
        self.concurrentKernels = val;
        self
    }
    pub fn ECCEnabled(mut self, val: ::std::os::raw::c_int) -> Self {
        self.ECCEnabled = val;
        self
    }
    pub fn pciBusID(mut self, val: ::std::os::raw::c_int) -> Self {
        self.pciBusID = val;
        self
    }
    pub fn pciDeviceID(mut self, val: ::std::os::raw::c_int) -> Self {
        self.pciDeviceID = val;
        self
    }
    pub fn pciDomainID(mut self, val: ::std::os::raw::c_int) -> Self {
        self.pciDomainID = val;
        self
    }
    pub fn tccDriver(mut self, val: ::std::os::raw::c_int) -> Self {
        self.tccDriver = val;
        self
    }
    pub fn asyncEngineCount(mut self, val: ::std::os::raw::c_int) -> Self {
        self.asyncEngineCount = val;
        self
    }
    pub fn unifiedAddressing(mut self, val: ::std::os::raw::c_int) -> Self {
        self.unifiedAddressing = val;
        self
    }
    pub fn memoryBusWidth(mut self, val: ::std::os::raw::c_int) -> Self {
        self.memoryBusWidth = val;
        self
    }
    pub fn l2CacheSize(mut self, val: ::std::os::raw::c_int) -> Self {
        self.l2CacheSize = val;
        self
    }
    pub fn persistingL2CacheMaxSize(mut self, val: ::std::os::raw::c_int) -> Self {
        self.persistingL2CacheMaxSize = val;
        self
    }
    pub fn maxThreadsPerMultiProcessor(mut self, val: ::std::os::raw::c_int) -> Self {
        self.maxThreadsPerMultiProcessor = val;
        self
    }
    pub fn streamPrioritiesSupported(mut self, val: ::std::os::raw::c_int) -> Self {
        self.streamPrioritiesSupported = val;
        self
    }
    pub fn globalL1CacheSupported(mut self, val: ::std::os::raw::c_int) -> Self {
        self.globalL1CacheSupported = val;
        self
    }
    pub fn localL1CacheSupported(mut self, val: ::std::os::raw::c_int) -> Self {
        self.localL1CacheSupported = val;
        self
    }
    pub fn sharedMemPerMultiprocessor(mut self, val: usize) -> Self {
        self.sharedMemPerMultiprocessor = val;
        self
    }
    pub fn regsPerMultiprocessor(mut self, val: ::std::os::raw::c_int) -> Self {
        self.regsPerMultiprocessor = val;
        self
    }
    pub fn managedMemory(mut self, val: ::std::os::raw::c_int) -> Self {
        self.managedMemory = val;
        self
    }
    pub fn isMultiGpuBoard(mut self, val: ::std::os::raw::c_int) -> Self {
        self.isMultiGpuBoard = val;
        self
    }
    pub fn multiGpuBoardGroupID(mut self, val: ::std::os::raw::c_int) -> Self {
        self.multiGpuBoardGroupID = val;
        self
    }
    pub fn hostNativeAtomicSupported(mut self, val: ::std::os::raw::c_int) -> Self {
        self.hostNativeAtomicSupported = val;
        self
    }
    pub fn pageableMemoryAccess(mut self, val: ::std::os::raw::c_int) -> Self {
        self.pageableMemoryAccess = val;
        self
    }
    pub fn concurrentManagedAccess(mut self, val: ::std::os::raw::c_int) -> Self {
        self.concurrentManagedAccess = val;
        self
    }
    pub fn computePreemptionSupported(mut self, val: ::std::os::raw::c_int) -> Self {
        self.computePreemptionSupported = val;
        self
    }
    pub fn canUseHostPointerForRegisteredMem(mut self, val: ::std::os::raw::c_int) -> Self {
        self.canUseHostPointerForRegisteredMem = val;
        self
    }
    pub fn cooperativeLaunch(mut self, val: ::std::os::raw::c_int) -> Self {
        self.cooperativeLaunch = val;
        self
    }
    pub fn sharedMemPerBlockOptin(mut self, val: usize) -> Self {
        self.sharedMemPerBlockOptin = val;
        self
    }
    pub fn pageableMemoryAccessUsesHostPageTables(mut self, val: ::std::os::raw::c_int) -> Self {
        self.pageableMemoryAccessUsesHostPageTables = val;
        self
    }
    pub fn directManagedMemAccessFromHost(mut self, val: ::std::os::raw::c_int) -> Self {
        self.directManagedMemAccessFromHost = val;
        self
    }
    pub fn maxBlocksPerMultiProcessor(mut self, val: ::std::os::raw::c_int) -> Self {
        self.maxBlocksPerMultiProcessor = val;
        self
    }
    pub fn accessPolicyMaxWindowSize(mut self, val: ::std::os::raw::c_int) -> Self {
        self.accessPolicyMaxWindowSize = val;
        self
    }
    pub fn reservedSharedMemPerBlock(mut self, val: usize) -> Self {
        self.reservedSharedMemPerBlock = val;
        self
    }
    pub fn hostRegisterSupported(mut self, val: ::std::os::raw::c_int) -> Self {
        self.hostRegisterSupported = val;
        self
    }
    pub fn sparseCudaArraySupported(mut self, val: ::std::os::raw::c_int) -> Self {
        self.sparseCudaArraySupported = val;
        self
    }
    pub fn hostRegisterReadOnlySupported(mut self, val: ::std::os::raw::c_int) -> Self {
        self.hostRegisterReadOnlySupported = val;
        self
    }
    pub fn timelineSemaphoreInteropSupported(mut self, val: ::std::os::raw::c_int) -> Self {
        self.timelineSemaphoreInteropSupported = val;
        self
    }
    pub fn memoryPoolsSupported(mut self, val: ::std::os::raw::c_int) -> Self {
        self.memoryPoolsSupported = val;
        self
    }
    pub fn gpuDirectRDMASupported(mut self, val: ::std::os::raw::c_int) -> Self {
        self.gpuDirectRDMASupported = val;
        self
    }
    pub fn gpuDirectRDMAFlushWritesOptions(mut self, val: ::std::os::raw::c_uint) -> Self {
        self.gpuDirectRDMAFlushWritesOptions = val;
        self
    }
    pub fn gpuDirectRDMAWritesOrdering(mut self, val: ::std::os::raw::c_int) -> Self {
        self.gpuDirectRDMAWritesOrdering = val;
        self
    }
    pub fn memoryPoolSupportedHandleTypes(mut self, val: ::std::os::raw::c_uint) -> Self {
        self.memoryPoolSupportedHandleTypes = val;
        self
    }
    pub fn deferredMappingCudaArraySupported(mut self, val: ::std::os::raw::c_int) -> Self {
        self.deferredMappingCudaArraySupported = val;
        self
    }
    pub fn ipcEventSupported(mut self, val: ::std::os::raw::c_int) -> Self {
        self.ipcEventSupported = val;
        self
    }
    pub fn clusterLaunch(mut self, val: ::std::os::raw::c_int) -> Self {
        self.clusterLaunch = val;
        self
    }
    pub fn unifiedFunctionPointers(mut self, val: ::std::os::raw::c_int) -> Self {
        self.unifiedFunctionPointers = val;
        self
    }
    pub fn deviceNumaConfig(mut self, val: ::std::os::raw::c_int) -> Self {
        self.deviceNumaConfig = val;
        self
    }
    pub fn deviceNumaId(mut self, val: ::std::os::raw::c_int) -> Self {
        self.deviceNumaId = val;
        self
    }
    pub fn mpsEnabled(mut self, val: ::std::os::raw::c_int) -> Self {
        self.mpsEnabled = val;
        self
    }
    pub fn hostNumaId(mut self, val: ::std::os::raw::c_int) -> Self {
        self.hostNumaId = val;
        self
    }
    pub fn gpuPciDeviceID(mut self, val: ::std::os::raw::c_uint) -> Self {
        self.gpuPciDeviceID = val;
        self
    }
    pub fn gpuPciSubsystemID(mut self, val: ::std::os::raw::c_uint) -> Self {
        self.gpuPciSubsystemID = val;
        self
    }
    pub fn hostNumaMultinodeIpcSupported(mut self, val: ::std::os::raw::c_int) -> Self {
        self.hostNumaMultinodeIpcSupported = val;
        self
    }
    pub fn reserved(mut self, val: [::std::os::raw::c_int; 56usize]) -> Self {
        self.reserved = val;
        self
    }
}
impl crate::sys::cudaIpcEventHandle_st {
    pub fn reserved(mut self, val: [::std::os::raw::c_char; 64usize]) -> Self {
        self.reserved = val;
        self
    }
}
impl crate::sys::cudaIpcMemHandle_st {
    pub fn reserved(mut self, val: [::std::os::raw::c_char; 64usize]) -> Self {
        self.reserved = val;
        self
    }
}
impl crate::sys::cudaMemFabricHandle_st {
    pub fn reserved(mut self, val: [::std::os::raw::c_char; 64usize]) -> Self {
        self.reserved = val;
        self
    }
}
impl crate::sys::cudaExternalMemoryHandleDesc {
    pub fn type_(mut self, val: cudaExternalMemoryHandleType) -> Self {
        self.type_ = val;
        self
    }
    pub fn handle(mut self, val: cudaExternalMemoryHandleDesc__bindgen_ty_1) -> Self {
        self.handle = val;
        self
    }
    pub fn size(mut self, val: ::std::os::raw::c_ulonglong) -> Self {
        self.size = val;
        self
    }
    pub fn flags(mut self, val: ::std::os::raw::c_uint) -> Self {
        self.flags = val;
        self
    }
    pub fn reserved(mut self, val: [::std::os::raw::c_uint; 16usize]) -> Self {
        self.reserved = val;
        self
    }
}
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
impl crate::sys::cudaExternalMemoryBufferDesc {
    pub fn offset(mut self, val: ::std::os::raw::c_ulonglong) -> Self {
        self.offset = val;
        self
    }
    pub fn size(mut self, val: ::std::os::raw::c_ulonglong) -> Self {
        self.size = val;
        self
    }
    pub fn flags(mut self, val: ::std::os::raw::c_uint) -> Self {
        self.flags = val;
        self
    }
    pub fn reserved(mut self, val: [::std::os::raw::c_uint; 16usize]) -> Self {
        self.reserved = val;
        self
    }
}
impl crate::sys::cudaExternalMemoryMipmappedArrayDesc {
    pub fn offset(mut self, val: ::std::os::raw::c_ulonglong) -> Self {
        self.offset = val;
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
    pub fn flags(mut self, val: ::std::os::raw::c_uint) -> Self {
        self.flags = val;
        self
    }
    pub fn numLevels(mut self, val: ::std::os::raw::c_uint) -> Self {
        self.numLevels = val;
        self
    }
    pub fn reserved(mut self, val: [::std::os::raw::c_uint; 16usize]) -> Self {
        self.reserved = val;
        self
    }
}
impl crate::sys::cudaExternalSemaphoreHandleDesc {
    pub fn type_(mut self, val: cudaExternalSemaphoreHandleType) -> Self {
        self.type_ = val;
        self
    }
    pub fn handle(mut self, val: cudaExternalSemaphoreHandleDesc__bindgen_ty_1) -> Self {
        self.handle = val;
        self
    }
    pub fn flags(mut self, val: ::std::os::raw::c_uint) -> Self {
        self.flags = val;
        self
    }
    pub fn reserved(mut self, val: [::std::os::raw::c_uint; 16usize]) -> Self {
        self.reserved = val;
        self
    }
}
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
impl crate::sys::cudaExternalSemaphoreSignalParams {
    pub fn params(mut self, val: cudaExternalSemaphoreSignalParams__bindgen_ty_1) -> Self {
        self.params = val;
        self
    }
    pub fn flags(mut self, val: ::std::os::raw::c_uint) -> Self {
        self.flags = val;
        self
    }
    pub fn reserved(mut self, val: [::std::os::raw::c_uint; 16usize]) -> Self {
        self.reserved = val;
        self
    }
}
impl crate::sys::cudaExternalSemaphoreSignalParams__bindgen_ty_1 {
    pub fn fence(
        mut self,
        val: cudaExternalSemaphoreSignalParams__bindgen_ty_1__bindgen_ty_1,
    ) -> Self {
        self.fence = val;
        self
    }
    pub fn nvSciSync(
        mut self,
        val: cudaExternalSemaphoreSignalParams__bindgen_ty_1__bindgen_ty_2,
    ) -> Self {
        self.nvSciSync = val;
        self
    }
    pub fn keyedMutex(
        mut self,
        val: cudaExternalSemaphoreSignalParams__bindgen_ty_1__bindgen_ty_3,
    ) -> Self {
        self.keyedMutex = val;
        self
    }
    pub fn reserved(mut self, val: [::std::os::raw::c_uint; 12usize]) -> Self {
        self.reserved = val;
        self
    }
}
impl crate::sys::cudaExternalSemaphoreSignalParams__bindgen_ty_1__bindgen_ty_1 {
    pub fn value(mut self, val: ::std::os::raw::c_ulonglong) -> Self {
        self.value = val;
        self
    }
}
impl crate::sys::cudaExternalSemaphoreSignalParams__bindgen_ty_1__bindgen_ty_3 {
    pub fn key(mut self, val: ::std::os::raw::c_ulonglong) -> Self {
        self.key = val;
        self
    }
}
impl crate::sys::cudaExternalSemaphoreWaitParams {
    pub fn params(mut self, val: cudaExternalSemaphoreWaitParams__bindgen_ty_1) -> Self {
        self.params = val;
        self
    }
    pub fn flags(mut self, val: ::std::os::raw::c_uint) -> Self {
        self.flags = val;
        self
    }
    pub fn reserved(mut self, val: [::std::os::raw::c_uint; 16usize]) -> Self {
        self.reserved = val;
        self
    }
}
impl crate::sys::cudaExternalSemaphoreWaitParams__bindgen_ty_1 {
    pub fn fence(
        mut self,
        val: cudaExternalSemaphoreWaitParams__bindgen_ty_1__bindgen_ty_1,
    ) -> Self {
        self.fence = val;
        self
    }
    pub fn nvSciSync(
        mut self,
        val: cudaExternalSemaphoreWaitParams__bindgen_ty_1__bindgen_ty_2,
    ) -> Self {
        self.nvSciSync = val;
        self
    }
    pub fn keyedMutex(
        mut self,
        val: cudaExternalSemaphoreWaitParams__bindgen_ty_1__bindgen_ty_3,
    ) -> Self {
        self.keyedMutex = val;
        self
    }
    pub fn reserved(mut self, val: [::std::os::raw::c_uint; 10usize]) -> Self {
        self.reserved = val;
        self
    }
}
impl crate::sys::cudaExternalSemaphoreWaitParams__bindgen_ty_1__bindgen_ty_1 {
    pub fn value(mut self, val: ::std::os::raw::c_ulonglong) -> Self {
        self.value = val;
        self
    }
}
impl crate::sys::cudaExternalSemaphoreWaitParams__bindgen_ty_1__bindgen_ty_3 {
    pub fn key(mut self, val: ::std::os::raw::c_ulonglong) -> Self {
        self.key = val;
        self
    }
    pub fn timeoutMs(mut self, val: ::std::os::raw::c_uint) -> Self {
        self.timeoutMs = val;
        self
    }
}
impl crate::sys::cudaDevSmResource {
    pub fn smCount(mut self, val: ::std::os::raw::c_uint) -> Self {
        self.smCount = val;
        self
    }
    pub fn minSmPartitionSize(mut self, val: ::std::os::raw::c_uint) -> Self {
        self.minSmPartitionSize = val;
        self
    }
    pub fn smCoscheduledAlignment(mut self, val: ::std::os::raw::c_uint) -> Self {
        self.smCoscheduledAlignment = val;
        self
    }
    pub fn flags(mut self, val: ::std::os::raw::c_uint) -> Self {
        self.flags = val;
        self
    }
}
impl crate::sys::cudaDevWorkqueueConfigResource {
    pub fn device(mut self, val: ::std::os::raw::c_int) -> Self {
        self.device = val;
        self
    }
    pub fn wqConcurrencyLimit(mut self, val: ::std::os::raw::c_uint) -> Self {
        self.wqConcurrencyLimit = val;
        self
    }
    pub fn sharingScope(mut self, val: cudaDevWorkqueueConfigScope) -> Self {
        self.sharingScope = val;
        self
    }
}
impl crate::sys::cudaDevWorkqueueResource {
    pub fn reserved(mut self, val: [::std::os::raw::c_uchar; 40usize]) -> Self {
        self.reserved = val;
        self
    }
}
impl crate::sys::cudaDevSmResourceGroupParams_st {
    pub fn smCount(mut self, val: ::std::os::raw::c_uint) -> Self {
        self.smCount = val;
        self
    }
    pub fn coscheduledSmCount(mut self, val: ::std::os::raw::c_uint) -> Self {
        self.coscheduledSmCount = val;
        self
    }
    pub fn preferredCoscheduledSmCount(mut self, val: ::std::os::raw::c_uint) -> Self {
        self.preferredCoscheduledSmCount = val;
        self
    }
    pub fn flags(mut self, val: ::std::os::raw::c_uint) -> Self {
        self.flags = val;
        self
    }
    pub fn reserved(mut self, val: [::std::os::raw::c_uint; 12usize]) -> Self {
        self.reserved = val;
        self
    }
}
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
    pub fn sharedMemBytes(mut self, val: ::std::os::raw::c_uint) -> Self {
        self.sharedMemBytes = val;
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
    pub fn sharedMemBytes(mut self, val: ::std::os::raw::c_uint) -> Self {
        self.sharedMemBytes = val;
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
impl crate::sys::cudaExternalSemaphoreSignalNodeParams {
    pub fn extSemArray(mut self, val: *mut cudaExternalSemaphore_t) -> Self {
        self.extSemArray = val;
        self
    }
    pub fn paramsArray(mut self, val: *const cudaExternalSemaphoreSignalParams) -> Self {
        self.paramsArray = val;
        self
    }
    pub fn numExtSems(mut self, val: ::std::os::raw::c_uint) -> Self {
        self.numExtSems = val;
        self
    }
}
impl crate::sys::cudaExternalSemaphoreSignalNodeParamsV2 {
    pub fn extSemArray(mut self, val: *mut cudaExternalSemaphore_t) -> Self {
        self.extSemArray = val;
        self
    }
    pub fn paramsArray(mut self, val: *const cudaExternalSemaphoreSignalParams) -> Self {
        self.paramsArray = val;
        self
    }
    pub fn numExtSems(mut self, val: ::std::os::raw::c_uint) -> Self {
        self.numExtSems = val;
        self
    }
}
impl crate::sys::cudaExternalSemaphoreWaitNodeParams {
    pub fn extSemArray(mut self, val: *mut cudaExternalSemaphore_t) -> Self {
        self.extSemArray = val;
        self
    }
    pub fn paramsArray(mut self, val: *const cudaExternalSemaphoreWaitParams) -> Self {
        self.paramsArray = val;
        self
    }
    pub fn numExtSems(mut self, val: ::std::os::raw::c_uint) -> Self {
        self.numExtSems = val;
        self
    }
}
impl crate::sys::cudaExternalSemaphoreWaitNodeParamsV2 {
    pub fn extSemArray(mut self, val: *mut cudaExternalSemaphore_t) -> Self {
        self.extSemArray = val;
        self
    }
    pub fn paramsArray(mut self, val: *const cudaExternalSemaphoreWaitParams) -> Self {
        self.paramsArray = val;
        self
    }
    pub fn numExtSems(mut self, val: ::std::os::raw::c_uint) -> Self {
        self.numExtSems = val;
        self
    }
}
impl crate::sys::cudaConditionalNodeParams {
    pub fn handle(mut self, val: cudaGraphConditionalHandle) -> Self {
        self.handle = val;
        self
    }
    pub fn type_(mut self, val: cudaGraphConditionalNodeType) -> Self {
        self.type_ = val;
        self
    }
    pub fn size(mut self, val: ::std::os::raw::c_uint) -> Self {
        self.size = val;
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
impl crate::sys::cudaEventRecordNodeParams {
    pub fn event(mut self, val: cudaEvent_t) -> Self {
        self.event = val;
        self
    }
}
impl crate::sys::cudaEventWaitNodeParams {
    pub fn event(mut self, val: cudaEvent_t) -> Self {
        self.event = val;
        self
    }
}
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
    pub fn reserved2(mut self, val: ::std::os::raw::c_longlong) -> Self {
        self.reserved2 = val;
        self
    }
}
impl crate::sys::cudaGraphEdgeData_st {
    pub fn from_port(mut self, val: ::std::os::raw::c_uchar) -> Self {
        self.from_port = val;
        self
    }
    pub fn to_port(mut self, val: ::std::os::raw::c_uchar) -> Self {
        self.to_port = val;
        self
    }
    pub fn type_(mut self, val: ::std::os::raw::c_uchar) -> Self {
        self.type_ = val;
        self
    }
    pub fn reserved(mut self, val: [::std::os::raw::c_uchar; 5usize]) -> Self {
        self.reserved = val;
        self
    }
}
impl crate::sys::cudaGraphInstantiateParams_st {
    pub fn flags(mut self, val: ::std::os::raw::c_ulonglong) -> Self {
        self.flags = val;
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
impl crate::sys::cudaLaunchMemSyncDomainMap_st {
    pub fn default_(mut self, val: ::std::os::raw::c_uchar) -> Self {
        self.default_ = val;
        self
    }
    pub fn remote(mut self, val: ::std::os::raw::c_uchar) -> Self {
        self.remote = val;
        self
    }
}
impl crate::sys::cudaLaunchAttributeValue__bindgen_ty_1 {
    pub fn x(mut self, val: ::std::os::raw::c_uint) -> Self {
        self.x = val;
        self
    }
    pub fn y(mut self, val: ::std::os::raw::c_uint) -> Self {
        self.y = val;
        self
    }
    pub fn z(mut self, val: ::std::os::raw::c_uint) -> Self {
        self.z = val;
        self
    }
}
impl crate::sys::cudaLaunchAttributeValue__bindgen_ty_2 {
    pub fn event(mut self, val: cudaEvent_t) -> Self {
        self.event = val;
        self
    }
    pub fn flags(mut self, val: ::std::os::raw::c_int) -> Self {
        self.flags = val;
        self
    }
    pub fn triggerAtBlockStart(mut self, val: ::std::os::raw::c_int) -> Self {
        self.triggerAtBlockStart = val;
        self
    }
}
impl crate::sys::cudaLaunchAttributeValue__bindgen_ty_3 {
    pub fn x(mut self, val: ::std::os::raw::c_uint) -> Self {
        self.x = val;
        self
    }
    pub fn y(mut self, val: ::std::os::raw::c_uint) -> Self {
        self.y = val;
        self
    }
    pub fn z(mut self, val: ::std::os::raw::c_uint) -> Self {
        self.z = val;
        self
    }
}
impl crate::sys::cudaLaunchAttributeValue__bindgen_ty_4 {
    pub fn event(mut self, val: cudaEvent_t) -> Self {
        self.event = val;
        self
    }
    pub fn flags(mut self, val: ::std::os::raw::c_int) -> Self {
        self.flags = val;
        self
    }
}
impl crate::sys::cudaLaunchAttributeValue__bindgen_ty_5 {
    pub fn deviceUpdatable(mut self, val: ::std::os::raw::c_int) -> Self {
        self.deviceUpdatable = val;
        self
    }
    pub fn devNode(mut self, val: cudaGraphDeviceNode_t) -> Self {
        self.devNode = val;
        self
    }
}
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
    pub fn numAttrs(mut self, val: ::std::os::raw::c_uint) -> Self {
        self.numAttrs = val;
        self
    }
}
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
impl crate::sys::cudaAsyncNotificationInfo__bindgen_ty_1__bindgen_ty_1 {
    pub fn bytesOverBudget(mut self, val: ::std::os::raw::c_ulonglong) -> Self {
        self.bytesOverBudget = val;
        self
    }
}
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
    pub fn sRGB(mut self, val: ::std::os::raw::c_int) -> Self {
        self.sRGB = val;
        self
    }
    pub fn borderColor(mut self, val: [f32; 4usize]) -> Self {
        self.borderColor = val;
        self
    }
    pub fn normalizedCoords(mut self, val: ::std::os::raw::c_int) -> Self {
        self.normalizedCoords = val;
        self
    }
    pub fn maxAnisotropy(mut self, val: ::std::os::raw::c_uint) -> Self {
        self.maxAnisotropy = val;
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
    pub fn disableTrilinearOptimization(mut self, val: ::std::os::raw::c_int) -> Self {
        self.disableTrilinearOptimization = val;
        self
    }
    pub fn seamlessCubemap(mut self, val: ::std::os::raw::c_int) -> Self {
        self.seamlessCubemap = val;
        self
    }
}
pub struct CudaExecutionContext {
    pub(crate) handle: crate::sys::cudaExecutionContext_t,
}
impl CudaExecutionContext {
    pub fn cudaGreenCtxCreate(
        desc: crate::sys::cudaDevResourceDesc_t,
        device: std::os::raw::c_int,
        flags: std::os::raw::c_uint,
    ) -> Result<Self, crate::sys::cudaError> {
        unsafe {
            let mut handle = std::ptr::null_mut();
            let status = crate::sys::cudaGreenCtxCreate(&mut handle, desc, device, flags);
            if status == crate::sys::cudaError::cudaSuccess {
                Ok(Self { handle })
            } else {
                Err(status)
            }
        }
    }
    pub fn cudaDeviceGetExecutionCtx(
        device: std::os::raw::c_int,
    ) -> Result<Self, crate::sys::cudaError> {
        unsafe {
            let mut handle = std::ptr::null_mut();
            let status = crate::sys::cudaDeviceGetExecutionCtx(&mut handle, device);
            if status == crate::sys::cudaError::cudaSuccess {
                Ok(Self { handle })
            } else {
                Err(status)
            }
        }
    }
    #[doc = " \\brief Create a conditional handle\n\n Creates a conditional handle associated with \\p hGraph.\n\n The conditional handle must be associated with a conditional node in this graph or one of its children.\n\n Handles not associated with a conditional node may cause graph instantiation to fail.\n\n \\param pHandle_out        - Pointer used to return the handle to the caller.\n \\param graph              - Graph which will contain the conditional node using this handle.\n \\param ctx                - Execution context for the handle and associated conditional node. If NULL, current context will be used.\n \\param defaultLaunchValue - Optional initial value for the conditional variable.\n                             Applied at the beginning of each graph execution if cudaGraphCondAssignDefault is set in \\p flags.\n \\param flags              - Currently must be cudaGraphCondAssignDefault or 0.\n\n \\return\n ::CUDA_SUCCESS,\n ::CUDA_ERROR_INVALID_VALUE,\n ::CUDA_ERROR_NOT_SUPPORTED\n \\note_graph_thread_safety\n \\notefnerr\n\n \\sa\n ::cuGraphAddNode,"]
    pub unsafe fn cudaGraphConditionalHandleCreate_v2(
        &self,
        pHandle_out: *mut cudaGraphConditionalHandle,
        graph: cudaGraph_t,
        defaultLaunchValue: ::std::os::raw::c_uint,
        flags: ::std::os::raw::c_uint,
    ) -> Result<(), crate::sys::cudaError> {
        let status = unsafe {
            crate::sys::cudaGraphConditionalHandleCreate_v2(
                pHandle_out,
                graph,
                self.handle,
                defaultLaunchValue,
                flags,
            )
        };
        if status == crate::sys::cudaError::cudaSuccess {
            Ok(())
        } else {
            Err(status)
        }
    }
    #[doc = " \\brief Get context resources\n\n Get the \\p type resources available to context represented by \\p ctx.\n\n Note: The API is not supported on 32-bit platforms.\n\n \\param ctx - Execution context to get resource for (required parameter, see note below)\n \\param resource - Output pointer to a cudaDevResource structure\n \\param type - Type of resource to retrieve\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorNotSupported,\n ::cudaErrorNotPermitted,\n ::cudaErrorCudartUnloading,\n ::cudaErrorInitializationError\n \\notefnerr\n \\note_callback\n \\note_cudaExecutionContext_t_required_param\n\n \\sa\n ::cudaDeviceGetDevResource,\n ::cudaDevSmResourceSplit,\n ::cudaDevResourceGenerateDesc,\n ::cudaGreenCtxCreate"]
    pub unsafe fn cudaExecutionCtxGetDevResource(
        &self,
        type_: cudaDevResourceType,
    ) -> Result<cudaDevResource, crate::sys::cudaError> {
        let mut out_1: std::mem::MaybeUninit<cudaDevResource> = std::mem::MaybeUninit::uninit();
        let status = unsafe {
            crate::sys::cudaExecutionCtxGetDevResource(
                self.handle,
                out_1.as_mut_ptr() as *mut _,
                type_,
            )
        };
        if status == crate::sys::cudaError::cudaSuccess {
            unsafe { Ok(out_1.assume_init()) }
        } else {
            Err(status)
        }
    }
    #[doc = " \\brief Returns the device handle for the execution context\n\n Returns in \\p *device the handle of the specified execution context's device.\n The execution context should not be NULL.\n\n \\param device - Returned device handle for the specified execution context\n \\param ctx - Execution context for which to obtain the device (required parameter, see note below)\n\n \\return\n ::cudaSuccess,\n ::cudaErrorCudartUnloading,\n ::cudaErrorInitializationError,\n ::cudaErrorInvalidValue,\n ::cudaErrorNotPermitted\n \\notefnerr\n \\note_callback\n \\note_cudaExecutionContext_t_required_param\n\n \\sa ::cudaGreenCtxCreate,\n ::cudaExecutionCtxDestroy,\n ::cuCtxGetDevice"]
    pub unsafe fn cudaExecutionCtxGetDevice(
        &self,
    ) -> Result<::std::os::raw::c_int, crate::sys::cudaError> {
        let mut out_0: std::mem::MaybeUninit<::std::os::raw::c_int> =
            std::mem::MaybeUninit::uninit();
        let status = unsafe {
            crate::sys::cudaExecutionCtxGetDevice(out_0.as_mut_ptr() as *mut _, self.handle)
        };
        if status == crate::sys::cudaError::cudaSuccess {
            unsafe { Ok(out_0.assume_init()) }
        } else {
            Err(status)
        }
    }
    #[doc = " \\brief Returns the unique Id associated with the execution context supplied\n\n Returns in \\p ctxId the unique Id which is associated with a given context.\n The Id is unique for the life of the program for this instance of CUDA.\n The execution context should not be NULL.\n\n \\param ctx - Context for which to obtain the Id (required parameter, see note below)\n \\param ctxId - Pointer to store the Id of the context\n\n \\return\n ::cudaSuccess,\n ::cudaErrorCudartUnloading,\n ::cudaErrorInitializationError,\n ::cudaErrorInvalidValue,\n ::cudaErrorNotPermitted\n \\notefnerr\n \\note_callback\n \\note_cudaExecutionContext_t_required_param\n\n \\sa ::cudaGreenCtxCreate,\n ::cudaExecutionCtxDestroy,\n ::cudaExecutionCtxGetDevice,\n ::cuCtxGetId"]
    pub unsafe fn cudaExecutionCtxGetId(
        &self,
    ) -> Result<::std::os::raw::c_ulonglong, crate::sys::cudaError> {
        let mut out_1: std::mem::MaybeUninit<::std::os::raw::c_ulonglong> =
            std::mem::MaybeUninit::uninit();
        let status =
            unsafe { crate::sys::cudaExecutionCtxGetId(self.handle, out_1.as_mut_ptr() as *mut _) };
        if status == crate::sys::cudaError::cudaSuccess {
            unsafe { Ok(out_1.assume_init()) }
        } else {
            Err(status)
        }
    }
    #[doc = " \\brief Creates a stream and initializes it for the given execution context.\n\n The API creates a CUDA stream with the specified \\p flags and \\p priority,\n initializing it with resources as defined at the time of creating the specified \\p ctx.\n Additionally, the API also enables work submitted to to the stream to be tracked under \\p ctx.\n\n The supported values for \\p flags are:\n - ::cudaStreamDefault: Default stream creation flag. This would be ::cudaStreamNonBlocking for\n   streams created on a green context.\n - ::cudaStreamNonBlocking: Specifies that work running in the created stream may run concurrently\n   with work in stream 0 (the NULL stream), and that the created stream should perform no implicit\n   synchronization with stream 0\n\n Specifying \\p priority affects the scheduling priority of work in the stream. Priorities provide a\n hint to preferentially run work with higher priority when possible, but do not preempt\n already-running work or provide any other functional guarantee on execution order.\n \\p priority follows a convention where lower numbers represent higher priorities.\n '0' represents default priority. The range of meaningful numerical priorities can\n be queried using ::cudaDeviceGetStreamPriorityRange. If the specified priority is\n outside the numerical range returned by ::cudaDeviceGetStreamPriorityRange,\n it will automatically be clamped to the lowest or the highest number in the range.\n\n \\param phStream - Returned stream handle\n \\param ctx      - Execution context to initialize the stream with (required parameter, see note below)\n \\param flags    - Flags for stream creation\n \\param priority - Stream priority\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorNotPermitted,\n ::cudaErrorOutOfMemory,\n ::cudaErrorCudartUnloading,\n ::cudaErrorInitializationError\n \\notefnerr\n \\note_callback\n \\note_cudaExecutionContext_t_required_param\n\n \\note In the current implementation, only compute kernels launched in\n priority streams are affected by the stream's priority. Stream priorities have\n no effect on host-to-device and device-to-host memory operations.\n\n \\sa ::cudaStreamDestroy,\n ::cudaGreenCtxCreate,\n ::cudaDeviceGetStreamPriorityRange,\n ::cudaStreamGetFlags,\n ::cudaStreamGetPriority,\n ::cudaStreamGetDevice,\n ::cudaStreamGetDevResource,\n ::cudaLaunchKernel,\n ::cudaEventRecord,\n ::cudaStreamWaitEvent,\n ::cudaStreamQuery,\n ::cudaStreamSynchronize,\n ::cudaStreamAddCallback"]
    pub unsafe fn cudaExecutionCtxStreamCreate(
        &self,
        phStream: *mut cudaStream_t,
        flags: ::std::os::raw::c_uint,
        priority: ::std::os::raw::c_int,
    ) -> Result<(), crate::sys::cudaError> {
        let status = unsafe {
            crate::sys::cudaExecutionCtxStreamCreate(phStream, self.handle, flags, priority)
        };
        if status == crate::sys::cudaError::cudaSuccess {
            Ok(())
        } else {
            Err(status)
        }
    }
    #[doc = " \\brief Block for the specified execution context's tasks to complete\n\n Blocks until the specified execution context has completed all preceding requested tasks.\n If the specified execution context is the device (primary) context obtained via ::cudaDeviceGetExecutionCtx,\n green contexts that have been created on the device will also be synchronized.\n\n The API returns an error if one of the preceding tasks failed.\n\n \\param ctx - Execution context to synchronize (required parameter, see note below)\n\n \\return\n ::cudaSuccess,\n ::cudaErrorCudartUnloading,\n ::cudaErrorDeviceUninitialized,\n ::cudaErrorInvalidValue\n \\notefnerr\n \\note_callback\n \\note_cudaExecutionContext_t_required_param\n\n \\sa\n ::cudaGreenCtxCreate,\n ::cudaExecutionCtxDestroy,\n ::cudaDeviceSynchronize,\n ::cuCtxSynchronize_v2"]
    pub unsafe fn cudaExecutionCtxSynchronize(&self) -> Result<(), crate::sys::cudaError> {
        let status = unsafe { crate::sys::cudaExecutionCtxSynchronize(self.handle) };
        if status == crate::sys::cudaError::cudaSuccess {
            Ok(())
        } else {
            Err(status)
        }
    }
    #[doc = " \\brief Records an event for the specified execution context\n\n Captures in \\p event all the activities of the execution context \\p ctx\n at the time of this call. \\p event and \\p ctx must be from the same\n CUDA device, otherwise ::cudaErrorInvalidHandle will be returned.\n Calls such as ::cudaEventQuery() or ::cudaExecutionCtxWaitEvent() will then examine\n or wait for completion of the work that was captured.\n Uses of \\p ctx after this call do not modify \\p event.\n If the execution context passed to \\p ctx is the device (primary) context obtained via\n ::cudaDeviceGetExecutionCtx(), \\p event will capture all the activities of the green\n contexts created on the device as well.\n\n \\note The API will return ::cudaErrorStreamCaptureUnsupported if the\n specified execution context \\p ctx has a stream in the capture mode. In such a case,\n the call will invalidate all the conflicting captures.\n\n \\param ctx - Execution context to record event for (required parameter, see note below)\n \\param event - Event to record\n\n \\return\n ::cudaSuccess,\n ::cudaErrorCudartUnloading,\n ::cudaErrorInitializationError,\n ::cudaErrorInvalidHandle,\n ::cudaErrorStreamCaptureUnsupported\n \\notefnerr\n \\note_callback\n \\note_cudaExecutionContext_t_required_param\n\n \\sa\n ::cudaEventRecord,\n ::cudaExecutionCtxWaitEvent,\n ::cuCtxRecordEvent,\n ::cuGreenCtxRecordEvent"]
    pub unsafe fn cudaExecutionCtxRecordEvent(
        &self,
        event: cudaEvent_t,
    ) -> Result<(), crate::sys::cudaError> {
        let status = unsafe { crate::sys::cudaExecutionCtxRecordEvent(self.handle, event) };
        if status == crate::sys::cudaError::cudaSuccess {
            Ok(())
        } else {
            Err(status)
        }
    }
    #[doc = " \\brief Make an execution context wait on an event\n\n Makes all future work submitted to execution context \\p ctx wait for all work\n captured in \\p event. The synchronization will be performed on the device\n and will not block the calling CPU thread. See ::cudaExecutionCtxRecordEvent()\n for details on what is captured by an event.\n If the execution context passed to \\p ctx is the device (primary) context obtained via\n ::cudaDeviceGetExecutionCtx(), all green contexts created on the device will wait for\n \\p event as well.\n\n \\note \\p event may be from a different execution context or device than \\p ctx.\n\n \\note The API will return ::cudaErrorStreamCaptureUnsupported and\n invalidate the capture if the specified event \\p event is part of an ongoing\n capture sequence or if the specified execution context \\p ctx has a stream in the capture mode.\n\n \\param ctx    - Execution context to wait for (required parameter, see note below)\n \\param event  - Event to wait on\n\n \\return\n ::cudaSuccess,\n ::cudaErrorCudartUnloading,\n ::cudaErrorInitializationError,\n ::cudaErrorInvalidHandle,\n ::cudaErrorStreamCaptureUnsupported\n \\notefnerr\n \\note_callback\n \\note_cudaExecutionContext_t_required_param\n\n \\sa\n ::cudaExecutionCtxRecordEvent,\n ::cudaStreamWaitEvent,\n ::cuCtxWaitEvent,\n ::cuGreenCtxWaitEvent"]
    pub unsafe fn cudaExecutionCtxWaitEvent(
        &self,
        event: cudaEvent_t,
    ) -> Result<(), crate::sys::cudaError> {
        let status = unsafe { crate::sys::cudaExecutionCtxWaitEvent(self.handle, event) };
        if status == crate::sys::cudaError::cudaSuccess {
            Ok(())
        } else {
            Err(status)
        }
    }
}
impl Drop for CudaExecutionContext {
    fn drop(&mut self) {
        unsafe {
            let _ = crate::sys::cudaExecutionCtxDestroy(self.handle);
        }
    }
}
#[doc = " \\brief Destroy all allocations and reset all state on the current device\n in the current process.\n\n Explicitly destroys and cleans up all resources associated with the current\n device in the current process. It is the caller's responsibility to ensure\n that the resources are not accessed or passed in subsequent API calls and\n doing so will result in undefined behavior. These resources include CUDA types\n ::cudaStream_t, ::cudaEvent_t, ::cudaArray_t, ::cudaMipmappedArray_t, ::cudaPitchedPtr,\n ::cudaTextureObject_t, ::cudaSurfaceObject_t, ::textureReference, ::surfaceReference,\n ::cudaExternalMemory_t, ::cudaExternalSemaphore_t and ::cudaGraphicsResource_t.\n These resources also include memory allocations by ::cudaMalloc, ::cudaMallocHost,\n ::cudaMallocManaged and ::cudaMallocPitch.\n Any subsequent API call to this device will reinitialize the device.\n\n Note that this function will reset the device immediately.  It is the caller's\n responsibility to ensure that the device is not being accessed by any\n other host threads from the process when this function is called.\n\n \\note ::cudaDeviceReset() will not destroy memory allocations by ::cudaMallocAsync() and\n ::cudaMallocFromPoolAsync(). These memory allocations need to be destroyed explicitly.\n \\note If a non-primary ::CUcontext is current to the thread, ::cudaDeviceReset()\n will destroy only the internal CUDA RT state for that ::CUcontext.\n\n \\return\n ::cudaSuccess\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaDeviceSynchronize"]
pub unsafe fn cudaDeviceReset() -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaDeviceReset() };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Wait for compute device to finish\n\n Blocks until the device has completed all preceding requested tasks.\n ::cudaDeviceSynchronize() returns an error if one of the preceding tasks\n has failed. If the ::cudaDeviceScheduleBlockingSync flag was set for\n this device, the host thread will block until the device has finished\n its work.\n\n \\return\n ::cudaSuccess,\n ::cudaErrorStreamCaptureUnsupported\n \\note_device_sync_deprecated\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa\n ::cudaDeviceReset,\n ::cuCtxSynchronize"]
pub unsafe fn cudaDeviceSynchronize() -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaDeviceSynchronize() };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Set resource limits\n\n Setting \\p limit to \\p value is a request by the application to update\n the current limit maintained by the device.  The driver is free to\n modify the requested value to meet h/w requirements (this could be\n clamping to minimum or maximum values, rounding up to nearest element\n size, etc).  The application can use ::cudaDeviceGetLimit() to find out\n exactly what the limit has been set to.\n\n Setting each ::cudaLimit has its own specific restrictions, so each is\n discussed here.\n\n - ::cudaLimitStackSize controls the stack size in bytes of each GPU thread.\n\n - ::cudaLimitPrintfFifoSize controls the size in bytes of the shared FIFO\n   used by the ::printf() device system call. Setting\n   ::cudaLimitPrintfFifoSize must not be performed after launching any kernel\n   that uses the ::printf() device system call - in such case\n   ::cudaErrorInvalidValue will be returned.\n\n - ::cudaLimitMallocHeapSize controls the size in bytes of the heap used by\n   the ::malloc() and ::free() device system calls. Setting\n   ::cudaLimitMallocHeapSize must not be performed after launching any kernel\n   that uses the ::malloc() or ::free() device system calls - in such case\n   ::cudaErrorInvalidValue will be returned.\n\n - ::cudaLimitDevRuntimeSyncDepth controls the maximum nesting depth of a\n   grid at which a thread can safely call ::cudaDeviceSynchronize(). Setting\n   this limit must be performed before any launch of a kernel that uses the\n   device runtime and calls ::cudaDeviceSynchronize() above the default sync\n   depth, two levels of grids. Calls to ::cudaDeviceSynchronize() will fail\n   with error code ::cudaErrorSyncDepthExceeded if the limitation is\n   violated. This limit can be set smaller than the default or up the maximum\n   launch depth of 24. When setting this limit, keep in mind that additional\n   levels of sync depth require the runtime to reserve large amounts of\n   device memory which can no longer be used for user allocations. If these\n   reservations of device memory fail, ::cudaDeviceSetLimit will return\n   ::cudaErrorMemoryAllocation, and the limit can be reset to a lower value.\n   This limit is only applicable to devices of compute capability < 9.0.\n   Attempting to set this limit on devices of other compute capability will\n   results in error ::cudaErrorUnsupportedLimit being returned.\n\n - ::cudaLimitDevRuntimePendingLaunchCount controls the maximum number of\n   outstanding device runtime launches that can be made from the current\n   device. A grid is outstanding from the point of launch up until the grid\n   is known to have been completed. Device runtime launches which violate\n   this limitation fail and return ::cudaErrorLaunchPendingCountExceeded when\n   ::cudaGetLastError() is called after launch. If more pending launches than\n   the default (2048 launches) are needed for a module using the device\n   runtime, this limit can be increased. Keep in mind that being able to\n   sustain additional pending launches will require the runtime to reserve\n   larger amounts of device memory upfront which can no longer be used for\n   allocations. If these reservations fail, ::cudaDeviceSetLimit will return\n   ::cudaErrorMemoryAllocation, and the limit can be reset to a lower value.\n   This limit is only applicable to devices of compute capability 3.5 and\n   higher. Attempting to set this limit on devices of compute capability less\n   than 3.5 will result in the error ::cudaErrorUnsupportedLimit being\n   returned.\n\n - ::cudaLimitMaxL2FetchGranularity controls the L2 cache fetch granularity.\n   Values can range from 0B to 128B. This is purely a performance hint and\n   it can be ignored or clamped depending on the platform.\n\n - ::cudaLimitPersistingL2CacheSize controls size in bytes available\n   for persisting L2 cache. This is purely a performance hint and it\n   can be ignored or clamped depending on the platform.\n\n \\param limit - Limit to set\n \\param value - Size of limit\n\n \\return\n ::cudaSuccess,\n ::cudaErrorUnsupportedLimit,\n ::cudaErrorInvalidValue,\n ::cudaErrorMemoryAllocation\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa\n ::cudaDeviceGetLimit,\n ::cuCtxSetLimit"]
pub unsafe fn cudaDeviceSetLimit(
    limit: cudaLimit,
    value: usize,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaDeviceSetLimit(limit, value) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Return resource limits\n\n Returns in \\p *pValue the current size of \\p limit. The following ::cudaLimit values are supported.\n - ::cudaLimitStackSize is the stack size in bytes of each GPU thread.\n - ::cudaLimitPrintfFifoSize is the size in bytes of the shared FIFO used by the\n   ::printf() device system call.\n - ::cudaLimitMallocHeapSize is the size in bytes of the heap used by the\n   ::malloc() and ::free() device system calls.\n - ::cudaLimitDevRuntimeSyncDepth is the maximum grid depth at which a\n   thread can isssue the device runtime call ::cudaDeviceSynchronize()\n   to wait on child grid launches to complete. This functionality is removed\n   for devices of compute capability >= 9.0, and hence will return error\n   ::cudaErrorUnsupportedLimit on such devices.\n - ::cudaLimitDevRuntimePendingLaunchCount is the maximum number of outstanding\n   device runtime launches.\n - ::cudaLimitMaxL2FetchGranularity is the L2 cache fetch granularity.\n - ::cudaLimitPersistingL2CacheSize is the persisting L2 cache size in bytes.\n\n \\param limit  - Limit to query\n \\param pValue - Returned size of the limit\n\n \\return\n ::cudaSuccess,\n ::cudaErrorUnsupportedLimit,\n ::cudaErrorInvalidValue\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa\n ::cudaDeviceSetLimit,\n ::cuCtxGetLimit"]
pub unsafe fn cudaDeviceGetLimit(limit: cudaLimit) -> Result<usize, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaDeviceGetLimit(out_0.as_mut_ptr() as *mut _, limit) };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok(out_0.assume_init()) }
    } else {
        Err(status)
    }
}
pub unsafe fn cudaDeviceGetTexture1DLinearMaxWidth(
    fmtDesc: *const cudaChannelFormatDesc,
    device: ::std::os::raw::c_int,
) -> Result<usize, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudaDeviceGetTexture1DLinearMaxWidth(
            out_0.as_mut_ptr() as *mut _,
            fmtDesc,
            device,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok(out_0.assume_init()) }
    } else {
        Err(status)
    }
}
#[doc = " \\brief Returns the preferred cache configuration for the current device.\n\n On devices where the L1 cache and shared memory use the same hardware\n resources, this returns through \\p pCacheConfig the preferred cache\n configuration for the current device. This is only a preference. The\n runtime will use the requested configuration if possible, but it is free to\n choose a different configuration if required to execute functions.\n\n This will return a \\p pCacheConfig of ::cudaFuncCachePreferNone on devices\n where the size of the L1 cache and shared memory are fixed.\n\n The supported cache configurations are:\n - ::cudaFuncCachePreferNone: no preference for shared memory or L1 (default)\n - ::cudaFuncCachePreferShared: prefer larger shared memory and smaller L1 cache\n - ::cudaFuncCachePreferL1: prefer larger L1 cache and smaller shared memory\n - ::cudaFuncCachePreferEqual: prefer equal size L1 cache and shared memory\n\n \\param pCacheConfig - Returned cache configuration\n\n \\return\n ::cudaSuccess\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaDeviceSetCacheConfig,\n \\ref ::cudaFuncSetCacheConfig(const void*, enum cudaFuncCache) \"cudaFuncSetCacheConfig (C API)\",\n \\ref ::cudaFuncSetCacheConfig(T*, enum cudaFuncCache) \"cudaFuncSetCacheConfig (C++ API)\",\n ::cuCtxGetCacheConfig"]
pub unsafe fn cudaDeviceGetCacheConfig() -> Result<cudaFuncCache, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<cudaFuncCache> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaDeviceGetCacheConfig(out_0.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok(out_0.assume_init()) }
    } else {
        Err(status)
    }
}
#[doc = " \\brief Returns numerical values that correspond to the least and\n greatest stream priorities.\n\n Returns in \\p *leastPriority and \\p *greatestPriority the numerical values that correspond\n to the least and greatest stream priorities respectively. Stream priorities\n follow a convention where lower numbers imply greater priorities. The range of\n meaningful stream priorities is given by [\\p *greatestPriority, \\p *leastPriority].\n If the user attempts to create a stream with a priority value that is\n outside the the meaningful range as specified by this API, the priority is\n automatically clamped down or up to either \\p *leastPriority or \\p *greatestPriority\n respectively. See ::cudaStreamCreateWithPriority for details on creating a\n priority stream.\n A NULL may be passed in for \\p *leastPriority or \\p *greatestPriority if the value\n is not desired.\n\n This function will return '0' in both \\p *leastPriority and \\p *greatestPriority if\n the current context's device does not support stream priorities\n (see ::cudaDeviceGetAttribute).\n\n \\param leastPriority    - Pointer to an int in which the numerical value for least\n                           stream priority is returned\n \\param greatestPriority - Pointer to an int in which the numerical value for greatest\n                           stream priority is returned\n\n \\return\n ::cudaSuccess\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaStreamCreateWithPriority,\n ::cudaStreamGetPriority,\n ::cuCtxGetStreamPriorityRange"]
pub unsafe fn cudaDeviceGetStreamPriorityRange(
) -> Result<(::std::os::raw::c_int, ::std::os::raw::c_int), crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let mut out_1: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudaDeviceGetStreamPriorityRange(
            out_0.as_mut_ptr() as *mut _,
            out_1.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok((out_0.assume_init(), out_1.assume_init())) }
    } else {
        Err(status)
    }
}
#[doc = " \\brief Sets the preferred cache configuration for the current device.\n\n On devices where the L1 cache and shared memory use the same hardware\n resources, this sets through \\p cacheConfig the preferred cache\n configuration for the current device. This is only a preference. The\n runtime will use the requested configuration if possible, but it is free to\n choose a different configuration if required to execute the function. Any\n function preference set via\n \\ref ::cudaFuncSetCacheConfig(const void*, enum cudaFuncCache) \"cudaFuncSetCacheConfig (C API)\"\n or\n \\ref ::cudaFuncSetCacheConfig(T*, enum cudaFuncCache) \"cudaFuncSetCacheConfig (C++ API)\"\n will be preferred over this device-wide setting. Setting the device-wide\n cache configuration to ::cudaFuncCachePreferNone will cause subsequent\n kernel launches to prefer to not change the cache configuration unless\n required to launch the kernel.\n\n This setting does nothing on devices where the size of the L1 cache and\n shared memory are fixed.\n\n Launching a kernel with a different preference than the most recent\n preference setting may insert a device-side synchronization point.\n\n The supported cache configurations are:\n - ::cudaFuncCachePreferNone: no preference for shared memory or L1 (default)\n - ::cudaFuncCachePreferShared: prefer larger shared memory and smaller L1 cache\n - ::cudaFuncCachePreferL1: prefer larger L1 cache and smaller shared memory\n - ::cudaFuncCachePreferEqual: prefer equal size L1 cache and shared memory\n\n \\param cacheConfig - Requested cache configuration\n\n \\return\n ::cudaSuccess\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaDeviceGetCacheConfig,\n \\ref ::cudaFuncSetCacheConfig(const void*, enum cudaFuncCache) \"cudaFuncSetCacheConfig (C API)\",\n \\ref ::cudaFuncSetCacheConfig(T*, enum cudaFuncCache) \"cudaFuncSetCacheConfig (C++ API)\",\n ::cuCtxSetCacheConfig"]
pub unsafe fn cudaDeviceSetCacheConfig(
    cacheConfig: cudaFuncCache,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaDeviceSetCacheConfig(cacheConfig) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Returns a handle to a compute device\n\n Returns in \\p *device a device ordinal given a PCI bus ID string.\n\n \\param device   - Returned device ordinal\n\n \\param pciBusId - String in one of the following forms:\n [domain]:[bus]:[device].[function]\n [domain]:[bus]:[device]\n [bus]:[device].[function]\n where \\p domain, \\p bus, \\p device, and \\p function are all hexadecimal values\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorInvalidDevice\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa\n ::cudaDeviceGetPCIBusId,\n ::cuDeviceGetByPCIBusId"]
pub unsafe fn cudaDeviceGetByPCIBusId(
    pciBusId: *const ::std::os::raw::c_char,
) -> Result<::std::os::raw::c_int, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status =
        unsafe { crate::sys::cudaDeviceGetByPCIBusId(out_0.as_mut_ptr() as *mut _, pciBusId) };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok(out_0.assume_init()) }
    } else {
        Err(status)
    }
}
#[doc = " \\brief Returns a PCI Bus Id string for the device\n\n Returns an ASCII string identifying the device \\p dev in the NULL-terminated\n string pointed to by \\p pciBusId. \\p len specifies the maximum length of the\n string that may be returned.\n\n \\param pciBusId - Returned identifier string for the device in the following format\n [domain]:[bus]:[device].[function]\n where \\p domain, \\p bus, \\p device, and \\p function are all hexadecimal values.\n pciBusId should be large enough to store 13 characters including the NULL-terminator.\n\n \\param len      - Maximum length of string to store in \\p name\n\n \\param device   - Device to get identifier string for\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorInvalidDevice\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa\n ::cudaDeviceGetByPCIBusId,\n ::cuDeviceGetPCIBusId"]
pub unsafe fn cudaDeviceGetPCIBusId(
    len: ::std::os::raw::c_int,
    device: ::std::os::raw::c_int,
) -> Result<::std::os::raw::c_char, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<::std::os::raw::c_char> = std::mem::MaybeUninit::uninit();
    let status =
        unsafe { crate::sys::cudaDeviceGetPCIBusId(out_0.as_mut_ptr() as *mut _, len, device) };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok(out_0.assume_init()) }
    } else {
        Err(status)
    }
}
#[doc = " \\brief Gets an interprocess handle for a previously allocated event\n\n Takes as input a previously allocated event. This event must have been\n created with the ::cudaEventInterprocess and ::cudaEventDisableTiming\n flags set. This opaque handle may be copied into other processes and\n opened with ::cudaIpcOpenEventHandle to allow efficient hardware\n synchronization between GPU work in different processes.\n\n After the event has been been opened in the importing process,\n ::cudaEventRecord, ::cudaEventSynchronize, ::cudaStreamWaitEvent and\n ::cudaEventQuery may be used in either process. Performing operations\n on the imported event after the exported event has been freed\n with ::cudaEventDestroy will result in undefined behavior.\n\n IPC functionality is restricted to devices with support for unified\n addressing on Linux and Windows operating systems.\n IPC functionality on Windows is supported for compatibility purposes\n but not recommended as it comes with performance cost.\n Users can test their device for IPC functionality by calling\n ::cudaDeviceGetAttribute with ::cudaDevAttrIpcEventSupport\n\n \\param handle - Pointer to a user allocated cudaIpcEventHandle\n                    in which to return the opaque event handle\n \\param event   - Event allocated with ::cudaEventInterprocess and\n                    ::cudaEventDisableTiming flags.\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidResourceHandle,\n ::cudaErrorMemoryAllocation,\n ::cudaErrorMapBufferObjectFailed,\n ::cudaErrorNotSupported,\n ::cudaErrorInvalidValue\n \\note_init_rt\n \\note_callback\n\n \\sa\n ::cudaEventCreate,\n ::cudaEventDestroy,\n ::cudaEventSynchronize,\n ::cudaEventQuery,\n ::cudaStreamWaitEvent,\n ::cudaIpcOpenEventHandle,\n ::cudaIpcGetMemHandle,\n ::cudaIpcOpenMemHandle,\n ::cudaIpcCloseMemHandle,\n ::cuIpcGetEventHandle"]
pub unsafe fn cudaIpcGetEventHandle(
    event: cudaEvent_t,
) -> Result<cudaIpcEventHandle_t, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<cudaIpcEventHandle_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaIpcGetEventHandle(out_0.as_mut_ptr() as *mut _, event) };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok(out_0.assume_init()) }
    } else {
        Err(status)
    }
}
#[doc = " \\brief Opens an interprocess event handle for use in the current process\n\n Opens an interprocess event handle exported from another process with\n ::cudaIpcGetEventHandle. This function returns a ::cudaEvent_t that behaves like\n a locally created event with the ::cudaEventDisableTiming flag specified.\n This event must be freed with ::cudaEventDestroy.\n\n Performing operations on the imported event after the exported event has\n been freed with ::cudaEventDestroy will result in undefined behavior.\n\n IPC functionality is restricted to devices with support for unified\n addressing on Linux and Windows operating systems.\n IPC functionality on Windows is supported for compatibility purposes\n but not recommended as it comes with performance cost.\n Users can test their device for IPC functionality by calling\n ::cudaDeviceGetAttribute with ::cudaDevAttrIpcEventSupport\n\n \\param event - Returns the imported event\n \\param handle  - Interprocess handle to open\n\n \\returns\n ::cudaSuccess,\n ::cudaErrorMapBufferObjectFailed,\n ::cudaErrorNotSupported,\n ::cudaErrorInvalidValue,\n ::cudaErrorDeviceUninitialized\n \\note_init_rt\n \\note_callback\n\n \\sa\n ::cudaEventCreate,\n ::cudaEventDestroy,\n ::cudaEventSynchronize,\n ::cudaEventQuery,\n ::cudaStreamWaitEvent,\n ::cudaIpcGetEventHandle,\n ::cudaIpcGetMemHandle,\n ::cudaIpcOpenMemHandle,\n ::cudaIpcCloseMemHandle,\n ::cuIpcOpenEventHandle"]
pub unsafe fn cudaIpcOpenEventHandle<T: ::cuda_libs::types::CudaAsPtr>(
    mut event: T,
    handle: cudaIpcEventHandle_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaIpcOpenEventHandle(event.as_mut_ptr() as *mut cudaEvent_t, handle)
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Gets an interprocess memory handle for an existing device memory\n          allocation\n\n Takes a pointer to the base of an existing device memory allocation created\n with ::cudaMalloc and exports it for use in another process. This is a\n lightweight operation and may be called multiple times on an allocation\n without adverse effects.\n\n If a region of memory is freed with ::cudaFree and a subsequent call\n to ::cudaMalloc returns memory with the same device address,\n ::cudaIpcGetMemHandle will return a unique handle for the\n new memory.\n\n IPC functionality is restricted to devices with support for unified\n addressing on Linux and Windows operating systems.\n IPC functionality on Windows is supported for compatibility purposes\n but not recommended as it comes with performance cost.\n Users can test their device for IPC functionality by calling\n ::cudaDeviceGetAttribute with ::cudaDevAttrIpcEventSupport\n\n \\param handle - Pointer to user allocated ::cudaIpcMemHandle to return\n                    the handle in.\n \\param devPtr - Base pointer to previously allocated device memory\n\n \\returns\n ::cudaSuccess,\n ::cudaErrorMemoryAllocation,\n ::cudaErrorMapBufferObjectFailed,\n ::cudaErrorNotSupported,\n ::cudaErrorInvalidValue\n \\note_init_rt\n \\note_callback\n\n \\sa\n ::cudaMalloc,\n ::cudaFree,\n ::cudaIpcGetEventHandle,\n ::cudaIpcOpenEventHandle,\n ::cudaIpcOpenMemHandle,\n ::cudaIpcCloseMemHandle,\n ::cuIpcGetMemHandle"]
pub unsafe fn cudaIpcGetMemHandle(
    devPtr: *mut ::std::os::raw::c_void,
) -> Result<cudaIpcMemHandle_t, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<cudaIpcMemHandle_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaIpcGetMemHandle(out_0.as_mut_ptr() as *mut _, devPtr) };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok(out_0.assume_init()) }
    } else {
        Err(status)
    }
}
#[doc = " \\brief Opens an interprocess memory handle exported from another process\n          and returns a device pointer usable in the local process.\n\n Maps memory exported from another process with ::cudaIpcGetMemHandle into\n the current device address space. For contexts on different devices\n ::cudaIpcOpenMemHandle can attempt to enable peer access between the\n devices as if the user called ::cudaDeviceEnablePeerAccess. This behavior is\n controlled by the ::cudaIpcMemLazyEnablePeerAccess flag.\n ::cudaDeviceCanAccessPeer can determine if a mapping is possible.\n\n ::cudaIpcOpenMemHandle can open handles to devices that may not be visible\n in the process calling the API.\n\n Contexts that may open ::cudaIpcMemHandles are restricted in the following way.\n ::cudaIpcMemHandles from each device in a given process may only be opened\n by one context per device per other process.\n\n If the memory handle has already been opened by the current context, the\n reference count on the handle is incremented by 1 and the existing device pointer\n is returned.\n\n Memory returned from ::cudaIpcOpenMemHandle must be freed with\n ::cudaIpcCloseMemHandle.\n\n Calling ::cudaFree on an exported memory region before calling\n ::cudaIpcCloseMemHandle in the importing context will result in undefined\n behavior.\n\n IPC functionality is restricted to devices with support for unified\n addressing on Linux and Windows operating systems.\n IPC functionality on Windows is supported for compatibility purposes\n but not recommended as it comes with performance cost.\n Users can test their device for IPC functionality by calling\n ::cudaDeviceGetAttribute with ::cudaDevAttrIpcEventSupport\n\n \\param devPtr - Returned device pointer\n \\param handle - ::cudaIpcMemHandle to open\n \\param flags  - Flags for this operation. Must be specified as ::cudaIpcMemLazyEnablePeerAccess\n\n \\returns\n ::cudaSuccess,\n ::cudaErrorMapBufferObjectFailed,\n ::cudaErrorInvalidResourceHandle,\n ::cudaErrorDeviceUninitialized,\n ::cudaErrorTooManyPeers,\n ::cudaErrorNotSupported,\n ::cudaErrorInvalidValue\n \\note_init_rt\n \\note_callback\n\n \\note No guarantees are made about the address returned in \\p *devPtr.\n In particular, multiple processes may not receive the same address for the same \\p handle.\n\n \\sa\n ::cudaMalloc,\n ::cudaFree,\n ::cudaIpcGetEventHandle,\n ::cudaIpcOpenEventHandle,\n ::cudaIpcGetMemHandle,\n ::cudaIpcCloseMemHandle,\n ::cudaDeviceEnablePeerAccess,\n ::cudaDeviceCanAccessPeer,\n ::cuIpcOpenMemHandle"]
pub unsafe fn cudaIpcOpenMemHandle<T: ::cuda_libs::types::CudaAsPtr>(
    mut devPtr: T,
    handle: cudaIpcMemHandle_t,
    flags: ::std::os::raw::c_uint,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaIpcOpenMemHandle(
            devPtr.as_mut_ptr() as *mut *mut ::std::os::raw::c_void,
            handle,
            flags,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Attempts to close memory mapped with cudaIpcOpenMemHandle\n\n Decrements the reference count of the memory returnd by ::cudaIpcOpenMemHandle by 1.\n When the reference count reaches 0, this API unmaps the memory. The original allocation\n in the exporting process as well as imported mappings in other processes\n will be unaffected.\n\n Any resources used to enable peer access will be freed if this is the\n last mapping using them.\n\n IPC functionality is restricted to devices with support for unified\n addressing on Linux and Windows operating systems.\n IPC functionality on Windows is supported for compatibility purposes\n but not recommended as it comes with performance cost.\n Users can test their device for IPC functionality by calling\n ::cudaDeviceGetAttribute with ::cudaDevAttrIpcEventSupport\n\n \\param devPtr - Device pointer returned by ::cudaIpcOpenMemHandle\n\n \\returns\n ::cudaSuccess,\n ::cudaErrorMapBufferObjectFailed,\n ::cudaErrorNotSupported,\n ::cudaErrorInvalidValue\n \\note_init_rt\n \\note_callback\n\n \\sa\n ::cudaMalloc,\n ::cudaFree,\n ::cudaIpcGetEventHandle,\n ::cudaIpcOpenEventHandle,\n ::cudaIpcGetMemHandle,\n ::cudaIpcOpenMemHandle,\n ::cuIpcCloseMemHandle"]
pub unsafe fn cudaIpcCloseMemHandle<T: ::cuda_libs::types::CudaAsPtr>(
    mut devPtr: T,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaIpcCloseMemHandle(devPtr.as_mut_ptr() as *mut ::std::os::raw::c_void)
    };
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
#[doc = " \\brief Registers a callback function to receive async notifications\n\n Registers \\p callbackFunc to receive async notifications.\n\n The \\p userData parameter is passed to the callback function at async notification time.\n Likewise, \\p callback is also passed to the callback function to distinguish between\n multiple registered callbacks.\n\n The callback function being registered should be designed to return quickly (~10ms).\n Any long running tasks should be queued for execution on an application thread.\n\n Callbacks may not call cudaDeviceRegisterAsyncNotification or cudaDeviceUnregisterAsyncNotification.\n Doing so will result in ::cudaErrorNotPermitted. Async notification callbacks execute\n in an undefined order and may be serialized.\n\n Returns in \\p *callback a handle representing the registered callback instance.\n\n \\param device - The device on which to register the callback\n \\param callbackFunc - The function to register as a callback\n \\param userData - A generic pointer to user data. This is passed into the callback function.\n \\param callback - A handle representing the registered callback instance\n\n \\return\n ::cudaSuccess\n ::cudaErrorNotSupported\n ::cudaErrorInvalidDevice\n ::cudaErrorInvalidValue\n ::cudaErrorNotPermitted\n ::cudaErrorUnknown\n \\notefnerr\n\n \\sa\n ::cudaDeviceUnregisterAsyncNotification"]
pub unsafe fn cudaDeviceRegisterAsyncNotification<T: ::cuda_libs::types::CudaAsPtr>(
    device: ::std::os::raw::c_int,
    callbackFunc: cudaAsyncCallback,
    mut userData: T,
    callback: *mut cudaAsyncCallbackHandle_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaDeviceRegisterAsyncNotification(
            device,
            callbackFunc,
            userData.as_mut_ptr() as *mut ::std::os::raw::c_void,
            callback,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Unregisters an async notification callback\n\n Unregisters \\p callback so that the corresponding callback function will stop receiving\n async notifications.\n\n \\param device - The device from which to remove \\p callback.\n \\param callback - The callback instance to unregister from receiving async notifications.\n\n \\return\n ::cudaSuccess\n ::cudaErrorNotSupported\n ::cudaErrorInvalidDevice\n ::cudaErrorInvalidValue\n ::cudaErrorNotPermitted\n ::cudaErrorUnknown\n \\notefnerr\n\n \\sa\n ::cudaDeviceRegisterAsyncNotification"]
pub unsafe fn cudaDeviceUnregisterAsyncNotification(
    device: ::std::os::raw::c_int,
    callback: cudaAsyncCallbackHandle_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaDeviceUnregisterAsyncNotification(device, callback) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Returns the shared memory configuration for the current device.\n\n \\deprecated\n\n This function will return in \\p pConfig the current size of shared memory banks\n on the current device. On devices with configurable shared memory banks,\n ::cudaDeviceSetSharedMemConfig can be used to change this setting, so that all\n subsequent kernel launches will by default use the new bank size. When\n ::cudaDeviceGetSharedMemConfig is called on devices without configurable shared\n memory, it will return the fixed bank size of the hardware.\n\n The returned bank configurations can be either:\n - ::cudaSharedMemBankSizeFourByte - shared memory bank width is four bytes.\n - ::cudaSharedMemBankSizeEightByte - shared memory bank width is eight bytes.\n\n \\param pConfig - Returned cache configuration\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaDeviceSetCacheConfig,\n ::cudaDeviceGetCacheConfig,\n ::cudaDeviceSetSharedMemConfig,\n ::cudaFuncSetCacheConfig,\n ::cuCtxGetSharedMemConfig"]
pub unsafe fn cudaDeviceGetSharedMemConfig() -> Result<cudaSharedMemConfig, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<cudaSharedMemConfig> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaDeviceGetSharedMemConfig(out_0.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok(out_0.assume_init()) }
    } else {
        Err(status)
    }
}
#[doc = " \\brief Sets the shared memory configuration for the current device.\n\n \\deprecated\n\n On devices with configurable shared memory banks, this function will set\n the shared memory bank size which is used for all subsequent kernel launches.\n Any per-function setting of shared memory set via ::cudaFuncSetSharedMemConfig\n will override the device wide setting.\n\n Changing the shared memory configuration between launches may introduce\n a device side synchronization point.\n\n Changing the shared memory bank size will not increase shared memory usage\n or affect occupancy of kernels, but may have major effects on performance.\n Larger bank sizes will allow for greater potential bandwidth to shared memory,\n but will change what kinds of accesses to shared memory will result in bank\n conflicts.\n\n This function will do nothing on devices with fixed shared memory bank size.\n\n The supported bank configurations are:\n - ::cudaSharedMemBankSizeDefault: set bank width the device default (currently,\n   four bytes)\n - ::cudaSharedMemBankSizeFourByte: set shared memory bank width to be four bytes\n   natively.\n - ::cudaSharedMemBankSizeEightByte: set shared memory bank width to be eight\n   bytes natively.\n\n \\param config - Requested cache configuration\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaDeviceSetCacheConfig,\n ::cudaDeviceGetCacheConfig,\n ::cudaDeviceGetSharedMemConfig,\n ::cudaFuncSetCacheConfig,\n ::cuCtxSetSharedMemConfig"]
pub unsafe fn cudaDeviceSetSharedMemConfig(
    config: cudaSharedMemConfig,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaDeviceSetSharedMemConfig(config) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Returns the last error from a runtime call\n\n Returns the last error that has been produced by any of the runtime calls\n in the same instance of the CUDA Runtime library in the host thread and\n resets it to ::cudaSuccess.\n\n Note: Multiple instances of the CUDA Runtime library can be present in an\n application when using a library that statically links the CUDA Runtime.\n\n \\return\n ::cudaSuccess,\n ::cudaErrorMissingConfiguration,\n ::cudaErrorMemoryAllocation,\n ::cudaErrorInitializationError,\n ::cudaErrorLaunchFailure,\n ::cudaErrorLaunchTimeout,\n ::cudaErrorLaunchOutOfResources,\n ::cudaErrorInvalidDeviceFunction,\n ::cudaErrorInvalidConfiguration,\n ::cudaErrorInvalidDevice,\n ::cudaErrorInvalidValue,\n ::cudaErrorInvalidPitchValue,\n ::cudaErrorInvalidSymbol,\n ::cudaErrorUnmapBufferObjectFailed,\n ::cudaErrorInvalidDevicePointer,\n ::cudaErrorInvalidTexture,\n ::cudaErrorInvalidTextureBinding,\n ::cudaErrorInvalidChannelDescriptor,\n ::cudaErrorInvalidMemcpyDirection,\n ::cudaErrorInvalidFilterSetting,\n ::cudaErrorInvalidNormSetting,\n ::cudaErrorUnknown,\n ::cudaErrorInvalidResourceHandle,\n ::cudaErrorInsufficientDriver,\n ::cudaErrorNoDevice,\n ::cudaErrorSetOnActiveProcess,\n ::cudaErrorStartupFailure,\n ::cudaErrorInvalidPtx,\n ::cudaErrorUnsupportedPtxVersion,\n ::cudaErrorNoKernelImageForDevice,\n ::cudaErrorJitCompilerNotFound,\n ::cudaErrorJitCompilationDisabled\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaPeekAtLastError, ::cudaGetErrorName, ::cudaGetErrorString, ::cudaError"]
pub unsafe fn cudaGetLastError() -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaGetLastError() };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Returns the last error from a runtime call\n\n Returns the last error that has been produced by any of the runtime calls\n in the same instance of the CUDA Runtime library in the host thread. This\n call does not reset the error to ::cudaSuccess like ::cudaGetLastError().\n\n Note: Multiple instances of the CUDA Runtime library can be present in an\n application when using a library that statically links the CUDA Runtime.\n\n \\return\n ::cudaSuccess,\n ::cudaErrorMissingConfiguration,\n ::cudaErrorMemoryAllocation,\n ::cudaErrorInitializationError,\n ::cudaErrorLaunchFailure,\n ::cudaErrorLaunchTimeout,\n ::cudaErrorLaunchOutOfResources,\n ::cudaErrorInvalidDeviceFunction,\n ::cudaErrorInvalidConfiguration,\n ::cudaErrorInvalidDevice,\n ::cudaErrorInvalidValue,\n ::cudaErrorInvalidPitchValue,\n ::cudaErrorInvalidSymbol,\n ::cudaErrorUnmapBufferObjectFailed,\n ::cudaErrorInvalidDevicePointer,\n ::cudaErrorInvalidTexture,\n ::cudaErrorInvalidTextureBinding,\n ::cudaErrorInvalidChannelDescriptor,\n ::cudaErrorInvalidMemcpyDirection,\n ::cudaErrorInvalidFilterSetting,\n ::cudaErrorInvalidNormSetting,\n ::cudaErrorUnknown,\n ::cudaErrorInvalidResourceHandle,\n ::cudaErrorInsufficientDriver,\n ::cudaErrorNoDevice,\n ::cudaErrorSetOnActiveProcess,\n ::cudaErrorStartupFailure,\n ::cudaErrorInvalidPtx,\n ::cudaErrorUnsupportedPtxVersion,\n ::cudaErrorNoKernelImageForDevice,\n ::cudaErrorJitCompilerNotFound,\n ::cudaErrorJitCompilationDisabled\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaGetLastError, ::cudaGetErrorName, ::cudaGetErrorString, ::cudaError"]
pub unsafe fn cudaPeekAtLastError() -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaPeekAtLastError() };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Returns the string representation of an error code enum name\n\n Returns a string containing the name of an error code in the enum.  If the error\n code is not recognized, \"unrecognized error code\" is returned.\n\n \\param error - Error code to convert to string\n\n \\return\n \\p char* pointer to a NULL-terminated string\n\n \\sa ::cudaGetErrorString, ::cudaGetLastError, ::cudaPeekAtLastError, ::cudaError,\n ::cuGetErrorName"]
pub unsafe fn cudaGetErrorName(error: cudaError_t) -> *const ::std::os::raw::c_char {
    unsafe { crate::sys::cudaGetErrorName(error) }
}
#[doc = " \\brief Returns the description string for an error code\n\n Returns the description string for an error code.  If the error\n code is not recognized, \"unrecognized error code\" is returned.\n\n \\param error - Error code to convert to string\n\n \\return\n \\p char* pointer to a NULL-terminated string\n\n \\sa ::cudaGetErrorName, ::cudaGetLastError, ::cudaPeekAtLastError, ::cudaError,\n ::cuGetErrorString"]
pub unsafe fn cudaGetErrorString(error: cudaError_t) -> *const ::std::os::raw::c_char {
    unsafe { crate::sys::cudaGetErrorString(error) }
}
#[doc = " \\brief Returns the number of compute-capable devices\n\n Returns in \\p *count the number of devices with compute capability greater\n or equal to 2.0 that are available for execution.\n\n \\param count - Returns the number of devices with compute capability\n greater or equal to 2.0\n\n \\return\n ::cudaSuccess\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaGetDevice, ::cudaSetDevice, ::cudaGetDeviceProperties,\n ::cudaChooseDevice,\n ::cudaInitDevice,\n ::cuDeviceGetCount"]
pub unsafe fn cudaGetDeviceCount() -> Result<::std::os::raw::c_int, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaGetDeviceCount(out_0.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok(out_0.assume_init()) }
    } else {
        Err(status)
    }
}
#[doc = " \\brief Returns information about the compute-device\n\n Returns in \\p *prop the properties of device \\p dev.\n \\param prop   - Properties for the specified device\n \\param device - Device number to get properties for\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidDevice\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaGetDeviceCount, ::cudaGetDevice, ::cudaSetDevice, ::cudaChooseDevice,\n ::cudaDeviceGetAttribute,\n ::cudaInitDevice,\n ::cuDeviceGetAttribute,\n ::cuDeviceGetName"]
pub unsafe fn cudaGetDeviceProperties(
    device: ::std::os::raw::c_int,
) -> Result<cudaDeviceProp, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<cudaDeviceProp> = std::mem::MaybeUninit::uninit();
    let status =
        unsafe { crate::sys::cudaGetDeviceProperties(out_0.as_mut_ptr() as *mut _, device) };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok(out_0.assume_init()) }
    } else {
        Err(status)
    }
}
#[doc = " \\brief Returns information about the device\n\n Returns in \\p *value the integer value of the attribute \\p attr on device\n \\p device.\n\n \\param value  - Returned device attribute value\n \\param attr   - Device attribute to query\n \\param device - Device number to query\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidDevice,\n ::cudaErrorInvalidValue\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaGetDeviceCount, ::cudaGetDevice, ::cudaSetDevice, ::cudaChooseDevice,\n ::cudaGetDeviceProperties,\n ::cudaInitDevice,\n ::cuDeviceGetAttribute"]
pub unsafe fn cudaDeviceGetAttribute(
    attr: cudaDeviceAttr,
    device: ::std::os::raw::c_int,
) -> Result<::std::os::raw::c_int, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status =
        unsafe { crate::sys::cudaDeviceGetAttribute(out_0.as_mut_ptr() as *mut _, attr, device) };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok(out_0.assume_init()) }
    } else {
        Err(status)
    }
}
#[doc = " \\brief Queries details about atomic operations supported between the device and host.\n\n Returns in \\p *capabilities the details about requested atomic \\p *operations over the\n the link between \\p dev and the host. The allocated size of \\p *operations and\n \\p *capabilities must be \\p count.\n\n For each ::cudaAtomicOperation in \\p *operations, the corresponding result in \\p *capabilities\n will be a bitmask indicating which of ::cudaAtomicOperationCapability the link supports natively.\n\n Returns ::cudaErrorInvalidDevice if \\p dev is not valid.\n\n Returns ::cudaErrorInvalidValue if \\p *capabilities or \\p *operations is NULL, if \\p count is 0,\n or if any of \\p *operations is not valid.\n\n \\param capabilities          - Returned capability details of each requested operation\n \\param operations            - Requested operations\n \\param count                 - Count of requested operations and size of capabilities\n \\param dev                   - Device handle\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidDevice,\n ::cudaErrorInvalidValue\n \\notefnerr\n\n \\sa\n ::cudaDeviceGetAttribute,\n ::cudaDeviceGetP2PAtomicCapabilities,\n ::cuDeviceGeHostAtomicCapabilities"]
pub unsafe fn cudaDeviceGetHostAtomicCapabilities(
    operations: *const cudaAtomicOperation,
    count: ::std::os::raw::c_uint,
    device: ::std::os::raw::c_int,
) -> Result<::std::os::raw::c_uint, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<::std::os::raw::c_uint> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudaDeviceGetHostAtomicCapabilities(
            out_0.as_mut_ptr() as *mut _,
            operations,
            count,
            device,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok(out_0.assume_init()) }
    } else {
        Err(status)
    }
}
#[doc = " \\brief Returns the default mempool of a device\n\n The default mempool of a device contains device memory from that device.\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidDevice,\n ::cudaErrorInvalidValue\n ::cudaErrorNotSupported\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa ::cuDeviceGetDefaultMemPool, ::cudaMallocAsync, ::cudaMemPoolTrimTo, ::cudaMemPoolGetAttribute, ::cudaDeviceSetMemPool, ::cudaMemPoolSetAttribute, ::cudaMemPoolSetAccess"]
pub unsafe fn cudaDeviceGetDefaultMemPool(
    device: ::std::os::raw::c_int,
) -> Result<cudaMemPool_t, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<cudaMemPool_t> = std::mem::MaybeUninit::uninit();
    let status =
        unsafe { crate::sys::cudaDeviceGetDefaultMemPool(out_0.as_mut_ptr() as *mut _, device) };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok(out_0.assume_init()) }
    } else {
        Err(status)
    }
}
#[doc = " \\brief Sets the current memory pool of a device\n\n The memory pool must be local to the specified device.\n Unless a mempool is specified in the ::cudaMallocAsync call,\n ::cudaMallocAsync allocates from the current mempool of the provided stream's device.\n By default, a device's current memory pool is its default memory pool.\n\n \\note Use ::cudaMallocFromPoolAsync to specify asynchronous allocations from a device different\n than the one the stream runs on.\n\n \\returns\n ::cudaSuccess,\n ::cudaErrorInvalidValue\n ::cudaErrorInvalidDevice\n ::cudaErrorNotSupported\n \\notefnerr\n \\note_callback\n\n \\sa ::cuDeviceSetMemPool, ::cudaDeviceGetMemPool, ::cudaDeviceGetDefaultMemPool, ::cudaMemPoolCreate, ::cudaMemPoolDestroy, ::cudaMallocFromPoolAsync"]
pub unsafe fn cudaDeviceSetMemPool(
    device: ::std::os::raw::c_int,
    memPool: cudaMemPool_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaDeviceSetMemPool(device, memPool) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Gets the current mempool for a device\n\n Returns the last pool provided to ::cudaDeviceSetMemPool for this device\n or the device's default memory pool if ::cudaDeviceSetMemPool has never been called.\n By default the current mempool is the default mempool for a device,\n otherwise the returned pool must have been set with ::cuDeviceSetMemPool or ::cudaDeviceSetMemPool.\n\n \\returns\n ::cudaSuccess,\n ::cudaErrorInvalidValue\n ::cudaErrorNotSupported\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa ::cuDeviceGetMemPool, ::cudaDeviceGetDefaultMemPool, ::cudaDeviceSetMemPool"]
pub unsafe fn cudaDeviceGetMemPool(
    device: ::std::os::raw::c_int,
) -> Result<cudaMemPool_t, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<cudaMemPool_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaDeviceGetMemPool(out_0.as_mut_ptr() as *mut _, device) };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok(out_0.assume_init()) }
    } else {
        Err(status)
    }
}
#[doc = " \\brief Return NvSciSync attributes that this device can support.\n\n Returns in \\p nvSciSyncAttrList, the properties of NvSciSync that\n this CUDA device, \\p dev can support. The returned \\p nvSciSyncAttrList\n can be used to create an NvSciSync that matches this device's capabilities.\n\n If NvSciSyncAttrKey_RequiredPerm field in \\p nvSciSyncAttrList is\n already set this API will return ::cudaErrorInvalidValue.\n\n The applications should set \\p nvSciSyncAttrList to a valid\n NvSciSyncAttrList failing which this API will return\n ::cudaErrorInvalidHandle.\n\n The \\p flags controls how applications intends to use\n the NvSciSync created from the \\p nvSciSyncAttrList. The valid flags are:\n - ::cudaNvSciSyncAttrSignal, specifies that the applications intends to\n signal an NvSciSync on this CUDA device.\n - ::cudaNvSciSyncAttrWait, specifies that the applications intends to\n wait on an NvSciSync on this CUDA device.\n\n At least one of these flags must be set, failing which the API\n returns ::cudaErrorInvalidValue. Both the flags are orthogonal\n to one another: a developer may set both these flags that allows to\n set both wait and signal specific attributes in the same \\p nvSciSyncAttrList.\n\n Note that this API updates the input \\p nvSciSyncAttrList with values equivalent\n to the following public attribute key-values:\n NvSciSyncAttrKey_RequiredPerm is set to\n - NvSciSyncAccessPerm_SignalOnly if ::cudaNvSciSyncAttrSignal is set in \\p flags.\n - NvSciSyncAccessPerm_WaitOnly if ::cudaNvSciSyncAttrWait is set in \\p flags.\n - NvSciSyncAccessPerm_WaitSignal if both ::cudaNvSciSyncAttrWait and\n ::cudaNvSciSyncAttrSignal are set in \\p flags.\n NvSciSyncAttrKey_PrimitiveInfo is set to\n - NvSciSyncAttrValPrimitiveType_SysmemSemaphore on any valid \\p device.\n - NvSciSyncAttrValPrimitiveType_Syncpoint if \\p device is a Tegra device.\n - NvSciSyncAttrValPrimitiveType_SysmemSemaphorePayload64b if \\p device is GA10X+.\n NvSciSyncAttrKey_GpuId is set to the same UUID that is returned in\n \\p cudaDeviceProp.uuid from ::cudaDeviceGetProperties for this \\p device.\n\n \\param nvSciSyncAttrList     - Return NvSciSync attributes supported.\n \\param device                - Valid Cuda Device to get NvSciSync attributes for.\n \\param flags                 - flags describing NvSciSync usage.\n\n \\return\n\n ::cudaSuccess,\n ::cudaErrorDeviceUninitialized,\n ::cudaErrorInvalidValue,\n ::cudaErrorInvalidHandle,\n ::cudaErrorInvalidDevice,\n ::cudaErrorNotSupported,\n ::cudaErrorMemoryAllocation\n\n \\sa\n ::cudaImportExternalSemaphore,\n ::cudaDestroyExternalSemaphore,\n ::cudaSignalExternalSemaphoresAsync,\n ::cudaWaitExternalSemaphoresAsync"]
pub unsafe fn cudaDeviceGetNvSciSyncAttributes<T: ::cuda_libs::types::CudaAsPtr>(
    mut nvSciSyncAttrList: T,
    device: ::std::os::raw::c_int,
    flags: ::std::os::raw::c_int,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaDeviceGetNvSciSyncAttributes(
            nvSciSyncAttrList.as_mut_ptr() as *mut ::std::os::raw::c_void,
            device,
            flags,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Queries attributes of the link between two devices.\n\n Returns in \\p *value the value of the requested attribute \\p attrib of the\n link between \\p srcDevice and \\p dstDevice. The supported attributes are:\n - ::cudaDevP2PAttrPerformanceRank: A relative value indicating the\n   performance of the link between two devices. Lower value means better\n   performance (0 being the value used for most performant link).\n - ::cudaDevP2PAttrAccessSupported: 1 if peer access is enabled.\n - ::cudaDevP2PAttrNativeAtomicSupported: 1 if all native atomic operations\n   over the link are supported.\n - ::cudaDevP2PAttrCudaArrayAccessSupported: 1 if accessing CUDA arrays over\n   the link is supported.\n - ::cudaDevP2PAttrOnlyPartialNativeAtomicSupported: 1 if some\n   CUDA-valid atomic operations over the link are supported. Information about\n   specific operations can be retrieved with ::cudaDeviceGetP2PAtomicCapabilities.\n\n Returns ::cudaErrorInvalidDevice if \\p srcDevice or \\p dstDevice are not valid\n or if they represent the same device.\n\n Returns ::cudaErrorInvalidValue if \\p attrib is not valid or if \\p value is\n a null pointer.\n\n \\param value         - Returned value of the requested attribute\n \\param attrib        - The requested attribute of the link between \\p srcDevice and \\p dstDevice.\n \\param srcDevice     - The source device of the target link.\n \\param dstDevice     - The destination device of the target link.\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidDevice,\n ::cudaErrorInvalidValue\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaDeviceEnablePeerAccess,\n ::cudaDeviceDisablePeerAccess,\n ::cudaDeviceCanAccessPeer,\n ::cuDeviceGetP2PAttribute\n ::cudaDeviceGetP2PAtomicCapabilities"]
pub unsafe fn cudaDeviceGetP2PAttribute(
    attr: cudaDeviceP2PAttr,
    srcDevice: ::std::os::raw::c_int,
    dstDevice: ::std::os::raw::c_int,
) -> Result<::std::os::raw::c_int, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudaDeviceGetP2PAttribute(
            out_0.as_mut_ptr() as *mut _,
            attr,
            srcDevice,
            dstDevice,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok(out_0.assume_init()) }
    } else {
        Err(status)
    }
}
#[doc = " \\brief Queries details about atomic operations supported between two devices\n\n Returns in \\p *capabilities the details about requested atomic \\p *operations over the\n the link between \\p srcDevice and \\p dstDevice. The allocated size of \\p *operations and\n \\p *capabilities must be \\p count.\n\n For each ::cudaAtomicOperation in \\p *operations, the corresponding result in \\p *capabilities\n will be a bitmask indicating which of ::cudaAtomicOperationCapability the link supports natively.\n\n Returns ::cudaErrorInvalidDevice if \\p srcDevice or \\p dstDevice are not valid\n or if they represent the same device.\n\n Returns ::cudaErrorInvalidValue if \\p *capabilities or \\p *operations is NULL, if \\p count is 0,\n or if any of \\p *operations is not valid.\n\n \\param capabilities          - Returned capability details of each requested operation\n \\param operations            - Requested operations\n \\param count                 - Count of requested operations and size of capabilities\n \\param srcDevice             - The source device of the target link\n \\param dstDevice             - The destination device of the target link\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidDevice,\n ::cudaErrorInvalidValue\n \\notefnerr\n\n \\sa\n ::cudaDeviceGetP2PAttribute,\n ::cuDeviceGetP2PAttribute,\n ::cuDeviceGetP2PAtomicCapabilities"]
pub unsafe fn cudaDeviceGetP2PAtomicCapabilities(
    operations: *const cudaAtomicOperation,
    count: ::std::os::raw::c_uint,
    srcDevice: ::std::os::raw::c_int,
    dstDevice: ::std::os::raw::c_int,
) -> Result<::std::os::raw::c_uint, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<::std::os::raw::c_uint> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudaDeviceGetP2PAtomicCapabilities(
            out_0.as_mut_ptr() as *mut _,
            operations,
            count,
            srcDevice,
            dstDevice,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok(out_0.assume_init()) }
    } else {
        Err(status)
    }
}
#[doc = " \\brief Select compute-device which best matches criteria\n\n Returns in \\p *device the device which has properties that best match\n \\p *prop.\n\n \\param device - Device with best match\n \\param prop   - Desired device properties\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaGetDeviceCount, ::cudaGetDevice, ::cudaSetDevice,\n ::cudaGetDeviceProperties,\n ::cudaInitDevice"]
pub unsafe fn cudaChooseDevice<T: ::cuda_libs::types::CudaAsPtr>(
    mut device: T,
    prop: *const cudaDeviceProp,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaChooseDevice(device.as_mut_ptr() as *mut ::std::os::raw::c_int, prop)
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Initialize device to be used for GPU executions\n\n This function will initialize the CUDA Runtime structures and primary context on \\p device when called,\n but the context will not be made current to \\p device.\n\n When ::cudaInitDeviceFlagsAreValid is set in \\p flags, deviceFlags are applied to the requested device.\n The values of deviceFlags match those of the flags parameters in ::cudaSetDeviceFlags.\n The effect may be verified by ::cudaGetDeviceFlags.\n\n This function will return an error if the device is in ::cudaComputeModeExclusiveProcess\n and is occupied by another process or if the device is in ::cudaComputeModeProhibited.\n\n \\param device - Device on which the runtime will initialize itself.\n \\param deviceFlags - Parameters for device operation.\n \\param flags - Flags for controlling the device initialization.\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidDevice,\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaGetDeviceCount, ::cudaGetDevice, ::cudaGetDeviceProperties,\n ::cudaChooseDevice, ::cudaSetDevice\n ::cuCtxSetCurrent"]
pub unsafe fn cudaInitDevice(
    device: ::std::os::raw::c_int,
    deviceFlags: ::std::os::raw::c_uint,
    flags: ::std::os::raw::c_uint,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaInitDevice(device, deviceFlags, flags) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Set device to be used for GPU executions\n\n Sets \\p device as the current device for the calling host thread.\n Valid device id's are 0 to (::cudaGetDeviceCount() - 1).\n\n Any device memory subsequently allocated from this host thread\n using ::cudaMalloc(), ::cudaMallocPitch() or ::cudaMallocArray()\n will be physically resident on \\p device.  Any host memory allocated\n from this host thread using ::cudaMallocHost() or ::cudaHostAlloc()\n or ::cudaHostRegister() will have its lifetime associated  with\n \\p device.  Any streams or events created from this host thread will\n be associated with \\p device.  Any kernels launched from this host\n thread using the <<<>>> operator or ::cudaLaunchKernel() will be executed\n on \\p device.\n\n This call may be made from any host thread, to any device, and at\n any time.  This function will do no synchronization with the previous\n or new device,\n and should only take significant time when it initializes the runtime's context state.\n This call will bind the primary context of the specified device to the calling thread and all the\n subsequent memory allocations, stream and event creations, and kernel launches\n will be associated with the primary context.\n This function will also immediately initialize the runtime state on the primary context,\n and the context will be current on \\p device immediately. This function will return an\n error if the device is in ::cudaComputeModeExclusiveProcess and is occupied by another\n process or if the device is in ::cudaComputeModeProhibited.\n\n It is not required to call ::cudaInitDevice before using this function.\n \\param device - Device on which the active host thread should execute the\n device code.\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidDevice,\n ::cudaErrorDeviceUnavailable,\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaGetDeviceCount, ::cudaGetDevice, ::cudaGetDeviceProperties,\n ::cudaChooseDevice,\n ::cudaInitDevice,\n ::cuCtxSetCurrent"]
pub unsafe fn cudaSetDevice(device: ::std::os::raw::c_int) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaSetDevice(device) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Returns which device is currently being used\n\n Returns in \\p *device the current device for the calling host thread.\n\n \\param device - Returns the device on which the active host thread\n executes the device code.\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorDeviceUnavailable,\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaGetDeviceCount, ::cudaSetDevice, ::cudaGetDeviceProperties,\n ::cudaChooseDevice,\n ::cuCtxGetCurrent"]
pub unsafe fn cudaGetDevice() -> Result<::std::os::raw::c_int, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaGetDevice(out_0.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok(out_0.assume_init()) }
    } else {
        Err(status)
    }
}
#[doc = " \\brief Set a list of devices that can be used for CUDA\n\n Sets a list of devices for CUDA execution in priority order using\n \\p device_arr. The parameter \\p len specifies the number of elements in the\n list.  CUDA will try devices from the list sequentially until it finds one\n that works.  If this function is not called, or if it is called with a \\p len\n of 0, then CUDA will go back to its default behavior of trying devices\n sequentially from a default list containing all of the available CUDA\n devices in the system. If a specified device ID in the list does not exist,\n this function will return ::cudaErrorInvalidDevice. If \\p len is not 0 and\n \\p device_arr is NULL or if \\p len exceeds the number of devices in\n the system, then ::cudaErrorInvalidValue is returned.\n\n \\param device_arr - List of devices to try\n \\param len        - Number of devices in specified list\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorInvalidDevice\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaGetDeviceCount, ::cudaSetDevice, ::cudaGetDeviceProperties,\n ::cudaSetDeviceFlags,\n ::cudaChooseDevice"]
pub unsafe fn cudaSetValidDevices<T: ::cuda_libs::types::CudaAsPtr>(
    mut device_arr: T,
    len: ::std::os::raw::c_int,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaSetValidDevices(device_arr.as_mut_ptr() as *mut ::std::os::raw::c_int, len)
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Sets flags to be used for device executions\n\n Records \\p flags as the flags for the current device. If the current device\n has been set and that device has already been initialized, the previous flags\n are overwritten. If the current device has not been initialized, it is\n initialized with the provided flags. If no device has been made current to\n the calling thread, a default device is selected and initialized with the\n provided flags.\n\n The three LSBs of the \\p flags parameter can be used to control how the CPU\n thread interacts with the OS scheduler when waiting for results from the\n device.\n\n - ::cudaDeviceScheduleAuto: The default value if the \\p flags parameter is\n zero, uses a heuristic based on the number of active CUDA contexts in the\n process \\p C and the number of logical processors in the system \\p P. If\n \\p C \\> \\p P, then CUDA will yield to other OS threads when waiting for the\n device, otherwise CUDA will not yield while waiting for results and\n actively spin on the processor. Additionally, on Tegra devices,\n ::cudaDeviceScheduleAuto uses a heuristic based on the power profile of\n the platform and may choose ::cudaDeviceScheduleBlockingSync for low-powered\n devices.\n - ::cudaDeviceScheduleSpin: Instruct CUDA to actively spin when waiting for\n results from the device. This can decrease latency when waiting for the\n device, but may lower the performance of CPU threads if they are performing\n work in parallel with the CUDA thread.\n - ::cudaDeviceScheduleYield: Instruct CUDA to yield its thread when waiting\n for results from the device. This can increase latency when waiting for the\n device, but can increase the performance of CPU threads performing work in\n parallel with the device.\n - ::cudaDeviceScheduleBlockingSync: Instruct CUDA to block the CPU thread\n on a synchronization primitive when waiting for the device to finish work.\n - ::cudaDeviceBlockingSync: Instruct CUDA to block the CPU thread on a\n synchronization primitive when waiting for the device to finish work. <br>\n \\ref deprecated \"Deprecated:\" This flag was deprecated as of CUDA 4.0 and\n replaced with ::cudaDeviceScheduleBlockingSync.\n - ::cudaDeviceMapHost: This flag enables allocating pinned\n host memory that is accessible to the device. It is implicit for the\n runtime but may be absent if a context is created using the driver API.\n If this flag is not set, ::cudaHostGetDevicePointer() will always return\n a failure code.\n - ::cudaDeviceLmemResizeToMax: Instruct CUDA to not reduce local memory\n after resizing local memory for a kernel. This can prevent thrashing by\n local memory allocations when launching many kernels with high local\n memory usage at the cost of potentially increased memory usage. <br>\n \\ref deprecated \"Deprecated:\" This flag is deprecated and the behavior enabled\n by this flag is now the default and cannot be disabled.\n - ::cudaDeviceSyncMemops: Ensures that synchronous memory operations initiated\n on this context will always synchronize. See further documentation in the\n section titled \"API Synchronization behavior\" to learn more about cases when\n synchronous memory operations can exhibit asynchronous behavior.\n\n \\param flags - Parameters for device operation\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaGetDeviceFlags, ::cudaGetDeviceCount, ::cudaGetDevice, ::cudaGetDeviceProperties,\n ::cudaSetDevice, ::cudaSetValidDevices,\n ::cudaInitDevice,\n ::cudaChooseDevice,\n ::cuDevicePrimaryCtxSetFlags"]
pub unsafe fn cudaSetDeviceFlags(
    flags: ::std::os::raw::c_uint,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaSetDeviceFlags(flags) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Gets the flags for the current device\n\n\n Returns in \\p flags the flags for the current device. If there is a current\n device for the calling thread, the flags for the device are returned. If\n there is no current device, the flags for the first device are returned,\n which may be the default flags.  Compare to the behavior of\n ::cudaSetDeviceFlags.\n\n Typically, the flags returned should match the behavior that will be seen\n if the calling thread uses a device after this call, without any change to\n the flags or current device inbetween by this or another thread.  Note that\n if the device is not initialized, it is possible for another thread to\n change the flags for the current device before it is initialized.\n Additionally, when using exclusive mode, if this thread has not requested a\n specific device, it may use a device other than the first device, contrary\n to the assumption made by this function.\n\n If a context has been created via the driver API and is current to the\n calling thread, the flags for that context are always returned.\n\n Flags returned by this function may specifically include ::cudaDeviceMapHost\n even though it is not accepted by ::cudaSetDeviceFlags because it is\n implicit in runtime API flags.  The reason for this is that the current\n context may have been created via the driver API in which case the flag is\n not implicit and may be unset.\n\n \\param flags - Pointer to store the device flags\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidDevice\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaGetDevice, ::cudaGetDeviceProperties,\n ::cudaSetDevice, ::cudaSetDeviceFlags,\n ::cudaInitDevice,\n ::cuCtxGetFlags,\n ::cuDevicePrimaryCtxGetState"]
pub unsafe fn cudaGetDeviceFlags() -> Result<::std::os::raw::c_uint, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<::std::os::raw::c_uint> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaGetDeviceFlags(out_0.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok(out_0.assume_init()) }
    } else {
        Err(status)
    }
}
#[doc = " \\brief Create an asynchronous stream\n\n Creates a new asynchronous stream on the context that is current to the calling host thread.\n If no context is current to the calling host thread, then the primary context for a device\n is selected, made current to the calling thread, and initialized before creating a stream on it.\n\n \\param pStream - Pointer to new stream identifier\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue\n ::cudaErrorExternalDevice\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaStreamCreateWithPriority,\n ::cudaStreamCreateWithFlags,\n ::cudaStreamGetPriority,\n ::cudaStreamGetFlags,\n ::cudaStreamGetDevice,\n ::cudaStreamGetDevResource,\n ::cudaStreamQuery,\n ::cudaStreamSynchronize,\n ::cudaStreamWaitEvent,\n ::cudaStreamAddCallback,\n ::cudaSetDevice,\n ::cudaStreamDestroy,\n ::cuStreamCreate"]
pub unsafe fn cudaStreamCreate(pStream: *mut cudaStream_t) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaStreamCreate(pStream) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Create an asynchronous stream\n\n Creates a new asynchronous stream on the context that is current to the calling host thread.\n If no context is current to the calling host thread, then the primary context for a device\n is selected, made current to the calling thread, and initialized before creating a stream on it.\n The \\p flags argument determines the behaviors of the stream.  Valid values for \\p flags are\n - ::cudaStreamDefault: Default stream creation flag.\n - ::cudaStreamNonBlocking: Specifies that work running in the created\n   stream may run concurrently with work in stream 0 (the NULL stream), and that\n   the created stream should perform no implicit synchronization with stream 0.\n\n \\param pStream - Pointer to new stream identifier\n \\param flags   - Parameters for stream creation\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue\n ::cudaErrorExternalDevice\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaStreamCreate,\n ::cudaStreamCreateWithPriority,\n ::cudaStreamGetFlags,\n ::cudaStreamGetDevice,\n ::cudaStreamGetDevResource,\n ::cudaStreamQuery,\n ::cudaStreamSynchronize,\n ::cudaStreamWaitEvent,\n ::cudaStreamAddCallback,\n ::cudaSetDevice,\n ::cudaStreamDestroy,\n ::cuStreamCreate"]
pub unsafe fn cudaStreamCreateWithFlags(
    pStream: *mut cudaStream_t,
    flags: ::std::os::raw::c_uint,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaStreamCreateWithFlags(pStream, flags) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Create an asynchronous stream with the specified priority\n\n Creates a stream with the specified priority and returns a handle in \\p pStream.\n The stream is created on the context that is current to the calling host thread.\n If no context is current to the calling host thread, then the primary context for a device\n is selected, made current to the calling thread, and initialized before creating a stream on it.\n This affects the scheduling priority of work in the stream. Priorities provide a\n hint to preferentially run work with higher priority when possible, but do\n not preempt already-running work or provide any other functional guarantee on\n execution order.\n\n \\p priority follows a convention where lower numbers represent higher priorities.\n '0' represents default priority. The range of meaningful numerical priorities can\n be queried using ::cudaDeviceGetStreamPriorityRange. If the specified priority is\n outside the numerical range returned by ::cudaDeviceGetStreamPriorityRange,\n it will automatically be clamped to the lowest or the highest number in the range.\n\n \\param pStream  - Pointer to new stream identifier\n \\param flags    - Flags for stream creation. See ::cudaStreamCreateWithFlags for a list of valid flags that can be passed\n \\param priority - Priority of the stream. Lower numbers represent higher priorities.\n                   See ::cudaDeviceGetStreamPriorityRange for more information about\n                   the meaningful stream priorities that can be passed.\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue\n ::cudaErrorExternalDevice\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\note Stream priorities are supported only on GPUs\n with compute capability 3.5 or higher.\n\n \\note In the current implementation, only compute kernels launched in\n priority streams are affected by the stream's priority. Stream priorities have\n no effect on host-to-device and device-to-host memory operations.\n\n \\sa ::cudaStreamCreate,\n ::cudaStreamCreateWithFlags,\n ::cudaDeviceGetStreamPriorityRange,\n ::cudaStreamGetPriority,\n ::cudaStreamQuery,\n ::cudaStreamWaitEvent,\n ::cudaStreamAddCallback,\n ::cudaStreamSynchronize,\n ::cudaSetDevice,\n ::cudaStreamDestroy,\n ::cuStreamCreateWithPriority"]
pub unsafe fn cudaStreamCreateWithPriority(
    pStream: *mut cudaStream_t,
    flags: ::std::os::raw::c_uint,
    priority: ::std::os::raw::c_int,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaStreamCreateWithPriority(pStream, flags, priority) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Query the priority of a stream\n\n Query the priority of a stream. The priority is returned in in \\p priority.\n Note that if the stream was created with a priority outside the meaningful\n numerical range returned by ::cudaDeviceGetStreamPriorityRange,\n this function returns the clamped priority.\n See ::cudaStreamCreateWithPriority for details about priority clamping.\n\n \\param hStream    - Handle to the stream to be queried\n \\param priority   - Pointer to a signed integer in which the stream's priority is returned\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorInvalidResourceHandle\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaStreamCreateWithPriority,\n ::cudaDeviceGetStreamPriorityRange,\n ::cudaStreamGetFlags,\n ::cudaStreamGetDevice,\n ::cudaStreamGetDevResource,\n ::cuStreamGetPriority"]
pub unsafe fn cudaStreamGetPriority(
    hStream: cudaStream_t,
) -> Result<::std::os::raw::c_int, crate::sys::cudaError> {
    let mut out_1: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status =
        unsafe { crate::sys::cudaStreamGetPriority(hStream, out_1.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok(out_1.assume_init()) }
    } else {
        Err(status)
    }
}
#[doc = " \\brief Query the flags of a stream\n\n Query the flags of a stream. The flags are returned in \\p flags.\n See ::cudaStreamCreateWithFlags for a list of valid flags.\n\n \\param hStream - Handle to the stream to be queried\n \\param flags   - Pointer to an unsigned integer in which the stream's flags are returned\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorInvalidResourceHandle\n \\note_null_stream\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaStreamCreateWithPriority,\n ::cudaStreamCreateWithFlags,\n ::cudaStreamGetPriority,\n ::cudaStreamGetDevice,\n ::cuStreamGetFlags"]
pub unsafe fn cudaStreamGetFlags(
    hStream: cudaStream_t,
) -> Result<::std::os::raw::c_uint, crate::sys::cudaError> {
    let mut out_1: std::mem::MaybeUninit<::std::os::raw::c_uint> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaStreamGetFlags(hStream, out_1.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok(out_1.assume_init()) }
    } else {
        Err(status)
    }
}
#[doc = " \\brief Query the Id of a stream\n\n Query the Id of a stream. The Id is returned in \\p streamId.\n The Id is unique for the life of the program.\n\n The stream handle \\p hStream can refer to any of the following:\n <ul>\n   <li>a stream created via any of the CUDA runtime APIs such as ::cudaStreamCreate,\n   ::cudaStreamCreateWithFlags and ::cudaStreamCreateWithPriority, or their driver\n   API equivalents such as ::cuStreamCreate or ::cuStreamCreateWithPriority.\n   Passing an invalid handle will result in undefined behavior.</li>\n   <li>any of the special streams such as the NULL stream, ::cudaStreamLegacy\n   and ::cudaStreamPerThread respectively.  The driver API equivalents of these\n   are also accepted which are NULL, ::CU_STREAM_LEGACY and ::CU_STREAM_PER_THREAD.</li>\n </ul>\n\n \\param hStream    - Handle to the stream to be queried\n \\param streamId   - Pointer to an unsigned long long in which the stream Id is returned\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorInvalidResourceHandle\n \\note_null_stream\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaStreamCreateWithPriority,\n ::cudaStreamCreateWithFlags,\n ::cudaStreamGetPriority,\n ::cudaStreamGetFlags,\n ::cuStreamGetId"]
pub unsafe fn cudaStreamGetId(
    hStream: cudaStream_t,
) -> Result<::std::os::raw::c_ulonglong, crate::sys::cudaError> {
    let mut out_1: std::mem::MaybeUninit<::std::os::raw::c_ulonglong> =
        std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaStreamGetId(hStream, out_1.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok(out_1.assume_init()) }
    } else {
        Err(status)
    }
}
#[doc = " \\brief Query the device of a stream\n\n Returns in \\p *device the device of the stream.\n\n \\param hStream - Handle to the stream to be queried\n \\param device - Returns the device to which the stream belongs\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorDeviceUnavailable,\n \\note_null_stream\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaSetDevice,\n ::cudaGetDevice,\n ::cudaStreamCreate,\n ::cudaStreamGetPriority,\n ::cudaStreamGetFlags,\n ::cuStreamGetId"]
pub unsafe fn cudaStreamGetDevice(
    hStream: cudaStream_t,
) -> Result<::std::os::raw::c_int, crate::sys::cudaError> {
    let mut out_1: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaStreamGetDevice(hStream, out_1.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok(out_1.assume_init()) }
    } else {
        Err(status)
    }
}
#[doc = " \\brief Resets all persisting lines in cache to normal status.\n\n Resets all persisting lines in cache to normal status.\n Takes effect on function return.\n\n \\return\n ::cudaSuccess,\n \\notefnerr\n\n \\sa\n ::cudaAccessPolicyWindow"]
pub unsafe fn cudaCtxResetPersistingL2Cache() -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaCtxResetPersistingL2Cache() };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Copies attributes from source stream to destination stream.\n\n Copies attributes from source stream \\p src to destination stream \\p dst.\n Both streams must have the same context.\n\n \\param[out] dst Destination stream\n \\param[in] src Source stream\n For attributes see ::cudaStreamAttrID\n\n \\return\n ::cudaSuccess,\n ::cudaErrorNotSupported\n \\notefnerr\n\n \\sa\n ::cudaAccessPolicyWindow"]
pub unsafe fn cudaStreamCopyAttributes(
    dst: cudaStream_t,
    src: cudaStream_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaStreamCopyAttributes(dst, src) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Queries stream attribute.\n\n Queries attribute \\p attr from \\p hStream and stores it in corresponding\n member of \\p value_out.\n\n \\param[in] hStream\n \\param[in] attr\n \\param[out] value_out\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorInvalidResourceHandle\n \\notefnerr\n\n \\sa\n ::cudaAccessPolicyWindow"]
pub unsafe fn cudaStreamGetAttribute(
    hStream: cudaStream_t,
    attr: cudaLaunchAttributeID,
) -> Result<cudaLaunchAttributeValue, crate::sys::cudaError> {
    let mut out_2: std::mem::MaybeUninit<cudaLaunchAttributeValue> =
        std::mem::MaybeUninit::uninit();
    let status =
        unsafe { crate::sys::cudaStreamGetAttribute(hStream, attr, out_2.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok(out_2.assume_init()) }
    } else {
        Err(status)
    }
}
#[doc = " \\brief Sets stream attribute.\n\n Sets attribute \\p attr on \\p hStream from corresponding attribute of\n \\p value. The updated attribute will be applied to subsequent work\n submitted to the stream. It will not affect previously submitted work.\n\n \\param[out] hStream\n \\param[in] attr\n \\param[in] value\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorInvalidResourceHandle\n \\notefnerr\n\n \\sa\n ::cudaAccessPolicyWindow"]
pub unsafe fn cudaStreamSetAttribute<T: ::cuda_libs::types::CudaAsPtr>(
    hStream: cudaStream_t,
    attr: cudaLaunchAttributeID,
    value: T,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaStreamSetAttribute(
            hStream,
            attr,
            value.as_const_ptr() as *const cudaLaunchAttributeValue,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Destroys and cleans up an asynchronous stream\n\n Destroys and cleans up the asynchronous stream specified by \\p stream.\n\n In case the device is still doing work in the stream \\p stream\n when ::cudaStreamDestroy() is called, the function will return immediately\n and the resources associated with \\p stream will be released automatically\n once the device has completed all work in \\p stream.\n\n \\param stream - Stream identifier\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorInvalidResourceHandle\n ::cudaErrorExternalDevice\n \\note_null_stream\n \\notefnerr\n \\note_init_rt\n \\note_callback\n \\note_destroy_ub\n\n \\sa ::cudaStreamCreate,\n ::cudaStreamCreateWithFlags,\n ::cudaStreamQuery,\n ::cudaStreamWaitEvent,\n ::cudaStreamSynchronize,\n ::cudaStreamAddCallback,\n ::cuStreamDestroy"]
pub unsafe fn cudaStreamDestroy(stream: cudaStream_t) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaStreamDestroy(stream) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Make a compute stream wait on an event\n\n Makes all future work submitted to \\p stream wait for all work captured in\n \\p event.  See ::cudaEventRecord() for details on what is captured by an event.\n The synchronization will be performed efficiently on the device when applicable.\n \\p event may be from a different device than \\p stream.\n\n flags include:\n - ::cudaEventWaitDefault: Default event creation flag.\n - ::cudaEventWaitExternal: Event is captured in the graph as an external\n   event node when performing stream capture.\n\n \\param stream - Stream to wait\n \\param event  - Event to wait on\n \\param flags  - Parameters for the operation(See above)\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorInvalidResourceHandle\n \\note_null_stream\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaStreamCreate, ::cudaStreamCreateWithFlags, ::cudaStreamQuery, ::cudaStreamSynchronize, ::cudaStreamAddCallback, ::cudaStreamDestroy,\n ::cuStreamWaitEvent"]
pub unsafe fn cudaStreamWaitEvent(
    stream: cudaStream_t,
    event: cudaEvent_t,
    flags: ::std::os::raw::c_uint,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaStreamWaitEvent(stream, event, flags) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Add a callback to a compute stream\n\n \\note This function is slated for eventual deprecation and removal. If\n you do not require the callback to execute in case of a device error,\n consider using ::cudaLaunchHostFunc. Additionally, this function is not\n supported with ::cudaStreamBeginCapture and ::cudaStreamEndCapture, unlike\n ::cudaLaunchHostFunc.\n\n Adds a callback to be called on the host after all currently enqueued\n items in the stream have completed.  For each\n cudaStreamAddCallback call, a callback will be executed exactly once.\n The callback will block later work in the stream until it is finished.\n\n The callback may be passed ::cudaSuccess or an error code.  In the event\n of a device error, all subsequently executed callbacks will receive an\n appropriate ::cudaError_t.\n\n Callbacks must not make any CUDA API calls.  Attempting to use CUDA APIs\n may result in ::cudaErrorNotPermitted.  Callbacks must not perform any\n synchronization that may depend on outstanding device work or other callbacks\n that are not mandated to run earlier.  Callbacks without a mandated order\n (in independent streams) execute in undefined order and may be serialized.\n\n For the purposes of Unified Memory, callback execution makes a number of\n guarantees:\n <ul>\n   <li>The callback stream is considered idle for the duration of the\n   callback.  Thus, for example, a callback may always use memory attached\n   to the callback stream.</li>\n   <li>The start of execution of a callback has the same effect as\n   synchronizing an event recorded in the same stream immediately prior to\n   the callback.  It thus synchronizes streams which have been \"joined\"\n   prior to the callback.</li>\n   <li>Adding device work to any stream does not have the effect of making\n   the stream active until all preceding callbacks have executed.  Thus, for\n   example, a callback might use global attached memory even if work has\n   been added to another stream, if it has been properly ordered with an\n   event.</li>\n   <li>Completion of a callback does not cause a stream to become\n   active except as described above.  The callback stream will remain idle\n   if no device work follows the callback, and will remain idle across\n   consecutive callbacks without device work in between.  Thus, for example,\n   stream synchronization can be done by signaling from a callback at the\n   end of the stream.</li>\n </ul>\n\n \\param stream   - Stream to add callback to\n \\param callback - The function to call once preceding stream operations are complete\n \\param userData - User specified data to be passed to the callback function\n \\param flags    - Reserved for future use, must be 0\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidResourceHandle,\n ::cudaErrorInvalidValue,\n ::cudaErrorNotSupported\n \\note_null_stream\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaStreamCreate, ::cudaStreamCreateWithFlags, ::cudaStreamQuery, ::cudaStreamSynchronize, ::cudaStreamWaitEvent, ::cudaStreamDestroy, ::cudaMallocManaged, ::cudaStreamAttachMemAsync,\n ::cudaLaunchHostFunc, ::cuStreamAddCallback"]
pub unsafe fn cudaStreamAddCallback<T: ::cuda_libs::types::CudaAsPtr>(
    stream: cudaStream_t,
    callback: cudaStreamCallback_t,
    mut userData: T,
    flags: ::std::os::raw::c_uint,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaStreamAddCallback(
            stream,
            callback,
            userData.as_mut_ptr() as *mut ::std::os::raw::c_void,
            flags,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Waits for stream tasks to complete\n\n Blocks until \\p stream has completed all operations. If the\n ::cudaDeviceScheduleBlockingSync flag was set for this device,\n the host thread will block until the stream is finished with\n all of its tasks.\n\n \\param stream - Stream identifier\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidResourceHandle\n \\note_null_stream\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaStreamCreate, ::cudaStreamCreateWithFlags, ::cudaStreamQuery, ::cudaStreamWaitEvent, ::cudaStreamAddCallback, ::cudaStreamDestroy,\n ::cuStreamSynchronize"]
pub unsafe fn cudaStreamSynchronize(stream: cudaStream_t) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaStreamSynchronize(stream) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Queries an asynchronous stream for completion status\n\n Returns ::cudaSuccess if all operations in \\p stream have\n completed, or ::cudaErrorNotReady if not.\n\n For the purposes of Unified Memory, a return value of ::cudaSuccess\n is equivalent to having called ::cudaStreamSynchronize().\n\n \\param stream - Stream identifier\n\n \\return\n ::cudaSuccess,\n ::cudaErrorNotReady,\n ::cudaErrorInvalidResourceHandle\n \\note_null_stream\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaStreamCreate, ::cudaStreamCreateWithFlags, ::cudaStreamWaitEvent, ::cudaStreamSynchronize, ::cudaStreamAddCallback, ::cudaStreamDestroy,\n ::cuStreamQuery"]
pub unsafe fn cudaStreamQuery(stream: cudaStream_t) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaStreamQuery(stream) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudaStreamAttachMemAsync<T: ::cuda_libs::types::CudaAsPtr>(
    stream: cudaStream_t,
    mut devPtr: T,
    length: usize,
    flags: ::std::os::raw::c_uint,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaStreamAttachMemAsync(
            stream,
            devPtr.as_mut_ptr() as *mut ::std::os::raw::c_void,
            length,
            flags,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Begins graph capture on a stream\n\n Begin graph capture on \\p stream. When a stream is in capture mode, all operations\n pushed into the stream will not be executed, but will instead be captured into\n a graph, which will be returned via ::cudaStreamEndCapture. Capture may not be initiated\n if \\p stream is ::cudaStreamLegacy. Capture must be ended on the same stream in which\n it was initiated, and it may only be initiated if the stream is not already in capture\n mode. The capture mode may be queried via ::cudaStreamIsCapturing. A unique id\n representing the capture sequence may be queried via ::cudaStreamGetCaptureInfo.\n\n If \\p mode is not ::cudaStreamCaptureModeRelaxed, ::cudaStreamEndCapture must be\n called on this stream from the same thread.\n\n \\note Kernels captured using this API must not use texture and surface references.\n       Reading or writing through any texture or surface reference is undefined\n       behavior. This restriction does not apply to texture and surface objects.\n\n \\param stream - Stream in which to initiate capture\n \\param mode    - Controls the interaction of this capture sequence with other API\n                  calls that are potentially unsafe. For more details see\n                  ::cudaThreadExchangeStreamCaptureMode.\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue\n \\notefnerr\n\n \\sa\n ::cudaStreamCreate,\n ::cudaStreamIsCapturing,\n ::cudaStreamEndCapture,\n ::cudaThreadExchangeStreamCaptureMode"]
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
#[doc = " \\brief Begins graph capture on a stream to an existing graph\n\n Begin graph capture on \\p stream. When a stream is in capture mode, all operations\n pushed into the stream will not be executed, but will instead be captured into\n \\p graph, which will be returned via ::cudaStreamEndCapture.\n\n Capture may not be initiated if \\p stream is ::cudaStreamLegacy. Capture must be ended on the\n same stream in which it was initiated, and it may only be initiated if the stream is not\n already in capture mode. The capture mode may be queried via ::cudaStreamIsCapturing. A unique id\n representing the capture sequence may be queried via ::cudaStreamGetCaptureInfo.\n\n If \\p mode is not ::cudaStreamCaptureModeRelaxed, ::cudaStreamEndCapture must be\n called on this stream from the same thread.\n\n \\note Kernels captured using this API must not use texture and surface references.\n       Reading or writing through any texture or surface reference is undefined\n       behavior. This restriction does not apply to texture and surface objects.\n\n \\param stream          - Stream in which to initiate capture.\n \\param graph           - Graph to capture into.\n \\param dependencies    - Dependencies of the first node captured in the stream.  Can be NULL if numDependencies is 0.\n \\param dependencyData  - Optional array of data associated with each dependency.\n \\param numDependencies - Number of dependencies.\n \\param mode            - Controls the interaction of this capture sequence with other API\n                          calls that are potentially unsafe. For more details see\n                          ::cudaThreadExchangeStreamCaptureMode.\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue\n \\notefnerr\n\n \\sa\n ::cudaStreamCreate,\n ::cudaStreamIsCapturing,\n ::cudaStreamEndCapture,\n ::cudaThreadExchangeStreamCaptureMode"]
pub unsafe fn cudaStreamBeginCaptureToGraph<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
>(
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
            dependencies.as_const_ptr() as *const cudaGraphNode_t,
            dependencyData.as_const_ptr() as *const cudaGraphEdgeData,
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
#[doc = " \\brief Swaps the stream capture interaction mode for a thread\n\n Sets the calling thread's stream capture interaction mode to the value contained\n in \\p *mode, and overwrites \\p *mode with the previous mode for the thread. To\n facilitate deterministic behavior across function or module boundaries, callers\n are encouraged to use this API in a push-pop fashion: \\code\ncudaStreamCaptureMode mode = desiredMode;\ncudaThreadExchangeStreamCaptureMode(&mode);\n...\ncudaThreadExchangeStreamCaptureMode(&mode); // restore previous mode\n \\endcode\n\n During stream capture (see ::cudaStreamBeginCapture), some actions, such as a call\n to ::cudaMalloc, may be unsafe. In the case of ::cudaMalloc, the operation is\n not enqueued asynchronously to a stream, and is not observed by stream capture.\n Therefore, if the sequence of operations captured via ::cudaStreamBeginCapture\n depended on the allocation being replayed whenever the graph is launched, the\n captured graph would be invalid.\n\n Therefore, stream capture places restrictions on API calls that can be made within\n or concurrently to a ::cudaStreamBeginCapture-::cudaStreamEndCapture sequence. This\n behavior can be controlled via this API and flags to ::cudaStreamBeginCapture.\n\n A thread's mode is one of the following:\n - \\p cudaStreamCaptureModeGlobal: This is the default mode. If the local thread has\n   an ongoing capture sequence that was not initiated with\n   \\p cudaStreamCaptureModeRelaxed at \\p cuStreamBeginCapture, or if any other thread\n   has a concurrent capture sequence initiated with \\p cudaStreamCaptureModeGlobal,\n   this thread is prohibited from potentially unsafe API calls.\n - \\p cudaStreamCaptureModeThreadLocal: If the local thread has an ongoing capture\n   sequence not initiated with \\p cudaStreamCaptureModeRelaxed, it is prohibited\n   from potentially unsafe API calls. Concurrent capture sequences in other threads\n   are ignored.\n - \\p cudaStreamCaptureModeRelaxed: The local thread is not prohibited from potentially\n   unsafe API calls. Note that the thread is still prohibited from API calls which\n   necessarily conflict with stream capture, for example, attempting ::cudaEventQuery\n   on an event that was last recorded inside a capture sequence.\n\n \\param mode - Pointer to mode value to swap with the current mode\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue\n \\notefnerr\n\n \\sa\n ::cudaStreamBeginCapture"]
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
#[doc = " \\brief Ends capture on a stream, returning the captured graph\n\n End capture on \\p stream, returning the captured graph via \\p pGraph.\n Capture must have been initiated on \\p stream via a call to ::cudaStreamBeginCapture.\n If capture was invalidated, due to a violation of the rules of stream capture, then\n a NULL graph will be returned.\n\n If the \\p mode argument to ::cudaStreamBeginCapture was not\n ::cudaStreamCaptureModeRelaxed, this call must be from the same thread as\n ::cudaStreamBeginCapture.\n\n \\param stream - Stream to query\n \\param pGraph - The captured graph\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorStreamCaptureWrongThread\n \\notefnerr\n\n \\sa\n ::cudaStreamCreate,\n ::cudaStreamBeginCapture,\n ::cudaStreamIsCapturing,\n ::cudaGraphDestroy"]
pub unsafe fn cudaStreamEndCapture<T: ::cuda_libs::types::CudaAsPtr>(
    stream: cudaStream_t,
    mut pGraph: T,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaStreamEndCapture(stream, pGraph.as_mut_ptr() as *mut cudaGraph_t)
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Returns a stream's capture status\n\n Return the capture status of \\p stream via \\p pCaptureStatus. After a successful\n call, \\p *pCaptureStatus will contain one of the following:\n - ::cudaStreamCaptureStatusNone: The stream is not capturing.\n - ::cudaStreamCaptureStatusActive: The stream is capturing.\n - ::cudaStreamCaptureStatusInvalidated: The stream was capturing but an error\n   has invalidated the capture sequence. The capture sequence must be terminated\n   with ::cudaStreamEndCapture on the stream where it was initiated in order to\n   continue using \\p stream.\n\n Note that, if this is called on ::cudaStreamLegacy (the \"null stream\") while\n a blocking stream on the same device is capturing, it will return\n ::cudaErrorStreamCaptureImplicit and \\p *pCaptureStatus is unspecified\n after the call. The blocking stream capture is not invalidated.\n\n When a blocking stream is capturing, the legacy stream is in an\n unusable state until the blocking stream capture is terminated. The legacy\n stream is not supported for stream capture, but attempted use would have an\n implicit dependency on the capturing stream(s).\n\n \\param stream         - Stream to query\n \\param pCaptureStatus - Returns the stream's capture status\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorStreamCaptureImplicit\n \\notefnerr\n\n \\sa\n ::cudaStreamCreate,\n ::cudaStreamBeginCapture,\n ::cudaStreamEndCapture"]
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
#[doc = " \\brief Query a stream's capture state\n\n Query stream state related to stream capture.\n\n If called on ::cudaStreamLegacy (the \"null stream\") while a stream not created\n with ::cudaStreamNonBlocking is capturing, returns ::cudaErrorStreamCaptureImplicit.\n\n Valid data (other than capture status) is returned only if both of the following are true:\n - the call returns cudaSuccess\n - the returned capture status is ::cudaStreamCaptureStatusActive\n\n If \\p edgeData_out is non-NULL then \\p dependencies_out must be as well. If\n \\p dependencies_out is non-NULL and \\p edgeData_out is NULL, but there is non-zero edge\n data for one or more of the current stream dependencies, the call will return\n ::cudaErrorLossyQuery.\n\n \\param stream - The stream to query\n \\param captureStatus_out - Location to return the capture status of the stream; required\n \\param id_out - Optional location to return an id for the capture sequence, which is\n           unique over the lifetime of the process\n \\param graph_out - Optional location to return the graph being captured into. All\n           operations other than destroy and node removal are permitted on the graph\n           while the capture sequence is in progress. This API does not transfer\n           ownership of the graph, which is transferred or destroyed at\n           ::cudaStreamEndCapture. Note that the graph handle may be invalidated before\n           end of capture for certain errors. Nodes that are or become\n           unreachable from the original stream at ::cudaStreamEndCapture due to direct\n           actions on the graph do not trigger ::cudaErrorStreamCaptureUnjoined.\n \\param dependencies_out - Optional location to store a pointer to an array of nodes.\n           The next node to be captured in the stream will depend on this set of nodes,\n           absent operations such as event wait which modify this set. The array pointer\n           is valid until the next API call which operates on the stream or until the\n           capture is terminated. The node handles may be copied out and are valid until\n           they or the graph is destroyed. The driver-owned array may also be passed\n           directly to APIs that operate on the graph (not the stream) without copying.\n \\param edgeData_out - Optional location to store a pointer to an array of graph edge\n           data. This array parallels \\c dependencies_out; the next node to be added\n           has an edge to \\c dependencies_out[i] with annotation \\c edgeData_out[i] for\n           each \\c i. The array pointer is valid until the next API call which operates\n           on the stream or until the capture is terminated.\n \\param numDependencies_out - Optional location to store the size of the array\n           returned in dependencies_out.\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorStreamCaptureImplicit,\n ::cudaErrorLossyQuery\n \\note_graph_thread_safety\n \\notefnerr\n\n \\sa\n ::cudaStreamBeginCapture,\n ::cudaStreamIsCapturing,\n ::cudaStreamUpdateCaptureDependencies"]
pub unsafe fn cudaStreamGetCaptureInfo(
    stream: cudaStream_t,
) -> Result<
    (
        cudaStreamCaptureStatus,
        ::std::os::raw::c_ulonglong,
        cudaGraph_t,
        *const cudaGraphNode_t,
        *const cudaGraphEdgeData,
        usize,
    ),
    crate::sys::cudaError,
> {
    let mut out_1: std::mem::MaybeUninit<cudaStreamCaptureStatus> = std::mem::MaybeUninit::uninit();
    let mut out_2: std::mem::MaybeUninit<::std::os::raw::c_ulonglong> =
        std::mem::MaybeUninit::uninit();
    let mut out_3: std::mem::MaybeUninit<cudaGraph_t> = std::mem::MaybeUninit::uninit();
    let mut out_4: std::mem::MaybeUninit<*const cudaGraphNode_t> = std::mem::MaybeUninit::uninit();
    let mut out_5: std::mem::MaybeUninit<*const cudaGraphEdgeData> =
        std::mem::MaybeUninit::uninit();
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
    if status == crate::sys::cudaError::cudaSuccess {
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
#[doc = " \\brief Update the set of dependencies in a capturing stream\n\n Modifies the dependency set of a capturing stream. The dependency set is the set\n of nodes that the next captured node in the stream will depend on.\n\n Valid flags are ::cudaStreamAddCaptureDependencies and\n ::cudaStreamSetCaptureDependencies. These control whether the set passed to\n the API is added to the existing set or replaces it. A flags value of 0 defaults\n to ::cudaStreamAddCaptureDependencies.\n\n Nodes that are removed from the dependency set via this API do not result in\n ::cudaErrorStreamCaptureUnjoined if they are unreachable from the stream at\n ::cudaStreamEndCapture.\n\n Returns ::cudaErrorIllegalState if the stream is not capturing.\n\n \\param stream - The stream to update\n \\param dependencies - The set of dependencies to add\n \\param dependencyData - Optional array of data associated with each dependency.\n \\param numDependencies - The size of the dependencies array\n \\param flags - See above\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorIllegalState\n \\notefnerr\n\n \\sa\n ::cudaStreamBeginCapture,\n ::cudaStreamGetCaptureInfo,"]
pub unsafe fn cudaStreamUpdateCaptureDependencies<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
>(
    stream: cudaStream_t,
    mut dependencies: T,
    dependencyData: U,
    numDependencies: usize,
    flags: ::std::os::raw::c_uint,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaStreamUpdateCaptureDependencies(
            stream,
            dependencies.as_mut_ptr() as *mut cudaGraphNode_t,
            dependencyData.as_const_ptr() as *const cudaGraphEdgeData,
            numDependencies,
            flags,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Creates an event object\n\n Creates an event object for the current device using ::cudaEventDefault.\n\n \\param event - Newly created event\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorLaunchFailure,\n ::cudaErrorMemoryAllocation\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa \\ref ::cudaEventCreate(cudaEvent_t*, unsigned int) \"cudaEventCreate (C++ API)\",\n ::cudaEventCreateWithFlags, ::cudaEventRecord, ::cudaEventQuery,\n ::cudaEventSynchronize, ::cudaEventDestroy, ::cudaEventElapsedTime,\n ::cudaStreamWaitEvent,\n ::cuEventCreate"]
pub unsafe fn cudaEventCreate<T: ::cuda_libs::types::CudaAsPtr>(
    mut event: T,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaEventCreate(event.as_mut_ptr() as *mut cudaEvent_t) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Creates an event object with the specified flags\n\n Creates an event object for the current device with the specified flags. Valid\n flags include:\n - ::cudaEventDefault: Default event creation flag.\n - ::cudaEventBlockingSync: Specifies that event should use blocking\n   synchronization. A host thread that uses ::cudaEventSynchronize() to wait\n   on an event created with this flag will block until the event actually\n   completes.\n - ::cudaEventDisableTiming: Specifies that the created event does not need\n   to record timing data.  Events created with this flag specified and\n   the ::cudaEventBlockingSync flag not specified will provide the best\n   performance when used with ::cudaStreamWaitEvent() and ::cudaEventQuery().\n - ::cudaEventInterprocess: Specifies that the created event may be used as an\n   interprocess event by ::cudaIpcGetEventHandle(). ::cudaEventInterprocess must\n   be specified along with ::cudaEventDisableTiming.\n\n \\param event - Newly created event\n \\param flags - Flags for new event\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorLaunchFailure,\n ::cudaErrorMemoryAllocation\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa \\ref ::cudaEventCreate(cudaEvent_t*) \"cudaEventCreate (C API)\",\n ::cudaEventSynchronize, ::cudaEventDestroy, ::cudaEventElapsedTime,\n ::cudaStreamWaitEvent,\n ::cuEventCreate"]
pub unsafe fn cudaEventCreateWithFlags<T: ::cuda_libs::types::CudaAsPtr>(
    mut event: T,
    flags: ::std::os::raw::c_uint,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaEventCreateWithFlags(event.as_mut_ptr() as *mut cudaEvent_t, flags)
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Records an event\n\n Captures in \\p event the contents of \\p stream at the time of this call.\n \\p event and \\p stream must be on the same CUDA context.\n Calls such as ::cudaEventQuery() or ::cudaStreamWaitEvent() will then\n examine or wait for completion of the work that was captured. Uses of\n \\p stream after this call do not modify \\p event. See note on default\n stream behavior for what is captured in the default case.\n\n ::cudaEventRecord() can be called multiple times on the same event and\n will overwrite the previously captured state. Other APIs such as\n ::cudaStreamWaitEvent() use the most recently captured state at the time\n of the API call, and are not affected by later calls to\n ::cudaEventRecord(). Before the first call to ::cudaEventRecord(), an\n event represents an empty set of work, so for example ::cudaEventQuery()\n would return ::cudaSuccess.\n\n \\param event  - Event to record\n \\param stream - Stream in which to record event\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorInvalidResourceHandle,\n ::cudaErrorLaunchFailure\n \\note_null_stream\n \\notefnerr\n \\note_init_rt\n \\note_callback\n \\note_null_event\n\n \\sa \\ref ::cudaEventCreate(cudaEvent_t*) \"cudaEventCreate (C API)\",\n ::cudaEventCreateWithFlags, ::cudaEventQuery,\n ::cudaEventSynchronize, ::cudaEventDestroy, ::cudaEventElapsedTime,\n ::cudaStreamWaitEvent,\n ::cudaEventRecordWithFlags,\n ::cuEventRecord"]
pub unsafe fn cudaEventRecord(
    event: cudaEvent_t,
    stream: cudaStream_t,
) -> Result<(), crate::sys::cudaError> {
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
    flags: ::std::os::raw::c_uint,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaEventRecordWithFlags(event, stream, flags) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Queries an event's status\n\n Queries the status of all work currently captured by \\p event. See\n ::cudaEventRecord() for details on what is captured by an event.\n\n Returns ::cudaSuccess if all captured work has been completed, or\n ::cudaErrorNotReady if any captured work is incomplete.\n\n For the purposes of Unified Memory, a return value of ::cudaSuccess\n is equivalent to having called ::cudaEventSynchronize().\n\n \\param event - Event to query\n\n \\return\n ::cudaSuccess,\n ::cudaErrorNotReady,\n ::cudaErrorInvalidValue,\n ::cudaErrorInvalidResourceHandle,\n ::cudaErrorLaunchFailure\n \\notefnerr\n \\note_init_rt\n \\note_callback\n \\note_null_event\n\n \\sa \\ref ::cudaEventCreate(cudaEvent_t*) \"cudaEventCreate (C API)\",\n ::cudaEventCreateWithFlags, ::cudaEventRecord,\n ::cudaEventSynchronize, ::cudaEventDestroy, ::cudaEventElapsedTime,\n ::cuEventQuery"]
pub unsafe fn cudaEventQuery(event: cudaEvent_t) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaEventQuery(event) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Waits for an event to complete\n\n Waits until the completion of all work currently captured in \\p event.\n See ::cudaEventRecord() for details on what is captured by an event.\n\n Waiting for an event that was created with the ::cudaEventBlockingSync\n flag will cause the calling CPU thread to block until the event has\n been completed by the device.  If the ::cudaEventBlockingSync flag has\n not been set, then the CPU thread will busy-wait until the event has\n been completed by the device.\n\n \\param event - Event to wait for\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorInvalidResourceHandle,\n ::cudaErrorLaunchFailure\n \\notefnerr\n \\note_init_rt\n \\note_callback\n \\note_null_event\n\n \\sa \\ref ::cudaEventCreate(cudaEvent_t*) \"cudaEventCreate (C API)\",\n ::cudaEventCreateWithFlags, ::cudaEventRecord,\n ::cudaEventQuery, ::cudaEventDestroy, ::cudaEventElapsedTime,\n ::cuEventSynchronize"]
pub unsafe fn cudaEventSynchronize(event: cudaEvent_t) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaEventSynchronize(event) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Destroys an event object\n\n Destroys the event specified by \\p event.\n\n An event may be destroyed before it is complete (i.e., while\n ::cudaEventQuery() would return ::cudaErrorNotReady). In this case, the\n call does not block on completion of the event, and any associated\n resources will automatically be released asynchronously at completion.\n\n \\param event - Event to destroy\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorInvalidResourceHandle,\n ::cudaErrorLaunchFailure\n \\notefnerr\n \\note_init_rt\n \\note_callback\n \\note_destroy_ub\n \\note_null_event\n\n \\sa \\ref ::cudaEventCreate(cudaEvent_t*) \"cudaEventCreate (C API)\",\n ::cudaEventCreateWithFlags, ::cudaEventQuery,\n ::cudaEventSynchronize, ::cudaEventRecord, ::cudaEventElapsedTime,\n ::cuEventDestroy"]
pub unsafe fn cudaEventDestroy(event: cudaEvent_t) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaEventDestroy(event) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Computes the elapsed time between events\n\n Computes the elapsed time between two events (in milliseconds with a\n resolution of around 0.5 microseconds). Note this API is not guaranteed\n to return the latest errors for pending work. As such this API is intended to\n serve as a elapsed time calculation only and polling for completion on the\n events to be compared should be done with ::cudaEventQuery instead.\n\n If either event was last recorded in a non-NULL stream, the resulting time\n may be greater than expected (even if both used the same stream handle). This\n happens because the ::cudaEventRecord() operation takes place asynchronously\n and there is no guarantee that the measured latency is actually just between\n the two events. Any number of other different stream operations could execute\n in between the two measured events, thus altering the timing in a significant\n way.\n\n If ::cudaEventRecord() has not been called on either event, then\n ::cudaErrorInvalidResourceHandle is returned. If ::cudaEventRecord() has been\n called on both events but one or both of them has not yet been completed\n (that is, ::cudaEventQuery() would return ::cudaErrorNotReady on at least one\n of the events), ::cudaErrorNotReady is returned. If either event was created\n with the ::cudaEventDisableTiming flag, then this function will return\n ::cudaErrorInvalidResourceHandle.\n\n \\param ms    - Time between \\p start and \\p end in ms\n \\param start - Starting event\n \\param end   - Ending event\n\n \\return\n ::cudaSuccess,\n ::cudaErrorNotReady,\n ::cudaErrorInvalidValue,\n ::cudaErrorInvalidResourceHandle,\n ::cudaErrorLaunchFailure,\n ::cudaErrorUnknown\n \\notefnerr\n \\note_init_rt\n \\note_callback\n \\note_null_event\n\n \\sa \\ref ::cudaEventCreate(cudaEvent_t*) \"cudaEventCreate (C API)\",\n ::cudaEventCreateWithFlags, ::cudaEventQuery,\n ::cudaEventSynchronize, ::cudaEventDestroy, ::cudaEventRecord,\n ::cuEventElapsedTime"]
pub unsafe fn cudaEventElapsedTime<T: ::cuda_libs::types::CudaAsPtr>(
    mut ms: T,
    start: cudaEvent_t,
    end: cudaEvent_t,
) -> Result<(), crate::sys::cudaError> {
    let status =
        unsafe { crate::sys::cudaEventElapsedTime(ms.as_mut_ptr() as *mut f32, start, end) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Imports an external memory object\n\n Imports an externally allocated memory object and returns\n a handle to that in \\p extMem_out.\n\n The properties of the handle being imported must be described in\n \\p memHandleDesc. The ::cudaExternalMemoryHandleDesc structure\n is defined as follows:\n\n \\code\ntypedef struct cudaExternalMemoryHandleDesc_st {\ncudaExternalMemoryHandleType type;\nunion {\nint fd;\nstruct {\nvoid *handle;\nconst void *name;\n} win32;\nconst void *nvSciBufObject;\n} handle;\nunsigned long long size;\nunsigned int flags;\n} cudaExternalMemoryHandleDesc;\n \\endcode\n\n where ::cudaExternalMemoryHandleDesc::type specifies the type\n of handle being imported. ::cudaExternalMemoryHandleType is\n defined as:\n\n \\code\ntypedef enum cudaExternalMemoryHandleType_enum {\ncudaExternalMemoryHandleTypeOpaqueFd         = 1,\ncudaExternalMemoryHandleTypeOpaqueWin32      = 2,\ncudaExternalMemoryHandleTypeOpaqueWin32Kmt   = 3,\ncudaExternalMemoryHandleTypeD3D12Heap        = 4,\ncudaExternalMemoryHandleTypeD3D12Resource    = 5,\ncudaExternalMemoryHandleTypeD3D11Resource    = 6,\ncudaExternalMemoryHandleTypeD3D11ResourceKmt = 7,\ncudaExternalMemoryHandleTypeNvSciBuf         = 8\n} cudaExternalMemoryHandleType;\n \\endcode\n\n If ::cudaExternalMemoryHandleDesc::type is\n ::cudaExternalMemoryHandleTypeOpaqueFd, then\n ::cudaExternalMemoryHandleDesc::handle::fd must be a valid\n file descriptor referencing a memory object. Ownership of\n the file descriptor is transferred to the CUDA driver when the\n handle is imported successfully. Performing any operations on the\n file descriptor after it is imported results in undefined behavior.\n\n If ::cudaExternalMemoryHandleDesc::type is\n ::cudaExternalMemoryHandleTypeOpaqueWin32, then exactly one\n of ::cudaExternalMemoryHandleDesc::handle::win32::handle and\n ::cudaExternalMemoryHandleDesc::handle::win32::name must not be\n NULL. If ::cudaExternalMemoryHandleDesc::handle::win32::handle\n is not NULL, then it must represent a valid shared NT handle that\n references a memory object. Ownership of this handle is\n not transferred to CUDA after the import operation, so the\n application must release the handle using the appropriate system\n call. If ::cudaExternalMemoryHandleDesc::handle::win32::name\n is not NULL, then it must point to a NULL-terminated array of\n UTF-16 characters that refers to a memory object.\n\n If ::cudaExternalMemoryHandleDesc::type is\n ::cudaExternalMemoryHandleTypeOpaqueWin32Kmt, then\n ::cudaExternalMemoryHandleDesc::handle::win32::handle must\n be non-NULL and\n ::cudaExternalMemoryHandleDesc::handle::win32::name\n must be NULL. The handle specified must be a globally shared KMT\n handle. This handle does not hold a reference to the underlying\n object, and thus will be invalid when all references to the\n memory object are destroyed.\n\n If ::cudaExternalMemoryHandleDesc::type is\n ::cudaExternalMemoryHandleTypeD3D12Heap, then exactly one\n of ::cudaExternalMemoryHandleDesc::handle::win32::handle and\n ::cudaExternalMemoryHandleDesc::handle::win32::name must not be\n NULL. If ::cudaExternalMemoryHandleDesc::handle::win32::handle\n is not NULL, then it must represent a valid shared NT handle that\n is returned by ID3D12Device::CreateSharedHandle when referring to a\n ID3D12Heap object. This handle holds a reference to the underlying\n object. If ::cudaExternalMemoryHandleDesc::handle::win32::name\n is not NULL, then it must point to a NULL-terminated array of\n UTF-16 characters that refers to a ID3D12Heap object.\n\n If ::cudaExternalMemoryHandleDesc::type is\n ::cudaExternalMemoryHandleTypeD3D12Resource, then exactly one\n of ::cudaExternalMemoryHandleDesc::handle::win32::handle and\n ::cudaExternalMemoryHandleDesc::handle::win32::name must not be\n NULL. If ::cudaExternalMemoryHandleDesc::handle::win32::handle\n is not NULL, then it must represent a valid shared NT handle that\n is returned by ID3D12Device::CreateSharedHandle when referring to a\n ID3D12Resource object. This handle holds a reference to the\n underlying object. If\n ::cudaExternalMemoryHandleDesc::handle::win32::name\n is not NULL, then it must point to a NULL-terminated array of\n UTF-16 characters that refers to a ID3D12Resource object.\n\n If ::cudaExternalMemoryHandleDesc::type is\n ::cudaExternalMemoryHandleTypeD3D11Resource,then exactly one\n of ::cudaExternalMemoryHandleDesc::handle::win32::handle and\n ::cudaExternalMemoryHandleDesc::handle::win32::name must not be\n NULL. If ::cudaExternalMemoryHandleDesc::handle::win32::handle is\n not NULL, then it must represent a valid shared NT handle that is\n returned by  IDXGIResource1::CreateSharedHandle when referring to a\n ID3D11Resource object. If\n ::cudaExternalMemoryHandleDesc::handle::win32::name\n is not NULL, then it must point to a NULL-terminated array of\n UTF-16 characters that refers to a ID3D11Resource object.\n\n If ::cudaExternalMemoryHandleDesc::type is\n ::cudaExternalMemoryHandleTypeD3D11ResourceKmt, then\n ::cudaExternalMemoryHandleDesc::handle::win32::handle must\n be non-NULL and ::cudaExternalMemoryHandleDesc::handle::win32::name\n must be NULL. The handle specified must be a valid shared KMT\n handle that is returned by IDXGIResource::GetSharedHandle when\n referring to a ID3D11Resource object.\n\n If ::cudaExternalMemoryHandleDesc::type is\n ::cudaExternalMemoryHandleTypeNvSciBuf, then\n ::cudaExternalMemoryHandleDesc::handle::nvSciBufObject must be NON-NULL\n and reference a valid NvSciBuf object.\n If the NvSciBuf object imported into CUDA is also mapped by other drivers, then the\n application must use ::cudaWaitExternalSemaphoresAsync or ::cudaSignalExternalSemaphoresAsync\n as approprriate barriers to maintain coherence between CUDA and the other drivers.\n See ::cudaExternalSemaphoreWaitSkipNvSciBufMemSync and ::cudaExternalSemaphoreSignalSkipNvSciBufMemSync\n for memory synchronization.\n\n The size of the memory object must be specified in\n ::cudaExternalMemoryHandleDesc::size.\n\n Specifying the flag ::cudaExternalMemoryDedicated in\n ::cudaExternalMemoryHandleDesc::flags indicates that the\n resource is a dedicated resource. The definition of what a\n dedicated resource is outside the scope of this extension.\n This flag must be set if ::cudaExternalMemoryHandleDesc::type\n is one of the following:\n ::cudaExternalMemoryHandleTypeD3D12Resource\n ::cudaExternalMemoryHandleTypeD3D11Resource\n ::cudaExternalMemoryHandleTypeD3D11ResourceKmt\n\n \\param extMem_out    - Returned handle to an external memory object\n \\param memHandleDesc - Memory import handle descriptor\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorInvalidResourceHandle,\n ::cudaErrorOperatingSystem\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\note If the Vulkan memory imported into CUDA is mapped on the CPU then the\n application must use vkInvalidateMappedMemoryRanges/vkFlushMappedMemoryRanges\n as well as appropriate Vulkan pipeline barriers to maintain coherence between\n CPU and GPU. For more information on these APIs, please refer to \"Synchronization\n and Cache Control\" chapter from Vulkan specification.\n\n\n \\sa ::cudaDestroyExternalMemory,\n ::cudaExternalMemoryGetMappedBuffer,\n ::cudaExternalMemoryGetMappedMipmappedArray"]
pub unsafe fn cudaImportExternalMemory<T: ::cuda_libs::types::CudaAsPtr>(
    mut extMem_out: T,
    memHandleDesc: *const cudaExternalMemoryHandleDesc,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaImportExternalMemory(
            extMem_out.as_mut_ptr() as *mut cudaExternalMemory_t,
            memHandleDesc,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Maps a buffer onto an imported memory object\n\n Maps a buffer onto an imported memory object and returns a device\n pointer in \\p devPtr.\n\n The properties of the buffer being mapped must be described in\n \\p bufferDesc. The ::cudaExternalMemoryBufferDesc structure is\n defined as follows:\n\n \\code\ntypedef struct cudaExternalMemoryBufferDesc_st {\nunsigned long long offset;\nunsigned long long size;\nunsigned int flags;\n} cudaExternalMemoryBufferDesc;\n \\endcode\n\n where ::cudaExternalMemoryBufferDesc::offset is the offset in\n the memory object where the buffer's base address is.\n ::cudaExternalMemoryBufferDesc::size is the size of the buffer.\n ::cudaExternalMemoryBufferDesc::flags must be zero.\n\n The offset and size have to be suitably aligned to match the\n requirements of the external API. Mapping two buffers whose ranges\n overlap may or may not result in the same virtual address being\n returned for the overlapped portion. In such cases, the application\n must ensure that all accesses to that region from the GPU are\n volatile. Otherwise writes made via one address are not guaranteed\n to be visible via the other address, even if they're issued by the\n same thread. It is recommended that applications map the combined\n range instead of mapping separate buffers and then apply the\n appropriate offsets to the returned pointer to derive the\n individual buffers.\n\n The returned pointer \\p devPtr must be freed using ::cudaFree.\n\n \\param devPtr     - Returned device pointer to buffer\n \\param extMem     - Handle to external memory object\n \\param bufferDesc - Buffer descriptor\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorInvalidResourceHandle\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaImportExternalMemory,\n ::cudaDestroyExternalMemory,\n ::cudaExternalMemoryGetMappedMipmappedArray"]
pub unsafe fn cudaExternalMemoryGetMappedBuffer<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
>(
    mut devPtr: T,
    extMem: cudaExternalMemory_t,
    bufferDesc: U,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaExternalMemoryGetMappedBuffer(
            devPtr.as_mut_ptr() as *mut *mut ::std::os::raw::c_void,
            extMem,
            bufferDesc.as_const_ptr() as *const cudaExternalMemoryBufferDesc,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Maps a CUDA mipmapped array onto an external memory object\n\n Maps a CUDA mipmapped array onto an external object and returns a\n handle to it in \\p mipmap.\n\n The properties of the CUDA mipmapped array being mapped must be\n described in \\p mipmapDesc. The structure\n ::cudaExternalMemoryMipmappedArrayDesc is defined as follows:\n\n \\code\ntypedef struct cudaExternalMemoryMipmappedArrayDesc_st {\nunsigned long long offset;\ncudaChannelFormatDesc formatDesc;\ncudaExtent extent;\nunsigned int flags;\nunsigned int numLevels;\n} cudaExternalMemoryMipmappedArrayDesc;\n \\endcode\n\n where ::cudaExternalMemoryMipmappedArrayDesc::offset is the\n offset in the memory object where the base level of the mipmap\n chain is.\n ::cudaExternalMemoryMipmappedArrayDesc::formatDesc describes the\n format of the data.\n ::cudaExternalMemoryMipmappedArrayDesc::extent specifies the\n dimensions of the base level of the mipmap chain.\n ::cudaExternalMemoryMipmappedArrayDesc::flags are flags associated\n with CUDA mipmapped arrays. For further details, please refer to\n the documentation for ::cudaMalloc3DArray. Note that if the mipmapped\n array is bound as a color target in the graphics API, then the flag\n ::cudaArrayColorAttachment must be specified in\n ::cudaExternalMemoryMipmappedArrayDesc::flags.\n ::cudaExternalMemoryMipmappedArrayDesc::numLevels specifies\n the total number of levels in the mipmap chain.\n\n The returned CUDA mipmapped array must be freed using ::cudaFreeMipmappedArray.\n\n \\param mipmap     - Returned CUDA mipmapped array\n \\param extMem     - Handle to external memory object\n \\param mipmapDesc - CUDA array descriptor\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorInvalidResourceHandle\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaImportExternalMemory,\n ::cudaDestroyExternalMemory,\n ::cudaExternalMemoryGetMappedBuffer\n\n \\note If ::cudaExternalMemoryHandleDesc::type is\n ::cudaExternalMemoryHandleTypeNvSciBuf, then\n ::cudaExternalMemoryMipmappedArrayDesc::numLevels must not be greater than 1."]
pub unsafe fn cudaExternalMemoryGetMappedMipmappedArray(
    extMem: cudaExternalMemory_t,
    mipmapDesc: *const cudaExternalMemoryMipmappedArrayDesc,
) -> Result<cudaMipmappedArray_t, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<cudaMipmappedArray_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudaExternalMemoryGetMappedMipmappedArray(
            out_0.as_mut_ptr() as *mut _,
            extMem,
            mipmapDesc,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok(out_0.assume_init()) }
    } else {
        Err(status)
    }
}
#[doc = " \\brief Destroys an external memory object.\n\n Destroys the specified external memory object. Any existing buffers\n and CUDA mipmapped arrays mapped onto this object must no longer be\n used and must be explicitly freed using ::cudaFree and\n ::cudaFreeMipmappedArray respectively.\n\n \\param extMem - External memory object to be destroyed\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidResourceHandle\n \\notefnerr\n \\note_init_rt\n \\note_callback\n \\note_destroy_ub\n\n \\sa ::cudaImportExternalMemory,\n ::cudaExternalMemoryGetMappedBuffer,\n ::cudaExternalMemoryGetMappedMipmappedArray"]
pub unsafe fn cudaDestroyExternalMemory(
    extMem: cudaExternalMemory_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaDestroyExternalMemory(extMem) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Imports an external semaphore\n\n Imports an externally allocated synchronization object and returns\n a handle to that in \\p extSem_out.\n\n The properties of the handle being imported must be described in\n \\p semHandleDesc. The ::cudaExternalSemaphoreHandleDesc is defined\n as follows:\n\n \\code\ntypedef struct cudaExternalSemaphoreHandleDesc_st {\ncudaExternalSemaphoreHandleType type;\nunion {\nint fd;\nstruct {\nvoid *handle;\nconst void *name;\n} win32;\nconst void* NvSciSyncObj;\n} handle;\nunsigned int flags;\n} cudaExternalSemaphoreHandleDesc;\n \\endcode\n\n where ::cudaExternalSemaphoreHandleDesc::type specifies the type of\n handle being imported. ::cudaExternalSemaphoreHandleType is defined\n as:\n\n \\code\ntypedef enum cudaExternalSemaphoreHandleType_enum {\ncudaExternalSemaphoreHandleTypeOpaqueFd                = 1,\ncudaExternalSemaphoreHandleTypeOpaqueWin32             = 2,\ncudaExternalSemaphoreHandleTypeOpaqueWin32Kmt          = 3,\ncudaExternalSemaphoreHandleTypeD3D12Fence              = 4,\ncudaExternalSemaphoreHandleTypeD3D11Fence              = 5,\ncudaExternalSemaphoreHandleTypeNvSciSync               = 6,\ncudaExternalSemaphoreHandleTypeKeyedMutex              = 7,\ncudaExternalSemaphoreHandleTypeKeyedMutexKmt           = 8,\ncudaExternalSemaphoreHandleTypeTimelineSemaphoreFd     = 9,\ncudaExternalSemaphoreHandleTypeTimelineSemaphoreWin32  = 10\n} cudaExternalSemaphoreHandleType;\n \\endcode\n\n If ::cudaExternalSemaphoreHandleDesc::type is\n ::cudaExternalSemaphoreHandleTypeOpaqueFd, then\n ::cudaExternalSemaphoreHandleDesc::handle::fd must be a valid file\n descriptor referencing a synchronization object. Ownership of the\n file descriptor is transferred to the CUDA driver when the handle\n is imported successfully. Performing any operations on the file\n descriptor after it is imported results in undefined behavior.\n\n If ::cudaExternalSemaphoreHandleDesc::type is\n ::cudaExternalSemaphoreHandleTypeOpaqueWin32, then exactly one of\n ::cudaExternalSemaphoreHandleDesc::handle::win32::handle and\n ::cudaExternalSemaphoreHandleDesc::handle::win32::name must not be\n NULL. If ::cudaExternalSemaphoreHandleDesc::handle::win32::handle\n is not NULL, then it must represent a valid shared NT handle that\n references a synchronization object. Ownership of this handle is\n not transferred to CUDA after the import operation, so the\n application must release the handle using the appropriate system\n call. If ::cudaExternalSemaphoreHandleDesc::handle::win32::name is\n not NULL, then it must name a valid synchronization object.\n\n If ::cudaExternalSemaphoreHandleDesc::type is\n ::cudaExternalSemaphoreHandleTypeOpaqueWin32Kmt, then\n ::cudaExternalSemaphoreHandleDesc::handle::win32::handle must be\n non-NULL and ::cudaExternalSemaphoreHandleDesc::handle::win32::name\n must be NULL. The handle specified must be a globally shared KMT\n handle. This handle does not hold a reference to the underlying\n object, and thus will be invalid when all references to the\n synchronization object are destroyed.\n\n If ::cudaExternalSemaphoreHandleDesc::type is\n ::cudaExternalSemaphoreHandleTypeD3D12Fence, then exactly one of\n ::cudaExternalSemaphoreHandleDesc::handle::win32::handle and\n ::cudaExternalSemaphoreHandleDesc::handle::win32::name must not be\n NULL. If ::cudaExternalSemaphoreHandleDesc::handle::win32::handle\n is not NULL, then it must represent a valid shared NT handle that\n is returned by ID3D12Device::CreateSharedHandle when referring to a\n ID3D12Fence object. This handle holds a reference to the underlying\n object. If ::cudaExternalSemaphoreHandleDesc::handle::win32::name\n is not NULL, then it must name a valid synchronization object that\n refers to a valid ID3D12Fence object.\n\n If ::cudaExternalSemaphoreHandleDesc::type is\n ::cudaExternalSemaphoreHandleTypeD3D11Fence, then exactly one of\n ::cudaExternalSemaphoreHandleDesc::handle::win32::handle and\n ::cudaExternalSemaphoreHandleDesc::handle::win32::name must not be\n NULL. If ::cudaExternalSemaphoreHandleDesc::handle::win32::handle\n is not NULL, then it must represent a valid shared NT handle that\n is returned by ID3D11Fence::CreateSharedHandle. If\n ::cudaExternalSemaphoreHandleDesc::handle::win32::name\n is not NULL, then it must name a valid synchronization object that\n refers to a valid ID3D11Fence object.\n\n If ::cudaExternalSemaphoreHandleDesc::type is\n ::cudaExternalSemaphoreHandleTypeNvSciSync, then\n ::cudaExternalSemaphoreHandleDesc::handle::nvSciSyncObj\n represents a valid NvSciSyncObj.\n\n ::cudaExternalSemaphoreHandleTypeKeyedMutex, then exactly one of\n ::cudaExternalSemaphoreHandleDesc::handle::win32::handle and\n ::cudaExternalSemaphoreHandleDesc::handle::win32::name must not be\n NULL. If ::cudaExternalSemaphoreHandleDesc::handle::win32::handle\n is not NULL, then it represent a valid shared NT handle that\n is returned by IDXGIResource1::CreateSharedHandle when referring to\n a IDXGIKeyedMutex object.\n\n If ::cudaExternalSemaphoreHandleDesc::type is\n ::cudaExternalSemaphoreHandleTypeKeyedMutexKmt, then\n ::cudaExternalSemaphoreHandleDesc::handle::win32::handle must be\n non-NULL and ::cudaExternalSemaphoreHandleDesc::handle::win32::name\n must be NULL. The handle specified must represent a valid KMT\n handle that is returned by IDXGIResource::GetSharedHandle when\n referring to a IDXGIKeyedMutex object.\n\n If ::cudaExternalSemaphoreHandleDesc::type is\n ::cudaExternalSemaphoreHandleTypeTimelineSemaphoreFd, then\n ::cudaExternalSemaphoreHandleDesc::handle::fd must be a valid file\n descriptor referencing a synchronization object. Ownership of the\n file descriptor is transferred to the CUDA driver when the handle\n is imported successfully. Performing any operations on the file\n descriptor after it is imported results in undefined behavior.\n\n If ::cudaExternalSemaphoreHandleDesc::type is\n ::cudaExternalSemaphoreHandleTypeTimelineSemaphoreWin32, then exactly one of\n ::cudaExternalSemaphoreHandleDesc::handle::win32::handle and\n ::cudaExternalSemaphoreHandleDesc::handle::win32::name must not be\n NULL. If ::cudaExternalSemaphoreHandleDesc::handle::win32::handle\n is not NULL, then it must represent a valid shared NT handle that\n references a synchronization object. Ownership of this handle is\n not transferred to CUDA after the import operation, so the\n application must release the handle using the appropriate system\n call. If ::cudaExternalSemaphoreHandleDesc::handle::win32::name is\n not NULL, then it must name a valid synchronization object.\n\n \\param extSem_out    - Returned handle to an external semaphore\n \\param semHandleDesc - Semaphore import handle descriptor\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidResourceHandle,\n ::cudaErrorOperatingSystem\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaDestroyExternalSemaphore,\n ::cudaSignalExternalSemaphoresAsync,\n ::cudaWaitExternalSemaphoresAsync"]
pub unsafe fn cudaImportExternalSemaphore<T: ::cuda_libs::types::CudaAsPtr>(
    mut extSem_out: T,
    semHandleDesc: *const cudaExternalSemaphoreHandleDesc,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaImportExternalSemaphore(
            extSem_out.as_mut_ptr() as *mut cudaExternalSemaphore_t,
            semHandleDesc,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Signals a set of external semaphore objects\n\n Enqueues a signal operation on a set of externally allocated\n semaphore object in the specified stream. The operations will be\n executed when all prior operations in the stream complete.\n\n The exact semantics of signaling a semaphore depends on the type of\n the object.\n\n If the semaphore object is any one of the following types:\n ::cudaExternalSemaphoreHandleTypeOpaqueFd,\n ::cudaExternalSemaphoreHandleTypeOpaqueWin32,\n ::cudaExternalSemaphoreHandleTypeOpaqueWin32Kmt\n then signaling the semaphore will set it to the signaled state.\n\n If the semaphore object is any one of the following types:\n ::cudaExternalSemaphoreHandleTypeD3D12Fence,\n ::cudaExternalSemaphoreHandleTypeD3D11Fence,\n ::cudaExternalSemaphoreHandleTypeTimelineSemaphoreFd,\n ::cudaExternalSemaphoreHandleTypeTimelineSemaphoreWin32\n then the semaphore will be set to the value specified in\n ::cudaExternalSemaphoreSignalParams::params::fence::value.\n\n If the semaphore object is of the type ::cudaExternalSemaphoreHandleTypeNvSciSync\n this API sets ::cudaExternalSemaphoreSignalParams::params::nvSciSync::fence to a\n value that can be used by subsequent waiters of the same NvSciSync object to\n order operations with those currently submitted in \\p stream. Such an update\n will overwrite previous contents of\n ::cudaExternalSemaphoreSignalParams::params::nvSciSync::fence. By default,\n signaling such an external semaphore object causes appropriate memory synchronization\n operations to be performed over all the external memory objects that are imported as\n ::cudaExternalMemoryHandleTypeNvSciBuf. This ensures that any subsequent accesses\n made by other importers of the same set of NvSciBuf memory object(s) are coherent.\n These operations can be skipped by specifying the flag\n ::cudaExternalSemaphoreSignalSkipNvSciBufMemSync, which can be used as a\n performance optimization when data coherency is not required. But specifying this\n flag in scenarios where data coherency is required results in undefined behavior.\n Also, for semaphore object of the type ::cudaExternalSemaphoreHandleTypeNvSciSync,\n if the NvSciSyncAttrList used to create the NvSciSyncObj had not set the flags in\n ::cudaDeviceGetNvSciSyncAttributes to cudaNvSciSyncAttrSignal, this API will return\n cudaErrorNotSupported.\n\n ::cudaExternalSemaphoreSignalParams::params::nvSciSync::fence associated with\n semaphore object of the type ::cudaExternalSemaphoreHandleTypeNvSciSync can be\n deterministic. For this the NvSciSyncAttrList used to create the semaphore object\n must have value of NvSciSyncAttrKey_RequireDeterministicFences key set to true.\n Deterministic fences allow users to enqueue a wait over the semaphore object even\n before corresponding signal is enqueued. For such a semaphore object, CUDA guarantees\n that each signal operation will increment the fence value by '1'. Users are expected\n to track count of signals enqueued on the semaphore object and insert waits accordingly.\n When such a semaphore object is signaled from multiple streams, due to concurrent\n stream execution, it is possible that the order in which the semaphore gets signaled\n is indeterministic. This could lead to waiters of the semaphore getting unblocked\n incorrectly. Users are expected to handle such situations, either by not using the\n same semaphore object with deterministic fence support enabled in different streams\n or by adding explicit dependency amongst such streams so that the semaphore is\n signaled in order.\n ::cudaExternalSemaphoreSignalParams::params::nvSciSync::fence associated with\n semaphore object of the type ::cudaExternalSemaphoreHandleTypeNvSciSync can be\n timestamp enabled. For this the NvSciSyncAttrList used to create the object must\n have the value of NvSciSyncAttrKey_WaiterRequireTimestamps key set to true. Timestamps\n are emitted asynchronously by the GPU and CUDA saves the GPU timestamp in the\n corresponding NvSciSyncFence at the time of signal on GPU. Users are expected to\n convert GPU clocks to CPU clocks using appropriate scaling functions. Users are\n expected to wait for the completion of the fence before extracting timestamp using\n appropriate NvSciSync APIs. Users are expected to ensure that there is only one\n outstanding timestamp enabled fence per Cuda-NvSciSync object at any point of time,\n failing which leads to undefined behavior. Extracting the timestamp before the\n corresponding fence is signalled could lead to undefined behaviour. Timestamp\n extracted via appropriate NvSciSync API would be in microseconds.\n\n If the semaphore object is any one of the following types:\n ::cudaExternalSemaphoreHandleTypeKeyedMutex,\n ::cudaExternalSemaphoreHandleTypeKeyedMutexKmt,\n then the keyed mutex will be released with the key specified in\n ::cudaExternalSemaphoreSignalParams::params::keyedmutex::key.\n\n \\param extSemArray - Set of external semaphores to be signaled\n \\param paramsArray - Array of semaphore parameters\n \\param numExtSems  - Number of semaphores to signal\n \\param stream     - Stream to enqueue the signal operations in\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidResourceHandle\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaImportExternalSemaphore,\n ::cudaDestroyExternalSemaphore,\n ::cudaWaitExternalSemaphoresAsync"]
pub unsafe fn cudaSignalExternalSemaphoresAsync<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
>(
    extSemArray: T,
    paramsArray: U,
    numExtSems: ::std::os::raw::c_uint,
    stream: cudaStream_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaSignalExternalSemaphoresAsync(
            extSemArray.as_const_ptr() as *const cudaExternalSemaphore_t,
            paramsArray.as_const_ptr() as *const cudaExternalSemaphoreSignalParams,
            numExtSems,
            stream,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Waits on a set of external semaphore objects\n\n Enqueues a wait operation on a set of externally allocated\n semaphore object in the specified stream. The operations will be\n executed when all prior operations in the stream complete.\n\n The exact semantics of waiting on a semaphore depends on the type\n of the object.\n\n If the semaphore object is any one of the following types:\n ::cudaExternalSemaphoreHandleTypeOpaqueFd,\n ::cudaExternalSemaphoreHandleTypeOpaqueWin32,\n ::cudaExternalSemaphoreHandleTypeOpaqueWin32Kmt\n then waiting on the semaphore will wait until the semaphore reaches\n the signaled state. The semaphore will then be reset to the\n unsignaled state. Therefore for every signal operation, there can\n only be one wait operation.\n\n If the semaphore object is any one of the following types:\n ::cudaExternalSemaphoreHandleTypeD3D12Fence,\n ::cudaExternalSemaphoreHandleTypeD3D11Fence,\n ::cudaExternalSemaphoreHandleTypeTimelineSemaphoreFd,\n ::cudaExternalSemaphoreHandleTypeTimelineSemaphoreWin32\n then waiting on the semaphore will wait until the value of the\n semaphore is greater than or equal to\n ::cudaExternalSemaphoreWaitParams::params::fence::value.\n\n If the semaphore object is of the type ::cudaExternalSemaphoreHandleTypeNvSciSync\n then, waiting on the semaphore will wait until the\n ::cudaExternalSemaphoreSignalParams::params::nvSciSync::fence is signaled by the\n signaler of the NvSciSyncObj that was associated with this semaphore object.\n By default, waiting on such an external semaphore object causes appropriate\n memory synchronization operations to be performed over all external memory objects\n that are imported as ::cudaExternalMemoryHandleTypeNvSciBuf. This ensures that\n any subsequent accesses made by other importers of the same set of NvSciBuf memory\n object(s) are coherent. These operations can be skipped by specifying the flag\n ::cudaExternalSemaphoreWaitSkipNvSciBufMemSync, which can be used as a\n performance optimization when data coherency is not required. But specifying this\n flag in scenarios where data coherency is required results in undefined behavior.\n Also, for semaphore object of the type ::cudaExternalSemaphoreHandleTypeNvSciSync,\n if the NvSciSyncAttrList used to create the NvSciSyncObj had not set the flags in\n ::cudaDeviceGetNvSciSyncAttributes to cudaNvSciSyncAttrWait, this API will return\n cudaErrorNotSupported.\n\n If the semaphore object is any one of the following types:\n ::cudaExternalSemaphoreHandleTypeKeyedMutex,\n ::cudaExternalSemaphoreHandleTypeKeyedMutexKmt,\n then the keyed mutex will be acquired when it is released with the key specified\n in ::cudaExternalSemaphoreSignalParams::params::keyedmutex::key or\n until the timeout specified by\n ::cudaExternalSemaphoreSignalParams::params::keyedmutex::timeoutMs\n has lapsed. The timeout interval can either be a finite value\n specified in milliseconds or an infinite value. In case an infinite\n value is specified the timeout never elapses. The windows INFINITE\n macro must be used to specify infinite timeout\n\n \\param extSemArray - External semaphores to be waited on\n \\param paramsArray - Array of semaphore parameters\n \\param numExtSems  - Number of semaphores to wait on\n \\param stream      - Stream to enqueue the wait operations in\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidResourceHandle\n ::cudaErrorTimeout\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaImportExternalSemaphore,\n ::cudaDestroyExternalSemaphore,\n ::cudaSignalExternalSemaphoresAsync"]
pub unsafe fn cudaWaitExternalSemaphoresAsync<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
>(
    extSemArray: T,
    paramsArray: U,
    numExtSems: ::std::os::raw::c_uint,
    stream: cudaStream_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaWaitExternalSemaphoresAsync(
            extSemArray.as_const_ptr() as *const cudaExternalSemaphore_t,
            paramsArray.as_const_ptr() as *const cudaExternalSemaphoreWaitParams,
            numExtSems,
            stream,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Destroys an external semaphore\n\n Destroys an external semaphore object and releases any references\n to the underlying resource. Any outstanding signals or waits must\n have completed before the semaphore is destroyed.\n\n \\param extSem - External semaphore to be destroyed\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidResourceHandle\n \\notefnerr\n \\note_init_rt\n \\note_callback\n \\note_destroy_ub\n\n \\sa ::cudaImportExternalSemaphore,\n ::cudaSignalExternalSemaphoresAsync,\n ::cudaWaitExternalSemaphoresAsync"]
pub unsafe fn cudaDestroyExternalSemaphore(
    extSem: cudaExternalSemaphore_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaDestroyExternalSemaphore(extSem) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Launches a device function\n\n The function invokes kernel \\p func on \\p gridDim (\\p gridDim.x &times; \\p gridDim.y\n &times; \\p gridDim.z) grid of blocks. Each block contains \\p blockDim (\\p blockDim.x &times;\n \\p blockDim.y &times; \\p blockDim.z) threads.\n\n If the kernel has N parameters the \\p args should point to array of N pointers.\n Each pointer, from <tt>args[0]</tt> to <tt>args[N - 1]</tt>, point to the region\n of memory from which the actual parameter will be copied.\n\n For templated functions, pass the function symbol as follows:\n func_name<template_arg_0,...,template_arg_N>\n\n \\p sharedMem sets the amount of dynamic shared memory that will be available to\n each thread block.\n\n \\p stream specifies a stream the invocation is associated to.\n\n \\param func        - Device function symbol\n \\param gridDim     - Grid dimentions\n \\param blockDim    - Block dimentions\n \\param args        - Arguments\n \\param sharedMem   - Shared memory\n \\param stream      - Stream identifier\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidDeviceFunction,\n ::cudaErrorInvalidConfiguration,\n ::cudaErrorLaunchFailure,\n ::cudaErrorLaunchTimeout,\n ::cudaErrorLaunchOutOfResources,\n ::cudaErrorSharedObjectInitFailed,\n ::cudaErrorInvalidPtx,\n ::cudaErrorUnsupportedPtxVersion,\n ::cudaErrorNoKernelImageForDevice,\n ::cudaErrorJitCompilerNotFound,\n ::cudaErrorJitCompilationDisabled\n \\note_null_stream\n \\notefnerr\n \\note_init_rt\n \\note_callback\n \\note_cudaKernel_t\n\n \\sa\n \\ref ::cudaLaunchKernel(const T *func, dim3 gridDim, dim3 blockDim, void **args, size_t sharedMem, cudaStream_t stream) \"cudaLaunchKernel (C++ API)\",\n ::cuLaunchKernel"]
pub unsafe fn cudaLaunchKernel<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
>(
    func: T,
    gridDim: dim3,
    blockDim: dim3,
    mut args: U,
    sharedMem: usize,
    stream: cudaStream_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaLaunchKernel(
            func.as_const_ptr() as *const ::std::os::raw::c_void,
            gridDim,
            blockDim,
            args.as_mut_ptr() as *mut *mut ::std::os::raw::c_void,
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
#[doc = " \\brief Launches a CUDA function with launch-time configuration\n\n Note that the functionally equivalent variadic template ::cudaLaunchKernelEx\n is available for C++11 and newer.\n\n Invokes the kernel \\p func on \\p config->gridDim (\\p config->gridDim.x\n &times; \\p config->gridDim.y &times; \\p config->gridDim.z) grid of blocks.\n Each block contains \\p config->blockDim (\\p config->blockDim.x &times;\n \\p config->blockDim.y &times; \\p config->blockDim.z) threads.\n\n \\p config->dynamicSmemBytes sets the amount of dynamic shared memory that\n will be available to each thread block.\n\n \\p config->stream specifies a stream the invocation is associated to.\n\n Configuration beyond grid and block dimensions, dynamic shared memory size,\n and stream can be provided with the following two fields of \\p config:\n\n \\p config->attrs is an array of \\p config->numAttrs contiguous\n ::cudaLaunchAttribute elements. The value of this pointer is not considered\n if \\p config->numAttrs is zero. However, in that case, it is recommended to\n set the pointer to NULL.\n \\p config->numAttrs is the number of attributes populating the first\n \\p config->numAttrs positions of the \\p config->attrs array.\n\n If the kernel has N parameters the \\p args should point to array of N\n pointers. Each pointer, from <tt>args[0]</tt> to <tt>args[N - 1]</tt>, point\n to the region of memory from which the actual parameter will be copied.\n\n N.B. This function is so named to avoid unintentionally invoking the\n      templated version, \\p cudaLaunchKernelEx, for kernels taking a single\n      void** or void* parameter.\n\n \\param config - Launch configuration\n \\param func   - Kernel to launch\n \\param args   - Array of pointers to kernel parameters\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidDeviceFunction,\n ::cudaErrorInvalidConfiguration,\n ::cudaErrorLaunchFailure,\n ::cudaErrorLaunchTimeout,\n ::cudaErrorLaunchOutOfResources,\n ::cudaErrorSharedObjectInitFailed,\n ::cudaErrorInvalidPtx,\n ::cudaErrorUnsupportedPtxVersion,\n ::cudaErrorNoKernelImageForDevice,\n ::cudaErrorJitCompilerNotFound,\n ::cudaErrorJitCompilationDisabled\n \\note_null_stream\n \\notefnerr\n \\note_init_rt\n \\note_callback\n \\note_cudaKernel_t\n\n \\sa\n \\ref ::cudaLaunchKernelEx(const cudaLaunchConfig_t *config, void (*kernel)(ExpTypes...), ActTypes &&... args) \"cudaLaunchKernelEx (C++ API)\",\n ::cuLaunchKernelEx"]
pub unsafe fn cudaLaunchKernelExC<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
>(
    config: T,
    func: U,
    mut args: V,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaLaunchKernelExC(
            config.as_const_ptr() as *const cudaLaunchConfig_t,
            func.as_const_ptr() as *const ::std::os::raw::c_void,
            args.as_mut_ptr() as *mut *mut ::std::os::raw::c_void,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Launches a device function where thread blocks can cooperate and synchronize as they execute\n\n The function invokes kernel \\p func on \\p gridDim (\\p gridDim.x &times; \\p gridDim.y\n &times; \\p gridDim.z) grid of blocks. Each block contains \\p blockDim (\\p blockDim.x &times;\n \\p blockDim.y &times; \\p blockDim.z) threads.\n\n The device on which this kernel is invoked must have a non-zero value for\n the device attribute ::cudaDevAttrCooperativeLaunch.\n\n The total number of blocks launched cannot exceed the maximum number of blocks per\n multiprocessor as returned by ::cudaOccupancyMaxActiveBlocksPerMultiprocessor (or\n ::cudaOccupancyMaxActiveBlocksPerMultiprocessorWithFlags) times the number of multiprocessors\n as specified by the device attribute ::cudaDevAttrMultiProcessorCount.\n\n The kernel cannot make use of CUDA dynamic parallelism.\n\n If the kernel has N parameters the \\p args should point to array of N pointers.\n Each pointer, from <tt>args[0]</tt> to <tt>args[N - 1]</tt>, point to the region\n of memory from which the actual parameter will be copied.\n\n For templated functions, pass the function symbol as follows:\n func_name<template_arg_0,...,template_arg_N>\n\n \\p sharedMem sets the amount of dynamic shared memory that will be available to\n each thread block.\n\n \\p stream specifies a stream the invocation is associated to.\n\n \\param func        - Device function symbol\n \\param gridDim     - Grid dimentions\n \\param blockDim    - Block dimentions\n \\param args        - Arguments\n \\param sharedMem   - Shared memory\n \\param stream      - Stream identifier\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidDeviceFunction,\n ::cudaErrorInvalidConfiguration,\n ::cudaErrorLaunchFailure,\n ::cudaErrorLaunchTimeout,\n ::cudaErrorLaunchOutOfResources,\n ::cudaErrorCooperativeLaunchTooLarge,\n ::cudaErrorSharedObjectInitFailed\n \\note_null_stream\n \\notefnerr\n \\note_init_rt\n \\note_callback\n \\note_cudaKernel_t\n\n \\sa\n \\ref ::cudaLaunchCooperativeKernel(const T *func, dim3 gridDim, dim3 blockDim, void **args, size_t sharedMem, cudaStream_t stream) \"cudaLaunchCooperativeKernel (C++ API)\",\n ::cuLaunchCooperativeKernel"]
pub unsafe fn cudaLaunchCooperativeKernel<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
>(
    func: T,
    gridDim: dim3,
    blockDim: dim3,
    mut args: U,
    sharedMem: usize,
    stream: cudaStream_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaLaunchCooperativeKernel(
            func.as_const_ptr() as *const ::std::os::raw::c_void,
            gridDim,
            blockDim,
            args.as_mut_ptr() as *mut *mut ::std::os::raw::c_void,
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
#[doc = " \\brief Sets the preferred cache configuration for a device function\n\n On devices where the L1 cache and shared memory use the same hardware\n resources, this sets through \\p cacheConfig the preferred cache configuration\n for the function specified via \\p func. This is only a preference. The\n runtime will use the requested configuration if possible, but it is free to\n choose a different configuration if required to execute \\p func.\n\n \\p func is a device function symbol and must be declared as a\n \\c __global__ function. If the specified function does not exist,\n then ::cudaErrorInvalidDeviceFunction is returned. For templated functions,\n pass the function symbol as follows: func_name<template_arg_0,...,template_arg_N>\n\n This setting does nothing on devices where the size of the L1 cache and\n shared memory are fixed.\n\n Launching a kernel with a different preference than the most recent\n preference setting may insert a device-side synchronization point.\n\n The supported cache configurations are:\n - ::cudaFuncCachePreferNone: no preference for shared memory or L1 (default)\n - ::cudaFuncCachePreferShared: prefer larger shared memory and smaller L1 cache\n - ::cudaFuncCachePreferL1: prefer larger L1 cache and smaller shared memory\n - ::cudaFuncCachePreferEqual: prefer equal size L1 cache and shared memory\n\n \\param func        - Device function symbol\n \\param cacheConfig - Requested cache configuration\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidDeviceFunction\n \\notefnerr\n \\note_string_api_deprecation2\n \\note_init_rt\n \\note_callback\n\n \\note This API does not accept a ::cudaKernel_t casted as void*. If cache config modification\n is required for a ::cudaKernel_t (or a __global__ function), it can be replaced with a call to\n ::cudaFuncSetAttributes with the attribute ::cudaFuncAttributePreferredSharedMemoryCarveout\n to specify a more granular L1 cache and shared memory split configuration.\n\n \\sa\n \\ref ::cudaFuncSetCacheConfig(T*, enum cudaFuncCache) \"cudaFuncSetCacheConfig (C++ API)\",\n \\ref ::cudaFuncGetAttributes(struct cudaFuncAttributes*, const void*) \"cudaFuncGetAttributes (C API)\",\n \\ref ::cudaLaunchKernel(const void *func, dim3 gridDim, dim3 blockDim, void **args, size_t sharedMem, cudaStream_t stream) \"cudaLaunchKernel (C API)\",\n ::cuFuncSetCacheConfig"]
pub unsafe fn cudaFuncSetCacheConfig<T: ::cuda_libs::types::CudaAsPtr>(
    func: T,
    cacheConfig: cudaFuncCache,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaFuncSetCacheConfig(
            func.as_const_ptr() as *const ::std::os::raw::c_void,
            cacheConfig,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Find out attributes for a given function\n\n This function obtains the attributes of a function specified via \\p func.\n \\p func is a device function symbol and must be declared as a\n \\c __global__ function. The fetched attributes are placed in \\p attr.\n If the specified function does not exist, then it is assumed to\n be a ::cudaKernel_t and used as is.\n For templated functions, pass the function symbol as follows:\n func_name<template_arg_0,...,template_arg_N>\n\n Note that some function attributes such as\n \\ref ::cudaFuncAttributes::maxThreadsPerBlock \"maxThreadsPerBlock\"\n may vary based on the device that is currently being used.\n\n \\param attr - Return pointer to function's attributes\n \\param func - Device function symbol\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidDeviceFunction\n \\notefnerr\n \\note_string_api_deprecation2\n \\note_init_rt\n \\note_callback\n \\note_cudaKernel_t\n\n \\sa\n \\ref ::cudaFuncSetCacheConfig(const void*, enum cudaFuncCache) \"cudaFuncSetCacheConfig (C API)\",\n \\ref ::cudaFuncGetAttributes(struct cudaFuncAttributes*, T*) \"cudaFuncGetAttributes (C++ API)\",\n \\ref ::cudaLaunchKernel(const void *func, dim3 gridDim, dim3 blockDim, void **args, size_t sharedMem, cudaStream_t stream) \"cudaLaunchKernel (C API)\",\n ::cuFuncGetAttribute"]
pub unsafe fn cudaFuncGetAttributes(
    func: *const ::std::os::raw::c_void,
) -> Result<cudaFuncAttributes, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<cudaFuncAttributes> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaFuncGetAttributes(out_0.as_mut_ptr() as *mut _, func) };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok(out_0.assume_init()) }
    } else {
        Err(status)
    }
}
#[doc = " \\brief Set attributes for a given function\n\n This function sets the attributes of a function specified via \\p func.\n The parameter \\p func must be a pointer to a function that executes\n on the device. The parameter specified by \\p func must be declared as a \\p __global__\n function. The enumeration defined by \\p attr is set to the value defined by \\p value.\n If the specified function does not exist, then it is assumed to\n be a ::cudaKernel_t and used as is.\n If the specified attribute cannot be written, or if the value is incorrect,\n then ::cudaErrorInvalidValue is returned.\n\n Valid values for \\p attr are:\n - ::cudaFuncAttributeMaxDynamicSharedMemorySize - The requested maximum size in bytes of dynamically-allocated shared memory. The sum of this value and the function attribute ::sharedSizeBytes\n   cannot exceed the device attribute ::cudaDevAttrMaxSharedMemoryPerBlockOptin. The maximal size of requestable dynamic shared memory may differ by GPU architecture.\n - ::cudaFuncAttributePreferredSharedMemoryCarveout - On devices where the L1 cache and shared memory use the same hardware resources,\n   this sets the shared memory carveout preference, in percent of the total shared memory. See ::cudaDevAttrMaxSharedMemoryPerMultiprocessor.\n   This is only a hint, and the driver can choose a different ratio if required to execute the function.\n - ::cudaFuncAttributeRequiredClusterWidth: The required cluster width in\n   blocks. The width, height, and depth values must either all be 0 or all be\n   positive. The validity of the cluster dimensions is checked at launch time.\n   If the value is set during compile time, it cannot be set at runtime.\n   Setting it at runtime will return cudaErrorNotPermitted.\n - ::cudaFuncAttributeRequiredClusterHeight: The required cluster height in\n   blocks. The width, height, and depth values must either all be 0 or all be\n   positive. The validity of the cluster dimensions is checked at launch time.\n   If the value is set during compile time, it cannot be set at runtime.\n   Setting it at runtime will return cudaErrorNotPermitted.\n - ::cudaFuncAttributeRequiredClusterDepth: The required cluster depth in\n   blocks. The width, height, and depth values must either all be 0 or all be\n   positive. The validity of the cluster dimensions is checked at launch time.\n   If the value is set during compile time, it cannot be set at runtime.\n   Setting it at runtime will return cudaErrorNotPermitted.\n - ::cudaFuncAttributeNonPortableClusterSizeAllowed: Indicates whether the\n   function can be launched with non-portable cluster size. 1 is allowed, 0 is\n   disallowed.\n - ::cudaFuncAttributeClusterSchedulingPolicyPreference: The block\n   scheduling policy of a function. The value type is cudaClusterSchedulingPolicy.\n\n \\param func  - Function to get attributes of\n \\param attr  - Attribute to set\n \\param value - Value to set\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidDeviceFunction,\n ::cudaErrorInvalidValue\n \\notefnerr\n \\note_init_rt\n \\note_callback\n \\note_cudaKernel_t\n\n \\ref ::cudaLaunchKernel(const T *func, dim3 gridDim, dim3 blockDim, void **args, size_t sharedMem, cudaStream_t stream) \"cudaLaunchKernel (C++ API)\",\n \\ref ::cudaFuncSetCacheConfig(T*, enum cudaFuncCache) \"cudaFuncSetCacheConfig (C++ API)\",\n \\ref ::cudaFuncGetAttributes(struct cudaFuncAttributes*, const void*) \"cudaFuncGetAttributes (C API)\","]
pub unsafe fn cudaFuncSetAttribute<T: ::cuda_libs::types::CudaAsPtr>(
    func: T,
    attr: cudaFuncAttribute,
    value: ::std::os::raw::c_int,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaFuncSetAttribute(
            func.as_const_ptr() as *const ::std::os::raw::c_void,
            attr,
            value,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Returns the function name for a device entry function pointer.\n\n Returns in \\p **name the function name associated with the symbol \\p func .\n The function name is returned as a null-terminated string. This API may\n return a mangled name if the function is not declared as having C linkage.\n If \\p **name is NULL, ::cudaErrorInvalidValue is returned.\n If \\p func is not a device entry function, then it is assumed to\n be a ::cudaKernel_t and used as is.\n\n \\param name - The returned name of the function\n \\param func - The function pointer to retrieve name for\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorInvalidDeviceFunction\n \\notefnerr\n \\note_init_rt\n \\note_callback\n \\note_cudaKernel_t\n\n \\ref ::cudaFuncGetName(const char **name, const T *func) \"cudaFuncGetName (C++ API)\""]
pub unsafe fn cudaFuncGetName<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
>(
    mut name: T,
    func: U,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaFuncGetName(
            name.as_mut_ptr() as *mut *const ::std::os::raw::c_char,
            func.as_const_ptr() as *const ::std::os::raw::c_void,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Returns the offset and size of a kernel parameter in the device-side parameter layout.\n\n Queries the kernel parameter at \\p paramIndex in \\p func's list of parameters and returns\n parameter information via \\p paramOffset and \\p paramSize. \\p paramOffset returns the\n offset of the parameter in the device-side parameter layout. \\p paramSize returns the size\n in bytes of the parameter. This information can be used to update kernel node parameters\n from the device via ::cudaGraphKernelNodeSetParam() and ::cudaGraphKernelNodeUpdatesApply().\n \\p paramIndex must be less than the number of parameters that \\p func takes.\n\n \\param func        - The function to query\n \\param paramIndex  - The parameter index to query\n \\param paramOffset - The offset into the device-side parameter layout at which the parameter resides\n \\param paramSize   - The size of the parameter in the device-side parameter layout\n\n \\return\n ::CUDA_SUCCESS,\n ::CUDA_ERROR_INVALID_VALUE,\n \\notefnerr\n \\note_cudaKernel_t"]
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
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok((out_2.assume_init(), out_3.assume_init())) }
    } else {
        Err(status)
    }
}
#[doc = " \\brief Returns the number of parameters used by the function\n\n Queries the number of kernel parameters used by \\p func and returns it in \\p paramCount.\n\n \\param func        - The function to query\n \\param paramCount  - Returns the number of parameters used by the function\n\n \\return\n ::CUDA_SUCCESS,\n ::CUDA_ERROR_INVALID_VALUE,\n \\notefnerr\n \\note_cudaKernel_t"]
pub unsafe fn cudaFuncGetParamCount(
    func: *const ::std::os::raw::c_void,
) -> Result<usize, crate::sys::cudaError> {
    let mut out_1: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaFuncGetParamCount(func, out_1.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok(out_1.assume_init()) }
    } else {
        Err(status)
    }
}
#[doc = " \\brief Enqueues a host function call in a stream\n\n Enqueues a host function to run in a stream.  The function will be called\n after currently enqueued work and will block work added after it.\n\n The host function must not make any CUDA API calls.  Attempting to use a\n CUDA API may result in ::cudaErrorNotPermitted, but this is not required.\n The host function must not perform any synchronization that may depend on\n outstanding CUDA work not mandated to run earlier.  Host functions without a\n mandated order (such as in independent streams) execute in undefined order\n and may be serialized.\n\n For the purposes of Unified Memory, execution makes a number of guarantees:\n <ul>\n   <li>The stream is considered idle for the duration of the function's\n   execution.  Thus, for example, the function may always use memory attached\n   to the stream it was enqueued in.</li>\n   <li>The start of execution of the function has the same effect as\n   synchronizing an event recorded in the same stream immediately prior to\n   the function.  It thus synchronizes streams which have been \"joined\"\n   prior to the function.</li>\n   <li>Adding device work to any stream does not have the effect of making\n   the stream active until all preceding host functions and stream callbacks\n   have executed.  Thus, for\n   example, a function might use global attached memory even if work has\n   been added to another stream, if the work has been ordered behind the\n   function call with an event.</li>\n   <li>Completion of the function does not cause a stream to become\n   active except as described above.  The stream will remain idle\n   if no device work follows the function, and will remain idle across\n   consecutive host functions or stream callbacks without device work in\n   between.  Thus, for example,\n   stream synchronization can be done by signaling from a host function at the\n   end of the stream.</li>\n </ul>\n\n Note that, in constrast to ::cuStreamAddCallback, the function will not be\n called in the event of an error in the CUDA context.\n\n \\param hStream  - Stream to enqueue function call in\n \\param fn       - The function to call once preceding stream operations are complete\n \\param userData - User-specified data to be passed to the function\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidResourceHandle,\n ::cudaErrorInvalidValue,\n ::cudaErrorNotSupported\n \\note_null_stream\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaStreamCreate,\n ::cudaStreamQuery,\n ::cudaStreamSynchronize,\n ::cudaStreamWaitEvent,\n ::cudaStreamDestroy,\n ::cudaMallocManaged,\n ::cudaStreamAttachMemAsync,\n ::cudaStreamAddCallback,\n ::cuLaunchHostFunc"]
pub unsafe fn cudaLaunchHostFunc<T: ::cuda_libs::types::CudaAsPtr>(
    stream: cudaStream_t,
    fn_: cudaHostFn_t,
    mut userData: T,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaLaunchHostFunc(
            stream,
            fn_,
            userData.as_mut_ptr() as *mut ::std::os::raw::c_void,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Enqueues a host function call in a stream\n\n Enqueues a host function to run in a stream.  The function will be called\n after currently enqueued work and will block work added after it.\n\n The host function must not make any CUDA API calls.  Attempting to use a\n CUDA API may result in ::cudaErrorNotPermitted, but this is not required.\n The host function must not perform any synchronization that may depend on\n outstanding CUDA work not mandated to run earlier.  Host functions without a\n mandated order (such as in independent streams) execute in undefined order\n and may be serialized.\n\n For the purposes of Unified Memory, execution makes a number of guarantees:\n <ul>\n   <li>The stream is considered idle for the duration of the function's\n   execution.  Thus, for example, the function may always use memory attached\n   to the stream it was enqueued in.</li>\n   <li>The start of execution of the function has the same effect as\n   synchronizing an event recorded in the same stream immediately prior to\n   the function.  It thus synchronizes streams which have been \"joined\"\n   prior to the function.</li>\n   <li>Adding device work to any stream does not have the effect of making\n   the stream active until all preceding host functions and stream callbacks\n   have executed.  Thus, for\n   example, a function might use global attached memory even if work has\n   been added to another stream, if the work has been ordered behind the\n   function call with an event.</li>\n   <li>Completion of the function does not cause a stream to become\n   active except as described above.  The stream will remain idle\n   if no device work follows the function, and will remain idle across\n   consecutive host functions or stream callbacks without device work in\n   between.  Thus, for example,\n   stream synchronization can be done by signaling from a host function at the\n   end of the stream.</li>\n </ul>\n\n Note that, in constrast to ::cuStreamAddCallback, the function will not be\n called in the event of an error in the CUDA context.\n\n \\param hStream  - Stream to enqueue function call in\n \\param fn       - The function to call once preceding stream operations are complete\n \\param userData - User-specified data to be passed to the function\n \\param syncMode - Sync mode for the host function\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidResourceHandle,\n ::cudaErrorInvalidValue,\n ::cudaErrorNotSupported\n \\note_null_stream\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaStreamCreate,\n ::cudaStreamQuery,\n ::cudaStreamSynchronize,\n ::cudaStreamWaitEvent,\n ::cudaStreamDestroy,\n ::cudaMallocManaged,\n ::cudaStreamAttachMemAsync,\n ::cudaStreamAddCallback,\n ::cuLaunchHostFunc"]
pub unsafe fn cudaLaunchHostFunc_v2<T: ::cuda_libs::types::CudaAsPtr>(
    stream: cudaStream_t,
    fn_: cudaHostFn_t,
    mut userData: T,
    syncMode: ::std::os::raw::c_uint,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaLaunchHostFunc_v2(
            stream,
            fn_,
            userData.as_mut_ptr() as *mut ::std::os::raw::c_void,
            syncMode,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Sets the shared memory configuration for a device function\n\n \\deprecated\n\n On devices with configurable shared memory banks, this function will\n force all subsequent launches of the specified device function to have\n the given shared memory bank size configuration. On any given launch of the\n function, the shared memory configuration of the device will be temporarily\n changed if needed to suit the function's preferred configuration. Changes in\n shared memory configuration between subsequent launches of functions,\n may introduce a device side synchronization point.\n\n Any per-function setting of shared memory bank size set via\n ::cudaFuncSetSharedMemConfig will override the device wide setting set by\n ::cudaDeviceSetSharedMemConfig.\n\n Changing the shared memory bank size will not increase shared memory usage\n or affect occupancy of kernels, but may have major effects on performance.\n Larger bank sizes will allow for greater potential bandwidth to shared memory,\n but will change what kinds of accesses to shared memory will result in bank\n conflicts.\n\n This function will do nothing on devices with fixed shared memory bank size.\n\n For templated functions, pass the function symbol as follows:\n func_name<template_arg_0,...,template_arg_N>\n\n The supported bank configurations are:\n - ::cudaSharedMemBankSizeDefault: use the device's shared memory configuration\n   when launching this function.\n - ::cudaSharedMemBankSizeFourByte: set shared memory bank width to be\n   four bytes natively when launching this function.\n - ::cudaSharedMemBankSizeEightByte: set shared memory bank width to be eight\n   bytes natively when launching this function.\n\n \\param func   - Device function symbol\n \\param config - Requested shared memory configuration\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidDeviceFunction,\n ::cudaErrorInvalidValue,\n \\notefnerr\n \\note_string_api_deprecation2\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaDeviceSetSharedMemConfig,\n ::cudaDeviceGetSharedMemConfig,\n ::cudaDeviceSetCacheConfig,\n ::cudaDeviceGetCacheConfig,\n ::cudaFuncSetCacheConfig,\n ::cuFuncSetSharedMemConfig"]
pub unsafe fn cudaFuncSetSharedMemConfig<T: ::cuda_libs::types::CudaAsPtr>(
    func: T,
    config: cudaSharedMemConfig,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaFuncSetSharedMemConfig(
            func.as_const_ptr() as *const ::std::os::raw::c_void,
            config,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Returns occupancy for a device function\n\n Returns in \\p *numBlocks the maximum number of active blocks per\n streaming multiprocessor for the device function.\n\n \\param numBlocks       - Returned occupancy\n \\param func            - Kernel function for which occupancy is calculated\n \\param blockSize       - Block size the kernel is intended to be launched with\n \\param dynamicSMemSize - Per-block dynamic shared memory usage intended, in bytes\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidDevice,\n ::cudaErrorInvalidDeviceFunction,\n ::cudaErrorInvalidValue,\n ::cudaErrorUnknown,\n \\notefnerr\n \\note_init_rt\n \\note_callback\n \\note_cudaKernel_t\n\n \\sa ::cudaOccupancyMaxActiveBlocksPerMultiprocessorWithFlags,\n \\ref ::cudaOccupancyMaxPotentialBlockSize(int*, int*, T, size_t, int) \"cudaOccupancyMaxPotentialBlockSize (C++ API)\",\n \\ref ::cudaOccupancyMaxPotentialBlockSizeWithFlags(int*, int*, T, size_t, int, unsigned int) \"cudaOccupancyMaxPotentialBlockSizeWithFlags (C++ API)\",\n \\ref ::cudaOccupancyMaxPotentialBlockSizeVariableSMem(int*, int*, T, UnaryFunction, int) \"cudaOccupancyMaxPotentialBlockSizeVariableSMem (C++ API)\",\n \\ref ::cudaOccupancyMaxPotentialBlockSizeVariableSMemWithFlags(int*, int*, T, UnaryFunction, int, unsigned int) \"cudaOccupancyMaxPotentialBlockSizeVariableSMemWithFlags (C++ API)\",\n \\ref ::cudaOccupancyAvailableDynamicSMemPerBlock(size_t*, T, int, int) \"cudaOccupancyAvailableDynamicSMemPerBlock (C++ API)\",\n ::cuOccupancyMaxActiveBlocksPerMultiprocessor"]
pub unsafe fn cudaOccupancyMaxActiveBlocksPerMultiprocessor<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
>(
    mut numBlocks: T,
    func: U,
    blockSize: ::std::os::raw::c_int,
    dynamicSMemSize: usize,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaOccupancyMaxActiveBlocksPerMultiprocessor(
            numBlocks.as_mut_ptr() as *mut ::std::os::raw::c_int,
            func.as_const_ptr() as *const ::std::os::raw::c_void,
            blockSize,
            dynamicSMemSize,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Returns dynamic shared memory available per block when launching \\p numBlocks blocks on SM.\n\n Returns in \\p *dynamicSmemSize the maximum size of dynamic shared memory to allow \\p numBlocks blocks per SM.\n\n \\param dynamicSmemSize - Returned maximum dynamic shared memory\n \\param func            - Kernel function for which occupancy is calculated\n \\param numBlocks       - Number of blocks to fit on SM\n \\param blockSize       - Size of the block\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidDevice,\n ::cudaErrorInvalidDeviceFunction,\n ::cudaErrorInvalidValue,\n ::cudaErrorUnknown,\n \\notefnerr\n \\note_init_rt\n \\note_callback\n \\note_cudaKernel_t\n\n \\sa ::cudaOccupancyMaxActiveBlocksPerMultiprocessorWithFlags,\n \\ref ::cudaOccupancyMaxPotentialBlockSize(int*, int*, T, size_t, int) \"cudaOccupancyMaxPotentialBlockSize (C++ API)\",\n \\ref ::cudaOccupancyMaxPotentialBlockSizeWithFlags(int*, int*, T, size_t, int, unsigned int) \"cudaOccupancyMaxPotentialBlockSizeWithFlags (C++ API)\",\n \\ref ::cudaOccupancyMaxPotentialBlockSizeVariableSMem(int*, int*, T, UnaryFunction, int) \"cudaOccupancyMaxPotentialBlockSizeVariableSMem (C++ API)\",\n \\ref ::cudaOccupancyMaxPotentialBlockSizeVariableSMemWithFlags(int*, int*, T, UnaryFunction, int, unsigned int) \"cudaOccupancyMaxPotentialBlockSizeVariableSMemWithFlags (C++ API)\",\n ::cudaOccupancyAvailableDynamicSMemPerBlock"]
pub unsafe fn cudaOccupancyAvailableDynamicSMemPerBlock<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
>(
    mut dynamicSmemSize: T,
    func: U,
    numBlocks: ::std::os::raw::c_int,
    blockSize: ::std::os::raw::c_int,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaOccupancyAvailableDynamicSMemPerBlock(
            dynamicSmemSize.as_mut_ptr() as *mut usize,
            func.as_const_ptr() as *const ::std::os::raw::c_void,
            numBlocks,
            blockSize,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Returns occupancy for a device function with the specified flags\n\n Returns in \\p *numBlocks the maximum number of active blocks per\n streaming multiprocessor for the device function.\n\n The \\p flags parameter controls how special cases are handled. Valid flags include:\n\n - ::cudaOccupancyDefault: keeps the default behavior as\n   ::cudaOccupancyMaxActiveBlocksPerMultiprocessor\n\n - ::cudaOccupancyDisableCachingOverride: This flag suppresses the default behavior\n   on platform where global caching affects occupancy. On such platforms, if caching\n   is enabled, but per-block SM resource usage would result in zero occupancy, the\n   occupancy calculator will calculate the occupancy as if caching is disabled.\n   Setting this flag makes the occupancy calculator to return 0 in such cases.\n   More information can be found about this feature in the \"Unified L1/Texture Cache\"\n   section of the Maxwell tuning guide.\n\n \\param numBlocks       - Returned occupancy\n \\param func            - Kernel function for which occupancy is calculated\n \\param blockSize       - Block size the kernel is intended to be launched with\n \\param dynamicSMemSize - Per-block dynamic shared memory usage intended, in bytes\n \\param flags           - Requested behavior for the occupancy calculator\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidDevice,\n ::cudaErrorInvalidDeviceFunction,\n ::cudaErrorInvalidValue,\n ::cudaErrorUnknown,\n \\notefnerr\n \\note_init_rt\n \\note_callback\n \\note_cudaKernel_t\n\n \\sa ::cudaOccupancyMaxActiveBlocksPerMultiprocessor,\n \\ref ::cudaOccupancyMaxPotentialBlockSize(int*, int*, T, size_t, int) \"cudaOccupancyMaxPotentialBlockSize (C++ API)\",\n \\ref ::cudaOccupancyMaxPotentialBlockSizeWithFlags(int*, int*, T, size_t, int, unsigned int) \"cudaOccupancyMaxPotentialBlockSizeWithFlags (C++ API)\",\n \\ref ::cudaOccupancyMaxPotentialBlockSizeVariableSMem(int*, int*, T, UnaryFunction, int) \"cudaOccupancyMaxPotentialBlockSizeVariableSMem (C++ API)\",\n \\ref ::cudaOccupancyMaxPotentialBlockSizeVariableSMemWithFlags(int*, int*, T, UnaryFunction, int, unsigned int) \"cudaOccupancyMaxPotentialBlockSizeVariableSMemWithFlags (C++ API)\",\n \\ref ::cudaOccupancyAvailableDynamicSMemPerBlock(size_t*, T, int, int) \"cudaOccupancyAvailableDynamicSMemPerBlock (C++ API)\",\n ::cuOccupancyMaxActiveBlocksPerMultiprocessorWithFlags"]
pub unsafe fn cudaOccupancyMaxActiveBlocksPerMultiprocessorWithFlags<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
>(
    mut numBlocks: T,
    func: U,
    blockSize: ::std::os::raw::c_int,
    dynamicSMemSize: usize,
    flags: ::std::os::raw::c_uint,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaOccupancyMaxActiveBlocksPerMultiprocessorWithFlags(
            numBlocks.as_mut_ptr() as *mut ::std::os::raw::c_int,
            func.as_const_ptr() as *const ::std::os::raw::c_void,
            blockSize,
            dynamicSMemSize,
            flags,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Given the kernel function (\\p func) and launch configuration\n (\\p config), return the maximum cluster size in \\p *clusterSize.\n\n The cluster dimensions in \\p config are ignored. If func has a required\n cluster size set (see ::cudaFuncGetAttributes),\\p *clusterSize will reflect\n the required cluster size.\n\n By default this function will always return a value that's portable on\n future hardware. A higher value may be returned if the kernel function\n allows non-portable cluster sizes.\n\n This function will respect the compile time launch bounds.\n\n \\param clusterSize - Returned maximum cluster size that can be launched\n                      for the given kernel function and launch configuration\n \\param func        - Kernel function for which maximum cluster\n                      size is calculated\n \\param config      - Launch configuration for the given kernel function\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidDeviceFunction,\n ::cudaErrorInvalidValue,\n ::cudaErrorUnknown,\n \\notefnerr\n \\note_init_rt\n \\note_callback\n \\note_cudaKernel_t\n\n \\sa ::cudaFuncGetAttributes\n \\ref ::cudaOccupancyMaxPotentialClusterSize(int*, T, const cudaLaunchConfig_t*) \"cudaOccupancyMaxPotentialClusterSize (C++ API)\",\n ::cuOccupancyMaxPotentialClusterSize"]
pub unsafe fn cudaOccupancyMaxPotentialClusterSize<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
>(
    mut clusterSize: T,
    func: U,
    launchConfig: V,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaOccupancyMaxPotentialClusterSize(
            clusterSize.as_mut_ptr() as *mut ::std::os::raw::c_int,
            func.as_const_ptr() as *const ::std::os::raw::c_void,
            launchConfig.as_const_ptr() as *const cudaLaunchConfig_t,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Given the kernel function (\\p func) and launch configuration\n (\\p config), return the maximum number of clusters that could co-exist\n on the target device in \\p *numClusters.\n\n If the function has required cluster size already set (see\n ::cudaFuncGetAttributes), the cluster size from config must either be\n unspecified or match the required size.\n Without required sizes, the cluster size must be specified in config,\n else the function will return an error.\n\n Note that various attributes of the kernel function may affect occupancy\n calculation. Runtime environment may affect how the hardware schedules\n the clusters, so the calculated occupancy is not guaranteed to be achievable.\n\n \\param numClusters - Returned maximum number of clusters that\n                      could co-exist on the target device\n \\param func        - Kernel function for which maximum number\n                      of clusters are calculated\n \\param config      - Launch configuration for the given kernel function\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidDeviceFunction,\n ::cudaErrorInvalidValue,\n ::cudaErrorInvalidClusterSize,\n ::cudaErrorUnknown,\n \\notefnerr\n \\note_init_rt\n \\note_callback\n \\note_cudaKernel_t\n\n \\sa\n ::cudaFuncGetAttributes\n \\ref ::cudaOccupancyMaxActiveClusters(int*, T, const cudaLaunchConfig_t*) \"cudaOccupancyMaxActiveClusters (C++ API)\",\n ::cuOccupancyMaxActiveClusters"]
pub unsafe fn cudaOccupancyMaxActiveClusters<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
>(
    mut numClusters: T,
    func: U,
    launchConfig: V,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaOccupancyMaxActiveClusters(
            numClusters.as_mut_ptr() as *mut ::std::os::raw::c_int,
            func.as_const_ptr() as *const ::std::os::raw::c_void,
            launchConfig.as_const_ptr() as *const cudaLaunchConfig_t,
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
    flags: ::std::os::raw::c_uint,
) -> Result<::cuda_libs::types::cuDeviceAllocation<T>, crate::sys::cudaError> {
    let mut dev_ptr = std::ptr::null_mut();
    let status =
        unsafe { crate::sys::cudaMallocManaged(&mut dev_ptr as *mut _ as *mut _, size, flags) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(::cuda_libs::types::cuDeviceAllocation(dev_ptr as *mut T))
    } else {
        Err(status)
    }
}
pub unsafe fn cudaMalloc<T>(
    size: usize,
) -> Result<::cuda_libs::types::cuDeviceAllocation<T>, crate::sys::cudaError> {
    let mut dev_ptr = std::ptr::null_mut();
    let status = unsafe { crate::sys::cudaMalloc(&mut dev_ptr as *mut _ as *mut _, size) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(::cuda_libs::types::cuDeviceAllocation(dev_ptr as *mut T))
    } else {
        Err(status)
    }
}
pub unsafe fn cudaMallocHost<T>(size: usize) -> Result<*mut T, crate::sys::cudaError> {
    let mut dev_ptr = std::ptr::null_mut();
    let status = unsafe { crate::sys::cudaMallocHost(&mut dev_ptr as *mut _ as *mut _, size) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(dev_ptr as *mut T)
    } else {
        Err(status)
    }
}
pub unsafe fn cudaMallocPitch<T>(
    pitch: *mut usize,
    width: usize,
    height: usize,
) -> Result<::cuda_libs::types::cuDeviceAllocation<T>, crate::sys::cudaError> {
    let mut dev_ptr = std::ptr::null_mut();
    let status = unsafe {
        crate::sys::cudaMallocPitch(&mut dev_ptr as *mut _ as *mut _, pitch, width, height)
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(::cuda_libs::types::cuDeviceAllocation(dev_ptr as *mut T))
    } else {
        Err(status)
    }
}
pub unsafe fn cudaMallocArray(
    desc: *const cudaChannelFormatDesc,
    width: usize,
    height: usize,
    flags: ::std::os::raw::c_uint,
) -> Result<cudaArray_t, crate::sys::cudaError> {
    let mut dev_ptr: cudaArray_t = std::mem::zeroed();
    let status = unsafe {
        crate::sys::cudaMallocArray(&mut dev_ptr as *mut _ as *mut _, desc, width, height, flags)
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(dev_ptr)
    } else {
        Err(status)
    }
}
pub unsafe fn cudaFree<T>(
    devPtr: ::cuda_libs::types::cuDeviceAllocation<T>,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaFree(devPtr.0 as *mut _) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudaFreeHost<T>(ptr: *mut T) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaFreeHost(ptr as *mut _) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudaFreeArray(array: cudaArray_t) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaFreeArray(array) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudaFreeMipmappedArray(
    mipmappedArray: cudaMipmappedArray_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaFreeMipmappedArray(mipmappedArray) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Allocates page-locked memory on the host\n\n Allocates \\p size bytes of host memory that is page-locked and accessible\n to the device. The driver tracks the virtual memory ranges allocated with\n this function and automatically accelerates calls to functions such as\n ::cudaMemcpy(). Since the memory can be accessed directly by the device, it\n can be read or written with much higher bandwidth than pageable memory\n obtained with functions such as ::malloc(). Allocating excessive amounts of\n pinned memory may degrade system performance, since it reduces the amount\n of memory available to the system for paging. As a result, this function is\n best used sparingly to allocate staging areas for data exchange between host\n and device.\n\n The \\p flags parameter enables different options to be specified that affect\n the allocation, as follows.\n - ::cudaHostAllocDefault: This flag's value is defined to be 0 and causes\n ::cudaHostAlloc() to emulate ::cudaMallocHost().\n - ::cudaHostAllocPortable: The memory returned by this call will be\n considered as pinned memory by all CUDA contexts, not just the one that\n performed the allocation.\n - ::cudaHostAllocMapped: Maps the allocation into the CUDA address space.\n The device pointer to the memory may be obtained by calling\n ::cudaHostGetDevicePointer().\n - ::cudaHostAllocWriteCombined: Allocates the memory as write-combined (WC).\n WC memory can be transferred across the PCI Express bus more quickly on some\n system configurations, but cannot be read efficiently by most CPUs.  WC\n memory is a good option for buffers that will be written by the CPU and read\n by the device via mapped pinned memory or host->device transfers.\n\n All of these flags are orthogonal to one another: a developer may allocate\n memory that is portable, mapped and/or write-combined with no restrictions.\n\n In order for the ::cudaHostAllocMapped flag to have any effect, the CUDA context\n must support the ::cudaDeviceMapHost flag, which can be checked via\n ::cudaGetDeviceFlags(). The ::cudaDeviceMapHost flag is implicitly set for\n contexts created via the runtime API.\n\n The ::cudaHostAllocMapped flag may be specified on CUDA contexts for devices\n that do not support mapped pinned memory. The failure is deferred to\n ::cudaHostGetDevicePointer() because the memory may be mapped into other\n CUDA contexts via the ::cudaHostAllocPortable flag.\n\n Memory allocated by this function must be freed with ::cudaFreeHost().\n\n \\param pHost - Device pointer to allocated memory\n \\param size  - Requested allocation size in bytes\n \\param flags - Requested properties of allocated memory\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorMemoryAllocation,\n ::cudaErrorExternalDevice\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaSetDeviceFlags,\n \\ref ::cudaMallocHost(void**, size_t) \"cudaMallocHost (C API)\",\n ::cudaFreeHost,\n ::cudaGetDeviceFlags,\n ::cuMemHostAlloc"]
pub unsafe fn cudaHostAlloc<T: ::cuda_libs::types::CudaAsPtr>(
    mut pHost: T,
    size: usize,
    flags: ::std::os::raw::c_uint,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaHostAlloc(
            pHost.as_mut_ptr() as *mut *mut ::std::os::raw::c_void,
            size,
            flags,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Registers an existing host memory range for use by CUDA\n\n Page-locks the memory range specified by \\p ptr and \\p size and maps it\n for the device(s) as specified by \\p flags. This memory range also is added\n to the same tracking mechanism as ::cudaHostAlloc() to automatically accelerate\n calls to functions such as ::cudaMemcpy(). Since the memory can be accessed\n directly by the device, it can be read or written with much higher bandwidth\n than pageable memory that has not been registered.  Page-locking excessive\n amounts of memory may degrade system performance, since it reduces the amount\n of memory available to the system for paging. As a result, this function is\n best used sparingly to register staging areas for data exchange between\n host and device.\n\n On systems where ::pageableMemoryAccessUsesHostPageTables is true, ::cudaHostRegister\n will not page-lock the memory range specified by \\p ptr but only populate\n unpopulated pages.\n\n ::cudaHostRegister is supported only on I/O coherent devices that have a non-zero\n value for the device attribute ::cudaDevAttrHostRegisterSupported.\n\n The \\p flags parameter enables different options to be specified that\n affect the allocation, as follows.\n\n - ::cudaHostRegisterDefault: On a system with unified virtual addressing,\n   the memory will be both mapped and portable.  On a system with no unified\n   virtual addressing, the memory will be neither mapped nor portable.\n\n - ::cudaHostRegisterPortable: The memory returned by this call will be\n   considered as pinned memory by all CUDA contexts, not just the one that\n   performed the allocation.\n\n - ::cudaHostRegisterMapped: Maps the allocation into the CUDA address\n   space. The device pointer to the memory may be obtained by calling\n   ::cudaHostGetDevicePointer().\n\n - ::cudaHostRegisterIoMemory: The passed memory pointer is treated as\n   pointing to some memory-mapped I/O space, e.g. belonging to a\n   third-party PCIe device, and it will marked as non cache-coherent and\n   contiguous.\n\n - ::cudaHostRegisterReadOnly: The passed memory pointer is treated as\n   pointing to memory that is considered read-only by the device.  On\n   platforms without ::cudaDevAttrPageableMemoryAccessUsesHostPageTables, this\n   flag is required in order to register memory mapped to the CPU as\n   read-only.  Support for the use of this flag can be queried from the device\n   attribute ::cudaDevAttrHostRegisterReadOnlySupported.  Using this flag with\n   a current context associated with a device that does not have this attribute\n   set will cause ::cudaHostRegister to error with cudaErrorNotSupported.\n\n All of these flags are orthogonal to one another: a developer may page-lock\n memory that is portable or mapped with no restrictions.\n\n The CUDA context must have been created with the ::cudaMapHost flag in\n order for the ::cudaHostRegisterMapped flag to have any effect.\n\n The ::cudaHostRegisterMapped flag may be specified on CUDA contexts for\n devices that do not support mapped pinned memory. The failure is deferred\n to ::cudaHostGetDevicePointer() because the memory may be mapped into\n other CUDA contexts via the ::cudaHostRegisterPortable flag.\n\n For devices that have a non-zero value for the device attribute\n ::cudaDevAttrCanUseHostPointerForRegisteredMem, the memory\n can also be accessed from the device using the host pointer \\p ptr.\n The device pointer returned by ::cudaHostGetDevicePointer() may or may not\n match the original host pointer \\p ptr and depends on the devices visible to the\n application. If all devices visible to the application have a non-zero value for the\n device attribute, the device pointer returned by ::cudaHostGetDevicePointer()\n will match the original pointer \\p ptr. If any device visible to the application\n has a zero value for the device attribute, the device pointer returned by\n ::cudaHostGetDevicePointer() will not match the original host pointer \\p ptr,\n but it will be suitable for use on all devices provided Unified Virtual Addressing\n is enabled. In such systems, it is valid to access the memory using either pointer\n on devices that have a non-zero value for the device attribute. Note however that\n such devices should access the memory using only of the two pointers and not both.\n\n The memory page-locked by this function must be unregistered with ::cudaHostUnregister().\n\n \\param ptr   - Host pointer to memory to page-lock\n \\param size  - Size in bytes of the address range to page-lock in bytes\n \\param flags - Flags for allocation request\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorMemoryAllocation,\n ::cudaErrorHostMemoryAlreadyRegistered,\n ::cudaErrorNotSupported,\n ::cudaErrorExternalDevice\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaHostUnregister, ::cudaHostGetFlags, ::cudaHostGetDevicePointer,\n ::cuMemHostRegister"]
pub unsafe fn cudaHostRegister<T: ::cuda_libs::types::CudaAsPtr>(
    mut ptr: T,
    size: usize,
    flags: ::std::os::raw::c_uint,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaHostRegister(ptr.as_mut_ptr() as *mut ::std::os::raw::c_void, size, flags)
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Unregisters a memory range that was registered with cudaHostRegister\n\n Unmaps the memory range whose base address is specified by \\p ptr, and makes\n it pageable again.\n\n The base address must be the same one specified to ::cudaHostRegister().\n\n \\param ptr - Host pointer to memory to unregister\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorHostMemoryNotRegistered\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaHostUnregister,\n ::cuMemHostUnregister"]
pub unsafe fn cudaHostUnregister<T: ::cuda_libs::types::CudaAsPtr>(
    mut ptr: T,
) -> Result<(), crate::sys::cudaError> {
    let status =
        unsafe { crate::sys::cudaHostUnregister(ptr.as_mut_ptr() as *mut ::std::os::raw::c_void) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Passes back device pointer of mapped host memory allocated by\n cudaHostAlloc or registered by cudaHostRegister\n\n Passes back the device pointer corresponding to the mapped, pinned host\n buffer allocated by ::cudaHostAlloc() or registered by ::cudaHostRegister().\n\n ::cudaHostGetDevicePointer() will fail if the ::cudaDeviceMapHost flag was\n not specified before deferred context creation occurred, or if called on a\n device that does not support mapped, pinned memory.\n\n For devices that have a non-zero value for the device attribute\n ::cudaDevAttrCanUseHostPointerForRegisteredMem, the memory\n can also be accessed from the device using the host pointer \\p pHost.\n The device pointer returned by ::cudaHostGetDevicePointer() may or may not\n match the original host pointer \\p pHost and depends on the devices visible to the\n application. If all devices visible to the application have a non-zero value for the\n device attribute, the device pointer returned by ::cudaHostGetDevicePointer()\n will match the original pointer \\p pHost. If any device visible to the application\n has a zero value for the device attribute, the device pointer returned by\n ::cudaHostGetDevicePointer() will not match the original host pointer \\p pHost,\n but it will be suitable for use on all devices provided Unified Virtual Addressing\n is enabled. In such systems, it is valid to access the memory using either pointer\n on devices that have a non-zero value for the device attribute. Note however that\n such devices should access the memory using only of the two pointers and not both.\n\n \\p flags provides for future releases.  For now, it must be set to 0.\n\n \\param pDevice - Returned device pointer for mapped memory\n \\param pHost   - Requested host pointer mapping\n \\param flags   - Flags for extensions (must be 0 for now)\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorMemoryAllocation\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaSetDeviceFlags, ::cudaHostAlloc,\n ::cuMemHostGetDevicePointer"]
pub unsafe fn cudaHostGetDevicePointer<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
>(
    mut pDevice: T,
    mut pHost: U,
    flags: ::std::os::raw::c_uint,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaHostGetDevicePointer(
            pDevice.as_mut_ptr() as *mut *mut ::std::os::raw::c_void,
            pHost.as_mut_ptr() as *mut ::std::os::raw::c_void,
            flags,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Passes back flags used to allocate pinned host memory allocated by\n cudaHostAlloc\n\n ::cudaHostGetFlags() will fail if the input pointer does not\n reside in an address range allocated by ::cudaHostAlloc().\n\n \\param pFlags - Returned flags word\n \\param pHost - Host pointer\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaHostAlloc,\n ::cuMemHostGetFlags"]
pub unsafe fn cudaHostGetFlags(
    pHost: *mut ::std::os::raw::c_void,
) -> Result<::std::os::raw::c_uint, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<::std::os::raw::c_uint> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaHostGetFlags(out_0.as_mut_ptr() as *mut _, pHost) };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok(out_0.assume_init()) }
    } else {
        Err(status)
    }
}
pub unsafe fn cudaMalloc3D(extent: cudaExtent) -> Result<cudaPitchedPtr, crate::sys::cudaError> {
    let mut dev_ptr: cudaPitchedPtr = std::mem::zeroed();
    let status = unsafe { crate::sys::cudaMalloc3D(&mut dev_ptr as *mut _ as *mut _, extent) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(dev_ptr)
    } else {
        Err(status)
    }
}
pub unsafe fn cudaMalloc3DArray(
    desc: *const cudaChannelFormatDesc,
    extent: cudaExtent,
    flags: ::std::os::raw::c_uint,
) -> Result<cudaArray_t, crate::sys::cudaError> {
    let mut dev_ptr: cudaArray_t = std::mem::zeroed();
    let status = unsafe {
        crate::sys::cudaMalloc3DArray(&mut dev_ptr as *mut _ as *mut _, desc, extent, flags)
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(dev_ptr)
    } else {
        Err(status)
    }
}
pub unsafe fn cudaMallocMipmappedArray(
    desc: *const cudaChannelFormatDesc,
    extent: cudaExtent,
    numLevels: ::std::os::raw::c_uint,
    flags: ::std::os::raw::c_uint,
) -> Result<cudaMipmappedArray_t, crate::sys::cudaError> {
    let mut dev_ptr: cudaMipmappedArray_t = std::mem::zeroed();
    let status = unsafe {
        crate::sys::cudaMallocMipmappedArray(
            &mut dev_ptr as *mut _ as *mut _,
            desc,
            extent,
            numLevels,
            flags,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(dev_ptr)
    } else {
        Err(status)
    }
}
#[doc = " \\brief Gets a mipmap level of a CUDA mipmapped array\n\n Returns in \\p *levelArray a CUDA array that represents a single mipmap level\n of the CUDA mipmapped array \\p mipmappedArray.\n\n If \\p level is greater than the maximum number of levels in this mipmapped array,\n ::cudaErrorInvalidValue is returned.\n\n If \\p mipmappedArray is NULL,\n ::cudaErrorInvalidResourceHandle is returned.\n\n \\param levelArray     - Returned mipmap level CUDA array\n \\param mipmappedArray - CUDA mipmapped array\n \\param level          - Mipmap level\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue\n ::cudaErrorInvalidResourceHandle\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaMalloc3D, ::cudaMalloc, ::cudaMallocPitch, ::cudaFree,\n ::cudaFreeArray,\n \\ref ::cudaMallocHost(void**, size_t) \"cudaMallocHost (C API)\",\n ::cudaFreeHost, ::cudaHostAlloc,\n ::make_cudaExtent,\n ::cuMipmappedArrayGetLevel"]
pub unsafe fn cudaGetMipmappedArrayLevel<T: ::cuda_libs::types::CudaAsPtr>(
    mut levelArray: T,
    mipmappedArray: cudaMipmappedArray_const_t,
    level: ::std::os::raw::c_uint,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGetMipmappedArrayLevel(
            levelArray.as_mut_ptr() as *mut cudaArray_t,
            mipmappedArray,
            level,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Copies data between 3D objects\n\n\\code\nstruct cudaExtent {\nsize_t width;\nsize_t height;\nsize_t depth;\n};\nstruct cudaExtent make_cudaExtent(size_t w, size_t h, size_t d);\n\nstruct cudaPos {\nsize_t x;\nsize_t y;\nsize_t z;\n};\nstruct cudaPos make_cudaPos(size_t x, size_t y, size_t z);\n\nstruct cudaMemcpy3DParms {\ncudaArray_t           srcArray;\nstruct cudaPos        srcPos;\nstruct cudaPitchedPtr srcPtr;\ncudaArray_t           dstArray;\nstruct cudaPos        dstPos;\nstruct cudaPitchedPtr dstPtr;\nstruct cudaExtent     extent;\nenum cudaMemcpyKind   kind;\n};\n\\endcode\n\n ::cudaMemcpy3D() copies data betwen two 3D objects. The source and\n destination objects may be in either host memory, device memory, or a CUDA\n array. The source, destination, extent, and kind of copy performed is\n specified by the ::cudaMemcpy3DParms struct which should be initialized to\n zero before use:\n\\code\ncudaMemcpy3DParms myParms = {0};\n\\endcode\n\n The struct passed to ::cudaMemcpy3D() must specify one of \\p srcArray or\n \\p srcPtr and one of \\p dstArray or \\p dstPtr. Passing more than one\n non-zero source or destination will cause ::cudaMemcpy3D() to return an\n error.\n\n The \\p srcPos and \\p dstPos fields are optional offsets into the source and\n destination objects and are defined in units of each object's elements. The\n element for a host or device pointer is assumed to be <b>unsigned char</b>.\n\n The \\p extent field defines the dimensions of the transferred area in\n elements. If a CUDA array is participating in the copy, the extent is\n defined in terms of that array's elements. If no CUDA array is\n participating in the copy then the extents are defined in elements of\n <b>unsigned char</b>.\n\n The \\p kind field defines the direction of the copy. It must be one of\n ::cudaMemcpyHostToHost, ::cudaMemcpyHostToDevice, ::cudaMemcpyDeviceToHost,\n ::cudaMemcpyDeviceToDevice, or ::cudaMemcpyDefault. Passing\n ::cudaMemcpyDefault is recommended, in which case the type of transfer is\n inferred from the pointer values. However, ::cudaMemcpyDefault is only\n allowed on systems that support unified virtual addressing.\n For ::cudaMemcpyHostToHost or ::cudaMemcpyHostToDevice or ::cudaMemcpyDeviceToHost\n passed as kind and cudaArray type passed as source or destination, if the kind\n implies cudaArray type to be present on the host, ::cudaMemcpy3D() will\n disregard that implication and silently correct the kind based on the fact that\n cudaArray type can only be present on the device.\n\n If the source and destination are both arrays, ::cudaMemcpy3D() will return\n an error if they do not have the same element size.\n\n The source and destination object may not overlap. If overlapping source\n and destination objects are specified, undefined behavior will result.\n\n The source object must entirely contain the region defined by \\p srcPos\n and \\p extent. The destination object must entirely contain the region\n defined by \\p dstPos and \\p extent.\n\n ::cudaMemcpy3D() returns an error if the pitch of \\p srcPtr or \\p dstPtr\n exceeds the maximum allowed. The pitch of a ::cudaPitchedPtr allocated\n with ::cudaMalloc3D() will always be valid.\n\n \\param p - 3D memory copy parameters\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorInvalidPitchValue,\n ::cudaErrorInvalidMemcpyDirection\n \\notefnerr\n \\note_sync\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaMalloc3D, ::cudaMalloc3DArray, ::cudaMemset3D, ::cudaMemcpy3DAsync,\n ::cudaMemcpy, ::cudaMemcpy2D,\n ::cudaMemcpy2DToArray, ::cudaMemcpy2DFromArray,\n ::cudaMemcpy2DArrayToArray, ::cudaMemcpyToSymbol,\n ::cudaMemcpyFromSymbol, ::cudaMemcpyAsync, ::cudaMemcpy2DAsync,\n ::cudaMemcpy2DToArrayAsync,\n ::cudaMemcpy2DFromArrayAsync,\n ::cudaMemcpyToSymbolAsync, ::cudaMemcpyFromSymbolAsync,\n ::make_cudaExtent, ::make_cudaPos,\n ::cuMemcpy3D"]
pub unsafe fn cudaMemcpy3D<T: ::cuda_libs::types::CudaAsPtr>(
    p: T,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaMemcpy3D(p.as_const_ptr() as *const cudaMemcpy3DParms) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Copies memory between devices\n\n Perform a 3D memory copy according to the parameters specified in\n \\p p.  See the definition of the ::cudaMemcpy3DPeerParms structure\n for documentation of its parameters.\n\n Note that this function is synchronous with respect to the host only if\n the source or destination of the transfer is host memory.  Note also\n that this copy is serialized with respect to all pending and future\n asynchronous work in to the current device, the copy's source device,\n and the copy's destination device (use ::cudaMemcpy3DPeerAsync to avoid\n this synchronization).\n\n \\param p - Parameters for the memory copy\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorInvalidDevice,\n ::cudaErrorInvalidPitchValue\n \\notefnerr\n \\note_sync\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaMemcpy, ::cudaMemcpyPeer, ::cudaMemcpyAsync, ::cudaMemcpyPeerAsync,\n ::cudaMemcpy3DPeerAsync,\n ::cuMemcpy3DPeer"]
pub unsafe fn cudaMemcpy3DPeer<T: ::cuda_libs::types::CudaAsPtr>(
    p: T,
) -> Result<(), crate::sys::cudaError> {
    let status =
        unsafe { crate::sys::cudaMemcpy3DPeer(p.as_const_ptr() as *const cudaMemcpy3DPeerParms) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Copies data between 3D objects\n\n\\code\nstruct cudaExtent {\nsize_t width;\nsize_t height;\nsize_t depth;\n};\nstruct cudaExtent make_cudaExtent(size_t w, size_t h, size_t d);\n\nstruct cudaPos {\nsize_t x;\nsize_t y;\nsize_t z;\n};\nstruct cudaPos make_cudaPos(size_t x, size_t y, size_t z);\n\nstruct cudaMemcpy3DParms {\ncudaArray_t           srcArray;\nstruct cudaPos        srcPos;\nstruct cudaPitchedPtr srcPtr;\ncudaArray_t           dstArray;\nstruct cudaPos        dstPos;\nstruct cudaPitchedPtr dstPtr;\nstruct cudaExtent     extent;\nenum cudaMemcpyKind   kind;\n};\n\\endcode\n\n ::cudaMemcpy3DAsync() copies data betwen two 3D objects. The source and\n destination objects may be in either host memory, device memory, or a CUDA\n array. The source, destination, extent, and kind of copy performed is\n specified by the ::cudaMemcpy3DParms struct which should be initialized to\n zero before use:\n\\code\ncudaMemcpy3DParms myParms = {0};\n\\endcode\n\n The struct passed to ::cudaMemcpy3DAsync() must specify one of \\p srcArray\n or \\p srcPtr and one of \\p dstArray or \\p dstPtr. Passing more than one\n non-zero source or destination will cause ::cudaMemcpy3DAsync() to return an\n error.\n\n The \\p srcPos and \\p dstPos fields are optional offsets into the source and\n destination objects and are defined in units of each object's elements. The\n element for a host or device pointer is assumed to be <b>unsigned char</b>.\n For CUDA arrays, positions must be in the range [0, 2048) for any\n dimension.\n\n The \\p extent field defines the dimensions of the transferred area in\n elements. If a CUDA array is participating in the copy, the extent is\n defined in terms of that array's elements. If no CUDA array is\n participating in the copy then the extents are defined in elements of\n <b>unsigned char</b>.\n\n The \\p kind field defines the direction of the copy. It must be one of\n ::cudaMemcpyHostToHost, ::cudaMemcpyHostToDevice, ::cudaMemcpyDeviceToHost,\n ::cudaMemcpyDeviceToDevice, or ::cudaMemcpyDefault. Passing\n ::cudaMemcpyDefault is recommended, in which case the type of transfer is\n inferred from the pointer values. However, ::cudaMemcpyDefault is only\n allowed on systems that support unified virtual addressing.\n For ::cudaMemcpyHostToHost or ::cudaMemcpyHostToDevice or ::cudaMemcpyDeviceToHost\n passed as kind and cudaArray type passed as source or destination, if the kind\n implies cudaArray type to be present on the host, ::cudaMemcpy3DAsync() will\n disregard that implication and silently correct the kind based on the fact that\n cudaArray type can only be present on the device.\n\n If the source and destination are both arrays, ::cudaMemcpy3DAsync() will\n return an error if they do not have the same element size.\n\n The source and destination object may not overlap. If overlapping source\n and destination objects are specified, undefined behavior will result.\n\n The source object must lie entirely within the region defined by \\p srcPos\n and \\p extent. The destination object must lie entirely within the region\n defined by \\p dstPos and \\p extent.\n\n ::cudaMemcpy3DAsync() returns an error if the pitch of \\p srcPtr or\n \\p dstPtr exceeds the maximum allowed. The pitch of a\n ::cudaPitchedPtr allocated with ::cudaMalloc3D() will always be valid.\n\n ::cudaMemcpy3DAsync() is asynchronous with respect to the host, so\n the call may return before the copy is complete. The copy can optionally\n be associated to a stream by passing a non-zero \\p stream argument. If\n \\p kind is ::cudaMemcpyHostToDevice or ::cudaMemcpyDeviceToHost and \\p stream\n is non-zero, the copy may overlap with operations in other streams.\n\n The device version of this function only handles device to device copies and\n cannot be given local or shared pointers.\n\n \\param p      - 3D memory copy parameters\n \\param stream - Stream identifier\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorInvalidPitchValue,\n ::cudaErrorInvalidMemcpyDirection\n \\notefnerr\n \\note_async\n \\note_null_stream\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaMalloc3D, ::cudaMalloc3DArray, ::cudaMemset3D, ::cudaMemcpy3D,\n ::cudaMemcpy, ::cudaMemcpy2D,\n ::cudaMemcpy2DToArray, :::cudaMemcpy2DFromArray,\n ::cudaMemcpy2DArrayToArray, ::cudaMemcpyToSymbol,\n ::cudaMemcpyFromSymbol, ::cudaMemcpyAsync, ::cudaMemcpy2DAsync,\n ::cudaMemcpy2DToArrayAsync,\n ::cudaMemcpy2DFromArrayAsync,\n ::cudaMemcpyToSymbolAsync, ::cudaMemcpyFromSymbolAsync,\n ::make_cudaExtent, ::make_cudaPos,\n ::cuMemcpy3DAsync"]
pub unsafe fn cudaMemcpy3DAsync<T: ::cuda_libs::types::CudaAsPtr>(
    p: T,
    stream: cudaStream_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaMemcpy3DAsync(p.as_const_ptr() as *const cudaMemcpy3DParms, stream)
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Copies memory between devices asynchronously.\n\n Perform a 3D memory copy according to the parameters specified in\n \\p p.  See the definition of the ::cudaMemcpy3DPeerParms structure\n for documentation of its parameters.\n\n \\param p      - Parameters for the memory copy\n \\param stream - Stream identifier\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorInvalidDevice,\n ::cudaErrorInvalidPitchValue\n \\notefnerr\n \\note_async\n \\note_null_stream\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaMemcpy, ::cudaMemcpyPeer, ::cudaMemcpyAsync, ::cudaMemcpyPeerAsync,\n ::cudaMemcpy3DPeerAsync,\n ::cuMemcpy3DPeerAsync"]
pub unsafe fn cudaMemcpy3DPeerAsync<T: ::cuda_libs::types::CudaAsPtr>(
    p: T,
    stream: cudaStream_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaMemcpy3DPeerAsync(p.as_const_ptr() as *const cudaMemcpy3DPeerParms, stream)
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Gets free and total device memory\n\n Returns in \\p *total the total amount of memory available to the the current context.\n Returns in \\p *free the amount of memory on the device that is free according to the OS.\n CUDA is not guaranteed to be able to allocate all of the memory that the OS reports as free.\n In a multi-tenet situation, free estimate returned is prone to race condition where\n a new allocation/free done by a different process or a different thread in the same\n process between the time when free memory was estimated and reported, will result in\n deviation in free value reported and actual free memory.\n\n The integrated GPU on Tegra shares memory with CPU and other component\n of the SoC. The free and total values returned by the API excludes\n the SWAP memory space maintained by the OS on some platforms.\n The OS may move some of the memory pages into swap area as the GPU or\n CPU allocate or access memory. See Tegra app note on how to calculate\n total and free memory on Tegra.\n\n \\param free  - Returned free memory in bytes\n \\param total - Returned total memory in bytes\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorLaunchFailure\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa\n ::cuMemGetInfo"]
pub unsafe fn cudaMemGetInfo() -> Result<(usize, usize), crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let mut out_1: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudaMemGetInfo(out_0.as_mut_ptr() as *mut _, out_1.as_mut_ptr() as *mut _)
    };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok((out_0.assume_init(), out_1.assume_init())) }
    } else {
        Err(status)
    }
}
#[doc = " \\brief Gets info about the specified cudaArray\n\n Returns in \\p *desc, \\p *extent and \\p *flags respectively, the type, shape\n and flags of \\p array.\n\n Any of \\p *desc, \\p *extent and \\p *flags may be specified as NULL.\n\n \\param desc   - Returned array type\n \\param extent - Returned array shape. 2D arrays will have depth of zero\n \\param flags  - Returned array flags\n \\param array  - The ::cudaArray to get info for\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa\n ::cuArrayGetDescriptor,\n ::cuArray3DGetDescriptor"]
pub unsafe fn cudaArrayGetInfo(
    array: cudaArray_t,
) -> Result<(cudaChannelFormatDesc, cudaExtent, ::std::os::raw::c_uint), crate::sys::cudaError> {
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
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe {
            Ok((
                out_0.assume_init(),
                out_1.assume_init(),
                out_2.assume_init(),
            ))
        }
    } else {
        Err(status)
    }
}
#[doc = " \\brief Gets a CUDA array plane from a CUDA array\n\n Returns in \\p pPlaneArray a CUDA array that represents a single format plane\n of the CUDA array \\p hArray.\n\n If \\p planeIdx is greater than the maximum number of planes in this array or if the array does\n not have a multi-planar format e.g: ::cudaChannelFormatKindNV12, then ::cudaErrorInvalidValue is returned.\n\n Note that if the \\p hArray has format ::cudaChannelFormatKindNV12, then passing in 0 for \\p planeIdx returns\n a CUDA array of the same size as \\p hArray but with one 8-bit channel and ::cudaChannelFormatKindUnsigned as its format kind.\n If 1 is passed for \\p planeIdx, then the returned CUDA array has half the height and width\n of \\p hArray with two 8-bit channels and ::cudaChannelFormatKindUnsigned as its format kind.\n\n \\param pPlaneArray   - Returned CUDA array referenced by the \\p planeIdx\n \\param hArray        - CUDA array\n \\param planeIdx      - Plane index\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue\n ::cudaErrorInvalidResourceHandle\n \\notefnerr\n\n \\sa\n ::cuArrayGetPlane"]
pub unsafe fn cudaArrayGetPlane<T: ::cuda_libs::types::CudaAsPtr>(
    mut pPlaneArray: T,
    hArray: cudaArray_t,
    planeIdx: ::std::os::raw::c_uint,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaArrayGetPlane(
            pPlaneArray.as_mut_ptr() as *mut cudaArray_t,
            hArray,
            planeIdx,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Returns the memory requirements of a CUDA array\n\n Returns the memory requirements of a CUDA array in \\p memoryRequirements\n If the CUDA array is not allocated with flag ::cudaArrayDeferredMapping\n ::cudaErrorInvalidValue will be returned.\n\n The returned value in ::cudaArrayMemoryRequirements::size\n represents the total size of the CUDA array.\n The returned value in ::cudaArrayMemoryRequirements::alignment\n represents the alignment necessary for mapping the CUDA array.\n\n \\return\n ::cudaSuccess\n ::cudaErrorInvalidValue\n\n \\param[out] memoryRequirements - Pointer to ::cudaArrayMemoryRequirements\n \\param[in] array - CUDA array to get the memory requirements of\n \\param[in] device - Device to get the memory requirements for\n \\sa ::cudaMipmappedArrayGetMemoryRequirements"]
pub unsafe fn cudaArrayGetMemoryRequirements(
    array: cudaArray_t,
    device: ::std::os::raw::c_int,
) -> Result<cudaArrayMemoryRequirements, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<cudaArrayMemoryRequirements> =
        std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudaArrayGetMemoryRequirements(out_0.as_mut_ptr() as *mut _, array, device)
    };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok(out_0.assume_init()) }
    } else {
        Err(status)
    }
}
#[doc = " \\brief Returns the memory requirements of a CUDA mipmapped array\n\n Returns the memory requirements of a CUDA mipmapped array in \\p memoryRequirements\n If the CUDA mipmapped array is not allocated with flag ::cudaArrayDeferredMapping\n ::cudaErrorInvalidValue will be returned.\n\n The returned value in ::cudaArrayMemoryRequirements::size\n represents the total size of the CUDA mipmapped array.\n The returned value in ::cudaArrayMemoryRequirements::alignment\n represents the alignment necessary for mapping the CUDA mipmapped\n array.\n\n \\return\n ::cudaSuccess\n ::cudaErrorInvalidValue\n\n \\param[out] memoryRequirements - Pointer to ::cudaArrayMemoryRequirements\n \\param[in] mipmap - CUDA mipmapped array to get the memory requirements of\n \\param[in] device - Device to get the memory requirements for\n \\sa ::cudaArrayGetMemoryRequirements"]
pub unsafe fn cudaMipmappedArrayGetMemoryRequirements(
    mipmap: cudaMipmappedArray_t,
    device: ::std::os::raw::c_int,
) -> Result<cudaArrayMemoryRequirements, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<cudaArrayMemoryRequirements> =
        std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudaMipmappedArrayGetMemoryRequirements(
            out_0.as_mut_ptr() as *mut _,
            mipmap,
            device,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok(out_0.assume_init()) }
    } else {
        Err(status)
    }
}
pub unsafe fn cudaArrayGetSparseProperties(
    array: cudaArray_t,
) -> Result<cudaArraySparseProperties, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<cudaArraySparseProperties> =
        std::mem::MaybeUninit::uninit();
    let status =
        unsafe { crate::sys::cudaArrayGetSparseProperties(out_0.as_mut_ptr() as *mut _, array) };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok(out_0.assume_init()) }
    } else {
        Err(status)
    }
}
pub unsafe fn cudaMipmappedArrayGetSparseProperties(
    mipmap: cudaMipmappedArray_t,
) -> Result<cudaArraySparseProperties, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<cudaArraySparseProperties> =
        std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudaMipmappedArrayGetSparseProperties(out_0.as_mut_ptr() as *mut _, mipmap)
    };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok(out_0.assume_init()) }
    } else {
        Err(status)
    }
}
#[doc = " \\brief Copies data between host and device\n\n Copies \\p count bytes from the memory area pointed to by \\p src to the\n memory area pointed to by \\p dst, where \\p kind specifies the direction\n of the copy, and must be one of ::cudaMemcpyHostToHost,\n ::cudaMemcpyHostToDevice, ::cudaMemcpyDeviceToHost,\n ::cudaMemcpyDeviceToDevice, or ::cudaMemcpyDefault. Passing\n ::cudaMemcpyDefault is recommended, in which case the type of transfer is\n inferred from the pointer values. However, ::cudaMemcpyDefault is only\n allowed on systems that support unified virtual addressing. Calling\n ::cudaMemcpy() with dst and src pointers that do not match the direction of\n the copy results in an undefined behavior.\n\n \\param dst   - Destination memory address\n \\param src   - Source memory address\n \\param count - Size in bytes to copy\n \\param kind  - Type of transfer\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorInvalidMemcpyDirection\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\note_sync\n \\note_memcpy\n\n \\sa ::cudaMemcpy2D,\n ::cudaMemcpy2DToArray, ::cudaMemcpy2DFromArray,\n ::cudaMemcpy2DArrayToArray, ::cudaMemcpyToSymbol,\n ::cudaMemcpyFromSymbol, ::cudaMemcpyAsync, ::cudaMemcpy2DAsync,\n ::cudaMemcpy2DToArrayAsync,\n ::cudaMemcpy2DFromArrayAsync,\n ::cudaMemcpyToSymbolAsync, ::cudaMemcpyFromSymbolAsync,\n ::cuMemcpyDtoH,\n ::cuMemcpyHtoD,\n ::cuMemcpyDtoD,\n ::cuMemcpy"]
pub unsafe fn cudaMemcpy<T: ::cuda_libs::types::CudaAsPtr, U: ::cuda_libs::types::CudaAsPtr>(
    mut dst: T,
    src: U,
    count: usize,
    kind: cudaMemcpyKind,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaMemcpy(
            dst.as_mut_ptr() as *mut ::std::os::raw::c_void,
            src.as_const_ptr() as *const ::std::os::raw::c_void,
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
#[doc = " \\brief Copies memory between two devices\n\n Copies memory from one device to memory on another device.  \\p dst is the\n base device pointer of the destination memory and \\p dstDevice is the\n destination device.  \\p src is the base device pointer of the source memory\n and \\p srcDevice is the source device.  \\p count specifies the number of bytes\n to copy.\n\n Note that this function is asynchronous with respect to the host, but\n serialized with respect all pending and future asynchronous work in to the\n current device, \\p srcDevice, and \\p dstDevice (use ::cudaMemcpyPeerAsync\n to avoid this synchronization).\n\n \\param dst       - Destination device pointer\n \\param dstDevice - Destination device\n \\param src       - Source device pointer\n \\param srcDevice - Source device\n \\param count     - Size of memory copy in bytes\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorInvalidDevice\n \\notefnerr\n \\note_sync\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaMemcpy, ::cudaMemcpyAsync, ::cudaMemcpyPeerAsync,\n ::cudaMemcpy3DPeerAsync,\n ::cuMemcpyPeer"]
pub unsafe fn cudaMemcpyPeer<T: ::cuda_libs::types::CudaAsPtr, U: ::cuda_libs::types::CudaAsPtr>(
    mut dst: T,
    dstDevice: ::std::os::raw::c_int,
    src: U,
    srcDevice: ::std::os::raw::c_int,
    count: usize,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaMemcpyPeer(
            dst.as_mut_ptr() as *mut ::std::os::raw::c_void,
            dstDevice,
            src.as_const_ptr() as *const ::std::os::raw::c_void,
            srcDevice,
            count,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Copies data between host and device\n\n Copies a matrix (\\p height rows of \\p width bytes each) from the memory\n area pointed to by \\p src to the memory area pointed to by \\p dst, where\n \\p kind specifies the direction of the copy, and must be one of\n ::cudaMemcpyHostToHost, ::cudaMemcpyHostToDevice, ::cudaMemcpyDeviceToHost,\n ::cudaMemcpyDeviceToDevice, or ::cudaMemcpyDefault. Passing\n ::cudaMemcpyDefault is recommended, in which case the type of transfer is\n inferred from the pointer values. However, ::cudaMemcpyDefault is only\n allowed on systems that support unified virtual addressing. \\p dpitch and\n \\p spitch are the widths in memory in bytes of the 2D arrays pointed to by\n \\p dst and \\p src, including any padding added to the end of each row. The\n memory areas may not overlap. \\p width must not exceed either \\p dpitch or\n \\p spitch. Calling ::cudaMemcpy2D() with \\p dst and \\p src pointers that do\n not match the direction of the copy results in an undefined behavior.\n ::cudaMemcpy2D() returns an error if \\p dpitch or \\p spitch exceeds\n the maximum allowed.\n\n \\param dst    - Destination memory address\n \\param dpitch - Pitch of destination memory\n \\param src    - Source memory address\n \\param spitch - Pitch of source memory\n \\param width  - Width of matrix transfer (columns in bytes)\n \\param height - Height of matrix transfer (rows)\n \\param kind   - Type of transfer\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorInvalidPitchValue,\n ::cudaErrorInvalidMemcpyDirection\n \\notefnerr\n \\note_init_rt\n \\note_callback\n \\note_memcpy\n\n \\sa ::cudaMemcpy,\n ::cudaMemcpy2DToArray, ::cudaMemcpy2DFromArray,\n ::cudaMemcpy2DArrayToArray, ::cudaMemcpyToSymbol,\n ::cudaMemcpyFromSymbol, ::cudaMemcpyAsync, ::cudaMemcpy2DAsync,\n ::cudaMemcpy2DToArrayAsync,\n ::cudaMemcpy2DFromArrayAsync,\n ::cudaMemcpyToSymbolAsync, ::cudaMemcpyFromSymbolAsync,\n ::cuMemcpy2D,\n ::cuMemcpy2DUnaligned"]
pub unsafe fn cudaMemcpy2D<T: ::cuda_libs::types::CudaAsPtr, U: ::cuda_libs::types::CudaAsPtr>(
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
            dst.as_mut_ptr() as *mut ::std::os::raw::c_void,
            dpitch,
            src.as_const_ptr() as *const ::std::os::raw::c_void,
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
#[doc = " \\brief Copies data between host and device\n\n Copies a matrix (\\p height rows of \\p width bytes each) from the memory\n area pointed to by \\p src to the CUDA array \\p dst starting at\n \\p hOffset rows and \\p wOffset bytes from the upper left corner,\n where \\p kind specifies the direction of the copy, and must be one\n of ::cudaMemcpyHostToHost, ::cudaMemcpyHostToDevice, ::cudaMemcpyDeviceToHost,\n ::cudaMemcpyDeviceToDevice, or ::cudaMemcpyDefault. Passing\n ::cudaMemcpyDefault is recommended, in which case the type of transfer is\n inferred from the pointer values. However, ::cudaMemcpyDefault is only\n allowed on systems that support unified virtual addressing.\n \\p spitch is the width in memory in bytes of the 2D array pointed to by\n \\p src, including any padding added to the end of each row. \\p wOffset +\n \\p width must not exceed the width of the CUDA array \\p dst. \\p width must\n not exceed \\p spitch. ::cudaMemcpy2DToArray() returns an error if \\p spitch\n exceeds the maximum allowed.\n\n \\param dst     - Destination memory address\n \\param wOffset - Destination starting X offset (columns in bytes)\n \\param hOffset - Destination starting Y offset (rows)\n \\param src     - Source memory address\n \\param spitch  - Pitch of source memory\n \\param width   - Width of matrix transfer (columns in bytes)\n \\param height  - Height of matrix transfer (rows)\n \\param kind    - Type of transfer\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorInvalidPitchValue,\n ::cudaErrorInvalidMemcpyDirection\n \\notefnerr\n \\note_sync\n \\note_init_rt\n \\note_callback\n \\note_memcpy\n\n \\sa ::cudaMemcpy, ::cudaMemcpy2D,\n ::cudaMemcpy2DFromArray,\n ::cudaMemcpy2DArrayToArray, ::cudaMemcpyToSymbol,\n ::cudaMemcpyFromSymbol, ::cudaMemcpyAsync, ::cudaMemcpy2DAsync,\n ::cudaMemcpy2DToArrayAsync,\n ::cudaMemcpy2DFromArrayAsync,\n ::cudaMemcpyToSymbolAsync, ::cudaMemcpyFromSymbolAsync,\n ::cuMemcpy2D,\n ::cuMemcpy2DUnaligned"]
pub unsafe fn cudaMemcpy2DToArray<T: ::cuda_libs::types::CudaAsPtr>(
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
            src.as_const_ptr() as *const ::std::os::raw::c_void,
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
#[doc = " \\brief Copies data between host and device\n\n Copies a matrix (\\p height rows of \\p width bytes each) from the CUDA\n array \\p src starting at \\p hOffset rows and \\p wOffset bytes from the\n upper left corner to the memory area pointed to by \\p dst, where\n \\p kind specifies the direction of the copy, and must be one of\n ::cudaMemcpyHostToHost, ::cudaMemcpyHostToDevice, ::cudaMemcpyDeviceToHost,\n ::cudaMemcpyDeviceToDevice, or ::cudaMemcpyDefault. Passing\n ::cudaMemcpyDefault is recommended, in which case the type of transfer is\n inferred from the pointer values. However, ::cudaMemcpyDefault is only\n allowed on systems that support unified virtual addressing. \\p dpitch is the\n width in memory in bytes of the 2D array pointed to by \\p dst, including any\n padding added to the end of each row. \\p wOffset + \\p width must not exceed\n the width of the CUDA array \\p src. \\p width must not exceed \\p dpitch.\n ::cudaMemcpy2DFromArray() returns an error if \\p dpitch exceeds the maximum\n allowed.\n\n \\param dst     - Destination memory address\n \\param dpitch  - Pitch of destination memory\n \\param src     - Source memory address\n \\param wOffset - Source starting X offset (columns in bytes)\n \\param hOffset - Source starting Y offset (rows)\n \\param width   - Width of matrix transfer (columns in bytes)\n \\param height  - Height of matrix transfer (rows)\n \\param kind    - Type of transfer\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorInvalidPitchValue,\n ::cudaErrorInvalidMemcpyDirection\n \\notefnerr\n \\note_sync\n \\note_init_rt\n \\note_callback\n \\note_memcpy\n\n \\sa ::cudaMemcpy, ::cudaMemcpy2D,\n ::cudaMemcpy2DToArray,\n ::cudaMemcpy2DArrayToArray, ::cudaMemcpyToSymbol,\n ::cudaMemcpyFromSymbol, ::cudaMemcpyAsync, ::cudaMemcpy2DAsync,\n ::cudaMemcpy2DToArrayAsync,\n ::cudaMemcpy2DFromArrayAsync,\n ::cudaMemcpyToSymbolAsync, ::cudaMemcpyFromSymbolAsync,\n ::cuMemcpy2D,\n ::cuMemcpy2DUnaligned"]
pub unsafe fn cudaMemcpy2DFromArray<T: ::cuda_libs::types::CudaAsPtr>(
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
            dst.as_mut_ptr() as *mut ::std::os::raw::c_void,
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
#[doc = " \\brief Copies data between host and device\n\n Copies a matrix (\\p height rows of \\p width bytes each) from the CUDA\n array \\p src starting at \\p hOffsetSrc rows and \\p wOffsetSrc bytes from the\n upper left corner to the CUDA array \\p dst starting at \\p hOffsetDst rows\n and \\p wOffsetDst bytes from the upper left corner, where \\p kind\n specifies the direction of the copy, and must be one of\n ::cudaMemcpyHostToHost, ::cudaMemcpyHostToDevice, ::cudaMemcpyDeviceToHost,\n ::cudaMemcpyDeviceToDevice, or ::cudaMemcpyDefault. Passing\n ::cudaMemcpyDefault is recommended, in which case the type of transfer is\n inferred from the pointer values. However, ::cudaMemcpyDefault is only\n allowed on systems that support unified virtual addressing.\n \\p wOffsetDst + \\p width must not exceed the width of the CUDA array \\p dst.\n \\p wOffsetSrc + \\p width must not exceed the width of the CUDA array \\p src.\n\n \\param dst        - Destination memory address\n \\param wOffsetDst - Destination starting X offset (columns in bytes)\n \\param hOffsetDst - Destination starting Y offset (rows)\n \\param src        - Source memory address\n \\param wOffsetSrc - Source starting X offset (columns in bytes)\n \\param hOffsetSrc - Source starting Y offset (rows)\n \\param width      - Width of matrix transfer (columns in bytes)\n \\param height     - Height of matrix transfer (rows)\n \\param kind       - Type of transfer\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorInvalidMemcpyDirection\n \\notefnerr\n \\note_sync\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaMemcpy, ::cudaMemcpy2D,\n ::cudaMemcpy2DToArray, ::cudaMemcpy2DFromArray,\n ::cudaMemcpyToSymbol,\n ::cudaMemcpyFromSymbol, ::cudaMemcpyAsync, ::cudaMemcpy2DAsync,\n ::cudaMemcpy2DToArrayAsync,\n ::cudaMemcpy2DFromArrayAsync,\n ::cudaMemcpyToSymbolAsync, ::cudaMemcpyFromSymbolAsync,\n ::cuMemcpy2D,\n ::cuMemcpy2DUnaligned"]
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
#[doc = " \\brief Copies data to the given symbol on the device\n\n Copies \\p count bytes from the memory area pointed to by \\p src\n to the memory area pointed to by \\p offset bytes from the start of symbol\n \\p symbol. The memory areas may not overlap. \\p symbol is a variable that\n resides in global or constant memory space. \\p kind can be either\n ::cudaMemcpyHostToDevice, ::cudaMemcpyDeviceToDevice, or ::cudaMemcpyDefault.\n Passing ::cudaMemcpyDefault is recommended, in which case the type of\n transfer is inferred from the pointer values. However, ::cudaMemcpyDefault\n is only allowed on systems that support unified virtual addressing.\n\n \\param symbol - Device symbol address\n \\param src    - Source memory address\n \\param count  - Size in bytes to copy\n \\param offset - Offset from start of symbol in bytes\n \\param kind   - Type of transfer\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorInvalidSymbol,\n ::cudaErrorInvalidMemcpyDirection,\n ::cudaErrorNoKernelImageForDevice\n \\notefnerr\n \\note_sync\n \\note_string_api_deprecation\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaMemcpy, ::cudaMemcpy2D,\n ::cudaMemcpy2DToArray,  ::cudaMemcpy2DFromArray,\n ::cudaMemcpy2DArrayToArray,\n ::cudaMemcpyFromSymbol, ::cudaMemcpyAsync, ::cudaMemcpy2DAsync,\n ::cudaMemcpy2DToArrayAsync,\n ::cudaMemcpy2DFromArrayAsync,\n ::cudaMemcpyToSymbolAsync, ::cudaMemcpyFromSymbolAsync,\n ::cuMemcpy,\n ::cuMemcpyHtoD,\n ::cuMemcpyDtoD"]
pub unsafe fn cudaMemcpyToSymbol<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
>(
    symbol: T,
    src: U,
    count: usize,
    offset: usize,
    kind: cudaMemcpyKind,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaMemcpyToSymbol(
            symbol.as_const_ptr() as *const ::std::os::raw::c_void,
            src.as_const_ptr() as *const ::std::os::raw::c_void,
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
#[doc = " \\brief Copies data from the given symbol on the device\n\n Copies \\p count bytes from the memory area pointed to by \\p offset bytes\n from the start of symbol \\p symbol to the memory area pointed to by \\p dst.\n The memory areas may not overlap. \\p symbol is a variable that\n resides in global or constant memory space. \\p kind can be either\n ::cudaMemcpyDeviceToHost, ::cudaMemcpyDeviceToDevice, or ::cudaMemcpyDefault.\n Passing ::cudaMemcpyDefault is recommended, in which case the type of\n transfer is inferred from the pointer values. However, ::cudaMemcpyDefault\n is only allowed on systems that support unified virtual addressing.\n\n \\param dst    - Destination memory address\n \\param symbol - Device symbol address\n \\param count  - Size in bytes to copy\n \\param offset - Offset from start of symbol in bytes\n \\param kind   - Type of transfer\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorInvalidSymbol,\n ::cudaErrorInvalidMemcpyDirection,\n ::cudaErrorNoKernelImageForDevice\n \\notefnerr\n \\note_sync\n \\note_string_api_deprecation\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaMemcpy, ::cudaMemcpy2D,\n ::cudaMemcpy2DToArray, ::cudaMemcpy2DFromArray,\n ::cudaMemcpy2DArrayToArray, ::cudaMemcpyToSymbol,\n ::cudaMemcpyAsync, ::cudaMemcpy2DAsync,\n ::cudaMemcpy2DToArrayAsync,\n ::cudaMemcpy2DFromArrayAsync,\n ::cudaMemcpyToSymbolAsync, ::cudaMemcpyFromSymbolAsync,\n ::cuMemcpy,\n ::cuMemcpyDtoH,\n ::cuMemcpyDtoD"]
pub unsafe fn cudaMemcpyFromSymbol<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
>(
    mut dst: T,
    symbol: U,
    count: usize,
    offset: usize,
    kind: cudaMemcpyKind,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaMemcpyFromSymbol(
            dst.as_mut_ptr() as *mut ::std::os::raw::c_void,
            symbol.as_const_ptr() as *const ::std::os::raw::c_void,
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
#[doc = " \\brief Copies data between host and device\n\n Copies \\p count bytes from the memory area pointed to by \\p src to the\n memory area pointed to by \\p dst, where \\p kind specifies the\n direction of the copy, and must be one of ::cudaMemcpyHostToHost,\n ::cudaMemcpyHostToDevice, ::cudaMemcpyDeviceToHost,\n ::cudaMemcpyDeviceToDevice, or ::cudaMemcpyDefault. Passing\n ::cudaMemcpyDefault is recommended, in which case the type of transfer is\n inferred from the pointer values. However, ::cudaMemcpyDefault is only\n allowed on systems that support unified virtual addressing.\n\n The memory areas may not overlap. Calling ::cudaMemcpyAsync() with \\p dst and\n \\p src pointers that do not match the direction of the copy results in an\n undefined behavior.\n\n ::cudaMemcpyAsync() is asynchronous with respect to the host, so the call\n may return before the copy is complete. The copy can optionally be\n associated to a stream by passing a non-zero \\p stream argument. If \\p kind\n is ::cudaMemcpyHostToDevice or ::cudaMemcpyDeviceToHost and the \\p stream is\n non-zero, the copy may overlap with operations in other streams.\n\n The device version of this function only handles device to device copies and\n cannot be given local or shared pointers.\n\n \\param dst    - Destination memory address\n \\param src    - Source memory address\n \\param count  - Size in bytes to copy\n \\param kind   - Type of transfer\n \\param stream - Stream identifier\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorInvalidMemcpyDirection\n \\notefnerr\n \\note_async\n \\note_null_stream\n \\note_init_rt\n \\note_callback\n \\note_memcpy\n\n \\sa ::cudaMemcpy, ::cudaMemcpy2D,\n ::cudaMemcpy2DToArray, ::cudaMemcpy2DFromArray,\n ::cudaMemcpy2DArrayToArray, ::cudaMemcpyToSymbol,\n ::cudaMemcpyFromSymbol, ::cudaMemcpy2DAsync,\n ::cudaMemcpy2DToArrayAsync,\n ::cudaMemcpy2DFromArrayAsync,\n ::cudaMemcpyToSymbolAsync, ::cudaMemcpyFromSymbolAsync,\n ::cuMemcpyAsync,\n ::cuMemcpyDtoHAsync,\n ::cuMemcpyHtoDAsync,\n ::cuMemcpyDtoDAsync"]
pub unsafe fn cudaMemcpyAsync<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
>(
    mut dst: T,
    src: U,
    count: usize,
    kind: cudaMemcpyKind,
    stream: cudaStream_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaMemcpyAsync(
            dst.as_mut_ptr() as *mut ::std::os::raw::c_void,
            src.as_const_ptr() as *const ::std::os::raw::c_void,
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
#[doc = " \\brief Copies memory between two devices asynchronously.\n\n Copies memory from one device to memory on another device.  \\p dst is the\n base device pointer of the destination memory and \\p dstDevice is the\n destination device.  \\p src is the base device pointer of the source memory\n and \\p srcDevice is the source device.  \\p count specifies the number of bytes\n to copy.\n\n Note that this function is asynchronous with respect to the host and all work\n on other devices.\n\n \\param dst       - Destination device pointer\n \\param dstDevice - Destination device\n \\param src       - Source device pointer\n \\param srcDevice - Source device\n \\param count     - Size of memory copy in bytes\n \\param stream    - Stream identifier\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorInvalidDevice\n \\notefnerr\n \\note_async\n \\note_null_stream\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaMemcpy, ::cudaMemcpyPeer, ::cudaMemcpyAsync,\n ::cudaMemcpy3DPeerAsync,\n ::cuMemcpyPeerAsync"]
pub unsafe fn cudaMemcpyPeerAsync<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
>(
    mut dst: T,
    dstDevice: ::std::os::raw::c_int,
    src: U,
    srcDevice: ::std::os::raw::c_int,
    count: usize,
    stream: cudaStream_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaMemcpyPeerAsync(
            dst.as_mut_ptr() as *mut ::std::os::raw::c_void,
            dstDevice,
            src.as_const_ptr() as *const ::std::os::raw::c_void,
            srcDevice,
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
#[doc = " \\brief Performs a batch of memory copies asynchronously.\n\n Performs a batch of memory copies. The batch as a whole executes in stream order but copies within a\n batch are not guaranteed to execute in any specific order. This API only supports pointer-to-pointer copies.\n For copies involving CUDA arrays, please see ::cudaMemcpy3DBatchAsync.\n\n Performs memory copies from source buffers specified in \\p srcs to destination buffers specified in \\p dsts.\n The size of each copy is specified in \\p sizes. All three arrays must be of the same length as specified\n by \\p count. Since there are no ordering guarantees for copies within a batch, specifying any dependent copies\n within a batch will result in undefined behavior.\n\n Every copy in the batch has to be associated with a set of attributes specified in the \\p attrs array.\n Each entry in this array can apply to more than one copy. This can be done by specifying in the \\p attrsIdxs array,\n the index of the first copy that the corresponding entry in the \\p attrs array applies to. Both \\p attrs and\n \\p attrsIdxs must be of the same length as specified by \\p numAttrs. For example, if a batch has 10 copies listed\n in dst/src/sizes, the first 6 of which have one set of attributes and the remaining 4 another, then \\p numAttrs\n will be 2, \\p attrsIdxs will be {0, 6} and \\p attrs will contains the two sets of attributes. Note that the first entry\n in \\p attrsIdxs must always be 0. Also, each entry must be greater than the previous entry and the last entry should be\n less than \\p count. Furthermore, \\p numAttrs must be lesser than or equal to \\p count.\n\n The ::cudaMemcpyAttributes::srcAccessOrder indicates the source access ordering to be observed for copies associated\n with the attribute. If the source access order is set to ::cudaMemcpySrcAccessOrderStream, then the source will\n be accessed in stream order. If the source access order is set to ::cudaMemcpySrcAccessOrderDuringApiCall then\n it indicates that access to the source pointer can be out of stream order and all accesses must be complete before the\n API call returns. This flag is suited for ephemeral sources (ex., stack variables) when it's known that no prior\n operations in the stream can be accessing the memory and also that the lifetime of the memory is limited to the scope\n that the source variable was declared in. Specifying this flag allows the driver to optimize the copy and removes the\n need for the user to synchronize the stream after the API call. If the source access order is set to\n ::cudaMemcpySrcAccessOrderAny then it indicates that access to the source pointer can be out of stream order and the\n accesses can happen even after the API call returns. This flag is suited for host pointers allocated\n outside CUDA (ex., via malloc) when it's known that no prior operations in the stream can be accessing the memory.\n Specifying this flag allows the driver to optimize the copy on certain platforms. Each memcpy operation in the batch\n must have a valid ::cudaMemcpyAttributes corresponding to it including the appropriate srcAccessOrder setting,\n otherwise the API will return ::cudaErrorInvalidValue.\n\n The ::cudaMemcpyAttributes::srcLocHint and ::cudaMemcpyAttributes::dstLocHint allows applications to specify hint locations\n for operands of a copy when the operand doesn't have a fixed location. That is, these hints are\n only applicable for managed memory pointers on devices where ::cudaDevAttrConcurrentManagedAccess is true or\n system-allocated pageable memory on devices where ::cudaDevAttrPageableMemoryAccess is true.\n For other cases, these hints are ignored.\n\n The ::cudaMemcpyAttributes::flags field can be used to specify certain flags for copies. Setting the\n ::cudaMemcpyFlagPreferOverlapWithCompute flag indicates that the associated copies should preferably overlap with\n any compute work. Note that this flag is a hint and can be ignored depending on the platform and other parameters of the copy.\n\n\n \\param dsts          - Array of destination pointers.\n \\param srcs          - Array of memcpy source pointers.\n \\param sizes         - Array of sizes for memcpy operations.\n \\param count         - Size of \\p dsts, \\p srcs and \\p sizes arrays\n \\param attrs         - Array of memcpy attributes.\n \\param attrsIdxs     - Array of indices to specify which copies each entry in the \\p attrs array applies to.\n                        The attributes specified in attrs[k] will be applied to copies starting from attrsIdxs[k]\n                        through attrsIdxs[k+1] - 1. Also attrs[numAttrs-1] will apply to copies starting from\n                        attrsIdxs[numAttrs-1] through count - 1.\n \\param numAttrs      - Size of \\p attrs and \\p attrsIdxs arrays.\n \\param hStream       - The stream to enqueue the operations in. Must not be legacy NULL stream.\n\n \\return\n ::cudaSuccess\n ::cudaErrorInvalidValue\n \\notefnerr\n \\note_async\n \\note_memcpy"]
pub unsafe fn cudaMemcpyBatchAsync<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
    W: ::cuda_libs::types::CudaAsPtr,
    X: ::cuda_libs::types::CudaAsPtr,
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
            dsts.as_const_ptr() as *const *mut ::std::os::raw::c_void,
            srcs.as_const_ptr() as *const *const ::std::os::raw::c_void,
            sizes.as_const_ptr() as *const usize,
            count,
            attrs.as_mut_ptr() as *mut cudaMemcpyAttributes,
            attrsIdxs.as_mut_ptr() as *mut usize,
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
#[doc = " \\brief Performs a batch of 3D memory copies asynchronously.\n\n Performs a batch of memory copies. The batch as a whole executes in stream order but copies within a\n batch are not guaranteed to execute in any specific order. Note that this means specifying any dependent\n copies within a batch will result in undefined behavior.\n\n Performs memory copies as specified in the \\p opList array. The length of this array is specified in \\p numOps.\n Each entry in this array describes a copy operation. This includes among other things, the source and destination\n operands for the copy as specified in ::cudaMemcpy3DBatchOp::src and ::cudaMemcpy3DBatchOp::dst respectively.\n The source and destination operands of a copy can either be a pointer or a CUDA array. The width, height and depth\n of a copy is specified in ::cudaMemcpy3DBatchOp::extent. The width, height and depth of a copy are specified in\n elements and must not be zero. For pointer-to-pointer copies, the element size is considered to be 1. For pointer\n to CUDA array or vice versa copies, the element size is determined by the CUDA array. For CUDA array to CUDA array copies,\n the element size of the two CUDA arrays must match.\n\n For a given operand, if ::cudaMemcpy3DOperand::type is specified as ::cudaMemcpyOperandTypePointer, then\n ::cudaMemcpy3DOperand::op::ptr will be used. The ::cudaMemcpy3DOperand::op::ptr::ptr field must contain the pointer where\n the copy should begin. The ::cudaMemcpy3DOperand::op::ptr::rowLength field specifies the length of each row in elements and\n must either be zero or be greater than or equal to the width of the copy specified in ::cudaMemcpy3DBatchOp::extent::width.\n The ::cudaMemcpy3DOperand::op::ptr::layerHeight field specifies the height of each layer and must either be\n zero or be greater than or equal to the height of the copy specified in ::cudaMemcpy3DBatchOp::extent::height.\n When either of these values is zero, that aspect of the operand is considered to be tightly packed according to the copy extent.\n For managed memory pointers on devices where ::cudaDevAttrConcurrentManagedAccess is true or system-allocated pageable memory\n on devices where ::cudaDevAttrPageableMemoryAccess is true, the ::cudaMemcpy3DOperand::op::ptr::locHint field can be used to hint\n the location of the operand.\n\n If an operand's type is specified as ::cudaMemcpyOperandTypeArray, then ::cudaMemcpy3DOperand::op::array will be used.\n The ::cudaMemcpy3DOperand::op::array::array field specifies the CUDA array and ::cudaMemcpy3DOperand::op::array::offset specifies\n the 3D offset into that array where the copy begins.\n\n The ::cudaMemcpyAttributes::srcAccessOrder indicates the source access ordering to be observed for copies associated\n with the attribute. If the source access order is set to ::cudaMemcpySrcAccessOrderStream, then the source will\n be accessed in stream order. If the source access order is set to ::cudaMemcpySrcAccessOrderDuringApiCall then\n it indicates that access to the source pointer can be out of stream order and all accesses must be complete before the\n API call returns. This flag is suited for ephemeral sources (ex., stack variables) when it's known that no prior\n operations in the stream can be accessing the memory and also that the lifetime of the memory is limited to the scope\n that the source variable was declared in. Specifying this flag allows the driver to optimize the copy and removes the\n need for the user to synchronize the stream after the API call. If the source access order is set to\n ::cudaMemcpySrcAccessOrderAny then it indicates that access to the source pointer can be out of stream order and the\n accesses can happen even after the API call returns. This flag is suited for host pointers allocated\n outside CUDA (ex., via malloc) when it's known that no prior operations in the stream can be accessing the memory.\n Specifying this flag allows the driver to optimize the copy on certain platforms. Each memcopy operation in \\p opList\n must have a valid srcAccessOrder setting, otherwise this API will return ::cudaErrorInvalidValue.\n\n The ::cudaMemcpyAttributes::flags field can be used to specify certain flags for copies. Setting the\n ::cudaMemcpyFlagPreferOverlapWithCompute flag indicates that the associated copies should preferably overlap with\n any compute work. Note that this flag is a hint and can be ignored depending on the platform and other parameters of the copy.\n\n\n \\param numOps     - Total number of memcpy operations.\n \\param opList     - Array of size \\p numOps containing the actual memcpy operations.\n \\param flags      - Flags for future use, must be zero now.\n \\param hStream    - The stream to enqueue the operations in. Must not be default NULL stream.\n\n \\return\n ::cudaSuccess\n ::cudaErrorInvalidValue\n \\notefnerr\n \\note_async\n \\note_memcpy"]
pub unsafe fn cudaMemcpy3DBatchAsync<T: ::cuda_libs::types::CudaAsPtr>(
    numOps: usize,
    mut opList: T,
    flags: ::std::os::raw::c_ulonglong,
    stream: cudaStream_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaMemcpy3DBatchAsync(
            numOps,
            opList.as_mut_ptr() as *mut cudaMemcpy3DBatchOp,
            flags,
            stream,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " Performs asynchronous memory copy operation with the specified attributes.\n\n Performs asynchronous memory copy operation where \\p dst and \\p src are\n the destination and source pointers respectively. \\p size specifies\n the number of bytes to copy. \\p attr specifies the attributes for the copy and\n \\p hStream specifies the stream to enqueue the operation in.\n\n For more information regarding the attributes, please refer to ::cudaMemcpyAttributes and it's usage desciption in::cudaMemcpyBatchAsync\n\n \\param dst - Destination device pointer\n \\param src - Source device pointer\n \\param size - Number of bytes to copy\n \\param attr - Attributes for the copy\n \\param hStream - Stream to enqueue the operation in\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue\n \\notefnerr\n \\note_async\n \\note_memcpy\n\n \\sa ::cudaMemcpyBatchAsync"]
pub unsafe fn cudaMemcpyWithAttributesAsync<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
>(
    mut dst: T,
    src: U,
    size: usize,
    mut attr: V,
    stream: cudaStream_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaMemcpyWithAttributesAsync(
            dst.as_mut_ptr() as *mut ::std::os::raw::c_void,
            src.as_const_ptr() as *const ::std::os::raw::c_void,
            size,
            attr.as_mut_ptr() as *mut cudaMemcpyAttributes,
            stream,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " Performs 3D asynchronous memory copy with the specified attributes.\n\n Performs the copy operation specified in \\p op.\n \\p flags specifies the flags for the copy and \\p hStream specifies the stream to enqueue the operation in.\n\n For more information regarding the operation, please refer to ::cudaMemcpy3DBatchOp and it's usage desciption in::cudaMemcpy3DBatchAsync\n\n \\param op - Operation to perform\n \\param flags - Flags for the copy, must be zero now.\n \\param hStream - Stream to enqueue the operation in\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue\n \\notefnerr\n \\note_async\n \\note_memcpy\n\n \\sa ::cudaMemcpy3DBatchAsync"]
pub unsafe fn cudaMemcpy3DWithAttributesAsync<T: ::cuda_libs::types::CudaAsPtr>(
    mut op: T,
    flags: ::std::os::raw::c_ulonglong,
    stream: cudaStream_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaMemcpy3DWithAttributesAsync(
            op.as_mut_ptr() as *mut cudaMemcpy3DBatchOp,
            flags,
            stream,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Copies data between host and device\n\n Copies a matrix (\\p height rows of \\p width bytes each) from the memory\n area pointed to by \\p src to the memory area pointed to by \\p dst, where\n \\p kind specifies the direction of the copy, and must be one of\n ::cudaMemcpyHostToHost, ::cudaMemcpyHostToDevice, ::cudaMemcpyDeviceToHost,\n ::cudaMemcpyDeviceToDevice, or ::cudaMemcpyDefault. Passing\n ::cudaMemcpyDefault is recommended, in which case the type of transfer is\n inferred from the pointer values. However, ::cudaMemcpyDefault is only\n allowed on systems that support unified virtual addressing.\n \\p dpitch and \\p spitch are the widths in memory in bytes of the 2D arrays\n pointed to by \\p dst and \\p src, including any padding added to the end of\n each row. The memory areas may not overlap. \\p width must not exceed either\n \\p dpitch or \\p spitch.\n\n Calling ::cudaMemcpy2DAsync() with \\p dst and \\p src pointers that do not\n match the direction of the copy results in an undefined behavior.\n ::cudaMemcpy2DAsync() returns an error if \\p dpitch or \\p spitch is greater\n than the maximum allowed.\n\n ::cudaMemcpy2DAsync() is asynchronous with respect to the host, so\n the call may return before the copy is complete. The copy can optionally\n be associated to a stream by passing a non-zero \\p stream argument. If\n \\p kind is ::cudaMemcpyHostToDevice or ::cudaMemcpyDeviceToHost and\n \\p stream is non-zero, the copy may overlap with operations in other\n streams.\n\n The device version of this function only handles device to device copies and\n cannot be given local or shared pointers.\n\n \\param dst    - Destination memory address\n \\param dpitch - Pitch of destination memory\n \\param src    - Source memory address\n \\param spitch - Pitch of source memory\n \\param width  - Width of matrix transfer (columns in bytes)\n \\param height - Height of matrix transfer (rows)\n \\param kind   - Type of transfer\n \\param stream - Stream identifier\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorInvalidPitchValue,\n ::cudaErrorInvalidMemcpyDirection\n \\notefnerr\n \\note_async\n \\note_null_stream\n \\note_init_rt\n \\note_callback\n \\note_memcpy\n\n \\sa ::cudaMemcpy, ::cudaMemcpy2D,\n ::cudaMemcpy2DToArray, ::cudaMemcpy2DFromArray,\n ::cudaMemcpy2DArrayToArray, ::cudaMemcpyToSymbol,\n ::cudaMemcpyFromSymbol, ::cudaMemcpyAsync,\n ::cudaMemcpy2DToArrayAsync,\n ::cudaMemcpy2DFromArrayAsync,\n ::cudaMemcpyToSymbolAsync, ::cudaMemcpyFromSymbolAsync,\n ::cuMemcpy2DAsync"]
pub unsafe fn cudaMemcpy2DAsync<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
>(
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
            dst.as_mut_ptr() as *mut ::std::os::raw::c_void,
            dpitch,
            src.as_const_ptr() as *const ::std::os::raw::c_void,
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
#[doc = " \\brief Copies data between host and device\n\n Copies a matrix (\\p height rows of \\p width bytes each) from the memory\n area pointed to by \\p src to the CUDA array \\p dst starting at \\p hOffset\n rows and \\p wOffset bytes from the upper left corner, where \\p kind specifies\n the direction of the copy, and must be one of ::cudaMemcpyHostToHost,\n ::cudaMemcpyHostToDevice, ::cudaMemcpyDeviceToHost,\n ::cudaMemcpyDeviceToDevice, or ::cudaMemcpyDefault. Passing\n ::cudaMemcpyDefault is recommended, in which case the type of transfer is\n inferred from the pointer values. However, ::cudaMemcpyDefault is only\n allowed on systems that support unified virtual addressing.\n \\p spitch is the width in memory in bytes of the 2D array pointed to by\n \\p src, including any padding added to the end of each row. \\p wOffset +\n \\p width must not exceed the width of the CUDA array \\p dst. \\p width must\n not exceed \\p spitch. ::cudaMemcpy2DToArrayAsync() returns an error if\n \\p spitch exceeds the maximum allowed.\n\n ::cudaMemcpy2DToArrayAsync() is asynchronous with respect to the host, so\n the call may return before the copy is complete. The copy can optionally\n be associated to a stream by passing a non-zero \\p stream argument. If\n \\p kind is ::cudaMemcpyHostToDevice or ::cudaMemcpyDeviceToHost and\n \\p stream is non-zero, the copy may overlap with operations in other\n streams.\n\n \\param dst     - Destination memory address\n \\param wOffset - Destination starting X offset (columns in bytes)\n \\param hOffset - Destination starting Y offset (rows)\n \\param src     - Source memory address\n \\param spitch  - Pitch of source memory\n \\param width   - Width of matrix transfer (columns in bytes)\n \\param height  - Height of matrix transfer (rows)\n \\param kind    - Type of transfer\n \\param stream  - Stream identifier\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorInvalidPitchValue,\n ::cudaErrorInvalidMemcpyDirection\n \\notefnerr\n \\note_async\n \\note_null_stream\n \\note_init_rt\n \\note_callback\n \\note_memcpy\n\n \\sa ::cudaMemcpy, ::cudaMemcpy2D,\n ::cudaMemcpy2DToArray, ::cudaMemcpy2DFromArray,\n ::cudaMemcpy2DArrayToArray, ::cudaMemcpyToSymbol,\n ::cudaMemcpyFromSymbol, ::cudaMemcpyAsync, ::cudaMemcpy2DAsync,\n\n ::cudaMemcpy2DFromArrayAsync,\n ::cudaMemcpyToSymbolAsync, ::cudaMemcpyFromSymbolAsync,\n ::cuMemcpy2DAsync"]
pub unsafe fn cudaMemcpy2DToArrayAsync<T: ::cuda_libs::types::CudaAsPtr>(
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
            src.as_const_ptr() as *const ::std::os::raw::c_void,
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
#[doc = " \\brief Copies data between host and device\n\n Copies a matrix (\\p height rows of \\p width bytes each) from the CUDA\n array \\p src starting at \\p hOffset rows and \\p wOffset bytes from the\n upper left corner to the memory area pointed to by \\p dst,\n where \\p kind specifies the direction of the copy, and must be one of\n ::cudaMemcpyHostToHost, ::cudaMemcpyHostToDevice, ::cudaMemcpyDeviceToHost,\n ::cudaMemcpyDeviceToDevice, or ::cudaMemcpyDefault. Passing\n ::cudaMemcpyDefault is recommended, in which case the type of transfer is\n inferred from the pointer values. However, ::cudaMemcpyDefault is only\n allowed on systems that support unified virtual addressing.\n \\p dpitch is the width in memory in bytes of the 2D\n array pointed to by \\p dst, including any padding added to the end of each\n row. \\p wOffset + \\p width must not exceed the width of the CUDA array\n \\p src. \\p width must not exceed \\p dpitch. ::cudaMemcpy2DFromArrayAsync()\n returns an error if \\p dpitch exceeds the maximum allowed.\n\n ::cudaMemcpy2DFromArrayAsync() is asynchronous with respect to the host, so\n the call may return before the copy is complete. The copy can optionally be\n associated to a stream by passing a non-zero \\p stream argument. If \\p kind\n is ::cudaMemcpyHostToDevice or ::cudaMemcpyDeviceToHost and \\p stream is\n non-zero, the copy may overlap with operations in other streams.\n\n \\param dst     - Destination memory address\n \\param dpitch  - Pitch of destination memory\n \\param src     - Source memory address\n \\param wOffset - Source starting X offset (columns in bytes)\n \\param hOffset - Source starting Y offset (rows)\n \\param width   - Width of matrix transfer (columns in bytes)\n \\param height  - Height of matrix transfer (rows)\n \\param kind    - Type of transfer\n \\param stream  - Stream identifier\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorInvalidPitchValue,\n ::cudaErrorInvalidMemcpyDirection\n \\notefnerr\n \\note_async\n \\note_null_stream\n \\note_init_rt\n \\note_callback\n \\note_memcpy\n\n \\sa ::cudaMemcpy, ::cudaMemcpy2D,\n ::cudaMemcpy2DToArray, ::cudaMemcpy2DFromArray,\n ::cudaMemcpy2DArrayToArray, ::cudaMemcpyToSymbol,\n ::cudaMemcpyFromSymbol, ::cudaMemcpyAsync, ::cudaMemcpy2DAsync,\n ::cudaMemcpy2DToArrayAsync,\n\n ::cudaMemcpyToSymbolAsync, ::cudaMemcpyFromSymbolAsync,\n ::cuMemcpy2DAsync"]
pub unsafe fn cudaMemcpy2DFromArrayAsync<T: ::cuda_libs::types::CudaAsPtr>(
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
            dst.as_mut_ptr() as *mut ::std::os::raw::c_void,
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
#[doc = " \\brief Copies data to the given symbol on the device\n\n Copies \\p count bytes from the memory area pointed to by \\p src\n to the memory area pointed to by \\p offset bytes from the start of symbol\n \\p symbol. The memory areas may not overlap. \\p symbol is a variable that\n resides in global or constant memory space. \\p kind can be either\n ::cudaMemcpyHostToDevice, ::cudaMemcpyDeviceToDevice, or ::cudaMemcpyDefault.\n Passing ::cudaMemcpyDefault is recommended, in which case the type of transfer\n is inferred from the pointer values. However, ::cudaMemcpyDefault is only\n allowed on systems that support unified virtual addressing.\n\n ::cudaMemcpyToSymbolAsync() is asynchronous with respect to the host, so\n the call may return before the copy is complete. The copy can optionally\n be associated to a stream by passing a non-zero \\p stream argument. If\n \\p kind is ::cudaMemcpyHostToDevice and \\p stream is non-zero, the copy\n may overlap with operations in other streams.\n\n \\param symbol - Device symbol address\n \\param src    - Source memory address\n \\param count  - Size in bytes to copy\n \\param offset - Offset from start of symbol in bytes\n \\param kind   - Type of transfer\n \\param stream - Stream identifier\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorInvalidSymbol,\n ::cudaErrorInvalidMemcpyDirection,\n ::cudaErrorNoKernelImageForDevice\n \\notefnerr\n \\note_async\n \\note_null_stream\n \\note_string_api_deprecation\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaMemcpy, ::cudaMemcpy2D,\n ::cudaMemcpy2DToArray, ::cudaMemcpy2DFromArray,\n ::cudaMemcpy2DArrayToArray, ::cudaMemcpyToSymbol,\n ::cudaMemcpyFromSymbol, ::cudaMemcpyAsync, ::cudaMemcpy2DAsync,\n ::cudaMemcpy2DToArrayAsync,\n ::cudaMemcpy2DFromArrayAsync,\n ::cudaMemcpyFromSymbolAsync,\n ::cuMemcpyAsync,\n ::cuMemcpyHtoDAsync,\n ::cuMemcpyDtoDAsync"]
pub unsafe fn cudaMemcpyToSymbolAsync<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
>(
    symbol: T,
    src: U,
    count: usize,
    offset: usize,
    kind: cudaMemcpyKind,
    stream: cudaStream_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaMemcpyToSymbolAsync(
            symbol.as_const_ptr() as *const ::std::os::raw::c_void,
            src.as_const_ptr() as *const ::std::os::raw::c_void,
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
#[doc = " \\brief Copies data from the given symbol on the device\n\n Copies \\p count bytes from the memory area pointed to by \\p offset bytes\n from the start of symbol \\p symbol to the memory area pointed to by \\p dst.\n The memory areas may not overlap. \\p symbol is a variable that resides in\n global or constant memory space. \\p kind can be either\n ::cudaMemcpyDeviceToHost, ::cudaMemcpyDeviceToDevice, or ::cudaMemcpyDefault.\n Passing ::cudaMemcpyDefault is recommended, in which case the type of transfer\n is inferred from the pointer values. However, ::cudaMemcpyDefault is only\n allowed on systems that support unified virtual addressing.\n\n ::cudaMemcpyFromSymbolAsync() is asynchronous with respect to the host, so\n the call may return before the copy is complete. The copy can optionally be\n associated to a stream by passing a non-zero \\p stream argument. If \\p kind\n is ::cudaMemcpyDeviceToHost and \\p stream is non-zero, the copy may overlap\n with operations in other streams.\n\n \\param dst    - Destination memory address\n \\param symbol - Device symbol address\n \\param count  - Size in bytes to copy\n \\param offset - Offset from start of symbol in bytes\n \\param kind   - Type of transfer\n \\param stream - Stream identifier\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorInvalidSymbol,\n ::cudaErrorInvalidMemcpyDirection,\n ::cudaErrorNoKernelImageForDevice\n \\notefnerr\n \\note_async\n \\note_null_stream\n \\note_string_api_deprecation\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaMemcpy, ::cudaMemcpy2D,\n ::cudaMemcpy2DToArray, ::cudaMemcpy2DFromArray,\n ::cudaMemcpy2DArrayToArray, ::cudaMemcpyToSymbol,\n ::cudaMemcpyFromSymbol, ::cudaMemcpyAsync, ::cudaMemcpy2DAsync,\n ::cudaMemcpy2DToArrayAsync,\n ::cudaMemcpy2DFromArrayAsync,\n ::cudaMemcpyToSymbolAsync,\n ::cuMemcpyAsync,\n ::cuMemcpyDtoHAsync,\n ::cuMemcpyDtoDAsync"]
pub unsafe fn cudaMemcpyFromSymbolAsync<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
>(
    mut dst: T,
    symbol: U,
    count: usize,
    offset: usize,
    kind: cudaMemcpyKind,
    stream: cudaStream_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaMemcpyFromSymbolAsync(
            dst.as_mut_ptr() as *mut ::std::os::raw::c_void,
            symbol.as_const_ptr() as *const ::std::os::raw::c_void,
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
#[doc = " \\brief Initializes or sets device memory to a value\n\n Fills the first \\p count bytes of the memory area pointed to by \\p devPtr\n with the constant byte value \\p value.\n\n Note that this function is asynchronous with respect to the host unless\n \\p devPtr refers to pinned host memory.\n\n \\param devPtr - Pointer to device memory\n \\param value  - Value to set for each byte of specified memory\n \\param count  - Size in bytes to set\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n \\notefnerr\n \\note_memset\n \\note_init_rt\n \\note_callback\n\n \\sa\n ::cuMemsetD8,\n ::cuMemsetD16,\n ::cuMemsetD32"]
pub unsafe fn cudaMemset<T: ::cuda_libs::types::CudaAsPtr>(
    mut devPtr: T,
    value: ::std::os::raw::c_int,
    count: usize,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaMemset(
            devPtr.as_mut_ptr() as *mut ::std::os::raw::c_void,
            value,
            count,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Initializes or sets device memory to a value\n\n Sets to the specified value \\p value a matrix (\\p height rows of \\p width\n bytes each) pointed to by \\p dstPtr. \\p pitch is the width in bytes of the\n 2D array pointed to by \\p dstPtr, including any padding added to the end\n of each row. This function performs fastest when the pitch is one that has\n been passed back by ::cudaMallocPitch().\n\n Note that this function is asynchronous with respect to the host unless\n \\p devPtr refers to pinned host memory.\n\n \\param devPtr - Pointer to 2D device memory\n \\param pitch  - Pitch in bytes of 2D device memory(Unused if \\p height is 1)\n \\param value  - Value to set for each byte of specified memory\n \\param width  - Width of matrix set (columns in bytes)\n \\param height - Height of matrix set (rows)\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n \\notefnerr\n \\note_memset\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaMemset, ::cudaMemset3D, ::cudaMemsetAsync,\n ::cudaMemset2DAsync, ::cudaMemset3DAsync,\n ::cuMemsetD2D8,\n ::cuMemsetD2D16,\n ::cuMemsetD2D32"]
pub unsafe fn cudaMemset2D<T: ::cuda_libs::types::CudaAsPtr>(
    mut devPtr: T,
    pitch: usize,
    value: ::std::os::raw::c_int,
    width: usize,
    height: usize,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaMemset2D(
            devPtr.as_mut_ptr() as *mut ::std::os::raw::c_void,
            pitch,
            value,
            width,
            height,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Initializes or sets device memory to a value\n\n Initializes each element of a 3D array to the specified value \\p value.\n The object to initialize is defined by \\p pitchedDevPtr. The \\p pitch field\n of \\p pitchedDevPtr is the width in memory in bytes of the 3D array pointed\n to by \\p pitchedDevPtr, including any padding added to the end of each row.\n The \\p xsize field specifies the logical width of each row in bytes, while\n the \\p ysize field specifies the height of each 2D slice in rows.\n The \\p pitch field of \\p pitchedDevPtr is ignored when \\p height and \\p depth\n are both equal to 1.\n\n The extents of the initialized region are specified as a \\p width in bytes,\n a \\p height in rows, and a \\p depth in slices.\n\n Extents with \\p width greater than or equal to the \\p xsize of\n \\p pitchedDevPtr may perform significantly faster than extents narrower\n than the \\p xsize. Secondarily, extents with \\p height equal to the\n \\p ysize of \\p pitchedDevPtr will perform faster than when the \\p height is\n shorter than the \\p ysize.\n\n This function performs fastest when the \\p pitchedDevPtr has been allocated\n by ::cudaMalloc3D().\n\n Note that this function is asynchronous with respect to the host unless\n \\p pitchedDevPtr refers to pinned host memory.\n\n \\param pitchedDevPtr - Pointer to pitched device memory\n \\param value         - Value to set for each byte of specified memory\n \\param extent        - Size parameters for where to set device memory (\\p width field in bytes)\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n \\notefnerr\n \\note_memset\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaMemset, ::cudaMemset2D,\n ::cudaMemsetAsync, ::cudaMemset2DAsync, ::cudaMemset3DAsync,\n ::cudaMalloc3D, ::make_cudaPitchedPtr,\n ::make_cudaExtent"]
pub unsafe fn cudaMemset3D(
    pitchedDevPtr: cudaPitchedPtr,
    value: ::std::os::raw::c_int,
    extent: cudaExtent,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaMemset3D(pitchedDevPtr, value, extent) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Initializes or sets device memory to a value\n\n Fills the first \\p count bytes of the memory area pointed to by \\p devPtr\n with the constant byte value \\p value.\n\n ::cudaMemsetAsync() is asynchronous with respect to the host, so\n the call may return before the memset is complete. The operation can optionally\n be associated to a stream by passing a non-zero \\p stream argument.\n If \\p stream is non-zero, the operation may overlap with operations in other streams.\n\n The device version of this function only handles device to device copies and\n cannot be given local or shared pointers.\n\n \\param devPtr - Pointer to device memory\n \\param value  - Value to set for each byte of specified memory\n \\param count  - Size in bytes to set\n \\param stream - Stream identifier\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n \\notefnerr\n \\note_memset\n \\note_null_stream\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaMemset, ::cudaMemset2D, ::cudaMemset3D,\n ::cudaMemset2DAsync, ::cudaMemset3DAsync,\n ::cuMemsetD8Async,\n ::cuMemsetD16Async,\n ::cuMemsetD32Async"]
pub unsafe fn cudaMemsetAsync<T: ::cuda_libs::types::CudaAsPtr>(
    mut devPtr: T,
    value: ::std::os::raw::c_int,
    count: usize,
    stream: cudaStream_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaMemsetAsync(
            devPtr.as_mut_ptr() as *mut ::std::os::raw::c_void,
            value,
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
#[doc = " \\brief Initializes or sets device memory to a value\n\n Sets to the specified value \\p value a matrix (\\p height rows of \\p width\n bytes each) pointed to by \\p dstPtr. \\p pitch is the width in bytes of the\n 2D array pointed to by \\p dstPtr, including any padding added to the end\n of each row. This function performs fastest when the pitch is one that has\n been passed back by ::cudaMallocPitch().\n\n ::cudaMemset2DAsync() is asynchronous with respect to the host, so\n the call may return before the memset is complete. The operation can optionally\n be associated to a stream by passing a non-zero \\p stream argument.\n If \\p stream is non-zero, the operation may overlap with operations in other streams.\n\n The device version of this function only handles device to device copies and\n cannot be given local or shared pointers.\n\n \\param devPtr - Pointer to 2D device memory\n \\param pitch  - Pitch in bytes of 2D device memory(Unused if \\p height is 1)\n \\param value  - Value to set for each byte of specified memory\n \\param width  - Width of matrix set (columns in bytes)\n \\param height - Height of matrix set (rows)\n \\param stream - Stream identifier\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n \\notefnerr\n \\note_memset\n \\note_null_stream\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaMemset, ::cudaMemset2D, ::cudaMemset3D,\n ::cudaMemsetAsync, ::cudaMemset3DAsync,\n ::cuMemsetD2D8Async,\n ::cuMemsetD2D16Async,\n ::cuMemsetD2D32Async"]
pub unsafe fn cudaMemset2DAsync<T: ::cuda_libs::types::CudaAsPtr>(
    mut devPtr: T,
    pitch: usize,
    value: ::std::os::raw::c_int,
    width: usize,
    height: usize,
    stream: cudaStream_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaMemset2DAsync(
            devPtr.as_mut_ptr() as *mut ::std::os::raw::c_void,
            pitch,
            value,
            width,
            height,
            stream,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Initializes or sets device memory to a value\n\n Initializes each element of a 3D array to the specified value \\p value.\n The object to initialize is defined by \\p pitchedDevPtr. The \\p pitch field\n of \\p pitchedDevPtr is the width in memory in bytes of the 3D array pointed\n to by \\p pitchedDevPtr, including any padding added to the end of each row.\n The \\p xsize field specifies the logical width of each row in bytes, while\n the \\p ysize field specifies the height of each 2D slice in rows.\n The \\p pitch field of \\p pitchedDevPtr is ignored when \\p height and \\p depth\n are both equal to 1.\n\n The extents of the initialized region are specified as a \\p width in bytes,\n a \\p height in rows, and a \\p depth in slices.\n\n Extents with \\p width greater than or equal to the \\p xsize of\n \\p pitchedDevPtr may perform significantly faster than extents narrower\n than the \\p xsize. Secondarily, extents with \\p height equal to the\n \\p ysize of \\p pitchedDevPtr will perform faster than when the \\p height is\n shorter than the \\p ysize.\n\n This function performs fastest when the \\p pitchedDevPtr has been allocated\n by ::cudaMalloc3D().\n\n ::cudaMemset3DAsync() is asynchronous with respect to the host, so\n the call may return before the memset is complete. The operation can optionally\n be associated to a stream by passing a non-zero \\p stream argument.\n If \\p stream is non-zero, the operation may overlap with operations in other streams.\n\n The device version of this function only handles device to device copies and\n cannot be given local or shared pointers.\n\n \\param pitchedDevPtr - Pointer to pitched device memory\n \\param value         - Value to set for each byte of specified memory\n \\param extent        - Size parameters for where to set device memory (\\p width field in bytes)\n \\param stream - Stream identifier\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n \\notefnerr\n \\note_memset\n \\note_null_stream\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaMemset, ::cudaMemset2D, ::cudaMemset3D,\n ::cudaMemsetAsync, ::cudaMemset2DAsync,\n ::cudaMalloc3D, ::make_cudaPitchedPtr,\n ::make_cudaExtent"]
pub unsafe fn cudaMemset3DAsync(
    pitchedDevPtr: cudaPitchedPtr,
    value: ::std::os::raw::c_int,
    extent: cudaExtent,
    stream: cudaStream_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaMemset3DAsync(pitchedDevPtr, value, extent, stream) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Finds the address associated with a CUDA symbol\n\n Returns in \\p *devPtr the address of symbol \\p symbol on the device.\n \\p symbol is a variable that resides in global or constant memory space.\n If \\p symbol cannot be found, or if \\p symbol is not declared in the\n global or constant memory space, \\p *devPtr is unchanged and the error\n ::cudaErrorInvalidSymbol is returned.\n\n \\param devPtr - Return device pointer associated with symbol\n \\param symbol - Device symbol address\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidSymbol,\n ::cudaErrorNoKernelImageForDevice\n \\notefnerr\n \\note_string_api_deprecation\n \\note_init_rt\n \\note_callback\n\n \\sa\n \\ref ::cudaGetSymbolAddress(void**, const T&) \"cudaGetSymbolAddress (C++ API)\",\n \\ref ::cudaGetSymbolSize(size_t*, const void*) \"cudaGetSymbolSize (C API)\",\n ::cuModuleGetGlobal"]
pub unsafe fn cudaGetSymbolAddress<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
>(
    mut devPtr: T,
    symbol: U,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGetSymbolAddress(
            devPtr.as_mut_ptr() as *mut *mut ::std::os::raw::c_void,
            symbol.as_const_ptr() as *const ::std::os::raw::c_void,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Finds the size of the object associated with a CUDA symbol\n\n Returns in \\p *size the size of symbol \\p symbol. \\p symbol is a variable that\n resides in global or constant memory space. If \\p symbol cannot be found, or\n if \\p symbol is not declared in global or constant memory space, \\p *size is\n unchanged and the error ::cudaErrorInvalidSymbol is returned.\n\n \\param size   - Size of object associated with symbol\n \\param symbol - Device symbol address\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidSymbol,\n ::cudaErrorNoKernelImageForDevice\n \\notefnerr\n \\note_string_api_deprecation\n \\note_init_rt\n \\note_callback\n\n \\sa\n \\ref ::cudaGetSymbolAddress(void**, const void*) \"cudaGetSymbolAddress (C API)\",\n \\ref ::cudaGetSymbolSize(size_t*, const T&) \"cudaGetSymbolSize (C++ API)\",\n ::cuModuleGetGlobal"]
pub unsafe fn cudaGetSymbolSize(
    symbol: *const ::std::os::raw::c_void,
) -> Result<usize, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaGetSymbolSize(out_0.as_mut_ptr() as *mut _, symbol) };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok(out_0.assume_init()) }
    } else {
        Err(status)
    }
}
#[doc = " \\brief Prefetches memory to the specified destination location\n\n Prefetches memory to the specified destination location.  \\p devPtr is the\n base device pointer of the memory to be prefetched and \\p location specifies the\n destination location. \\p count specifies the number of bytes to copy. \\p stream\n is the stream in which the operation is enqueued. The memory range must refer\n to managed memory allocated via ::cudaMallocManaged or declared via __managed__ variables,\n or it may also refer to memory allocated from a managed memory pool,\n or it may also refer to system-allocated memory on systems with non-zero\n cudaDevAttrPageableMemoryAccess.\n\n Specifying ::cudaMemLocationTypeDevice for ::cudaMemLocation::type will prefetch memory to GPU\n specified by device ordinal ::cudaMemLocation::id which must have non-zero value for the device attribute\n ::concurrentManagedAccess. Additionally, \\p stream must be associated with a device\n that has a non-zero value for the device attribute ::concurrentManagedAccess.\n Specifying ::cudaMemLocationTypeHost as ::cudaMemLocation::type will prefetch data to host memory.\n Applications can request prefetching memory to a specific host NUMA node by specifying\n ::cudaMemLocationTypeHostNuma for ::cudaMemLocation::type and a valid host NUMA node id in ::cudaMemLocation::id\n Users can also request prefetching memory to the host NUMA node closest to the current thread's CPU by specifying\n ::cudaMemLocationTypeHostNumaCurrent for ::cudaMemLocation::type. Note when ::cudaMemLocation::type is etiher\n ::cudaMemLocationTypeHost OR ::cudaMemLocationTypeHostNumaCurrent, ::cudaMemLocation::id will be ignored.\n\n The start address and end address of the memory range will be rounded down and rounded up\n respectively to be aligned to CPU page size before the prefetch operation is enqueued\n in the stream.\n\n If no physical memory has been allocated for this region, then this memory region\n will be populated and mapped on the destination device. If there's insufficient\n memory to prefetch the desired region, the Unified Memory driver may evict pages from other\n ::cudaMallocManaged allocations to host memory in order to make room. Device memory\n allocated using ::cudaMalloc or ::cudaMallocArray will not be evicted.\n\n By default, any mappings to the previous location of the migrated pages are removed and\n mappings for the new location are only setup on the destination location. The exact behavior however\n also depends on the settings applied to this memory range via ::cuMemAdvise as described\n below:\n\n If ::cudaMemAdviseSetReadMostly was set on any subset of this memory range,\n then that subset will create a read-only copy of the pages on destination location.\n If however the destination location is a host NUMA node, then any pages of that subset\n that are already in another host NUMA node will be transferred to the destination.\n\n If ::cudaMemAdviseSetPreferredLocation was called on any subset of this memory\n range, then the pages will be migrated to \\p location even if \\p location is not the\n preferred location of any pages in the memory range.\n\n If ::cudaMemAdviseSetAccessedBy was called on any subset of this memory range,\n then mappings to those pages from all the appropriate processors are updated to\n refer to the new location if establishing such a mapping is possible. Otherwise,\n those mappings are cleared.\n\n Note that this API is not required for functionality and only serves to improve performance\n by allowing the application to migrate data to a suitable location before it is accessed.\n Memory accesses to this range are always coherent and are allowed even when the data is\n actively being migrated.\n\n Note that this function is asynchronous with respect to the host and all work\n on other devices.\n\n \\param devPtr    - Pointer to be prefetched\n \\param count     - Size in bytes\n \\param location  - location to prefetch to\n \\param flags     - flags for future use, must be zero now.\n \\param stream    - Stream to enqueue prefetch operation\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorInvalidDevice\n \\notefnerr\n \\note_async\n \\note_null_stream\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaMemcpy, ::cudaMemcpyPeer, ::cudaMemcpyAsync,\n ::cudaMemcpy3DPeerAsync, ::cudaMemAdvise, ::cuMemPrefetchAsync"]
pub unsafe fn cudaMemPrefetchAsync<T: ::cuda_libs::types::CudaAsPtr>(
    devPtr: T,
    count: usize,
    location: cudaMemLocation,
    flags: ::std::os::raw::c_uint,
    stream: cudaStream_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaMemPrefetchAsync(
            devPtr.as_const_ptr() as *const ::std::os::raw::c_void,
            count,
            location,
            flags,
            stream,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Performs a batch of memory prefetches asynchronously\n\n Performs a batch of memory prefetches. The batch as a whole executes in stream order\n but operations within a batch are not guaranteed to execute in any specific order.\n All devices in the system must have a non-zero value for the device attribute\n ::cudaDevAttrConcurrentManagedAccess otherwise the API will return an error.\n\n The semantics of the individual prefetch operations are as described in ::cudaMemPrefetchAsync.\n\n Performs memory prefetch on address ranges specified in \\p dptrs and \\p sizes.\n Both arrays must be of the same length as specified by \\p count. Each memory range specified\n must refer to managed memory allocated via ::cudaMallocManaged or declared via\n __managed__ variables or it may also refer to system-allocated memory when all devices\n have a non-zero value for ::cudaDevAttrPageableMemoryAccess. The prefetch location for\n every operation in the batch is specified in the \\p prefetchLocs array. Each entry in\n this array can apply to more than one operation. This can be done by specifying in the\n \\p prefetchLocIdxs array, the index of the first prefetch operation that the corresponding entry\n in the \\p prefetchLocs array applies to. Both \\p prefetchLocs and \\p prefetchLocIdxs must be of\n the same length as specified by \\p numPrefetchLocs. For example, if a batch has 10 prefetches listed\n in dptrs/sizes, the first 4 of which are to be prefetched to one location and the remaining 6 are to be prefetched\n to another, then \\p numPrefetchLocs will be 2, \\p prefetchLocIdxs will be {0, 4} and \\p prefetchLocs\n will contain the two locations. Note the first entry in \\p prefetchLocIdxs must always be 0.\n Also, each entry must be greater than the previous entry and the last entry should be less than \\p count.\n Furthermore, \\p numPrefetchLocs must be lesser than or equal to \\p count.\n\n \\param dptrs           - Array of pointers to be prefetched\n \\param sizes           - Array of sizes for memory prefetch operations.\n \\param count           - Size of \\p dptrs and \\p sizes arrays.\n \\param prefetchLocs    - Array of locations to prefetch to.\n \\param prefetchLocIdxs - Array of indices to specify which operands each entry in the \\p prefetchLocs array applies to.\n                          The locations specified in prefetchLocs[k] will be applied to copies starting from  prefetchLocIdxs[k]\n                          through  prefetchLocIdxs[k+1] - 1. Also prefetchLocs[numPrefetchLocs - 1] will apply to prefetches starting from\n                          prefetchLocIdxs[numPrefetchLocs - 1] through count - 1.\n \\param numPrefetchLocs - Size of \\p prefetchLocs and \\p prefetchLocIdxs arrays.\n \\param flags           - Flags reserved for future use. Must be zero.\n \\param hStream         - The stream to enqueue the operations in. Must not be legacy NULL stream.\n"]
pub unsafe fn cudaMemPrefetchBatchAsync<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
    W: ::cuda_libs::types::CudaAsPtr,
>(
    mut dptrs: T,
    mut sizes: U,
    count: usize,
    mut prefetchLocs: V,
    mut prefetchLocIdxs: W,
    numPrefetchLocs: usize,
    flags: ::std::os::raw::c_ulonglong,
    stream: cudaStream_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaMemPrefetchBatchAsync(
            dptrs.as_mut_ptr() as *mut *mut ::std::os::raw::c_void,
            sizes.as_mut_ptr() as *mut usize,
            count,
            prefetchLocs.as_mut_ptr() as *mut cudaMemLocation,
            prefetchLocIdxs.as_mut_ptr() as *mut usize,
            numPrefetchLocs,
            flags,
            stream,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Performs a batch of memory discards asynchronously\n\n Performs a batch of memory discards. The batch as a whole executes in stream order\n but operations within a batch are not guaranteed to execute in any specific order.\n All devices in the system must have a non-zero value for the device attribute\n ::cudaDevAttrConcurrentManagedAccess otherwise the API will return an error.\n\n Discarding a memory range informs the driver that the contents of that range are no longer useful.\n Discarding memory ranges allows the driver to optimize certain data migrations and can also help\n reduce memory pressure. This operation can be undone on any part of the range by either writing to it\n or prefetching it via ::cudaMemPrefetchAsync or ::cudaMemPrefetchBatchAsync. Reading from a discarded range,\n without a subsequent write or prefetch to that part of the range, will return an indeterminate value.\n Note that any reads, writes or prefetches to any part of the memory range that occur simultaneously with\n the discard operation result in undefined behavior.\n\n Performs memory discard on address ranges specified in \\p dptrs and \\p sizes.\n Both arrays must be of the same length as specified by \\p count. Each memory range\n specified must refer to managed memory allocated via ::cudaMallocManaged or declared\n via __managed__ variables or it may also refer to system-allocated memory when all devices\n have a non-zero value for ::cudaDevAttrPageableMemoryAccess.\n\n \\param dptrs   - Array of pointers to be discarded\n \\param sizes   - Array of sizes for memory discard operations.\n \\param count   - Size of \\p dptrs and \\p sizes arrays.\n \\param flags   - Flags reserved for future use. Must be zero.\n \\param hStream - The stream to enqueue the operations in. Must not be legacy NULL stream.\n"]
pub unsafe fn cudaMemDiscardBatchAsync<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
>(
    mut dptrs: T,
    mut sizes: U,
    count: usize,
    flags: ::std::os::raw::c_ulonglong,
    stream: cudaStream_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaMemDiscardBatchAsync(
            dptrs.as_mut_ptr() as *mut *mut ::std::os::raw::c_void,
            sizes.as_mut_ptr() as *mut usize,
            count,
            flags,
            stream,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Performs a batch of memory discards and prefetches asynchronously\n\n Performs a batch of memory discards followed by prefetches. The batch as a whole executes\n in stream order but operations within a batch are not guaranteed to execute in any specific order.\n All devices in the system must have a non-zero value for the device attribute\n ::cudaDevAttrConcurrentManagedAccess otherwise the API will return an error.\n\n Calling ::cudaMemDiscardAndPrefetchBatchAsync is semantically equivalent to calling\n ::cudaMemDiscardBatchAsync followed by ::cudaMemPrefetchBatchAsync, but is more optimal.\n For more details on what discarding and prefetching imply, please refer to ::cudaMemDiscardBatchAsync\n and ::cudaMemPrefetchBatchAsync respectively. Note that any reads, writes or prefetches to any part\n of the memory range that occur simultaneously with this combined discard+prefetch operation\n result in undefined behavior.\n\n Performs memory discard and prefetch on address ranges specified in \\p dptrs and \\p sizes.\n Both arrays must be of the same length as specified by \\p count. Each memory range specified\n must refer to managed memory allocated via ::cudaMallocManaged or declared via\n __managed__ variables or it may also refer to system-allocated memory when all devices\n have a non-zero value for ::cudaDevAttrPageableMemoryAccess. Every operation in the batch\n has to be associated with a valid location to prefetch the address range to and specified in\n the \\p prefetchLocs array. Each entry in this array can apply to more than one operation.\n This can be done by specifying in the \\p prefetchLocIdxs array, the index of the first\n operation that the corresponding entry in the \\p prefetchLocs array applies to.\n Both \\p prefetchLocs and \\p prefetchLocIdxs must be of the same length as specified by\n \\p numPrefetchLocs. For example, if a batch has 10 operations listed in dptrs/sizes,\n the first 6 of which are to be prefetched to one location and the remaining 4 are to be\n prefetched to another, then \\p numPrefetchLocs will be 2, \\p prefetchLocIdxs will be {0, 6}\n and \\p prefetchLocs will contain the two set of locations. Note the first entry in\n \\p prefetchLocIdxs must always be 0. Also, each entry must be greater than the previous\n entry and the last entry should be less than \\p count. Furthermore, \\p numPrefetchLocs\n must be lesser than or equal to \\p count.\n\n \\param dptrs           - Array of pointers to be discarded\n \\param sizes           - Array of sizes for memory discard operations.\n \\param count           - Size of \\p dptrs and \\p sizes arrays.\n \\param prefetchLocs    - Array of locations to prefetch to.\n \\param prefetchLocIdxs - Array of indices to specify which operands each entry in the \\p prefetchLocs array applies to.\n                          The locations specified in prefetchLocs[k] will be applied to operations starting from  prefetchLocIdxs[k]\n                          through prefetchLocIdxs[k+1] - 1. Also prefetchLocs[numPrefetchLocs - 1] will apply to copies starting from\n                          prefetchLocIdxs[numPrefetchLocs - 1] through count - 1.\n \\param numPrefetchLocs - Size of \\p prefetchLocs and \\p prefetchLocIdxs arrays.\n \\param flags           - Flags reserved for future use. Must be zero.\n \\param hStream         - The stream to enqueue the operations in. Must not be legacy NULL stream.\n"]
pub unsafe fn cudaMemDiscardAndPrefetchBatchAsync<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
    W: ::cuda_libs::types::CudaAsPtr,
>(
    mut dptrs: T,
    mut sizes: U,
    count: usize,
    mut prefetchLocs: V,
    mut prefetchLocIdxs: W,
    numPrefetchLocs: usize,
    flags: ::std::os::raw::c_ulonglong,
    stream: cudaStream_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaMemDiscardAndPrefetchBatchAsync(
            dptrs.as_mut_ptr() as *mut *mut ::std::os::raw::c_void,
            sizes.as_mut_ptr() as *mut usize,
            count,
            prefetchLocs.as_mut_ptr() as *mut cudaMemLocation,
            prefetchLocIdxs.as_mut_ptr() as *mut usize,
            numPrefetchLocs,
            flags,
            stream,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Advise about the usage of a given memory range\n\n Advise the Unified Memory subsystem about the usage pattern for the memory range\n starting at \\p devPtr with a size of \\p count bytes. The start address and end address of the memory\n range will be rounded down and rounded up respectively to be aligned to CPU page size before the\n advice is applied. The memory range must refer to managed memory allocated via ::cudaMallocManaged\n or declared via __managed__ variables. The memory range could also refer to system-allocated pageable\n memory provided it represents a valid, host-accessible region of memory and all additional constraints\n imposed by \\p advice as outlined below are also satisfied. Specifying an invalid system-allocated pageable\n memory range results in an error being returned.\n\n The \\p advice parameter can take the following values:\n - ::cudaMemAdviseSetReadMostly: This implies that the data is mostly going to be read\n from and only occasionally written to. Any read accesses from any processor to this region will create a\n read-only copy of at least the accessed pages in that processor's memory. Additionally, if ::cudaMemPrefetchAsync\n or ::cudaMemPrefetchAsync is called on this region, it will create a read-only copy of the data on the destination processor.\n If the target location for ::cudaMemPrefetchAsync is a host NUMA node and a read-only copy already exists on\n another host NUMA node, that copy will be migrated to the targeted host NUMA node.\n If any processor writes to this region, all copies of the corresponding page will be invalidated\n except for the one where the write occurred. If the writing processor is the CPU and the preferred location of\n the page is a host NUMA node, then the page will also be migrated to that host NUMA node. The \\p location argument is ignored for this advice.\n Note that for a page to be read-duplicated, the accessing processor must either be the CPU or a GPU\n that has a non-zero value for the device attribute ::cudaDevAttrConcurrentManagedAccess.\n Also, if a context is created on a device that does not have the device attribute\n ::cudaDevAttrConcurrentManagedAccess set, then read-duplication will not occur until\n all such contexts are destroyed.\n If the memory region refers to valid system-allocated pageable memory, then the accessing device must\n have a non-zero value for the device attribute ::cudaDevAttrPageableMemoryAccess for a read-only\n copy to be created on that device. Note however that if the accessing device also has a non-zero value for the\n device attribute ::cudaDevAttrPageableMemoryAccessUsesHostPageTables, then setting this advice\n will not create a read-only copy when that device accesses this memory region.\n\n - ::cudaMemAdviceUnsetReadMostly:  Undoes the effect of ::cudaMemAdviseSetReadMostly and also prevents the\n Unified Memory driver from attempting heuristic read-duplication on the memory range. Any read-duplicated\n copies of the data will be collapsed into a single copy. The location for the collapsed\n copy will be the preferred location if the page has a preferred location and one of the read-duplicated\n copies was resident at that location. Otherwise, the location chosen is arbitrary.\n Note: The \\p location argument is ignored for this advice.\n\n - ::cudaMemAdviseSetPreferredLocation: This advice sets the preferred location for the\n data to be the memory belonging to \\p location. When ::cudaMemLocation::type is ::cudaMemLocationTypeHost,\n ::cudaMemLocation::id is ignored and the preferred location is set to be host memory. To set the preferred location\n to a specific host NUMA node, applications must set ::cudaMemLocation::type to ::cudaMemLocationTypeHostNuma and\n ::cudaMemLocation::id must specify the NUMA ID of the host NUMA node. If ::cudaMemLocation::type is set to ::cudaMemLocationTypeHostNumaCurrent,\n ::cudaMemLocation::id will be ignored and the host NUMA node closest to the calling thread's CPU will be used as the preferred location.\n If ::cudaMemLocation::type is a ::cudaMemLocationTypeDevice, then ::cudaMemLocation::id must be a valid device ordinal\n and the device must have a non-zero value for the device attribute ::cudaDevAttrConcurrentManagedAccess.\n Setting the preferred location does not cause data to migrate to that location immediately. Instead, it guides the migration policy\n when a fault occurs on that memory region. If the data is already in its preferred location and the\n faulting processor can establish a mapping without requiring the data to be migrated, then\n data migration will be avoided. On the other hand, if the data is not in its preferred location\n or if a direct mapping cannot be established, then it will be migrated to the processor accessing\n it. It is important to note that setting the preferred location does not prevent data prefetching\n done using ::cudaMemPrefetchAsync.\n Having a preferred location can override the page thrash detection and resolution logic in the Unified\n Memory driver. Normally, if a page is detected to be constantly thrashing between for example host and device\n memory, the page may eventually be pinned to host memory by the Unified Memory driver. But\n if the preferred location is set as device memory, then the page will continue to thrash indefinitely.\n If ::cudaMemAdviseSetReadMostly is also set on this memory region or any subset of it, then the\n policies associated with that advice will override the policies of this advice, unless read accesses from\n \\p location will not result in a read-only copy being created on that procesor as outlined in description for\n the advice ::cudaMemAdviseSetReadMostly.\n If the memory region refers to valid system-allocated pageable memory, and ::cudaMemLocation::type is ::cudaMemLocationTypeDevice\n then ::cudaMemLocation::id must be a valid device that has a non-zero alue for the device attribute ::cudaDevAttrPageableMemoryAccess.\n\n - ::cudaMemAdviseUnsetPreferredLocation: Undoes the effect of ::cudaMemAdviseSetPreferredLocation\n and changes the preferred location to none. The \\p location argument is ignored for this advice.\n\n - ::cudaMemAdviseSetAccessedBy: This advice implies that the data will be accessed by processor \\p location.\n The ::cudaMemLocation::type must be either ::cudaMemLocationTypeDevice with ::cudaMemLocation::id representing a valid device\n ordinal or ::cudaMemLocationTypeHost and ::cudaMemLocation::id will be ignored. All other location types are invalid.\n If ::cudaMemLocation::id is a GPU, then the device attribute ::cudaDevAttrConcurrentManagedAccess must be non-zero.\n This advice does not cause data migration and has no impact on the location of the data per se. Instead,\n it causes the data to always be mapped in the specified processor's page tables, as long as the\n location of the data permits a mapping to be established. If the data gets migrated for any reason,\n the mappings are updated accordingly.\n This advice is recommended in scenarios where data locality is not important, but avoiding faults is.\n Consider for example a system containing multiple GPUs with peer-to-peer access enabled, where the\n data located on one GPU is occasionally accessed by peer GPUs. In such scenarios, migrating data\n over to the other GPUs is not as important because the accesses are infrequent and the overhead of\n migration may be too high. But preventing faults can still help improve performance, and so having\n a mapping set up in advance is useful. Note that on CPU access of this data, the data may be migrated\n to host memory because the CPU typically cannot access device memory directly. Any GPU that had the\n ::cudaMemAdviseSetAccessedBy flag set for this data will now have its mapping updated to point to the\n page in host memory.\n If ::cudaMemAdviseSetReadMostly is also set on this memory region or any subset of it, then the\n policies associated with that advice will override the policies of this advice. Additionally, if the\n preferred location of this memory region or any subset of it is also \\p location, then the policies\n associated with ::CU_MEM_ADVISE_SET_PREFERRED_LOCATION will override the policies of this advice.\n If the memory region refers to valid system-allocated pageable memory, and ::cudaMemLocation::type is ::cudaMemLocationTypeDevice\n then device in ::cudaMemLocation::id must have a non-zero value for the device attribute ::cudaDevAttrPageableMemoryAccess.\n Additionally, if ::cudaMemLocation::id has a non-zero value for the device attribute ::cudaDevAttrPageableMemoryAccessUsesHostPageTables,\n then this call has no effect.\n\n - ::CU_MEM_ADVISE_UNSET_ACCESSED_BY: Undoes the effect of ::cudaMemAdviseSetAccessedBy. Any mappings to\n the data from \\p location may be removed at any time causing accesses to result in non-fatal page faults.\n If the memory region refers to valid system-allocated pageable memory, and ::cudaMemLocation::type is ::cudaMemLocationTypeDevice\n then device in ::cudaMemLocation::id must have a non-zero value for the device attribute ::cudaDevAttrPageableMemoryAccess.\n Additionally, if ::cudaMemLocation::id has a non-zero value for the device attribute ::cudaDevAttrPageableMemoryAccessUsesHostPageTables,\n then this call has no effect.\n\n \\param devPtr   - Pointer to memory to set the advice for\n \\param count    - Size in bytes of the memory range\n \\param advice   - Advice to be applied for the specified memory range\n \\param location - location to apply the advice for\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorInvalidDevice\n \\notefnerr\n \\note_async\n \\note_null_stream\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaMemcpy, ::cudaMemcpyPeer, ::cudaMemcpyAsync,\n ::cudaMemcpy3DPeerAsync, ::cudaMemPrefetchAsync,\n ::cuMemAdvise"]
pub unsafe fn cudaMemAdvise<T: ::cuda_libs::types::CudaAsPtr>(
    devPtr: T,
    count: usize,
    advice: cudaMemoryAdvise,
    location: cudaMemLocation,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaMemAdvise(
            devPtr.as_const_ptr() as *const ::std::os::raw::c_void,
            count,
            advice,
            location,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Query an attribute of a given memory range\n\n Query an attribute about the memory range starting at \\p devPtr with a size of \\p count bytes. The\n memory range must refer to managed memory allocated via ::cudaMallocManaged or declared via\n __managed__ variables.\n\n The \\p attribute parameter can take the following values:\n - ::cudaMemRangeAttributeReadMostly: If this attribute is specified, \\p data will be interpreted\n as a 32-bit integer, and \\p dataSize must be 4. The result returned will be 1 if all pages in the given\n memory range have read-duplication enabled, or 0 otherwise.\n - ::cudaMemRangeAttributePreferredLocation: If this attribute is specified, \\p data will be\n interpreted as a 32-bit integer, and \\p dataSize must be 4. The result returned will be a GPU device\n id if all pages in the memory range have that GPU as their preferred location, or it will be cudaCpuDeviceId\n if all pages in the memory range have the CPU as their preferred location, or it will be cudaInvalidDeviceId\n if either all the pages don't have the same preferred location or some of the pages don't have a\n preferred location at all. Note that the actual location of the pages in the memory range at the time of\n the query may be different from the preferred location.\n - ::cudaMemRangeAttributeAccessedBy: If this attribute is specified, \\p data will be interpreted\n as an array of 32-bit integers, and \\p dataSize must be a non-zero multiple of 4. The result returned\n will be a list of device ids that had ::cudaMemAdviceSetAccessedBy set for that entire memory range.\n If any device does not have that advice set for the entire memory range, that device will not be included.\n If \\p data is larger than the number of devices that have that advice set for that memory range,\n cudaInvalidDeviceId will be returned in all the extra space provided. For ex., if \\p dataSize is 12\n (i.e. \\p data has 3 elements) and only device 0 has the advice set, then the result returned will be\n { 0, cudaInvalidDeviceId, cudaInvalidDeviceId }. If \\p data is smaller than the number of devices that have\n that advice set, then only as many devices will be returned as can fit in the array. There is no\n guarantee on which specific devices will be returned, however.\n - ::cudaMemRangeAttributeLastPrefetchLocation: If this attribute is specified, \\p data will be\n interpreted as a 32-bit integer, and \\p dataSize must be 4. The result returned will be the last location\n to which all pages in the memory range were prefetched explicitly via ::cudaMemPrefetchAsync. This will either be\n a GPU id or cudaCpuDeviceId depending on whether the last location for prefetch was a GPU or the CPU\n respectively. If any page in the memory range was never explicitly prefetched or if all pages were not\n prefetched to the same location, cudaInvalidDeviceId will be returned. Note that this simply returns the\n last location that the applicaton requested to prefetch the memory range to. It gives no indication as to\n whether the prefetch operation to that location has completed or even begun.\n - ::cudaMemRangeAttributePreferredLocationType: If this attribute is specified, \\p data will be\n interpreted as a ::cudaMemLocationType, and \\p dataSize must be sizeof(cudaMemLocationType). The ::cudaMemLocationType returned will be\n ::cudaMemLocationTypeDevice if all pages in the memory range have the same GPU as their preferred location, or ::cudaMemLocationType\n will be ::cudaMemLocationTypeHost if all pages in the memory range have the CPU as their preferred location, or or it will be ::cudaMemLocationTypeHostNuma\n if all the pages in the memory range have the same host NUMA node ID as their preferred location or it will be ::cudaMemLocationTypeInvalid\n if either all the pages don't have the same preferred location or some of the pages don't have a preferred location at all.\n Note that the actual location type of the pages in the memory range at the time of the query may be different from the preferred location type.\n  - ::cudaMemRangeAttributePreferredLocationId: If this attribute is specified, \\p data will be\n interpreted as a 32-bit integer, and \\p dataSize must be 4. If the ::cudaMemRangeAttributePreferredLocationType query for the same address range\n returns ::cudaMemLocationTypeDevice, it will be a valid device ordinal or if it returns ::cudaMemLocationTypeHostNuma, it will be a valid host NUMA node ID\n or if it returns any other location type, the id should be ignored.\n - ::cudaMemRangeAttributeLastPrefetchLocationType: If this attribute is specified, \\p data will be\n interpreted as a ::cudaMemLocationType, and \\p dataSize must be sizeof(cudaMemLocationType). The result returned will be the last location type\n to which all pages in the memory range were prefetched explicitly via ::cuMemPrefetchAsync. The ::cudaMemLocationType returned\n will be ::cudaMemLocationTypeDevice if the last prefetch location was the GPU or ::cudaMemLocationTypeHost if it was the CPU or ::cudaMemLocationTypeHostNuma if\n the last prefetch location was a specific host NUMA node. If any page in the memory range was never explicitly prefetched or if all pages were not\n prefetched to the same location, ::CUmemLocationType will be ::cudaMemLocationTypeInvalid.\n Note that this simply returns the last location type that the application requested to prefetch the memory range to. It gives no indication as to\n whether the prefetch operation to that location has completed or even begun.\n  - ::cudaMemRangeAttributeLastPrefetchLocationId: If this attribute is specified, \\p data will be\n interpreted as a 32-bit integer, and \\p dataSize must be 4. If the ::cudaMemRangeAttributeLastPrefetchLocationType query for the same address range\n returns ::cudaMemLocationTypeDevice, it will be a valid device ordinal or if it returns ::cudaMemLocationTypeHostNuma, it will be a valid host NUMA node ID\n or if it returns any other location type, the id should be ignored.\n\n \\param data      - A pointers to a memory location where the result\n                    of each attribute query will be written to.\n \\param dataSize  - Array containing the size of data\n \\param attribute - The attribute to query\n \\param devPtr    - Start of the range to query\n \\param count     - Size of the range to query\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue\n \\notefnerr\n \\note_async\n \\note_null_stream\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaMemRangeGetAttributes, ::cudaMemPrefetchAsync,\n ::cudaMemAdvise,\n ::cuMemRangeGetAttribute"]
pub unsafe fn cudaMemRangeGetAttribute<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
>(
    mut data: T,
    dataSize: usize,
    attribute: cudaMemRangeAttribute,
    devPtr: U,
    count: usize,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaMemRangeGetAttribute(
            data.as_mut_ptr() as *mut ::std::os::raw::c_void,
            dataSize,
            attribute,
            devPtr.as_const_ptr() as *const ::std::os::raw::c_void,
            count,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Query attributes of a given memory range.\n\n Query attributes of the memory range starting at \\p devPtr with a size of \\p count bytes. The\n memory range must refer to managed memory allocated via ::cudaMallocManaged or declared via\n __managed__ variables. The \\p attributes array will be interpreted to have \\p numAttributes\n entries. The \\p dataSizes array will also be interpreted to have \\p numAttributes entries.\n The results of the query will be stored in \\p data.\n\n The list of supported attributes are given below. Please refer to ::cudaMemRangeGetAttribute for\n attribute descriptions and restrictions.\n\n - ::cudaMemRangeAttributeReadMostly\n - ::cudaMemRangeAttributePreferredLocation\n - ::cudaMemRangeAttributeAccessedBy\n - ::cudaMemRangeAttributeLastPrefetchLocation\n - :: cudaMemRangeAttributePreferredLocationType\n - :: cudaMemRangeAttributePreferredLocationId\n - :: cudaMemRangeAttributeLastPrefetchLocationType\n - :: cudaMemRangeAttributeLastPrefetchLocationId\n\n \\param data          - A two-dimensional array containing pointers to memory\n                        locations where the result of each attribute query will be written to.\n \\param dataSizes     - Array containing the sizes of each result\n \\param attributes    - An array of attributes to query\n                        (numAttributes and the number of attributes in this array should match)\n \\param numAttributes - Number of attributes to query\n \\param devPtr        - Start of the range to query\n \\param count         - Size of the range to query\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaMemRangeGetAttribute, ::cudaMemAdvise,\n ::cudaMemPrefetchAsync,\n ::cuMemRangeGetAttributes"]
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
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok((out_1.assume_init(), out_2.assume_init())) }
    } else {
        Err(status)
    }
}
#[doc = " \\brief Copies data between host and device\n\n \\deprecated\n\n Copies \\p count bytes from the memory area pointed to by \\p src to the\n CUDA array \\p dst starting at \\p hOffset rows and \\p wOffset bytes from\n the upper left corner, where \\p kind specifies the direction\n of the copy, and must be one of ::cudaMemcpyHostToHost,\n ::cudaMemcpyHostToDevice, ::cudaMemcpyDeviceToHost,\n ::cudaMemcpyDeviceToDevice, or ::cudaMemcpyDefault. Passing\n ::cudaMemcpyDefault is recommended, in which case the type of transfer is\n inferred from the pointer values. However, ::cudaMemcpyDefault is only\n allowed on systems that support unified virtual addressing.\n\n \\param dst     - Destination memory address\n \\param wOffset - Destination starting X offset (columns in bytes)\n \\param hOffset - Destination starting Y offset (rows)\n \\param src     - Source memory address\n \\param count   - Size in bytes to copy\n \\param kind    - Type of transfer\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorInvalidMemcpyDirection\n \\notefnerr\n \\note_sync\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaMemcpy, ::cudaMemcpy2D,\n ::cudaMemcpy2DToArray, ::cudaMemcpyFromArray, ::cudaMemcpy2DFromArray,\n ::cudaMemcpyArrayToArray, ::cudaMemcpy2DArrayToArray, ::cudaMemcpyToSymbol,\n ::cudaMemcpyFromSymbol, ::cudaMemcpyAsync, ::cudaMemcpy2DAsync,\n ::cudaMemcpyToArrayAsync, ::cudaMemcpy2DToArrayAsync,\n ::cudaMemcpyFromArrayAsync, ::cudaMemcpy2DFromArrayAsync,\n ::cudaMemcpyToSymbolAsync, ::cudaMemcpyFromSymbolAsync,\n ::cuMemcpyHtoA,\n ::cuMemcpyDtoA"]
pub unsafe fn cudaMemcpyToArray<T: ::cuda_libs::types::CudaAsPtr>(
    dst: cudaArray_t,
    wOffset: usize,
    hOffset: usize,
    src: T,
    count: usize,
    kind: cudaMemcpyKind,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaMemcpyToArray(
            dst,
            wOffset,
            hOffset,
            src.as_const_ptr() as *const ::std::os::raw::c_void,
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
#[doc = " \\brief Copies data between host and device\n\n \\deprecated\n\n Copies \\p count bytes from the CUDA array \\p src starting at \\p hOffset rows\n and \\p wOffset bytes from the upper left corner to the memory area pointed to\n by \\p dst, where \\p kind specifies the direction of the copy, and must be one of\n ::cudaMemcpyHostToHost, ::cudaMemcpyHostToDevice, ::cudaMemcpyDeviceToHost,\n ::cudaMemcpyDeviceToDevice, or ::cudaMemcpyDefault. Passing\n ::cudaMemcpyDefault is recommended, in which case the type of transfer is\n inferred from the pointer values. However, ::cudaMemcpyDefault is only\n allowed on systems that support unified virtual addressing.\n\n \\param dst     - Destination memory address\n \\param src     - Source memory address\n \\param wOffset - Source starting X offset (columns in bytes)\n \\param hOffset - Source starting Y offset (rows)\n \\param count   - Size in bytes to copy\n \\param kind    - Type of transfer\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorInvalidMemcpyDirection\n \\notefnerr\n \\note_sync\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaMemcpy, ::cudaMemcpy2D, ::cudaMemcpyToArray,\n ::cudaMemcpy2DToArray, ::cudaMemcpy2DFromArray,\n ::cudaMemcpyArrayToArray, ::cudaMemcpy2DArrayToArray, ::cudaMemcpyToSymbol,\n ::cudaMemcpyFromSymbol, ::cudaMemcpyAsync, ::cudaMemcpy2DAsync,\n ::cudaMemcpyToArrayAsync, ::cudaMemcpy2DToArrayAsync,\n ::cudaMemcpyFromArrayAsync, ::cudaMemcpy2DFromArrayAsync,\n ::cudaMemcpyToSymbolAsync, ::cudaMemcpyFromSymbolAsync,\n ::cuMemcpyAtoH,\n ::cuMemcpyAtoD"]
pub unsafe fn cudaMemcpyFromArray<T: ::cuda_libs::types::CudaAsPtr>(
    mut dst: T,
    src: cudaArray_const_t,
    wOffset: usize,
    hOffset: usize,
    count: usize,
    kind: cudaMemcpyKind,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaMemcpyFromArray(
            dst.as_mut_ptr() as *mut ::std::os::raw::c_void,
            src,
            wOffset,
            hOffset,
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
#[doc = " \\brief Copies data between host and device\n\n \\deprecated\n\n Copies \\p count bytes from the CUDA array \\p src starting at \\p hOffsetSrc\n rows and \\p wOffsetSrc bytes from the upper left corner to the CUDA array\n \\p dst starting at \\p hOffsetDst rows and \\p wOffsetDst bytes from the upper\n left corner, where \\p kind specifies the direction of the copy, and must be one of\n ::cudaMemcpyHostToHost, ::cudaMemcpyHostToDevice, ::cudaMemcpyDeviceToHost,\n ::cudaMemcpyDeviceToDevice, or ::cudaMemcpyDefault. Passing\n ::cudaMemcpyDefault is recommended, in which case the type of transfer is\n inferred from the pointer values. However, ::cudaMemcpyDefault is only\n allowed on systems that support unified virtual addressing.\n\n \\param dst        - Destination memory address\n \\param wOffsetDst - Destination starting X offset (columns in bytes)\n \\param hOffsetDst - Destination starting Y offset (rows)\n \\param src        - Source memory address\n \\param wOffsetSrc - Source starting X offset (columns in bytes)\n \\param hOffsetSrc - Source starting Y offset (rows)\n \\param count      - Size in bytes to copy\n \\param kind       - Type of transfer\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorInvalidMemcpyDirection\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaMemcpy, ::cudaMemcpy2D, ::cudaMemcpyToArray,\n ::cudaMemcpy2DToArray, ::cudaMemcpyFromArray, ::cudaMemcpy2DFromArray,\n ::cudaMemcpy2DArrayToArray, ::cudaMemcpyToSymbol,\n ::cudaMemcpyFromSymbol, ::cudaMemcpyAsync, ::cudaMemcpy2DAsync,\n ::cudaMemcpyToArrayAsync, ::cudaMemcpy2DToArrayAsync,\n ::cudaMemcpyFromArrayAsync, ::cudaMemcpy2DFromArrayAsync,\n ::cudaMemcpyToSymbolAsync, ::cudaMemcpyFromSymbolAsync,\n ::cuMemcpyAtoA"]
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
        crate::sys::cudaMemcpyArrayToArray(
            dst, wOffsetDst, hOffsetDst, src, wOffsetSrc, hOffsetSrc, count, kind,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Copies data between host and device\n\n \\deprecated\n\n Copies \\p count bytes from the memory area pointed to by \\p src to the\n CUDA array \\p dst starting at \\p hOffset rows and \\p wOffset bytes from\n the upper left corner, where \\p kind specifies the\n direction of the copy, and must be one of ::cudaMemcpyHostToHost,\n ::cudaMemcpyHostToDevice, ::cudaMemcpyDeviceToHost,\n ::cudaMemcpyDeviceToDevice, or ::cudaMemcpyDefault. Passing\n ::cudaMemcpyDefault is recommended, in which case the type of transfer is\n inferred from the pointer values. However, ::cudaMemcpyDefault is only\n allowed on systems that support unified virtual addressing.\n\n ::cudaMemcpyToArrayAsync() is asynchronous with respect to the host, so\n the call may return before the copy is complete. The copy can optionally\n be associated to a stream by passing a non-zero \\p stream argument. If \\p\n kind is ::cudaMemcpyHostToDevice or ::cudaMemcpyDeviceToHost and \\p stream\n is non-zero, the copy may overlap with operations in other streams.\n\n \\param dst     - Destination memory address\n \\param wOffset - Destination starting X offset (columns in bytes)\n \\param hOffset - Destination starting Y offset (rows)\n \\param src     - Source memory address\n \\param count   - Size in bytes to copy\n \\param kind    - Type of transfer\n \\param stream  - Stream identifier\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorInvalidMemcpyDirection\n \\notefnerr\n \\note_async\n \\note_null_stream\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaMemcpy, ::cudaMemcpy2D, ::cudaMemcpyToArray,\n ::cudaMemcpy2DToArray, ::cudaMemcpyFromArray, ::cudaMemcpy2DFromArray,\n ::cudaMemcpyArrayToArray, ::cudaMemcpy2DArrayToArray, ::cudaMemcpyToSymbol,\n ::cudaMemcpyFromSymbol, ::cudaMemcpyAsync, ::cudaMemcpy2DAsync,\n ::cudaMemcpy2DToArrayAsync,\n ::cudaMemcpyFromArrayAsync, ::cudaMemcpy2DFromArrayAsync,\n ::cudaMemcpyToSymbolAsync, ::cudaMemcpyFromSymbolAsync,\n ::cuMemcpyHtoAAsync,\n ::cuMemcpy2DAsync"]
pub unsafe fn cudaMemcpyToArrayAsync<T: ::cuda_libs::types::CudaAsPtr>(
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
            src.as_const_ptr() as *const ::std::os::raw::c_void,
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
#[doc = " \\brief Copies data between host and device\n\n \\deprecated\n\n Copies \\p count bytes from the CUDA array \\p src starting at \\p hOffset rows\n and \\p wOffset bytes from the upper left corner to the memory area pointed to\n by \\p dst, where \\p kind specifies the direction of the copy, and must be one of\n ::cudaMemcpyHostToHost, ::cudaMemcpyHostToDevice, ::cudaMemcpyDeviceToHost,\n ::cudaMemcpyDeviceToDevice, or ::cudaMemcpyDefault. Passing\n ::cudaMemcpyDefault is recommended, in which case the type of transfer is\n inferred from the pointer values. However, ::cudaMemcpyDefault is only\n allowed on systems that support unified virtual addressing.\n\n ::cudaMemcpyFromArrayAsync() is asynchronous with respect to the host, so\n the call may return before the copy is complete. The copy can optionally\n be associated to a stream by passing a non-zero \\p stream argument. If \\p\n kind is ::cudaMemcpyHostToDevice or ::cudaMemcpyDeviceToHost and \\p stream\n is non-zero, the copy may overlap with operations in other streams.\n\n \\param dst     - Destination memory address\n \\param src     - Source memory address\n \\param wOffset - Source starting X offset (columns in bytes)\n \\param hOffset - Source starting Y offset (rows)\n \\param count   - Size in bytes to copy\n \\param kind    - Type of transfer\n \\param stream  - Stream identifier\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorInvalidMemcpyDirection\n \\notefnerr\n \\note_async\n \\note_null_stream\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaMemcpy, ::cudaMemcpy2D, ::cudaMemcpyToArray,\n ::cudaMemcpy2DToArray, ::cudaMemcpyFromArray, ::cudaMemcpy2DFromArray,\n ::cudaMemcpyArrayToArray, ::cudaMemcpy2DArrayToArray, ::cudaMemcpyToSymbol,\n ::cudaMemcpyFromSymbol, ::cudaMemcpyAsync, ::cudaMemcpy2DAsync,\n ::cudaMemcpyToArrayAsync, ::cudaMemcpy2DToArrayAsync,\n ::cudaMemcpy2DFromArrayAsync,\n ::cudaMemcpyToSymbolAsync, ::cudaMemcpyFromSymbolAsync,\n ::cuMemcpyAtoHAsync,\n ::cuMemcpy2DAsync"]
pub unsafe fn cudaMemcpyFromArrayAsync<T: ::cuda_libs::types::CudaAsPtr>(
    mut dst: T,
    src: cudaArray_const_t,
    wOffset: usize,
    hOffset: usize,
    count: usize,
    kind: cudaMemcpyKind,
    stream: cudaStream_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaMemcpyFromArrayAsync(
            dst.as_mut_ptr() as *mut ::std::os::raw::c_void,
            src,
            wOffset,
            hOffset,
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
pub unsafe fn cudaMallocAsync<T>(
    size: usize,
    hStream: cudaStream_t,
) -> Result<::cuda_libs::types::cuDeviceAllocation<T>, crate::sys::cudaError> {
    let mut dev_ptr = std::ptr::null_mut();
    let status =
        unsafe { crate::sys::cudaMallocAsync(&mut dev_ptr as *mut _ as *mut _, size, hStream) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(::cuda_libs::types::cuDeviceAllocation(dev_ptr as *mut T))
    } else {
        Err(status)
    }
}
pub unsafe fn cudaFreeAsync<T>(
    devPtr: ::cuda_libs::types::cuDeviceAllocation<T>,
    hStream: cudaStream_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaFreeAsync(devPtr.0 as *mut _, hStream) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Tries to release memory back to the OS\n\n Releases memory back to the OS until the pool contains fewer than minBytesToKeep\n reserved bytes, or there is no more memory that the allocator can safely release.\n The allocator cannot release OS allocations that back outstanding asynchronous allocations.\n The OS allocations may happen at different granularity from the user allocations.\n\n \\note: Allocations that have not been freed count as outstanding.\n \\note: Allocations that have been asynchronously freed but whose completion has\n        not been observed on the host (eg. by a synchronize) can count as outstanding.\n\n \\param[in] pool           - The memory pool to trim\n \\param[in] minBytesToKeep - If the pool has less than minBytesToKeep reserved,\n the TrimTo operation is a no-op.  Otherwise the pool will be guaranteed to have\n at least minBytesToKeep bytes reserved after the operation.\n \\returns\n ::cudaSuccess,\n ::cudaErrorInvalidValue\n \\note_callback\n\n \\sa ::cuMemPoolTrimTo, ::cudaMallocAsync, ::cudaFreeAsync, ::cudaDeviceGetDefaultMemPool, ::cudaDeviceGetMemPool, ::cudaMemPoolCreate"]
pub unsafe fn cudaMemPoolTrimTo(
    memPool: cudaMemPool_t,
    minBytesToKeep: usize,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaMemPoolTrimTo(memPool, minBytesToKeep) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Sets attributes of a memory pool\n\n Supported attributes are:\n - ::cudaMemPoolAttrReleaseThreshold: (value type = cuuint64_t)\n                    Amount of reserved memory in bytes to hold onto before trying\n                    to release memory back to the OS. When more than the release\n                    threshold bytes of memory are held by the memory pool, the\n                    allocator will try to release memory back to the OS on the\n                    next call to stream, event or context synchronize. (default 0)\n - ::cudaMemPoolReuseFollowEventDependencies: (value type = int)\n                    Allow ::cudaMallocAsync to use memory asynchronously freed\n                    in another stream as long as a stream ordering dependency\n                    of the allocating stream on the free action exists.\n                    Cuda events and null stream interactions can create the required\n                    stream ordered dependencies. (default enabled)\n - ::cudaMemPoolReuseAllowOpportunistic: (value type = int)\n                    Allow reuse of already completed frees when there is no dependency\n                    between the free and allocation. (default enabled)\n - ::cudaMemPoolReuseAllowInternalDependencies: (value type = int)\n                    Allow ::cudaMallocAsync to insert new stream dependencies\n                    in order to establish the stream ordering required to reuse\n                    a piece of memory released by ::cudaFreeAsync (default enabled).\n - ::cudaMemPoolAttrReservedMemHigh: (value type = cuuint64_t)\n                    Reset the high watermark that tracks the amount of backing memory that was\n                    allocated for the memory pool. It is illegal to set this attribute to a non-zero value.\n - ::cudaMemPoolAttrUsedMemHigh: (value type = cuuint64_t)\n                    Reset the high watermark that tracks the amount of used memory that was\n                    allocated for the memory pool. It is illegal to set this attribute to a non-zero value.\n\n \\param[in] pool  - The memory pool to modify\n \\param[in] attr  - The attribute to modify\n \\param[in] value - Pointer to the value to assign\n\n \\returns\n ::cudaSuccess,\n ::cudaErrorInvalidValue\n \\note_callback\n\n \\sa ::cuMemPoolSetAttribute, ::cudaMallocAsync, ::cudaFreeAsync, ::cudaDeviceGetDefaultMemPool, ::cudaDeviceGetMemPool, ::cudaMemPoolCreate\n"]
pub unsafe fn cudaMemPoolSetAttribute<T: ::cuda_libs::types::CudaAsPtr>(
    memPool: cudaMemPool_t,
    attr: cudaMemPoolAttr,
    mut value: T,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaMemPoolSetAttribute(
            memPool,
            attr,
            value.as_mut_ptr() as *mut ::std::os::raw::c_void,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Gets attributes of a memory pool\n\n Supported attributes are:\n - ::cudaMemPoolAttrReleaseThreshold: (value type = cuuint64_t)\n                    Amount of reserved memory in bytes to hold onto before trying\n                    to release memory back to the OS. When more than the release\n                    threshold bytes of memory are held by the memory pool, the\n                    allocator will try to release memory back to the OS on the\n                    next call to stream, event or context synchronize. (default 0)\n - ::cudaMemPoolReuseFollowEventDependencies: (value type = int)\n                    Allow ::cudaMallocAsync to use memory asynchronously freed\n                    in another stream as long as a stream ordering dependency\n                    of the allocating stream on the free action exists.\n                    Cuda events and null stream interactions can create the required\n                    stream ordered dependencies. (default enabled)\n - ::cudaMemPoolReuseAllowOpportunistic: (value type = int)\n                    Allow reuse of already completed frees when there is no dependency\n                    between the free and allocation. (default enabled)\n - ::cudaMemPoolReuseAllowInternalDependencies: (value type = int)\n                    Allow ::cudaMallocAsync to insert new stream dependencies\n                    in order to establish the stream ordering required to reuse\n                    a piece of memory released by ::cudaFreeAsync (default enabled).\n - ::cudaMemPoolAttrReservedMemCurrent: (value type = cuuint64_t)\n                    Amount of backing memory currently allocated for the mempool.\n - ::cudaMemPoolAttrReservedMemHigh: (value type = cuuint64_t)\n                    High watermark of backing memory allocated for the mempool since\n                    the last time it was reset.\n - ::cudaMemPoolAttrUsedMemCurrent: (value type = cuuint64_t)\n                    Amount of memory from the pool that is currently in use by the application.\n - ::cudaMemPoolAttrUsedMemHigh: (value type = cuuint64_t)\n                    High watermark of the amount of memory from the pool that was in use by the\n                    application since the last time it was reset.\n\n The following properties can be also be queried on imported and default pools:\n - ::cudaMemPoolAttrAllocationType: (value type = cudaMemAllocationType)\n                    The allocation type of the mempool\n - ::cudaMemPoolAttrExportHandleTypes: (value type = cudaMemAllocationHandleType)\n                    Available export handle types for the mempool. For imported pools this\n                    value is always cudaMemHandleTypeNone as an imported pool cannot be\n                    re-exported\n - ::cudaMemPoolAttrLocationId: (value type = int)\n                    The location id for the mempool. If the location type for this pool is\n                    cudaMemLocationTypeInvisible then ID will be cudaInvalidDeviceId.\n - ::cudaMemPoolAttrLocationType: (value type = cudaMemLocationType)\n                    The location type for the mempool. For imported memory pools where the\n                    device is not directly visible to the importing process or pools imported\n                    via fabric handles across nodes this will be\n                    cudaMemlocataionTypeInvisible.\n - ::cudaMemPoolAttrMaxPoolSize: (value type = cuuint64_t)\n                    Maximum size of the pool in bytes, this value may be higher than what was\n                    initially passed to cuMemPoolCreate due to alignment requirements. A\n                    value of 0 indicates no maximum size. For cudaMemAllocationTypeManaged\n                    and IPC imported pools this value will be system dependent.\n - ::cudaMemPoolAttrHwDecompressEnabled: (value type = int)\n                    Indicates whether the pool has hardware compresssion enabled\n\n \\param[in] pool  - The memory pool to get attributes of\n \\param[in] attr  - The attribute to get\n \\param[in] value - Retrieved value\n\n \\returns\n ::cudaSuccess,\n ::cudaErrorInvalidValue\n \\note_callback\n\n \\sa ::cuMemPoolGetAttribute, ::cudaMallocAsync, ::cudaFreeAsync, ::cudaDeviceGetDefaultMemPool, ::cudaDeviceGetMemPool, ::cudaMemPoolCreate\n"]
pub unsafe fn cudaMemPoolGetAttribute<T: ::cuda_libs::types::CudaAsPtr>(
    memPool: cudaMemPool_t,
    attr: cudaMemPoolAttr,
    mut value: T,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaMemPoolGetAttribute(
            memPool,
            attr,
            value.as_mut_ptr() as *mut ::std::os::raw::c_void,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Controls visibility of pools between devices\n\n \\param[in] pool  - The pool being modified\n \\param[in] map   - Array of access descriptors. Each descriptor instructs the access to enable for a single gpu\n \\param[in] count - Number of descriptors in the map array.\n\n \\returns\n ::cudaSuccess,\n ::cudaErrorInvalidValue\n\n \\sa ::cuMemPoolSetAccess, ::cudaMemPoolGetAccess, ::cudaMallocAsync, cudaFreeAsync"]
pub unsafe fn cudaMemPoolSetAccess<T: ::cuda_libs::types::CudaAsPtr>(
    memPool: cudaMemPool_t,
    descList: T,
    count: usize,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaMemPoolSetAccess(
            memPool,
            descList.as_const_ptr() as *const cudaMemAccessDesc,
            count,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Returns the accessibility of a pool from a device\n\n Returns the accessibility of the pool's memory from the specified location.\n\n \\param[out] flags   - the accessibility of the pool from the specified location\n \\param[in] memPool  - the pool being queried\n \\param[in] location - the location accessing the pool\n\n \\sa ::cuMemPoolGetAccess, ::cudaMemPoolSetAccess"]
pub unsafe fn cudaMemPoolGetAccess(
    memPool: cudaMemPool_t,
) -> Result<(cudaMemAccessFlags, cudaMemLocation), crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<cudaMemAccessFlags> = std::mem::MaybeUninit::uninit();
    let mut out_2: std::mem::MaybeUninit<cudaMemLocation> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudaMemPoolGetAccess(
            out_0.as_mut_ptr() as *mut _,
            memPool,
            out_2.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok((out_0.assume_init(), out_2.assume_init())) }
    } else {
        Err(status)
    }
}
#[doc = " \\brief Creates a memory pool\n\n Creates a CUDA memory pool and returns the handle in \\p pool.  The \\p poolProps determines\n the properties of the pool such as the backing device and IPC capabilities.\n\n To create a memory pool for host memory not targeting a specific NUMA node, applications must set\n set ::cudaMemPoolProps::cudaMemLocation::type to ::cudaMemLocationTypeHost.\n ::cudaMemPoolProps::cudaMemLocation::id is ignored for such pools.\n Pools created with the type ::cudaMemLocationTypeHost are not IPC capable and\n ::cudaMemPoolProps::handleTypes must be 0, any other values will result in\n ::cudaErrorInvalidValue.\n To create a memory pool targeting a specific host NUMA node, applications must\n set ::cudaMemPoolProps::cudaMemLocation::type to ::cudaMemLocationTypeHostNuma and\n ::cudaMemPoolProps::cudaMemLocation::id must specify the NUMA ID of the host memory node.\n Specifying ::cudaMemLocationTypeHostNumaCurrent as the\n ::cudaMemPoolProps::cudaMemLocation::type will result in ::cudaErrorInvalidValue.\n By default, the pool's memory will be accessible from the device it is allocated on.\n In the case of pools created with ::cudaMemLocationTypeHostNuma or\n ::cudaMemLocationTypeHost, their default accessibility will be from the host\n CPU.\n Applications can control the maximum size of the pool by specifying a non-zero value for ::cudaMemPoolProps::maxSize.\n If set to 0, the maximum size of the pool will default to a system dependent value.\n\n Applications that intend to use ::CU_MEM_HANDLE_TYPE_FABRIC based memory sharing must ensure:\n (1) `nvidia-caps-imex-channels` character device is created by the driver and is listed under /proc/devices\n (2) have at least one IMEX channel file accessible by the user launching the application.\n\n When exporter and importer CUDA processes have been granted access to the same IMEX channel, they can securely\n share memory.\n\n The IMEX channel security model works on a per user basis. Which means all processes under a user can share\n memory if the user has access to a valid IMEX channel. When multi-user isolation is desired, a separate IMEX\n channel is required for each user.\n\n These channel files exist in /dev/nvidia-caps-imex-channels/channel* and can be created using standard OS\n native calls like mknod on Linux. For example: To create channel0 with the major number from /proc/devices\n users can execute the following command: `mknod /dev/nvidia-caps-imex-channels/channel0 c <major number> 0`\n\n To create a managed memory pool, applications must set ::cudaMemPoolProps:cudaMemAllocationType to ::cudaMemAllocationTypeManaged.\n ::cudaMemPoolProps::cudaMemAllocationHandleType must also be set to ::cudaMemHandleTypeNone since IPC is not supported.\n For managed memory pools, ::cudaMemPoolProps::cudaMemLocation will be treated as the preferred location for all allocations created from the pool.\n An application can also set ::cudaMemLocationTypeNone to indicate no preferred location.\n ::cudaMemPoolProps::maxSize must be set to zero for managed memory pools.\n ::cudaMemPoolProps::usage should be zero as decompress for managed memory is not supported.\n For managed memory pools, all devices on the system must have non-zero ::concurrentManagedAccess. If not, this call returns ::cudaErrorNotSupported\n\n \\note Specifying ::cudaMemHandleTypeNone creates a memory pool that will not support IPC.\n\n \\returns\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorNotSupported\n\n \\sa ::cuMemPoolCreate, ::cudaDeviceSetMemPool, ::cudaMallocFromPoolAsync, ::cudaMemPoolExportToShareableHandle, ::cudaDeviceGetDefaultMemPool, ::cudaDeviceGetMemPool\n"]
pub unsafe fn cudaMemPoolCreate<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
>(
    mut memPool: T,
    poolProps: U,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaMemPoolCreate(
            memPool.as_mut_ptr() as *mut cudaMemPool_t,
            poolProps.as_const_ptr() as *const cudaMemPoolProps,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Destroys the specified memory pool\n\n If any pointers obtained from this pool haven't been freed or\n the pool has free operations that haven't completed\n when ::cudaMemPoolDestroy is invoked, the function will return immediately and the\n resources associated with the pool will be released automatically\n once there are no more outstanding allocations.\n\n Destroying the current mempool of a device sets the default mempool of\n that device as the current mempool for that device.\n\n \\note A device's default memory pool cannot be destroyed.\n\n \\returns\n ::cudaSuccess,\n ::cudaErrorInvalidValue\n\n \\sa cuMemPoolDestroy, ::cudaFreeAsync, ::cudaDeviceSetMemPool, ::cudaDeviceGetDefaultMemPool, ::cudaDeviceGetMemPool, ::cudaMemPoolCreate"]
pub unsafe fn cudaMemPoolDestroy(memPool: cudaMemPool_t) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaMemPoolDestroy(memPool) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Returns the default memory pool for a given location and allocation type\n\n The memory location can be of one of ::cudaMemLocationTypeDevice, ::cudaMemLocationTypeHost or\n ::cudaMemLocationTypeHostNuma. The allocation type can be one of ::cudaMemAllocationTypePinned or\n ::cudaMemAllocationTypeManaged. When the allocation type is ::cudaMemAllocationTypeManaged,\n the location type can also be ::cudaMemLocationTypeNone to indicate no preferred location\n for the managed memory pool. In all other cases, the call return ::cudaErrorInvalidValue\n\n \\returns\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorNotSupported,\n\n \\sa ::cuMemAllocAsync, ::cuMemPoolTrimTo, ::cuMemPoolGetAttribute, ::cuMemPoolSetAttribute, cuMemPoolSetAccess, ::cuMemGetMemPool, ::cuMemPoolCreate"]
pub unsafe fn cudaMemGetDefaultMemPool(
    type_: cudaMemAllocationType,
) -> Result<(cudaMemPool_t, cudaMemLocation), crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<cudaMemPool_t> = std::mem::MaybeUninit::uninit();
    let mut out_1: std::mem::MaybeUninit<cudaMemLocation> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudaMemGetDefaultMemPool(
            out_0.as_mut_ptr() as *mut _,
            out_1.as_mut_ptr() as *mut _,
            type_,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok((out_0.assume_init(), out_1.assume_init())) }
    } else {
        Err(status)
    }
}
#[doc = " \\brief Gets the current memory pool for a given memory location and allocation type\n\n The memory location can be of one of ::cudaMemLocationTypeDevice, ::cudaMemLocationTypeHost or\n ::cudaMemLocationTypeHostNuma. The allocation type can be one of ::cudaMemAllocationTypePinned or\n ::cudaMemAllocationTypeManaged. When the allocation type is ::cudaMemAllocationTypeManaged,\n the location type can also be ::cudaMemLocationTypeNone to indicate no preferred location\n for the managed memory pool. In all other cases, the call return ::cudaErrorInvalidValue\n\n Returns the last pool provided to ::cudaMemSetMemPool or ::cudaDeviceSetMemPool for this location and allocation type\n or the location's default memory pool if ::cudaMemSetMemPool or ::cudaDeviceSetMemPool for that allocType and location\n has never been called.\n By default the current mempool of a location is the default mempool for a device that can be obtained via cudaMemGetDefaultMemPool\n Otherwise the returned pool must have been set with ::cudaDeviceSetMemPool.\n\n \\returns\n ::cudaSuccess,\n ::cudaErrorInvalidValue\n\n \\sa ::cuDeviceGetDefaultMemPool, ::cuMemPoolCreate, ::cuDeviceSetMemPool, ::cuMemSetMemPool"]
pub unsafe fn cudaMemGetMemPool(
    type_: cudaMemAllocationType,
) -> Result<(cudaMemPool_t, cudaMemLocation), crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<cudaMemPool_t> = std::mem::MaybeUninit::uninit();
    let mut out_1: std::mem::MaybeUninit<cudaMemLocation> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudaMemGetMemPool(
            out_0.as_mut_ptr() as *mut _,
            out_1.as_mut_ptr() as *mut _,
            type_,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok((out_0.assume_init(), out_1.assume_init())) }
    } else {
        Err(status)
    }
}
#[doc = " \\brief Sets the current memory pool for a memory location and allocation type\n\n The memory location can be of one of ::cudaMemLocationTypeDevice, ::cudaMemLocationTypeHost or\n ::cudaMemLocationTypeHostNuma. The allocation type can be one of ::cudaMemAllocationTypePinned or\n ::cudaMemAllocationTypeManaged. When the allocation type is ::cudaMemAllocationTypeManaged,\n the location type can also be ::cudaMemLocationTypeNone to indicate no preferred location\n for the managed memory pool. In all other cases, the call return ::cudaErrorInvalidValue\n\n When a memory pool is set as the current memory pool, the location parameter should be the same as the location of the pool.\n If the location type or index don't match, the call returns ::cudaErrorInvalidValue.\n The type of memory pool should also match the parameter allocType. Else the call returns ::cudaErrorInvalidValue.\n By default, a memory location's current memory pool is its default memory pool.\n If the location type is ::cudaMemLocationTypeDevice and the allocation type is ::cudaMemAllocationTypePinned, then\n this API is the equivalent of calling ::cudaDeviceSetMemPool with the location id as the device.\n For further details on the implications, please refer to the documentation for ::cudaDeviceSetMemPool.\n\n \\note Use ::cudaMallocFromPoolAsync to specify asynchronous allocations from a device different\n than the one the stream runs on.\n\n \\returns\n ::cudaSuccess,\n ::cudaErrorInvalidValue\n\n \\sa ::cuDeviceGetDefaultMemPool, ::cuDeviceGetMemPool, ::cuMemGetMemPool, ::cuMemPoolCreate, ::cuMemPoolDestroy, ::cuMemAllocFromPoolAsync"]
pub unsafe fn cudaMemSetMemPool<T: ::cuda_libs::types::CudaAsPtr>(
    mut location: T,
    type_: cudaMemAllocationType,
    memPool: cudaMemPool_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaMemSetMemPool(
            location.as_mut_ptr() as *mut cudaMemLocation,
            type_,
            memPool,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudaMallocFromPoolAsync<T>(
    size: usize,
    memPool: cudaMemPool_t,
    stream: cudaStream_t,
) -> Result<::cuda_libs::types::cuDeviceAllocation<T>, crate::sys::cudaError> {
    let mut dev_ptr = std::ptr::null_mut();
    let status = unsafe {
        crate::sys::cudaMallocFromPoolAsync(&mut dev_ptr as *mut _ as *mut _, size, memPool, stream)
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(::cuda_libs::types::cuDeviceAllocation(dev_ptr as *mut T))
    } else {
        Err(status)
    }
}
#[doc = " \\brief Exports a memory pool to the requested handle type.\n\n Given an IPC capable mempool, create an OS handle to share the pool with another process.\n A recipient process can convert the shareable handle into a mempool with ::cudaMemPoolImportFromShareableHandle.\n Individual pointers can then be shared with the ::cudaMemPoolExportPointer and ::cudaMemPoolImportPointer APIs.\n The implementation of what the shareable handle is and how it can be transferred is defined by the requested\n handle type.\n\n \\note: To create an IPC capable mempool, create a mempool with a CUmemAllocationHandleType other than cudaMemHandleTypeNone.\n\n \\param[out] handle_out  - pointer to the location in which to store the requested handle\n \\param[in] pool         - pool to export\n \\param[in] handleType   - the type of handle to create\n \\param[in] flags        - must be 0\n\n \\returns\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorOutOfMemory\n\n \\sa ::cuMemPoolExportToShareableHandle, ::cudaMemPoolImportFromShareableHandle, ::cudaMemPoolExportPointer, ::cudaMemPoolImportPointer"]
pub unsafe fn cudaMemPoolExportToShareableHandle<T: ::cuda_libs::types::CudaAsPtr>(
    mut shareableHandle: T,
    memPool: cudaMemPool_t,
    handleType: cudaMemAllocationHandleType,
    flags: ::std::os::raw::c_uint,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaMemPoolExportToShareableHandle(
            shareableHandle.as_mut_ptr() as *mut ::std::os::raw::c_void,
            memPool,
            handleType,
            flags,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief imports a memory pool from a shared handle.\n\n Specific allocations can be imported from the imported pool with ::cudaMemPoolImportPointer.\n\n \\note Imported memory pools do not support creating new allocations.\n       As such imported memory pools may not be used in ::cudaDeviceSetMemPool\n       or ::cudaMallocFromPoolAsync calls.\n\n \\param[out] pool_out    - Returned memory pool\n \\param[in] handle       - OS handle of the pool to open\n \\param[in] handleType   - The type of handle being imported\n \\param[in] flags        - must be 0\n\n \\returns\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorOutOfMemory\n\n \\sa ::cuMemPoolImportFromShareableHandle, ::cudaMemPoolExportToShareableHandle, ::cudaMemPoolExportPointer, ::cudaMemPoolImportPointer"]
pub unsafe fn cudaMemPoolImportFromShareableHandle<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
>(
    mut memPool: T,
    mut shareableHandle: U,
    handleType: cudaMemAllocationHandleType,
    flags: ::std::os::raw::c_uint,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaMemPoolImportFromShareableHandle(
            memPool.as_mut_ptr() as *mut cudaMemPool_t,
            shareableHandle.as_mut_ptr() as *mut ::std::os::raw::c_void,
            handleType,
            flags,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Export data to share a memory pool allocation between processes.\n\n Constructs \\p shareData_out for sharing a specific allocation from an already shared memory pool.\n The recipient process can import the allocation with the ::cudaMemPoolImportPointer api.\n The data is not a handle and may be shared through any IPC mechanism.\n\n \\param[out] shareData_out - Returned export data\n \\param[in] ptr            - pointer to memory being exported\n\n \\returns\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorOutOfMemory\n\n \\sa ::cuMemPoolExportPointer, ::cudaMemPoolExportToShareableHandle, ::cudaMemPoolImportFromShareableHandle, ::cudaMemPoolImportPointer"]
pub unsafe fn cudaMemPoolExportPointer<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
>(
    mut exportData: T,
    mut ptr: U,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaMemPoolExportPointer(
            exportData.as_mut_ptr() as *mut cudaMemPoolPtrExportData,
            ptr.as_mut_ptr() as *mut ::std::os::raw::c_void,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Import a memory pool allocation from another process.\n\n Returns in \\p ptr_out a pointer to the imported memory.\n The imported memory must not be accessed before the allocation operation completes\n in the exporting process. The imported memory must be freed from all importing processes before\n being freed in the exporting process. The pointer may be freed with cudaFree\n or cudaFreeAsync.  If ::cudaFreeAsync is used, the free must be completed\n on the importing process before the free operation on the exporting process.\n\n \\note The ::cudaFreeAsync api may be used in the exporting process before\n       the ::cudaFreeAsync operation completes in its stream as long as the\n       ::cudaFreeAsync in the exporting process specifies a stream with\n       a stream dependency on the importing process's ::cudaFreeAsync.\n\n \\param[out] ptr_out  - pointer to imported memory\n \\param[in] pool      - pool from which to import\n \\param[in] shareData - data specifying the memory to import\n\n \\returns\n ::CUDA_SUCCESS,\n ::CUDA_ERROR_INVALID_VALUE,\n ::CUDA_ERROR_NOT_INITIALIZED,\n ::CUDA_ERROR_OUT_OF_MEMORY\n\n \\sa ::cuMemPoolImportPointer, ::cudaMemPoolExportToShareableHandle, ::cudaMemPoolImportFromShareableHandle, ::cudaMemPoolExportPointer"]
pub unsafe fn cudaMemPoolImportPointer<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
>(
    mut ptr: T,
    memPool: cudaMemPool_t,
    mut exportData: U,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaMemPoolImportPointer(
            ptr.as_mut_ptr() as *mut *mut ::std::os::raw::c_void,
            memPool,
            exportData.as_mut_ptr() as *mut cudaMemPoolPtrExportData,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Returns attributes about a specified pointer\n\n Returns in \\p *attributes the attributes of the pointer \\p ptr.\n If pointer was not allocated in, mapped by or registered with context\n supporting unified addressing ::cudaErrorInvalidValue is returned.\n\n \\note In CUDA 11.0 forward passing host pointer will return ::cudaMemoryTypeUnregistered\n in ::cudaPointerAttributes::type and call will return ::cudaSuccess.\n\n The ::cudaPointerAttributes structure is defined as:\n \\code\nstruct cudaPointerAttributes {\nenum cudaMemoryType type;\nint device;\nvoid *devicePointer;\nvoid *hostPointer;\n}\n\\endcode\n In this structure, the individual fields mean\n\n - \\ref ::cudaPointerAttributes::type identifies type of memory. It can be\n    ::cudaMemoryTypeUnregistered for unregistered host memory,\n    ::cudaMemoryTypeHost for registered host memory, ::cudaMemoryTypeDevice for device\n    memory or  ::cudaMemoryTypeManaged for managed memory.\n\n - \\ref ::cudaPointerAttributes::device \"device\" is the device against which\n   \\p ptr was allocated.  If \\p ptr has memory type ::cudaMemoryTypeDevice\n   then this identifies the device on which the memory referred to by \\p ptr\n   physically resides.  If \\p ptr has memory type ::cudaMemoryTypeHost then this\n   identifies the device which was current when the allocation was made\n   (and if that device is deinitialized then this allocation will vanish\n   with that device's state).\n\n - \\ref ::cudaPointerAttributes::devicePointer \"devicePointer\" is\n   the device pointer alias through which the memory referred to by \\p ptr\n   may be accessed on the current device.\n   If the memory referred to by \\p ptr cannot be accessed directly by the\n   current device then this is NULL.\n\n - \\ref ::cudaPointerAttributes::hostPointer \"hostPointer\" is\n   the host pointer alias through which the memory referred to by \\p ptr\n   may be accessed on the host.\n   If the memory referred to by \\p ptr cannot be accessed directly by the\n   host then this is NULL.\n\n \\param attributes - Attributes for the specified pointer\n \\param ptr        - Pointer to get attributes for\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidDevice,\n ::cudaErrorInvalidValue\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaGetDeviceCount, ::cudaGetDevice, ::cudaSetDevice,\n ::cudaChooseDevice,\n ::cudaInitDevice,\n ::cuPointerGetAttributes"]
pub unsafe fn cudaPointerGetAttributes(
    ptr: *const ::std::os::raw::c_void,
) -> Result<cudaPointerAttributes, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<cudaPointerAttributes> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaPointerGetAttributes(out_0.as_mut_ptr() as *mut _, ptr) };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok(out_0.assume_init()) }
    } else {
        Err(status)
    }
}
#[doc = " \\brief Queries if a device may directly access a peer device's memory.\n\n Returns in \\p *canAccessPeer a value of 1 if device \\p device is capable of\n directly accessing memory from \\p peerDevice and 0 otherwise.  If direct\n access of \\p peerDevice from \\p device is possible, then access may be\n enabled by calling ::cudaDeviceEnablePeerAccess().\n\n \\param canAccessPeer - Returned access capability\n \\param device        - Device from which allocations on \\p peerDevice are to\n                        be directly accessed.\n \\param peerDevice    - Device on which the allocations to be directly accessed\n                        by \\p device reside.\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidDevice\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaDeviceEnablePeerAccess,\n ::cudaDeviceDisablePeerAccess,\n ::cuDeviceCanAccessPeer"]
pub unsafe fn cudaDeviceCanAccessPeer<T: ::cuda_libs::types::CudaAsPtr>(
    mut canAccessPeer: T,
    device: ::std::os::raw::c_int,
    peerDevice: ::std::os::raw::c_int,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaDeviceCanAccessPeer(
            canAccessPeer.as_mut_ptr() as *mut ::std::os::raw::c_int,
            device,
            peerDevice,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Enables direct access to memory allocations on a peer device.\n\n On success, all allocations from \\p peerDevice will immediately be accessible by\n the current device.  They will remain accessible until access is explicitly\n disabled using ::cudaDeviceDisablePeerAccess() or either device is reset using\n ::cudaDeviceReset().\n\n Note that access granted by this call is unidirectional and that in order to access\n memory on the current device from \\p peerDevice, a separate symmetric call\n to ::cudaDeviceEnablePeerAccess() is required.\n\n Note that there are both device-wide and system-wide limitations per system\n configuration, as noted in the CUDA Programming Guide under the section\n \"Peer-to-Peer Memory Access\".\n\n Returns ::cudaErrorInvalidDevice if ::cudaDeviceCanAccessPeer() indicates\n that the current device cannot directly access memory from \\p peerDevice.\n\n Returns ::cudaErrorPeerAccessAlreadyEnabled if direct access of\n \\p peerDevice from the current device has already been enabled.\n\n Returns ::cudaErrorInvalidValue if \\p flags is not 0.\n\n \\param peerDevice  - Peer device to enable direct access to from the current device\n \\param flags       - Reserved for future use and must be set to 0\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidDevice,\n ::cudaErrorPeerAccessAlreadyEnabled,\n ::cudaErrorInvalidValue\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaDeviceCanAccessPeer,\n ::cudaDeviceDisablePeerAccess,\n ::cuCtxEnablePeerAccess"]
pub unsafe fn cudaDeviceEnablePeerAccess(
    peerDevice: ::std::os::raw::c_int,
    flags: ::std::os::raw::c_uint,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaDeviceEnablePeerAccess(peerDevice, flags) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Disables direct access to memory allocations on a peer device.\n\n Returns ::cudaErrorPeerAccessNotEnabled if direct access to memory on\n \\p peerDevice has not yet been enabled from the current device.\n\n \\param peerDevice - Peer device to disable direct access to\n\n \\return\n ::cudaSuccess,\n ::cudaErrorPeerAccessNotEnabled,\n ::cudaErrorInvalidDevice\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa ::cudaDeviceCanAccessPeer,\n ::cudaDeviceEnablePeerAccess,\n ::cuCtxDisablePeerAccess"]
pub unsafe fn cudaDeviceDisablePeerAccess(
    peerDevice: ::std::os::raw::c_int,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaDeviceDisablePeerAccess(peerDevice) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Unregisters a graphics resource for access by CUDA\n\n Unregisters the graphics resource \\p resource so it is not accessible by\n CUDA unless registered again.\n\n If \\p resource is invalid then ::cudaErrorInvalidResourceHandle is\n returned.\n\n \\param resource - Resource to unregister\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidResourceHandle,\n ::cudaErrorUnknown\n \\notefnerr\n \\note_init_rt\n \\note_callback\n \\note_destroy_ub\n\n \\sa\n ::cudaGraphicsD3D9RegisterResource,\n ::cudaGraphicsD3D10RegisterResource,\n ::cudaGraphicsD3D11RegisterResource,\n ::cudaGraphicsGLRegisterBuffer,\n ::cudaGraphicsGLRegisterImage,\n ::cuGraphicsUnregisterResource"]
pub unsafe fn cudaGraphicsUnregisterResource(
    resource: cudaGraphicsResource_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaGraphicsUnregisterResource(resource) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Set usage flags for mapping a graphics resource\n\n Set \\p flags for mapping the graphics resource \\p resource.\n\n Changes to \\p flags will take effect the next time \\p resource is mapped.\n The \\p flags argument may be any of the following:\n - ::cudaGraphicsMapFlagsNone: Specifies no hints about how \\p resource will\n     be used. It is therefore assumed that CUDA may read from or write to \\p resource.\n - ::cudaGraphicsMapFlagsReadOnly: Specifies that CUDA will not write to \\p resource.\n - ::cudaGraphicsMapFlagsWriteDiscard: Specifies CUDA will not read from \\p resource and will\n   write over the entire contents of \\p resource, so none of the data\n   previously stored in \\p resource will be preserved.\n\n If \\p resource is presently mapped for access by CUDA then ::cudaErrorUnknown is returned.\n If \\p flags is not one of the above values then ::cudaErrorInvalidValue is returned.\n\n \\param resource - Registered resource to set flags for\n \\param flags    - Parameters for resource mapping\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorInvalidResourceHandle,\n ::cudaErrorUnknown,\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa\n ::cudaGraphicsMapResources,\n ::cuGraphicsResourceSetMapFlags"]
pub unsafe fn cudaGraphicsResourceSetMapFlags(
    resource: cudaGraphicsResource_t,
    flags: ::std::os::raw::c_uint,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaGraphicsResourceSetMapFlags(resource, flags) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Map graphics resources for access by CUDA\n\n Maps the \\p count graphics resources in \\p resources for access by CUDA.\n\n The resources in \\p resources may be accessed by CUDA until they\n are unmapped. The graphics API from which \\p resources were registered\n should not access any resources while they are mapped by CUDA. If an\n application does so, the results are undefined.\n\n This function provides the synchronization guarantee that any graphics calls\n issued before ::cudaGraphicsMapResources() will complete before any subsequent CUDA\n work issued in \\p stream begins.\n\n If \\p resources contains any duplicate entries then ::cudaErrorInvalidResourceHandle\n is returned. If any of \\p resources are presently mapped for access by\n CUDA then ::cudaErrorUnknown is returned.\n\n \\param count     - Number of resources to map\n \\param resources - Resources to map for CUDA\n \\param stream    - Stream for synchronization\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidResourceHandle,\n ::cudaErrorUnknown\n \\note_null_stream\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa\n ::cudaGraphicsResourceGetMappedPointer,\n ::cudaGraphicsSubResourceGetMappedArray,\n ::cudaGraphicsUnmapResources,\n ::cuGraphicsMapResources"]
pub unsafe fn cudaGraphicsMapResources<T: ::cuda_libs::types::CudaAsPtr>(
    count: ::std::os::raw::c_int,
    mut resources: T,
    stream: cudaStream_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphicsMapResources(
            count,
            resources.as_mut_ptr() as *mut cudaGraphicsResource_t,
            stream,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Unmap graphics resources.\n\n Unmaps the \\p count graphics resources in \\p resources.\n\n Once unmapped, the resources in \\p resources may not be accessed by CUDA\n until they are mapped again.\n\n This function provides the synchronization guarantee that any CUDA work issued\n in \\p stream before ::cudaGraphicsUnmapResources() will complete before any\n subsequently issued graphics work begins.\n\n If \\p resources contains any duplicate entries then ::cudaErrorInvalidResourceHandle\n is returned. If any of \\p resources are not presently mapped for access by\n CUDA then ::cudaErrorUnknown is returned.\n\n \\param count     - Number of resources to unmap\n \\param resources - Resources to unmap\n \\param stream    - Stream for synchronization\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidResourceHandle,\n ::cudaErrorUnknown\n \\note_null_stream\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa\n ::cudaGraphicsMapResources,\n ::cuGraphicsUnmapResources"]
pub unsafe fn cudaGraphicsUnmapResources<T: ::cuda_libs::types::CudaAsPtr>(
    count: ::std::os::raw::c_int,
    mut resources: T,
    stream: cudaStream_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphicsUnmapResources(
            count,
            resources.as_mut_ptr() as *mut cudaGraphicsResource_t,
            stream,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Get an device pointer through which to access a mapped graphics resource.\n\n Returns in \\p *devPtr a pointer through which the mapped graphics resource\n \\p resource may be accessed.\n Returns in \\p *size the size of the memory in bytes which may be accessed from that pointer.\n The value set in \\p devPtr may change every time that \\p resource is mapped.\n\n If \\p resource is not a buffer then it cannot be accessed via a pointer and\n ::cudaErrorUnknown is returned.\n If \\p resource is not mapped then ::cudaErrorUnknown is returned.\n *\n \\param devPtr     - Returned pointer through which \\p resource may be accessed\n \\param size       - Returned size of the buffer accessible starting at \\p *devPtr\n \\param resource   - Mapped resource to access\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorInvalidResourceHandle,\n ::cudaErrorUnknown\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa\n ::cudaGraphicsMapResources,\n ::cudaGraphicsSubResourceGetMappedArray,\n ::cuGraphicsResourceGetMappedPointer"]
pub unsafe fn cudaGraphicsResourceGetMappedPointer(
    devPtr: *mut *mut ::std::os::raw::c_void,
    resource: cudaGraphicsResource_t,
) -> Result<usize, crate::sys::cudaError> {
    let mut out_1: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudaGraphicsResourceGetMappedPointer(
            devPtr,
            out_1.as_mut_ptr() as *mut _,
            resource,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok(out_1.assume_init()) }
    } else {
        Err(status)
    }
}
#[doc = " \\brief Get an array through which to access a subresource of a mapped graphics resource.\n\n Returns in \\p *array an array through which the subresource of the mapped\n graphics resource \\p resource which corresponds to array index \\p arrayIndex\n and mipmap level \\p mipLevel may be accessed.  The value set in \\p array may\n change every time that \\p resource is mapped.\n\n If \\p resource is not a texture then it cannot be accessed via an array and\n ::cudaErrorUnknown is returned.\n If \\p arrayIndex is not a valid array index for \\p resource then\n ::cudaErrorInvalidValue is returned.\n If \\p mipLevel is not a valid mipmap level for \\p resource then\n ::cudaErrorInvalidValue is returned.\n If \\p resource is not mapped then ::cudaErrorUnknown is returned.\n\n \\param array       - Returned array through which a subresource of \\p resource may be accessed\n \\param resource    - Mapped resource to access\n \\param arrayIndex  - Array index for array textures or cubemap face\n                      index as defined by ::cudaGraphicsCubeFace for\n                      cubemap textures for the subresource to access\n \\param mipLevel    - Mipmap level for the subresource to access\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorInvalidResourceHandle,\n ::cudaErrorUnknown\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa\n ::cudaGraphicsResourceGetMappedPointer,\n ::cuGraphicsSubResourceGetMappedArray"]
pub unsafe fn cudaGraphicsSubResourceGetMappedArray(
    resource: cudaGraphicsResource_t,
    arrayIndex: ::std::os::raw::c_uint,
    mipLevel: ::std::os::raw::c_uint,
) -> Result<cudaArray_t, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<cudaArray_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudaGraphicsSubResourceGetMappedArray(
            out_0.as_mut_ptr() as *mut _,
            resource,
            arrayIndex,
            mipLevel,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok(out_0.assume_init()) }
    } else {
        Err(status)
    }
}
#[doc = " \\brief Get a mipmapped array through which to access a mapped graphics resource.\n\n Returns in \\p *mipmappedArray a mipmapped array through which the mapped\n graphics resource \\p resource may be accessed. The value set in \\p mipmappedArray may\n change every time that \\p resource is mapped.\n\n If \\p resource is not a texture then it cannot be accessed via an array and\n ::cudaErrorUnknown is returned.\n If \\p resource is not mapped then ::cudaErrorUnknown is returned.\n\n \\param mipmappedArray - Returned mipmapped array through which \\p resource may be accessed\n \\param resource       - Mapped resource to access\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorInvalidResourceHandle,\n ::cudaErrorUnknown\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa\n ::cudaGraphicsResourceGetMappedPointer,\n ::cuGraphicsResourceGetMappedMipmappedArray"]
pub unsafe fn cudaGraphicsResourceGetMappedMipmappedArray<T: ::cuda_libs::types::CudaAsPtr>(
    mut mipmappedArray: T,
    resource: cudaGraphicsResource_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphicsResourceGetMappedMipmappedArray(
            mipmappedArray.as_mut_ptr() as *mut cudaMipmappedArray_t,
            resource,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Get the channel descriptor of an array\n\n Returns in \\p *desc the channel descriptor of the CUDA array \\p array.\n\n \\param desc  - Channel format\n \\param array - Memory array on device\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa \\ref ::cudaCreateChannelDesc(int, int, int, int, cudaChannelFormatKind) \"cudaCreateChannelDesc (C API)\",\n ::cudaCreateTextureObject, ::cudaCreateSurfaceObject"]
pub unsafe fn cudaGetChannelDesc(
    array: cudaArray_const_t,
) -> Result<cudaChannelFormatDesc, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<cudaChannelFormatDesc> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaGetChannelDesc(out_0.as_mut_ptr() as *mut _, array) };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok(out_0.assume_init()) }
    } else {
        Err(status)
    }
}
#[doc = " \\brief Returns a channel descriptor using the specified format\n\n Returns a channel descriptor with format \\p f and number of bits of each\n component \\p x, \\p y, \\p z, and \\p w.  The ::cudaChannelFormatDesc is\n defined as:\n \\code\nstruct cudaChannelFormatDesc {\nint x, y, z, w;\nenum cudaChannelFormatKind f;\n};\n \\endcode\n\n where ::cudaChannelFormatKind is one of ::cudaChannelFormatKindSigned,\n ::cudaChannelFormatKindUnsigned, or ::cudaChannelFormatKindFloat.\n\n \\param x - X component\n \\param y - Y component\n \\param z - Z component\n \\param w - W component\n \\param f - Channel format\n\n \\return\n Channel descriptor with format \\p f\n\n \\sa \\ref ::cudaCreateChannelDesc(void) \"cudaCreateChannelDesc (C++ API)\",\n ::cudaGetChannelDesc, ::cudaCreateTextureObject, ::cudaCreateSurfaceObject"]
pub unsafe fn cudaCreateChannelDesc(
    x: ::std::os::raw::c_int,
    y: ::std::os::raw::c_int,
    z: ::std::os::raw::c_int,
    w: ::std::os::raw::c_int,
    f: cudaChannelFormatKind,
) -> cudaChannelFormatDesc {
    unsafe { crate::sys::cudaCreateChannelDesc(x, y, z, w, f) }
}
#[doc = " \\brief Creates a texture object\n\n Creates a texture object and returns it in \\p pTexObject. \\p pResDesc describes\n the data to texture from. \\p pTexDesc describes how the data should be sampled.\n \\p pResViewDesc is an optional argument that specifies an alternate format for\n the data described by \\p pResDesc, and also describes the subresource region\n to restrict access to when texturing. \\p pResViewDesc can only be specified if\n the type of resource is a CUDA array or a CUDA mipmapped array not in a block\n compressed format.\n\n Texture objects are only supported on devices of compute capability 3.0 or higher.\n Additionally, a texture object is an opaque value, and, as such, should only be\n accessed through CUDA API calls.\n\n The ::cudaResourceDesc structure is defined as:\n \\code\nstruct cudaResourceDesc {\nenum cudaResourceType resType;\n\nunion {\nstruct {\ncudaArray_t array;\n} array;\nstruct {\ncudaMipmappedArray_t mipmap;\n} mipmap;\nstruct {\nvoid *devPtr;\nstruct cudaChannelFormatDesc desc;\nsize_t sizeInBytes;\n} linear;\nstruct {\nvoid *devPtr;\nstruct cudaChannelFormatDesc desc;\nsize_t width;\nsize_t height;\nsize_t pitchInBytes;\n} pitch2D;\n} res;\n};\n \\endcode\n where:\n - ::cudaResourceDesc::resType specifies the type of resource to texture from.\n CUresourceType is defined as:\n \\code\nenum cudaResourceType {\ncudaResourceTypeArray          = 0x00,\ncudaResourceTypeMipmappedArray = 0x01,\ncudaResourceTypeLinear         = 0x02,\ncudaResourceTypePitch2D        = 0x03\n};\n \\endcode\n\n \\par\n If ::cudaResourceDesc::resType is set to ::cudaResourceTypeArray, ::cudaResourceDesc::res::array::array\n must be set to a valid CUDA array handle.\n\n \\par\n If ::cudaResourceDesc::resType is set to ::cudaResourceTypeMipmappedArray, ::cudaResourceDesc::res::mipmap::mipmap\n must be set to a valid CUDA mipmapped array handle and ::cudaTextureDesc::normalizedCoords must be set to true.\n\n \\par\n If ::cudaResourceDesc::resType is set to ::cudaResourceTypeLinear, ::cudaResourceDesc::res::linear::devPtr\n must be set to a valid device pointer, that is aligned to ::cudaDeviceProp::textureAlignment.\n ::cudaResourceDesc::res::linear::desc describes the format and the number of components per array element. ::cudaResourceDesc::res::linear::sizeInBytes\n specifies the size of the array in bytes. The total number of elements in the linear address range cannot exceed\n ::cudaDeviceGetTexture1DLinearMaxWidth(). The number of elements is computed as (sizeInBytes / sizeof(desc)).\n\n \\par\n If ::cudaResourceDesc::resType is set to ::cudaResourceTypePitch2D, ::cudaResourceDesc::res::pitch2D::devPtr\n must be set to a valid device pointer, that is aligned to ::cudaDeviceProp::textureAlignment.\n ::cudaResourceDesc::res::pitch2D::desc describes the format and the number of components per array element. ::cudaResourceDesc::res::pitch2D::width\n and ::cudaResourceDesc::res::pitch2D::height specify the width and height of the array in elements, and cannot exceed\n ::cudaDeviceProp::maxTexture2DLinear[0] and ::cudaDeviceProp::maxTexture2DLinear[1] respectively.\n ::cudaResourceDesc::res::pitch2D::pitchInBytes specifies the pitch between two rows in bytes and has to be aligned to\n ::cudaDeviceProp::texturePitchAlignment. Pitch cannot exceed ::cudaDeviceProp::maxTexture2DLinear[2].\n\n\n The ::cudaTextureDesc struct is defined as\n \\code\nstruct cudaTextureDesc {\nenum cudaTextureAddressMode addressMode[3];\nenum cudaTextureFilterMode  filterMode;\nenum cudaTextureReadMode    readMode;\nint                         sRGB;\nfloat                       borderColor[4];\nint                         normalizedCoords;\nunsigned int                maxAnisotropy;\nenum cudaTextureFilterMode  mipmapFilterMode;\nfloat                       mipmapLevelBias;\nfloat                       minMipmapLevelClamp;\nfloat                       maxMipmapLevelClamp;\nint                         disableTrilinearOptimization;\nint                         seamlessCubemap;\n};\n \\endcode\n where\n - ::cudaTextureDesc::addressMode specifies the addressing mode for each dimension of the texture data. ::cudaTextureAddressMode is defined as:\n   \\code\nenum cudaTextureAddressMode {\ncudaAddressModeWrap   = 0,\ncudaAddressModeClamp  = 1,\ncudaAddressModeMirror = 2,\ncudaAddressModeBorder = 3\n};\n   \\endcode\n   This is ignored if ::cudaResourceDesc::resType is ::cudaResourceTypeLinear. Also, if ::cudaTextureDesc::normalizedCoords\n   is set to zero, ::cudaAddressModeWrap and ::cudaAddressModeMirror won't be supported and will be switched to ::cudaAddressModeClamp.\n\n - ::cudaTextureDesc::filterMode specifies the filtering mode to be used when fetching from the texture. ::cudaTextureFilterMode is defined as:\n   \\code\nenum cudaTextureFilterMode {\ncudaFilterModePoint  = 0,\ncudaFilterModeLinear = 1\n};\n   \\endcode\n   This is ignored if ::cudaResourceDesc::resType is ::cudaResourceTypeLinear.\n\n - ::cudaTextureDesc::readMode specifies whether integer data should be converted to floating point or not. ::cudaTextureReadMode is defined as:\n   \\code\nenum cudaTextureReadMode {\ncudaReadModeElementType     = 0,\ncudaReadModeNormalizedFloat = 1\n};\n   \\endcode\n   Note that this applies only to 8-bit and 16-bit integer formats. 32-bit integer format would not be promoted, regardless of\n   whether or not this ::cudaTextureDesc::readMode is set ::cudaReadModeNormalizedFloat is specified.\n\n - ::cudaTextureDesc::sRGB specifies whether sRGB to linear conversion should be performed during texture fetch.\n\n - ::cudaTextureDesc::borderColor specifies the float values of color. where:\n   ::cudaTextureDesc::borderColor[0] contains value of 'R',\n   ::cudaTextureDesc::borderColor[1] contains value of 'G',\n   ::cudaTextureDesc::borderColor[2] contains value of 'B',\n   ::cudaTextureDesc::borderColor[3] contains value of 'A'\n   Note that application using integer border color values will need to <reinterpret_cast> these values to float.\n   The values are set only when the addressing mode specified by ::cudaTextureDesc::addressMode is cudaAddressModeBorder.\n\n - ::cudaTextureDesc::normalizedCoords specifies whether the texture coordinates will be normalized or not.\n\n - ::cudaTextureDesc::maxAnisotropy specifies the maximum anistropy ratio to be used when doing anisotropic filtering. This value will be\n   clamped to the range [1,16].\n\n - ::cudaTextureDesc::mipmapFilterMode specifies the filter mode when the calculated mipmap level lies between two defined mipmap levels.\n\n - ::cudaTextureDesc::mipmapLevelBias specifies the offset to be applied to the calculated mipmap level.\n\n - ::cudaTextureDesc::minMipmapLevelClamp specifies the lower end of the mipmap level range to clamp access to.\n\n - ::cudaTextureDesc::maxMipmapLevelClamp specifies the upper end of the mipmap level range to clamp access to.\n\n - ::cudaTextureDesc::disableTrilinearOptimization specifies whether the trilinear filtering optimizations will be disabled.\n\n - ::cudaTextureDesc::seamlessCubemap specifies whether seamless cube map filtering is enabled. This flag can only be specified if the\n   underlying resource is a CUDA array or a CUDA mipmapped array that was created with the flag ::cudaArrayCubemap.\n   When seamless cube map filtering is enabled, texture address modes specified by ::cudaTextureDesc::addressMode are ignored.\n   Instead, if the ::cudaTextureDesc::filterMode is set to ::cudaFilterModePoint the address mode ::cudaAddressModeClamp will be applied for all dimensions.\n   If the ::cudaTextureDesc::filterMode is set to ::cudaFilterModeLinear seamless cube map filtering will be performed when sampling along the cube face borders.\n\n The ::cudaResourceViewDesc struct is defined as\n \\code\nstruct cudaResourceViewDesc {\nenum cudaResourceViewFormat format;\nsize_t                      width;\nsize_t                      height;\nsize_t                      depth;\nunsigned int                firstMipmapLevel;\nunsigned int                lastMipmapLevel;\nunsigned int                firstLayer;\nunsigned int                lastLayer;\n};\n \\endcode\n where:\n - ::cudaResourceViewDesc::format specifies how the data contained in the CUDA array or CUDA mipmapped array should\n   be interpreted. Note that this can incur a change in size of the texture data. If the resource view format is a block\n   compressed format, then the underlying CUDA array or CUDA mipmapped array has to have a 32-bit unsigned integer format\n   with 2 or 4 channels, depending on the block compressed format. For ex., BC1 and BC4 require the underlying CUDA array to have\n   a 32-bit unsigned int with 2 channels. The other BC formats require the underlying resource to have the same 32-bit unsigned int\n   format but with 4 channels.\n\n - ::cudaResourceViewDesc::width specifies the new width of the texture data. If the resource view format is a block\n   compressed format, this value has to be 4 times the original width of the resource. For non block compressed formats,\n   this value has to be equal to that of the original resource.\n\n - ::cudaResourceViewDesc::height specifies the new height of the texture data. If the resource view format is a block\n   compressed format, this value has to be 4 times the original height of the resource. For non block compressed formats,\n   this value has to be equal to that of the original resource.\n\n - ::cudaResourceViewDesc::depth specifies the new depth of the texture data. This value has to be equal to that of the\n   original resource.\n\n - ::cudaResourceViewDesc::firstMipmapLevel specifies the most detailed mipmap level. This will be the new mipmap level zero.\n   For non-mipmapped resources, this value has to be zero.::cudaTextureDesc::minMipmapLevelClamp and ::cudaTextureDesc::maxMipmapLevelClamp\n   will be relative to this value. For ex., if the firstMipmapLevel is set to 2, and a minMipmapLevelClamp of 1.2 is specified,\n   then the actual minimum mipmap level clamp will be 3.2.\n\n - ::cudaResourceViewDesc::lastMipmapLevel specifies the least detailed mipmap level. For non-mipmapped resources, this value\n   has to be zero.\n\n - ::cudaResourceViewDesc::firstLayer specifies the first layer index for layered textures. This will be the new layer zero.\n   For non-layered resources, this value has to be zero.\n\n - ::cudaResourceViewDesc::lastLayer specifies the last layer index for layered textures. For non-layered resources,\n   this value has to be zero.\n\n\n \\param pTexObject   - Texture object to create\n \\param pResDesc     - Resource descriptor\n \\param pTexDesc     - Texture descriptor\n \\param pResViewDesc - Resource view descriptor\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue\n \\note_init_rt\n \\note_callback\n\n \\sa\n ::cudaDestroyTextureObject,\n ::cuTexObjectCreate"]
pub unsafe fn cudaCreateTextureObject<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
    W: ::cuda_libs::types::CudaAsPtr,
>(
    mut pTexObject: T,
    pResDesc: U,
    pTexDesc: V,
    pResViewDesc: W,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaCreateTextureObject(
            pTexObject.as_mut_ptr() as *mut cudaTextureObject_t,
            pResDesc.as_const_ptr() as *const cudaResourceDesc,
            pTexDesc.as_const_ptr() as *const cudaTextureDesc,
            pResViewDesc.as_const_ptr() as *const cudaResourceViewDesc,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Destroys a texture object\n\n Destroys the texture object specified by \\p texObject.\n\n \\param texObject - Texture object to destroy\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue\n \\note_init_rt\n \\note_callback\n \\note_destroy_ub\n\n \\sa\n ::cudaCreateTextureObject,\n ::cuTexObjectDestroy"]
pub unsafe fn cudaDestroyTextureObject(
    texObject: cudaTextureObject_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaDestroyTextureObject(texObject) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Returns a texture object's resource descriptor\n\n Returns the resource descriptor for the texture object specified by \\p texObject.\n\n \\param pResDesc  - Resource descriptor\n \\param texObject - Texture object\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue\n \\note_init_rt\n \\note_callback\n\n \\sa\n ::cudaCreateTextureObject,\n ::cuTexObjectGetResourceDesc"]
pub unsafe fn cudaGetTextureObjectResourceDesc(
    texObject: cudaTextureObject_t,
) -> Result<cudaResourceDesc, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<cudaResourceDesc> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudaGetTextureObjectResourceDesc(out_0.as_mut_ptr() as *mut _, texObject)
    };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok(out_0.assume_init()) }
    } else {
        Err(status)
    }
}
#[doc = " \\brief Returns a texture object's texture descriptor\n\n Returns the texture descriptor for the texture object specified by \\p texObject.\n\n \\param pTexDesc  - Texture descriptor\n \\param texObject - Texture object\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue\n \\note_init_rt\n \\note_callback\n\n \\sa\n ::cudaCreateTextureObject,\n ::cuTexObjectGetTextureDesc"]
pub unsafe fn cudaGetTextureObjectTextureDesc(
    texObject: cudaTextureObject_t,
) -> Result<cudaTextureDesc, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<cudaTextureDesc> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudaGetTextureObjectTextureDesc(out_0.as_mut_ptr() as *mut _, texObject)
    };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok(out_0.assume_init()) }
    } else {
        Err(status)
    }
}
#[doc = " \\brief Returns a texture object's resource view descriptor\n\n Returns the resource view descriptor for the texture object specified by \\p texObject.\n If no resource view was specified, ::cudaErrorInvalidValue is returned.\n\n \\param pResViewDesc - Resource view descriptor\n \\param texObject    - Texture object\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue\n \\note_init_rt\n \\note_callback\n\n \\sa\n ::cudaCreateTextureObject,\n ::cuTexObjectGetResourceViewDesc"]
pub unsafe fn cudaGetTextureObjectResourceViewDesc(
    texObject: cudaTextureObject_t,
) -> Result<cudaResourceViewDesc, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<cudaResourceViewDesc> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudaGetTextureObjectResourceViewDesc(out_0.as_mut_ptr() as *mut _, texObject)
    };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok(out_0.assume_init()) }
    } else {
        Err(status)
    }
}
#[doc = " \\brief Creates a surface object\n\n Creates a surface object and returns it in \\p pSurfObject. \\p pResDesc describes\n the data to perform surface load/stores on. ::cudaResourceDesc::resType must be\n ::cudaResourceTypeArray and  ::cudaResourceDesc::res::array::array\n must be set to a valid CUDA array handle.\n\n Surface objects are only supported on devices of compute capability 3.0 or higher.\n Additionally, a surface object is an opaque value, and, as such, should only be\n accessed through CUDA API calls.\n\n \\param pSurfObject - Surface object to create\n \\param pResDesc    - Resource descriptor\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorInvalidChannelDescriptor,\n ::cudaErrorInvalidResourceHandle\n \\note_init_rt\n \\note_callback\n\n \\sa\n ::cudaDestroySurfaceObject,\n ::cuSurfObjectCreate"]
pub unsafe fn cudaCreateSurfaceObject<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
>(
    mut pSurfObject: T,
    pResDesc: U,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaCreateSurfaceObject(
            pSurfObject.as_mut_ptr() as *mut cudaSurfaceObject_t,
            pResDesc.as_const_ptr() as *const cudaResourceDesc,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Destroys a surface object\n\n Destroys the surface object specified by \\p surfObject.\n\n \\param surfObject - Surface object to destroy\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue\n \\note_init_rt\n \\note_callback\n \\note_destroy_ub\n\n \\sa\n ::cudaCreateSurfaceObject,\n ::cuSurfObjectDestroy"]
pub unsafe fn cudaDestroySurfaceObject(
    surfObject: cudaSurfaceObject_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaDestroySurfaceObject(surfObject) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Returns a surface object's resource descriptor\n Returns the resource descriptor for the surface object specified by \\p surfObject.\n\n \\param pResDesc   - Resource descriptor\n \\param surfObject - Surface object\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue\n \\note_init_rt\n \\note_callback\n\n \\sa\n ::cudaCreateSurfaceObject,\n ::cuSurfObjectGetResourceDesc"]
pub unsafe fn cudaGetSurfaceObjectResourceDesc(
    surfObject: cudaSurfaceObject_t,
) -> Result<cudaResourceDesc, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<cudaResourceDesc> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudaGetSurfaceObjectResourceDesc(out_0.as_mut_ptr() as *mut _, surfObject)
    };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok(out_0.assume_init()) }
    } else {
        Err(status)
    }
}
#[doc = " \\brief Returns the latest version of CUDA supported by the driver\n\n Returns in \\p *driverVersion the latest version of CUDA supported by\n the driver. The version is returned as (1000 &times; major + 10 &times; minor).\n For example, CUDA 9.2 would be represented by 9020. If no driver is installed,\n then 0 is returned as the driver version.\n\n This function automatically returns ::cudaErrorInvalidValue\n if \\p driverVersion is NULL.\n\n \\param driverVersion - Returns the CUDA driver version.\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa\n ::cudaRuntimeGetVersion,\n ::cuDriverGetVersion"]
pub unsafe fn cudaDriverGetVersion() -> Result<::std::os::raw::c_int, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaDriverGetVersion(out_0.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok(out_0.assume_init()) }
    } else {
        Err(status)
    }
}
#[doc = " \\brief Returns the CUDA Runtime version\n\n Returns in \\p *runtimeVersion the version number of the current CUDA\n Runtime instance. The version is returned as\n (1000 &times; major + 10 &times; minor). For example,\n CUDA 9.2 would be represented by 9020.\n\n As of CUDA 12.0, this function no longer initializes CUDA. The purpose\n of this API is solely to return a compile-time constant stating the\n CUDA Toolkit version in the above format.\n\n This function automatically returns ::cudaErrorInvalidValue if\n the \\p runtimeVersion argument is NULL.\n\n \\param runtimeVersion - Returns the CUDA Runtime version.\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue\n \\note_init_rt\n \\note_callback\n\n \\sa\n ::cudaDriverGetVersion,\n ::cuDriverGetVersion"]
pub unsafe fn cudaRuntimeGetVersion() -> Result<::std::os::raw::c_int, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaRuntimeGetVersion(out_0.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok(out_0.assume_init()) }
    } else {
        Err(status)
    }
}
#[doc = " \\brief Register a callback function to receive error log messages\n\n \\param callbackFunc  - The function to register as a callback\n \\param userData      - A generic pointer to user data. This is passed into the callback function.\n \\param callback_out  - Optional location to store the callback handle after it is registered\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,"]
pub unsafe fn cudaLogsRegisterCallback<T: ::cuda_libs::types::CudaAsPtr>(
    callbackFunc: cudaLogsCallback_t,
    mut userData: T,
    callback_out: *mut cudaLogsCallbackHandle,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaLogsRegisterCallback(
            callbackFunc,
            userData.as_mut_ptr() as *mut ::std::os::raw::c_void,
            callback_out,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Unregister a log message callback\n\n \\param callback  - The callback instance to unregister from receiving log messages\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,"]
pub unsafe fn cudaLogsUnregisterCallback(
    callback: cudaLogsCallbackHandle,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaLogsUnregisterCallback(callback) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Sets log iterator to point to the end of log buffer, where the next message would be written.\n\n \\param iterator_out - Location to store an iterator to the current tail of the logs\n \\param flags        - Reserved for future use, must be 0\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,"]
pub unsafe fn cudaLogsCurrent<T: ::cuda_libs::types::CudaAsPtr>(
    mut iterator_out: T,
    flags: ::std::os::raw::c_uint,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaLogsCurrent(iterator_out.as_mut_ptr() as *mut cudaLogIterator, flags)
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Dump accumulated driver logs into a file\n\n Logs generated by the driver are stored in an internal buffer and can be copied out using this API.\n This API dumps all driver logs starting from \\p iterator into \\p pathToFile provided.\n\n \\note \\p iterator is auto-advancing. Dumping logs will update the value of\n       \\p iterator to receive the next generated log.\n\n \\note The driver reserves limited memory for storing logs.\n       The oldest logs may be overwritten and become unrecoverable. An indication will appear in the\n       destination outupt if the logs have been truncated. Call dump after each failed API to mitigate this\n       risk.\n\n \\param iterator   - Optional auto-advancing iterator specifying the starting log to read. NULL value dumps all logs.\n \\param pathToFile - Path to output file for dumping logs\n \\param flags      - Reserved for future use, must be 0\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,"]
pub unsafe fn cudaLogsDumpToFile<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
>(
    mut iterator: T,
    pathToFile: U,
    flags: ::std::os::raw::c_uint,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaLogsDumpToFile(
            iterator.as_mut_ptr() as *mut cudaLogIterator,
            pathToFile.as_const_ptr() as *const ::std::os::raw::c_char,
            flags,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Dump accumulated driver logs into a buffer\n\n Logs generated by the driver are stored in an internal buffer and can be copied out using this API.\n This API dumps driver logs from \\p iterator into \\p buffer up to the size specified in \\p *size.\n The driver will always null terminate the buffer but there will not be a null character between log\n entries, only a newline \\\\n. The driver will then return the actual number of bytes written in\n \\p *size, excluding the null terminator. If there are no messages to dump, \\p *size will be set to 0\n and the function will return ::CUDA_SUCCESS.\n If the provided \\p buffer is not large enough to hold any messages, \\p *size will be set to 0 and\n the function will return ::CUDA_ERROR_INVALID_VALUE.\n\n \\note \\p iterator is auto-advancing. Dumping logs will update the value of\n       \\p iterator to receive the next generated log.\n\n \\note The driver reserves limited memory for storing logs. The maximum size of the buffer is 25600 bytes.\n       The oldest logs may be overwritten and become unrecoverable. An indication will appear in the\n       destination outupt if the logs have been truncated. Call dump after each failed API to mitigate this\n       risk.\n\n \\note If the provided value in \\p *size is not large enough to hold all buffered messages, a message will\n       be added at the head of the buffer indicating this. The driver then computes the number of messages\n       it is able to store in \\p buffer and writes it out. The final message in \\p buffer will always be\n       the most recent log message as of when the API is called.\n\n \\param iterator  - Optional auto-advancing iterator specifying the starting log to read. NULL value dumps all logs.\n \\param buffer    - Pointer to dump logs\n \\param size      - See description\n \\param flags     - Reserved for future use, must be 0\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,"]
pub unsafe fn cudaLogsDumpToMemory<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
>(
    mut iterator: T,
    mut buffer: U,
    mut size: V,
    flags: ::std::os::raw::c_uint,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaLogsDumpToMemory(
            iterator.as_mut_ptr() as *mut cudaLogIterator,
            buffer.as_mut_ptr() as *mut ::std::os::raw::c_char,
            size.as_mut_ptr() as *mut usize,
            flags,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Creates a graph\n\n Creates an empty graph, which is returned via \\p pGraph.\n\n \\param pGraph - Returns newly created graph\n \\param flags   - Graph creation flags, must be 0\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorMemoryAllocation\n \\note_graph_thread_safety\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa\n ::cudaGraphAddChildGraphNode,\n ::cudaGraphAddEmptyNode,\n ::cudaGraphAddKernelNode,\n ::cudaGraphAddHostNode,\n ::cudaGraphAddMemcpyNode,\n ::cudaGraphAddMemsetNode,\n ::cudaGraphInstantiate,\n ::cudaGraphDestroy,\n ::cudaGraphGetNodes,\n ::cudaGraphGetRootNodes,\n ::cudaGraphGetEdges,\n ::cudaGraphClone"]
pub unsafe fn cudaGraphCreate<T: ::cuda_libs::types::CudaAsPtr>(
    mut pGraph: T,
    flags: ::std::os::raw::c_uint,
) -> Result<(), crate::sys::cudaError> {
    let status =
        unsafe { crate::sys::cudaGraphCreate(pGraph.as_mut_ptr() as *mut cudaGraph_t, flags) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Creates a kernel execution node and adds it to a graph\n\n Creates a new kernel execution node and adds it to \\p graph with \\p numDependencies\n dependencies specified via \\p pDependencies and arguments specified in \\p pNodeParams.\n It is possible for \\p numDependencies to be 0, in which case the node will be placed\n at the root of the graph. \\p pDependencies may not have any duplicate entries.\n A handle to the new node will be returned in \\p pGraphNode.\n\n The cudaKernelNodeParams structure is defined as:\n\n \\code\n  struct cudaKernelNodeParams\n  {\n      void* func;\n      dim3 gridDim;\n      dim3 blockDim;\n      unsigned int sharedMemBytes;\n      void **kernelParams;\n      void **extra;\n  };\n \\endcode\n\n When the graph is launched, the node will invoke kernel \\p func on a (\\p gridDim.x x\n \\p gridDim.y x \\p gridDim.z) grid of blocks. Each block contains\n (\\p blockDim.x x \\p blockDim.y x \\p blockDim.z) threads.\n\n \\p sharedMem sets the amount of dynamic shared memory that will be\n available to each thread block.\n\n Kernel parameters to \\p func can be specified in one of two ways:\n\n 1) Kernel parameters can be specified via \\p kernelParams. If the kernel has N\n parameters, then \\p kernelParams needs to be an array of N pointers. Each pointer,\n from \\p kernelParams[0] to \\p kernelParams[N-1], points to the region of memory from which the actual\n parameter will be copied. The number of kernel parameters and their offsets and sizes do not need\n to be specified as that information is retrieved directly from the kernel's image.\n\n 2) Kernel parameters can also be packaged by the application into a single buffer that is passed in\n via \\p extra. This places the burden on the application of knowing each kernel\n parameter's size and alignment/padding within the buffer. The \\p extra parameter exists\n to allow this function to take additional less commonly used arguments. \\p extra specifies\n a list of names of extra settings and their corresponding values. Each extra setting name is\n immediately followed by the corresponding value. The list must be terminated with either NULL or\n CU_LAUNCH_PARAM_END.\n\n - ::CU_LAUNCH_PARAM_END, which indicates the end of the \\p extra\n   array;\n - ::CU_LAUNCH_PARAM_BUFFER_POINTER, which specifies that the next\n   value in \\p extra will be a pointer to a buffer\n   containing all the kernel parameters for launching kernel\n   \\p func;\n - ::CU_LAUNCH_PARAM_BUFFER_SIZE, which specifies that the next\n   value in \\p extra will be a pointer to a size_t\n   containing the size of the buffer specified with\n   ::CU_LAUNCH_PARAM_BUFFER_POINTER;\n\n The error ::cudaErrorInvalidValue will be returned if kernel parameters are specified with both\n \\p kernelParams and \\p extra (i.e. both \\p kernelParams and\n \\p extra are non-NULL).\n\n The \\p kernelParams or \\p extra array, as well as the argument values it points to,\n are copied during this call.\n\n \\note Kernels launched using graphs must not use texture and surface references. Reading or\n       writing through any texture or surface reference is undefined behavior.\n       This restriction does not apply to texture and surface objects.\n\n \\param pGraphNode     - Returns newly created node\n \\param graph          - Graph to which to add the node\n \\param pDependencies    - Dependencies of the node\n \\param numDependencies - Number of dependencies\n \\param pNodeParams      - Parameters for the GPU execution node\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorInvalidDeviceFunction\n \\note_graph_thread_safety\n \\notefnerr\n \\note_init_rt\n \\note_callback\n \\note_cudaKernel_t\n\n \\sa\n ::cudaGraphAddNode,\n ::cudaLaunchKernel,\n ::cudaGraphKernelNodeGetParams,\n ::cudaGraphKernelNodeSetParams,\n ::cudaGraphCreate,\n ::cudaGraphDestroyNode,\n ::cudaGraphAddChildGraphNode,\n ::cudaGraphAddEmptyNode,\n ::cudaGraphAddHostNode,\n ::cudaGraphAddMemcpyNode,\n ::cudaGraphAddMemsetNode"]
pub unsafe fn cudaGraphAddKernelNode<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
>(
    mut pGraphNode: T,
    graph: cudaGraph_t,
    pDependencies: U,
    numDependencies: usize,
    pNodeParams: V,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphAddKernelNode(
            pGraphNode.as_mut_ptr() as *mut cudaGraphNode_t,
            graph,
            pDependencies.as_const_ptr() as *const cudaGraphNode_t,
            numDependencies,
            pNodeParams.as_const_ptr() as *const cudaKernelNodeParams,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Returns a kernel node's parameters\n\n Returns the parameters of kernel node \\p node in \\p pNodeParams.\n The \\p kernelParams or \\p extra array returned in \\p pNodeParams,\n as well as the argument values it points to, are owned by the node.\n This memory remains valid until the node is destroyed or its\n parameters are modified, and should not be modified\n directly. Use ::cudaGraphKernelNodeSetParams to update the\n parameters of this node.\n\n The params will contain either \\p kernelParams or \\p extra,\n according to which of these was most recently set on the node.\n\n \\param node        - Node to get the parameters for\n \\param pNodeParams - Pointer to return the parameters\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorInvalidDeviceFunction\n \\note_graph_thread_safety\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa\n ::cudaGraphNodeGetParams,\n ::cudaLaunchKernel,\n ::cudaGraphAddKernelNode,\n ::cudaGraphKernelNodeSetParams"]
pub unsafe fn cudaGraphKernelNodeGetParams(
    node: cudaGraphNode_t,
) -> Result<cudaKernelNodeParams, crate::sys::cudaError> {
    let mut out_1: std::mem::MaybeUninit<cudaKernelNodeParams> = std::mem::MaybeUninit::uninit();
    let status =
        unsafe { crate::sys::cudaGraphKernelNodeGetParams(node, out_1.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok(out_1.assume_init()) }
    } else {
        Err(status)
    }
}
#[doc = " \\brief Sets a kernel node's parameters\n\n Sets the parameters of kernel node \\p node to \\p pNodeParams.\n\n \\param node        - Node to set the parameters for\n \\param pNodeParams - Parameters to copy\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorInvalidResourceHandle,\n ::cudaErrorMemoryAllocation\n \\note_graph_thread_safety\n \\notefnerr\n \\note_init_rt\n \\note_callback\n \\note_cudaKernel_t\n\n \\sa\n ::cudaGraphNodeSetParams,\n ::cudaLaunchKernel,\n ::cudaGraphAddKernelNode,\n ::cudaGraphKernelNodeGetParams"]
pub unsafe fn cudaGraphKernelNodeSetParams<T: ::cuda_libs::types::CudaAsPtr>(
    node: cudaGraphNode_t,
    pNodeParams: T,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphKernelNodeSetParams(
            node,
            pNodeParams.as_const_ptr() as *const cudaKernelNodeParams,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Copies attributes from source node to destination node.\n\n Copies attributes from source node \\p hSrc to destination node \\p hDst.\n Both node must have the same context.\n\n \\param[out] hDst Destination node\n \\param[in] hSrc Source node\n For list of attributes see ::cudaKernelNodeAttrID\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidContext\n \\notefnerr\n\n \\sa\n ::cudaAccessPolicyWindow"]
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
#[doc = " \\brief Queries node attribute.\n\n Queries attribute \\p attr from node \\p hNode and stores it in corresponding\n member of \\p value_out.\n\n \\param[in] hNode\n \\param[in] attr\n \\param[out] value_out\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorInvalidResourceHandle\n \\notefnerr\n\n \\sa\n ::cudaAccessPolicyWindow"]
pub unsafe fn cudaGraphKernelNodeGetAttribute(
    hNode: cudaGraphNode_t,
    attr: cudaLaunchAttributeID,
) -> Result<cudaLaunchAttributeValue, crate::sys::cudaError> {
    let mut out_2: std::mem::MaybeUninit<cudaLaunchAttributeValue> =
        std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudaGraphKernelNodeGetAttribute(hNode, attr, out_2.as_mut_ptr() as *mut _)
    };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok(out_2.assume_init()) }
    } else {
        Err(status)
    }
}
#[doc = " \\brief Sets node attribute.\n\n Sets attribute \\p attr on node \\p hNode from corresponding attribute of\n \\p value.\n\n \\param[out] hNode\n \\param[in] attr\n \\param[out] value\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorInvalidResourceHandle\n \\notefnerr\n\n \\sa\n ::cudaAccessPolicyWindow"]
pub unsafe fn cudaGraphKernelNodeSetAttribute<T: ::cuda_libs::types::CudaAsPtr>(
    hNode: cudaGraphNode_t,
    attr: cudaLaunchAttributeID,
    value: T,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphKernelNodeSetAttribute(
            hNode,
            attr,
            value.as_const_ptr() as *const cudaLaunchAttributeValue,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Creates a memcpy node and adds it to a graph\n\n Creates a new memcpy node and adds it to \\p graph with \\p numDependencies\n dependencies specified via \\p pDependencies.\n It is possible for \\p numDependencies to be 0, in which case the node will be placed\n at the root of the graph. \\p pDependencies may not have any duplicate entries.\n A handle to the new node will be returned in \\p pGraphNode.\n\n When the graph is launched, the node will perform the memcpy described by \\p pCopyParams.\n See ::cudaMemcpy3D() for a description of the structure and its restrictions.\n\n Memcpy nodes have some additional restrictions with regards to managed memory, if the\n system contains at least one device which has a zero value for the device attribute\n ::cudaDevAttrConcurrentManagedAccess.\n\n \\param pGraphNode     - Returns newly created node\n \\param graph          - Graph to which to add the node\n \\param pDependencies    - Dependencies of the node\n \\param numDependencies - Number of dependencies\n \\param pCopyParams      - Parameters for the memory copy\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue\n \\note_graph_thread_safety\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa\n ::cudaGraphAddNode,\n ::cudaMemcpy3D,\n ::cudaGraphAddMemcpyNodeToSymbol,\n ::cudaGraphAddMemcpyNodeFromSymbol,\n ::cudaGraphAddMemcpyNode1D,\n ::cudaGraphMemcpyNodeGetParams,\n ::cudaGraphMemcpyNodeSetParams,\n ::cudaGraphCreate,\n ::cudaGraphDestroyNode,\n ::cudaGraphAddChildGraphNode,\n ::cudaGraphAddEmptyNode,\n ::cudaGraphAddKernelNode,\n ::cudaGraphAddHostNode,\n ::cudaGraphAddMemsetNode"]
pub unsafe fn cudaGraphAddMemcpyNode<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
>(
    mut pGraphNode: T,
    graph: cudaGraph_t,
    pDependencies: U,
    numDependencies: usize,
    pCopyParams: V,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphAddMemcpyNode(
            pGraphNode.as_mut_ptr() as *mut cudaGraphNode_t,
            graph,
            pDependencies.as_const_ptr() as *const cudaGraphNode_t,
            numDependencies,
            pCopyParams.as_const_ptr() as *const cudaMemcpy3DParms,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudaGraphAddMemcpyNodeToSymbol<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
    W: ::cuda_libs::types::CudaAsPtr,
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
            pGraphNode.as_mut_ptr() as *mut cudaGraphNode_t,
            graph,
            pDependencies.as_const_ptr() as *const cudaGraphNode_t,
            numDependencies,
            symbol.as_const_ptr() as *const ::std::os::raw::c_void,
            src.as_const_ptr() as *const ::std::os::raw::c_void,
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
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
    W: ::cuda_libs::types::CudaAsPtr,
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
            pGraphNode.as_mut_ptr() as *mut cudaGraphNode_t,
            graph,
            pDependencies.as_const_ptr() as *const cudaGraphNode_t,
            numDependencies,
            dst.as_mut_ptr() as *mut ::std::os::raw::c_void,
            symbol.as_const_ptr() as *const ::std::os::raw::c_void,
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
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
    W: ::cuda_libs::types::CudaAsPtr,
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
            pGraphNode.as_mut_ptr() as *mut cudaGraphNode_t,
            graph,
            pDependencies.as_const_ptr() as *const cudaGraphNode_t,
            numDependencies,
            dst.as_mut_ptr() as *mut ::std::os::raw::c_void,
            src.as_const_ptr() as *const ::std::os::raw::c_void,
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
#[doc = " \\brief Returns a memcpy node's parameters\n\n Returns the parameters of memcpy node \\p node in \\p pNodeParams.\n\n \\param node        - Node to get the parameters for\n \\param pNodeParams - Pointer to return the parameters\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue\n \\note_graph_thread_safety\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa\n ::cudaGraphNodeGetParams,\n ::cudaMemcpy3D,\n ::cudaGraphAddMemcpyNode,\n ::cudaGraphMemcpyNodeSetParams"]
pub unsafe fn cudaGraphMemcpyNodeGetParams<T: ::cuda_libs::types::CudaAsPtr>(
    node: cudaGraphNode_t,
    mut pNodeParams: T,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphMemcpyNodeGetParams(
            node,
            pNodeParams.as_mut_ptr() as *mut cudaMemcpy3DParms,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Sets a memcpy node's parameters\n\n Sets the parameters of memcpy node \\p node to \\p pNodeParams.\n\n \\param node        - Node to set the parameters for\n \\param pNodeParams - Parameters to copy\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n \\note_graph_thread_safety\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa\n ::cudaGraphNodeSetParams,\n ::cudaMemcpy3D,\n ::cudaGraphMemcpyNodeSetParamsToSymbol,\n ::cudaGraphMemcpyNodeSetParamsFromSymbol,\n ::cudaGraphMemcpyNodeSetParams1D,\n ::cudaGraphAddMemcpyNode,\n ::cudaGraphMemcpyNodeGetParams"]
pub unsafe fn cudaGraphMemcpyNodeSetParams<T: ::cuda_libs::types::CudaAsPtr>(
    node: cudaGraphNode_t,
    pNodeParams: T,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphMemcpyNodeSetParams(
            node,
            pNodeParams.as_const_ptr() as *const cudaMemcpy3DParms,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudaGraphMemcpyNodeSetParamsToSymbol<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
>(
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
            symbol.as_const_ptr() as *const ::std::os::raw::c_void,
            src.as_const_ptr() as *const ::std::os::raw::c_void,
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
pub unsafe fn cudaGraphMemcpyNodeSetParamsFromSymbol<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
>(
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
            dst.as_mut_ptr() as *mut ::std::os::raw::c_void,
            symbol.as_const_ptr() as *const ::std::os::raw::c_void,
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
pub unsafe fn cudaGraphMemcpyNodeSetParams1D<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
>(
    node: cudaGraphNode_t,
    mut dst: T,
    src: U,
    count: usize,
    kind: cudaMemcpyKind,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphMemcpyNodeSetParams1D(
            node,
            dst.as_mut_ptr() as *mut ::std::os::raw::c_void,
            src.as_const_ptr() as *const ::std::os::raw::c_void,
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
#[doc = " \\brief Creates a memset node and adds it to a graph\n\n Creates a new memset node and adds it to \\p graph with \\p numDependencies\n dependencies specified via \\p pDependencies.\n It is possible for \\p numDependencies to be 0, in which case the node will be placed\n at the root of the graph. \\p pDependencies may not have any duplicate entries.\n A handle to the new node will be returned in \\p pGraphNode.\n\n The element size must be 1, 2, or 4 bytes.\n When the graph is launched, the node will perform the memset described by \\p pMemsetParams.\n\n \\param pGraphNode     - Returns newly created node\n \\param graph          - Graph to which to add the node\n \\param pDependencies    - Dependencies of the node\n \\param numDependencies - Number of dependencies\n \\param pMemsetParams    - Parameters for the memory set\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorInvalidDevice\n \\note_graph_thread_safety\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa\n ::cudaGraphAddNode,\n ::cudaMemset2D,\n ::cudaGraphMemsetNodeGetParams,\n ::cudaGraphMemsetNodeSetParams,\n ::cudaGraphCreate,\n ::cudaGraphDestroyNode,\n ::cudaGraphAddChildGraphNode,\n ::cudaGraphAddEmptyNode,\n ::cudaGraphAddKernelNode,\n ::cudaGraphAddHostNode,\n ::cudaGraphAddMemcpyNode"]
pub unsafe fn cudaGraphAddMemsetNode<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
>(
    mut pGraphNode: T,
    graph: cudaGraph_t,
    pDependencies: U,
    numDependencies: usize,
    pMemsetParams: V,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphAddMemsetNode(
            pGraphNode.as_mut_ptr() as *mut cudaGraphNode_t,
            graph,
            pDependencies.as_const_ptr() as *const cudaGraphNode_t,
            numDependencies,
            pMemsetParams.as_const_ptr() as *const cudaMemsetParams,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Returns a memset node's parameters\n\n Returns the parameters of memset node \\p node in \\p pNodeParams.\n\n \\param node        - Node to get the parameters for\n \\param pNodeParams - Pointer to return the parameters\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue\n \\note_graph_thread_safety\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa\n ::cudaGraphNodeGetParams,\n ::cudaMemset2D,\n ::cudaGraphAddMemsetNode,\n ::cudaGraphMemsetNodeSetParams"]
pub unsafe fn cudaGraphMemsetNodeGetParams(
    node: cudaGraphNode_t,
) -> Result<cudaMemsetParams, crate::sys::cudaError> {
    let mut out_1: std::mem::MaybeUninit<cudaMemsetParams> = std::mem::MaybeUninit::uninit();
    let status =
        unsafe { crate::sys::cudaGraphMemsetNodeGetParams(node, out_1.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok(out_1.assume_init()) }
    } else {
        Err(status)
    }
}
#[doc = " \\brief Sets a memset node's parameters\n\n Sets the parameters of memset node \\p node to \\p pNodeParams.\n\n \\param node        - Node to set the parameters for\n \\param pNodeParams - Parameters to copy\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue\n \\note_graph_thread_safety\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa\n ::cudaGraphNodeSetParams,\n ::cudaMemset2D,\n ::cudaGraphAddMemsetNode,\n ::cudaGraphMemsetNodeGetParams"]
pub unsafe fn cudaGraphMemsetNodeSetParams<T: ::cuda_libs::types::CudaAsPtr>(
    node: cudaGraphNode_t,
    pNodeParams: T,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphMemsetNodeSetParams(
            node,
            pNodeParams.as_const_ptr() as *const cudaMemsetParams,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Creates a host execution node and adds it to a graph\n\n Creates a new CPU execution node and adds it to \\p graph with \\p numDependencies\n dependencies specified via \\p pDependencies and arguments specified in \\p pNodeParams.\n It is possible for \\p numDependencies to be 0, in which case the node will be placed\n at the root of the graph. \\p pDependencies may not have any duplicate entries.\n A handle to the new node will be returned in \\p pGraphNode.\n\n When the graph is launched, the node will invoke the specified CPU function.\n Host nodes are not supported under MPS with pre-Volta GPUs.\n\n \\param pGraphNode     - Returns newly created node\n \\param graph          - Graph to which to add the node\n \\param pDependencies    - Dependencies of the node\n \\param numDependencies - Number of dependencies\n \\param pNodeParams      - Parameters for the host node\n\n \\return\n ::cudaSuccess,\n ::cudaErrorNotSupported,\n ::cudaErrorInvalidValue\n \\note_graph_thread_safety\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa\n ::cudaGraphAddNode,\n ::cudaLaunchHostFunc,\n ::cudaGraphHostNodeGetParams,\n ::cudaGraphHostNodeSetParams,\n ::cudaGraphCreate,\n ::cudaGraphDestroyNode,\n ::cudaGraphAddChildGraphNode,\n ::cudaGraphAddEmptyNode,\n ::cudaGraphAddKernelNode,\n ::cudaGraphAddMemcpyNode,\n ::cudaGraphAddMemsetNode"]
pub unsafe fn cudaGraphAddHostNode<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
>(
    mut pGraphNode: T,
    graph: cudaGraph_t,
    pDependencies: U,
    numDependencies: usize,
    pNodeParams: V,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphAddHostNode(
            pGraphNode.as_mut_ptr() as *mut cudaGraphNode_t,
            graph,
            pDependencies.as_const_ptr() as *const cudaGraphNode_t,
            numDependencies,
            pNodeParams.as_const_ptr() as *const cudaHostNodeParams,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Returns a host node's parameters\n\n Returns the parameters of host node \\p node in \\p pNodeParams.\n\n \\param node        - Node to get the parameters for\n \\param pNodeParams - Pointer to return the parameters\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue\n \\note_graph_thread_safety\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa\n ::cudaGraphNodeGetParams,\n ::cudaLaunchHostFunc,\n ::cudaGraphAddHostNode,\n ::cudaGraphHostNodeSetParams"]
pub unsafe fn cudaGraphHostNodeGetParams(
    node: cudaGraphNode_t,
) -> Result<cudaHostNodeParams, crate::sys::cudaError> {
    let mut out_1: std::mem::MaybeUninit<cudaHostNodeParams> = std::mem::MaybeUninit::uninit();
    let status =
        unsafe { crate::sys::cudaGraphHostNodeGetParams(node, out_1.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok(out_1.assume_init()) }
    } else {
        Err(status)
    }
}
#[doc = " \\brief Sets a host node's parameters\n\n Sets the parameters of host node \\p node to \\p nodeParams.\n\n \\param node        - Node to set the parameters for\n \\param pNodeParams - Parameters to copy\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue\n \\note_graph_thread_safety\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa\n ::cudaGraphNodeSetParams,\n ::cudaLaunchHostFunc,\n ::cudaGraphAddHostNode,\n ::cudaGraphHostNodeGetParams"]
pub unsafe fn cudaGraphHostNodeSetParams<T: ::cuda_libs::types::CudaAsPtr>(
    node: cudaGraphNode_t,
    pNodeParams: T,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphHostNodeSetParams(
            node,
            pNodeParams.as_const_ptr() as *const cudaHostNodeParams,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Creates a child graph node and adds it to a graph\n\n Creates a new node which executes an embedded graph, and adds it to \\p graph with\n \\p numDependencies dependencies specified via \\p pDependencies.\n It is possible for \\p numDependencies to be 0, in which case the node will be placed\n at the root of the graph. \\p pDependencies may not have any duplicate entries.\n A handle to the new node will be returned in \\p pGraphNode.\n\n If \\p childGraph contains allocation nodes, free nodes, or conditional nodes, this call will\n return an error.\n\n The node executes an embedded child graph. The child graph is cloned in this call.\n\n \\param pGraphNode     - Returns newly created node\n \\param graph          - Graph to which to add the node\n \\param pDependencies    - Dependencies of the node\n \\param numDependencies - Number of dependencies\n \\param childGraph      - The graph to clone into this node\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue\n \\note_graph_thread_safety\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa\n ::cudaGraphAddNode,\n ::cudaGraphChildGraphNodeGetGraph,\n ::cudaGraphCreate,\n ::cudaGraphDestroyNode,\n ::cudaGraphAddEmptyNode,\n ::cudaGraphAddKernelNode,\n ::cudaGraphAddHostNode,\n ::cudaGraphAddMemcpyNode,\n ::cudaGraphAddMemsetNode,\n ::cudaGraphClone"]
pub unsafe fn cudaGraphAddChildGraphNode<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
>(
    mut pGraphNode: T,
    graph: cudaGraph_t,
    pDependencies: U,
    numDependencies: usize,
    childGraph: cudaGraph_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphAddChildGraphNode(
            pGraphNode.as_mut_ptr() as *mut cudaGraphNode_t,
            graph,
            pDependencies.as_const_ptr() as *const cudaGraphNode_t,
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
#[doc = " \\brief Gets a handle to the embedded graph of a child graph node\n\n Gets a handle to the embedded graph in a child graph node. This call\n does not clone the graph. Changes to the graph will be reflected in\n the node, and the node retains ownership of the graph.\n\n Allocation and free nodes cannot be added to the returned graph.\n Attempting to do so will return an error.\n\n \\param node   - Node to get the embedded graph for\n \\param pGraph - Location to store a handle to the graph\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue\n \\note_graph_thread_safety\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa\n ::cudaGraphAddChildGraphNode,\n ::cudaGraphNodeFindInClone"]
pub unsafe fn cudaGraphChildGraphNodeGetGraph(
    node: cudaGraphNode_t,
) -> Result<cudaGraph_t, crate::sys::cudaError> {
    let mut out_1: std::mem::MaybeUninit<cudaGraph_t> = std::mem::MaybeUninit::uninit();
    let status =
        unsafe { crate::sys::cudaGraphChildGraphNodeGetGraph(node, out_1.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok(out_1.assume_init()) }
    } else {
        Err(status)
    }
}
#[doc = " \\brief Creates an empty node and adds it to a graph\n\n Creates a new node which performs no operation, and adds it to \\p graph with\n \\p numDependencies dependencies specified via \\p pDependencies.\n It is possible for \\p numDependencies to be 0, in which case the node will be placed\n at the root of the graph. \\p pDependencies may not have any duplicate entries.\n A handle to the new node will be returned in \\p pGraphNode.\n\n An empty node performs no operation during execution, but can be used for\n transitive ordering. For example, a phased execution graph with 2 groups of n\n nodes with a barrier between them can be represented using an empty node and\n 2*n dependency edges, rather than no empty node and n^2 dependency edges.\n\n \\param pGraphNode     - Returns newly created node\n \\param graph          - Graph to which to add the node\n \\param pDependencies    - Dependencies of the node\n \\param numDependencies - Number of dependencies\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue\n \\note_graph_thread_safety\n \\note_init_rt\n \\note_callback\n\n \\sa\n ::cudaGraphAddNode,\n ::cudaGraphCreate,\n ::cudaGraphDestroyNode,\n ::cudaGraphAddChildGraphNode,\n ::cudaGraphAddKernelNode,\n ::cudaGraphAddHostNode,\n ::cudaGraphAddMemcpyNode,\n ::cudaGraphAddMemsetNode"]
pub unsafe fn cudaGraphAddEmptyNode<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
>(
    mut pGraphNode: T,
    graph: cudaGraph_t,
    pDependencies: U,
    numDependencies: usize,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphAddEmptyNode(
            pGraphNode.as_mut_ptr() as *mut cudaGraphNode_t,
            graph,
            pDependencies.as_const_ptr() as *const cudaGraphNode_t,
            numDependencies,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudaGraphAddEventRecordNode<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
>(
    mut pGraphNode: T,
    graph: cudaGraph_t,
    pDependencies: U,
    numDependencies: usize,
    event: cudaEvent_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphAddEventRecordNode(
            pGraphNode.as_mut_ptr() as *mut cudaGraphNode_t,
            graph,
            pDependencies.as_const_ptr() as *const cudaGraphNode_t,
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
pub unsafe fn cudaGraphEventRecordNodeGetEvent(
    node: cudaGraphNode_t,
) -> Result<cudaEvent_t, crate::sys::cudaError> {
    let mut out_1: std::mem::MaybeUninit<cudaEvent_t> = std::mem::MaybeUninit::uninit();
    let status =
        unsafe { crate::sys::cudaGraphEventRecordNodeGetEvent(node, out_1.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok(out_1.assume_init()) }
    } else {
        Err(status)
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
pub unsafe fn cudaGraphAddEventWaitNode<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
>(
    mut pGraphNode: T,
    graph: cudaGraph_t,
    pDependencies: U,
    numDependencies: usize,
    event: cudaEvent_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphAddEventWaitNode(
            pGraphNode.as_mut_ptr() as *mut cudaGraphNode_t,
            graph,
            pDependencies.as_const_ptr() as *const cudaGraphNode_t,
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
pub unsafe fn cudaGraphEventWaitNodeGetEvent(
    node: cudaGraphNode_t,
) -> Result<cudaEvent_t, crate::sys::cudaError> {
    let mut out_1: std::mem::MaybeUninit<cudaEvent_t> = std::mem::MaybeUninit::uninit();
    let status =
        unsafe { crate::sys::cudaGraphEventWaitNodeGetEvent(node, out_1.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok(out_1.assume_init()) }
    } else {
        Err(status)
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
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
>(
    mut pGraphNode: T,
    graph: cudaGraph_t,
    pDependencies: U,
    numDependencies: usize,
    nodeParams: V,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphAddExternalSemaphoresSignalNode(
            pGraphNode.as_mut_ptr() as *mut cudaGraphNode_t,
            graph,
            pDependencies.as_const_ptr() as *const cudaGraphNode_t,
            numDependencies,
            nodeParams.as_const_ptr() as *const cudaExternalSemaphoreSignalNodeParams,
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
    let mut out_1: std::mem::MaybeUninit<cudaExternalSemaphoreSignalNodeParams> =
        std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudaGraphExternalSemaphoresSignalNodeGetParams(
            hNode,
            out_1.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok(out_1.assume_init()) }
    } else {
        Err(status)
    }
}
pub unsafe fn cudaGraphExternalSemaphoresSignalNodeSetParams<T: ::cuda_libs::types::CudaAsPtr>(
    hNode: cudaGraphNode_t,
    nodeParams: T,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphExternalSemaphoresSignalNodeSetParams(
            hNode,
            nodeParams.as_const_ptr() as *const cudaExternalSemaphoreSignalNodeParams,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudaGraphAddExternalSemaphoresWaitNode<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
>(
    mut pGraphNode: T,
    graph: cudaGraph_t,
    pDependencies: U,
    numDependencies: usize,
    nodeParams: V,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphAddExternalSemaphoresWaitNode(
            pGraphNode.as_mut_ptr() as *mut cudaGraphNode_t,
            graph,
            pDependencies.as_const_ptr() as *const cudaGraphNode_t,
            numDependencies,
            nodeParams.as_const_ptr() as *const cudaExternalSemaphoreWaitNodeParams,
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
    let mut out_1: std::mem::MaybeUninit<cudaExternalSemaphoreWaitNodeParams> =
        std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudaGraphExternalSemaphoresWaitNodeGetParams(
            hNode,
            out_1.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok(out_1.assume_init()) }
    } else {
        Err(status)
    }
}
pub unsafe fn cudaGraphExternalSemaphoresWaitNodeSetParams<T: ::cuda_libs::types::CudaAsPtr>(
    hNode: cudaGraphNode_t,
    nodeParams: T,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphExternalSemaphoresWaitNodeSetParams(
            hNode,
            nodeParams.as_const_ptr() as *const cudaExternalSemaphoreWaitNodeParams,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudaGraphAddMemAllocNode<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
>(
    mut pGraphNode: T,
    graph: cudaGraph_t,
    pDependencies: U,
    numDependencies: usize,
    mut nodeParams: V,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphAddMemAllocNode(
            pGraphNode.as_mut_ptr() as *mut cudaGraphNode_t,
            graph,
            pDependencies.as_const_ptr() as *const cudaGraphNode_t,
            numDependencies,
            nodeParams.as_mut_ptr() as *mut cudaMemAllocNodeParams,
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
    let status =
        unsafe { crate::sys::cudaGraphMemAllocNodeGetParams(node, out_1.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok(out_1.assume_init()) }
    } else {
        Err(status)
    }
}
pub unsafe fn cudaGraphAddMemFreeNode<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
>(
    mut pGraphNode: T,
    graph: cudaGraph_t,
    pDependencies: U,
    numDependencies: usize,
    mut dptr: V,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphAddMemFreeNode(
            pGraphNode.as_mut_ptr() as *mut cudaGraphNode_t,
            graph,
            pDependencies.as_const_ptr() as *const cudaGraphNode_t,
            numDependencies,
            dptr.as_mut_ptr() as *mut ::std::os::raw::c_void,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudaGraphMemFreeNodeGetParams<T: ::cuda_libs::types::CudaAsPtr>(
    node: cudaGraphNode_t,
    mut dptr_out: T,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphMemFreeNodeGetParams(
            node,
            dptr_out.as_mut_ptr() as *mut ::std::os::raw::c_void,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudaDeviceGraphMemTrim(
    device: ::std::os::raw::c_int,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaDeviceGraphMemTrim(device) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudaDeviceGetGraphMemAttribute<T: ::cuda_libs::types::CudaAsPtr>(
    device: ::std::os::raw::c_int,
    attr: cudaGraphMemAttributeType,
    mut value: T,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaDeviceGetGraphMemAttribute(
            device,
            attr,
            value.as_mut_ptr() as *mut ::std::os::raw::c_void,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudaDeviceSetGraphMemAttribute<T: ::cuda_libs::types::CudaAsPtr>(
    device: ::std::os::raw::c_int,
    attr: cudaGraphMemAttributeType,
    mut value: T,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaDeviceSetGraphMemAttribute(
            device,
            attr,
            value.as_mut_ptr() as *mut ::std::os::raw::c_void,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Clones a graph\n\n This function creates a copy of \\p originalGraph and returns it in \\p pGraphClone.\n All parameters are copied into the cloned graph. The original graph may be modified\n after this call without affecting the clone.\n\n Child graph nodes in the original graph are recursively copied into the clone.\n\n \\note: Cloning is not supported for graphs which contain memory allocation nodes,\n        memory free nodes, or conditional nodes.\n\n \\param pGraphClone  - Returns newly created cloned graph\n \\param originalGraph - Graph to clone\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorMemoryAllocation\n \\note_graph_thread_safety\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa\n ::cudaGraphCreate,\n ::cudaGraphNodeFindInClone"]
pub unsafe fn cudaGraphClone<T: ::cuda_libs::types::CudaAsPtr>(
    mut pGraphClone: T,
    originalGraph: cudaGraph_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphClone(pGraphClone.as_mut_ptr() as *mut cudaGraph_t, originalGraph)
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Finds a cloned version of a node\n\n This function returns the node in \\p clonedGraph corresponding to \\p originalNode\n in the original graph.\n\n \\p clonedGraph must have been cloned from \\p originalGraph via ::cudaGraphClone.\n \\p originalNode must have been in \\p originalGraph at the time of the call to\n ::cudaGraphClone, and the corresponding cloned node in \\p clonedGraph must not have\n been removed. The cloned node is then returned via \\p pClonedNode.\n\n \\param pNode  - Returns handle to the cloned node\n \\param originalNode - Handle to the original node\n \\param clonedGraph - Cloned graph to query\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue\n \\note_graph_thread_safety\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa\n ::cudaGraphClone"]
pub unsafe fn cudaGraphNodeFindInClone<T: ::cuda_libs::types::CudaAsPtr>(
    mut pNode: T,
    originalNode: cudaGraphNode_t,
    clonedGraph: cudaGraph_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphNodeFindInClone(
            pNode.as_mut_ptr() as *mut cudaGraphNode_t,
            originalNode,
            clonedGraph,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Returns a node's type\n\n Returns the node type of \\p node in \\p pType.\n\n \\param node - Node to query\n \\param pType  - Pointer to return the node type\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue\n \\note_graph_thread_safety\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa\n ::cudaGraphGetNodes,\n ::cudaGraphGetRootNodes,\n ::cudaGraphChildGraphNodeGetGraph,\n ::cudaGraphKernelNodeGetParams,\n ::cudaGraphKernelNodeSetParams,\n ::cudaGraphHostNodeGetParams,\n ::cudaGraphHostNodeSetParams,\n ::cudaGraphMemcpyNodeGetParams,\n ::cudaGraphMemcpyNodeSetParams,\n ::cudaGraphMemsetNodeGetParams,\n ::cudaGraphMemsetNodeSetParams"]
pub unsafe fn cudaGraphNodeGetType(
    node: cudaGraphNode_t,
) -> Result<cudaGraphNodeType, crate::sys::cudaError> {
    let mut out_1: std::mem::MaybeUninit<cudaGraphNodeType> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaGraphNodeGetType(node, out_1.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok(out_1.assume_init()) }
    } else {
        Err(status)
    }
}
#[doc = " \\brief Returns the graph that contains a given graph node\n\n Returns the graph that contains \\p hNode in \\p *phGraph.\n If hNode is in a child graph, the child graph it is in is returned.\n\n \\param hNode - Node to query\n \\param phGraph - Pointer to return the containing graph\n \\return\n ::cudaSuccess\n ::cudaErrorInvalidValue\n\n \\sa\n ::cudaGraphGetNodes,\n ::cudaGraphDebugDotPrint\n ::cudaGraphNodeGetLocalId\n ::cudaGraphNodeGetToolsId\n ::cudaGraphGetId\n ::cudaGraphExecGetId"]
pub unsafe fn cudaGraphNodeGetContainingGraph(
    hNode: cudaGraphNode_t,
) -> Result<cudaGraph_t, crate::sys::cudaError> {
    let mut out_1: std::mem::MaybeUninit<cudaGraph_t> = std::mem::MaybeUninit::uninit();
    let status =
        unsafe { crate::sys::cudaGraphNodeGetContainingGraph(hNode, out_1.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok(out_1.assume_init()) }
    } else {
        Err(status)
    }
}
#[doc = " \\brief Returns the node id of a given graph node\n\n Returns the node id of \\p hNode in \\p *nodeId.\n The nodeId matches that referenced by ::cudaGraphDebugDotPrint.\n The local nodeId and graphId together can uniquely identify the node.\n\n \\param hNode - Node to query\n \\param nodeId - Pointer to return the nodeId\n \\return\n ::cudaSuccess\n ::cudaErrorInvalidValue\n\n \\sa\n ::cudaGraphGetNodes,\n ::cudaGraphDebugDotPrint\n ::cudaGraphNodeGetContainingGraph\n ::cudaGraphNodeGetToolsId\n ::cudaGraphGetId\n ::cudaGraphExecGetId"]
pub unsafe fn cudaGraphNodeGetLocalId(
    hNode: cudaGraphNode_t,
) -> Result<::std::os::raw::c_uint, crate::sys::cudaError> {
    let mut out_1: std::mem::MaybeUninit<::std::os::raw::c_uint> = std::mem::MaybeUninit::uninit();
    let status =
        unsafe { crate::sys::cudaGraphNodeGetLocalId(hNode, out_1.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok(out_1.assume_init()) }
    } else {
        Err(status)
    }
}
#[doc = " \\brief Returns an id used by tools to identify a given node\n\n \\param hNode - Node to query\n \\param *toolsNodeId - Pointer to return the id used by tools\n \\return\n ::CUDA_SUCCESS\n ::cudaErrorInvalidValue\n\n \\sa\n ::cudaGraphGetNodes,\n ::cudaGraphDebugDotPrint\n ::cudaGraphNodeGetContainingGraph\n ::cudaGraphNodeGetLocalId\n ::cudaGraphGetId\n ::cudaGraphExecGetId\n"]
pub unsafe fn cudaGraphNodeGetToolsId(
    hNode: cudaGraphNode_t,
) -> Result<::std::os::raw::c_ulonglong, crate::sys::cudaError> {
    let mut out_1: std::mem::MaybeUninit<::std::os::raw::c_ulonglong> =
        std::mem::MaybeUninit::uninit();
    let status =
        unsafe { crate::sys::cudaGraphNodeGetToolsId(hNode, out_1.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok(out_1.assume_init()) }
    } else {
        Err(status)
    }
}
#[doc = " \\brief Returns the id of a given graph\n\n Returns the id of \\p hGraph in \\p *graphId.\n The value in \\p *graphId matches that referenced by ::cudaGraphDebugDotPrint.\n\n \\param hGraph - Graph to query\n \\param graphId - Pointer to return the graphId\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue\n\n \\sa\n ::cudaGraphGetNodes,\n ::cudaGraphDebugDotPrint\n ::cudaGraphNodeGetContainingGraph\n ::cudaGraphNodeGetLocalId\n ::cudaGraphNodeGetToolsId\n ::cudaGraphExecGetId"]
pub unsafe fn cudaGraphGetId(
    hGraph: cudaGraph_t,
) -> Result<::std::os::raw::c_uint, crate::sys::cudaError> {
    let mut out_1: std::mem::MaybeUninit<::std::os::raw::c_uint> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaGraphGetId(hGraph, out_1.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok(out_1.assume_init()) }
    } else {
        Err(status)
    }
}
#[doc = " \\brief Returns the id of a given graph exec\n\n Returns the id of \\p hGraphExec in \\p *graphId.\n The value in \\p *graphId matches that referenced by ::cudaGraphDebugDotPrint.\n\n \\param hGraphExec - Graph to query\n \\param graphId - Pointer to return the graphId\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue\n\n \\sa\n ::cudaGraphGetNodes,\n ::cudaGraphDebugDotPrint\n ::cudaGraphNodeGetContainingGraph\n ::cudaGraphNodeGetLocalId\n ::cudaGraphNodeGetToolsId\n ::cudaGraphGetId"]
pub unsafe fn cudaGraphExecGetId(
    hGraphExec: cudaGraphExec_t,
) -> Result<::std::os::raw::c_uint, crate::sys::cudaError> {
    let mut out_1: std::mem::MaybeUninit<::std::os::raw::c_uint> = std::mem::MaybeUninit::uninit();
    let status =
        unsafe { crate::sys::cudaGraphExecGetId(hGraphExec, out_1.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok(out_1.assume_init()) }
    } else {
        Err(status)
    }
}
#[doc = " \\brief Returns a graph's nodes\n\n Returns a list of \\p graph's nodes. \\p nodes may be NULL, in which case this\n function will return the number of nodes in \\p numNodes. Otherwise,\n \\p numNodes entries will be filled in. If \\p numNodes is higher than the actual\n number of nodes, the remaining entries in \\p nodes will be set to NULL, and the\n number of nodes actually obtained will be returned in \\p numNodes.\n\n \\param graph    - Graph to query\n \\param nodes    - Pointer to return the nodes\n \\param numNodes - See description\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue\n \\note_graph_thread_safety\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa\n ::cudaGraphCreate,\n ::cudaGraphGetRootNodes,\n ::cudaGraphGetEdges,\n ::cudaGraphNodeGetType,\n ::cudaGraphNodeGetDependencies,\n ::cudaGraphNodeGetDependentNodes"]
pub unsafe fn cudaGraphGetNodes(
    graph: cudaGraph_t,
) -> Result<(cudaGraphNode_t, usize), crate::sys::cudaError> {
    let mut out_1: std::mem::MaybeUninit<cudaGraphNode_t> = std::mem::MaybeUninit::uninit();
    let mut out_2: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudaGraphGetNodes(
            graph,
            out_1.as_mut_ptr() as *mut _,
            out_2.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok((out_1.assume_init(), out_2.assume_init())) }
    } else {
        Err(status)
    }
}
#[doc = " \\brief Returns a graph's root nodes\n\n Returns a list of \\p graph's root nodes. \\p pRootNodes may be NULL, in which case this\n function will return the number of root nodes in \\p pNumRootNodes. Otherwise,\n \\p pNumRootNodes entries will be filled in. If \\p pNumRootNodes is higher than the actual\n number of root nodes, the remaining entries in \\p pRootNodes will be set to NULL, and the\n number of nodes actually obtained will be returned in \\p pNumRootNodes.\n\n \\param graph       - Graph to query\n \\param pRootNodes    - Pointer to return the root nodes\n \\param pNumRootNodes - See description\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue\n \\note_graph_thread_safety\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa\n ::cudaGraphCreate,\n ::cudaGraphGetNodes,\n ::cudaGraphGetEdges,\n ::cudaGraphNodeGetType,\n ::cudaGraphNodeGetDependencies,\n ::cudaGraphNodeGetDependentNodes"]
pub unsafe fn cudaGraphGetRootNodes(
    graph: cudaGraph_t,
) -> Result<(cudaGraphNode_t, usize), crate::sys::cudaError> {
    let mut out_1: std::mem::MaybeUninit<cudaGraphNode_t> = std::mem::MaybeUninit::uninit();
    let mut out_2: std::mem::MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudaGraphGetRootNodes(
            graph,
            out_1.as_mut_ptr() as *mut _,
            out_2.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok((out_1.assume_init(), out_2.assume_init())) }
    } else {
        Err(status)
    }
}
#[doc = " \\brief Returns a graph's dependency edges\n\n Returns a list of \\p graph's dependency edges. Edges are returned via corresponding\n indices in \\p from, \\p to and \\p edgeData; that is, the node in \\p to[i] has a\n dependency on the node in \\p from[i] with data \\p edgeData[i]. \\p from and \\p to may\n both be NULL, in which case this function only returns the number of edges in\n \\p numEdges. Otherwise, \\p numEdges entries will be filled in. If \\p numEdges is higher\n than the actual number of edges, the remaining entries in \\p from and \\p to will be\n set to NULL, and the number of edges actually returned will be written to \\p numEdges.\n \\p edgeData may alone be NULL, in which case the edges must all have default (zeroed)\n edge data. Attempting a losst query via NULL \\p edgeData will result in\n ::cudaErrorLossyQuery. If \\p edgeData is non-NULL then \\p from and \\p to must be as\n well.\n\n \\param graph    - Graph to get the edges from\n \\param from     - Location to return edge endpoints\n \\param to       - Location to return edge endpoints\n \\param edgeData - Optional location to return edge data\n \\param numEdges - See description\n\n \\return\n ::cudaSuccess,\n ::cudaErrorLossyQuery,\n ::cudaErrorInvalidValue\n \\note_graph_thread_safety\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa\n ::cudaGraphGetNodes,\n ::cudaGraphGetRootNodes,\n ::cudaGraphAddDependencies,\n ::cudaGraphRemoveDependencies,\n ::cudaGraphNodeGetDependencies,\n ::cudaGraphNodeGetDependentNodes"]
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
    if status == crate::sys::cudaError::cudaSuccess {
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
#[doc = " \\brief Returns a node's dependencies\n\n Returns a list of \\p node's dependencies. \\p pDependencies may be NULL, in which case this\n function will return the number of dependencies in \\p pNumDependencies. Otherwise,\n \\p pNumDependencies entries will be filled in. If \\p pNumDependencies is higher than the actual\n number of dependencies, the remaining entries in \\p pDependencies will be set to NULL, and the\n number of nodes actually obtained will be returned in \\p pNumDependencies.\n\n Note that if an edge has non-zero (non-default) edge data and \\p edgeData is NULL,\n this API will return ::cudaErrorLossyQuery. If \\p edgeData is non-NULL, then\n \\p pDependencies must be as well.\n\n \\param node             - Node to query\n \\param pDependencies    - Pointer to return the dependencies\n \\param edgeData         - Optional array to return edge data for each dependency\n \\param pNumDependencies - See description\n\n \\return\n ::cudaSuccess,\n ::cudaErrorLossyQuery,\n ::cudaErrorInvalidValue\n \\note_graph_thread_safety\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa\n ::cudaGraphNodeGetDependentNodes,\n ::cudaGraphGetNodes,\n ::cudaGraphGetRootNodes,\n ::cudaGraphGetEdges,\n ::cudaGraphAddDependencies,\n ::cudaGraphRemoveDependencies"]
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
    if status == crate::sys::cudaError::cudaSuccess {
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
#[doc = " \\brief Returns a node's dependent nodes\n\n Returns a list of \\p node's dependent nodes. \\p pDependentNodes may be NULL, in which\n case this function will return the number of dependent nodes in \\p pNumDependentNodes.\n Otherwise, \\p pNumDependentNodes entries will be filled in. If \\p pNumDependentNodes is\n higher than the actual number of dependent nodes, the remaining entries in\n \\p pDependentNodes will be set to NULL, and the number of nodes actually obtained will\n be returned in \\p pNumDependentNodes.\n\n Note that if an edge has non-zero (non-default) edge data and \\p edgeData is NULL,\n this API will return ::cudaErrorLossyQuery. If \\p edgeData is non-NULL, then\n \\p pDependentNodes must be as well.\n\n \\param node               - Node to query\n \\param pDependentNodes    - Pointer to return the dependent nodes\n \\param edgeData           - Optional pointer to return edge data for dependent nodes\n \\param pNumDependentNodes - See description\n\n \\return\n ::cudaSuccess,\n ::cudaErrorLossyQuery,\n ::cudaErrorInvalidValue\n \\note_graph_thread_safety\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa\n ::cudaGraphNodeGetDependencies,\n ::cudaGraphGetNodes,\n ::cudaGraphGetRootNodes,\n ::cudaGraphGetEdges,\n ::cudaGraphAddDependencies,\n ::cudaGraphRemoveDependencies"]
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
    if status == crate::sys::cudaError::cudaSuccess {
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
#[doc = " \\brief Adds dependency edges to a graph.\n\n The number of dependencies to be added is defined by \\p numDependencies\n Elements in \\p pFrom and \\p pTo at corresponding indices define a dependency.\n Each node in \\p pFrom and \\p pTo must belong to \\p graph.\n\n If \\p numDependencies is 0, elements in \\p pFrom and \\p pTo will be ignored.\n Specifying an existing dependency will return an error.\n\n \\param graph - Graph to which dependencies are added\n \\param from - Array of nodes that provide the dependencies\n \\param to - Array of dependent nodes\n \\param edgeData - Optional array of edge data. If NULL, default (zeroed) edge data is assumed.\n \\param numDependencies - Number of dependencies to be added\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue\n \\note_graph_thread_safety\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa\n ::cudaGraphRemoveDependencies,\n ::cudaGraphGetEdges,\n ::cudaGraphNodeGetDependencies,\n ::cudaGraphNodeGetDependentNodes"]
pub unsafe fn cudaGraphAddDependencies<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
>(
    graph: cudaGraph_t,
    from: T,
    to: U,
    edgeData: V,
    numDependencies: usize,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphAddDependencies(
            graph,
            from.as_const_ptr() as *const cudaGraphNode_t,
            to.as_const_ptr() as *const cudaGraphNode_t,
            edgeData.as_const_ptr() as *const cudaGraphEdgeData,
            numDependencies,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Removes dependency edges from a graph.\n\n The number of \\p pDependencies to be removed is defined by \\p numDependencies.\n Elements in \\p pFrom and \\p pTo at corresponding indices define a dependency.\n Each node in \\p pFrom and \\p pTo must belong to \\p graph.\n\n If \\p numDependencies is 0, elements in \\p pFrom and \\p pTo will be ignored.\n Specifying an edge that does not exist in the graph, with data matching\n \\p edgeData, results in an error. \\p edgeData is nullable, which is equivalent\n to passing default (zeroed) data for each edge.\n\n \\param graph - Graph from which to remove dependencies\n \\param from - Array of nodes that provide the dependencies\n \\param to - Array of dependent nodes\n \\param edgeData - Optional array of edge data. If NULL, edge data is assumed to\n                   be default (zeroed).\n \\param numDependencies - Number of dependencies to be removed\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue\n \\note_graph_thread_safety\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa\n ::cudaGraphAddDependencies,\n ::cudaGraphGetEdges,\n ::cudaGraphNodeGetDependencies,\n ::cudaGraphNodeGetDependentNodes"]
pub unsafe fn cudaGraphRemoveDependencies<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
>(
    graph: cudaGraph_t,
    from: T,
    to: U,
    edgeData: V,
    numDependencies: usize,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphRemoveDependencies(
            graph,
            from.as_const_ptr() as *const cudaGraphNode_t,
            to.as_const_ptr() as *const cudaGraphNode_t,
            edgeData.as_const_ptr() as *const cudaGraphEdgeData,
            numDependencies,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Remove a node from the graph\n\n Removes \\p node from its graph. This operation also severs any dependencies of other nodes\n on \\p node and vice versa.\n\n Dependencies cannot be removed from graphs which contain allocation or free nodes.\n Any attempt to do so will return an error.\n\n \\param node  - Node to remove\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue\n \\note_graph_thread_safety\n \\notefnerr\n \\note_init_rt\n \\note_callback\n \\note_destroy_ub\n\n \\sa\n ::cudaGraphAddChildGraphNode,\n ::cudaGraphAddEmptyNode,\n ::cudaGraphAddKernelNode,\n ::cudaGraphAddHostNode,\n ::cudaGraphAddMemcpyNode,\n ::cudaGraphAddMemsetNode"]
pub unsafe fn cudaGraphDestroyNode(node: cudaGraphNode_t) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaGraphDestroyNode(node) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Creates an executable graph from a graph\n\n Instantiates \\p graph as an executable graph. The graph is validated for any\n structural constraints or intra-node constraints which were not previously\n validated. If instantiation is successful, a handle to the instantiated graph\n is returned in \\p pGraphExec.\n\n The \\p flags parameter controls the behavior of instantiation and subsequent\n graph launches.  Valid flags are:\n\n - ::cudaGraphInstantiateFlagAutoFreeOnLaunch, which configures a\n graph containing memory allocation nodes to automatically free any\n unfreed memory allocations before the graph is relaunched.\n\n - ::cudaGraphInstantiateFlagDeviceLaunch, which configures the graph for launch\n from the device. If this flag is passed, the executable graph handle returned can be\n used to launch the graph from both the host and device. This flag cannot be used in\n conjunction with ::cudaGraphInstantiateFlagAutoFreeOnLaunch.\n\n - ::cudaGraphInstantiateFlagUseNodePriority, which causes the graph\n to use the priorities from the per-node attributes rather than the priority\n of the launch stream during execution. Note that priorities are only available\n on kernel nodes, and are copied from stream priority during stream capture.\n\n If \\p graph contains any allocation or free nodes, there can be at most one\n executable graph in existence for that graph at a time. An attempt to\n instantiate a second executable graph before destroying the first with\n ::cudaGraphExecDestroy will result in an error.\n The same also applies if \\p graph contains any device-updatable kernel nodes.\n\n Graphs instantiated for launch on the device have additional restrictions which do not\n apply to host graphs:\n\n - The graph's nodes must reside on a single device.\n - The graph can only contain kernel nodes, memcpy nodes, memset nodes, and child graph nodes.\n - The graph cannot be empty and must contain at least one kernel, memcpy, or memset node.\n   Operation-specific restrictions are outlined below.\n - Kernel nodes:\n   - Use of CUDA Dynamic Parallelism is not permitted.\n   - Cooperative launches are permitted as long as MPS is not in use.\n - Memcpy nodes:\n   - Only copies involving device memory and/or pinned device-mapped host memory are permitted.\n   - Copies involving CUDA arrays are not permitted.\n   - Both operands must be accessible from the current device, and the current device must\n     match the device of other nodes in the graph.\n\n If \\p graph is not instantiated for launch on the device but contains kernels which\n call device-side cudaGraphLaunch() from multiple devices, this will result in an error.\n\n \\param pGraphExec - Returns instantiated graph\n \\param graph      - Graph to instantiate\n \\param flags      - Flags to control instantiation.  See ::CUgraphInstantiate_flags.\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue\n \\note_graph_thread_safety\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa\n ::cudaGraphInstantiateWithFlags,\n ::cudaGraphCreate,\n ::cudaGraphUpload,\n ::cudaGraphLaunch,\n ::cudaGraphExecDestroy"]
pub unsafe fn cudaGraphInstantiate<T: ::cuda_libs::types::CudaAsPtr>(
    mut pGraphExec: T,
    graph: cudaGraph_t,
    flags: ::std::os::raw::c_ulonglong,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphInstantiate(
            pGraphExec.as_mut_ptr() as *mut cudaGraphExec_t,
            graph,
            flags,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudaGraphInstantiateWithFlags<T: ::cuda_libs::types::CudaAsPtr>(
    mut pGraphExec: T,
    graph: cudaGraph_t,
    flags: ::std::os::raw::c_ulonglong,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphInstantiateWithFlags(
            pGraphExec.as_mut_ptr() as *mut cudaGraphExec_t,
            graph,
            flags,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Creates an executable graph from a graph\n\n Instantiates \\p graph as an executable graph according to the \\p instantiateParams structure.\n The graph is validated for any structural constraints or intra-node constraints\n which were not previously validated. If instantiation is successful, a handle to\n the instantiated graph is returned in \\p pGraphExec.\n\n \\p instantiateParams controls the behavior of instantiation and subsequent\n graph launches, as well as returning more detailed information in the event of an error.\n ::cudaGraphInstantiateParams is defined as:\n\n \\code\ntypedef struct {\nunsigned long long flags;\ncudaStream_t uploadStream;\ncudaGraphNode_t errNode_out;\ncudaGraphInstantiateResult result_out;\n} cudaGraphInstantiateParams;\n \\endcode\n\n The \\p flags field controls the behavior of instantiation and subsequent\n graph launches. Valid flags are:\n\n - ::cudaGraphInstantiateFlagAutoFreeOnLaunch, which configures a\n graph containing memory allocation nodes to automatically free any\n unfreed memory allocations before the graph is relaunched.\n\n - ::cudaGraphInstantiateFlagUpload, which will perform an upload of the graph\n into \\p uploadStream once the graph has been instantiated.\n\n - ::cudaGraphInstantiateFlagDeviceLaunch, which configures the graph for launch\n from the device. If this flag is passed, the executable graph handle returned can be\n used to launch the graph from both the host and device. This flag can only be used\n on platforms which support unified addressing. This flag cannot be used in\n conjunction with ::cudaGraphInstantiateFlagAutoFreeOnLaunch.\n\n - ::cudaGraphInstantiateFlagUseNodePriority, which causes the graph\n to use the priorities from the per-node attributes rather than the priority\n of the launch stream during execution. Note that priorities are only available\n on kernel nodes, and are copied from stream priority during stream capture.\n\n If \\p graph contains any allocation or free nodes, there can be at most one\n executable graph in existence for that graph at a time. An attempt to instantiate a\n second executable graph before destroying the first with ::cudaGraphExecDestroy will\n result in an error.\n The same also applies if \\p graph contains any device-updatable kernel nodes.\n\n If \\p graph contains kernels which call device-side cudaGraphLaunch() from multiple\n devices, this will result in an error.\n\n Graphs instantiated for launch on the device have additional restrictions which do not\n apply to host graphs:\n\n - The graph's nodes must reside on a single device.\n - The graph can only contain kernel nodes, memcpy nodes, memset nodes, and child graph nodes.\n - The graph cannot be empty and must contain at least one kernel, memcpy, or memset node.\n   Operation-specific restrictions are outlined below.\n - Kernel nodes:\n   - Use of CUDA Dynamic Parallelism is not permitted.\n   - Cooperative launches are permitted as long as MPS is not in use.\n - Memcpy nodes:\n   - Only copies involving device memory and/or pinned device-mapped host memory are permitted.\n   - Copies involving CUDA arrays are not permitted.\n   - Both operands must be accessible from the current device, and the current device must\n     match the device of other nodes in the graph.\n\n In the event of an error, the \\p result_out and \\p errNode_out fields will contain more\n information about the nature of the error. Possible error reporting includes:\n\n - ::cudaGraphInstantiateError, if passed an invalid value or if an unexpected error occurred\n   which is described by the return value of the function. \\p errNode_out will be set to NULL.\n - ::cudaGraphInstantiateInvalidStructure, if the graph structure is invalid. \\p errNode_out\n   will be set to one of the offending nodes.\n - ::cudaGraphInstantiateNodeOperationNotSupported, if the graph is instantiated for device\n   launch but contains a node of an unsupported node type, or a node which performs unsupported\n   operations, such as use of CUDA dynamic parallelism within a kernel node. \\p errNode_out will\n   be set to this node.\n - ::cudaGraphInstantiateMultipleDevicesNotSupported, if the graph is instantiated for device\n   launch but a node’s device differs from that of another node. This error can also be returned\n   if a graph is not instantiated for device launch and it contains kernels which call device-side\n   cudaGraphLaunch() from multiple devices. \\p errNode_out will be set to this node.\n\n If instantiation is successful, \\p result_out will be set to ::cudaGraphInstantiateSuccess,\n and \\p hErrNode_out will be set to NULL.\n\n \\param pGraphExec       - Returns instantiated graph\n \\param graph            - Graph to instantiate\n \\param instantiateParams - Instantiation parameters\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue\n \\note_graph_thread_safety\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa\n ::cudaGraphCreate,\n ::cudaGraphInstantiate,\n ::cudaGraphInstantiateWithFlags,\n ::cudaGraphExecDestroy"]
pub unsafe fn cudaGraphInstantiateWithParams<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
>(
    mut pGraphExec: T,
    graph: cudaGraph_t,
    mut instantiateParams: U,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphInstantiateWithParams(
            pGraphExec.as_mut_ptr() as *mut cudaGraphExec_t,
            graph,
            instantiateParams.as_mut_ptr() as *mut cudaGraphInstantiateParams,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Query the instantiation flags of an executable graph\n\n Returns the flags that were passed to instantiation for the given executable graph.\n ::cudaGraphInstantiateFlagUpload will not be returned by this API as it does\n not affect the resulting executable graph.\n\n \\param graphExec - The executable graph to query\n \\param flags     - Returns the instantiation flags\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue\n \\note_graph_thread_safety\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa\n ::cudaGraphInstantiate,\n ::cudaGraphInstantiateWithFlags,\n ::cudaGraphInstantiateWithParams"]
pub unsafe fn cudaGraphExecGetFlags(
    graphExec: cudaGraphExec_t,
) -> Result<::std::os::raw::c_ulonglong, crate::sys::cudaError> {
    let mut out_1: std::mem::MaybeUninit<::std::os::raw::c_ulonglong> =
        std::mem::MaybeUninit::uninit();
    let status =
        unsafe { crate::sys::cudaGraphExecGetFlags(graphExec, out_1.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok(out_1.assume_init()) }
    } else {
        Err(status)
    }
}
#[doc = " \\brief Sets the parameters for a kernel node in the given graphExec\n\n Sets the parameters of a kernel node in an executable graph \\p hGraphExec.\n The node is identified by the corresponding node \\p node in the\n non-executable graph, from which the executable graph was instantiated.\n\n \\p node must not have been removed from the original graph. All \\p nodeParams\n fields may change, but the following restrictions apply to \\p func updates:\n\n   - The owning device of the function cannot change.\n   - A node whose function originally did not use CUDA dynamic parallelism cannot be updated\n     to a function which uses CDP\n   - A node whose function originally did not make device-side update calls cannot be updated\n     to a function which makes device-side update calls.\n   - If \\p hGraphExec was not instantiated for device launch, a node whose function originally\n     did not use device-side cudaGraphLaunch() cannot be updated to a function which uses\n     device-side cudaGraphLaunch() unless the node resides on the same device as nodes which\n     contained such calls at instantiate-time. If no such calls were present at instantiation,\n     these updates cannot be performed at all.\n\n The modifications only affect future launches of \\p hGraphExec. Already\n enqueued or running launches of \\p hGraphExec are not affected by this call.\n \\p node is also not modified by this call.\n\n If \\p node is a device-updatable kernel node, the next upload/launch of \\p hGraphExec\n will overwrite any previous device-side updates. Additionally, applying host updates to a\n device-updatable kernel node while it is being updated from the device will result in\n undefined behavior.\n\n \\param hGraphExec  - The executable graph in which to set the specified node\n \\param node        - kernel node from the graph from which graphExec was instantiated\n \\param pNodeParams - Updated Parameters to set\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n \\note_graph_thread_safety\n \\notefnerr\n \\note_init_rt\n \\note_callback\n \\note_cudaKernel_t\n\n \\sa\n ::cudaGraphExecNodeSetParams,\n ::cudaGraphAddKernelNode,\n ::cudaGraphKernelNodeSetParams,\n ::cudaGraphExecMemcpyNodeSetParams,\n ::cudaGraphExecMemsetNodeSetParams,\n ::cudaGraphExecHostNodeSetParams,\n ::cudaGraphExecChildGraphNodeSetParams,\n ::cudaGraphExecEventRecordNodeSetEvent,\n ::cudaGraphExecEventWaitNodeSetEvent,\n ::cudaGraphExecExternalSemaphoresSignalNodeSetParams,\n ::cudaGraphExecExternalSemaphoresWaitNodeSetParams,\n ::cudaGraphExecUpdate,\n ::cudaGraphInstantiate"]
pub unsafe fn cudaGraphExecKernelNodeSetParams<T: ::cuda_libs::types::CudaAsPtr>(
    hGraphExec: cudaGraphExec_t,
    node: cudaGraphNode_t,
    pNodeParams: T,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphExecKernelNodeSetParams(
            hGraphExec,
            node,
            pNodeParams.as_const_ptr() as *const cudaKernelNodeParams,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Sets the parameters for a memcpy node in the given graphExec.\n\n Updates the work represented by \\p node in \\p hGraphExec as though \\p node had\n contained \\p pNodeParams at instantiation.  \\p node must remain in the graph which was\n used to instantiate \\p hGraphExec.  Changed edges to and from \\p node are ignored.\n\n The source and destination memory in \\p pNodeParams must be allocated from the same\n contexts as the original source and destination memory.  Both the instantiation-time\n memory operands and the memory operands in \\p pNodeParams must be 1-dimensional.\n Zero-length operations are not supported.\n\n The modifications only affect future launches of \\p hGraphExec.  Already enqueued\n or running launches of \\p hGraphExec are not affected by this call.  \\p node is also\n not modified by this call.\n\n Returns ::cudaErrorInvalidValue if the memory operands' mappings changed or\n either the original or new memory operands are multidimensional.\n\n \\param hGraphExec  - The executable graph in which to set the specified node\n \\param node        - Memcpy node from the graph which was used to instantiate graphExec\n \\param pNodeParams - Updated Parameters to set\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n \\note_graph_thread_safety\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa\n ::cudaGraphExecNodeSetParams,\n ::cudaGraphAddMemcpyNode,\n ::cudaGraphMemcpyNodeSetParams,\n ::cudaGraphExecMemcpyNodeSetParamsToSymbol,\n ::cudaGraphExecMemcpyNodeSetParamsFromSymbol,\n ::cudaGraphExecMemcpyNodeSetParams1D,\n ::cudaGraphExecKernelNodeSetParams,\n ::cudaGraphExecMemsetNodeSetParams,\n ::cudaGraphExecHostNodeSetParams,\n ::cudaGraphExecChildGraphNodeSetParams,\n ::cudaGraphExecEventRecordNodeSetEvent,\n ::cudaGraphExecEventWaitNodeSetEvent,\n ::cudaGraphExecExternalSemaphoresSignalNodeSetParams,\n ::cudaGraphExecExternalSemaphoresWaitNodeSetParams,\n ::cudaGraphExecUpdate,\n ::cudaGraphInstantiate"]
pub unsafe fn cudaGraphExecMemcpyNodeSetParams<T: ::cuda_libs::types::CudaAsPtr>(
    hGraphExec: cudaGraphExec_t,
    node: cudaGraphNode_t,
    pNodeParams: T,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphExecMemcpyNodeSetParams(
            hGraphExec,
            node,
            pNodeParams.as_const_ptr() as *const cudaMemcpy3DParms,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudaGraphExecMemcpyNodeSetParamsToSymbol<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
>(
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
            symbol.as_const_ptr() as *const ::std::os::raw::c_void,
            src.as_const_ptr() as *const ::std::os::raw::c_void,
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
pub unsafe fn cudaGraphExecMemcpyNodeSetParamsFromSymbol<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
>(
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
            dst.as_mut_ptr() as *mut ::std::os::raw::c_void,
            symbol.as_const_ptr() as *const ::std::os::raw::c_void,
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
pub unsafe fn cudaGraphExecMemcpyNodeSetParams1D<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
>(
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
            dst.as_mut_ptr() as *mut ::std::os::raw::c_void,
            src.as_const_ptr() as *const ::std::os::raw::c_void,
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
#[doc = " \\brief Sets the parameters for a memset node in the given graphExec.\n\n Updates the work represented by \\p node in \\p hGraphExec as though \\p node had\n contained \\p pNodeParams at instantiation.  \\p node must remain in the graph which was\n used to instantiate \\p hGraphExec.  Changed edges to and from \\p node are ignored.\n\n Zero sized operations are not supported.\n\n The new destination pointer in \\p pNodeParams must be to the same kind of allocation\n as the original destination pointer and have the same context association and device mapping\n as the original destination pointer.\n\n Both the value and pointer address may be updated.\n Changing other aspects of the memset (width, height, element size or pitch) may cause the update to be rejected.\n Specifically, for 2d memsets, all dimension changes are rejected.\n For 1d memsets, changes in height are explicitly rejected and other changes are opportunistically allowed\n if the resulting work maps onto the work resources already allocated for the node.\n\n The modifications only affect future launches of \\p hGraphExec.  Already enqueued\n or running launches of \\p hGraphExec are not affected by this call.  \\p node is also\n not modified by this call.\n\n \\param hGraphExec  - The executable graph in which to set the specified node\n \\param node        - Memset node from the graph which was used to instantiate graphExec\n \\param pNodeParams - Updated Parameters to set\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n \\note_graph_thread_safety\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa\n ::cudaGraphExecNodeSetParams,\n ::cudaGraphAddMemsetNode,\n ::cudaGraphMemsetNodeSetParams,\n ::cudaGraphExecKernelNodeSetParams,\n ::cudaGraphExecMemcpyNodeSetParams,\n ::cudaGraphExecHostNodeSetParams,\n ::cudaGraphExecChildGraphNodeSetParams,\n ::cudaGraphExecEventRecordNodeSetEvent,\n ::cudaGraphExecEventWaitNodeSetEvent,\n ::cudaGraphExecExternalSemaphoresSignalNodeSetParams,\n ::cudaGraphExecExternalSemaphoresWaitNodeSetParams,\n ::cudaGraphExecUpdate,\n ::cudaGraphInstantiate"]
pub unsafe fn cudaGraphExecMemsetNodeSetParams<T: ::cuda_libs::types::CudaAsPtr>(
    hGraphExec: cudaGraphExec_t,
    node: cudaGraphNode_t,
    pNodeParams: T,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphExecMemsetNodeSetParams(
            hGraphExec,
            node,
            pNodeParams.as_const_ptr() as *const cudaMemsetParams,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Sets the parameters for a host node in the given graphExec.\n\n Updates the work represented by \\p node in \\p hGraphExec as though \\p node had\n contained \\p pNodeParams at instantiation.  \\p node must remain in the graph which was\n used to instantiate \\p hGraphExec.  Changed edges to and from \\p node are ignored.\n\n The modifications only affect future launches of \\p hGraphExec.  Already enqueued\n or running launches of \\p hGraphExec are not affected by this call.  \\p node is also\n not modified by this call.\n\n \\param hGraphExec  - The executable graph in which to set the specified node\n \\param node        - Host node from the graph which was used to instantiate graphExec\n \\param pNodeParams - Updated Parameters to set\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n \\note_graph_thread_safety\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa\n ::cudaGraphExecNodeSetParams,\n ::cudaGraphAddHostNode,\n ::cudaGraphHostNodeSetParams,\n ::cudaGraphExecKernelNodeSetParams,\n ::cudaGraphExecMemcpyNodeSetParams,\n ::cudaGraphExecMemsetNodeSetParams,\n ::cudaGraphExecChildGraphNodeSetParams,\n ::cudaGraphExecEventRecordNodeSetEvent,\n ::cudaGraphExecEventWaitNodeSetEvent,\n ::cudaGraphExecExternalSemaphoresSignalNodeSetParams,\n ::cudaGraphExecExternalSemaphoresWaitNodeSetParams,\n ::cudaGraphExecUpdate,\n ::cudaGraphInstantiate"]
pub unsafe fn cudaGraphExecHostNodeSetParams<T: ::cuda_libs::types::CudaAsPtr>(
    hGraphExec: cudaGraphExec_t,
    node: cudaGraphNode_t,
    pNodeParams: T,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphExecHostNodeSetParams(
            hGraphExec,
            node,
            pNodeParams.as_const_ptr() as *const cudaHostNodeParams,
        )
    };
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
    let status =
        unsafe { crate::sys::cudaGraphExecChildGraphNodeSetParams(hGraphExec, node, childGraph) };
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
    let status =
        unsafe { crate::sys::cudaGraphExecEventRecordNodeSetEvent(hGraphExec, hNode, event) };
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
    let status =
        unsafe { crate::sys::cudaGraphExecEventWaitNodeSetEvent(hGraphExec, hNode, event) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudaGraphExecExternalSemaphoresSignalNodeSetParams<
    T: ::cuda_libs::types::CudaAsPtr,
>(
    hGraphExec: cudaGraphExec_t,
    hNode: cudaGraphNode_t,
    nodeParams: T,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphExecExternalSemaphoresSignalNodeSetParams(
            hGraphExec,
            hNode,
            nodeParams.as_const_ptr() as *const cudaExternalSemaphoreSignalNodeParams,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudaGraphExecExternalSemaphoresWaitNodeSetParams<T: ::cuda_libs::types::CudaAsPtr>(
    hGraphExec: cudaGraphExec_t,
    hNode: cudaGraphNode_t,
    nodeParams: T,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphExecExternalSemaphoresWaitNodeSetParams(
            hGraphExec,
            hNode,
            nodeParams.as_const_ptr() as *const cudaExternalSemaphoreWaitNodeParams,
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
    isEnabled: ::std::os::raw::c_uint,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaGraphNodeSetEnabled(hGraphExec, hNode, isEnabled) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudaGraphNodeGetEnabled(
    hGraphExec: cudaGraphExec_t,
    hNode: cudaGraphNode_t,
) -> Result<::std::os::raw::c_uint, crate::sys::cudaError> {
    let mut out_2: std::mem::MaybeUninit<::std::os::raw::c_uint> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudaGraphNodeGetEnabled(hGraphExec, hNode, out_2.as_mut_ptr() as *mut _)
    };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok(out_2.assume_init()) }
    } else {
        Err(status)
    }
}
#[doc = " \\brief Check whether an executable graph can be updated with a graph and perform the update if possible\n\n Updates the node parameters in the instantiated graph specified by \\p hGraphExec with the\n node parameters in a topologically identical graph specified by \\p hGraph.\n\n Limitations:\n\n - Kernel nodes:\n   - The owning context of the function cannot change.\n   - A node whose function originally did not use CUDA dynamic parallelism cannot be updated\n     to a function which uses CDP.\n   - A node whose function originally did not make device-side update calls cannot be updated\n     to a function which makes device-side update calls.\n   - A cooperative node cannot be updated to a non-cooperative node, and vice-versa.\n   - If the graph was instantiated with cudaGraphInstantiateFlagUseNodePriority, the\n     priority attribute cannot change. Equality is checked on the originally requested\n     priority values, before they are clamped to the device's supported range.\n   - If \\p hGraphExec was not instantiated for device launch, a node whose function originally\n     did not use device-side cudaGraphLaunch() cannot be updated to a function which uses\n     device-side cudaGraphLaunch() unless the node resides on the same device as nodes which\n     contained such calls at instantiate-time. If no such calls were present at instantiation,\n     these updates cannot be performed at all.\n   - Neither \\p hGraph nor \\p hGraphExec may contain device-updatable kernel nodes.\n - Memset and memcpy nodes:\n   - The CUDA device(s) to which the operand(s) was allocated/mapped cannot change.\n   - The source/destination memory must be allocated from the same contexts as the original\n     source/destination memory.\n   - For 2d memsets, only address and assigned value may be updated.\n   - For 1d memsets, updating dimensions is also allowed, but may fail if the resulting operation doesn't\n     map onto the work resources already allocated for the node.\n - Additional memcpy node restrictions:\n   - Changing either the source or destination memory type(i.e. CU_MEMORYTYPE_DEVICE,\n     CU_MEMORYTYPE_ARRAY, etc.) is not supported.\n - Conditional nodes:\n   - Changing node parameters is not supported.\n   - Changing parameters of nodes within the conditional body graph is subject to the rules above.\n   - Conditional handle flags and default values are updated as part of the graph update.\n\n Note:  The API may add further restrictions in future releases.  The return code should always be checked.\n\n cudaGraphExecUpdate sets the result member of \\p resultInfo to cudaGraphExecUpdateErrorTopologyChanged\n under the following conditions:\n - The count of nodes directly in \\p hGraphExec and \\p hGraph differ, in which case resultInfo->errorNode\n   is set to NULL.\n - \\p hGraph has more exit nodes than \\p hGraph, in which case resultInfo->errorNode is set to one of\n   the exit nodes in hGraph.\n - A node in \\p hGraph has a different number of dependencies than the node from \\p hGraphExec it is paired with,\n   in which case resultInfo->errorNode is set to the node from \\p hGraph.\n - A node in \\p hGraph has a dependency that does not match with the corresponding dependency of the paired node\n   from \\p hGraphExec. resultInfo->errorNode will be set to the node from \\p hGraph. resultInfo->errorFromNode\n   will be set to the mismatched dependency. The dependencies are paired based on edge order and a dependency\n   does not match when the nodes are already paired based on other edges examined in the graph.\n\n cudaGraphExecUpdate sets \\p the result member of \\p resultInfo to:\n - cudaGraphExecUpdateError if passed an invalid value.\n - cudaGraphExecUpdateErrorTopologyChanged if the graph topology changed\n - cudaGraphExecUpdateErrorNodeTypeChanged if the type of a node changed, in which case\n   \\p hErrorNode_out is set to the node from \\p hGraph.\n - cudaGraphExecUpdateErrorFunctionChanged if the function of a kernel node changed (CUDA driver < 11.2)\n - cudaGraphExecUpdateErrorUnsupportedFunctionChange if the func field of a kernel changed in an\n   unsupported way(see note above), in which case \\p hErrorNode_out is set to the node from \\p hGraph\n - cudaGraphExecUpdateErrorParametersChanged if any parameters to a node changed in a way\n   that is not supported, in which case \\p hErrorNode_out is set to the node from \\p hGraph\n - cudaGraphExecUpdateErrorAttributesChanged if any attributes of a node changed in a way\n   that is not supported, in which case \\p hErrorNode_out is set to the node from \\p hGraph\n - cudaGraphExecUpdateErrorNotSupported if something about a node is unsupported, like\n   the node's type or configuration, in which case \\p hErrorNode_out is set to the node from \\p hGraph\n\n If the update fails for a reason not listed above, the result member of \\p resultInfo will be set\n to cudaGraphExecUpdateError. If the update succeeds, the result member will be set to cudaGraphExecUpdateSuccess.\n\n cudaGraphExecUpdate returns cudaSuccess when the updated was performed successfully.  It returns\n cudaErrorGraphExecUpdateFailure if the graph update was not performed because it included\n changes which violated constraints specific to instantiated graph update.\n\n \\param hGraphExec The instantiated graph to be updated\n \\param hGraph The graph containing the updated parameters\n\\param resultInfo the error info structure\n\n \\return\n ::cudaSuccess,\n ::cudaErrorGraphExecUpdateFailure,\n \\note_graph_thread_safety\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa\n ::cudaGraphInstantiate"]
pub unsafe fn cudaGraphExecUpdate<T: ::cuda_libs::types::CudaAsPtr>(
    hGraphExec: cudaGraphExec_t,
    hGraph: cudaGraph_t,
    mut resultInfo: T,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphExecUpdate(
            hGraphExec,
            hGraph,
            resultInfo.as_mut_ptr() as *mut cudaGraphExecUpdateResultInfo,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudaGraphUpload(
    graphExec: cudaGraphExec_t,
    stream: cudaStream_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaGraphUpload(graphExec, stream) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Launches an executable graph in a stream\n\n Executes \\p graphExec in \\p stream. Only one instance of \\p graphExec may be executing\n at a time. Each launch is ordered behind both any previous work in \\p stream\n and any previous launches of \\p graphExec. To execute a graph concurrently, it must be\n instantiated multiple times into multiple executable graphs.\n\n If any allocations created by \\p graphExec remain unfreed (from a previous launch) and\n \\p graphExec was not instantiated with ::cudaGraphInstantiateFlagAutoFreeOnLaunch,\n the launch will fail with ::cudaErrorInvalidValue.\n\n \\param graphExec - Executable graph to launch\n \\param stream    - Stream in which to launch the graph\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue\n \\note_graph_thread_safety\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa\n ::cudaGraphInstantiate,\n ::cudaGraphUpload,\n ::cudaGraphExecDestroy"]
pub unsafe fn cudaGraphLaunch(
    graphExec: cudaGraphExec_t,
    stream: cudaStream_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaGraphLaunch(graphExec, stream) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Destroys an executable graph\n\n Destroys the executable graph specified by \\p graphExec.\n\n \\param graphExec - Executable graph to destroy\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue\n \\note_graph_thread_safety\n \\notefnerr\n \\note_init_rt\n \\note_callback\n \\note_destroy_ub\n\n \\sa\n ::cudaGraphInstantiate,\n ::cudaGraphUpload,\n ::cudaGraphLaunch"]
pub unsafe fn cudaGraphExecDestroy(
    graphExec: cudaGraphExec_t,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaGraphExecDestroy(graphExec) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Destroys a graph\n\n Destroys the graph specified by \\p graph, as well as all of its nodes.\n\n \\param graph - Graph to destroy\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue\n \\note_graph_thread_safety\n \\notefnerr\n \\note_init_rt\n \\note_callback\n \\note_destroy_ub\n\n \\sa\n ::cudaGraphCreate"]
pub unsafe fn cudaGraphDestroy(graph: cudaGraph_t) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaGraphDestroy(graph) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Write a DOT file describing graph structure\n\n Using the provided \\p graph, write to \\p path a DOT formatted description of the graph.\n By default this includes the graph topology, node types, node id, kernel names and memcpy direction.\n \\p flags can be specified to write more detailed information about each node type such as\n parameter values, kernel attributes, node and function handles.\n\n \\param graph - The graph to create a DOT file from\n \\param path  - The path to write the DOT file to\n \\param flags - Flags from cudaGraphDebugDotFlags for specifying which additional node information to write\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorOperatingSystem"]
pub unsafe fn cudaGraphDebugDotPrint<T: ::cuda_libs::types::CudaAsPtr>(
    graph: cudaGraph_t,
    path: T,
    flags: ::std::os::raw::c_uint,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphDebugDotPrint(
            graph,
            path.as_const_ptr() as *const ::std::os::raw::c_char,
            flags,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Create a user object\n\n Create a user object with the specified destructor callback and initial reference count. The\n initial references are owned by the caller.\n\n Destructor callbacks cannot make CUDA API calls and should avoid blocking behavior, as they\n are executed by a shared internal thread. Another thread may be signaled to perform such\n actions, if it does not block forward progress of tasks scheduled through CUDA.\n\n See CUDA User Objects in the CUDA C++ Programming Guide for more information on user objects.\n\n \\param object_out      - Location to return the user object handle\n \\param ptr             - The pointer to pass to the destroy function\n \\param destroy         - Callback to free the user object when it is no longer in use\n \\param initialRefcount - The initial refcount to create the object with, typically 1. The\n                          initial references are owned by the calling thread.\n \\param flags           - Currently it is required to pass ::cudaUserObjectNoDestructorSync,\n                          which is the only defined flag. This indicates that the destroy\n                          callback cannot be waited on by any CUDA API. Users requiring\n                          synchronization of the callback should signal its completion\n                          manually.\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue\n\n \\sa\n ::cudaUserObjectRetain,\n ::cudaUserObjectRelease,\n ::cudaGraphRetainUserObject,\n ::cudaGraphReleaseUserObject,\n ::cudaGraphCreate"]
pub unsafe fn cudaUserObjectCreate<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
>(
    mut object_out: T,
    mut ptr: U,
    destroy: cudaHostFn_t,
    initialRefcount: ::std::os::raw::c_uint,
    flags: ::std::os::raw::c_uint,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaUserObjectCreate(
            object_out.as_mut_ptr() as *mut cudaUserObject_t,
            ptr.as_mut_ptr() as *mut ::std::os::raw::c_void,
            destroy,
            initialRefcount,
            flags,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Retain a reference to a user object\n\n Retains new references to a user object. The new references are owned by the caller.\n\n See CUDA User Objects in the CUDA C++ Programming Guide for more information on user objects.\n\n \\param object - The object to retain\n \\param count  - The number of references to retain, typically 1. Must be nonzero\n                 and not larger than INT_MAX.\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue\n\n \\sa\n ::cudaUserObjectCreate,\n ::cudaUserObjectRelease,\n ::cudaGraphRetainUserObject,\n ::cudaGraphReleaseUserObject,\n ::cudaGraphCreate"]
pub unsafe fn cudaUserObjectRetain(
    object: cudaUserObject_t,
    count: ::std::os::raw::c_uint,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaUserObjectRetain(object, count) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Release a reference to a user object\n\n Releases user object references owned by the caller. The object's destructor is invoked if\n the reference count reaches zero.\n\n It is undefined behavior to release references not owned by the caller, or to use a user\n object handle after all references are released.\n\n See CUDA User Objects in the CUDA C++ Programming Guide for more information on user objects.\n\n \\param object - The object to release\n \\param count  - The number of references to release, typically 1. Must be nonzero\n                 and not larger than INT_MAX.\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue\n\n \\sa\n ::cudaUserObjectCreate,\n ::cudaUserObjectRetain,\n ::cudaGraphRetainUserObject,\n ::cudaGraphReleaseUserObject,\n ::cudaGraphCreate"]
pub unsafe fn cudaUserObjectRelease(
    object: cudaUserObject_t,
    count: ::std::os::raw::c_uint,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaUserObjectRelease(object, count) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Retain a reference to a user object from a graph\n\n Creates or moves user object references that will be owned by a CUDA graph.\n\n See CUDA User Objects in the CUDA C++ Programming Guide for more information on user objects.\n\n \\param graph  - The graph to associate the reference with\n \\param object - The user object to retain a reference for\n \\param count  - The number of references to add to the graph, typically 1. Must be\n                 nonzero and not larger than INT_MAX.\n \\param flags  - The optional flag ::cudaGraphUserObjectMove transfers references\n                 from the calling thread, rather than create new references. Pass 0\n                 to create new references.\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue\n\n \\sa\n ::cudaUserObjectCreate\n ::cudaUserObjectRetain,\n ::cudaUserObjectRelease,\n ::cudaGraphReleaseUserObject,\n ::cudaGraphCreate"]
pub unsafe fn cudaGraphRetainUserObject(
    graph: cudaGraph_t,
    object: cudaUserObject_t,
    count: ::std::os::raw::c_uint,
    flags: ::std::os::raw::c_uint,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaGraphRetainUserObject(graph, object, count, flags) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Release a user object reference from a graph\n\n Releases user object references owned by a graph.\n\n See CUDA User Objects in the CUDA C++ Programming Guide for more information on user objects.\n\n \\param graph  - The graph that will release the reference\n \\param object - The user object to release a reference for\n \\param count  - The number of references to release, typically 1. Must be nonzero\n                 and not larger than INT_MAX.\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue\n\n \\sa\n ::cudaUserObjectCreate\n ::cudaUserObjectRetain,\n ::cudaUserObjectRelease,\n ::cudaGraphRetainUserObject,\n ::cudaGraphCreate"]
pub unsafe fn cudaGraphReleaseUserObject(
    graph: cudaGraph_t,
    object: cudaUserObject_t,
    count: ::std::os::raw::c_uint,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe { crate::sys::cudaGraphReleaseUserObject(graph, object, count) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Adds a node of arbitrary type to a graph\n\n Creates a new node in \\p graph described by \\p nodeParams with \\p numDependencies\n dependencies specified via \\p pDependencies. \\p numDependencies may be 0.\n \\p pDependencies may be null if \\p numDependencies is 0. \\p pDependencies may not have\n any duplicate entries.\n\n \\p nodeParams is a tagged union. The node type should be specified in the \\p type field,\n and type-specific parameters in the corresponding union member. All unused bytes - that\n is, \\p reserved0 and all bytes past the utilized union member - must be set to zero.\n It is recommended to use brace initialization or memset to ensure all bytes are\n initialized.\n\n Note that for some node types, \\p nodeParams may contain \"out parameters\" which are\n modified during the call, such as \\p nodeParams->alloc.dptr.\n\n A handle to the new node will be returned in \\p phGraphNode.\n\n \\param pGraphNode      - Returns newly created node\n \\param graph           - Graph to which to add the node\n \\param pDependencies   - Dependencies of the node\n \\param dependencyData  - Optional edge data for the dependencies. If NULL, the data is\n                          assumed to be default (zeroed) for all dependencies.\n \\param numDependencies - Number of dependencies\n \\param nodeParams      - Specification of the node\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorInvalidDeviceFunction,\n ::cudaErrorNotSupported\n \\note_graph_thread_safety\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa\n ::cudaGraphCreate,\n ::cudaGraphNodeSetParams,\n ::cudaGraphExecNodeSetParams"]
pub unsafe fn cudaGraphAddNode<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
    W: ::cuda_libs::types::CudaAsPtr,
>(
    mut pGraphNode: T,
    graph: cudaGraph_t,
    pDependencies: U,
    dependencyData: V,
    numDependencies: usize,
    mut nodeParams: W,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphAddNode(
            pGraphNode.as_mut_ptr() as *mut cudaGraphNode_t,
            graph,
            pDependencies.as_const_ptr() as *const cudaGraphNode_t,
            dependencyData.as_const_ptr() as *const cudaGraphEdgeData,
            numDependencies,
            nodeParams.as_mut_ptr() as *mut cudaGraphNodeParams,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Update a graph node's parameters\n\n Sets the parameters of graph node \\p node to \\p nodeParams. The node type specified by\n \\p nodeParams->type must match the type of \\p node. \\p nodeParams must be fully\n initialized and all unused bytes (reserved, padding) zeroed.\n\n Modifying parameters is not supported for node types cudaGraphNodeTypeMemAlloc and\n cudaGraphNodeTypeMemFree.\n\n \\param node       - Node to set the parameters for\n \\param nodeParams - Parameters to copy\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorInvalidDeviceFunction,\n ::cudaErrorNotSupported\n \\note_graph_thread_safety\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa\n ::cudaGraphNodeGetParams,\n ::cudaGraphAddNode,\n ::cudaGraphExecNodeSetParams"]
pub unsafe fn cudaGraphNodeSetParams<T: ::cuda_libs::types::CudaAsPtr>(
    node: cudaGraphNode_t,
    mut nodeParams: T,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphNodeSetParams(
            node,
            nodeParams.as_mut_ptr() as *mut cudaGraphNodeParams,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Returns a graph node's parameters\n\n Returns the parameters of graph node \\p node in \\p *nodeParams.\n\n Any pointers returned in \\p *nodeParams point to driver-owned memory associated\n with the node. This memory remains valid until the node is destroyed.  Any memory\n pointed to from \\p *nodeParams must not be modified.\n\n The returned parameters are a description of the node, but may not be identical to the\n struct provided at creation and may not be suitable for direct creation of identical\n nodes. This is because parameters may be partially unspecified and filled in by the\n driver at creation, may reference non-copyable handles, or may describe ownership\n semantics or other parameters that govern behavior of node creation but are not part\n of the final functional descriptor.\n\n \\param node       - Node to get the parameters for\n \\param nodeParams - Pointer to return the parameters\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorNotSupported\n \\note_graph_thread_safety\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa\n ::cudaGraphNodeGetParams,\n ::cudaGraphNodeSetParams,\n ::cudaGraphAddNode,\n ::cudaGraphExecNodeSetParams"]
pub unsafe fn cudaGraphNodeGetParams(
    node: cudaGraphNode_t,
) -> Result<cudaGraphNodeParams, crate::sys::cudaError> {
    let mut out_1: std::mem::MaybeUninit<cudaGraphNodeParams> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaGraphNodeGetParams(node, out_1.as_mut_ptr() as *mut _) };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok(out_1.assume_init()) }
    } else {
        Err(status)
    }
}
#[doc = " \\brief Update a graph node's parameters in an instantiated graph\n\n Sets the parameters of a node in an executable graph \\p graphExec. The node is identified\n by the corresponding node \\p node in the non-executable graph from which the executable\n graph was instantiated. \\p node must not have been removed from the original graph.\n\n The modifications only affect future launches of \\p graphExec. Already\n enqueued or running launches of \\p graphExec are not affected by this call.\n \\p node is also not modified by this call.\n\n Allowed changes to parameters on executable graphs are as follows:\n <table>\n   <tr><th>Node type<th>Allowed changes\n   <tr><td>kernel<td>See ::cudaGraphExecKernelNodeSetParams\n   <tr><td>memcpy<td>Addresses for 1-dimensional copies if allocated in same context; see ::cudaGraphExecMemcpyNodeSetParams\n   <tr><td>memset<td>Addresses for 1-dimensional memsets if allocated in same context; see ::cudaGraphExecMemsetNodeSetParams\n   <tr><td>host<td>Unrestricted\n   <tr><td>child graph<td>Topology must match and restrictions apply recursively; see ::cudaGraphExecUpdate\n   <tr><td>event wait<td>Unrestricted\n   <tr><td>event record<td>Unrestricted\n   <tr><td>external semaphore signal<td>Number of semaphore operations cannot change\n   <tr><td>external semaphore wait<td>Number of semaphore operations cannot change\n   <tr><td>memory allocation<td>API unsupported\n   <tr><td>memory free<td>API unsupported\n </table>\n\n \\param graphExec  - The executable graph in which to update the specified node\n \\param node       - Corresponding node from the graph from which graphExec was instantiated\n \\param nodeParams - Updated Parameters to set\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorInvalidDeviceFunction,\n ::cudaErrorNotSupported\n \\note_graph_thread_safety\n \\notefnerr\n \\note_init_rt\n \\note_callback\n\n \\sa\n ::cudaGraphAddNode,\n ::cudaGraphNodeSetParams\n ::cudaGraphExecUpdate,\n ::cudaGraphInstantiate"]
pub unsafe fn cudaGraphExecNodeSetParams<T: ::cuda_libs::types::CudaAsPtr>(
    graphExec: cudaGraphExec_t,
    node: cudaGraphNode_t,
    mut nodeParams: T,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphExecNodeSetParams(
            graphExec,
            node,
            nodeParams.as_mut_ptr() as *mut cudaGraphNodeParams,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Create a conditional handle\n\n Creates a conditional handle associated with \\p hGraph.\n\n The conditional handle must be associated with a conditional node in this graph or one of its children.\n\n Handles not associated with a conditional node may cause graph instantiation to fail.\n\n \\param pHandle_out        - Pointer used to return the handle to the caller.\n \\param graph              - Graph which will contain the conditional node using this handle.\n \\param defaultLaunchValue - Optional initial value for the conditional variable.\n                             Applied at the beginning of each graph execution if cudaGraphCondAssignDefault is set in \\p flags.\n \\param flags              - Currently must be cudaGraphCondAssignDefault or 0.\n\n \\return\n ::CUDA_SUCCESS,\n ::CUDA_ERROR_INVALID_VALUE,\n ::CUDA_ERROR_NOT_SUPPORTED\n \\note_graph_thread_safety\n \\notefnerr\n\n \\sa\n ::cuGraphAddNode,"]
pub unsafe fn cudaGraphConditionalHandleCreate(
    pHandle_out: *mut cudaGraphConditionalHandle,
    graph: cudaGraph_t,
    defaultLaunchValue: ::std::os::raw::c_uint,
    flags: ::std::os::raw::c_uint,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGraphConditionalHandleCreate(pHandle_out, graph, defaultLaunchValue, flags)
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
pub unsafe fn cudaGetDriverEntryPoint(
    symbol: *const ::std::os::raw::c_char,
    funcPtr: *mut *mut ::std::os::raw::c_void,
    flags: ::std::os::raw::c_ulonglong,
) -> Result<cudaDriverEntryPointQueryResult, crate::sys::cudaError> {
    let mut out_3: std::mem::MaybeUninit<cudaDriverEntryPointQueryResult> =
        std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudaGetDriverEntryPoint(symbol, funcPtr, flags, out_3.as_mut_ptr() as *mut _)
    };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok(out_3.assume_init()) }
    } else {
        Err(status)
    }
}
pub unsafe fn cudaGetDriverEntryPointByVersion(
    symbol: *const ::std::os::raw::c_char,
    funcPtr: *mut *mut ::std::os::raw::c_void,
    cudaVersion: ::std::os::raw::c_uint,
    flags: ::std::os::raw::c_ulonglong,
) -> Result<cudaDriverEntryPointQueryResult, crate::sys::cudaError> {
    let mut out_4: std::mem::MaybeUninit<cudaDriverEntryPointQueryResult> =
        std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudaGetDriverEntryPointByVersion(
            symbol,
            funcPtr,
            cudaVersion,
            flags,
            out_4.as_mut_ptr() as *mut _,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok(out_4.assume_init()) }
    } else {
        Err(status)
    }
}
#[doc = " \\brief Sets information about a kernel\n\n This call sets the value of a specified attribute \\p attr on the kernel \\p kernel\n for the requested device \\p device to an integer value specified by \\p value.\n This function returns ::cudaSuccess if the new value of the attribute could be\n successfully set. If the set fails, this call will return an error.\n Not all attributes can have values set. Attempting to set a value on a read-only\n attribute will result in an error (::cudaErrorInvalidValue)\n\n Note that attributes set using ::cudaFuncSetAttribute() will override the attribute\n set by this API irrespective of whether the call to ::cudaFuncSetAttribute() is made\n before or after this API call. Because of this and the stricter locking requirements\n mentioned below it is suggested that this call be used during the initialization path\n and not on each thread accessing \\p kernel such as on kernel launches or on the\n critical path.\n\n Valid values for \\p attr are:\n - ::cudaFuncAttributeMaxDynamicSharedMemorySize - The requested maximum size in bytes of dynamically-allocated shared memory. The sum of this value and the function attribute ::sharedSizeBytes\n   cannot exceed the device attribute ::cudaDevAttrMaxSharedMemoryPerBlockOptin. The maximal size of requestable dynamic shared memory may differ by GPU architecture.\n - ::cudaFuncAttributePreferredSharedMemoryCarveout - On devices where the L1 cache and shared memory use the same hardware resources,\n   this sets the shared memory carveout preference, in percent of the total shared memory. See ::cudaDevAttrMaxSharedMemoryPerMultiprocessor.\n   This is only a hint, and the driver can choose a different ratio if required to execute the function.\n - ::cudaFuncAttributeRequiredClusterWidth: The required cluster width in\n   blocks. The width, height, and depth values must either all be 0 or all be\n   positive. The validity of the cluster dimensions is checked at launch time.\n   If the value is set during compile time, it cannot be set at runtime.\n   Setting it at runtime will return cudaErrorNotPermitted.\n - ::cudaFuncAttributeRequiredClusterHeight: The required cluster height in\n   blocks. The width, height, and depth values must either all be 0 or all be\n   positive. The validity of the cluster dimensions is checked at launch time.\n   If the value is set during compile time, it cannot be set at runtime.\n   Setting it at runtime will return cudaErrorNotPermitted.\n - ::cudaFuncAttributeRequiredClusterDepth: The required cluster depth in\n   blocks. The width, height, and depth values must either all be 0 or all be\n   positive. The validity of the cluster dimensions is checked at launch time.\n   If the value is set during compile time, it cannot be set at runtime.\n   Setting it at runtime will return cudaErrorNotPermitted.\n - ::cudaFuncAttributeNonPortableClusterSizeAllowed: Indicates whether the\n   function can be launched with non-portable cluster size. 1 is allowed, 0 is\n   disallowed.\n - ::cudaFuncAttributeClusterSchedulingPolicyPreference: The block\n   scheduling policy of a function. The value type is cudaClusterSchedulingPolicy.\n\n \\note The API has stricter locking requirements in comparison to its legacy counterpart\n ::cudaFuncSetAttribute() due to device-wide semantics. If multiple threads are trying to\n set the same attribute on the same device simultaneously, the attribute setting will depend\n on the interleavings chosen by the OS scheduler and memory consistency.\n\n \\param kernel  - Kernel to set attribute of\n \\param attr - Attribute requested\n \\param value - Value to set\n \\param device - Device to set attribute of\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidDeviceFunction,\n ::cudaErrorInvalidValue\n\n \\sa ::cudaLibraryLoadData,\n ::cudaLibraryLoadFromFile,\n ::cudaLibraryUnload,\n ::cudaLibraryGetKernel,\n ::cudaLaunchKernel,\n ::cudaFuncSetAttribute,\n ::cuKernelSetAttribute"]
pub unsafe fn cudaKernelSetAttributeForDevice(
    kernel: cudaKernel_t,
    attr: cudaFuncAttribute,
    value: ::std::os::raw::c_int,
    device: ::std::os::raw::c_int,
) -> Result<(), crate::sys::cudaError> {
    let status =
        unsafe { crate::sys::cudaKernelSetAttributeForDevice(kernel, attr, value, device) };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Get device resources\n\n Get the \\p type resources available to the \\p device.\n This may often be the starting point for further partitioning or configuring\n of resources.\n\n Note: The API is not supported on 32-bit platforms.\n\n \\param device - Device to get resource for\n \\param resource - Output pointer to a cudaDevResource structure\n \\param type - Type of resource to retrieve\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorNotPermitted,\n ::cudaErrorInvalidDevice,\n ::cudaErrorInvalidResourceType,\n ::cudaErrorNotSupported,\n ::cudaErrorCudartUnloading,\n ::cudaErrorInitializationError\n \\note_callback\n\n \\sa\n ::cuDeviceGetDevResource,\n ::cudaExecutionCtxGetDevResource,\n ::cudaDevSmResourceSplit,\n ::cudaDevResourceGenerateDesc"]
pub unsafe fn cudaDeviceGetDevResource(
    device: ::std::os::raw::c_int,
    type_: cudaDevResourceType,
) -> Result<cudaDevResource, crate::sys::cudaError> {
    let mut out_1: std::mem::MaybeUninit<cudaDevResource> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudaDeviceGetDevResource(device, out_1.as_mut_ptr() as *mut _, type_)
    };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok(out_1.assume_init()) }
    } else {
        Err(status)
    }
}
#[doc = " \\brief Splits \\p cudaDevResourceTypeSm resources.\n\n Splits \\p cudaDevResourceTypeSm resources into \\p nbGroups, adhering to the\n minimum SM count specified in \\p minCount and the usage flags in \\p flags.\n If \\p result is NULL, the API simulates a split and provides the amount of groups that\n would be created in \\p nbGroups. Otherwise, \\p nbGroups must point to the amount of elements\n in \\p result and on return, the API will overwrite \\p nbGroups with the amount actually created.\n The groups are written to the array in \\p result.\n \\p nbGroups can be less than the total amount if a smaller number of groups is needed.\n\n This API is used to spatially partition the input resource. The input resource needs to come\n from one of ::cudaDeviceGetDevResource, or ::cudaExecutionCtxGetDevResource.\n A limitation of the API is that the output results cannot be split again without\n first creating a descriptor and a green context with that descriptor.\n\n When creating the groups, the API will take into account the performance and functional\n characteristics of the input resource, and guarantee a split that will create a disjoint\n set of symmetrical partitions. This may lead to fewer groups created than purely dividing\n the total SM count by the \\p minCount due to cluster requirements or alignment and granularity\n requirements for the minCount.\n These requirements can be queried with ::cudaDeviceGetDevResource, or ::cudaExecutionCtxGetDevResource\n for ::cudaDevResourceTypeSm, using the \\p minSmPartitionSize and \\p smCoscheduledAlignment fields\n to determine minimum partition size and alignment granularity, respectively.\n\n The \\p remainder set does not have the same functional or performance guarantees as the groups\n in \\p result. Its use should be carefully planned and future partitions of the \\p remainder set\n are discouraged.\n\n The following flags are supported:\n - \\p cudaDevSmResourceSplitIgnoreSmCoscheduling : Lower the minimum SM count and alignment, and\n   treat each SM independent of its hierarchy. This allows more fine grained partitions but at the\n   cost of advanced features (such as large clusters on compute capability 9.0+).\n - \\p cudaDevSmResourceSplitMaxPotentialClusterSize : Compute Capability 9.0+ only. Attempt to\n   create groups that may allow for maximally sized thread clusters. This can be queried post\n   green context creation using ::cudaOccupancyMaxPotentialClusterSize.\n\n A successful API call must either have:\n - A valid array of \\p result pointers of size passed in \\p nbGroups, with \\p input of type\n   \\p cudaDevResourceTypeSm. Value of \\p minCount must be between 0 and the SM count specified\n   in \\p input. \\p remaining may be NULL.\n - NULL passed in for \\p result, with a valid integer pointer in \\p nbGroups and \\p input of\n   type \\p cudaDevResourceTypeSm. Value of \\p minCount must be between 0 and the SM count\n   specified in \\p input. \\p remaining may be NULL. This queries the number of groups that\n   would be created by the API.\n\n Note: The API is not supported on 32-bit platforms.\n\n \\param result - Output array of \\p cudaDevResource resources. Can be NULL to query the\n number of groups.\n \\param nbGroups - This is a pointer, specifying the number of groups that would be or\n should be created as described below.\n \\param input - Input SM resource to be split. Must be a valid \\p cudaDevSmResource resource.\n \\param remaining - If the input resource cannot be cleanly split among \\p nbGroups,\n the remaining is placed in here. Can be ommitted (NULL) if the user does not need the remaining set.\n \\param flags - Flags specifying how these partitions are used or which constraints to abide by\n when splitting the input. Zero is valid for default behavior.\n \\param minCount - Minimum number of SMs required\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorNotPermitted,\n ::cudaErrorInvalidResourceType,\n ::cudaErrorInvalidResourceConfiguration,\n ::cudaErrorNotSupported,\n ::cudaErrorCudartUnloading,\n ::cudaErrorInitializationError\n \\note_callback\n\n \\sa\n ::cuDevSmResourceSplitByCount,\n ::cudaDeviceGetDevResource,\n ::cudaExecutionCtxGetDevResource,\n ::cudaDevResourceGenerateDesc"]
pub unsafe fn cudaDevSmResourceSplitByCount<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
    W: ::cuda_libs::types::CudaAsPtr,
>(
    mut result: T,
    mut nbGroups: U,
    input: V,
    mut remaining: W,
    flags: ::std::os::raw::c_uint,
    minCount: ::std::os::raw::c_uint,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaDevSmResourceSplitByCount(
            result.as_mut_ptr() as *mut cudaDevResource,
            nbGroups.as_mut_ptr() as *mut ::std::os::raw::c_uint,
            input.as_const_ptr() as *const cudaDevResource,
            remaining.as_mut_ptr() as *mut cudaDevResource,
            flags,
            minCount,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Splits a \\p cudaDevResourceTypeSm resource into structured groups.\n\n This API will split a resource of ::cudaDevResourceTypeSm into \\p nbGroups structured device resource groups (the \\p result array),\n as well as an optional \\p remainder, according to a set of requirements specified in the \\p groupParams array. The term “structured”\n is a trait that specifies the \\p result has SMs that are co-scheduled together. This co-scheduling can be specified via the \\p coscheduledSmCount\n field of the \\p groupParams structure, while the \\p smCount will specify how many SMs are required in total for that result.\n The remainder is always “unstructured”, it does not have any set guarantees with respect to co-scheduling and those properties will need to\n either be queried via the occupancy set of APIs or further split into structured groups by this API.\n\n The API has a discovery mode for use cases where it is difficult to know ahead of time what the SM count should be.\n Discovery happens when the \\p smCount field of a given \\p groupParams array entry is set to 0 - the smCount will be filled in by the API\n with the derived SM count according to the provided \\p groupParams fields and constraints. Discovery can be used with both a valid result\n array and with a NULL \\p result pointer value. The latter is useful in situations where the smCount will end up being zero, which is an invalid\n value to create a result entry with, but allowed for discovery purposes when the \\p result is NULL.\n\n The \\p groupParams array is evaluated from index 0 to \\p nbGroups - 1. For each index in the \\p groupParams array,\n the API will evaluate which SMs may be a good fit based on constraints and assign those SMs to \\p result.\n This evaluation order is important to consider when using discovery mode, as it helps discover the remaining SMs.\n\n For a valid call:\n - \\p result should point to a \\p cudaDevResource array of size \\p nbGroups, or alternatively, may be NULL, if the developer wishes for only the groupParams entries to be updated\n\n - \\p input should be a valid ::cudaDevResourceTypeSm resource that originates from querying the execution context, or device.\n\n - The \\p remainder group may be NULL.\n\n - There are no API \\p flags at this time, so the value passed in should be 0.\n\n - A ::cudaDevSmResourceGroupParams array of size \\p nbGroups. Each entry must be zero-initialized.\n     - \\p smCount: must be either 0 or in the range of [2,inputSmCount] where inputSmCount is the amount of SMs the \\p input resource has.\n     \\p smCount must be a multiple of 2, as well as a multiple of \\p coscheduledSmCount. When assigning SMs to a group (and if results are\n     expected by having the \\p result parameter set), \\p smCount cannot end up with 0 or a value less than \\p coscheduledSmCount\n     otherwise ::cudaErrorInvalidResourceConfiguration will be returned.\n     - \\p coscheduledSmCount: allows grouping SMs together in order to be able to launch clusters on Compute Architecture 9.0+.\n     The default value may be queried from the device’s ::cudaDevResourceTypeSm resource (8 on Compute Architecture 9.0+ and 2 otherwise).\n     The maximum is 32 on Compute Architecture 9.0+ and 2 otherwise.\n     - \\p preferredCoscheduledSmCount: Attempts to merge \\p coscheduledSmCount groups into larger groups,\n     in order to make use of \\p preferredClusterDimensions on Compute Architecture 10.0+. The default value is set to \\p coscheduledSmCount.\n     - \\p flags:\n         - \\p cudaDevSmResourceGroupBackfill: lets \\p smCount be a non-multiple of \\p coscheduledSmCount, filling the difference between SM count\n         and already assigned co-scheduled groupings with other SMs. This lets any resulting group behave similar to the \\p remainder group for example.\n\n <b>Example params and their effect:</b>\n\n A groupParams array element is defined in the following order:\n \\code\n { .smCount, .coscheduledSmCount, .preferredCoscheduledSmCount, .flags, \\/\\* .reserved \\*\\/ }\n \\endcode\n\n \\code\n// Example 1\n// Will discover how many SMs there are, that are co-scheduled in groups of smCoscheduledAlignment.\n// The rest is placed in the optional remainder.\ncudaDevSmResourceGroupParams params { 0, 0, 0, 0 };\n \\endcode\n \\code\n// Example 2\n// Assuming the device has 10+ SMs, the result will have 10 SMs that are co-scheduled in groups of 2 SMs.\n// The rest is placed in the optional remainder.\ncudaDevSmResourceGroupParams params { 10, 2, 0, 0};\n// Setting the coscheduledSmCount to 2 guarantees that we can always have a valid result\n// as long as the SM count is less than or equal to the input resource SM count.\n \\endcode\n \\code\n// Example 3\n// A single piece is split-off, but instead of assigning the rest to the remainder, a second group contains everything else\n// This assumes the device has 10+ SMs (8 of which are coscheduled in groups of 4),\n// otherwise the second group could end up with 0 SMs, which is not allowed.\ncudaDevSmResourceGroupParams params { {8, 4, 0, 0}, {0, 2, 0, cudaDevSmResourceGroupBackfill } }\n \\endcode\n\n The difference between a catch-all param group as the last entry and the remainder is in two aspects:\n - The remainder may be NULL / _TYPE_INVALID (if there are no SMs remaining), while a result group must always be valid.\n - The remainder does not have a structure, while the result group will always need to adhere to a structure\n of coscheduledSmCount (even if its just 2), and therefore must always have enough coscheduled SMs to cover\n that requirement (even with the \\p cudaDevSmResourceGroupBackfill flag enabled).\n\n Splitting an input into N groups, can be accomplished by repeatedly splitting off 1 group and re-splitting\n the remainder (a bisect operation). However, it's recommended to accomplish this with a single call wherever possible.\n\n \\param result - Output array of \\p cudaDevResource resources. Can be NULL, alongside an smCount of 0, for discovery purpose.\n \\param nbGroups - Specifies the number of groups in \\p result and \\p groupParams\n \\param input - Input SM resource to be split. Must be a valid \\p cudaDevResourceTypeSm resource.\n \\param remainder - If splitting the input resource leaves any SMs, the remainder is placed in here.\n \\param flags - Flags specifying how the API should behave. The value should be 0 for now.\n \\param groupParams - Description of how the SMs should be split and assigned to the corresponding result entry.\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorNotPermitted,\n ::cudaErrorInvalidResourceType,\n ::cudaErrorInvalidResourceConfiguration,\n ::cudaErrorNotSupported,\n ::cudaErrorCudartUnloading,\n ::cudaErrorInitializationError\n \\note_callback\n\n \\sa\n ::cuDevSmResourceSplit,\n ::cudaDeviceGetDevResource,\n ::cudaExecutionCtxGetDevResource,\n ::cudaDevResourceGenerateDesc"]
pub unsafe fn cudaDevSmResourceSplit<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
    V: ::cuda_libs::types::CudaAsPtr,
    W: ::cuda_libs::types::CudaAsPtr,
>(
    mut result: T,
    nbGroups: ::std::os::raw::c_uint,
    input: U,
    mut remainder: V,
    flags: ::std::os::raw::c_uint,
    mut groupParams: W,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaDevSmResourceSplit(
            result.as_mut_ptr() as *mut cudaDevResource,
            nbGroups,
            input.as_const_ptr() as *const cudaDevResource,
            remainder.as_mut_ptr() as *mut cudaDevResource,
            flags,
            groupParams.as_mut_ptr() as *mut cudaDevSmResourceGroupParams,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Generate a resource descriptor\n\n Generates a single resource descriptor with the set of resources specified in \\p resources.\n The generated resource descriptor is necessary for the creation of green contexts via the\n ::cudaGreenCtxCreate API. Resources of the same type can be passed in, provided they meet\n the requirements as noted below.\n\n A successful API call must have:\n - A valid output pointer for the \\p phDesc descriptor as well as a valid array of \\p resources pointers,\n with the array size passed in \\p nbResources.\n If multiple resources are provided in \\p resources, the device they came from must be the same,\n otherwise ::cudaErrorInvalidResourceConfiguration is returned.\n If multiple resources are provided in \\p resources and they are of type ::cudaDevResourceTypeSm,\n they must be outputs (whether \\p result or \\p remaining) from the same split API instance and have\n the same smCoscheduledAlignment values, otherwise ::cudaErrorInvalidResourceConfiguration is returned.\n\n Note: The API is not supported on 32-bit platforms.\n\n \\param phDesc - Output descriptor\n \\param resources - Array of resources to be included in the descriptor\n \\param nbResources - Number of resources passed in \\p resources\n\n \\return\n ::cudaSuccess,\n ::cudaErrorInvalidValue,\n ::cudaErrorNotPermitted,\n ::cudaErrorInvalidResourceType,\n ::cudaErrorInvalidResourceConfiguration,\n ::cudaErrorNotSupported,\n ::cudaErrorOutOfMemory,\n ::cudaErrorCudartUnloading,\n ::cudaErrorInitializationError\n \\note_callback\n\n \\sa\n ::cuDevResourceGenerateDesc,\n ::cudaDeviceGetDevResource,\n ::cudaExecutionCtxGetDevResource,\n ::cudaDevSmResourceSplit,\n ::cudaGreenCtxCreate"]
pub unsafe fn cudaDevResourceGenerateDesc<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
>(
    mut phDesc: T,
    mut resources: U,
    nbResources: ::std::os::raw::c_uint,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaDevResourceGenerateDesc(
            phDesc.as_mut_ptr() as *mut cudaDevResourceDesc_t,
            resources.as_mut_ptr() as *mut cudaDevResource,
            nbResources,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Get stream resources\n\n Get the \\p type resources available to the \\p hStream and store them in \\p resource.\n\n Note: The API will return ::cudaErrorInvalidResourceType is \\p type is\n \\p cudaDevResourceTypeWorkqueueConfig or \\p cudaDevResourceTypeWorkqueue.\n\n \\param hStream - Stream to get resource for\n \\param resource - Output pointer to a cudaDevResource structure\n \\param type - Type of resource to retrieve\n\n \\return\n ::cudaSuccess,\n ::cudaErrorCudartUnloading,\n ::cudaErrorInitializationError,\n ::cudaErrorDeviceUninitialized,\n ::cudaErrorInvalidResourceType,\n ::cudaErrorInvalidValue,\n ::cudaErrorInvalidHandle,\n ::cudaErrorNotPermitted,\n ::cudaErrorCallRequiresNewerDriver,\n \\notefnerr\n \\note_callback\n\n \\sa\n ::cudaGreenCtxCreate,\n ::cudaExecutionCtxStreamCreate,\n ::cudaStreamCreate,\n ::cudaDevSmResourceSplit,\n ::cudaDevResourceGenerateDesc,\n ::cuStreamGetDevResource"]
pub unsafe fn cudaStreamGetDevResource(
    hStream: cudaStream_t,
    type_: cudaDevResourceType,
) -> Result<cudaDevResource, crate::sys::cudaError> {
    let mut out_1: std::mem::MaybeUninit<cudaDevResource> = std::mem::MaybeUninit::uninit();
    let status = unsafe {
        crate::sys::cudaStreamGetDevResource(hStream, out_1.as_mut_ptr() as *mut _, type_)
    };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok(out_1.assume_init()) }
    } else {
        Err(status)
    }
}
#[doc = " \\cond impl_private"]
pub unsafe fn cudaGetExportTable<
    T: ::cuda_libs::types::CudaAsPtr,
    U: ::cuda_libs::types::CudaAsPtr,
>(
    mut ppExportTable: T,
    pExportTableId: U,
) -> Result<(), crate::sys::cudaError> {
    let status = unsafe {
        crate::sys::cudaGetExportTable(
            ppExportTable.as_mut_ptr() as *mut *const ::std::os::raw::c_void,
            pExportTableId.as_const_ptr() as *const cudaUUID_t,
        )
    };
    if status == crate::sys::cudaError::cudaSuccess {
        Ok(())
    } else {
        Err(status)
    }
}
#[doc = " \\brief Get pointer to device entry function that matches entry function \\p symbolPtr\n\n Returns in \\p functionPtr the device entry function corresponding to the symbol \\p symbolPtr.\n\n \\param functionPtr     - Returns the device entry function\n \\param symbolPtr       - Pointer to device entry function to search for\n\n \\return\n ::cudaSuccess\n"]
pub unsafe fn cudaGetFuncBySymbol(
    symbolPtr: *const ::std::os::raw::c_void,
) -> Result<cudaFunction_t, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<cudaFunction_t> = std::mem::MaybeUninit::uninit();
    let status =
        unsafe { crate::sys::cudaGetFuncBySymbol(out_0.as_mut_ptr() as *mut _, symbolPtr) };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok(out_0.assume_init()) }
    } else {
        Err(status)
    }
}
#[doc = " \\brief Get pointer to device kernel that matches entry function \\p entryFuncAddr\n\n Returns in \\p kernelPtr the device kernel corresponding to the entry function \\p entryFuncAddr.\n\n Note that it is possible that there are multiple symbols belonging to different\n translation units with the same \\p entryFuncAddr registered with this CUDA Runtime\n and so the order which the translation units are loaded and registered with the\n CUDA Runtime can lead to differing return pointers in \\p kernelPtr .\n Suggested methods of ensuring uniqueness are to limit visibility of __global__\n device functions by using static or hidden visibility attribute in the\n respective translation units.\n\n \\param kernelPtr          - Returns the device kernel\n \\param entryFuncAddr      - Address of device entry function to search kernel for\n\n \\return\n ::cudaSuccess\n\n \\sa\n \\ref ::cudaGetKernel(cudaKernel_t *kernelPtr, const T *entryFuncAddr) \"cudaGetKernel (C++ API)\""]
pub unsafe fn cudaGetKernel(
    entryFuncAddr: *const ::std::os::raw::c_void,
) -> Result<cudaKernel_t, crate::sys::cudaError> {
    let mut out_0: std::mem::MaybeUninit<cudaKernel_t> = std::mem::MaybeUninit::uninit();
    let status = unsafe { crate::sys::cudaGetKernel(out_0.as_mut_ptr() as *mut _, entryFuncAddr) };
    if status == crate::sys::cudaError::cudaSuccess {
        unsafe { Ok(out_0.assume_init()) }
    } else {
        Err(status)
    }
}
