mod module;

use proc_macro::TokenStream;

// Module Related
#[proc_macro_derive(Module, attributes(route))]
pub fn module_derive(input: TokenStream) -> TokenStream {
    module::derive(input.into()).into()
}

#[proc_macro_attribute]
pub fn get(args: TokenStream, input: TokenStream) -> TokenStream {
    module::get(args.into(), input.into()).into()
}

#[proc_macro_attribute]
pub fn post(args: TokenStream, input: TokenStream) -> TokenStream {
    module::post(args.into(), input.into()).into()
}

#[proc_macro_attribute]
pub fn put(args: TokenStream, input: TokenStream) -> TokenStream {
    module::put(args.into(), input.into()).into()
}

#[proc_macro_attribute]
pub fn delete(args: TokenStream, input: TokenStream) -> TokenStream {
    module::delete(args.into(), input.into()).into()
}

// TODO: DB Related
