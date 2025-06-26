use proc_macro2::{TokenStream, TokenTree, token_stream::IntoIter};
use quote::quote;
use syn::Ident;

pub fn condition(input: TokenStream) -> TokenStream {
    let mut input_iter = input.into_iter();
    let Some(entity_path_tt) = input_iter.next() else {
        panic!("Missing db entity path");
    };
    let TokenTree::Ident(entity_path) = entity_path_tt else {
        panic!("db entity path must be an ident");
    };
    let conditions = parse_conditions(&mut input_iter, &entity_path);
    quote! {
        use sea_orm::ColumnTrait;
        #conditions
    }
}

fn parse_conditions(input_iter: &mut IntoIter, entity_path: &Ident) -> TokenStream {
    if let Some(TokenTree::Punct(punct)) = input_iter.next() {
        if punct.as_char() != ',' {
            panic!("Expected comma after entity name");
        }
    } else {
        panic!("Missing conditions");
    }

    let mut outter_condition = quote!(sea_orm::Condition::any()); // Or Conditions
    let mut inner_condition = quote!(sea_orm::Condition::all()); // and conditions
    quote!()
}

fn parse_condition(input_iter: &mut IntoIter, entity_path: &Ident) -> Option<TokenStream> {
    todo!()
}
