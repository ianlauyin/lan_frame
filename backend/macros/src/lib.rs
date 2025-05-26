mod http;
mod module;

use proc_macro::TokenStream;

// Module related
#[proc_macro_attribute]
pub fn module(_: TokenStream, input: TokenStream) -> TokenStream {
    module::derive_module(input.into()).into()
}

#[proc_macro_attribute]
pub fn interface(_: TokenStream, input: TokenStream) -> TokenStream {
    module::derive_interface(input.into()).into()
}

// HTTP related
#[proc_macro_attribute]
pub fn get(args: TokenStream, input: TokenStream) -> TokenStream {
    http::derive_get(args.into(), input.into()).into()
}

#[proc_macro_attribute]
pub fn post(args: TokenStream, input: TokenStream) -> TokenStream {
    http::derive_post(args.into(), input.into()).into()
}

#[proc_macro_attribute]
pub fn put(args: TokenStream, input: TokenStream) -> TokenStream {
    http::derive_put(args.into(), input.into()).into()
}

#[proc_macro_attribute]
pub fn delete(args: TokenStream, input: TokenStream) -> TokenStream {
    http::derive_delete(args.into(), input.into()).into()
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
