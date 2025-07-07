use proc_macro2::{Group, TokenStream, TokenTree, token_stream::IntoIter};
use quote::{ToTokens, TokenStreamExt, quote};
use syn::Ident;

// TODO: update prase conditions flow
pub fn condition(input: TokenStream) -> TokenStream {
    let mut input_iter = input.into_iter();
    let column_tokens = parse_path_to_column(input_iter.next());
    let Some(TokenTree::Punct(punct)) = input_iter.next() else {
        panic!("Missing conditions");
    };
    if punct.as_char() != ',' {
        panic!("Expected ',' after entity module path");
    }
    let all_conditions = parse_conditions(&mut input_iter, &column_tokens);
    quote! {
        {
            use sea_orm::ColumnTrait;
            #all_conditions
        }
    }
}

fn parse_path_to_column(entity_path_tt_opt: Option<TokenTree>) -> TokenStream {
    let entity_path_tt = entity_path_tt_opt.expect("Missing db entity module path");
    let TokenTree::Ident(entity_path) = entity_path_tt else {
        panic!("db entity module path must be an ident");
    };
    quote! {#entity_path::Column}
}

// TODO: refactor this loop
fn parse_conditions(input_iter: &mut IntoIter, column_tokens: &TokenStream) -> TokenStream {
    let mut condition_wrapper = ConditionWrapper::None;
    let mut conditions = vec![];
    loop {
        let next_tt = input_iter.next().expect("Missing condition");
        let (next_condition_wrapper, next_condition) = {
            let first_token = next_tt;
            match first_token {
                TokenTree::Ident(ident) => parse_condition(ident, input_iter, column_tokens),
                TokenTree::Group(group) => parse_group(group, input_iter.next(), column_tokens),
                _ => panic!("Unexpected token {:?}", first_token),
            }
        };
        conditions.push(next_condition);
        if next_condition_wrapper == ConditionWrapper::None {
            break;
        }
        if condition_wrapper != ConditionWrapper::None
            && next_condition_wrapper != condition_wrapper
        {
            panic!("Cannot mix AND and OR conditions");
        }
        condition_wrapper = next_condition_wrapper;
    }
    if conditions.len() == 1 {
        return conditions.pop().unwrap();
    }
    quote! { #condition_wrapper #(.add(#conditions))* }
}

fn parse_condition(
    raw_column_name: Ident,
    input_iter: &mut IntoIter,
    column_tokens: &TokenStream,
) -> (ConditionWrapper, TokenStream) {
    let column_name = snake_to_camel_column(raw_column_name);
    let operator = parse_operator(input_iter.next());
    let (condition_wrapper, values) = parse_value(input_iter);
    (
        condition_wrapper,
        quote! {
            #column_tokens::#column_name #operator(#values)
        },
    )
}

fn parse_group(
    group: Group,
    next_condition_wrapper_tt_opt: Option<TokenTree>,
    column_tokens: &TokenStream,
) -> (ConditionWrapper, TokenStream) {
    let mut group_tokens_iter = group.stream().into_iter();
    (
        ConditionWrapper::from_tt_opt(next_condition_wrapper_tt_opt).unwrap(),
        parse_conditions(&mut group_tokens_iter, column_tokens),
    )
}

// TODO: support more values
fn parse_value(input_iter: &mut IntoIter) -> (ConditionWrapper, TokenStream) {
    let mut values = TokenStream::new();
    while let Some(tt) = input_iter.next() {
        if let Ok(next_condition_wrapper) = ConditionWrapper::from_tt(&tt) {
            return (next_condition_wrapper, values);
        }
        values.append(tt);
    }
    values.is_empty().then(|| panic!("Missing values"));
    (ConditionWrapper::None, values)
}

// TODO: support more operators
fn parse_operator(operator_tt_opt: Option<TokenTree>) -> TokenStream {
    let operator_tt = operator_tt_opt.expect("Missing operator");
    match operator_tt {
        TokenTree::Punct(punct) if punct.as_char() == '=' => {
            quote!(.eq)
        }
        _ => panic!("Unexpected operator {:?}", operator_tt),
    }
}

#[derive(PartialEq)]
enum ConditionWrapper {
    Any,
    All,
    None,
}

impl ConditionWrapper {
    fn from_tt(tt: &TokenTree) -> Result<Self, String> {
        match tt {
            TokenTree::Ident(ident) if ident.to_string() == "AND" => Ok(ConditionWrapper::All),
            TokenTree::Ident(ident) if ident.to_string() == "OR" => Ok(ConditionWrapper::Any),
            _ => Err(format!("Unexpected token {:?}", tt)),
        }
    }

    fn from_tt_opt(tt_opt: Option<TokenTree>) -> Result<Self, String> {
        match tt_opt {
            Some(tt) => Self::from_tt(&tt),
            None => Ok(ConditionWrapper::None),
        }
    }
}

impl ToTokens for ConditionWrapper {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            ConditionWrapper::Any => quote! {sea_orm::Condition::any()},
            ConditionWrapper::All => quote! {sea_orm::Condition::all()},
            ConditionWrapper::None => {
                panic!("ConditionWrapper::None should not be used in ToTokens")
            }
        });
    }
}

fn snake_to_camel_column(snake: Ident) -> Ident {
    Ident::new(
        &snake
            .to_string()
            .split("_")
            .map(|s| s[0..1].to_uppercase() + &s[1..])
            .collect::<String>(),
        snake.span(),
    )
}
