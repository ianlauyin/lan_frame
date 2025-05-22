mod module;

use proc_macro::TokenStream;

#[proc_macro_derive(Module, attributes(route, get, post, put, delete))]
pub fn module_derive(input: TokenStream) -> TokenStream {
    module::derive(input.into()).into()
}

// TODO: DB Related
