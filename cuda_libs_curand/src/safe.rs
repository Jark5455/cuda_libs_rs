pub use crate::sys::curandStatus_t as CudaTargetStatus;
#[allow(unused_imports)]
use crate::sys::*;
use cuda_libs_cudart;
#[allow(unused_imports)]
use cuda_libs_cudart::sys::*;
#[allow(unused_imports)]
use cuda_libs_cudart::types;
#[cfg(feature = "runtime-link")]
impl crate::sys::DynamicBindings {
    pub fn curandCreateGenerator(mut self, val: Option<unsafe extern "C" fn(generator: *mut curandGenerator_t, rng_type: curandRngType_t) -> curandStatus_t>) -> Self {
        self.curandCreateGenerator = val;
        self
    }
    pub fn curandCreateGeneratorHost(mut self, val: Option<unsafe extern "C" fn(generator: *mut curandGenerator_t, rng_type: curandRngType_t) -> curandStatus_t>) -> Self {
        self.curandCreateGeneratorHost = val;
        self
    }
    pub fn curandDestroyGenerator(mut self, val: Option<unsafe extern "C" fn(generator: curandGenerator_t) -> curandStatus_t>) -> Self {
        self.curandDestroyGenerator = val;
        self
    }
    pub fn curandGetVersion(mut self, val: Option<unsafe extern "C" fn(version: *mut ::std::os::raw::c_int) -> curandStatus_t>) -> Self {
        self.curandGetVersion = val;
        self
    }
    pub fn curandGetProperty(mut self, val: Option<unsafe extern "C" fn(type_: libraryPropertyType, value: *mut ::std::os::raw::c_int) -> curandStatus_t>) -> Self {
        self.curandGetProperty = val;
        self
    }
    pub fn curandSetStream(mut self, val: Option<unsafe extern "C" fn(generator: curandGenerator_t, stream: cudaStream_t) -> curandStatus_t>) -> Self {
        self.curandSetStream = val;
        self
    }
    pub fn curandSetPseudoRandomGeneratorSeed(mut self, val: Option<unsafe extern "C" fn(generator: curandGenerator_t, seed: ::std::os::raw::c_ulonglong) -> curandStatus_t>) -> Self {
        self.curandSetPseudoRandomGeneratorSeed = val;
        self
    }
    pub fn curandSetGeneratorOffset(mut self, val: Option<unsafe extern "C" fn(generator: curandGenerator_t, offset: ::std::os::raw::c_ulonglong) -> curandStatus_t>) -> Self {
        self.curandSetGeneratorOffset = val;
        self
    }
    pub fn curandSetGeneratorOrdering(mut self, val: Option<unsafe extern "C" fn(generator: curandGenerator_t, order: curandOrdering_t) -> curandStatus_t>) -> Self {
        self.curandSetGeneratorOrdering = val;
        self
    }
    pub fn curandSetQuasiRandomGeneratorDimensions(mut self, val: Option<unsafe extern "C" fn(generator: curandGenerator_t, num_dimensions: ::std::os::raw::c_uint) -> curandStatus_t>) -> Self {
        self.curandSetQuasiRandomGeneratorDimensions = val;
        self
    }
    pub fn curandGenerate(mut self, val: Option<unsafe extern "C" fn(generator: curandGenerator_t, outputPtr: *mut ::std::os::raw::c_uint, num: usize) -> curandStatus_t>) -> Self {
        self.curandGenerate = val;
        self
    }
    pub fn curandGenerateLongLong(mut self, val: Option<unsafe extern "C" fn(generator: curandGenerator_t, outputPtr: *mut ::std::os::raw::c_ulonglong, num: usize) -> curandStatus_t>) -> Self {
        self.curandGenerateLongLong = val;
        self
    }
    pub fn curandGenerateUniform(mut self, val: Option<unsafe extern "C" fn(generator: curandGenerator_t, outputPtr: *mut f32, num: usize) -> curandStatus_t>) -> Self {
        self.curandGenerateUniform = val;
        self
    }
    pub fn curandGenerateUniformDouble(mut self, val: Option<unsafe extern "C" fn(generator: curandGenerator_t, outputPtr: *mut f64, num: usize) -> curandStatus_t>) -> Self {
        self.curandGenerateUniformDouble = val;
        self
    }
    pub fn curandGenerateNormal(mut self, val: Option<unsafe extern "C" fn(generator: curandGenerator_t, outputPtr: *mut f32, n: usize, mean: f32, stddev: f32) -> curandStatus_t>) -> Self {
        self.curandGenerateNormal = val;
        self
    }
    pub fn curandGenerateNormalDouble(mut self, val: Option<unsafe extern "C" fn(generator: curandGenerator_t, outputPtr: *mut f64, n: usize, mean: f64, stddev: f64) -> curandStatus_t>) -> Self {
        self.curandGenerateNormalDouble = val;
        self
    }
    pub fn curandGenerateLogNormal(mut self, val: Option<unsafe extern "C" fn(generator: curandGenerator_t, outputPtr: *mut f32, n: usize, mean: f32, stddev: f32) -> curandStatus_t>) -> Self {
        self.curandGenerateLogNormal = val;
        self
    }
    pub fn curandGenerateLogNormalDouble(mut self, val: Option<unsafe extern "C" fn(generator: curandGenerator_t, outputPtr: *mut f64, n: usize, mean: f64, stddev: f64) -> curandStatus_t>) -> Self {
        self.curandGenerateLogNormalDouble = val;
        self
    }
    pub fn curandCreatePoissonDistribution(mut self, val: Option<unsafe extern "C" fn(lambda: f64, discrete_distribution: *mut curandDiscreteDistribution_t) -> curandStatus_t>) -> Self {
        self.curandCreatePoissonDistribution = val;
        self
    }
    pub fn curandDestroyDistribution(mut self, val: Option<unsafe extern "C" fn(discrete_distribution: curandDiscreteDistribution_t) -> curandStatus_t>) -> Self {
        self.curandDestroyDistribution = val;
        self
    }
    pub fn curandGeneratePoisson(mut self, val: Option<unsafe extern "C" fn(generator: curandGenerator_t, outputPtr: *mut ::std::os::raw::c_uint, n: usize, lambda: f64) -> curandStatus_t>) -> Self {
        self.curandGeneratePoisson = val;
        self
    }
    pub fn curandGeneratePoissonMethod(mut self, val: Option<unsafe extern "C" fn(generator: curandGenerator_t, outputPtr: *mut ::std::os::raw::c_uint, n: usize, lambda: f64, method: curandMethod_t) -> curandStatus_t>) -> Self {
        self.curandGeneratePoissonMethod = val;
        self
    }
    pub fn curandGenerateBinomial(mut self, val: Option<unsafe extern "C" fn(generator: curandGenerator_t, outputPtr: *mut ::std::os::raw::c_uint, num: usize, n: ::std::os::raw::c_uint, p: f64) -> curandStatus_t>) -> Self {
        self.curandGenerateBinomial = val;
        self
    }
    pub fn curandGenerateBinomialMethod(mut self, val: Option<unsafe extern "C" fn(generator: curandGenerator_t, outputPtr: *mut ::std::os::raw::c_uint, num: usize, n: ::std::os::raw::c_uint, p: f64, method: curandMethod_t) -> curandStatus_t>) -> Self {
        self.curandGenerateBinomialMethod = val;
        self
    }
    pub fn curandGenerateSeeds(mut self, val: Option<unsafe extern "C" fn(generator: curandGenerator_t) -> curandStatus_t>) -> Self {
        self.curandGenerateSeeds = val;
        self
    }
    pub fn curandGetDirectionVectors32(mut self, val: Option<unsafe extern "C" fn(vectors: *mut *mut curandDirectionVectors32_t, set: curandDirectionVectorSet_t) -> curandStatus_t>) -> Self {
        self.curandGetDirectionVectors32 = val;
        self
    }
    pub fn curandGetScrambleConstants32(mut self, val: Option<unsafe extern "C" fn(constants: *mut *mut ::std::os::raw::c_uint) -> curandStatus_t>) -> Self {
        self.curandGetScrambleConstants32 = val;
        self
    }
    pub fn curandGetDirectionVectors64(mut self, val: Option<unsafe extern "C" fn(vectors: *mut *mut curandDirectionVectors64_t, set: curandDirectionVectorSet_t) -> curandStatus_t>) -> Self {
        self.curandGetDirectionVectors64 = val;
        self
    }
    pub fn curandGetScrambleConstants64(mut self, val: Option<unsafe extern "C" fn(constants: *mut *mut ::std::os::raw::c_ulonglong) -> curandStatus_t>) -> Self {
        self.curandGetScrambleConstants64 = val;
        self
    }
}
#[doc = "Create new random number generator.\nCreates a new random number generator of type `rng_type`\nand returns it in `*generator.`\nLegal values for `rng_type` are:\n- CURAND_RNG_PSEUDO_DEFAULT\n- CURAND_RNG_PSEUDO_XORWOW\n- CURAND_RNG_PSEUDO_MRG32K3A\n- CURAND_RNG_PSEUDO_MTGP32\n- CURAND_RNG_PSEUDO_MT19937\n- CURAND_RNG_PSEUDO_PHILOX4_32_10\n- CURAND_RNG_QUASI_DEFAULT\n- CURAND_RNG_QUASI_SOBOL32\n- CURAND_RNG_QUASI_SCRAMBLED_SOBOL32\n- CURAND_RNG_QUASI_SOBOL64\n- CURAND_RNG_QUASI_SCRAMBLED_SOBOL64\nWhen `rng_type` is CURAND_RNG_PSEUDO_DEFAULT, the type chosen\nis CURAND_RNG_PSEUDO_XORWOW.  \\n When `rng_type` is CURAND_RNG_QUASI_DEFAULT,\nthe type chosen is CURAND_RNG_QUASI_SOBOL32.\nThe default values for `rng_type` = CURAND_RNG_PSEUDO_XORWOW are:\n- `seed` = 0\n- `offset` = 0\n- `ordering` = CURAND_ORDERING_PSEUDO_DEFAULT\nThe default values for `rng_type` = CURAND_RNG_PSEUDO_MRG32K3A are:\n- `seed` = 0\n- `offset` = 0\n- `ordering` = CURAND_ORDERING_PSEUDO_DEFAULT\nThe default values for `rng_type` = CURAND_RNG_PSEUDO_MTGP32 are:\n- `seed` = 0\n- `offset` = 0\n- `ordering` = CURAND_ORDERING_PSEUDO_DEFAULT\nThe default values for `rng_type` = CURAND_RNG_PSEUDO_MT19937 are:\n- `seed` = 0\n- `offset` = 0\n- `ordering` = CURAND_ORDERING_PSEUDO_DEFAULT\n* The default values for `rng_type` = CURAND_RNG_PSEUDO_PHILOX4_32_10 are:\n- `seed` = 0\n- `offset` = 0\n- `ordering` = CURAND_ORDERING_PSEUDO_DEFAULT\nThe default values for `rng_type` = CURAND_RNG_QUASI_SOBOL32 are:\n- `dimensions` = 1\n- `offset` = 0\n- `ordering` = CURAND_ORDERING_QUASI_DEFAULT\nThe default values for `rng_type` = CURAND_RNG_QUASI_SOBOL64 are:\n- `dimensions` = 1\n- `offset` = 0\n- `ordering` = CURAND_ORDERING_QUASI_DEFAULT\nThe default values for `rng_type` = CURAND_RNG_QUASI_SCRAMBBLED_SOBOL32 are:\n- `dimensions` = 1\n- `offset` = 0\n- `ordering` = CURAND_ORDERING_QUASI_DEFAULT\nThe default values for `rng_type` = CURAND_RNG_QUASI_SCRAMBLED_SOBOL64 are:\n- `dimensions` = 1\n- `offset` = 0\n- `ordering` = CURAND_ORDERING_QUASI_DEFAULT\n\n# Arguments\n\n* `generator` - - Pointer to generator\n* `rng_type` - - Type of generator to create\n\n# Returns\n\n- CURAND_STATUS_ALLOCATION_FAILED, if memory could not be allocated \\n - CURAND_STATUS_INITIALIZATION_FAILED if there was a problem setting up the GPU \\n - CURAND_STATUS_VERSION_MISMATCH if the header file version does not match the\ndynamically linked library version \\n - CURAND_STATUS_TYPE_ERROR if the value for `rng_type` is invalid \\n - CURAND_STATUS_SUCCESS if generator was created successfully \\n "]
pub unsafe fn curandCreateGenerator(rng_type: curandRngType_t) -> Result<curandGenerator_t, crate::sys::curandStatus_t> {
    let mut out_0: std::mem::MaybeUninit<curandGenerator_t> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::curandCreateGenerator(out_0.as_mut_ptr() as *mut _, rng_type) };
    if status as usize == crate::sys::curandStatus_t::CURAND_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as curandGenerator_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Create new host CPU random number generator.\nCreates a new host CPU random number generator of type `rng_type`\nand returns it in `*generator.`\nLegal values for `rng_type` are:\n- CURAND_RNG_PSEUDO_DEFAULT\n- CURAND_RNG_PSEUDO_XORWOW\n- CURAND_RNG_PSEUDO_MRG32K3A\n- CURAND_RNG_PSEUDO_MTGP32\n- CURAND_RNG_PSEUDO_MT19937\n- CURAND_RNG_PSEUDO_PHILOX4_32_10\n- CURAND_RNG_QUASI_DEFAULT\n- CURAND_RNG_QUASI_SOBOL32\n- CURAND_RNG_QUASI_SCRAMBLED_SOBOL32\n- CURAND_RNG_QUASI_SOBOL64\n- CURAND_RNG_QUASI_SCRAMBLED_SOBOL64\nWhen `rng_type` is CURAND_RNG_PSEUDO_DEFAULT, the type chosen\nis CURAND_RNG_PSEUDO_XORWOW.  \\n When `rng_type` is CURAND_RNG_QUASI_DEFAULT,\nthe type chosen is CURAND_RNG_QUASI_SOBOL32.\nThe default values for `rng_type` = CURAND_RNG_PSEUDO_XORWOW are:\n- `seed` = 0\n- `offset` = 0\n- `ordering` = CURAND_ORDERING_PSEUDO_DEFAULT\nThe default values for `rng_type` = CURAND_RNG_PSEUDO_MRG32K3A are:\n- `seed` = 0\n- `offset` = 0\n- `ordering` = CURAND_ORDERING_PSEUDO_DEFAULT\nThe default values for `rng_type` = CURAND_RNG_PSEUDO_MTGP32 are:\n- `seed` = 0\n- `offset` = 0\n- `ordering` = CURAND_ORDERING_PSEUDO_DEFAULT\nThe default values for `rng_type` = CURAND_RNG_PSEUDO_MT19937 are:\n- `seed` = 0\n- `offset` = 0\n- `ordering` = CURAND_ORDERING_PSEUDO_DEFAULT\n* The default values for `rng_type` = CURAND_RNG_PSEUDO_PHILOX4_32_10 are:\n- `seed` = 0\n- `offset` = 0\n- `ordering` = CURAND_ORDERING_PSEUDO_DEFAULT\nThe default values for `rng_type` = CURAND_RNG_QUASI_SOBOL32 are:\n- `dimensions` = 1\n- `offset` = 0\n- `ordering` = CURAND_ORDERING_QUASI_DEFAULT\nThe default values for `rng_type` = CURAND_RNG_QUASI_SOBOL64 are:\n- `dimensions` = 1\n- `offset` = 0\n- `ordering` = CURAND_ORDERING_QUASI_DEFAULT\nThe default values for `rng_type` = CURAND_RNG_QUASI_SCRAMBLED_SOBOL32 are:\n- `dimensions` = 1\n- `offset` = 0\n- `ordering` = CURAND_ORDERING_QUASI_DEFAULT\nThe default values for `rng_type` = CURAND_RNG_QUASI_SCRAMBLED_SOBOL64 are:\n- `dimensions` = 1\n- `offset` = 0\n- `ordering` = CURAND_ORDERING_QUASI_DEFAULT\n\n# Arguments\n\n* `generator` - - Pointer to generator\n* `rng_type` - - Type of generator to create\n\n# Returns\n\n- CURAND_STATUS_ALLOCATION_FAILED if memory could not be allocated \\n - CURAND_STATUS_INITIALIZATION_FAILED if there was a problem setting up the GPU \\n - CURAND_STATUS_VERSION_MISMATCH if the header file version does not match the\ndynamically linked library version \\n - CURAND_STATUS_TYPE_ERROR if the value for `rng_type` is invalid \\n - CURAND_STATUS_SUCCESS if generator was created successfully \\n "]
pub unsafe fn curandCreateGeneratorHost(rng_type: curandRngType_t) -> Result<curandGenerator_t, crate::sys::curandStatus_t> {
    let mut out_0: std::mem::MaybeUninit<curandGenerator_t> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::curandCreateGeneratorHost(out_0.as_mut_ptr() as *mut _, rng_type) };
    if status as usize == crate::sys::curandStatus_t::CURAND_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as curandGenerator_t) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Destroy an existing generator.\nDestroy an existing generator and free all memory associated with its state.\n\n# Arguments\n\n* `generator` - - Generator to destroy\n\n# Returns\n\n- CURAND_STATUS_NOT_INITIALIZED if the generator was never created \\n - CURAND_STATUS_SUCCESS if generator was destroyed successfully \\n "]
pub unsafe fn curandDestroyGenerator(generator: curandGenerator_t) -> Result<(), crate::sys::curandStatus_t> {
    let status = unsafe { crate::sys::curandDestroyGenerator(generator) };
    if status == crate::sys::curandStatus_t::CURAND_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Return the version number of the library.\nReturn in `*version` the version number of the dynamically linked CURAND\nlibrary.  The format is the same as CUDART_VERSION from the CUDA Runtime.\nThe only supported configuration is CURAND version equal to CUDA Runtime\nversion.\n\n# Arguments\n\n* `version` - - CURAND library version\n\n# Returns\n\n- CURAND_STATUS_SUCCESS if the version number was successfully returned \\n "]
pub unsafe fn curandGetVersion() -> Result<i32, crate::sys::curandStatus_t> {
    let mut out_0: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::curandGetVersion(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::curandStatus_t::CURAND_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as i32) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Return the value of the curand property.\nReturn in `*value` the number for the property described by `type` of the\ndynamically linked CURAND library.\n\n# Arguments\n\n* `type` - - CUDA library property\n* `value` - - integer value for the requested property\n\n# Returns\n\n- CURAND_STATUS_SUCCESS if the property value was successfully returned \\n - CURAND_STATUS_OUT_OF_RANGE if the property type is not recognized \\n "]
pub unsafe fn curandGetProperty(type_: libraryPropertyType) -> Result<i32, crate::sys::curandStatus_t> {
    let mut out_1: std::mem::MaybeUninit<::std::os::raw::c_int> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::curandGetProperty(type_, out_1.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::curandStatus_t::CURAND_STATUS_SUCCESS as usize {
        unsafe { Ok(out_1.assume_init() as i32) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Set the current stream for CURAND kernel launches.\nSet the current stream for CURAND kernel launches.  All library functions\nwill use this stream until set again.\n\n# Arguments\n\n* `generator` - - Generator to modify\n* `stream` - - Stream to use or ::NULL for null stream\n\n# Returns\n\n- CURAND_STATUS_NOT_INITIALIZED if the generator was never created \\n - CURAND_STATUS_SUCCESS if stream was set successfully \\n "]
pub unsafe fn curandSetStream(generator: curandGenerator_t, stream: cudaStream_t) -> Result<(), crate::sys::curandStatus_t> {
    let status = unsafe { crate::sys::curandSetStream(generator, stream) };
    if status == crate::sys::curandStatus_t::CURAND_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Set the seed value of the pseudo-random number generator.\nSet the seed value of the pseudorandom number generator.\nAll values of seed are valid.  Different seeds will produce different sequences.\nDifferent seeds will often not be statistically correlated with each other,\nbut some pairs of seed values may generate sequences which are statistically correlated.\n\n# Arguments\n\n* `generator` - - Generator to modify\n* `seed` - - Seed value\n\n# Returns\n\n- CURAND_STATUS_NOT_INITIALIZED if the generator was never created \\n - CURAND_STATUS_TYPE_ERROR if the generator is not a pseudorandom number generator \\n - CURAND_STATUS_SUCCESS if generator seed was set successfully \\n "]
pub unsafe fn curandSetPseudoRandomGeneratorSeed(generator: curandGenerator_t, seed: u64) -> Result<(), crate::sys::curandStatus_t> {
    let status = unsafe { crate::sys::curandSetPseudoRandomGeneratorSeed(generator, seed as _) };
    if status == crate::sys::curandStatus_t::CURAND_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Set the absolute offset of the pseudo or quasirandom number generator.\nSet the absolute offset of the pseudo or quasirandom number generator.\nAll values of offset are valid.  The offset position is absolute, not\nrelative to the current position in the sequence.\n\n# Arguments\n\n* `generator` - - Generator to modify\n* `offset` - - Absolute offset position\n\n# Returns\n\n- CURAND_STATUS_NOT_INITIALIZED if the generator was never created \\n - CURAND_STATUS_SUCCESS if generator offset was set successfully \\n "]
pub unsafe fn curandSetGeneratorOffset(generator: curandGenerator_t, offset: u64) -> Result<(), crate::sys::curandStatus_t> {
    let status = unsafe { crate::sys::curandSetGeneratorOffset(generator, offset as _) };
    if status == crate::sys::curandStatus_t::CURAND_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Set the ordering of results of the pseudo or quasirandom number generator.\nSet the ordering of results of the pseudo or quasirandom number generator.\nLegal values of `order` for pseudorandom generators are:\n- CURAND_ORDERING_PSEUDO_DEFAULT\n- CURAND_ORDERING_PSEUDO_BEST\n- CURAND_ORDERING_PSEUDO_SEEDED\n- CURAND_ORDERING_PSEUDO_LEGACY\nLegal values of `order` for quasirandom generators are:\n- CURAND_ORDERING_QUASI_DEFAULT\n\n# Arguments\n\n* `generator` - - Generator to modify\n* `order` - - Ordering of results\n\n# Returns\n\n- CURAND_STATUS_NOT_INITIALIZED if the generator was never created \\n - CURAND_STATUS_OUT_OF_RANGE if the ordering is not valid \\n - CURAND_STATUS_SUCCESS if generator ordering was set successfully \\n "]
pub unsafe fn curandSetGeneratorOrdering(generator: curandGenerator_t, order: curandOrdering_t) -> Result<(), crate::sys::curandStatus_t> {
    let status = unsafe { crate::sys::curandSetGeneratorOrdering(generator, order) };
    if status == crate::sys::curandStatus_t::CURAND_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Set the number of dimensions.\nSet the number of dimensions to be generated by the quasirandom number\ngenerator.\nLegal values for `num_dimensions` are 1 to 20000.\n\n# Arguments\n\n* `generator` - - Generator to modify\n* `num_dimensions` - - Number of dimensions\n\n# Returns\n\n- CURAND_STATUS_NOT_INITIALIZED if the generator was never created \\n - CURAND_STATUS_OUT_OF_RANGE if num_dimensions is not valid \\n - CURAND_STATUS_TYPE_ERROR if the generator is not a quasirandom number generator \\n - CURAND_STATUS_SUCCESS if generator ordering was set successfully \\n "]
pub unsafe fn curandSetQuasiRandomGeneratorDimensions(generator: curandGenerator_t, num_dimensions: u32) -> Result<(), crate::sys::curandStatus_t> {
    let status = unsafe { crate::sys::curandSetQuasiRandomGeneratorDimensions(generator, num_dimensions as _) };
    if status == crate::sys::curandStatus_t::CURAND_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Generate 32-bit pseudo or quasirandom numbers.\nUse `generator` to generate `num` 32-bit results into the device memory at\n`outputPtr.`  The device memory must have been previously allocated and be\nlarge enough to hold all the results.  Launches are done with the stream\nset using ::curandSetStream(), or the null stream if no stream has been set.\nResults are 32-bit values with every bit random.\n\n# Arguments\n\n* `generator` - - Generator to use\n* `outputPtr` - - Pointer to device memory to store CUDA-generated results, or\nPointer to host memory to store CPU-generated results\n* `num` - - Number of random 32-bit values to generate\n\n# Returns\n\n- CURAND_STATUS_ALLOCATION_FAILED if memory could not be allocated \\n - CURAND_STATUS_NOT_INITIALIZED if the generator was never created \\n - CURAND_STATUS_PREEXISTING_FAILURE if there was an existing error from\na previous kernel launch \\n - CURAND_STATUS_LENGTH_NOT_MULTIPLE if the number of output samples is\nnot a multiple of the quasirandom dimension \\n - CURAND_STATUS_LAUNCH_FAILURE if the kernel launch failed for any reason \\n - CURAND_STATUS_TYPE_ERROR if the generator is a 64 bit quasirandom generator.\n(use ::curandGenerateLongLong() with 64 bit quasirandom generators)\n- CURAND_STATUS_SUCCESS if the results were generated successfully \\n "]
pub unsafe fn curandGenerate<T: types::CudaAsPtr>(generator: curandGenerator_t, mut outputPtr: T, num: usize) -> Result<(), crate::sys::curandStatus_t> {
    let status = unsafe { crate::sys::curandGenerate(generator, outputPtr.as_mut_ptr() as *mut _, num) };
    if status == crate::sys::curandStatus_t::CURAND_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Generate 64-bit quasirandom numbers.\nUse `generator` to generate `num` 64-bit results into the device memory at\n`outputPtr.`  The device memory must have been previously allocated and be\nlarge enough to hold all the results.  Launches are done with the stream\nset using ::curandSetStream(), or the null stream if no stream has been set.\nResults are 64-bit values with every bit random.\n\n# Arguments\n\n* `generator` - - Generator to use\n* `outputPtr` - - Pointer to device memory to store CUDA-generated results, or\nPointer to host memory to store CPU-generated results\n* `num` - - Number of random 64-bit values to generate\n\n# Returns\n\n- CURAND_STATUS_NOT_INITIALIZED if the generator was never created \\n - CURAND_STATUS_PREEXISTING_FAILURE if there was an existing error from\na previous kernel launch \\n - CURAND_STATUS_LENGTH_NOT_MULTIPLE if the number of output samples is\nnot a multiple of the quasirandom dimension \\n - CURAND_STATUS_LAUNCH_FAILURE if the kernel launch failed for any reason \\n - CURAND_STATUS_TYPE_ERROR if the generator is not a 64 bit quasirandom generator\\n - CURAND_STATUS_SUCCESS if the results were generated successfully \\n "]
pub unsafe fn curandGenerateLongLong<T: types::CudaAsPtr>(generator: curandGenerator_t, mut outputPtr: T, num: usize) -> Result<(), crate::sys::curandStatus_t> {
    let status = unsafe { crate::sys::curandGenerateLongLong(generator, outputPtr.as_mut_ptr() as *mut _, num) };
    if status == crate::sys::curandStatus_t::CURAND_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Generate uniformly distributed floats.\nUse `generator` to generate `num` float results into the device memory at\n`outputPtr.`  The device memory must have been previously allocated and be\nlarge enough to hold all the results.  Launches are done with the stream\nset using ::curandSetStream(), or the null stream if no stream has been set.\nResults are 32-bit floating point values between `0.0f` and `1.0f,`\nexcluding `0.0f` and including `1.0f.`\n\n# Arguments\n\n* `generator` - - Generator to use\n* `outputPtr` - - Pointer to device memory to store CUDA-generated results, or\nPointer to host memory to store CPU-generated results\n* `num` - - Number of floats to generate\n\n# Returns\n\n- CURAND_STATUS_ALLOCATION_FAILED if memory could not be allocated \\n - CURAND_STATUS_NOT_INITIALIZED if the generator was never created \\n - CURAND_STATUS_PREEXISTING_FAILURE if there was an existing error from\na previous kernel launch \\n - CURAND_STATUS_LAUNCH_FAILURE if the kernel launch failed for any reason \\n - CURAND_STATUS_LENGTH_NOT_MULTIPLE if the number of output samples is\nnot a multiple of the quasirandom dimension \\n - CURAND_STATUS_SUCCESS if the results were generated successfully \\n "]
pub unsafe fn curandGenerateUniform<T: types::CudaAsPtr>(generator: curandGenerator_t, mut outputPtr: T, num: usize) -> Result<(), crate::sys::curandStatus_t> {
    let status = unsafe { crate::sys::curandGenerateUniform(generator, outputPtr.as_mut_ptr() as *mut _, num) };
    if status == crate::sys::curandStatus_t::CURAND_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Generate uniformly distributed doubles.\nUse `generator` to generate `num` double results into the device memory at\n`outputPtr.`  The device memory must have been previously allocated and be\nlarge enough to hold all the results.  Launches are done with the stream\nset using ::curandSetStream(), or the null stream if no stream has been set.\nResults are 64-bit double precision floating point values between\n`0.0` and `1.0,` excluding `0.0` and including `1.0.`\n\n# Arguments\n\n* `generator` - - Generator to use\n* `outputPtr` - - Pointer to device memory to store CUDA-generated results, or\nPointer to host memory to store CPU-generated results\n* `num` - - Number of doubles to generate\n\n# Returns\n\n- CURAND_STATUS_ALLOCATION_FAILED if memory could not be allocated \\n - CURAND_STATUS_NOT_INITIALIZED if the generator was never created \\n - CURAND_STATUS_PREEXISTING_FAILURE if there was an existing error from\na previous kernel launch \\n - CURAND_STATUS_LAUNCH_FAILURE if the kernel launch failed for any reason \\n - CURAND_STATUS_LENGTH_NOT_MULTIPLE if the number of output samples is\nnot a multiple of the quasirandom dimension \\n - CURAND_STATUS_DOUBLE_PRECISION_REQUIRED if the GPU does not support double precision \\n - CURAND_STATUS_SUCCESS if the results were generated successfully \\n "]
pub unsafe fn curandGenerateUniformDouble<T: types::CudaAsPtr>(generator: curandGenerator_t, mut outputPtr: T, num: usize) -> Result<(), crate::sys::curandStatus_t> {
    let status = unsafe { crate::sys::curandGenerateUniformDouble(generator, outputPtr.as_mut_ptr() as *mut _, num) };
    if status == crate::sys::curandStatus_t::CURAND_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Generate normally distributed doubles.\nUse `generator` to generate `n` float results into the device memory at\n`outputPtr.`  The device memory must have been previously allocated and be\nlarge enough to hold all the results.  Launches are done with the stream\nset using ::curandSetStream(), or the null stream if no stream has been set.\nResults are 32-bit floating point values with mean `mean` and standard\ndeviation `stddev.`\nNormally distributed results are generated from pseudorandom generators\nwith a Box-Muller transform, and so require `n` to be even.\nQuasirandom generators use an inverse cumulative distribution\nfunction to preserve dimensionality.\nThere may be slight numerical differences between results generated\non the GPU with generators created with ::curandCreateGenerator()\nand results calculated on the CPU with generators created with\n::curandCreateGeneratorHost().  These differences arise because of\ndifferences in results for transcendental functions.  In addition,\nfuture versions of CURAND may use newer versions of the CUDA math\nlibrary, so different versions of CURAND may give slightly different\nnumerical values.\n\n# Arguments\n\n* `generator` - - Generator to use\n* `outputPtr` - - Pointer to device memory to store CUDA-generated results, or\nPointer to host memory to store CPU-generated results\n* `n` - - Number of floats to generate\n* `mean` - - Mean of normal distribution\n* `stddev` - - Standard deviation of normal distribution\n\n# Returns\n\n- CURAND_STATUS_ALLOCATION_FAILED if memory could not be allocated \\n - CURAND_STATUS_NOT_INITIALIZED if the generator was never created \\n - CURAND_STATUS_PREEXISTING_FAILURE if there was an existing error from\na previous kernel launch \\n - CURAND_STATUS_LAUNCH_FAILURE if the kernel launch failed for any reason \\n - CURAND_STATUS_LENGTH_NOT_MULTIPLE if the number of output samples is\nnot a multiple of the quasirandom dimension, or is not a multiple\nof two for pseudorandom generators \\n - CURAND_STATUS_SUCCESS if the results were generated successfully \\n "]
pub unsafe fn curandGenerateNormal<T: types::CudaAsPtr>(generator: curandGenerator_t, mut outputPtr: T, n: usize, mean: f32, stddev: f32) -> Result<(), crate::sys::curandStatus_t> {
    let status = unsafe { crate::sys::curandGenerateNormal(generator, outputPtr.as_mut_ptr() as *mut _, n, mean, stddev) };
    if status == crate::sys::curandStatus_t::CURAND_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Generate normally distributed doubles.\nUse `generator` to generate `n` double results into the device memory at\n`outputPtr.`  The device memory must have been previously allocated and be\nlarge enough to hold all the results.  Launches are done with the stream\nset using ::curandSetStream(), or the null stream if no stream has been set.\nResults are 64-bit floating point values with mean `mean` and standard\ndeviation `stddev.`\nNormally distributed results are generated from pseudorandom generators\nwith a Box-Muller transform, and so require `n` to be even.\nQuasirandom generators use an inverse cumulative distribution\nfunction to preserve dimensionality.\nThere may be slight numerical differences between results generated\non the GPU with generators created with ::curandCreateGenerator()\nand results calculated on the CPU with generators created with\n::curandCreateGeneratorHost().  These differences arise because of\ndifferences in results for transcendental functions.  In addition,\nfuture versions of CURAND may use newer versions of the CUDA math\nlibrary, so different versions of CURAND may give slightly different\nnumerical values.\n\n# Arguments\n\n* `generator` - - Generator to use\n* `outputPtr` - - Pointer to device memory to store CUDA-generated results, or\nPointer to host memory to store CPU-generated results\n* `n` - - Number of doubles to generate\n* `mean` - - Mean of normal distribution\n* `stddev` - - Standard deviation of normal distribution\n\n# Returns\n\n- CURAND_STATUS_ALLOCATION_FAILED if memory could not be allocated \\n - CURAND_STATUS_NOT_INITIALIZED if the generator was never created \\n - CURAND_STATUS_PREEXISTING_FAILURE if there was an existing error from\na previous kernel launch \\n - CURAND_STATUS_LAUNCH_FAILURE if the kernel launch failed for any reason \\n - CURAND_STATUS_LENGTH_NOT_MULTIPLE if the number of output samples is\nnot a multiple of the quasirandom dimension, or is not a multiple\nof two for pseudorandom generators \\n - CURAND_STATUS_DOUBLE_PRECISION_REQUIRED if the GPU does not support double precision \\n - CURAND_STATUS_SUCCESS if the results were generated successfully \\n "]
pub unsafe fn curandGenerateNormalDouble<T: types::CudaAsPtr>(generator: curandGenerator_t, mut outputPtr: T, n: usize, mean: f64, stddev: f64) -> Result<(), crate::sys::curandStatus_t> {
    let status = unsafe { crate::sys::curandGenerateNormalDouble(generator, outputPtr.as_mut_ptr() as *mut _, n, mean, stddev) };
    if status == crate::sys::curandStatus_t::CURAND_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Generate log-normally distributed floats.\nUse `generator` to generate `n` float results into the device memory at\n`outputPtr.`  The device memory must have been previously allocated and be\nlarge enough to hold all the results.  Launches are done with the stream\nset using ::curandSetStream(), or the null stream if no stream has been set.\nResults are 32-bit floating point values with log-normal distribution based on\nan associated normal distribution with mean `mean` and standard deviation `stddev.`\nNormally distributed results are generated from pseudorandom generators\nwith a Box-Muller transform, and so require `n` to be even.\nQuasirandom generators use an inverse cumulative distribution\nfunction to preserve dimensionality.\nThe normally distributed results are transformed into log-normal distribution.\nThere may be slight numerical differences between results generated\non the GPU with generators created with ::curandCreateGenerator()\nand results calculated on the CPU with generators created with\n::curandCreateGeneratorHost().  These differences arise because of\ndifferences in results for transcendental functions.  In addition,\nfuture versions of CURAND may use newer versions of the CUDA math\nlibrary, so different versions of CURAND may give slightly different\nnumerical values.\n\n# Arguments\n\n* `generator` - - Generator to use\n* `outputPtr` - - Pointer to device memory to store CUDA-generated results, or\nPointer to host memory to store CPU-generated results\n* `n` - - Number of floats to generate\n* `mean` - - Mean of associated normal distribution\n* `stddev` - - Standard deviation of associated normal distribution\n\n# Returns\n\n- CURAND_STATUS_ALLOCATION_FAILED if memory could not be allocated \\n - CURAND_STATUS_NOT_INITIALIZED if the generator was never created \\n - CURAND_STATUS_PREEXISTING_FAILURE if there was an existing error from\na previous kernel launch \\n - CURAND_STATUS_LAUNCH_FAILURE if the kernel launch failed for any reason \\n - CURAND_STATUS_LENGTH_NOT_MULTIPLE if the number of output samples is\nnot a multiple of the quasirandom dimension, or is not a multiple\nof two for pseudorandom generators \\n - CURAND_STATUS_SUCCESS if the results were generated successfully \\n "]
pub unsafe fn curandGenerateLogNormal<T: types::CudaAsPtr>(generator: curandGenerator_t, mut outputPtr: T, n: usize, mean: f32, stddev: f32) -> Result<(), crate::sys::curandStatus_t> {
    let status = unsafe { crate::sys::curandGenerateLogNormal(generator, outputPtr.as_mut_ptr() as *mut _, n, mean, stddev) };
    if status == crate::sys::curandStatus_t::CURAND_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Generate log-normally distributed doubles.\nUse `generator` to generate `n` double results into the device memory at\n`outputPtr.`  The device memory must have been previously allocated and be\nlarge enough to hold all the results.  Launches are done with the stream\nset using ::curandSetStream(), or the null stream if no stream has been set.\nResults are 64-bit floating point values with log-normal distribution based on\nan associated normal distribution with mean `mean` and standard deviation `stddev.`\nNormally distributed results are generated from pseudorandom generators\nwith a Box-Muller transform, and so require `n` to be even.\nQuasirandom generators use an inverse cumulative distribution\nfunction to preserve dimensionality.\nThe normally distributed results are transformed into log-normal distribution.\nThere may be slight numerical differences between results generated\non the GPU with generators created with ::curandCreateGenerator()\nand results calculated on the CPU with generators created with\n::curandCreateGeneratorHost().  These differences arise because of\ndifferences in results for transcendental functions.  In addition,\nfuture versions of CURAND may use newer versions of the CUDA math\nlibrary, so different versions of CURAND may give slightly different\nnumerical values.\n\n# Arguments\n\n* `generator` - - Generator to use\n* `outputPtr` - - Pointer to device memory to store CUDA-generated results, or\nPointer to host memory to store CPU-generated results\n* `n` - - Number of doubles to generate\n* `mean` - - Mean of normal distribution\n* `stddev` - - Standard deviation of normal distribution\n\n# Returns\n\n- CURAND_STATUS_ALLOCATION_FAILED if memory could not be allocated \\n - CURAND_STATUS_NOT_INITIALIZED if the generator was never created \\n - CURAND_STATUS_PREEXISTING_FAILURE if there was an existing error from\na previous kernel launch \\n - CURAND_STATUS_LAUNCH_FAILURE if the kernel launch failed for any reason \\n - CURAND_STATUS_LENGTH_NOT_MULTIPLE if the number of output samples is\nnot a multiple of the quasirandom dimension, or is not a multiple\nof two for pseudorandom generators \\n - CURAND_STATUS_DOUBLE_PRECISION_REQUIRED if the GPU does not support double precision \\n - CURAND_STATUS_SUCCESS if the results were generated successfully \\n "]
pub unsafe fn curandGenerateLogNormalDouble<T: types::CudaAsPtr>(generator: curandGenerator_t, mut outputPtr: T, n: usize, mean: f64, stddev: f64) -> Result<(), crate::sys::curandStatus_t> {
    let status = unsafe { crate::sys::curandGenerateLogNormalDouble(generator, outputPtr.as_mut_ptr() as *mut _, n, mean, stddev) };
    if status == crate::sys::curandStatus_t::CURAND_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Construct the histogram array for a Poisson distribution.\nConstruct the histogram array for the Poisson distribution with lambda `lambda.`\nFor lambda greater than 2000, an approximation with a normal distribution is used.\n\n# Arguments\n\n* `lambda` - - lambda for the Poisson distribution\n* `discrete_distribution` - - pointer to the histogram in device memory\n\n# Returns\n\n- CURAND_STATUS_ALLOCATION_FAILED if memory could not be allocated \\n - CURAND_STATUS_DOUBLE_PRECISION_REQUIRED if the GPU does not support double precision \\n - CURAND_STATUS_INITIALIZATION_FAILED if there was a problem setting up the GPU \\n - CURAND_STATUS_NOT_INITIALIZED if the distribution pointer was null \\n - CURAND_STATUS_PREEXISTING_FAILURE if there was an existing error from\na previous kernel launch \\n - CURAND_STATUS_OUT_OF_RANGE if lambda is non-positive or greater than 400,000 \\n - CURAND_STATUS_SUCCESS if the histogram was generated successfully \\n "]
pub unsafe fn curandCreatePoissonDistribution<T: types::CudaAsPtr>(lambda: f64, mut discrete_distribution: T) -> Result<(), crate::sys::curandStatus_t> {
    let status = unsafe { crate::sys::curandCreatePoissonDistribution(lambda, discrete_distribution.as_mut_ptr() as *mut _) };
    if status == crate::sys::curandStatus_t::CURAND_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Destroy the histogram array for a discrete distribution (e.g. Poisson).\nDestroy the histogram array for a discrete distribution created by curandCreatePoissonDistribution.\n\n# Arguments\n\n* `discrete_distribution` - - pointer to device memory where the histogram is stored\n\n# Returns\n\n- CURAND_STATUS_NOT_INITIALIZED if the histogram was never created \\n - CURAND_STATUS_SUCCESS if the histogram was destroyed successfully \\n "]
pub unsafe fn curandDestroyDistribution(discrete_distribution: curandDiscreteDistribution_t) -> Result<(), crate::sys::curandStatus_t> {
    let status = unsafe { crate::sys::curandDestroyDistribution(discrete_distribution) };
    if status == crate::sys::curandStatus_t::CURAND_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Generate Poisson-distributed unsigned ints.\nUse `generator` to generate `n` unsigned int results into device memory at\n`outputPtr.`  The device memory must have been previously allocated and must be\nlarge enough to hold all the results.  Launches are done with the stream\nset using ::curandSetStream(), or the null stream if no stream has been set.\nResults are 32-bit unsigned int point values with Poisson distribution, with lambda `lambda.`\n\n# Arguments\n\n* `generator` - - Generator to use\n* `outputPtr` - - Pointer to device memory to store CUDA-generated results, or\nPointer to host memory to store CPU-generated results\n* `n` - - Number of unsigned ints to generate\n* `lambda` - - lambda for the Poisson distribution\n\n# Returns\n\n- CURAND_STATUS_ALLOCATION_FAILED if memory could not be allocated \\n - CURAND_STATUS_NOT_INITIALIZED if the generator was never created \\n - CURAND_STATUS_PREEXISTING_FAILURE if there was an existing error from\na previous kernel launch \\n - CURAND_STATUS_LAUNCH_FAILURE if the kernel launch failed for any reason \\n - CURAND_STATUS_LENGTH_NOT_MULTIPLE if the number of output samples is\nnot a multiple of the quasirandom dimension\\n - CURAND_STATUS_DOUBLE_PRECISION_REQUIRED if the GPU or sm does not support double precision \\n - CURAND_STATUS_OUT_OF_RANGE if lambda is non-positive or greater than 400,000 \\n - CURAND_STATUS_SUCCESS if the results were generated successfully \\n "]
pub unsafe fn curandGeneratePoisson<T: types::CudaAsPtr>(generator: curandGenerator_t, mut outputPtr: T, n: usize, lambda: f64) -> Result<(), crate::sys::curandStatus_t> {
    let status = unsafe { crate::sys::curandGeneratePoisson(generator, outputPtr.as_mut_ptr() as *mut _, n, lambda) };
    if status == crate::sys::curandStatus_t::CURAND_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn curandGeneratePoissonMethod<T: types::CudaAsPtr>(generator: curandGenerator_t, mut outputPtr: T, n: usize, lambda: f64, method: curandMethod_t) -> Result<(), crate::sys::curandStatus_t> {
    let status = unsafe { crate::sys::curandGeneratePoissonMethod(generator, outputPtr.as_mut_ptr() as *mut _, n, lambda, method) };
    if status == crate::sys::curandStatus_t::CURAND_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn curandGenerateBinomial<T: types::CudaAsPtr>(generator: curandGenerator_t, mut outputPtr: T, num: usize, n: u32, p: f64) -> Result<(), crate::sys::curandStatus_t> {
    let status = unsafe { crate::sys::curandGenerateBinomial(generator, outputPtr.as_mut_ptr() as *mut _, num, n as _, p) };
    if status == crate::sys::curandStatus_t::CURAND_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
pub unsafe fn curandGenerateBinomialMethod<T: types::CudaAsPtr>(generator: curandGenerator_t, mut outputPtr: T, num: usize, n: u32, p: f64, method: curandMethod_t) -> Result<(), crate::sys::curandStatus_t> {
    let status = unsafe { crate::sys::curandGenerateBinomialMethod(generator, outputPtr.as_mut_ptr() as *mut _, num, n as _, p, method) };
    if status == crate::sys::curandStatus_t::CURAND_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Setup starting states.\nGenerate the starting state of the generator.  This function is\nautomatically called by generation functions such as\n::curandGenerate() and ::curandGenerateUniform().\nIt can be called manually for performance testing reasons to separate\ntimings for starting state generation and random number generation.\n\n# Arguments\n\n* `generator` - - Generator to update\n\n# Returns\n\n- CURAND_STATUS_ALLOCATION_FAILED if memory could not be allocated \\n - CURAND_STATUS_NOT_INITIALIZED if the generator was never created \\n - CURAND_STATUS_PREEXISTING_FAILURE if there was an existing error from\na previous kernel launch \\n - CURAND_STATUS_LAUNCH_FAILURE if the kernel launch failed for any reason \\n - CURAND_STATUS_SUCCESS if the seeds were generated successfully \\n "]
pub unsafe fn curandGenerateSeeds(generator: curandGenerator_t) -> Result<(), crate::sys::curandStatus_t> {
    let status = unsafe { crate::sys::curandGenerateSeeds(generator) };
    if status == crate::sys::curandStatus_t::CURAND_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Get direction vectors for 32-bit quasirandom number generation.\nGet a pointer to an array of direction vectors that can be used\nfor quasirandom number generation.  The resulting pointer will\nreference an array of direction vectors in host memory.\nThe array contains vectors for many dimensions.  Each dimension\nhas 32 vectors.  Each individual vector is an unsigned int.\nLegal values for `set` are:\n- CURAND_DIRECTION_VECTORS_32_JOEKUO6 (20,000 dimensions)\n- CURAND_SCRAMBLED_DIRECTION_VECTORS_32_JOEKUO6 (20,000 dimensions)\n\n# Arguments\n\n* `vectors` - - Address of pointer in which to return direction vectors\n* `set` - - Which set of direction vectors to use\n\n# Returns\n\n- CURAND_STATUS_OUT_OF_RANGE if the choice of set is invalid \\n - CURAND_STATUS_SUCCESS if the pointer was set successfully \\n "]
pub unsafe fn curandGetDirectionVectors32<T: types::CudaAsPtr>(mut vectors: T, set: curandDirectionVectorSet_t) -> Result<(), crate::sys::curandStatus_t> {
    let status = unsafe { crate::sys::curandGetDirectionVectors32(vectors.as_mut_ptr() as *mut _, set) };
    if status == crate::sys::curandStatus_t::CURAND_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Get scramble constants for 32-bit scrambled Sobol' .\nGet a pointer to an array of scramble constants that can be used\nfor quasirandom number generation.  The resulting pointer will\nreference an array of unsinged ints in host memory.\nThe array contains constants for many dimensions.  Each dimension\nhas a single unsigned int constant.\n\n# Arguments\n\n* `constants` - - Address of pointer in which to return scramble constants\n\n# Returns\n\n- CURAND_STATUS_SUCCESS if the pointer was set successfully \\n "]
pub unsafe fn curandGetScrambleConstants32() -> Result<*mut ::std::os::raw::c_uint, crate::sys::curandStatus_t> {
    let mut out_0: std::mem::MaybeUninit<*mut ::std::os::raw::c_uint> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::curandGetScrambleConstants32(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::curandStatus_t::CURAND_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as *mut ::std::os::raw::c_uint) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
#[doc = "Get direction vectors for 64-bit quasirandom number generation.\nGet a pointer to an array of direction vectors that can be used\nfor quasirandom number generation.  The resulting pointer will\nreference an array of direction vectors in host memory.\nThe array contains vectors for many dimensions.  Each dimension\nhas 64 vectors.  Each individual vector is an unsigned long long.\nLegal values for `set` are:\n- CURAND_DIRECTION_VECTORS_64_JOEKUO6 (20,000 dimensions)\n- CURAND_SCRAMBLED_DIRECTION_VECTORS_64_JOEKUO6 (20,000 dimensions)\n\n# Arguments\n\n* `vectors` - - Address of pointer in which to return direction vectors\n* `set` - - Which set of direction vectors to use\n\n# Returns\n\n- CURAND_STATUS_OUT_OF_RANGE if the choice of set is invalid \\n - CURAND_STATUS_SUCCESS if the pointer was set successfully \\n "]
pub unsafe fn curandGetDirectionVectors64<T: types::CudaAsPtr>(mut vectors: T, set: curandDirectionVectorSet_t) -> Result<(), crate::sys::curandStatus_t> {
    let status = unsafe { crate::sys::curandGetDirectionVectors64(vectors.as_mut_ptr() as *mut _, set) };
    if status == crate::sys::curandStatus_t::CURAND_STATUS_SUCCESS { Ok(()) } else { Err(status) }
}
#[doc = "Get scramble constants for 64-bit scrambled Sobol' .\nGet a pointer to an array of scramble constants that can be used\nfor quasirandom number generation.  The resulting pointer will\nreference an array of unsinged long longs in host memory.\nThe array contains constants for many dimensions.  Each dimension\nhas a single unsigned long long constant.\n\n# Arguments\n\n* `constants` - - Address of pointer in which to return scramble constants\n\n# Returns\n\n- CURAND_STATUS_SUCCESS if the pointer was set successfully \\n "]
pub unsafe fn curandGetScrambleConstants64() -> Result<*mut ::std::os::raw::c_ulonglong, crate::sys::curandStatus_t> {
    let mut out_0: std::mem::MaybeUninit<*mut ::std::os::raw::c_ulonglong> = std::mem::MaybeUninit::zeroed();
    let status = unsafe { crate::sys::curandGetScrambleConstants64(out_0.as_mut_ptr() as *mut _) };
    if status as usize == crate::sys::curandStatus_t::CURAND_STATUS_SUCCESS as usize {
        unsafe { Ok(out_0.assume_init() as *mut ::std::os::raw::c_ulonglong) }
    } else {
        Err(unsafe { std::mem::transmute(status) })
    }
}
