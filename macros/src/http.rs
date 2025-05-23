use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{DeriveInput, parse2};

pub fn derive_get(args: TokenStream, input: TokenStream) -> TokenStream {
    todo!()
}

pub fn derive_post(args: TokenStream, input: TokenStream) -> TokenStream {
    todo!()
}

pub fn derive_put(args: TokenStream, input: TokenStream) -> TokenStream {
    todo!()
}

pub fn derive_delete(args: TokenStream, input: TokenStream) -> TokenStream {
    todo!()
}

pub fn derive_request(input: TokenStream) -> TokenStream {
    let mut ast: DeriveInput = parse2(input).unwrap();
    let wrapper_ident = ast.ident.clone();
    ast.ident = Ident::new(&format!("_{}", wrapper_ident), Span::call_site());
    let inner_ident = ast.ident.clone();

    quote! {
        pub type #wrapper_ident = Json<#inner_ident>;

        #[derive(serde::Deserialize)]
        #ast
    }
}

pub fn derive_response(input: TokenStream) -> TokenStream {
    let mut ast: DeriveInput = parse2(input).unwrap();
    let wrapper_ident = ast.ident.clone();
    ast.ident = Ident::new(&format!("_{}", wrapper_ident), Span::call_site());
    let inner_ident = ast.ident.clone();

    quote! {
        pub type #wrapper_ident = Json<#inner_ident>;

        #[derive(serde::Serialize)]
        #ast
    }
}
