use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, DeriveInput, Ident, LitStr};

struct ModuleAttr<'a> {
    name: &'a Ident,
    route: Option<String>,
}

// TODO: Add get, post, put, delete, db
pub fn derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse2(input).unwrap();
    let name = &ast.ident;

    let mut module_attr = ModuleAttr {
        name: name,
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
    let name = module_attr.name;
    let route = module_attr.route.unwrap_or("/".to_string());

    quote! {
        use lan_frame::axum::Router;

        impl Module for #name {
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
