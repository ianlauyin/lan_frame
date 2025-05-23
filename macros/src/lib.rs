mod http;
mod module;

use proc_macro::TokenStream;

#[proc_macro_derive(Module, attributes(route, get, post, put, delete))]
pub fn module_derive(input: TokenStream) -> TokenStream {
    module::derive(input.into()).into()
}

#[proc_macro_attribute]
pub fn request(_: TokenStream, input: TokenStream) -> TokenStream {
    http::derive_request(input.into()).into()
}

#[proc_macro_attribute]
pub fn response(_: TokenStream, input: TokenStream) -> TokenStream {
    http::derive_response(input.into()).into()
}

// TODO: DB Related
