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
    }

    #[cfg(target_os = "windows")]
    {
        use std::fs;
        use std::path::PathBuf;

        let cudnn_home = env::var("CUDNN_HOME");
        if let Ok(cudnn_home) = cudnn_home {

            let paths = vec![ format!("{}\\bin", cudnn_home), format!("{}\\lib", cudnn_home)];

            for path in paths {
                let path = PathBuf::from(path);
                if let Ok(entries) = fs::read_dir(&path) {
                    let mut versions: Vec<String> = entries
                        .filter_map(|e| e.ok())
                        .filter(|e| e.path().is_dir())
                        .filter_map(|e| e.file_name().into_string().ok())
                        .collect();

                    versions.sort_by(|a, b| {
                        a.partial_cmp(b).unwrap()
                    });

                    if let Some(newest) = versions.last() {
                        let final_path = path.join(newest).join("x64");

                        if final_path.exists() {
                            println!("cargo:rustc-link-search=native={}", final_path.display());
                        }
                    }
                }
            }
        }
    }

    let dynamic = env::var("CARGO_FEATURE_DYNAMIC_LINK").is_ok();
    let kind = if dynamic { "dylib" } else { "static" };

    #[cfg(target_os = "linux")]
    let suffix = if dynamic { "" } else { "_static" };

    #[cfg(target_os = "windows")]
    let suffix = "";

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
