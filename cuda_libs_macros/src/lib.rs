extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

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
