mod db;
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

// DB Related
#[proc_macro_derive(Row)]
pub fn row(input: TokenStream) -> TokenStream {
    db::derive_row(input.into()).into()
}

#[proc_macro_derive(Table, attributes(meta, row))]
pub fn table(input: TokenStream) -> TokenStream {
    db::derive_table(input.into()).into()
}
