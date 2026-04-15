use std::path::PathBuf;
use quote::quote;
use syn::{ForeignItem, Item, ReturnType, Type};

pub struct LibraryConfig {
    pub lib_name: &'static str,
    pub out_dir: &'static str,
    pub headers: Vec<&'static str>,
    pub allowlist_functions: &'static str,
    pub allowlist_types: &'static str,
    pub allowlist_vars: &'static str,
    pub blocklist_types: Vec<&'static str>,
    pub status_type: &'static str,
    pub success_variant: &'static str,
    pub handle_type: &'static str,
    pub handle_types_regex: Vec<&'static str>,
    pub generate_handle_wrapper: bool,
    pub handle_wrapper_name: &'static str,
    pub extra_imports: Vec<&'static str>,
    pub extra_safe_code: &'static str,
}

pub fn generate_library(config: &LibraryConfig) {
    let out_dir = PathBuf::from(config.out_dir);
    std::fs::create_dir_all(&out_dir).unwrap();

    let mut builder = bindgen::Builder::default()
        .clang_arg("-I/opt/cuda/include")
        .clang_arg("-I/usr/include")
        .default_enum_style(bindgen::EnumVariation::Rust { non_exhaustive: false })
        .derive_default(true)
        .allowlist_function(config.allowlist_functions)
        .allowlist_type(config.allowlist_types)
        .allowlist_var(config.allowlist_vars)
        .raw_line("pub use cuda_libs_rt::sys::*;");

    for import in &config.extra_imports {
        builder = builder.raw_line(format!("pub use {}::sys::*;", import));
    }

    for block in &config.blocklist_types {
        builder = builder.blocklist_type(*block);
    }

    for header in &config.headers {
        println!("cargo:rerun-if-changed={}", header);
        builder = builder.header(*header);
    }

    let bindings = builder.generate().expect("Unable to generate bindings for lib");
    let sys_path = out_dir.join("sys.rs");
    bindings.write_to_file(&sys_path).expect("Couldn't write sys.rs!");

    let bindings_code = std::fs::read_to_string(&sys_path).unwrap();
    let ast: syn::File = syn::parse_str(&bindings_code).unwrap();

    let mut standalone_funcs = Vec::new();
    let mut handle_methods = Vec::new();
    let mut builder_impls = Vec::new();

    for item in ast.items {
        if let Item::ForeignMod(foreign_mod) = item {
            for foreign_item in foreign_mod.items {
                if let ForeignItem::Fn(func) = foreign_item {
                    let attrs = &func.attrs;
                    let sig = &func.sig;
                    let fn_name = &sig.ident;
                    let fn_str = fn_name.to_string();

                    if fn_str.contains("Create") || fn_str.contains("Destroy") {
                        continue;
                    }

                    let mut is_method = false;
                    if let Some(syn::FnArg::Typed(pat_type)) = sig.inputs.first() {
                        let real_ty = &pat_type.ty;
                        if quote!(#real_ty).to_string() == config.handle_type {
                            is_method = true;
                        }
                    }

                    let mut safe_inputs = Vec::new();
                    let mut safe_inputs_generics = Vec::<proc_macro2::TokenStream>::new();
                    let mut call_args = Vec::new();
                    let generic_letters = ["T", "U", "V", "W", "X", "Y", "Z", "A", "B", "C", "D", "E", "F"];
                    let mut generic_idx = 0;

                    let is_getter = fn_str.contains("Get") && !fn_str.contains("String") && !fn_str.contains("Name") && !fn_str.contains("Vector") && !fn_str.contains("Matrix");

                    if is_getter {
                        let mut out_vars = Vec::new();
                        let mut out_types = Vec::new();
                        let mut out_dcls = Vec::new();
                        let mut has_output = false;

                        for (i, input) in sig.inputs.iter().enumerate() {
                            if let syn::FnArg::Typed(pat_type) = input {
                                let pat = &pat_type.pat;
                                let ty = &pat_type.ty;
                                let pat_str = quote!(#pat).to_string();

                                if i == 0 && is_method {
                                    call_args.push(quote!(self.handle));
                                    continue;
                                }

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

                            let status_type_ident = syn::Ident::new(config.status_type, proc_macro2::Span::call_site());
                            let success_variant_ident = syn::Ident::new(config.success_variant, proc_macro2::Span::call_site());

                            let safe_fn = if is_method {
                                quote! {
                                    #(#attrs)*
                                    pub unsafe fn #fn_name(&self, #(#safe_inputs),*) -> Result<#return_type, crate::sys::#status_type_ident> {
                                        #(#out_dcls)*
                                        let status = unsafe { crate::sys::#fn_name(#(#call_args),*) };
                                        if status == crate::sys::#status_type_ident::#success_variant_ident {
                                            unsafe { Ok(#return_vals) }
                                        } else {
                                            Err(status)
                                        }
                                    }
                                }
                            } else {
                                quote! {
                                    #(#attrs)*
                                    pub unsafe fn #fn_name(#(#safe_inputs),*) -> Result<#return_type, crate::sys::#status_type_ident> {
                                        #(#out_dcls)*
                                        let status = unsafe { crate::sys::#fn_name(#(#call_args),*) };
                                        if status == crate::sys::#status_type_ident::#success_variant_ident {
                                            unsafe { Ok(#return_vals) }
                                        } else {
                                            Err(status)
                                        }
                                    }
                                }
                            };
                            if is_method {
                                handle_methods.push(safe_fn);
                            } else {
                                standalone_funcs.push(safe_fn);
                            }
                            continue;
                        } else {
                            safe_inputs.clear();
                            call_args.clear();
                        }
                    }

                    for (i, input) in sig.inputs.iter().enumerate() {
                        if let syn::FnArg::Typed(pat_type) = input {
                            let pat = &pat_type.pat;
                            let ty = &pat_type.ty;

                            if i == 0 && is_method {
                                call_args.push(quote!(self.handle));
                                continue;
                            }

                            if let Type::Ptr(ptr_ty) = &**ty {
                                let inner_ty = &ptr_ty.elem;
                                let ty_str = quote!(#inner_ty).to_string();
                                
                                let mut is_handle = ty_str.contains("Context") || 
                                                ty_str.contains("Stream_t") || 
                                                ty_str.contains("Stream") ||
                                                ty_str.contains("ctx") ||
                                                ty_str.contains("Device") ||
                                                ty_str.contains("CUstream_st");
                                
                                for handle_check in &config.handle_types_regex {
                                    if ty_str.contains(handle_check) {
                                        is_handle = true;
                                        break;
                                    }
                                }

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
                        quote!(#ty).to_string() == config.status_type
                    } else {
                        false
                    };

                    let status_type_ident = syn::Ident::new(config.status_type, proc_macro2::Span::call_site());
                    let success_variant_ident = syn::Ident::new(config.success_variant, proc_macro2::Span::call_site());

                    let safe_fn = if has_status_ret {
                        if is_method {
                            quote! {
                                #(#attrs)*
                                pub unsafe fn #fn_name<#(#safe_inputs_generics),*>(&self, #(#safe_inputs),*) -> Result<(), crate::sys::#status_type_ident> {
                                    let status = unsafe { crate::sys::#fn_name(#(#call_args),*) };
                                    if status == crate::sys::#status_type_ident::#success_variant_ident {
                                        Ok(())
                                    } else {
                                        Err(status)
                                    }
                                }
                            }
                        } else {
                            quote! {
                                #(#attrs)*
                                pub unsafe fn #fn_name<#(#safe_inputs_generics),*>(#(#safe_inputs),*) -> Result<(), crate::sys::#status_type_ident> {
                                    let status = unsafe { crate::sys::#fn_name(#(#call_args),*) };
                                    if status == crate::sys::#status_type_ident::#success_variant_ident {
                                        Ok(())
                                    } else {
                                        Err(status)
                                    }
                                }
                            }
                        }
                    } else {
                        let ret = &sig.output;
                        if is_method {
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
                    if is_method {
                        handle_methods.push(safe_fn);
                    } else {
                        standalone_funcs.push(safe_fn);
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

    let status_type_ident = syn::Ident::new(config.status_type, proc_macro2::Span::call_site());
    let handle_type_ident = syn::Ident::new(config.handle_type, proc_macro2::Span::call_site());
    let wrapper_name_ident = syn::Ident::new(config.handle_wrapper_name, proc_macro2::Span::call_site());
    
    let wrapper_struct = if config.generate_handle_wrapper {
        quote! {
            pub struct #wrapper_name_ident {
                pub(crate) handle: crate::sys::#handle_type_ident,
            }
            
            impl #wrapper_name_ident {
                #(#handle_methods)*
            }
        }
    } else {
        quote! {
            #(#handle_methods)*
        }
    };

    let mut extra_safes = Vec::new();
    for import in &config.extra_imports {
        let import_ident = syn::Ident::new(import, proc_macro2::Span::call_site());
        extra_safes.push(quote! {
            use #import_ident::sys::*;
        });
    }

    let safe_module = quote! {
        pub use crate::sys::#status_type_ident as CudaTargetStatus;
        #[allow(unused_imports)]
        use crate::sys::*;
        use cuda_libs_rt::sys::*;
        #(#extra_safes)*

        #(#builder_impls)*

        #wrapper_struct

        #(#standalone_funcs)*
    };

    let extra_safe = if !config.extra_safe_code.is_empty() {
        syn::parse_str::<proc_macro2::TokenStream>(config.extra_safe_code).unwrap()
    } else {
        quote! {}
    };

    let full_safe_module = quote! {
        #safe_module
        #extra_safe
    };

    let safe_path = out_dir.join("safe.rs");
    std::fs::write(&safe_path, full_safe_module.to_string()).unwrap();

    std::process::Command::new("rustfmt")
        .arg(&safe_path)
        .status()
        .expect("Failed to run rustfmt on generated safe.rs");
}
