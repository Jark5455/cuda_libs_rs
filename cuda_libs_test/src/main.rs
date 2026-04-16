use cuda_libs::prelude::cudaGetDeviceCount;

#[cuda_libs::cuda_load]
fn main() {
    println!("Hello, world!");
    let device_count = unsafe { cudaGetDeviceCount().expect("Failed to get device count") };
    println!("Cuda device count: {}", device_count);
}