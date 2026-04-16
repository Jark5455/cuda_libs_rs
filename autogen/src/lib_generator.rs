use quote::quote;
use std::path::PathBuf;
use syn::{ForeignItem, Item, ReturnType, Type};

pub struct HandleConfig {
    pub wrapper_name: &'static str,
    pub handle_type: &'static str,
}

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
    pub handles: Vec<HandleConfig>,
    pub handle_types_regex: Vec<&'static str>,
    pub extra_imports: Vec<&'static str>,
    pub extra_safe_code: &'static str,
}

pub fn generate_library(config: &LibraryConfig) {
    let out_dir = PathBuf::from(config.out_dir);
    std::fs::create_dir_all(&out_dir).unwrap();

    let mut builder = bindgen::Builder::default()
        .clang_arg("-I/opt/cuda/include")
        .clang_arg("-I/usr/include")
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .derive_default(true)
        .allowlist_function(config.allowlist_functions)
        .allowlist_type(config.allowlist_types)
        .allowlist_var(config.allowlist_vars);

    if config.lib_name != "cuda_libs_cudart" {
        builder = builder.raw_line("pub use cuda_libs_cudart::sys::*;");
    }

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

    let bindings = builder
        .generate()
        .expect("Unable to generate bindings for lib");
    let sys_path = out_dir.join("sys.rs");
    bindings
        .write_to_file(&sys_path)
        .expect("Couldn't write sys.rs!");

    let bindings_code = std::fs::read_to_string(&sys_path).unwrap();
    let ast: syn::File = syn::parse_str(&bindings_code).unwrap();

    let mut dynamic_fields = Vec::new();
    let mut dynamic_wrappers = Vec::new();
    let mut dynamic_loaders = Vec::new();
    let mut new_sys_items = Vec::new();

    for item in &ast.items {
        if let syn::Item::ForeignMod(foreign_mod) = item {
            let mut fm_static = foreign_mod.clone();
            fm_static
                .attrs
                .push(syn::parse_quote!(#[cfg(not(feature = "runtime-link"))]));
            new_sys_items.push(syn::Item::ForeignMod(fm_static));

            for foreign_item in &foreign_mod.items {
                if let syn::ForeignItem::Fn(func) = foreign_item {
                    let sig = &func.sig;
                    if sig.variadic.is_some() {
                        continue;
                    }
                    let fn_name = &sig.ident;
                    let fn_name_str = fn_name.to_string();
                    let fn_name_nul = syn::LitByteStr::new(
                        format!("{}\0", fn_name_str).as_bytes(),
                        proc_macro2::Span::call_site(),
                    );

                    let mut arg_types = Vec::new();
                    let mut arg_names = Vec::new();
                    for param in &sig.inputs {
                        if let syn::FnArg::Typed(pat_type) = param {
                            let pat = &pat_type.pat;
                            let ty = &pat_type.ty;
                            arg_types.push(quote!(#pat: #ty));
                            arg_names.push(quote!(#pat));
                        }
                    }
                    let output = &sig.output;

                    dynamic_fields.push(quote! {
                        pub #fn_name: Option<unsafe extern "C" fn(#(#arg_types),*) #output>
                    });

                    dynamic_wrappers.push(quote! {
                        #[cfg(feature = "runtime-link")]
                        #[inline(always)]
                        pub unsafe extern "C" fn #fn_name(#(#arg_types),*) #output {
                            match DYNAMIC_BINDINGS.get().expect("CUDA library not loaded. Did you forget to call #[cuda_load]?").#fn_name {
                                Some(____func) => unsafe { ____func(#(#arg_names),*) },
                                None => panic!("CUDA symbol '{}' not found in the loaded library. This typically happens when using a CUDA version older than the one the bindings were generated for.", #fn_name_str),
                            }
                        }
                    });

                    dynamic_loaders.push(quote! {
                        #fn_name: {
                            let p = get_proc_addr(lib, #fn_name_nul.as_ptr());
                            if p.is_null() {
                                None
                            } else {
                                Some(std::mem::transmute(p))
                            }
                        }
                    });
                }
            }
        } else {
            new_sys_items.push(item.clone());
        }
    }

    if !dynamic_fields.is_empty() {
        let dyn_mod = quote! {
            #[cfg(feature = "runtime-link")]
            pub struct DynamicBindings {
                #(#dynamic_fields),*
            }

            #[cfg(feature = "runtime-link")]
            pub static DYNAMIC_BINDINGS: std::sync::OnceLock<DynamicBindings> = std::sync::OnceLock::new();

            #(#dynamic_wrappers)*

            #[cfg(feature = "runtime-link")]
            pub unsafe fn load_dynamic_bindings(lib: *mut std::ffi::c_void, get_proc_addr: unsafe fn(*mut std::ffi::c_void, *const u8) -> *mut std::ffi::c_void) {
                let bindings = unsafe {
                    DynamicBindings {
                        #(#dynamic_loaders),*
                    }
                };
                DYNAMIC_BINDINGS.set(bindings).ok();
            }
        };
        let dyn_file: syn::File =
            syn::parse2(dyn_mod).expect("Failed to parse dynamic wrapper items");
        new_sys_items.extend(dyn_file.items);
    }

    let new_ast = syn::File {
        shebang: ast.shebang.clone(),
        attrs: ast.attrs.clone(),
        items: new_sys_items,
    };
    std::fs::write(&sys_path, quote!(#new_ast).to_string()).unwrap();

    let mut standalone_funcs = Vec::new();
    let mut handle_methods: std::collections::HashMap<String, Vec<proc_macro2::TokenStream>> =
        std::collections::HashMap::new();
    for h in &config.handles {
        handle_methods.insert(h.wrapper_name.to_string(), Vec::new());
    }
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

                    let mut current_handle_wrapper = None;
                    if let Some(syn::FnArg::Typed(pat_type)) = sig.inputs.first() {
                        let real_ty = &pat_type.ty;
                        let ty_str = quote!(#real_ty).to_string();
                        for h in &config.handles {
                            if ty_str == h.handle_type {
                                current_handle_wrapper = Some(h.wrapper_name.to_string());
                                break;
                            }
                        }
                    }

                    // Cudart specific check: find handle anywhere in args
                    if current_handle_wrapper.is_none() && config.lib_name == "cuda_libs_cudart" {
                        for input in &sig.inputs {
                            if let syn::FnArg::Typed(pat_type) = input {
                                let real_ty = &pat_type.ty;
                                let ty_str = quote!(#real_ty).to_string();
                                if ty_str == "cudaExecutionContext_t" {
                                    current_handle_wrapper =
                                        Some("CudaExecutionContext".to_string());
                                    break;
                                }
                            }
                        }
                    }

                    let is_method = current_handle_wrapper.is_some();

                    let mut safe_inputs = Vec::new();
                    let mut safe_inputs_generics = Vec::<proc_macro2::TokenStream>::new();
                    let mut call_args = Vec::new();
                    let generic_letters = [
                        "T", "U", "V", "W", "X", "Y", "Z", "A", "B", "C", "D", "E", "F",
                    ];
                    let mut generic_idx = 0;

                    let is_getter = fn_str.contains("Get")
                        && !fn_str.contains("String")
                        && !fn_str.contains("Name")
                        && !fn_str.contains("Vector")
                        && !fn_str.contains("Matrix");

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

                                if i == 0 && is_method && config.lib_name != "cuda_libs_cudart" {
                                    call_args.push(quote!(self.handle));
                                    continue;
                                }

                                if config.lib_name == "cuda_libs_cudart" && is_method {
                                    if let syn::Type::Path(p) = &**ty {
                                        if p.path.is_ident("cudaExecutionContext_t") {
                                            call_args.push(quote!(self.handle));
                                            continue;
                                        }
                                    }
                                }

                                let mut is_output = false;
                                if let Type::Ptr(ptr_ty) = &**ty {
                                    if ptr_ty.mutability.is_some()
                                        && !quote!(#ptr_ty).to_string().contains("c_void")
                                        && !pat_str.contains("Array")
                                        && !pat_str.contains("Devices")
                                        && !pat_str.contains("device_arr")
                                    {
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

                            let status_type_ident =
                                syn::Ident::new(config.status_type, proc_macro2::Span::call_site());
                            let success_variant_ident = syn::Ident::new(
                                config.success_variant,
                                proc_macro2::Span::call_site(),
                            );

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
                            if let Some(wrapper) = &current_handle_wrapper {
                                handle_methods.get_mut(wrapper).unwrap().push(safe_fn);
                            } else {
                                standalone_funcs.push(safe_fn);
                            }
                            continue;
                        } else {
                            safe_inputs.clear();
                            call_args.clear();
                        }
                    }

                    // Specialized Malloc logic
                    if config.lib_name == "cuda_libs_cudart" && fn_str.starts_with("cudaMalloc") {
                        let mut m_safe_inputs = Vec::new();
                        let mut m_call_args = Vec::new();
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
                                    m_call_args.push(quote!(&mut dev_ptr as *mut _ as *mut _));
                                } else {
                                    m_safe_inputs.push(quote!(#pat: #ty));
                                    m_call_args.push(quote!(#pat));
                                }
                            }
                        }

                        let safe_fn = if is_array || is_3d_pitched {
                            quote! {
                                pub unsafe fn #fn_name(#(#m_safe_inputs),*) -> Result<#output_type, crate::sys::cudaError> {
                                    let mut dev_ptr: #output_type = unsafe { std::mem::zeroed() };
                                    let status = unsafe { crate::sys::#fn_name(#(#m_call_args),*) };
                                    if status == crate::sys::cudaError::cudaSuccess {
                                        Ok(dev_ptr)
                                    } else {
                                        Err(status)
                                    }
                                }
                            }
                        } else if is_host {
                            quote! {
                                pub unsafe fn #fn_name<T>(#(#m_safe_inputs),*) -> Result<*mut T, crate::sys::cudaError> {
                                    let mut dev_ptr = std::ptr::null_mut();
                                    let status = unsafe { crate::sys::#fn_name(#(#m_call_args),*) };
                                    if status == crate::sys::cudaError::cudaSuccess {
                                        Ok(dev_ptr as *mut T)
                                    } else {
                                        Err(status)
                                    }
                                }
                            }
                        } else {
                            quote! {
                                pub unsafe fn #fn_name<T>(#(#m_safe_inputs),*) -> Result<::cuda_libs_cudart::types::cuDeviceAllocation<T>, crate::sys::cudaError> {
                                    let mut dev_ptr = std::ptr::null_mut();
                                    let status = unsafe { crate::sys::#fn_name(#(#m_call_args),*) };
                                    if status == crate::sys::cudaError::cudaSuccess {
                                        Ok(::cuda_libs_cudart::types::cuDeviceAllocation(dev_ptr as *mut T))
                                    } else {
                                        Err(status)
                                    }
                                }
                            }
                        };
                        standalone_funcs.push(safe_fn);
                        continue;
                    }

                    // Specialized Free logic
                    if config.lib_name == "cuda_libs_cudart" && fn_str.starts_with("cudaFree") {
                        let mut f_safe_inputs = Vec::new();
                        let mut f_call_args = Vec::new();
                        let is_host = fn_str.contains("Host");
                        let is_array_or_3d =
                            fn_str.contains("Array") || fn_str.contains("Mipmapped");

                        for (i, input) in sig.inputs.iter().enumerate() {
                            if let syn::FnArg::Typed(pat_type) = input {
                                let pat = &pat_type.pat;
                                let ty = &pat_type.ty;
                                if i == 0 {
                                    if is_array_or_3d {
                                        f_safe_inputs.push(quote!(#pat: #ty));
                                        f_call_args.push(quote!(#pat));
                                    } else if is_host {
                                        f_safe_inputs.push(quote!(#pat: *mut T));
                                        f_call_args.push(quote!(#pat as *mut _));
                                    } else {
                                        f_safe_inputs.push(quote!(#pat: ::cuda_libs_cudart::types::cuDeviceAllocation<T>));
                                        f_call_args.push(quote!(#pat.0 as *mut _));
                                    }
                                } else {
                                    f_safe_inputs.push(quote!(#pat: #ty));
                                    f_call_args.push(quote!(#pat));
                                }
                            }
                        }

                        let safe_fn = if is_array_or_3d {
                            quote! {
                                pub unsafe fn #fn_name(#(#f_safe_inputs),*) -> Result<(), crate::sys::cudaError> {
                                    let status = unsafe { crate::sys::#fn_name(#(#f_call_args),*) };
                                    if status == crate::sys::cudaError::cudaSuccess {
                                        Ok(())
                                    } else {
                                        Err(status)
                                    }
                                }
                            }
                        } else {
                            quote! {
                                pub unsafe fn #fn_name<T>(#(#f_safe_inputs),*) -> Result<(), crate::sys::cudaError> {
                                    let status = unsafe { crate::sys::#fn_name(#(#f_call_args),*) };
                                    if status == crate::sys::cudaError::cudaSuccess {
                                        Ok(())
                                    } else {
                                        Err(status)
                                    }
                                }
                            }
                        };
                        standalone_funcs.push(safe_fn);
                        continue;
                    }

                    for (i, input) in sig.inputs.iter().enumerate() {
                        if let syn::FnArg::Typed(pat_type) = input {
                            let pat = &pat_type.pat;
                            let ty = &pat_type.ty;

                            if i == 0 && is_method && config.lib_name != "cuda_libs_cudart" {
                                call_args.push(quote!(self.handle));
                                continue;
                            }

                            if config.lib_name == "cuda_libs_cudart" && is_method {
                                if let syn::Type::Path(p) = &**ty {
                                    if p.path.is_ident("cudaExecutionContext_t") {
                                        call_args.push(quote!(self.handle));
                                        continue;
                                    }
                                }
                            }

                            if let Type::Ptr(ptr_ty) = &**ty {
                                let inner_ty = &ptr_ty.elem;
                                let ty_str = quote!(#inner_ty).to_string();

                                let mut is_handle = ty_str.contains("Context")
                                    || ty_str.contains("Stream_t")
                                    || ty_str.contains("Stream")
                                    || ty_str.contains("ctx")
                                    || ty_str.contains("Device")
                                    || ty_str.contains("CUstream_st");

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
                                    safe_inputs_generics.push(quote!(#generic_ident: ::cuda_libs_cudart::types::CudaAsPtr));

                                    if mutability.is_some() {
                                        safe_inputs.push(quote!(mut #pat: #generic_ident));
                                        call_args.push(quote!(#pat.as_mut_ptr() as *mut #inner_ty));
                                    } else {
                                        safe_inputs.push(quote!(#pat: #generic_ident));
                                        call_args
                                            .push(quote!(#pat.as_const_ptr() as *const #inner_ty));
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

                    let status_type_ident =
                        syn::Ident::new(config.status_type, proc_macro2::Span::call_site());
                    let success_variant_ident =
                        syn::Ident::new(config.success_variant, proc_macro2::Span::call_site());

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
                        let wrapper = current_handle_wrapper.as_ref().unwrap();
                        handle_methods.get_mut(wrapper).unwrap().push(safe_fn);
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

    let mut wrapper_structs = Vec::new();
    for h in &config.handles {
        let h_type_ident = syn::Ident::new(h.handle_type, proc_macro2::Span::call_site());
        let w_name_ident = syn::Ident::new(h.wrapper_name, proc_macro2::Span::call_site());
        let methods = handle_methods.get(h.wrapper_name).unwrap();

        wrapper_structs.push(quote! {
            pub struct #w_name_ident {
                pub(crate) handle: crate::sys::#h_type_ident,
            }

            impl #w_name_ident {
                #(#methods)*
            }
        });
    }

    let mut extra_safes = Vec::new();
    for import in &config.extra_imports {
        let import_ident = syn::Ident::new(import, proc_macro2::Span::call_site());
        extra_safes.push(quote! {
            use #import_ident::sys::*;
        });
    }

    let status_type_ident = syn::Ident::new(config.status_type, proc_macro2::Span::call_site());
    let rt_import = if config.lib_name != "cuda_libs_cudart" {
        quote!(
            use cuda_libs_cudart::sys::*;
        )
    } else {
        quote!()
    };

    let safe_module = quote! {
        pub use crate::sys::#status_type_ident as CudaTargetStatus;
        #[allow(unused_imports)]
        use crate::sys::*;
        #rt_import
        #(#extra_safes)*

        #(#builder_impls)*

        #(#wrapper_structs)*

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

    std::process::Command::new("cargo")
        .arg("fmt")
        .status()
        .expect("Failed to run cargo fmt on generated bindings");
}
