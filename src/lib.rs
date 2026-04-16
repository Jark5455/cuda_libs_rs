#[cfg(feature = "cudart")]
pub use cuda_libs_cudart::types;

#[cfg(feature = "runtime-link")]
pub use cuda_libs_macros::cuda_load;

#[cfg(feature = "cublas")]
pub use cuda_libs_cublas as cublas;
#[cfg(feature = "cublas_lt")]
pub use cuda_libs_cublas_lt as cublas_lt;
#[cfg(feature = "cudart")]
pub use cuda_libs_cudart as cudart;
#[cfg(feature = "cudnn")]
pub use cuda_libs_cudnn as cudnn;
#[cfg(feature = "cufft")]
pub use cuda_libs_cufft as cufft;
#[cfg(feature = "curand")]
pub use cuda_libs_curand as curand;
#[cfg(feature = "cusolver")]
pub use cuda_libs_cusolver as cusolver;
#[cfg(feature = "cusparse")]
pub use cuda_libs_cusparse as cusparse;

// Feature validations
#[cfg(all(feature = "cublas", not(feature = "cudart")))]
compile_error!("Feature `cublas` requires feature `cudart` to be enabled.");

#[cfg(all(feature = "cublas_lt", not(feature = "cudart")))]
compile_error!("Feature `cublas_lt` requires feature `cudart` to be enabled.");

#[cfg(all(feature = "cublas_lt", not(feature = "cublas")))]
compile_error!("Feature `cublas_lt` requires feature `cublas` to be enabled.");

#[cfg(all(feature = "cudnn", not(feature = "cudart")))]
compile_error!("Feature `cudnn` requires feature `cudart` to be enabled.");

#[cfg(all(feature = "cudnn", not(feature = "cublas")))]
compile_error!("Feature `cudnn` requires feature `cublas` to be enabled.");

#[cfg(all(feature = "cufft", not(feature = "cudart")))]
compile_error!("Feature `cufft` requires feature `cudart` to be enabled.");

#[cfg(all(feature = "curand", not(feature = "cudart")))]
compile_error!("Feature `curand` requires feature `cudart` to be enabled.");

#[cfg(all(feature = "cusolver", not(feature = "cudart")))]
compile_error!("Feature `cusolver` requires feature `cudart` to be enabled.");

#[cfg(all(feature = "cusolver", not(feature = "cublas")))]
compile_error!("Feature `cusolver` requires feature `cublas` to be enabled.");

#[cfg(all(feature = "cusparse", not(feature = "cudart")))]
compile_error!("Feature `cusparse` requires feature `cudart` to be enabled.");

pub mod prelude {
    #[cfg(feature = "cudart")]
    pub use cuda_libs_cudart::safe::*;
    #[cfg(feature = "cublas")]
    pub use cuda_libs_cublas::safe::*;
    #[cfg(feature = "cublas_lt")]
    pub use cuda_libs_cublas_lt::safe::*;
    #[cfg(feature = "cudnn")]
    pub use cuda_libs_cudnn::safe::*;
    #[cfg(feature = "cufft")]
    pub use cuda_libs_cufft::safe::*;
    #[cfg(feature = "curand")]
    pub use cuda_libs_curand::safe::*;
    #[cfg(feature = "cusolver")]
    pub use cuda_libs_cusolver::safe::*;
    #[cfg(feature = "cusparse")]
    pub use cuda_libs_cusparse::safe::*;
}

#[cfg(feature = "runtime-link")]
pub fn runtime_link_load() {
    static INIT: std::sync::Once = std::sync::Once::new();
    INIT.call_once(|| {
        let home =
            std::env::var("CUDA_HOME").expect("CUDA_HOME must be set when using runtime linking");

        let mut names = std::vec::Vec::new();
        #[cfg(feature = "cudart")]
        names.push("cudart");
        #[cfg(feature = "cublas")]
        names.push("cublas");
        #[cfg(feature = "cublas_lt")]
        names.push("cublasLt");
        #[cfg(feature = "cudnn")]
        names.push("cudnn");
        #[cfg(feature = "cufft")]
        names.push("cufft");
        #[cfg(feature = "curand")]
        names.push("curand");
        #[cfg(feature = "cusolver")]
        names.push("cusolver");
        #[cfg(feature = "cusparse")]
        names.push("cusparse");

        macro_rules! bind_sys {
            ($lib:ident, $fetch_p:ident, $name:expr) => {
                #[cfg(feature = "cudart")]
                if $name == &"cudart" {
                    crate::cudart::sys::load_dynamic_bindings($lib, $fetch_p);
                    continue;
                }
                #[cfg(feature = "cublas")]
                if $name == &"cublas" {
                    crate::cublas::sys::load_dynamic_bindings($lib, $fetch_p);
                    continue;
                }
                #[cfg(feature = "cublas_lt")]
                if $name == &"cublasLt" {
                    crate::cublas_lt::sys::load_dynamic_bindings($lib, $fetch_p);
                    continue;
                }
                #[cfg(feature = "cudnn")]
                if $name == &"cudnn" {
                    crate::cudnn::sys::load_dynamic_bindings($lib, $fetch_p);
                    continue;
                }
                #[cfg(feature = "cufft")]
                if $name == &"cufft" {
                    crate::cufft::sys::load_dynamic_bindings($lib, $fetch_p);
                    continue;
                }
                #[cfg(feature = "curand")]
                if $name == &"curand" {
                    crate::curand::sys::load_dynamic_bindings($lib, $fetch_p);
                    continue;
                }
                #[cfg(feature = "cusolver")]
                if $name == &"cusolver" {
                    crate::cusolver::sys::load_dynamic_bindings($lib, $fetch_p);
                    continue;
                }
                #[cfg(feature = "cusparse")]
                if $name == &"cusparse" {
                    crate::cusparse::sys::load_dynamic_bindings($lib, $fetch_p);
                    continue;
                }
            };
        }

        #[cfg(target_os = "windows")]
        {
            extern "system" {
                fn LoadLibraryW(lpLibFileName: *const u16) -> *mut std::ffi::c_void;
                fn GetProcAddress(
                    hModule: *mut std::ffi::c_void,
                    lpProcName: *const u8,
                ) -> *mut std::ffi::c_void;
            }
            use std::os::windows::ffi::OsStrExt;

            let search_paths = [
                std::path::Path::new(&home).join("lib"),
                std::path::Path::new(&home).join("bin"),
            ];
            for path in search_paths {
                if let Ok(entries) = std::fs::read_dir(&path) {
                    for entry in entries.filter_map(std::result::Result::ok) {
                        let p = entry.path();
                        if p.extension().and_then(|s| s.to_str()) == Some("dll") {
                            for name in &names {
                                if let Some(file_name) = p.file_name().and_then(|n| n.to_str()) {
                                    if file_name.starts_with(&format!("{}64_", name))
                                        || file_name == &format!("{}.dll", name)
                                    {
                                        let mut wide: std::vec::Vec<u16> =
                                            p.as_os_str().encode_wide().collect();
                                        wide.push(0);
                                        unsafe {
                                            let lib = LoadLibraryW(wide.as_ptr());
                                            if !lib.is_null() {
                                                unsafe fn fetch_p(
                                                    handle: *mut core::ffi::c_void,
                                                    sym: *const u8,
                                                ) -> *mut core::ffi::c_void
                                                {
                                                    GetProcAddress(handle, sym)
                                                }
                                                bind_sys!(lib, fetch_p, name);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        #[cfg(target_os = "linux")]
        {
            use std::os::unix::ffi::OsStrExt;
            let flag = libc::RTLD_LAZY | libc::RTLD_GLOBAL;

            let search_paths = [
                std::path::Path::new(&home).join("lib64"),
                std::path::Path::new(&home).join("lib"),
            ];
            for path in search_paths {
                if let Ok(entries) = std::fs::read_dir(&path) {
                    for entry in entries.filter_map(std::result::Result::ok) {
                        let p = entry.path();
                        if p.extension().and_then(|s| s.to_str()) == Some("so") {
                            for name in &names {
                                if let Some(file_name) = p.file_name().and_then(|n| n.to_str()) {
                                    if file_name.starts_with(&format!("lib{}", name)) {
                                        let mut bytes = p.as_os_str().as_bytes().to_vec();
                                        bytes.push(0);
                                        unsafe {
                                            let lib = libc::dlopen(
                                                bytes.as_ptr() as *const libc::c_char,
                                                flag,
                                            );
                                            if !lib.is_null() {
                                                unsafe fn fetch_p(
                                                    handle: *mut core::ffi::c_void,
                                                    sym: *const u8,
                                                ) -> *mut core::ffi::c_void
                                                {
                                                    unsafe { libc::dlsym(handle, sym as *const _) }
                                                }
                                                bind_sys!(lib, fetch_p, name);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    });
}
