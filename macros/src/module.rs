use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::{
    DeriveInput, Ident, ItemImpl, ItemTrait, LitStr, Token, TraitItem, TraitItemFn, Type, parse2,
};

pub fn derive_module(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse2(input).unwrap();
    let module = &ast.ident;
    let name = &module.to_string();
    quote!(
        impl lan_be_frame::module::Module for #module {
            fn name(&self) -> &str {
                #name
            }
        }
    )
}

pub fn derive_interface(input: TokenStream) -> TokenStream {
    let ast: ItemTrait = parse2(input).unwrap();
    let trait_item_fns = trait_item_fns(&ast);
    let interface_tokens = interface_tokens(&trait_item_fns, &ast.ident);
    let handler_tokens = handler_tokens(&trait_item_fns, &ast.ident);

    quote! {
        #interface_tokens
        #handler_tokens
    }
}

pub fn derive_handler(input: TokenStream) -> TokenStream {
    let mut ast: ItemImpl = parse2(input).unwrap();
    let Type::Path(module_type_path) = ast.self_ty.as_ref() else {
        panic!("Handler must impl a module");
    };
    let mut handler_path = module_type_path.path.clone();
    let Some(last_segment) = handler_path.segments.last_mut() else {
        panic!("Handler must have a module name");
    };
    last_segment.ident = Ident::new(
        &format!("{}Handler", last_segment.ident),
        last_segment.ident.span(),
    );
    ast.trait_ = Some((None, handler_path, Token![for](Span::call_site())));
    ast.to_token_stream()
}

fn trait_item_fns(item_trait: &ItemTrait) -> Vec<&TraitItemFn> {
    item_trait
        .items
        .iter()
        .map(|item| {
            if let TraitItem::Fn(trait_fn) = item {
                trait_fn
            } else {
                panic!(
                    "Unknown Token Found in Trait Fn {:?}",
                    item.to_token_stream()
                );
            }
        })
        .collect()
}

fn interface_tokens(trait_fns: &Vec<&TraitItemFn>, module: &Ident) -> TokenStream {
    let all_route_tokens: Vec<TokenStream> = trait_fns
        .iter()
        .map(|trait_fn| route_tokens(trait_fn))
        .collect();

    quote! {
        use lan_be_frame::module::Interface;
        impl Interface for #module {
            fn _get_all_routes(&self) -> Vec<(&str, lan_be_frame::axum::routing::MethodRouter)> {
                vec![#(#all_route_tokens),*]
            }
        }
    }
}

fn route_tokens(trait_fn: &TraitItemFn) -> TokenStream {
    let fn_name = &trait_fn.sig.ident;
    let attr = trait_fn
        .attrs
        .first()
        .expect(&format!("Method attribute is missing for fn: {}", fn_name));
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
    let route: LitStr = attr
        .parse_args()
        .expect(&format!("route must be string literal for fn: {}", fn_name));

    let route_str = route.value();

    quote! {(#route_str,#prefix(Self::#fn_name))}
}

fn handler_tokens(trait_fns: &Vec<&TraitItemFn>, module: &Ident) -> TokenStream {
    let handler = Ident::new(&format!("{}Handler", module), module.span());
    let handler_tokens: Vec<TokenStream> = trait_fns
        .iter()
        .map(|trait_fn| trait_fn.sig.to_token_stream())
        .collect();

    quote! {
         trait #handler {
            #(#handler_tokens);*;
        }
    }
}
