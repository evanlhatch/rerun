//! Shim: derive macro for SizeBytes.

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[proc_macro_derive(SizeBytes, attributes(size_bytes))]
pub fn derive_size_bytes(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let expanded = quote! {
        impl re_byte_size::SizeBytes for #name {}
    };
    TokenStream::from(expanded)
}
