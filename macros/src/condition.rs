use proc_macro2::{TokenStream, TokenTree, token_stream::IntoIter};
use quote::{ToTokens, quote};
use syn::Ident;

pub fn condition(input: TokenStream) -> TokenStream {
    let mut input_iter = input.into_iter();
    let Some(entity_path_tt) = input_iter.next() else {
        panic!("Missing db entity path");
    };
    let TokenTree::Ident(entity_path) = entity_path_tt else {
        panic!("db entity path must be an ident");
    };
    let Some(TokenTree::Punct(punct)) = input_iter.next() else {
        panic!("Missing conditions");
    };
    if punct.as_char() != ',' {
        panic!("Expected ',' after entity path");
    }
    let all_conditions = parse_remaining(&mut input_iter, &entity_path);
    quote! {
        {
            use sea_orm::ColumnTrait;
            #all_conditions
        }
    }
}

fn parse_remaining(input_iter: &mut IntoIter, entity_path: &Ident) -> TokenStream {
    let mut all_conditions = quote!();
    while let Some(tt) = input_iter.next() {
        if let TokenTree::Group(group) = tt {
            // TODO: handle branklet
            continue;
        }

        let (condition, next_token) = parse_condition(tt, input_iter, entity_path);
        all_conditions = condition;
    }
    if all_conditions.is_empty() {
        panic!("Missing conditions");
    }
    all_conditions
}

enum NextToken {
    AND,
    OR,
    NONE,
}

fn parse_condition(
    first_tt: TokenTree,
    input_iter: &mut IntoIter,
    entity_path: &Ident,
) -> (TokenStream, NextToken) {
    let next_token: NextToken;
    let column_name = parse_column_name(first_tt);
    let operator_token = parse_operator(input_iter);
    let first_value_tt = input_iter.next().expect("Missing value");
    let mut values_tt = vec![first_value_tt];
    loop {
        let value_tt = input_iter.next();
        match value_tt {
            Some(TokenTree::Ident(ident)) if ident.to_string() == "AND" => {
                next_token = NextToken::AND;
                break;
            }
            Some(TokenTree::Ident(ident)) if ident.to_string() == "OR" => {
                next_token = NextToken::OR;
                break;
            }
            None => {
                next_token = NextToken::NONE;
                break;
            }
            Some(value_tt) => values_tt.push(value_tt),
        }
    }

    let values_stream = TokenStream::from_iter(values_tt);
    (
        quote! {
            #entity_path::Column::#column_name #operator_token(#values_stream)
        },
        next_token,
    )
}

fn parse_column_name(tt: TokenTree) -> Ident {
    let TokenTree::Ident(column_name) = tt else {
        panic!("Column name must be an ident: {:?}", tt);
    };
    Ident::new(
        &snake_to_camel(&column_name.to_string()),
        column_name.span(),
    )
}

fn parse_operator(input_iter: &mut IntoIter) -> TokenStream {
    let operator_tt = input_iter.next().expect("Missing operator");
    match operator_tt {
        TokenTree::Punct(punct) if punct.as_char() == '=' => {
            quote!(.eq)
        }
        _ => panic!("Unexpected operator {:?}", operator_tt),
    }
}

fn snake_to_camel(snake: &str) -> String {
    snake
        .split("_")
        .map(|s| s[0..1].to_uppercase() + &s[1..])
        .collect()
}
