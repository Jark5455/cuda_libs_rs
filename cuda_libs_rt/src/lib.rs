extern crate cuda_libs;

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub mod sys;

pub mod safe;

// TODO: Implement custom safe loader abstractions for `cudaLibrary` endpoints directly addressing host pointers and Jit buffers.
