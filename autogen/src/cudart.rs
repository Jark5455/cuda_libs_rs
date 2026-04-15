use std::path::PathBuf;
use quote::quote;
use syn::{ForeignItem, Item, ReturnType, Type};

pub fn generate() {
    let out_dir = PathBuf::from("../cuda_libs_rt/src");
    std::fs::create_dir_all(&out_dir).unwrap();

    let header = "/opt/cuda/include/cuda_runtime.h";
    println!("cargo:rerun-if-changed={}", header);
    
    let builder = bindgen::Builder::default()
        .header(header)
        .clang_arg("-I/opt/cuda/include")
        .default_enum_style(bindgen::EnumVariation::Rust { non_exhaustive: false })
        .derive_default(true)
        .allowlist_function("cuda.*")
        .allowlist_type("cuda.*")
        .allowlist_var("CUDA.*");

    let bindings = builder.generate().expect("Unable to generate bindings");

    let sys_path = out_dir.join("sys.rs");
    bindings
        .write_to_file(&sys_path)
        .expect("Couldn't write sys.rs!");

    let bindings_code = std::fs::read_to_string(&sys_path).unwrap();
    let ast: syn::File = syn::parse_str(&bindings_code).unwrap();

    let mut safe_funcs = Vec::new();
    let mut ctx_methods = Vec::new();
    let mut builder_impls = Vec::new();

    for item in ast.items {
        if let Item::ForeignMod(foreign_mod) = item {
            for foreign_item in foreign_mod.items {
                if let ForeignItem::Fn(func) = foreign_item {
                    let attrs = &func.attrs;
                    let sig = &func.sig;
                    let fn_name = &sig.ident;
                    let mut safe_inputs = Vec::new();
                    let mut safe_inputs_generics = Vec::<proc_macro2::TokenStream>::new();
                    let mut call_args = Vec::new();
                    let generic_letters = ["T", "U", "V", "W", "X", "Y", "Z", "A", "B", "C", "D", "E", "F"];
                    let mut generic_idx = 0;
                    let fn_str = fn_name.to_string();

                    if fn_str == "cudaGreenCtxCreate" || fn_str == "cudaExecutionCtxDestroy" || fn_str == "cudaDeviceGetExecutionCtx" {
                        continue;
                    }

                    if fn_str.starts_with("cudaLibrary") {
                        continue;
                    }

                    let mut is_ctx_method = false;
                    let mut ctx_arg_idx = 0;
                    for (i, input) in sig.inputs.iter().enumerate() {
                        if let syn::FnArg::Typed(pat_type) = input {
                            let real_ty = &pat_type.ty;
                            if quote!(#real_ty).to_string() == "cudaExecutionContext_t" {
                                is_ctx_method = true;
                                ctx_arg_idx = i;
                                break;
                            }
                        }
                    }

                    let is_custom_malloc = fn_str.starts_with("cudaMalloc");
                    let is_custom_free = fn_str.starts_with("cudaFree");
                    let is_getter = fn_str.contains("Get") && !fn_str.contains("String") && !fn_str.contains("Name") && !fn_str.contains("Memcpy") && !fn_str.contains("Devices");

                    if is_custom_malloc {
                        let mut safe_inputs = Vec::new();
                        let mut call_args = Vec::new();
                        
                        let is_array = fn_str.contains("Array") || fn_str.contains("Mipmapped");
                        let is_host = fn_str.contains("Host");
                        let is_3d_pitched = fn_str == "cudaMalloc3D";

                        let mut output_type = quote!();

                        for (i, input) in sig.inputs.iter().enumerate() {
                            if let syn::FnArg::Typed(pat_type) = input {
                                let pat = &pat_type.pat;
                                let ty = &pat_type.ty;
                                if i == 0 {
                                    if let Type::Ptr(ptr_ty) = &**ty {
                                        let inner_ty = &ptr_ty.elem;
                                        output_type = quote!(#inner_ty);
                                    }
                                    call_args.push(quote!(&mut dev_ptr as *mut _ as *mut _));
                                } else {
                                    safe_inputs.push(quote!(#pat: #ty));
                                    call_args.push(quote!(#pat));
                                }
                            }
                        }

                        let safe_fn = if is_array || is_3d_pitched {
                            quote! {
                                pub unsafe fn #fn_name(#(#safe_inputs),*) -> Result<#output_type, crate::sys::cudaError> {
                                    let mut dev_ptr: #output_type = std::mem::zeroed();
                                    let status = unsafe { crate::sys::#fn_name(#(#call_args),*) };
                                    if status == crate::sys::cudaError::cudaSuccess {
                                        Ok(dev_ptr)
                                    } else {
                                        Err(status)
                                    }
                                }
                            }
                        } else if is_host {
                            quote! {
                                pub unsafe fn #fn_name<T>(#(#safe_inputs),*) -> Result<*mut T, crate::sys::cudaError> {
                                    let mut dev_ptr = std::ptr::null_mut();
                                    let status = unsafe { crate::sys::#fn_name(#(#call_args),*) };
                                    if status == crate::sys::cudaError::cudaSuccess {
                                        Ok(dev_ptr as *mut T)
                                    } else {
                                        Err(status)
                                    }
                                }
                            }
                        } else {
                            quote! {
                                pub unsafe fn #fn_name<T>(#(#safe_inputs),*) -> Result<::cuda_libs::types::cuDeviceAllocation<T>, crate::sys::cudaError> {
                                    let mut dev_ptr = std::ptr::null_mut();
                                    let status = unsafe { crate::sys::#fn_name(#(#call_args),*) };
                                    if status == crate::sys::cudaError::cudaSuccess {
                                        Ok(::cuda_libs::types::cuDeviceAllocation(dev_ptr as *mut T))
                                    } else {
                                        Err(status)
                                    }
                                }
                            }
                        };
                        safe_funcs.push(safe_fn);
                        continue;
                    }

                    if is_custom_free {
                         let mut safe_inputs = Vec::new();
                         let mut call_args = Vec::new();
                         
                         let is_host = fn_str.contains("Host");
                         let is_array_or_3d = fn_str.contains("Array") || fn_str.contains("Mipmapped");
                         
                         for (i, input) in sig.inputs.iter().enumerate() {
                             if let syn::FnArg::Typed(pat_type) = input {
                                let pat = &pat_type.pat;
                                let ty = &pat_type.ty;
                                
                                if i == 0 {
                                    if is_array_or_3d {
                                        // They take specific types.
                                        safe_inputs.push(quote!(#pat: #ty));
                                        call_args.push(quote!(#pat));
                                    } else if is_host {
                                        safe_inputs.push(quote!(#pat: *mut T));
                                        call_args.push(quote!(#pat as *mut _));
                                    } else {
                                        safe_inputs.push(quote!(#pat: ::cuda_libs::types::cuDeviceAllocation<T>));
                                        call_args.push(quote!(#pat.0 as *mut _));
                                    }
                                } else {
                                    safe_inputs.push(quote!(#pat: #ty));
                                    call_args.push(quote!(#pat));
                                }
                             }
                         }
                         
                         let safe_fn = if is_array_or_3d {
                             quote! {
                                 pub unsafe fn #fn_name(#(#safe_inputs),*) -> Result<(), crate::sys::cudaError> {
                                     let status = unsafe { crate::sys::#fn_name(#(#call_args),*) };
                                     if status == crate::sys::cudaError::cudaSuccess {
                                         Ok(())
                                     } else {
                                         Err(status)
                                     }
                                 }
                             }
                         } else {
                             quote! {
                                 pub unsafe fn #fn_name<T>(#(#safe_inputs),*) -> Result<(), crate::sys::cudaError> {
                                     let status = unsafe { crate::sys::#fn_name(#(#call_args),*) };
                                     if status == crate::sys::cudaError::cudaSuccess {
                                         Ok(())
                                     } else {
                                         Err(status)
                                     }
                                 }
                             }
                         };
                         safe_funcs.push(safe_fn);
                         continue;
                    }

                    if is_getter {
                        let mut safe_inputs = Vec::new();
                        let mut call_args = Vec::new();
                        let mut out_vars = Vec::new();
                        let mut out_types = Vec::new();
                        let mut out_dcls = Vec::new();
                        
                        let mut has_output = false;

                        for (i, input) in sig.inputs.iter().enumerate() {
                            if let syn::FnArg::Typed(pat_type) = input {
                                let pat = &pat_type.pat;
                                let ty = &pat_type.ty;
                                let pat_str = quote!(#pat).to_string();

                                let mut is_output = false;
                                if let Type::Ptr(ptr_ty) = &**ty {
                                    if ptr_ty.mutability.is_some() && 
                                       !quote!(#ptr_ty).to_string().contains("c_void") && 
                                       !pat_str.contains("Array") && !pat_str.contains("Devices") && !pat_str.contains("device_arr") {
                                        
                                        is_output = true;
                                        has_output = true;
                                        let inner_ty = &ptr_ty.elem;
                                        
                                        let var_name = quote::format_ident!("out_{}", i);
                                        out_dcls.push(quote! {
                                            let mut #var_name: std::mem::MaybeUninit<#inner_ty> = std::mem::MaybeUninit::uninit();
                                        });
                                        
                                        call_args.push(quote! { #var_name.as_mut_ptr() as *mut _ });
                                        out_vars.push(quote! { #var_name.assume_init() });
                                        out_types.push(quote! { #inner_ty });
                                    }
                                }

                                if i == ctx_arg_idx && is_ctx_method {
                                    call_args.push(quote!(self.handle));
                                    continue;
                                }

                                if !is_output {
                                    safe_inputs.push(quote!(#pat: #ty));
                                    call_args.push(quote!(#pat));
                                }
                            }
                        }

                        if has_output {
                            let return_type = if out_types.len() == 1 {
                                let t = &out_types[0];
                                quote!(#t)
                            } else {
                                quote!((#(#out_types),*))
                            };

                            let return_vals = if out_vars.len() == 1 {
                                let v = &out_vars[0];
                                quote!(#v)
                            } else {
                                quote!((#(#out_vars),*))
                            };

                            let safe_fn = if is_ctx_method {
                                quote! {
                                    #(#attrs)*
                                    pub unsafe fn #fn_name(&self, #(#safe_inputs),*) -> Result<#return_type, crate::sys::cudaError> {
                                        #(#out_dcls)*
                                        let status = unsafe { crate::sys::#fn_name(#(#call_args),*) };
                                        if status == crate::sys::cudaError::cudaSuccess {
                                            unsafe { Ok(#return_vals) }
                                        } else {
                                            Err(status)
                                        }
                                    }
                                }
                            } else {
                                quote! {
                                    #(#attrs)*
                                    pub unsafe fn #fn_name(#(#safe_inputs),*) -> Result<#return_type, crate::sys::cudaError> {
                                        #(#out_dcls)*
                                        let status = unsafe { crate::sys::#fn_name(#(#call_args),*) };
                                        if status == crate::sys::cudaError::cudaSuccess {
                                            unsafe { Ok(#return_vals) }
                                        } else {
                                            Err(status)
                                        }
                                    }
                                }
                            };
                            
                            if is_ctx_method {
                                ctx_methods.push(safe_fn);
                            } else {
                                safe_funcs.push(safe_fn);
                            }
                            continue;
                        }
                    }

                    let mut safe_inputs_generics = Vec::new();

                    for (i, input) in sig.inputs.iter().enumerate() {
                        if let syn::FnArg::Typed(pat_type) = input {
                            let pat = &pat_type.pat;
                            let ty = &pat_type.ty;

                            if i == ctx_arg_idx && is_ctx_method {
                                call_args.push(quote!(self.handle));
                                continue;
                            }

                            if let Type::Ptr(ptr_ty) = &**ty {
                                let inner_ty = &ptr_ty.elem;
                                let ty_str = quote!(#inner_ty).to_string();
                                
                                let is_handle = ty_str.contains("Context") || 
                                                ty_str.contains("Stream_t") || 
                                                ty_str.contains("Handle") ||
                                                ty_str.contains("Stream") ||
                                                ty_str.contains("ctx") ||
                                                ty_str.contains("Device") ||
                                                ty_str.contains("CUstream_st");

                                let mutability = &ptr_ty.mutability;

                                if is_handle {
                                    safe_inputs.push(quote!(#pat: #ty));
                                    call_args.push(quote!(#pat));
                                } else {
                                    let generic_ident = if generic_idx < generic_letters.len() {
                                        quote::format_ident!("{}", generic_letters[generic_idx])
                                    } else {
                                        quote::format_ident!("T{}", generic_idx)
                                    };
                                    generic_idx += 1;
                                    safe_inputs_generics.push(quote!(#generic_ident: ::cuda_libs::types::CudaAsPtr));
                                    
                                    if mutability.is_some() {
                                        safe_inputs.push(quote!(mut #pat: #generic_ident));
                                        call_args.push(quote!(#pat.as_mut_ptr() as *mut #inner_ty));
                                    } else {
                                        safe_inputs.push(quote!(#pat: #generic_ident));
                                        call_args.push(quote!(#pat.as_const_ptr() as *const #inner_ty));
                                    }
                                }
                            } else {
                                safe_inputs.push(quote!(#pat: #ty));
                                call_args.push(quote!(#pat));
                            }
                        }
                    }

                    let has_status_ret = if let ReturnType::Type(_, ty) = &sig.output {
                        let ty_s = quote!(#ty).to_string();
                        ty_s == "cudaError" || ty_s == "cudaError_t"
                    } else {
                        false
                    };

                    let safe_fn = if has_status_ret {
                        if is_ctx_method {
                            quote! {
                                #(#attrs)*
                                pub unsafe fn #fn_name<#(#safe_inputs_generics),*>(&self, #(#safe_inputs),*) -> Result<(), crate::sys::cudaError> {
                                    let status = unsafe { crate::sys::#fn_name(#(#call_args),*) };
                                    if status == crate::sys::cudaError::cudaSuccess {
                                        Ok(())
                                    } else {
                                        Err(status)
                                    }
                                }
                            }
                        } else {
                            quote! {
                                #(#attrs)*
                                pub unsafe fn #fn_name<#(#safe_inputs_generics),*>(#(#safe_inputs),*) -> Result<(), crate::sys::cudaError> {
                                    let status = unsafe { crate::sys::#fn_name(#(#call_args),*) };
                                    if status == crate::sys::cudaError::cudaSuccess {
                                        Ok(())
                                    } else {
                                        Err(status)
                                    }
                                }
                            }
                        }
                    } else {
                        let ret = &sig.output;
                        if is_ctx_method {
                            quote! {
                                #(#attrs)*
                                pub unsafe fn #fn_name<#(#safe_inputs_generics),*>(&self, #(#safe_inputs),*) #ret {
                                    unsafe { crate::sys::#fn_name(#(#call_args),*) }
                                }
                            }
                        } else {
                            quote! {
                                #(#attrs)*
                                pub unsafe fn #fn_name<#(#safe_inputs_generics),*>(#(#safe_inputs),*) #ret {
                                    unsafe { crate::sys::#fn_name(#(#call_args),*) }
                                }
                            }
                        }
                    };

                    if is_ctx_method {
                        ctx_methods.push(safe_fn);
                    } else {
                        safe_funcs.push(safe_fn);
                    }
                }
            }
        } else if let Item::Struct(s) = item {
            let ident = &s.ident;
            if let syn::Fields::Named(fields) = &s.fields {
                if !fields.named.is_empty() {
                    let mut builder_fns = Vec::new();
                    for field in &fields.named {
                        if matches!(field.vis, syn::Visibility::Public(_)) {
                            if let Some(f_name) = &field.ident {
                                let f_ty = &field.ty;
                                builder_fns.push(quote! {
                                    pub fn #f_name(mut self, val: #f_ty) -> Self {
                                        self.#f_name = val;
                                        self
                                    }
                                });
                            }
                        }
                    }
                    if !builder_fns.is_empty() {
                        builder_impls.push(quote! {
                            impl crate::sys::#ident {
                                #(#builder_fns)*
                            }
                        });
                    }
                }
            }
        }
    }

    let safe_module = quote! {
        pub use crate::sys::cudaError as CudaStatusEnum;
        #[allow(unused_imports)]
        use crate::sys::*;

        #(#builder_impls)*

        pub struct CudaExecutionContext {
            pub(crate) handle: crate::sys::cudaExecutionContext_t,
        }

        impl CudaExecutionContext {
            pub fn cudaGreenCtxCreate(desc: crate::sys::cudaDevResourceDesc_t, device: std::os::raw::c_int, flags: std::os::raw::c_uint) -> Result<Self, crate::sys::cudaError> {
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

            pub fn cudaDeviceGetExecutionCtx(device: std::os::raw::c_int) -> Result<Self, crate::sys::cudaError> {
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

            #(#ctx_methods)*
        }

        impl Drop for CudaExecutionContext {
            fn drop(&mut self) {
                unsafe {
                    let _ = crate::sys::cudaExecutionCtxDestroy(self.handle);
                }
            }
        }

        #(#safe_funcs)*
    };

    let safe_path = out_dir.join("safe.rs");
    std::fs::write(&safe_path, safe_module.to_string()).unwrap();

    std::process::Command::new("rustfmt")
        .arg(&safe_path)
        .status()
        .expect("Failed to run rustfmt on generated safe.rs");
}
