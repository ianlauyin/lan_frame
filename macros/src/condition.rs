use proc_macro2::{TokenStream, TokenTree, token_stream::IntoIter};
use quote::{ToTokens, TokenStreamExt, quote};
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
    let first_token = input_iter.next().expect("Missing conditions");
    let all_conditions = parse_group_conditions(first_token, &mut input_iter, &entity_path);
    quote! {
        {
            use sea_orm::ColumnTrait;
            #all_conditions
        }
    }
}

// Should return TokenStream of IntoCondition
fn parse_group_conditions(
    first_token: TokenTree,
    input_iter: &mut IntoIter,
    entity_path: &Ident,
) -> TokenStream {
    match first_token {
        TokenTree::Group(group) => {
            let mut group_conditions = group.stream().into_iter();
            let group_first_token = group_conditions.next().expect("Missing conditions");
            parse_group_conditions(group_first_token, &mut group_conditions, entity_path)
        }
        TokenTree::Ident(ident) => {
            let (mut condition_wrapper, condition) =
                parse_condition(ident, input_iter, entity_path);
            // Simple Case: Only return Column::column.operator(values)
            if condition_wrapper == ConditionWrapper::None {
                return condition;
            }
            // Complex Case: return Cond::all/Cond::any
            todo!("loop util condition_wrapper is None")
        }
        _ => panic!("Unexpected token {:?}", first_token),
    }
}

#[derive(PartialEq)]
enum ConditionWrapper {
    Any,
    All,
    None,
}

impl ToTokens for ConditionWrapper {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            ConditionWrapper::Any => quote! {lan_be_frame::sea_orm::Condition::any()},
            ConditionWrapper::All => quote! {lan_be_frame::sea_orm::Condition::all()},
            ConditionWrapper::None => quote! {},
        });
    }
}

// Should return a ConditionWrapper and a TokenStream with Column::column.operator(values)
fn parse_condition(
    column_name_ident: Ident,
    input_iter: &mut IntoIter,
    entity_path: &Ident,
) -> (ConditionWrapper, TokenStream) {
    let mut condition_wrapper = ConditionWrapper::None;
    let column_name = parse_column_name(column_name_ident);
    let operator = parse_operator(input_iter.next());
    let mut values = TokenStream::new();
    while let Some(tt) = input_iter.next() {
        match tt {
            TokenTree::Ident(ident) if ident.to_string() == "AND" => {
                condition_wrapper = ConditionWrapper::All;
                break;
            }
            TokenTree::Ident(ident) if ident.to_string() == "OR" => {
                condition_wrapper = ConditionWrapper::Any;
                break;
            }
            _ => values.append(tt),
        }
    }
    if values.is_empty() {
        panic!("Missing values");
    }
    (
        condition_wrapper,
        quote! {
            #entity_path::Column::#column_name #operator(#values)
        },
    )
}

fn parse_column_name(column_name_ident: Ident) -> Ident {
    Ident::new(
        &snake_to_camel(&column_name_ident.to_string()),
        column_name_ident.span(),
    )
}

fn parse_operator(operator_tt_opt: Option<TokenTree>) -> TokenStream {
    let Some(operator_tt) = operator_tt_opt else {
        panic!("Missing operator");
    };
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
