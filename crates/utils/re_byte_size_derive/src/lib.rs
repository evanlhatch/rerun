//! Shim: derive macro for SizeBytes. Handles generics and where clauses.

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(SizeBytes, attributes(size_bytes))]
pub fn derive_size_bytes(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // Add T: SizeBytes bound to each generic type parameter.
    let generics = &input.generics;
    let type_params = generics.type_params().map(|tp| tp.ident.clone());
    let bounds = type_params.map(|t| {
        quote! { #t: re_byte_size::SizeBytes }
    });

    let expanded = if bounds.len() == 0 {
        quote! {
            impl #impl_generics re_byte_size::SizeBytes for #name #ty_generics #where_clause {}
        }
    } else {
        quote! {
            impl #impl_generics re_byte_size::SizeBytes for #name #ty_generics
            where
                #(#bounds,)*
                #where_clause
            {}
        }
    };

    TokenStream::from(expanded)
}
