use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse2(input).unwrap();
    let name = &ast.ident;

    quote! {
        use lan_frame::axum;

        impl Module for #name {
            fn router(&self) -> axum::Router {
                axum::Router::new()
            }
        }
    }
}

pub fn get(args: TokenStream, input: TokenStream) -> TokenStream {
    input
}

pub fn post(args: TokenStream, input: TokenStream) -> TokenStream {
    input
}

pub fn put(args: TokenStream, input: TokenStream) -> TokenStream {
    input
}

pub fn delete(args: TokenStream, input: TokenStream) -> TokenStream {
    input
}
