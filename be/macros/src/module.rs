use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Ident, ItemTrait, TraitItem, TraitItemFn, parse2};

pub fn derive_module(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse2(input).unwrap();
    let module = &ast.ident;
    let name = &module.to_string();
    quote!(
        impl lan_be_frame::module::Module for #module {
            fn _name(&self) -> &str {
                #name
            }

            fn _router(&self) -> lan_be_frame::axum::Router {
                let mut router = lan_be_frame::axum::Router::new();
                for route in self._get_all_routes() {
                    router = router.route(&route.0, route.1);
                }
                router
            }
        }
    )
}

pub fn derive_interface(input: TokenStream) -> TokenStream {
    let ast: ItemTrait = parse2(input).unwrap();
    let module = &ast.ident;
    let parsed_fns = parse_fns(&ast);
    let (route_tokens, internal_fn_tokens): (Vec<TokenStream>, Vec<TokenStream>) = parsed_fns
        .into_iter()
        .map(|parsed_fn| (parsed_fn.route_tokens, parsed_fn.internal_fn_tokens))
        .unzip();

    quote! {
        use lan_be_frame::module::Interface;

        impl Interface for #module {
            fn _get_all_routes(&self) -> Vec<(&str, lan_be_frame::axum::routing::MethodRouter)> {
                vec![#(#route_tokens),*]
            }
        }

        impl #module{
            #(#internal_fn_tokens)*
        }
    }
}

struct ParsedFn {
    route_tokens: TokenStream,
    internal_fn_tokens: TokenStream,
}

fn parse_fns(item_trait: &ItemTrait) -> Vec<ParsedFn> {
    item_trait
        .items
        .iter()
        .map(|item| {
            if let TraitItem::Fn(trait_fn) = item {
                parse_fn(&trait_fn)
            } else {
                panic!("Unknown Token Found in Trait Fn: {:?}", item);
            }
        })
        .collect()
}

fn parse_fn(trait_fn: &TraitItemFn) -> ParsedFn {
    let fn_name = &trait_fn.sig.ident;
    let Some(attr) = trait_fn.attrs.first() else {
        panic!("Method attribute is missing for fn: {}", fn_name);
    };
    let prefix = match attr.path().get_ident() {
        Some(ident) if ident == "get" => {
            quote!(lan_be_frame::axum::routing::get)
        }
        Some(ident) if ident == "post" => {
            quote!(lan_be_frame::axum::routing::post)
        }
        Some(ident) if ident == "put" => {
            quote!(lan_be_frame::axum::routing::put)
        }
        Some(ident) if ident == "delete" => {
            quote!(lan_be_frame::axum::routing::delete)
        }
        _ => {
            panic!("Unknown Method attribute for fn: {}", fn_name);
        }
    };
    let Ok(route) = attr.parse_args::<syn::LitStr>() else {
        panic!("route must be string literal for fn: {}", fn_name);
    };

    let internal_name = format!("_{}", fn_name);
    let internal_fn = Ident::new(&internal_name, fn_name.span());
    let route_str = route.value();

    // TODO: update handler and add a transformer in the internal fn transform handler into internal fn
    ParsedFn {
        route_tokens: quote! {(#route_str,#prefix(Self::#internal_fn))},
        internal_fn_tokens: quote! {
            async fn #internal_fn() -> &'static str {
                "Hello World!"
            }
        },
    }
}
