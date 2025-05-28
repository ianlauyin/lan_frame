mod http;
mod module;

use proc_macro::TokenStream;

// Module related
#[proc_macro_derive(Module)]
pub fn module(input: TokenStream) -> TokenStream {
    module::derive_module(input.into()).into()
}

#[proc_macro_attribute]
pub fn interface(_: TokenStream, input: TokenStream) -> TokenStream {
    module::derive_interface(input.into()).into()
}

// HTTP related
#[proc_macro_derive(Request)]
pub fn request(input: TokenStream) -> TokenStream {
    http::derive_request(input.into()).into()
}

#[proc_macro_derive(Response)]
pub fn response(input: TokenStream) -> TokenStream {
    http::derive_response(input.into()).into()
}

// TODO: DB Related
