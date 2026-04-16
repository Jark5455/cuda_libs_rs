use std::env;

fn main() {
    println!("cargo:rerun-if-env-changed=CUDA_HOME");
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_DYNAMIC_LINK");
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_RUNTIME_LINK");

    if env::var("CARGO_FEATURE_RUNTIME_LINK").is_ok() {
        return; // Runtime linking handles libraries via macros, not build time
    }

    let cuda_home = env::var("CUDA_HOME");
    if let Ok(home) = cuda_home {
        let arch_paths = if cfg!(target_os = "windows") {
            vec![format!("{}\\bin\\x64", home), format!("{}\\bin", home), format!("{}\\lib\\x64", home), format!("{}\\lib", home)]
        } else if cfg!(target_os = "linux") {
            vec![format!("{}/lib64", home), format!("{}/lib", home)]
        } else {
            vec![]
        };

        for path in arch_paths {
            println!("cargo:rustc-link-search=native={}", path);
        }
    } else {
        // Fallback to regular linker path implicitly by Cargo
    }

    let dynamic = env::var("CARGO_FEATURE_DYNAMIC_LINK").is_ok();
    let kind = if dynamic { "dylib" } else { "static" };
    let suffix = if dynamic { "" } else { "_static" };

    if env::var("CARGO_FEATURE_CUDART").is_ok() {
        println!("cargo:rustc-link-lib={}=cudart{}", kind, suffix);
    }
    if env::var("CARGO_FEATURE_CUBLAS").is_ok() {
        println!("cargo:rustc-link-lib={}=cublas{}", kind, suffix);
    }
    if env::var("CARGO_FEATURE_CUBLAS_LT").is_ok() {
        println!("cargo:rustc-link-lib={}=cublasLt{}", kind, suffix);
    }
    if env::var("CARGO_FEATURE_CUDNN").is_ok() {
        println!("cargo:rustc-link-lib={}=cudnn{}", kind, suffix);
    }
    if env::var("CARGO_FEATURE_CUFFT").is_ok() {
        println!("cargo:rustc-link-lib={}=cufft{}", kind, suffix);
    }
    if env::var("CARGO_FEATURE_CURAND").is_ok() {
        println!("cargo:rustc-link-lib={}=curand{}", kind, suffix);
    }
    if env::var("CARGO_FEATURE_CUSOLVER").is_ok() {
        println!("cargo:rustc-link-lib={}=cusolver{}", kind, suffix);
    }
    if env::var("CARGO_FEATURE_CUSPARSE").is_ok() {
        println!("cargo:rustc-link-lib={}=cusparse{}", kind, suffix);
    }
}
