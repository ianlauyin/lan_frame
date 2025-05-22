use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, DeriveInput, Ident, LitStr};

struct ModuleAttr<'a> {
    module: &'a Ident,
    route: Option<String>,
}

// TODO: Add get, post, put, delete, db
pub fn derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse2(input).unwrap();
    let module = &ast.ident;

    let mut module_attr = ModuleAttr {
        module,
        route: None,
    };

    for attr in ast.attrs.iter() {
        if attr.path().is_ident("route") {
            module_attr.route = Some(parse_route(attr));
        }
    }

    parse_attributes(module_attr)
}

fn parse_route(attr: &Attribute) -> String {
    let Ok(lit) = attr.parse_args::<LitStr>() else {
        panic!("route must be string")
    };
    lit.value()
}

fn parse_attributes(module_attr: ModuleAttr) -> TokenStream {
    let module = module_attr.module;
    let name = &module.to_string();
    let route = module_attr.route.unwrap_or("/".to_string());

    quote! {
        use lan_frame::axum::Router;

        impl Module for #module {
            fn name(&self) -> &str{
                #name
            }

            fn route(&self) -> &str {
                #route
            }

            fn router(&self) -> Router {
                Router::new()
            }
        }
    }
}
