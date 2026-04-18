extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use std::process::Command;
use std::{env, fs};
use syn::visit_mut::{self, VisitMut};
use syn::{ItemFn, parse_macro_input};

// A visitor that extracts shared statics and replaces them with pointer lookups
struct SharedStaticExtractor {
    global_asm_declarations: Vec<String>,
}

impl SharedStaticExtractor {
    fn get_type_info(ty: &syn::Type) -> Option<(usize, usize)> {
        match ty {
            syn::Type::Array(array) => {
                let (elem_size, _) = Self::get_type_info(&array.elem)?;
                if let syn::Expr::Lit(ref lit) = array.len {
                    if let syn::Lit::Int(ref int) = lit.lit {
                        let len: usize = int.base10_parse().ok()?;
                        return Some((elem_size * len, len));
                    }
                }
                None
            }
            syn::Type::Path(path) => {
                let segment = path.path.segments.last()?;
                let size = match segment.ident.to_string().as_str() {
                    "u8" | "i8" => Some(1),
                    "u16" | "i16" => Some(2),
                    "u32" | "i32" | "f32" => Some(4),
                    "u64" | "i64" | "f64" | "usize" | "isize" => Some(8),
                    _ => None,
                }?;
                Some((size, 1))
            }
            _ => None,
        }
    }
}

impl VisitMut for SharedStaticExtractor {
    fn visit_block_mut(&mut self, i: &mut syn::Block) {
        let mut new_stmts = Vec::new();
        for mut stmt in i.stmts.drain(..) {
            let mut extracted = false;
            if let syn::Stmt::Item(syn::Item::Static(ref mut item_static)) = stmt {
                let is_shared = item_static
                    .attrs
                    .iter()
                    .any(|attr| if let syn::Meta::NameValue(ref nv) = attr.meta { nv.path.is_ident("link_section") && format!("{:?}", nv.value).contains(".shared") } else { false });

                if is_shared {
                    let original_name = item_static.ident.clone();
                    let internal_name = format_ident!("__shmem_{}", original_name);

                    let ty = &*item_static.ty;
                    let (size, len) = Self::get_type_info(ty).unwrap_or((0, 0));

                    let elem_ty = if let syn::Type::Array(array) = ty { &*array.elem } else { ty };

                    // Generate global_asm! declaration
                    let decl = format!(".visible .shared .align 16 .b8 {}[{}];", internal_name, size);
                    self.global_asm_declarations.push(decl);

                    // Replace with a slice initialization to the element type
                    let replacement: syn::Stmt = syn::parse_quote! {
                        let #original_name = unsafe {
                            let ptr: *mut #elem_ty;
                            let mut _tmp: u64;
                            ::core::arch::asm!(
                                concat!("mov.u64 {tmp}, ", stringify!(#internal_name), ";"),
                                "cvta.shared.u64 {ptr}, {tmp};",
                                tmp = out(reg64) _tmp,
                                ptr = out(reg64) ptr,
                            );
                            ::core::slice::from_raw_parts_mut(ptr, #len)
                        };
                    };
                    new_stmts.push(replacement);
                    extracted = true;
                }
            }
            if !extracted {
                // Continue visiting nested blocks
                visit_mut::visit_stmt_mut(self, &mut stmt);
                new_stmts.push(stmt);
            }
        }
        i.stmts = new_stmts;
    }
}

#[proc_macro_attribute]
pub fn cuda_load(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let vis = &input_fn.vis;
    let sig = &input_fn.sig;
    let block = &input_fn.block;

    let init_code = quote! {
        ::cuda_libs::runtime_link_load();
    };

    let expanded = quote! {
        #vis #sig {
            #init_code
            #block
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn global(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut host_fn = parse_macro_input!(item as ItemFn);
    let original_name = host_fn.sig.ident.clone();

    let mut device_fn = host_fn.clone();

    // Hoist shared statics
    let mut extractor = SharedStaticExtractor { global_asm_declarations: Vec::new() };
    visit_mut::visit_item_fn_mut(&mut extractor, &mut device_fn);

    device_fn.vis = syn::Visibility::Public(syn::parse_quote!(pub));
    device_fn.sig.abi = Some(syn::parse_quote!(extern "ptx-kernel"));
    device_fn.attrs.push(syn::parse_quote!(#[no_mangle]));

    let gasm_decls = &extractor.global_asm_declarations;
    let fn_source = quote!(#device_fn).to_string();

    let hidden_name = format_ident!("__{}", original_name);
    host_fn.sig.ident = hidden_name.clone();

    let compile_source = format!(
        "#![no_std]\n\
         #![feature(asm_experimental_arch)]\n\
         #![feature(abi_ptx, stdarch_nvptx)]\n\
         #![crate_type = \"cdylib\"]\n\n\
         #[panic_handler]\n\
         fn panic(_info: &core::panic::PanicInfo) -> ! {{ unsafe {{ ::core::hint::unreachable_unchecked() }} }}\n\n\
         ::core::arch::global_asm!(\".extern .shared .align 16 .b8 __dynamic_smem[];\");\n\n\
         {}\n\n\
         macro_rules! shared {{\n\
             ($t:ty) => {{\n\
                 unsafe {{\n\
                     let ptr: *mut $t;\n\
                     let mut _tmp: u64;\n\
                     ::core::arch::asm!(\n\
                         \"    mov.u64 {{tmp}}, __dynamic_smem;\",\n\
                         \"    cvta.shared.u64 {{ptr}}, {{tmp}};\",\n\
                         tmp = out(reg64) _tmp,\n\
                         ptr = out(reg64) ptr,\n\
                     );\n\
                     ptr\n\
                 }}\n\
             }};\n\
         }}\n\n\
         {}",
        gasm_decls.iter().map(|d| format!("::core::arch::global_asm!(\"{}\");", d)).collect::<Vec<_>>().join("\n"),
        fn_source
    );

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is not set");

    let cache_dir = std::path::PathBuf::from(manifest_dir).join("target").join("ptx_cache").join(&original_name.to_string());

    let src_dir = cache_dir.join("src");
    fs::create_dir_all(&src_dir).expect("Failed to create isolated kernel cache directory");

    let cargo_toml = format!(
        r#"
        [package]
        name = "{}_ptx"
        version = "0.1.0"
        edition = "2021"

        [lib]
        crate-type = ["cdylib"]

        [workspace]
        "#,
        original_name
    );

    fs::write(cache_dir.join("Cargo.toml"), cargo_toml).expect("Failed to write Cargo.toml");

    let rs_file_path = src_dir.join("lib.rs");
    let existing_source = fs::read_to_string(&rs_file_path).unwrap_or_default();

    if existing_source != compile_source {
        fs::write(&rs_file_path, compile_source).expect("Failed to write kernel source");
    }

    let output = Command::new("cargo")
        .current_dir(&cache_dir)
        .arg("+nightly")
        .arg("build")
        .arg("--target")
        .arg("nvptx64-nvidia-cuda")
        .arg("-Z")
        .arg("build-std=core")
        .arg("--release")
        .env("RUSTFLAGS", "-C target-cpu=sm_80")
        .output()
        .expect("Failed to execute cargo sub-process.");

    if !output.status.success() {
        let stderr_msg = String::from_utf8_lossy(&output.stderr);
        panic!("Failed to compile kernel `{}` to PTX.\n\n--- CARGO ERROR ---\n{}", original_name, stderr_msg);
    }

    let ptx_file = cache_dir.join("target").join("nvptx64-nvidia-cuda").join("release").join(format!("{}_ptx.ptx", original_name));
    let ptx_data = format!("{}\0", fs::read_to_string(&ptx_file).expect("Failed to read generated PTX file from cache"));

    let ptx_const_name = format_ident!("__PTX_{}", original_name.to_string().to_uppercase());
    let macro_name = original_name.clone();
    let module_name_str = original_name.to_string();

    let expanded = quote! {
        #[doc(hidden)]
        pub const #ptx_const_name: &str = #ptx_data;

        #[allow(non_local_definitions)]
        #[macro_export]
        macro_rules! #macro_name {
            // 1. Entry Point
            ( <<< $($tail:tt)* ) => {
                #macro_name!(@munch_config () ; $($tail)*)
            };

            // 2. Muncher: Grab tokens until >>>
            (@munch_config ($($config:tt)*) ; >>> ($($arg:expr),*)) => {{
                let ptx_ptr = #ptx_const_name.as_ptr() as *const ::core::ffi::c_char;
                let func = ::cuda_libs::driver::types::DEFAULT_DEVICE.try_get_function(
                    #module_name_str,
                    ptx_ptr
                ).expect("failed to load ptx module, something went really really wrong");
                #macro_name!(@internal_config func, $($config)* ; $($arg),*)
            }};

            (@munch_config ($($head:tt)*) ; $next:tt $($tail:tt)*) => {
                #macro_name!(@munch_config ($($head)* $next) ; $($tail)*)
            };

            // 3. Dispatcher
            (@internal_config $f:expr, $grid:expr, $block:expr, $shared:expr, $stream:expr ; $($arg:expr),*) => {
                #macro_name!(@internal_launch $f, $grid, $block, $shared, $stream ; $($arg),*)
            };
            (@internal_config $f:expr, $grid:expr, $block:expr, $shared:expr ; $($arg:expr),*) => {
                #macro_name!(@internal_launch $f, $grid, $block, $shared, ::core::ptr::null_mut() ; $($arg),*)
            };
            (@internal_config $f:expr, $grid:expr, $block:expr ; $($arg:expr),*) => {
                #macro_name!(@internal_launch $f, $grid, $block, 0, ::core::ptr::null_mut() ; $($arg),*)
            };

            // 4. Launcher: The "No-Iterator" Pointer Packing
            (@internal_launch $f:expr, $grid:expr, $block:expr, $shared:expr, $stream:expr ; $($arg:expr),*) => {{
                // Build simple array of pointers
                let mut kernel_params = [
                    $( (&$arg as *const _ as *mut ::core::ffi::c_void) ),*
                ];

                #[allow(unused_unsafe)]
                unsafe {
                    ::cuda_libs::driver::safe::cuLaunchKernel(
                        $f,
                        $grid as u32, 1, 1,
                        $block as u32, 1, 1,
                        $shared as u32,
                        $stream,
                        kernel_params.as_mut_ptr(),
                        ::core::ptr::null_mut()
                    ).expect("failed to launch kernel");
                }
            }};
        }
    };

    TokenStream::from(expanded)
}
