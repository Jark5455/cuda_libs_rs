pub use cuda_libs_cudart::sys::*;
pub const CURAND_VER_MAJOR: u32 = 10;
pub const CURAND_VER_MINOR: u32 = 4;
pub const CURAND_VER_PATCH: u32 = 2;
pub const CURAND_VER_BUILD: u32 = 55;
pub const CURAND_VERSION: u32 = 10402;
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
#[repr(u32)]
#[doc = " CURAND function call status types"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum curandStatus {
    #[doc = "< No errors"]
    CURAND_STATUS_SUCCESS = 0,
    #[doc = "< Header file and linked library version do not match"]
    CURAND_STATUS_VERSION_MISMATCH = 100,
    #[doc = "< Generator not initialized"]
    CURAND_STATUS_NOT_INITIALIZED = 101,
    #[doc = "< Memory allocation failed"]
    CURAND_STATUS_ALLOCATION_FAILED = 102,
    #[doc = "< Generator is wrong type"]
    CURAND_STATUS_TYPE_ERROR = 103,
    #[doc = "< Argument out of range"]
    CURAND_STATUS_OUT_OF_RANGE = 104,
    #[doc = "< Length requested is not a multple of dimension"]
    CURAND_STATUS_LENGTH_NOT_MULTIPLE = 105,
    #[doc = "< GPU does not have double precision required by MRG32k3a"]
    CURAND_STATUS_DOUBLE_PRECISION_REQUIRED = 106,
    #[doc = "< Kernel launch failure"]
    CURAND_STATUS_LAUNCH_FAILURE = 201,
    #[doc = "< Preexisting failure on library entry"]
    CURAND_STATUS_PREEXISTING_FAILURE = 202,
    #[doc = "< Initialization of CUDA failed"]
    CURAND_STATUS_INITIALIZATION_FAILED = 203,
    #[doc = "< Architecture mismatch, GPU does not support requested feature"]
    CURAND_STATUS_ARCH_MISMATCH = 204,
    #[doc = "< Internal library error"]
    CURAND_STATUS_INTERNAL_ERROR = 999,
}
#[doc = " \\cond UNHIDE_TYPEDEFS"]
pub use self::curandStatus as curandStatus_t;
#[repr(u32)]
#[doc = " CURAND generator types"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum curandRngType {
    CURAND_RNG_TEST = 0,
    #[doc = "< Default pseudorandom generator"]
    CURAND_RNG_PSEUDO_DEFAULT = 100,
    #[doc = "< XORWOW pseudorandom generator"]
    CURAND_RNG_PSEUDO_XORWOW = 101,
    #[doc = "< MRG32k3a pseudorandom generator"]
    CURAND_RNG_PSEUDO_MRG32K3A = 121,
    #[doc = "< Mersenne Twister MTGP32 pseudorandom generator"]
    CURAND_RNG_PSEUDO_MTGP32 = 141,
    #[doc = "< Mersenne Twister MT19937 pseudorandom generator"]
    CURAND_RNG_PSEUDO_MT19937 = 142,
    #[doc = "< PHILOX-4x32-10 pseudorandom generator"]
    CURAND_RNG_PSEUDO_PHILOX4_32_10 = 161,
    #[doc = "< Default quasirandom generator"]
    CURAND_RNG_QUASI_DEFAULT = 200,
    #[doc = "< Sobol32 quasirandom generator"]
    CURAND_RNG_QUASI_SOBOL32 = 201,
    #[doc = "< Scrambled Sobol32 quasirandom generator"]
    CURAND_RNG_QUASI_SCRAMBLED_SOBOL32 = 202,
    #[doc = "< Sobol64 quasirandom generator"]
    CURAND_RNG_QUASI_SOBOL64 = 203,
    #[doc = "< Scrambled Sobol64 quasirandom generator"]
    CURAND_RNG_QUASI_SCRAMBLED_SOBOL64 = 204,
}
#[doc = " \\cond UNHIDE_TYPEDEFS"]
pub use self::curandRngType as curandRngType_t;
#[repr(u32)]
#[doc = " CURAND ordering of results in memory"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum curandOrdering {
    #[doc = "< Best ordering for pseudorandom results"]
    CURAND_ORDERING_PSEUDO_BEST = 100,
    #[doc = "< Specific default thread sequence for pseudorandom results, same as CURAND_ORDERING_PSEUDO_BEST"]
    CURAND_ORDERING_PSEUDO_DEFAULT = 101,
    #[doc = "< Specific seeding pattern for fast lower quality pseudorandom results"]
    CURAND_ORDERING_PSEUDO_SEEDED = 102,
    #[doc = "< Specific legacy sequence for pseudorandom results, guaranteed to remain the same for all cuRAND release"]
    CURAND_ORDERING_PSEUDO_LEGACY = 103,
    #[doc = "< Specific ordering adjusted to the device it is being executed on, provides the best performance"]
    CURAND_ORDERING_PSEUDO_DYNAMIC = 104,
    #[doc = "< Specific n-dimensional ordering for quasirandom results"]
    CURAND_ORDERING_QUASI_DEFAULT = 201,
}
#[doc = " \\cond UNHIDE_TYPEDEFS"]
pub use self::curandOrdering as curandOrdering_t;
#[repr(u32)]
#[doc = " CURAND choice of direction vector set"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum curandDirectionVectorSet {
    #[doc = "< Specific set of 32-bit direction vectors generated from polynomials recommended by S. Joe and F. Y. Kuo, for up to 20,000 dimensions"]
    CURAND_DIRECTION_VECTORS_32_JOEKUO6 = 101,
    #[doc = "< Specific set of 32-bit direction vectors generated from polynomials recommended by S. Joe and F. Y. Kuo, for up to 20,000 dimensions, and scrambled"]
    CURAND_SCRAMBLED_DIRECTION_VECTORS_32_JOEKUO6 = 102,
    #[doc = "< Specific set of 64-bit direction vectors generated from polynomials recommended by S. Joe and F. Y. Kuo, for up to 20,000 dimensions"]
    CURAND_DIRECTION_VECTORS_64_JOEKUO6 = 103,
    #[doc = "< Specific set of 64-bit direction vectors generated from polynomials recommended by S. Joe and F. Y. Kuo, for up to 20,000 dimensions, and scrambled"]
    CURAND_SCRAMBLED_DIRECTION_VECTORS_64_JOEKUO6 = 104,
}
#[doc = " \\cond UNHIDE_TYPEDEFS"]
pub use self::curandDirectionVectorSet as curandDirectionVectorSet_t;
#[doc = " CURAND array of 32-bit direction vectors\n/\n/** \\cond UNHIDE_TYPEDEFS"]
pub type curandDirectionVectors32_t = [::std::os::raw::c_uint; 32usize];
#[doc = " CURAND array of 64-bit direction vectors\n/\n/** \\cond UNHIDE_TYPEDEFS"]
pub type curandDirectionVectors64_t = [::std::os::raw::c_ulonglong; 64usize];
#[doc = " CURAND generator (opaque)"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct curandGenerator_st {
    _unused: [u8; 0],
}
#[doc = " CURAND generator\n/\n/** \\cond UNHIDE_TYPEDEFS"]
pub type curandGenerator_t = *mut curandGenerator_st;
#[doc = " CURAND distribution\n/\n/** \\cond UNHIDE_TYPEDEFS"]
pub type curandDistribution_st = f64;
pub type curandDistribution_t = *mut curandDistribution_st;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct curandDistributionShift_st {
    _unused: [u8; 0],
}
pub type curandDistributionShift_t = *mut curandDistributionShift_st;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct curandDistributionM2Shift_st {
    _unused: [u8; 0],
}
#[doc = " \\endcond */\n/**\n CURAND distribution M2\n/\n/** \\cond UNHIDE_TYPEDEFS"]
pub type curandDistributionM2Shift_t = *mut curandDistributionM2Shift_st;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct curandHistogramM2_st {
    _unused: [u8; 0],
}
pub type curandHistogramM2_t = *mut curandHistogramM2_st;
pub type curandHistogramM2K_st = ::std::os::raw::c_uint;
pub type curandHistogramM2K_t = *mut curandHistogramM2K_st;
pub type curandHistogramM2V_st = curandDistribution_st;
pub type curandHistogramM2V_t = *mut curandHistogramM2V_st;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct curandDiscreteDistribution_st {
    _unused: [u8; 0],
}
pub type curandDiscreteDistribution_t = *mut curandDiscreteDistribution_st;
#[repr(u32)]
#[doc = " \\cond UNHIDE_ENUMS"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum curandMethod {
    CURAND_CHOOSE_BEST = 0,
    CURAND_ITR = 1,
    CURAND_KNUTH = 2,
    CURAND_HITR = 3,
    CURAND_M1 = 4,
    CURAND_M2 = 5,
    CURAND_BINARY_SEARCH = 6,
    CURAND_DISCRETE_GAUSS = 7,
    CURAND_REJECTION = 8,
    CURAND_DEVICE_API = 9,
    CURAND_FAST_REJECTION = 10,
    CURAND_3RD = 11,
    CURAND_DEFINITION = 12,
    CURAND_POISSON = 13,
}
#[doc = " \\cond UNHIDE_ENUMS"]
pub use self::curandMethod as curandMethod_t;
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = " \\brief Create new random number generator.\n\n Creates a new random number generator of type \\p rng_type\n and returns it in \\p *generator.\n\n Legal values for \\p rng_type are:\n - CURAND_RNG_PSEUDO_DEFAULT\n - CURAND_RNG_PSEUDO_XORWOW\n - CURAND_RNG_PSEUDO_MRG32K3A\n - CURAND_RNG_PSEUDO_MTGP32\n - CURAND_RNG_PSEUDO_MT19937\n - CURAND_RNG_PSEUDO_PHILOX4_32_10\n - CURAND_RNG_QUASI_DEFAULT\n - CURAND_RNG_QUASI_SOBOL32\n - CURAND_RNG_QUASI_SCRAMBLED_SOBOL32\n - CURAND_RNG_QUASI_SOBOL64\n - CURAND_RNG_QUASI_SCRAMBLED_SOBOL64\n\n When \\p rng_type is CURAND_RNG_PSEUDO_DEFAULT, the type chosen\n is CURAND_RNG_PSEUDO_XORWOW.  \\n\n When \\p rng_type is CURAND_RNG_QUASI_DEFAULT,\n the type chosen is CURAND_RNG_QUASI_SOBOL32.\n\n The default values for \\p rng_type = CURAND_RNG_PSEUDO_XORWOW are:\n - \\p seed = 0\n - \\p offset = 0\n - \\p ordering = CURAND_ORDERING_PSEUDO_DEFAULT\n\n The default values for \\p rng_type = CURAND_RNG_PSEUDO_MRG32K3A are:\n - \\p seed = 0\n - \\p offset = 0\n - \\p ordering = CURAND_ORDERING_PSEUDO_DEFAULT\n\n The default values for \\p rng_type = CURAND_RNG_PSEUDO_MTGP32 are:\n - \\p seed = 0\n - \\p offset = 0\n - \\p ordering = CURAND_ORDERING_PSEUDO_DEFAULT\n\n The default values for \\p rng_type = CURAND_RNG_PSEUDO_MT19937 are:\n - \\p seed = 0\n - \\p offset = 0\n - \\p ordering = CURAND_ORDERING_PSEUDO_DEFAULT\n\n * The default values for \\p rng_type = CURAND_RNG_PSEUDO_PHILOX4_32_10 are:\n - \\p seed = 0\n - \\p offset = 0\n - \\p ordering = CURAND_ORDERING_PSEUDO_DEFAULT\n\n The default values for \\p rng_type = CURAND_RNG_QUASI_SOBOL32 are:\n - \\p dimensions = 1\n - \\p offset = 0\n - \\p ordering = CURAND_ORDERING_QUASI_DEFAULT\n\n The default values for \\p rng_type = CURAND_RNG_QUASI_SOBOL64 are:\n - \\p dimensions = 1\n - \\p offset = 0\n - \\p ordering = CURAND_ORDERING_QUASI_DEFAULT\n\n The default values for \\p rng_type = CURAND_RNG_QUASI_SCRAMBBLED_SOBOL32 are:\n - \\p dimensions = 1\n - \\p offset = 0\n - \\p ordering = CURAND_ORDERING_QUASI_DEFAULT\n\n The default values for \\p rng_type = CURAND_RNG_QUASI_SCRAMBLED_SOBOL64 are:\n - \\p dimensions = 1\n - \\p offset = 0\n - \\p ordering = CURAND_ORDERING_QUASI_DEFAULT\n\n \\param generator - Pointer to generator\n \\param rng_type - Type of generator to create\n\n \\return\n - CURAND_STATUS_ALLOCATION_FAILED, if memory could not be allocated \\n\n - CURAND_STATUS_INITIALIZATION_FAILED if there was a problem setting up the GPU \\n\n - CURAND_STATUS_VERSION_MISMATCH if the header file version does not match the\n   dynamically linked library version \\n\n - CURAND_STATUS_TYPE_ERROR if the value for \\p rng_type is invalid \\n\n - CURAND_STATUS_SUCCESS if generator was created successfully \\n\n"]
    pub fn curandCreateGenerator(
        generator: *mut curandGenerator_t,
        rng_type: curandRngType_t,
    ) -> curandStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = " \\brief Create new host CPU random number generator.\n\n Creates a new host CPU random number generator of type \\p rng_type\n and returns it in \\p *generator.\n\n Legal values for \\p rng_type are:\n - CURAND_RNG_PSEUDO_DEFAULT\n - CURAND_RNG_PSEUDO_XORWOW\n - CURAND_RNG_PSEUDO_MRG32K3A\n - CURAND_RNG_PSEUDO_MTGP32\n - CURAND_RNG_PSEUDO_MT19937\n - CURAND_RNG_PSEUDO_PHILOX4_32_10\n - CURAND_RNG_QUASI_DEFAULT\n - CURAND_RNG_QUASI_SOBOL32\n - CURAND_RNG_QUASI_SCRAMBLED_SOBOL32\n - CURAND_RNG_QUASI_SOBOL64\n - CURAND_RNG_QUASI_SCRAMBLED_SOBOL64\n\n When \\p rng_type is CURAND_RNG_PSEUDO_DEFAULT, the type chosen\n is CURAND_RNG_PSEUDO_XORWOW.  \\n\n When \\p rng_type is CURAND_RNG_QUASI_DEFAULT,\n the type chosen is CURAND_RNG_QUASI_SOBOL32.\n\n The default values for \\p rng_type = CURAND_RNG_PSEUDO_XORWOW are:\n - \\p seed = 0\n - \\p offset = 0\n - \\p ordering = CURAND_ORDERING_PSEUDO_DEFAULT\n\n The default values for \\p rng_type = CURAND_RNG_PSEUDO_MRG32K3A are:\n - \\p seed = 0\n - \\p offset = 0\n - \\p ordering = CURAND_ORDERING_PSEUDO_DEFAULT\n\n The default values for \\p rng_type = CURAND_RNG_PSEUDO_MTGP32 are:\n - \\p seed = 0\n - \\p offset = 0\n - \\p ordering = CURAND_ORDERING_PSEUDO_DEFAULT\n\n The default values for \\p rng_type = CURAND_RNG_PSEUDO_MT19937 are:\n - \\p seed = 0\n - \\p offset = 0\n - \\p ordering = CURAND_ORDERING_PSEUDO_DEFAULT\n\n * The default values for \\p rng_type = CURAND_RNG_PSEUDO_PHILOX4_32_10 are:\n - \\p seed = 0\n - \\p offset = 0\n - \\p ordering = CURAND_ORDERING_PSEUDO_DEFAULT\n\n The default values for \\p rng_type = CURAND_RNG_QUASI_SOBOL32 are:\n - \\p dimensions = 1\n - \\p offset = 0\n - \\p ordering = CURAND_ORDERING_QUASI_DEFAULT\n\n The default values for \\p rng_type = CURAND_RNG_QUASI_SOBOL64 are:\n - \\p dimensions = 1\n - \\p offset = 0\n - \\p ordering = CURAND_ORDERING_QUASI_DEFAULT\n\n The default values for \\p rng_type = CURAND_RNG_QUASI_SCRAMBLED_SOBOL32 are:\n - \\p dimensions = 1\n - \\p offset = 0\n - \\p ordering = CURAND_ORDERING_QUASI_DEFAULT\n\n The default values for \\p rng_type = CURAND_RNG_QUASI_SCRAMBLED_SOBOL64 are:\n - \\p dimensions = 1\n - \\p offset = 0\n - \\p ordering = CURAND_ORDERING_QUASI_DEFAULT\n\n \\param generator - Pointer to generator\n \\param rng_type - Type of generator to create\n\n \\return\n - CURAND_STATUS_ALLOCATION_FAILED if memory could not be allocated \\n\n - CURAND_STATUS_INITIALIZATION_FAILED if there was a problem setting up the GPU \\n\n - CURAND_STATUS_VERSION_MISMATCH if the header file version does not match the\n   dynamically linked library version \\n\n - CURAND_STATUS_TYPE_ERROR if the value for \\p rng_type is invalid \\n\n - CURAND_STATUS_SUCCESS if generator was created successfully \\n"]
    pub fn curandCreateGeneratorHost(
        generator: *mut curandGenerator_t,
        rng_type: curandRngType_t,
    ) -> curandStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = " \\brief Destroy an existing generator.\n\n Destroy an existing generator and free all memory associated with its state.\n\n \\param generator - Generator to destroy\n\n \\return\n - CURAND_STATUS_NOT_INITIALIZED if the generator was never created \\n\n - CURAND_STATUS_SUCCESS if generator was destroyed successfully \\n"]
    pub fn curandDestroyGenerator(generator: curandGenerator_t) -> curandStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = " \\brief Return the version number of the library.\n\n Return in \\p *version the version number of the dynamically linked CURAND\n library.  The format is the same as CUDART_VERSION from the CUDA Runtime.\n The only supported configuration is CURAND version equal to CUDA Runtime\n version.\n\n \\param version - CURAND library version\n\n \\return\n - CURAND_STATUS_SUCCESS if the version number was successfully returned \\n"]
    pub fn curandGetVersion(version: *mut ::std::os::raw::c_int) -> curandStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = " \\brief Return the value of the curand property.\n\n Return in \\p *value the number for the property described by \\p type of the\n dynamically linked CURAND library.\n\n \\param type - CUDA library property\n \\param value - integer value for the requested property\n\n \\return\n - CURAND_STATUS_SUCCESS if the property value was successfully returned \\n\n - CURAND_STATUS_OUT_OF_RANGE if the property type is not recognized \\n"]
    pub fn curandGetProperty(
        type_: libraryPropertyType,
        value: *mut ::std::os::raw::c_int,
    ) -> curandStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = " \\brief Set the current stream for CURAND kernel launches.\n\n Set the current stream for CURAND kernel launches.  All library functions\n will use this stream until set again.\n\n \\param generator - Generator to modify\n \\param stream - Stream to use or ::NULL for null stream\n\n \\return\n - CURAND_STATUS_NOT_INITIALIZED if the generator was never created \\n\n - CURAND_STATUS_SUCCESS if stream was set successfully \\n"]
    pub fn curandSetStream(generator: curandGenerator_t, stream: cudaStream_t) -> curandStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = " \\brief Set the seed value of the pseudo-random number generator.\n\n Set the seed value of the pseudorandom number generator.\n All values of seed are valid.  Different seeds will produce different sequences.\n Different seeds will often not be statistically correlated with each other,\n but some pairs of seed values may generate sequences which are statistically correlated.\n\n \\param generator - Generator to modify\n \\param seed - Seed value\n\n \\return\n - CURAND_STATUS_NOT_INITIALIZED if the generator was never created \\n\n - CURAND_STATUS_TYPE_ERROR if the generator is not a pseudorandom number generator \\n\n - CURAND_STATUS_SUCCESS if generator seed was set successfully \\n"]
    pub fn curandSetPseudoRandomGeneratorSeed(
        generator: curandGenerator_t,
        seed: ::std::os::raw::c_ulonglong,
    ) -> curandStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = " \\brief Set the absolute offset of the pseudo or quasirandom number generator.\n\n Set the absolute offset of the pseudo or quasirandom number generator.\n\n All values of offset are valid.  The offset position is absolute, not\n relative to the current position in the sequence.\n\n \\param generator - Generator to modify\n \\param offset - Absolute offset position\n\n \\return\n - CURAND_STATUS_NOT_INITIALIZED if the generator was never created \\n\n - CURAND_STATUS_SUCCESS if generator offset was set successfully \\n"]
    pub fn curandSetGeneratorOffset(
        generator: curandGenerator_t,
        offset: ::std::os::raw::c_ulonglong,
    ) -> curandStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = " \\brief Set the ordering of results of the pseudo or quasirandom number generator.\n\n Set the ordering of results of the pseudo or quasirandom number generator.\n\n Legal values of \\p order for pseudorandom generators are:\n - CURAND_ORDERING_PSEUDO_DEFAULT\n - CURAND_ORDERING_PSEUDO_BEST\n - CURAND_ORDERING_PSEUDO_SEEDED\n - CURAND_ORDERING_PSEUDO_LEGACY\n\n Legal values of \\p order for quasirandom generators are:\n - CURAND_ORDERING_QUASI_DEFAULT\n\n \\param generator - Generator to modify\n \\param order - Ordering of results\n\n \\return\n - CURAND_STATUS_NOT_INITIALIZED if the generator was never created \\n\n - CURAND_STATUS_OUT_OF_RANGE if the ordering is not valid \\n\n - CURAND_STATUS_SUCCESS if generator ordering was set successfully \\n"]
    pub fn curandSetGeneratorOrdering(
        generator: curandGenerator_t,
        order: curandOrdering_t,
    ) -> curandStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = " \\brief Set the number of dimensions.\n\n Set the number of dimensions to be generated by the quasirandom number\n generator.\n\n Legal values for \\p num_dimensions are 1 to 20000.\n\n \\param generator - Generator to modify\n \\param num_dimensions - Number of dimensions\n\n \\return\n - CURAND_STATUS_NOT_INITIALIZED if the generator was never created \\n\n - CURAND_STATUS_OUT_OF_RANGE if num_dimensions is not valid \\n\n - CURAND_STATUS_TYPE_ERROR if the generator is not a quasirandom number generator \\n\n - CURAND_STATUS_SUCCESS if generator ordering was set successfully \\n"]
    pub fn curandSetQuasiRandomGeneratorDimensions(
        generator: curandGenerator_t,
        num_dimensions: ::std::os::raw::c_uint,
    ) -> curandStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = " \\brief Generate 32-bit pseudo or quasirandom numbers.\n\n Use \\p generator to generate \\p num 32-bit results into the device memory at\n \\p outputPtr.  The device memory must have been previously allocated and be\n large enough to hold all the results.  Launches are done with the stream\n set using ::curandSetStream(), or the null stream if no stream has been set.\n\n Results are 32-bit values with every bit random.\n\n \\param generator - Generator to use\n \\param outputPtr - Pointer to device memory to store CUDA-generated results, or\n                 Pointer to host memory to store CPU-generated results\n \\param num - Number of random 32-bit values to generate\n\n \\return\n - CURAND_STATUS_ALLOCATION_FAILED if memory could not be allocated \\n\n - CURAND_STATUS_NOT_INITIALIZED if the generator was never created \\n\n - CURAND_STATUS_PREEXISTING_FAILURE if there was an existing error from\n     a previous kernel launch \\n\n - CURAND_STATUS_LENGTH_NOT_MULTIPLE if the number of output samples is\n    not a multiple of the quasirandom dimension \\n\n - CURAND_STATUS_LAUNCH_FAILURE if the kernel launch failed for any reason \\n\n - CURAND_STATUS_TYPE_ERROR if the generator is a 64 bit quasirandom generator.\n (use ::curandGenerateLongLong() with 64 bit quasirandom generators)\n - CURAND_STATUS_SUCCESS if the results were generated successfully \\n"]
    pub fn curandGenerate(
        generator: curandGenerator_t,
        outputPtr: *mut ::std::os::raw::c_uint,
        num: usize,
    ) -> curandStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = " \\brief Generate 64-bit quasirandom numbers.\n\n Use \\p generator to generate \\p num 64-bit results into the device memory at\n \\p outputPtr.  The device memory must have been previously allocated and be\n large enough to hold all the results.  Launches are done with the stream\n set using ::curandSetStream(), or the null stream if no stream has been set.\n\n Results are 64-bit values with every bit random.\n\n \\param generator - Generator to use\n \\param outputPtr - Pointer to device memory to store CUDA-generated results, or\n                 Pointer to host memory to store CPU-generated results\n \\param num - Number of random 64-bit values to generate\n\n \\return\n - CURAND_STATUS_NOT_INITIALIZED if the generator was never created \\n\n - CURAND_STATUS_PREEXISTING_FAILURE if there was an existing error from\n     a previous kernel launch \\n\n - CURAND_STATUS_LENGTH_NOT_MULTIPLE if the number of output samples is\n    not a multiple of the quasirandom dimension \\n\n - CURAND_STATUS_LAUNCH_FAILURE if the kernel launch failed for any reason \\n\n - CURAND_STATUS_TYPE_ERROR if the generator is not a 64 bit quasirandom generator\\n\n - CURAND_STATUS_SUCCESS if the results were generated successfully \\n"]
    pub fn curandGenerateLongLong(
        generator: curandGenerator_t,
        outputPtr: *mut ::std::os::raw::c_ulonglong,
        num: usize,
    ) -> curandStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = " \\brief Generate uniformly distributed floats.\n\n Use \\p generator to generate \\p num float results into the device memory at\n \\p outputPtr.  The device memory must have been previously allocated and be\n large enough to hold all the results.  Launches are done with the stream\n set using ::curandSetStream(), or the null stream if no stream has been set.\n\n Results are 32-bit floating point values between \\p 0.0f and \\p 1.0f,\n excluding \\p 0.0f and including \\p 1.0f.\n\n \\param generator - Generator to use\n \\param outputPtr - Pointer to device memory to store CUDA-generated results, or\n                 Pointer to host memory to store CPU-generated results\n \\param num - Number of floats to generate\n\n \\return\n - CURAND_STATUS_ALLOCATION_FAILED if memory could not be allocated \\n\n - CURAND_STATUS_NOT_INITIALIZED if the generator was never created \\n\n - CURAND_STATUS_PREEXISTING_FAILURE if there was an existing error from\n    a previous kernel launch \\n\n - CURAND_STATUS_LAUNCH_FAILURE if the kernel launch failed for any reason \\n\n - CURAND_STATUS_LENGTH_NOT_MULTIPLE if the number of output samples is\n    not a multiple of the quasirandom dimension \\n\n - CURAND_STATUS_SUCCESS if the results were generated successfully \\n"]
    pub fn curandGenerateUniform(
        generator: curandGenerator_t,
        outputPtr: *mut f32,
        num: usize,
    ) -> curandStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = " \\brief Generate uniformly distributed doubles.\n\n Use \\p generator to generate \\p num double results into the device memory at\n \\p outputPtr.  The device memory must have been previously allocated and be\n large enough to hold all the results.  Launches are done with the stream\n set using ::curandSetStream(), or the null stream if no stream has been set.\n\n Results are 64-bit double precision floating point values between\n \\p 0.0 and \\p 1.0, excluding \\p 0.0 and including \\p 1.0.\n\n \\param generator - Generator to use\n \\param outputPtr - Pointer to device memory to store CUDA-generated results, or\n                 Pointer to host memory to store CPU-generated results\n \\param num - Number of doubles to generate\n\n \\return\n - CURAND_STATUS_ALLOCATION_FAILED if memory could not be allocated \\n\n - CURAND_STATUS_NOT_INITIALIZED if the generator was never created \\n\n - CURAND_STATUS_PREEXISTING_FAILURE if there was an existing error from\n    a previous kernel launch \\n\n - CURAND_STATUS_LAUNCH_FAILURE if the kernel launch failed for any reason \\n\n - CURAND_STATUS_LENGTH_NOT_MULTIPLE if the number of output samples is\n    not a multiple of the quasirandom dimension \\n\n - CURAND_STATUS_DOUBLE_PRECISION_REQUIRED if the GPU does not support double precision \\n\n - CURAND_STATUS_SUCCESS if the results were generated successfully \\n"]
    pub fn curandGenerateUniformDouble(
        generator: curandGenerator_t,
        outputPtr: *mut f64,
        num: usize,
    ) -> curandStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = " \\brief Generate normally distributed doubles.\n\n Use \\p generator to generate \\p n float results into the device memory at\n \\p outputPtr.  The device memory must have been previously allocated and be\n large enough to hold all the results.  Launches are done with the stream\n set using ::curandSetStream(), or the null stream if no stream has been set.\n\n Results are 32-bit floating point values with mean \\p mean and standard\n deviation \\p stddev.\n\n Normally distributed results are generated from pseudorandom generators\n with a Box-Muller transform, and so require \\p n to be even.\n Quasirandom generators use an inverse cumulative distribution\n function to preserve dimensionality.\n\n There may be slight numerical differences between results generated\n on the GPU with generators created with ::curandCreateGenerator()\n and results calculated on the CPU with generators created with\n ::curandCreateGeneratorHost().  These differences arise because of\n differences in results for transcendental functions.  In addition,\n future versions of CURAND may use newer versions of the CUDA math\n library, so different versions of CURAND may give slightly different\n numerical values.\n\n \\param generator - Generator to use\n \\param outputPtr - Pointer to device memory to store CUDA-generated results, or\n                 Pointer to host memory to store CPU-generated results\n \\param n - Number of floats to generate\n \\param mean - Mean of normal distribution\n \\param stddev - Standard deviation of normal distribution\n\n \\return\n - CURAND_STATUS_ALLOCATION_FAILED if memory could not be allocated \\n\n - CURAND_STATUS_NOT_INITIALIZED if the generator was never created \\n\n - CURAND_STATUS_PREEXISTING_FAILURE if there was an existing error from\n    a previous kernel launch \\n\n - CURAND_STATUS_LAUNCH_FAILURE if the kernel launch failed for any reason \\n\n - CURAND_STATUS_LENGTH_NOT_MULTIPLE if the number of output samples is\n    not a multiple of the quasirandom dimension, or is not a multiple\n    of two for pseudorandom generators \\n\n - CURAND_STATUS_SUCCESS if the results were generated successfully \\n"]
    pub fn curandGenerateNormal(
        generator: curandGenerator_t,
        outputPtr: *mut f32,
        n: usize,
        mean: f32,
        stddev: f32,
    ) -> curandStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = " \\brief Generate normally distributed doubles.\n\n Use \\p generator to generate \\p n double results into the device memory at\n \\p outputPtr.  The device memory must have been previously allocated and be\n large enough to hold all the results.  Launches are done with the stream\n set using ::curandSetStream(), or the null stream if no stream has been set.\n\n Results are 64-bit floating point values with mean \\p mean and standard\n deviation \\p stddev.\n\n Normally distributed results are generated from pseudorandom generators\n with a Box-Muller transform, and so require \\p n to be even.\n Quasirandom generators use an inverse cumulative distribution\n function to preserve dimensionality.\n\n There may be slight numerical differences between results generated\n on the GPU with generators created with ::curandCreateGenerator()\n and results calculated on the CPU with generators created with\n ::curandCreateGeneratorHost().  These differences arise because of\n differences in results for transcendental functions.  In addition,\n future versions of CURAND may use newer versions of the CUDA math\n library, so different versions of CURAND may give slightly different\n numerical values.\n\n \\param generator - Generator to use\n \\param outputPtr - Pointer to device memory to store CUDA-generated results, or\n                 Pointer to host memory to store CPU-generated results\n \\param n - Number of doubles to generate\n \\param mean - Mean of normal distribution\n \\param stddev - Standard deviation of normal distribution\n\n \\return\n - CURAND_STATUS_ALLOCATION_FAILED if memory could not be allocated \\n\n - CURAND_STATUS_NOT_INITIALIZED if the generator was never created \\n\n - CURAND_STATUS_PREEXISTING_FAILURE if there was an existing error from\n    a previous kernel launch \\n\n - CURAND_STATUS_LAUNCH_FAILURE if the kernel launch failed for any reason \\n\n - CURAND_STATUS_LENGTH_NOT_MULTIPLE if the number of output samples is\n    not a multiple of the quasirandom dimension, or is not a multiple\n    of two for pseudorandom generators \\n\n - CURAND_STATUS_DOUBLE_PRECISION_REQUIRED if the GPU does not support double precision \\n\n - CURAND_STATUS_SUCCESS if the results were generated successfully \\n"]
    pub fn curandGenerateNormalDouble(
        generator: curandGenerator_t,
        outputPtr: *mut f64,
        n: usize,
        mean: f64,
        stddev: f64,
    ) -> curandStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = " \\brief Generate log-normally distributed floats.\n\n Use \\p generator to generate \\p n float results into the device memory at\n \\p outputPtr.  The device memory must have been previously allocated and be\n large enough to hold all the results.  Launches are done with the stream\n set using ::curandSetStream(), or the null stream if no stream has been set.\n\n Results are 32-bit floating point values with log-normal distribution based on\n an associated normal distribution with mean \\p mean and standard deviation \\p stddev.\n\n Normally distributed results are generated from pseudorandom generators\n with a Box-Muller transform, and so require \\p n to be even.\n Quasirandom generators use an inverse cumulative distribution\n function to preserve dimensionality.\n The normally distributed results are transformed into log-normal distribution.\n\n There may be slight numerical differences between results generated\n on the GPU with generators created with ::curandCreateGenerator()\n and results calculated on the CPU with generators created with\n ::curandCreateGeneratorHost().  These differences arise because of\n differences in results for transcendental functions.  In addition,\n future versions of CURAND may use newer versions of the CUDA math\n library, so different versions of CURAND may give slightly different\n numerical values.\n\n \\param generator - Generator to use\n \\param outputPtr - Pointer to device memory to store CUDA-generated results, or\n                 Pointer to host memory to store CPU-generated results\n \\param n - Number of floats to generate\n \\param mean - Mean of associated normal distribution\n \\param stddev - Standard deviation of associated normal distribution\n\n \\return\n - CURAND_STATUS_ALLOCATION_FAILED if memory could not be allocated \\n\n - CURAND_STATUS_NOT_INITIALIZED if the generator was never created \\n\n - CURAND_STATUS_PREEXISTING_FAILURE if there was an existing error from\n    a previous kernel launch \\n\n - CURAND_STATUS_LAUNCH_FAILURE if the kernel launch failed for any reason \\n\n - CURAND_STATUS_LENGTH_NOT_MULTIPLE if the number of output samples is\n    not a multiple of the quasirandom dimension, or is not a multiple\n    of two for pseudorandom generators \\n\n - CURAND_STATUS_SUCCESS if the results were generated successfully \\n"]
    pub fn curandGenerateLogNormal(
        generator: curandGenerator_t,
        outputPtr: *mut f32,
        n: usize,
        mean: f32,
        stddev: f32,
    ) -> curandStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = " \\brief Generate log-normally distributed doubles.\n\n Use \\p generator to generate \\p n double results into the device memory at\n \\p outputPtr.  The device memory must have been previously allocated and be\n large enough to hold all the results.  Launches are done with the stream\n set using ::curandSetStream(), or the null stream if no stream has been set.\n\n Results are 64-bit floating point values with log-normal distribution based on\n an associated normal distribution with mean \\p mean and standard deviation \\p stddev.\n\n Normally distributed results are generated from pseudorandom generators\n with a Box-Muller transform, and so require \\p n to be even.\n Quasirandom generators use an inverse cumulative distribution\n function to preserve dimensionality.\n The normally distributed results are transformed into log-normal distribution.\n\n There may be slight numerical differences between results generated\n on the GPU with generators created with ::curandCreateGenerator()\n and results calculated on the CPU with generators created with\n ::curandCreateGeneratorHost().  These differences arise because of\n differences in results for transcendental functions.  In addition,\n future versions of CURAND may use newer versions of the CUDA math\n library, so different versions of CURAND may give slightly different\n numerical values.\n\n \\param generator - Generator to use\n \\param outputPtr - Pointer to device memory to store CUDA-generated results, or\n                 Pointer to host memory to store CPU-generated results\n \\param n - Number of doubles to generate\n \\param mean - Mean of normal distribution\n \\param stddev - Standard deviation of normal distribution\n\n \\return\n - CURAND_STATUS_ALLOCATION_FAILED if memory could not be allocated \\n\n - CURAND_STATUS_NOT_INITIALIZED if the generator was never created \\n\n - CURAND_STATUS_PREEXISTING_FAILURE if there was an existing error from\n    a previous kernel launch \\n\n - CURAND_STATUS_LAUNCH_FAILURE if the kernel launch failed for any reason \\n\n - CURAND_STATUS_LENGTH_NOT_MULTIPLE if the number of output samples is\n    not a multiple of the quasirandom dimension, or is not a multiple\n    of two for pseudorandom generators \\n\n - CURAND_STATUS_DOUBLE_PRECISION_REQUIRED if the GPU does not support double precision \\n\n - CURAND_STATUS_SUCCESS if the results were generated successfully \\n"]
    pub fn curandGenerateLogNormalDouble(
        generator: curandGenerator_t,
        outputPtr: *mut f64,
        n: usize,
        mean: f64,
        stddev: f64,
    ) -> curandStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = " \\brief Construct the histogram array for a Poisson distribution.\n\n Construct the histogram array for the Poisson distribution with lambda \\p lambda.\n For lambda greater than 2000, an approximation with a normal distribution is used.\n\n \\param lambda - lambda for the Poisson distribution\n\n\n \\param discrete_distribution - pointer to the histogram in device memory\n\n \\return\n - CURAND_STATUS_ALLOCATION_FAILED if memory could not be allocated \\n\n - CURAND_STATUS_DOUBLE_PRECISION_REQUIRED if the GPU does not support double precision \\n\n - CURAND_STATUS_INITIALIZATION_FAILED if there was a problem setting up the GPU \\n\n - CURAND_STATUS_NOT_INITIALIZED if the distribution pointer was null \\n\n - CURAND_STATUS_PREEXISTING_FAILURE if there was an existing error from\n    a previous kernel launch \\n\n - CURAND_STATUS_OUT_OF_RANGE if lambda is non-positive or greater than 400,000 \\n\n - CURAND_STATUS_SUCCESS if the histogram was generated successfully \\n"]
    pub fn curandCreatePoissonDistribution(
        lambda: f64,
        discrete_distribution: *mut curandDiscreteDistribution_t,
    ) -> curandStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = " \\brief Destroy the histogram array for a discrete distribution (e.g. Poisson).\n\n Destroy the histogram array for a discrete distribution created by curandCreatePoissonDistribution.\n\n \\param discrete_distribution - pointer to device memory where the histogram is stored\n\n \\return\n - CURAND_STATUS_NOT_INITIALIZED if the histogram was never created \\n\n - CURAND_STATUS_SUCCESS if the histogram was destroyed successfully \\n"]
    pub fn curandDestroyDistribution(
        discrete_distribution: curandDiscreteDistribution_t,
    ) -> curandStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = " \\brief Generate Poisson-distributed unsigned ints.\n\n Use \\p generator to generate \\p n unsigned int results into device memory at\n \\p outputPtr.  The device memory must have been previously allocated and must be\n large enough to hold all the results.  Launches are done with the stream\n set using ::curandSetStream(), or the null stream if no stream has been set.\n\n Results are 32-bit unsigned int point values with Poisson distribution, with lambda \\p lambda.\n\n \\param generator - Generator to use\n \\param outputPtr - Pointer to device memory to store CUDA-generated results, or\n                 Pointer to host memory to store CPU-generated results\n \\param n - Number of unsigned ints to generate\n \\param lambda - lambda for the Poisson distribution\n\n \\return\n - CURAND_STATUS_ALLOCATION_FAILED if memory could not be allocated \\n\n - CURAND_STATUS_NOT_INITIALIZED if the generator was never created \\n\n - CURAND_STATUS_PREEXISTING_FAILURE if there was an existing error from\n    a previous kernel launch \\n\n - CURAND_STATUS_LAUNCH_FAILURE if the kernel launch failed for any reason \\n\n - CURAND_STATUS_LENGTH_NOT_MULTIPLE if the number of output samples is\n    not a multiple of the quasirandom dimension\\n\n - CURAND_STATUS_DOUBLE_PRECISION_REQUIRED if the GPU or sm does not support double precision \\n\n - CURAND_STATUS_OUT_OF_RANGE if lambda is non-positive or greater than 400,000 \\n\n - CURAND_STATUS_SUCCESS if the results were generated successfully \\n"]
    pub fn curandGeneratePoisson(
        generator: curandGenerator_t,
        outputPtr: *mut ::std::os::raw::c_uint,
        n: usize,
        lambda: f64,
    ) -> curandStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn curandGeneratePoissonMethod(
        generator: curandGenerator_t,
        outputPtr: *mut ::std::os::raw::c_uint,
        n: usize,
        lambda: f64,
        method: curandMethod_t,
    ) -> curandStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn curandGenerateBinomial(
        generator: curandGenerator_t,
        outputPtr: *mut ::std::os::raw::c_uint,
        num: usize,
        n: ::std::os::raw::c_uint,
        p: f64,
    ) -> curandStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    pub fn curandGenerateBinomialMethod(
        generator: curandGenerator_t,
        outputPtr: *mut ::std::os::raw::c_uint,
        num: usize,
        n: ::std::os::raw::c_uint,
        p: f64,
        method: curandMethod_t,
    ) -> curandStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = " \\brief Setup starting states.\n\n Generate the starting state of the generator.  This function is\n automatically called by generation functions such as\n ::curandGenerate() and ::curandGenerateUniform().\n It can be called manually for performance testing reasons to separate\n timings for starting state generation and random number generation.\n\n \\param generator - Generator to update\n\n \\return\n - CURAND_STATUS_ALLOCATION_FAILED if memory could not be allocated \\n\n - CURAND_STATUS_NOT_INITIALIZED if the generator was never created \\n\n - CURAND_STATUS_PREEXISTING_FAILURE if there was an existing error from\n     a previous kernel launch \\n\n - CURAND_STATUS_LAUNCH_FAILURE if the kernel launch failed for any reason \\n\n - CURAND_STATUS_SUCCESS if the seeds were generated successfully \\n"]
    pub fn curandGenerateSeeds(generator: curandGenerator_t) -> curandStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = " \\brief Get direction vectors for 32-bit quasirandom number generation.\n\n Get a pointer to an array of direction vectors that can be used\n for quasirandom number generation.  The resulting pointer will\n reference an array of direction vectors in host memory.\n\n The array contains vectors for many dimensions.  Each dimension\n has 32 vectors.  Each individual vector is an unsigned int.\n\n Legal values for \\p set are:\n - CURAND_DIRECTION_VECTORS_32_JOEKUO6 (20,000 dimensions)\n - CURAND_SCRAMBLED_DIRECTION_VECTORS_32_JOEKUO6 (20,000 dimensions)\n\n \\param vectors - Address of pointer in which to return direction vectors\n \\param set - Which set of direction vectors to use\n\n \\return\n - CURAND_STATUS_OUT_OF_RANGE if the choice of set is invalid \\n\n - CURAND_STATUS_SUCCESS if the pointer was set successfully \\n"]
    pub fn curandGetDirectionVectors32(
        vectors: *mut *mut curandDirectionVectors32_t,
        set: curandDirectionVectorSet_t,
    ) -> curandStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = " \\brief Get scramble constants for 32-bit scrambled Sobol' .\n\n Get a pointer to an array of scramble constants that can be used\n for quasirandom number generation.  The resulting pointer will\n reference an array of unsinged ints in host memory.\n\n The array contains constants for many dimensions.  Each dimension\n has a single unsigned int constant.\n\n \\param constants - Address of pointer in which to return scramble constants\n\n \\return\n - CURAND_STATUS_SUCCESS if the pointer was set successfully \\n"]
    pub fn curandGetScrambleConstants32(
        constants: *mut *mut ::std::os::raw::c_uint,
    ) -> curandStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = " \\brief Get direction vectors for 64-bit quasirandom number generation.\n\n Get a pointer to an array of direction vectors that can be used\n for quasirandom number generation.  The resulting pointer will\n reference an array of direction vectors in host memory.\n\n The array contains vectors for many dimensions.  Each dimension\n has 64 vectors.  Each individual vector is an unsigned long long.\n\n Legal values for \\p set are:\n - CURAND_DIRECTION_VECTORS_64_JOEKUO6 (20,000 dimensions)\n - CURAND_SCRAMBLED_DIRECTION_VECTORS_64_JOEKUO6 (20,000 dimensions)\n\n \\param vectors - Address of pointer in which to return direction vectors\n \\param set - Which set of direction vectors to use\n\n \\return\n - CURAND_STATUS_OUT_OF_RANGE if the choice of set is invalid \\n\n - CURAND_STATUS_SUCCESS if the pointer was set successfully \\n"]
    pub fn curandGetDirectionVectors64(
        vectors: *mut *mut curandDirectionVectors64_t,
        set: curandDirectionVectorSet_t,
    ) -> curandStatus_t;
}
#[cfg(not(feature = "runtime-link"))]
unsafe extern "C" {
    #[doc = " \\brief Get scramble constants for 64-bit scrambled Sobol' .\n\n Get a pointer to an array of scramble constants that can be used\n for quasirandom number generation.  The resulting pointer will\n reference an array of unsinged long longs in host memory.\n\n The array contains constants for many dimensions.  Each dimension\n has a single unsigned long long constant.\n\n \\param constants - Address of pointer in which to return scramble constants\n\n \\return\n - CURAND_STATUS_SUCCESS if the pointer was set successfully \\n"]
    pub fn curandGetScrambleConstants64(
        constants: *mut *mut ::std::os::raw::c_ulonglong,
    ) -> curandStatus_t;
}
#[cfg(feature = "runtime-link")]
pub struct DynamicBindings {
    pub curandCreateGenerator: Option<
        unsafe extern "C" fn(
            generator: *mut curandGenerator_t,
            rng_type: curandRngType_t,
        ) -> curandStatus_t,
    >,
    pub curandCreateGeneratorHost: Option<
        unsafe extern "C" fn(
            generator: *mut curandGenerator_t,
            rng_type: curandRngType_t,
        ) -> curandStatus_t,
    >,
    pub curandDestroyGenerator:
        Option<unsafe extern "C" fn(generator: curandGenerator_t) -> curandStatus_t>,
    pub curandGetVersion:
        Option<unsafe extern "C" fn(version: *mut ::std::os::raw::c_int) -> curandStatus_t>,
    pub curandGetProperty: Option<
        unsafe extern "C" fn(
            type_: libraryPropertyType,
            value: *mut ::std::os::raw::c_int,
        ) -> curandStatus_t,
    >,
    pub curandSetStream: Option<
        unsafe extern "C" fn(generator: curandGenerator_t, stream: cudaStream_t) -> curandStatus_t,
    >,
    pub curandSetPseudoRandomGeneratorSeed: Option<
        unsafe extern "C" fn(
            generator: curandGenerator_t,
            seed: ::std::os::raw::c_ulonglong,
        ) -> curandStatus_t,
    >,
    pub curandSetGeneratorOffset: Option<
        unsafe extern "C" fn(
            generator: curandGenerator_t,
            offset: ::std::os::raw::c_ulonglong,
        ) -> curandStatus_t,
    >,
    pub curandSetGeneratorOrdering: Option<
        unsafe extern "C" fn(
            generator: curandGenerator_t,
            order: curandOrdering_t,
        ) -> curandStatus_t,
    >,
    pub curandSetQuasiRandomGeneratorDimensions: Option<
        unsafe extern "C" fn(
            generator: curandGenerator_t,
            num_dimensions: ::std::os::raw::c_uint,
        ) -> curandStatus_t,
    >,
    pub curandGenerate: Option<
        unsafe extern "C" fn(
            generator: curandGenerator_t,
            outputPtr: *mut ::std::os::raw::c_uint,
            num: usize,
        ) -> curandStatus_t,
    >,
    pub curandGenerateLongLong: Option<
        unsafe extern "C" fn(
            generator: curandGenerator_t,
            outputPtr: *mut ::std::os::raw::c_ulonglong,
            num: usize,
        ) -> curandStatus_t,
    >,
    pub curandGenerateUniform: Option<
        unsafe extern "C" fn(
            generator: curandGenerator_t,
            outputPtr: *mut f32,
            num: usize,
        ) -> curandStatus_t,
    >,
    pub curandGenerateUniformDouble: Option<
        unsafe extern "C" fn(
            generator: curandGenerator_t,
            outputPtr: *mut f64,
            num: usize,
        ) -> curandStatus_t,
    >,
    pub curandGenerateNormal: Option<
        unsafe extern "C" fn(
            generator: curandGenerator_t,
            outputPtr: *mut f32,
            n: usize,
            mean: f32,
            stddev: f32,
        ) -> curandStatus_t,
    >,
    pub curandGenerateNormalDouble: Option<
        unsafe extern "C" fn(
            generator: curandGenerator_t,
            outputPtr: *mut f64,
            n: usize,
            mean: f64,
            stddev: f64,
        ) -> curandStatus_t,
    >,
    pub curandGenerateLogNormal: Option<
        unsafe extern "C" fn(
            generator: curandGenerator_t,
            outputPtr: *mut f32,
            n: usize,
            mean: f32,
            stddev: f32,
        ) -> curandStatus_t,
    >,
    pub curandGenerateLogNormalDouble: Option<
        unsafe extern "C" fn(
            generator: curandGenerator_t,
            outputPtr: *mut f64,
            n: usize,
            mean: f64,
            stddev: f64,
        ) -> curandStatus_t,
    >,
    pub curandCreatePoissonDistribution: Option<
        unsafe extern "C" fn(
            lambda: f64,
            discrete_distribution: *mut curandDiscreteDistribution_t,
        ) -> curandStatus_t,
    >,
    pub curandDestroyDistribution: Option<
        unsafe extern "C" fn(discrete_distribution: curandDiscreteDistribution_t) -> curandStatus_t,
    >,
    pub curandGeneratePoisson: Option<
        unsafe extern "C" fn(
            generator: curandGenerator_t,
            outputPtr: *mut ::std::os::raw::c_uint,
            n: usize,
            lambda: f64,
        ) -> curandStatus_t,
    >,
    pub curandGeneratePoissonMethod: Option<
        unsafe extern "C" fn(
            generator: curandGenerator_t,
            outputPtr: *mut ::std::os::raw::c_uint,
            n: usize,
            lambda: f64,
            method: curandMethod_t,
        ) -> curandStatus_t,
    >,
    pub curandGenerateBinomial: Option<
        unsafe extern "C" fn(
            generator: curandGenerator_t,
            outputPtr: *mut ::std::os::raw::c_uint,
            num: usize,
            n: ::std::os::raw::c_uint,
            p: f64,
        ) -> curandStatus_t,
    >,
    pub curandGenerateBinomialMethod: Option<
        unsafe extern "C" fn(
            generator: curandGenerator_t,
            outputPtr: *mut ::std::os::raw::c_uint,
            num: usize,
            n: ::std::os::raw::c_uint,
            p: f64,
            method: curandMethod_t,
        ) -> curandStatus_t,
    >,
    pub curandGenerateSeeds:
        Option<unsafe extern "C" fn(generator: curandGenerator_t) -> curandStatus_t>,
    pub curandGetDirectionVectors32: Option<
        unsafe extern "C" fn(
            vectors: *mut *mut curandDirectionVectors32_t,
            set: curandDirectionVectorSet_t,
        ) -> curandStatus_t,
    >,
    pub curandGetScrambleConstants32:
        Option<unsafe extern "C" fn(constants: *mut *mut ::std::os::raw::c_uint) -> curandStatus_t>,
    pub curandGetDirectionVectors64: Option<
        unsafe extern "C" fn(
            vectors: *mut *mut curandDirectionVectors64_t,
            set: curandDirectionVectorSet_t,
        ) -> curandStatus_t,
    >,
    pub curandGetScrambleConstants64: Option<
        unsafe extern "C" fn(constants: *mut *mut ::std::os::raw::c_ulonglong) -> curandStatus_t,
    >,
}
#[cfg(feature = "runtime-link")]
pub static DYNAMIC_BINDINGS: std::sync::OnceLock<Box<DynamicBindings>> = std::sync::OnceLock::new();
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn curandCreateGenerator(
    generator: *mut curandGenerator_t,
    rng_type: curandRngType_t,
) -> curandStatus_t {
    match DYNAMIC_BINDINGS
        .get()
        .expect("CUDA library not loaded. Did you forget to call #[cuda_load]?")
        .curandCreateGenerator
    {
        Some(____func) => unsafe { ____func(generator, rng_type) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "curandCreateGenerator"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn curandCreateGeneratorHost(
    generator: *mut curandGenerator_t,
    rng_type: curandRngType_t,
) -> curandStatus_t {
    match DYNAMIC_BINDINGS
        .get()
        .expect("CUDA library not loaded. Did you forget to call #[cuda_load]?")
        .curandCreateGeneratorHost
    {
        Some(____func) => unsafe { ____func(generator, rng_type) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "curandCreateGeneratorHost"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn curandDestroyGenerator(generator: curandGenerator_t) -> curandStatus_t {
    match DYNAMIC_BINDINGS
        .get()
        .expect("CUDA library not loaded. Did you forget to call #[cuda_load]?")
        .curandDestroyGenerator
    {
        Some(____func) => unsafe { ____func(generator) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "curandDestroyGenerator"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn curandGetVersion(version: *mut ::std::os::raw::c_int) -> curandStatus_t {
    match DYNAMIC_BINDINGS
        .get()
        .expect("CUDA library not loaded. Did you forget to call #[cuda_load]?")
        .curandGetVersion
    {
        Some(____func) => unsafe { ____func(version) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "curandGetVersion"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn curandGetProperty(
    type_: libraryPropertyType,
    value: *mut ::std::os::raw::c_int,
) -> curandStatus_t {
    match DYNAMIC_BINDINGS
        .get()
        .expect("CUDA library not loaded. Did you forget to call #[cuda_load]?")
        .curandGetProperty
    {
        Some(____func) => unsafe { ____func(type_, value) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "curandGetProperty"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn curandSetStream(
    generator: curandGenerator_t,
    stream: cudaStream_t,
) -> curandStatus_t {
    match DYNAMIC_BINDINGS
        .get()
        .expect("CUDA library not loaded. Did you forget to call #[cuda_load]?")
        .curandSetStream
    {
        Some(____func) => unsafe { ____func(generator, stream) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "curandSetStream"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn curandSetPseudoRandomGeneratorSeed(
    generator: curandGenerator_t,
    seed: ::std::os::raw::c_ulonglong,
) -> curandStatus_t {
    match DYNAMIC_BINDINGS
        .get()
        .expect("CUDA library not loaded. Did you forget to call #[cuda_load]?")
        .curandSetPseudoRandomGeneratorSeed
    {
        Some(____func) => unsafe { ____func(generator, seed) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "curandSetPseudoRandomGeneratorSeed"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn curandSetGeneratorOffset(
    generator: curandGenerator_t,
    offset: ::std::os::raw::c_ulonglong,
) -> curandStatus_t {
    match DYNAMIC_BINDINGS
        .get()
        .expect("CUDA library not loaded. Did you forget to call #[cuda_load]?")
        .curandSetGeneratorOffset
    {
        Some(____func) => unsafe { ____func(generator, offset) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "curandSetGeneratorOffset"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn curandSetGeneratorOrdering(
    generator: curandGenerator_t,
    order: curandOrdering_t,
) -> curandStatus_t {
    match DYNAMIC_BINDINGS
        .get()
        .expect("CUDA library not loaded. Did you forget to call #[cuda_load]?")
        .curandSetGeneratorOrdering
    {
        Some(____func) => unsafe { ____func(generator, order) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "curandSetGeneratorOrdering"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn curandSetQuasiRandomGeneratorDimensions(
    generator: curandGenerator_t,
    num_dimensions: ::std::os::raw::c_uint,
) -> curandStatus_t {
    match DYNAMIC_BINDINGS
        .get()
        .expect("CUDA library not loaded. Did you forget to call #[cuda_load]?")
        .curandSetQuasiRandomGeneratorDimensions
    {
        Some(____func) => unsafe { ____func(generator, num_dimensions) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "curandSetQuasiRandomGeneratorDimensions"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn curandGenerate(
    generator: curandGenerator_t,
    outputPtr: *mut ::std::os::raw::c_uint,
    num: usize,
) -> curandStatus_t {
    match DYNAMIC_BINDINGS
        .get()
        .expect("CUDA library not loaded. Did you forget to call #[cuda_load]?")
        .curandGenerate
    {
        Some(____func) => unsafe { ____func(generator, outputPtr, num) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "curandGenerate"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn curandGenerateLongLong(
    generator: curandGenerator_t,
    outputPtr: *mut ::std::os::raw::c_ulonglong,
    num: usize,
) -> curandStatus_t {
    match DYNAMIC_BINDINGS
        .get()
        .expect("CUDA library not loaded. Did you forget to call #[cuda_load]?")
        .curandGenerateLongLong
    {
        Some(____func) => unsafe { ____func(generator, outputPtr, num) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "curandGenerateLongLong"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn curandGenerateUniform(
    generator: curandGenerator_t,
    outputPtr: *mut f32,
    num: usize,
) -> curandStatus_t {
    match DYNAMIC_BINDINGS
        .get()
        .expect("CUDA library not loaded. Did you forget to call #[cuda_load]?")
        .curandGenerateUniform
    {
        Some(____func) => unsafe { ____func(generator, outputPtr, num) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "curandGenerateUniform"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn curandGenerateUniformDouble(
    generator: curandGenerator_t,
    outputPtr: *mut f64,
    num: usize,
) -> curandStatus_t {
    match DYNAMIC_BINDINGS
        .get()
        .expect("CUDA library not loaded. Did you forget to call #[cuda_load]?")
        .curandGenerateUniformDouble
    {
        Some(____func) => unsafe { ____func(generator, outputPtr, num) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "curandGenerateUniformDouble"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn curandGenerateNormal(
    generator: curandGenerator_t,
    outputPtr: *mut f32,
    n: usize,
    mean: f32,
    stddev: f32,
) -> curandStatus_t {
    match DYNAMIC_BINDINGS
        .get()
        .expect("CUDA library not loaded. Did you forget to call #[cuda_load]?")
        .curandGenerateNormal
    {
        Some(____func) => unsafe { ____func(generator, outputPtr, n, mean, stddev) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "curandGenerateNormal"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn curandGenerateNormalDouble(
    generator: curandGenerator_t,
    outputPtr: *mut f64,
    n: usize,
    mean: f64,
    stddev: f64,
) -> curandStatus_t {
    match DYNAMIC_BINDINGS
        .get()
        .expect("CUDA library not loaded. Did you forget to call #[cuda_load]?")
        .curandGenerateNormalDouble
    {
        Some(____func) => unsafe { ____func(generator, outputPtr, n, mean, stddev) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "curandGenerateNormalDouble"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn curandGenerateLogNormal(
    generator: curandGenerator_t,
    outputPtr: *mut f32,
    n: usize,
    mean: f32,
    stddev: f32,
) -> curandStatus_t {
    match DYNAMIC_BINDINGS
        .get()
        .expect("CUDA library not loaded. Did you forget to call #[cuda_load]?")
        .curandGenerateLogNormal
    {
        Some(____func) => unsafe { ____func(generator, outputPtr, n, mean, stddev) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "curandGenerateLogNormal"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn curandGenerateLogNormalDouble(
    generator: curandGenerator_t,
    outputPtr: *mut f64,
    n: usize,
    mean: f64,
    stddev: f64,
) -> curandStatus_t {
    match DYNAMIC_BINDINGS
        .get()
        .expect("CUDA library not loaded. Did you forget to call #[cuda_load]?")
        .curandGenerateLogNormalDouble
    {
        Some(____func) => unsafe { ____func(generator, outputPtr, n, mean, stddev) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "curandGenerateLogNormalDouble"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn curandCreatePoissonDistribution(
    lambda: f64,
    discrete_distribution: *mut curandDiscreteDistribution_t,
) -> curandStatus_t {
    match DYNAMIC_BINDINGS
        .get()
        .expect("CUDA library not loaded. Did you forget to call #[cuda_load]?")
        .curandCreatePoissonDistribution
    {
        Some(____func) => unsafe { ____func(lambda, discrete_distribution) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "curandCreatePoissonDistribution"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn curandDestroyDistribution(
    discrete_distribution: curandDiscreteDistribution_t,
) -> curandStatus_t {
    match DYNAMIC_BINDINGS
        .get()
        .expect("CUDA library not loaded. Did you forget to call #[cuda_load]?")
        .curandDestroyDistribution
    {
        Some(____func) => unsafe { ____func(discrete_distribution) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "curandDestroyDistribution"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn curandGeneratePoisson(
    generator: curandGenerator_t,
    outputPtr: *mut ::std::os::raw::c_uint,
    n: usize,
    lambda: f64,
) -> curandStatus_t {
    match DYNAMIC_BINDINGS
        .get()
        .expect("CUDA library not loaded. Did you forget to call #[cuda_load]?")
        .curandGeneratePoisson
    {
        Some(____func) => unsafe { ____func(generator, outputPtr, n, lambda) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "curandGeneratePoisson"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn curandGeneratePoissonMethod(
    generator: curandGenerator_t,
    outputPtr: *mut ::std::os::raw::c_uint,
    n: usize,
    lambda: f64,
    method: curandMethod_t,
) -> curandStatus_t {
    match DYNAMIC_BINDINGS
        .get()
        .expect("CUDA library not loaded. Did you forget to call #[cuda_load]?")
        .curandGeneratePoissonMethod
    {
        Some(____func) => unsafe { ____func(generator, outputPtr, n, lambda, method) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "curandGeneratePoissonMethod"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn curandGenerateBinomial(
    generator: curandGenerator_t,
    outputPtr: *mut ::std::os::raw::c_uint,
    num: usize,
    n: ::std::os::raw::c_uint,
    p: f64,
) -> curandStatus_t {
    match DYNAMIC_BINDINGS
        .get()
        .expect("CUDA library not loaded. Did you forget to call #[cuda_load]?")
        .curandGenerateBinomial
    {
        Some(____func) => unsafe { ____func(generator, outputPtr, num, n, p) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "curandGenerateBinomial"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn curandGenerateBinomialMethod(
    generator: curandGenerator_t,
    outputPtr: *mut ::std::os::raw::c_uint,
    num: usize,
    n: ::std::os::raw::c_uint,
    p: f64,
    method: curandMethod_t,
) -> curandStatus_t {
    match DYNAMIC_BINDINGS
        .get()
        .expect("CUDA library not loaded. Did you forget to call #[cuda_load]?")
        .curandGenerateBinomialMethod
    {
        Some(____func) => unsafe { ____func(generator, outputPtr, num, n, p, method) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "curandGenerateBinomialMethod"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn curandGenerateSeeds(generator: curandGenerator_t) -> curandStatus_t {
    match DYNAMIC_BINDINGS
        .get()
        .expect("CUDA library not loaded. Did you forget to call #[cuda_load]?")
        .curandGenerateSeeds
    {
        Some(____func) => unsafe { ____func(generator) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "curandGenerateSeeds"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn curandGetDirectionVectors32(
    vectors: *mut *mut curandDirectionVectors32_t,
    set: curandDirectionVectorSet_t,
) -> curandStatus_t {
    match DYNAMIC_BINDINGS
        .get()
        .expect("CUDA library not loaded. Did you forget to call #[cuda_load]?")
        .curandGetDirectionVectors32
    {
        Some(____func) => unsafe { ____func(vectors, set) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "curandGetDirectionVectors32"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn curandGetScrambleConstants32(
    constants: *mut *mut ::std::os::raw::c_uint,
) -> curandStatus_t {
    match DYNAMIC_BINDINGS
        .get()
        .expect("CUDA library not loaded. Did you forget to call #[cuda_load]?")
        .curandGetScrambleConstants32
    {
        Some(____func) => unsafe { ____func(constants) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "curandGetScrambleConstants32"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn curandGetDirectionVectors64(
    vectors: *mut *mut curandDirectionVectors64_t,
    set: curandDirectionVectorSet_t,
) -> curandStatus_t {
    match DYNAMIC_BINDINGS
        .get()
        .expect("CUDA library not loaded. Did you forget to call #[cuda_load]?")
        .curandGetDirectionVectors64
    {
        Some(____func) => unsafe { ____func(vectors, set) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "curandGetDirectionVectors64"
        ),
    }
}
#[cfg(feature = "runtime-link")]
#[inline(always)]
pub unsafe extern "C" fn curandGetScrambleConstants64(
    constants: *mut *mut ::std::os::raw::c_ulonglong,
) -> curandStatus_t {
    match DYNAMIC_BINDINGS
        .get()
        .expect("CUDA library not loaded. Did you forget to call #[cuda_load]?")
        .curandGetScrambleConstants64
    {
        Some(____func) => unsafe { ____func(constants) },
        None => panic!(
            "CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.",
            "curandGetScrambleConstants64"
        ),
    }
}
#[cfg(feature = "runtime-link")]
pub unsafe fn load_dynamic_bindings(
    lib: *mut std::ffi::c_void,
    get_proc_addr: unsafe fn(*mut std::ffi::c_void, *const u8) -> *mut std::ffi::c_void,
) {
    let bindings = unsafe {
        Box::new(DynamicBindings {
            curandCreateGenerator: {
                let p = get_proc_addr(lib, b"curandCreateGenerator\0".as_ptr());
                if p.is_null() {
                    None
                } else {
                    Some(std::mem::transmute(p))
                }
            },
            curandCreateGeneratorHost: {
                let p = get_proc_addr(lib, b"curandCreateGeneratorHost\0".as_ptr());
                if p.is_null() {
                    None
                } else {
                    Some(std::mem::transmute(p))
                }
            },
            curandDestroyGenerator: {
                let p = get_proc_addr(lib, b"curandDestroyGenerator\0".as_ptr());
                if p.is_null() {
                    None
                } else {
                    Some(std::mem::transmute(p))
                }
            },
            curandGetVersion: {
                let p = get_proc_addr(lib, b"curandGetVersion\0".as_ptr());
                if p.is_null() {
                    None
                } else {
                    Some(std::mem::transmute(p))
                }
            },
            curandGetProperty: {
                let p = get_proc_addr(lib, b"curandGetProperty\0".as_ptr());
                if p.is_null() {
                    None
                } else {
                    Some(std::mem::transmute(p))
                }
            },
            curandSetStream: {
                let p = get_proc_addr(lib, b"curandSetStream\0".as_ptr());
                if p.is_null() {
                    None
                } else {
                    Some(std::mem::transmute(p))
                }
            },
            curandSetPseudoRandomGeneratorSeed: {
                let p = get_proc_addr(lib, b"curandSetPseudoRandomGeneratorSeed\0".as_ptr());
                if p.is_null() {
                    None
                } else {
                    Some(std::mem::transmute(p))
                }
            },
            curandSetGeneratorOffset: {
                let p = get_proc_addr(lib, b"curandSetGeneratorOffset\0".as_ptr());
                if p.is_null() {
                    None
                } else {
                    Some(std::mem::transmute(p))
                }
            },
            curandSetGeneratorOrdering: {
                let p = get_proc_addr(lib, b"curandSetGeneratorOrdering\0".as_ptr());
                if p.is_null() {
                    None
                } else {
                    Some(std::mem::transmute(p))
                }
            },
            curandSetQuasiRandomGeneratorDimensions: {
                let p = get_proc_addr(lib, b"curandSetQuasiRandomGeneratorDimensions\0".as_ptr());
                if p.is_null() {
                    None
                } else {
                    Some(std::mem::transmute(p))
                }
            },
            curandGenerate: {
                let p = get_proc_addr(lib, b"curandGenerate\0".as_ptr());
                if p.is_null() {
                    None
                } else {
                    Some(std::mem::transmute(p))
                }
            },
            curandGenerateLongLong: {
                let p = get_proc_addr(lib, b"curandGenerateLongLong\0".as_ptr());
                if p.is_null() {
                    None
                } else {
                    Some(std::mem::transmute(p))
                }
            },
            curandGenerateUniform: {
                let p = get_proc_addr(lib, b"curandGenerateUniform\0".as_ptr());
                if p.is_null() {
                    None
                } else {
                    Some(std::mem::transmute(p))
                }
            },
            curandGenerateUniformDouble: {
                let p = get_proc_addr(lib, b"curandGenerateUniformDouble\0".as_ptr());
                if p.is_null() {
                    None
                } else {
                    Some(std::mem::transmute(p))
                }
            },
            curandGenerateNormal: {
                let p = get_proc_addr(lib, b"curandGenerateNormal\0".as_ptr());
                if p.is_null() {
                    None
                } else {
                    Some(std::mem::transmute(p))
                }
            },
            curandGenerateNormalDouble: {
                let p = get_proc_addr(lib, b"curandGenerateNormalDouble\0".as_ptr());
                if p.is_null() {
                    None
                } else {
                    Some(std::mem::transmute(p))
                }
            },
            curandGenerateLogNormal: {
                let p = get_proc_addr(lib, b"curandGenerateLogNormal\0".as_ptr());
                if p.is_null() {
                    None
                } else {
                    Some(std::mem::transmute(p))
                }
            },
            curandGenerateLogNormalDouble: {
                let p = get_proc_addr(lib, b"curandGenerateLogNormalDouble\0".as_ptr());
                if p.is_null() {
                    None
                } else {
                    Some(std::mem::transmute(p))
                }
            },
            curandCreatePoissonDistribution: {
                let p = get_proc_addr(lib, b"curandCreatePoissonDistribution\0".as_ptr());
                if p.is_null() {
                    None
                } else {
                    Some(std::mem::transmute(p))
                }
            },
            curandDestroyDistribution: {
                let p = get_proc_addr(lib, b"curandDestroyDistribution\0".as_ptr());
                if p.is_null() {
                    None
                } else {
                    Some(std::mem::transmute(p))
                }
            },
            curandGeneratePoisson: {
                let p = get_proc_addr(lib, b"curandGeneratePoisson\0".as_ptr());
                if p.is_null() {
                    None
                } else {
                    Some(std::mem::transmute(p))
                }
            },
            curandGeneratePoissonMethod: {
                let p = get_proc_addr(lib, b"curandGeneratePoissonMethod\0".as_ptr());
                if p.is_null() {
                    None
                } else {
                    Some(std::mem::transmute(p))
                }
            },
            curandGenerateBinomial: {
                let p = get_proc_addr(lib, b"curandGenerateBinomial\0".as_ptr());
                if p.is_null() {
                    None
                } else {
                    Some(std::mem::transmute(p))
                }
            },
            curandGenerateBinomialMethod: {
                let p = get_proc_addr(lib, b"curandGenerateBinomialMethod\0".as_ptr());
                if p.is_null() {
                    None
                } else {
                    Some(std::mem::transmute(p))
                }
            },
            curandGenerateSeeds: {
                let p = get_proc_addr(lib, b"curandGenerateSeeds\0".as_ptr());
                if p.is_null() {
                    None
                } else {
                    Some(std::mem::transmute(p))
                }
            },
            curandGetDirectionVectors32: {
                let p = get_proc_addr(lib, b"curandGetDirectionVectors32\0".as_ptr());
                if p.is_null() {
                    None
                } else {
                    Some(std::mem::transmute(p))
                }
            },
            curandGetScrambleConstants32: {
                let p = get_proc_addr(lib, b"curandGetScrambleConstants32\0".as_ptr());
                if p.is_null() {
                    None
                } else {
                    Some(std::mem::transmute(p))
                }
            },
            curandGetDirectionVectors64: {
                let p = get_proc_addr(lib, b"curandGetDirectionVectors64\0".as_ptr());
                if p.is_null() {
                    None
                } else {
                    Some(std::mem::transmute(p))
                }
            },
            curandGetScrambleConstants64: {
                let p = get_proc_addr(lib, b"curandGetScrambleConstants64\0".as_ptr());
                if p.is_null() {
                    None
                } else {
                    Some(std::mem::transmute(p))
                }
            },
        })
    };
    DYNAMIC_BINDINGS.set(bindings).ok();
}
