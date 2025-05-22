use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    Attribute, DeriveInput, Expr, ExprPath, Ident, Lit, LitStr, Token, punctuated::Punctuated,
};

struct ModuleAttr<'a> {
    module: &'a Ident,
    route: Option<String>,
    method_handlers: Vec<MethodHandler<'a>>,
}

struct MethodHandler<'a> {
    method: &'a Ident,
    route: LitStr,
    handler: ExprPath,
}

// TODO: Add db helper
pub fn derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse2(input).unwrap();
    let module = &ast.ident;

    let mut module_attr = ModuleAttr {
        module,
        route: None,
        method_handlers: Vec::new(),
    };

    for attr in ast.attrs.iter() {
        if attr.path().is_ident("route") {
            module_attr.route = Some(parse_route(attr));
            continue;
        }

        if let Some(ident) = attr.path().get_ident() {
            if ["get", "post", "put", "delete"].contains(&ident.to_string().as_str()) {
                module_attr.method_handlers.push(parse_method(attr, &ident));
            }
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

fn parse_method<'a>(attr: &Attribute, method: &'a Ident) -> MethodHandler<'a> {
    let punctuated = attr
        .parse_args_with(Punctuated::<Expr, Token![,]>::parse_terminated)
        .unwrap();

    let mut punctuated_iter = punctuated.into_iter();
    let Some(route_expr) = punctuated_iter.next() else {
        panic!("route is missing")
    };
    let Expr::Lit(route_lit) = route_expr else {
        panic!("route must be literal")
    };
    let Lit::Str(route) = route_lit.lit else {
        panic!("route must be string literal")
    };

    let Some(handler_expr) = punctuated_iter.next() else {
        panic!("handler is missing")
    };
    let Expr::Path(handler) = handler_expr else {
        panic!("handler must be path")
    };

    MethodHandler {
        method,
        route,
        handler,
    }
}

// TODO: Update method handler here
fn parse_attributes(module_attr: ModuleAttr) -> TokenStream {
    let module = module_attr.module;
    let name = &module.to_string();
    let route = module_attr.route.unwrap_or("/".to_string());

    quote! {
        impl Module for #module {
            fn name(&self) -> &str{
                #name
            }

            fn route(&self) -> &str {
                #route
            }

            fn router(&self) -> lan_frame::axum::Router {
                lan_frame::axum::Router::new()
            }
        }
    }
}
