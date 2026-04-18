use bindgen::callbacks::ParseCallbacks;
use quote::quote;
use regex::Regex;
use std::collections::HashMap;
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
    pub blocklist_functions: Vec<&'static str>,
    pub status_type: &'static str,
    pub success_variant: &'static str,
    pub handles: Vec<HandleConfig>,
    pub handle_types_regex: Vec<&'static str>,
    pub extra_imports: Vec<&'static str>,
    pub extra_safe_code: &'static str,
    /// Whether to use `types::CudaAsPtr` generic bounds for pointer arguments in safe wrappers.
    /// Set to false for libs that don't depend on cuda_libs_cudart (e.g. cuda_libs_driver).
    pub use_cuda_as_ptr: bool,
}

#[derive(Debug)]
struct DoxygenConverter;

impl ParseCallbacks for DoxygenConverter {
    fn process_comment(&self, comment: &str) -> Option<String> {
        match doxygen_bindgen::transform(comment) {
            Ok(transformed) => Some(transformed),
            Err(_) => None,
        }
    }
}

pub fn generate_library(config: &LibraryConfig) {
    let mut generator = Generator::new(config);
    generator.generate();
}

struct Generator<'a> {
    config: &'a LibraryConfig,
    blocklist_funcs: Vec<Regex>,
}

impl<'a> Generator<'a> {
    fn new(config: &'a LibraryConfig) -> Self {
        let blocklist_funcs = config.blocklist_functions.iter().map(|s| Regex::new(s).expect("Invalid blocklist function regex")).collect();
        Self { config, blocklist_funcs }
    }

    fn generate(&mut self) {
        let out_dir = PathBuf::from(self.config.out_dir);
        std::fs::create_dir_all(&out_dir).unwrap();

        let bindings_code = self.generate_sys_bindings(&out_dir);
        let ast: syn::File = syn::parse_str(&bindings_code).unwrap();

        let mut standalone_funcs = Vec::new();
        let mut handle_methods: HashMap<String, Vec<proc_macro2::TokenStream>> = HashMap::new();
        for h in &self.config.handles {
            handle_methods.insert(h.wrapper_name.to_string(), Vec::new());
        }
        let mut builder_impls = Vec::new();

        for item in &ast.items {
            match item {
                Item::ForeignMod(foreign_mod) => {
                    for foreign_item in &foreign_mod.items {
                        if let ForeignItem::Fn(func) = foreign_item {
                            if self.should_skip_function(func) {
                                continue;
                            }

                            if self.config.lib_name == "cuda_libs_cudart" {
                                if let Some(safe_fn) = self.handle_specialized_cudart(func) {
                                    standalone_funcs.push(safe_fn);
                                    continue;
                                }
                            }

                            let (safe_fn, _) = self.generate_function_wrapper(func);
                            standalone_funcs.push(safe_fn);
                        }
                    }
                }
                Item::Struct(s) => {
                    if let Some(builder_impl) = self.generate_builder_impl(s) {
                        builder_impls.push(builder_impl);
                    }
                }
                _ => {}
            }
        }

        self.write_safe_rs(&out_dir, standalone_funcs, builder_impls);
    }

    fn generate_sys_bindings(&self, out_dir: &std::path::Path) -> String {
        let mut builder = bindgen::Builder::default()
            .clang_arg("-I/opt/cuda/include")
            .clang_arg("-I/usr/include")
            .default_enum_style(bindgen::EnumVariation::Rust { non_exhaustive: false })
            .layout_tests(false)
            .derive_default(true)
            .allowlist_function(self.config.allowlist_functions)
            .allowlist_type(self.config.allowlist_types)
            .allowlist_var(self.config.allowlist_vars)
            .generate_comments(true)
            .parse_callbacks(Box::new(DoxygenConverter));

        for header in &self.config.headers {
            println!("cargo:rerun-if-changed={}", header);
            builder = builder.header(*header);
        }

        for block in &self.config.blocklist_types {
            builder = builder.blocklist_type(*block);
        }

        // Add extra imports to sys.rs
        for import in &self.config.extra_imports {
            builder = builder.raw_line(format!("use {}::sys::*; ", import));
        }

        let bindings = builder.generate().expect("Unable to generate bindings for lib");
        let sys_path = out_dir.join("sys.rs");
        bindings.write_to_file(&sys_path).expect("Couldn't write sys.rs!");

        let bindings_code = std::fs::read_to_string(&sys_path).unwrap();
        let ast: syn::File = syn::parse_str(&bindings_code).unwrap();

        self.generate_dynamic_sys(&sys_path, &ast);
        std::fs::read_to_string(&sys_path).unwrap()
    }

    fn generate_dynamic_sys(&self, sys_path: &std::path::Path, ast: &syn::File) {
        let mut dynamic_fields = Vec::new();
        let mut dynamic_wrappers = Vec::new();
        let mut dynamic_loaders = Vec::new();
        let mut new_sys_items = Vec::new();

        for item in &ast.items {
            if let syn::Item::ForeignMod(foreign_mod) = item {
                let mut fm_static = foreign_mod.clone();
                fm_static.attrs.push(syn::parse_quote!(#[cfg(not(feature = "runtime-link"))]));
                new_sys_items.push(syn::Item::ForeignMod(fm_static));

                for foreign_item in &foreign_mod.items {
                    if let syn::ForeignItem::Fn(func) = foreign_item {
                        let sig = &func.sig;
                        if sig.variadic.is_some() {
                            continue;
                        }
                        let fn_name = &sig.ident;
                        let fn_name_str = fn_name.to_string();
                        let fn_name_nul = syn::LitByteStr::new(format!("{}\0", fn_name_str).as_bytes(), proc_macro2::Span::call_site());

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

        // Collect all struct/enum (ident, generics) for Send+Sync, and only the configured
        // status enum for Display/Error.
        let mut send_sync_items: Vec<(syn::Ident, syn::Generics)> = Vec::new();
        let mut error_items: Vec<(syn::Ident, syn::Generics)> = Vec::new();
        let status_type_ident = syn::Ident::new(self.config.status_type, proc_macro2::Span::call_site());
        for item in &new_sys_items {
            match item {
                syn::Item::Struct(s) => send_sync_items.push((s.ident.clone(), s.generics.clone())),
                syn::Item::Enum(e) => {
                    send_sync_items.push((e.ident.clone(), e.generics.clone()));
                    if e.ident == status_type_ident {
                        error_items.push((e.ident.clone(), e.generics.clone()));
                    }
                }
                _ => {}
            }
        }

        let send_sync_impls = send_sync_items.iter().map(|(ident, generics)| {
            // Build impl generics with Send/Sync bounds on each type param
            let where_clause = &generics.where_clause;
            // Collect just the idents/lifetimes for the "for Type<...>" part
            let ty_args: Vec<proc_macro2::TokenStream> = generics
                .params
                .iter()
                .map(|p| match p {
                    syn::GenericParam::Type(t) => {
                        let id = &t.ident;
                        quote!(#id)
                    }
                    syn::GenericParam::Lifetime(l) => {
                        let lt = &l.lifetime;
                        quote!(#lt)
                    }
                    syn::GenericParam::Const(c) => {
                        let id = &c.ident;
                        quote!(#id)
                    }
                })
                .collect();
            // Add Send/Sync bounds to type params in the impl generics
            let impl_params: Vec<proc_macro2::TokenStream> = generics
                .params
                .iter()
                .map(|p| match p {
                    syn::GenericParam::Type(t) => {
                        let id = &t.ident;
                        let bounds = &t.bounds;
                        if bounds.is_empty() { quote!(#id: Send + Sync) } else { quote!(#id: #bounds + Send + Sync) }
                    }
                    other => quote!(#other),
                })
                .collect();

            if ty_args.is_empty() {
                quote! {
                    unsafe impl Send for #ident #where_clause {}
                    unsafe impl Sync for #ident #where_clause {}
                }
            } else {
                quote! {
                    unsafe impl<#(#impl_params),*> Send for #ident<#(#ty_args),*> #where_clause {}
                    unsafe impl<#(#impl_params),*> Sync for #ident<#(#ty_args),*> #where_clause {}
                }
            }
        });

        if !dynamic_fields.is_empty() {
            let dyn_mod = quote! {
                #[cfg(feature = "runtime-link")]
                pub struct DynamicBindings {
                    #(#dynamic_fields),*
                }

                #[cfg(feature = "runtime-link")]
                unsafe impl Send for DynamicBindings {}
                #[cfg(feature = "runtime-link")]
                unsafe impl Sync for DynamicBindings {}

                #[cfg(feature = "runtime-link")]
                pub static DYNAMIC_BINDINGS: std::sync::OnceLock<Box<DynamicBindings>> = std::sync::OnceLock::new();

                #(#dynamic_wrappers)*

                #[cfg(feature = "runtime-link")]
                pub unsafe fn load_dynamic_bindings(lib: *mut std::ffi::c_void, get_proc_addr: unsafe fn(*mut std::ffi::c_void, *const u8) -> *mut std::ffi::c_void) {
                    let bindings = unsafe {
                        Box::new(
                            DynamicBindings {
                                #(#dynamic_loaders),*
                            }
                        )
                    };
                    DYNAMIC_BINDINGS.set(bindings).ok();
                }
            };
            let dyn_file: syn::File = syn::parse2(dyn_mod).expect("Failed to parse dynamic wrapper items");
            new_sys_items.extend(dyn_file.items);
        }

        let send_sync_stream = quote! { #(#send_sync_impls)* };
        let send_sync_file: syn::File = syn::parse2(send_sync_stream).expect("Failed to parse Send+Sync impls");
        new_sys_items.extend(send_sync_file.items);

        let error_impls = error_items.iter().map(|(ident, generics)| {
            let where_clause = &generics.where_clause;
            let ty_args: Vec<proc_macro2::TokenStream> = generics
                .params
                .iter()
                .map(|p| match p {
                    syn::GenericParam::Type(t) => {
                        let id = &t.ident;
                        quote!(#id)
                    }
                    syn::GenericParam::Lifetime(l) => {
                        let lt = &l.lifetime;
                        quote!(#lt)
                    }
                    syn::GenericParam::Const(c) => {
                        let id = &c.ident;
                        quote!(#id)
                    }
                })
                .collect();
            let impl_params: Vec<proc_macro2::TokenStream> = generics
                .params
                .iter()
                .map(|p| match p {
                    syn::GenericParam::Type(t) => {
                        let id = &t.ident;
                        let bounds = &t.bounds;
                        if bounds.is_empty() { quote!(#id) } else { quote!(#id: #bounds) }
                    }
                    other => quote!(#other),
                })
                .collect();

            if ty_args.is_empty() {
                quote! {
                    impl std::fmt::Display for #ident #where_clause {
                        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                            write!(f, "{:?}", self)
                        }
                    }
                    impl std::error::Error for #ident #where_clause {}
                }
            } else {
                quote! {
                    impl<#(#impl_params),*> std::fmt::Display for #ident<#(#ty_args),*> #where_clause {
                        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                            write!(f, "{:?}", self)
                        }
                    }
                    impl<#(#impl_params),*> std::error::Error for #ident<#(#ty_args),*> #where_clause {}
                }
            }
        });
        let error_stream = quote! { #(#error_impls)* };
        let error_file: syn::File = syn::parse2(error_stream).expect("Failed to parse error impls");
        new_sys_items.extend(error_file.items);

        let new_ast = syn::File {
            shebang: ast.shebang.clone(),
            attrs: ast.attrs.clone(),
            items: new_sys_items,
        };
        let mut code = quote!(#new_ast).to_string();
        for import in &self.config.extra_imports {
            if import.contains("cudart") {
                let re = regex::Regex::new(r"pub use self :: (cuda[a-zA-Z0-9_]*)").unwrap();
                let replacement = format!("pub use {} :: sys :: $1", import);
                code = re.replace_all(&code, replacement.as_str()).to_string();
            }
        }

        std::fs::write(sys_path, code).unwrap();
    }

    fn map_ffi_type_to_rust(&self, ty: &syn::Type) -> (syn::Type, bool) {
        let ty_str = quote!(#ty).to_string().replace(" ", "");
        let new_ty_str = match ty_str.as_str() {
            "::std::os::raw::c_int" | "std::os::raw::c_int" => Some("i32"),
            "::std::os::raw::c_uint" | "std::os::raw::c_uint" => Some("u32"),
            "::std::os::raw::c_long" | "std::os::raw::c_long" => Some("i64"),
            "::std::os::raw::c_ulong" | "std::os::raw::c_ulong" => Some("u64"),
            "::std::os::raw::c_longlong" | "std::os::raw::c_longlong" => Some("i64"),
            "::std::os::raw::c_ulonglong" | "std::os::raw::c_ulonglong" => Some("u64"),
            "::std::os::raw::c_char" | "std::os::raw::c_char" => Some("i8"),
            "::std::os::raw::c_uchar" | "std::os::raw::c_uchar" => Some("u8"),
            "::std::os::raw::c_short" | "std::os::raw::c_short" => Some("i16"),
            "::std::os::raw::c_ushort" | "std::os::raw::c_ushort" => Some("u16"),
            _ => None,
        };

        if let Some(new_ty) = new_ty_str { (syn::parse_str(new_ty).unwrap(), true) } else { (ty.clone(), false) }
    }

    fn should_skip_function(&self, func: &syn::ForeignItemFn) -> bool {
        let name = func.sig.ident.to_string();
        self.blocklist_funcs.iter().any(|r| r.is_match(&name))
    }

    fn generate_function_wrapper(&self, func: &syn::ForeignItemFn) -> (proc_macro2::TokenStream, Option<String>) {
        let sig = &func.sig;
        let fn_name = &sig.ident;
        let fn_str = fn_name.to_string();
        let attrs = &func.attrs;

        let is_getter = (fn_str.contains("Get") || fn_str.contains("Create") || fn_str.contains("Plan") || fn_str.contains("Load")) && !fn_str.contains("String") && !fn_str.contains("Name") && !fn_str.contains("Vector") && !fn_str.contains("Matrix") && !fn_str.contains("Unload");

        if is_getter {
            if let Some(wrapper) = self.generate_getter_wrapper(func) {
                return (wrapper, None);
            }
        }

        let mut safe_inputs = Vec::new();
        let mut safe_inputs_generics = Vec::new();
        let mut call_args = Vec::new();
        let mut generic_idx = 0;
        let generic_letters = ["T", "U", "V", "W", "X", "Y", "Z", "A", "B", "C", "D", "E", "F"];

        for (_i, input) in sig.inputs.iter().enumerate() {
            if let syn::FnArg::Typed(pat_type) = input {
                let pat = &pat_type.pat;
                let ty = &pat_type.ty;
                let (mapped_ty, transformed) = self.map_ffi_type_to_rust(ty);
                let _ty_str = quote!(#ty).to_string().replace(" ", "");

                // Handle pointer arguments
                if let Type::Ptr(ptr_ty) = &**ty {
                    let inner_ty = &ptr_ty.elem;
                    let inner_ty_str = quote!(#inner_ty).to_string().replace(" ", "");

                    let mut is_handle = inner_ty_str.contains("Context") || inner_ty_str.contains("Stream_t") || inner_ty_str.contains("Stream") || inner_ty_str.contains("ctx") || inner_ty_str.contains("Device") || inner_ty_str.contains("CUstream_st");

                    for handle_check in &self.config.handle_types_regex {
                        if inner_ty_str.contains(handle_check) {
                            is_handle = true;
                            break;
                        }
                    }

                    if is_handle {
                        safe_inputs.push(quote!(#pat: #ty));
                        call_args.push(quote!(#pat));
                    } else if self.config.use_cuda_as_ptr {
                        let generic_ident = if generic_idx < generic_letters.len() {
                            syn::Ident::new(generic_letters[generic_idx], proc_macro2::Span::call_site())
                        } else {
                            syn::Ident::new(&format!("T{}", generic_idx), proc_macro2::Span::call_site())
                        };
                        generic_idx += 1;
                        safe_inputs_generics.push(quote!(#generic_ident: types::CudaAsPtr));

                        if ptr_ty.mutability.is_some() {
                            safe_inputs.push(quote!(mut #pat: #generic_ident));
                            call_args.push(quote!(#pat.as_mut_ptr() as *mut _));
                        } else {
                            safe_inputs.push(quote!(#pat: #generic_ident));
                            call_args.push(quote!(#pat.as_const_ptr() as *const _));
                        }
                    } else {
                        // No CudaAsPtr available — pass raw pointer directly
                        safe_inputs.push(quote!(#pat: #ty));
                        call_args.push(quote!(#pat));
                    }
                } else if transformed {
                    safe_inputs.push(quote!(#pat: #mapped_ty));
                    call_args.push(quote!(#pat as _));
                } else {
                    safe_inputs.push(quote!(#pat: #ty));
                    call_args.push(quote!(#pat));
                }
            }
        }

        let ret_ty_str = if let ReturnType::Type(_, ty) = &sig.output { quote!(#ty).to_string().replace(" ", "") } else { String::new() };
        let status_type = self.config.status_type;
        let has_status_ret = ret_ty_str == status_type || ret_ty_str == format!("{}_t", status_type);

        let status_type_ident = syn::Ident::new(self.config.status_type, proc_macro2::Span::call_site());
        let success_variant_ident = syn::Ident::new(self.config.success_variant, proc_macro2::Span::call_site());

        let generics_block = if safe_inputs_generics.is_empty() { quote!() } else { quote!(<#(#safe_inputs_generics),*>) };

        let safe_fn = if has_status_ret {
            quote! {
                #(#attrs)*
                pub unsafe fn #fn_name #generics_block (#(#safe_inputs),*) -> Result<(), crate::sys::#status_type_ident> {
                    let status = unsafe { crate::sys::#fn_name(#(#call_args),*) };
                    if status == crate::sys::#status_type_ident::#success_variant_ident {
                        Ok(())
                    } else {
                        Err(status)
                    }
                }
            }
        } else if let ReturnType::Type(_, ty) = &sig.output {
            let (mapped_ret_ty, transformed_ret) = self.map_ffi_type_to_rust(ty);
            let ret_expr = if transformed_ret {
                quote!((unsafe { crate::sys::#fn_name(#(#call_args),*) }) as #mapped_ret_ty)
            } else {
                quote!(unsafe { crate::sys::#fn_name(#(#call_args),*) })
            };

            quote! {
                #(#attrs)*
                pub unsafe fn #fn_name #generics_block (#(#safe_inputs),*) -> #mapped_ret_ty {
                    #ret_expr
                }
            }
        } else {
            quote! {
                #(#attrs)*
                pub unsafe fn #fn_name #generics_block (#(#safe_inputs),*) {
                    unsafe { crate::sys::#fn_name(#(#call_args),*) }
                }
            }
        };

        (safe_fn, None)
    }

    fn generate_getter_wrapper(&self, func: &syn::ForeignItemFn) -> Option<proc_macro2::TokenStream> {
        let sig = &func.sig;
        let fn_name = &sig.ident;
        let attrs = &func.attrs;

        let mut out_vars = Vec::new();
        let mut out_types = Vec::new();
        let mut out_dcls = Vec::new();
        let mut safe_inputs = Vec::new();
        let mut call_args = Vec::new();
        let mut has_output = false;

        let fn_str = fn_name.to_string();

        for (i, input) in sig.inputs.iter().enumerate() {
            if let syn::FnArg::Typed(pat_type) = input {
                let pat = &pat_type.pat;
                let ty = &pat_type.ty;
                let (mapped_ty, transformed) = self.map_ffi_type_to_rust(ty);
                let ty_str = quote!(#ty).to_string().replace(" ", "");

                let mut handled_as_output = false;
                if let Type::Ptr(ptr_ty) = &**ty {
                    let is_plan_or_create = fn_str.contains("Create") || fn_str.contains("Plan") || fn_str.contains("Load");
                    let is_get = fn_str.contains("Get");

                    if ptr_ty.mutability.is_some() && !ty_str.contains("c_void") && !quote!(#pat).to_string().contains("Array") {
                        if is_get || (is_plan_or_create && i == 0) {
                            handled_as_output = true;
                            has_output = true;
                        }
                    }

                    if handled_as_output {
                        let inner_ty = &ptr_ty.elem;
                        let (mapped_inner_ty, _) = self.map_ffi_type_to_rust(inner_ty);
                        let var_name = quote::format_ident!("out_{}", i);
                        out_dcls.push(quote! {
                            let mut #var_name: std::mem::MaybeUninit<#inner_ty> = std::mem::MaybeUninit::zeroed();
                        });
                        call_args.push(quote! { #var_name.as_mut_ptr() as *mut _ });
                        out_vars.push(quote! { #var_name.assume_init() as #mapped_inner_ty });
                        out_types.push(quote! { #mapped_inner_ty });
                    }
                }

                if !handled_as_output {
                    if transformed {
                        safe_inputs.push(quote!(#pat: #mapped_ty));
                        call_args.push(quote!(#pat as _));
                    } else {
                        safe_inputs.push(quote!(#pat: #ty));
                        call_args.push(quote!(#pat));
                    }
                }
            }
        }

        if !has_output {
            return None;
        }

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

        let status_type_ident = syn::Ident::new(self.config.status_type, proc_macro2::Span::call_site());
        let success_variant_ident = syn::Ident::new(self.config.success_variant, proc_macro2::Span::call_site());

        let (ret_expr, _ret_ty) = if let ReturnType::Type(_, ty) = &sig.output {
            let (mapped_ret_ty, transformed_ret) = self.map_ffi_type_to_rust(ty);
            if transformed_ret {
                (quote!((unsafe { crate::sys::#fn_name(#(#call_args),*) }) as #mapped_ret_ty), mapped_ret_ty)
            } else {
                (quote!(unsafe { crate::sys::#fn_name(#(#call_args),*) }), *ty.clone())
            }
        } else {
            (quote!(unsafe { crate::sys::#fn_name(#(#call_args),*) }), syn::parse_quote!(()))
        };

        Some(quote! {
            #(#attrs)*
            pub unsafe fn #fn_name(#(#safe_inputs),*) -> Result<#return_type, crate::sys::#status_type_ident> {
                #(#out_dcls)*
                let status = #ret_expr;
                if status as usize == crate::sys::#status_type_ident::#success_variant_ident as usize {
                    unsafe { Ok(#return_vals) }
                } else {
                    Err(unsafe { std::mem::transmute(status) })
                }
            }
        })
    }

    fn handle_specialized_cudart(&self, func: &syn::ForeignItemFn) -> Option<proc_macro2::TokenStream> {
        let sig = &func.sig;
        let fn_name = &sig.ident;
        let fn_str = fn_name.to_string();
        let attrs = &func.attrs;

        let status_type_ident = syn::Ident::new(self.config.status_type, proc_macro2::Span::call_site());
        let success_variant_ident = syn::Ident::new(self.config.success_variant, proc_macro2::Span::call_site());

        if fn_str.starts_with("cudaMalloc") {
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
                    let (mapped_ty, transformed) = self.map_ffi_type_to_rust(ty);
                    if i == 0 {
                        if let Type::Ptr(ptr_ty) = &**ty {
                            let inner_ty = &ptr_ty.elem;
                            output_type = quote!(#inner_ty);
                        }
                        m_call_args.push(quote!(&mut dev_ptr as *mut _ as *mut _));
                    } else if transformed {
                        m_safe_inputs.push(quote!(#pat: #mapped_ty));
                        m_call_args.push(quote!(#pat as _));
                    } else {
                        m_safe_inputs.push(quote!(#pat: #ty));
                        m_call_args.push(quote!(#pat));
                    }
                }
            }

            return Some(if is_array || is_3d_pitched {
                quote! {
                    #(#attrs)*
                    pub unsafe fn #fn_name(#(#m_safe_inputs),*) -> Result<#output_type, crate::sys::#status_type_ident> {
                        let mut dev_ptr: #output_type = unsafe { std::mem::zeroed() };
                        let status = unsafe { crate::sys::#fn_name(#(#m_call_args),*) };
                        if status == crate::sys::#status_type_ident::#success_variant_ident {
                            Ok(dev_ptr)
                        } else {
                            Err(status)
                        }
                    }
                }
            } else if is_host {
                quote! {
                    #(#attrs)*
                    pub unsafe fn #fn_name<T>(#(#m_safe_inputs),*) -> Result<*mut T, crate::sys::#status_type_ident> {
                        let mut dev_ptr = std::ptr::null_mut();
                        let status = unsafe { crate::sys::#fn_name(#(#m_call_args),*) };
                        if status == crate::sys::#status_type_ident::#success_variant_ident {
                            Ok(dev_ptr as *mut T)
                        } else {
                            Err(status)
                        }
                    }
                }
            } else {
                quote! {
                    #(#attrs)*
                    pub unsafe fn #fn_name<T>(#(#m_safe_inputs),*) -> Result<::cuda_libs_cudart::types::cuDeviceAllocation<T>, crate::sys::#status_type_ident> {
                        let mut dev_ptr = std::ptr::null_mut();
                        let status = unsafe { crate::sys::#fn_name(#(#m_call_args),*) };
                        if status == crate::sys::#status_type_ident::#success_variant_ident {
                            Ok(::cuda_libs_cudart::types::cuDeviceAllocation(dev_ptr as *mut T))
                        } else {
                            Err(status)
                        }
                    }
                }
            });
        }

        if fn_str.starts_with("cudaFree") {
            let mut f_safe_inputs = Vec::new();
            let mut f_call_args = Vec::new();
            let is_host = fn_str.contains("Host");
            let is_array_or_3d = fn_str.contains("Array") || fn_str.contains("Mipmapped");

            for (i, input) in sig.inputs.iter().enumerate() {
                if let syn::FnArg::Typed(pat_type) = input {
                    let pat = &pat_type.pat;
                    let ty = &pat_type.ty;
                    let (mapped_ty, transformed) = self.map_ffi_type_to_rust(ty);
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
                    } else if transformed {
                        f_safe_inputs.push(quote!(#pat: #mapped_ty));
                        f_call_args.push(quote!(#pat as _));
                    } else {
                        f_safe_inputs.push(quote!(#pat: #ty));
                        f_call_args.push(quote!(#pat));
                    }
                }
            }

            return Some(if is_array_or_3d {
                quote! {
                    #(#attrs)*
                    pub unsafe fn #fn_name(#(#f_safe_inputs),*) -> Result<(), crate::sys::#status_type_ident> {
                        let status = unsafe { crate::sys::#fn_name(#(#f_call_args),*) };
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
                    pub unsafe fn #fn_name<T>(#(#f_safe_inputs),*) -> Result<(), crate::sys::#status_type_ident> {
                        let status = unsafe { crate::sys::#fn_name(#(#f_call_args),*) };
                        if status == crate::sys::#status_type_ident::#success_variant_ident {
                            Ok(())
                        } else {
                            Err(status)
                        }
                    }
                }
            });
        }

        None
    }

    fn generate_builder_impl(&self, s: &syn::ItemStruct) -> Option<proc_macro2::TokenStream> {
        let ident = &s.ident;
        if let syn::Fields::Named(fields) = &s.fields {
            if !fields.named.is_empty() {
                let mut builder_fns = Vec::new();
                for field in &fields.named {
                    if matches!(field.vis, syn::Visibility::Public(_)) {
                        if let Some(f_name) = &field.ident {
                            let f_ty = &field.ty;
                            let (mapped_ty, transformed) = self.map_ffi_type_to_rust(f_ty);
                            let val_expr = if transformed { quote!(val as _) } else { quote!(val) };
                            builder_fns.push(quote! {
                                pub fn #f_name(mut self, val: #mapped_ty) -> Self {
                                    self.#f_name = #val_expr;
                                    self
                                }
                            });
                        }
                    }
                }
                if !builder_fns.is_empty() {
                    return Some(quote! {
                        #[cfg(feature = "runtime-link")]
                        impl crate::sys::#ident {
                            #(#builder_fns)*
                        }
                    });
                }
            }
        }
        None
    }

    fn write_safe_rs(&self, out_dir: &std::path::Path, standalone_funcs: Vec<proc_macro2::TokenStream>, builder_impls: Vec<proc_macro2::TokenStream>) {
        let mut extra_safes = Vec::new();
        let depends_on_cudart = self.config.lib_name == "cuda_libs_cudart" || self.config.extra_imports.iter().any(|i| i.contains("cudart"));
        if self.config.lib_name == "cuda_libs_cudart" {
            extra_safes.push(quote!(
                #[allow(unused_imports)]
                use crate::types;
            ));
        } else if depends_on_cudart {
            extra_safes.push(quote!(
                #[allow(unused_imports)]
                use cuda_libs_cudart::types;
            ));
        }
        for import in &self.config.extra_imports {
            let import_path: syn::Path = syn::parse_str(import).unwrap();
            extra_safes.push(quote! {
                use #import_path;
                #[allow(unused_imports)]
                use #import_path::sys::*;
            });
        }

        let status_type_ident = syn::Ident::new(self.config.status_type, proc_macro2::Span::call_site());

        let extra_safe = if !self.config.extra_safe_code.is_empty() {
            syn::parse_str::<proc_macro2::TokenStream>(self.config.extra_safe_code).unwrap()
        } else {
            quote! {}
        };

        let safe_module = quote! {
            pub use crate::sys::#status_type_ident as CudaTargetStatus;
            #[allow(unused_imports)]
            use crate::sys::*;
            #(#extra_safes)*

            #(#builder_impls)*
            #(#standalone_funcs)*
            #extra_safe
        };

        let safe_path = out_dir.join("safe.rs");
        std::fs::write(&safe_path, safe_module.to_string()).unwrap();

        std::process::Command::new("cargo").arg("fmt").status().ok();
    }
}
