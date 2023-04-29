// lib.rs
extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(ImplAddIntegers)]
pub fn impl_add_integers(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl #name {
            pub fn add_integers(a: i32, b: i32) -> i32 {
                a + b
            }
        }
    };

    TokenStream::from(expanded)
}
