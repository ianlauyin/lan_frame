use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Field, Fields, Ident, Type, parse2};

pub fn derive_row(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse2(input).unwrap();
    let row = &ast.ident;
    let Data::Struct(DataStruct { fields, .. }) = &ast.data else {
        panic!("Row must be a struct");
    };
    let RowAttr {
        primary_key,
        field_idents,
        field_pairs,
    } = derive_fields(fields);
    let from_row_tokens = from_row_tokens(row, &field_idents);
    let row_tokens = row_tokens(row, primary_key, &field_idents);
    let partial_tokens = partial_tokens(row, primary_key, field_pairs);

    quote! {
        #from_row_tokens
        #row_tokens
        #partial_tokens
    }
}

struct RowAttr<'a> {
    primary_key: &'a Field,
    field_idents: Vec<&'a Ident>,
    field_pairs: Vec<(&'a Ident, &'a Type)>,
}

fn derive_fields(fields: &Fields) -> RowAttr {
    let mut primary_key_op = None;
    let mut field_idents = Vec::new();
    let mut field_pairs = Vec::new();
    for field in fields {
        if let Some(attr) = field.attrs.get(0) {
            if attr.path().is_ident("primary_key") {
                primary_key_op = Some(field);
            }
        }
        field_idents.push(field.ident.as_ref().unwrap());
        field_pairs.push((field.ident.as_ref().unwrap(), &field.ty));
    }
    let primary_key = primary_key_op.expect("primary key is missing");
    RowAttr {
        primary_key,
        field_idents,
        field_pairs,
    }
}

fn from_row_tokens(row: &Ident, field_idents: &Vec<&Ident>) -> TokenStream {
    quote! {
        impl lan_be_frame::mysql::prelude::FromRow for #row {
            fn from_row_opt(row: lan_be_frame::mysql::Row) -> Result<Self, lan_be_frame::mysql::FromRowError> {
                let (#(#field_idents),*) = lan_be_frame::mysql::from_row(row);
                Ok(#row { #(#field_idents),* })
            }
        }
    }
}

fn row_tokens(row: &Ident, primary_key: &Field, field_idents: &Vec<&Ident>) -> TokenStream {
    let pk_type = &primary_key.ty;
    let pk_field = primary_key.ident.as_ref().unwrap().to_string();
    let field_names: Vec<String> = field_idents.iter().map(|ident| ident.to_string()).collect();

    quote! {
        impl lan_be_frame::db::Row for #row {
            type PKType = #pk_type;
            fn pk() -> &'static str {
                #pk_field
            }
            fn fields() -> Vec<String> {
                vec![#(#field_names.to_string()),*]
            }
        }
    }
}

fn partial_tokens(
    row_ident: &Ident,
    primary_key: &Field,
    field_pairs: Vec<(&Ident, &Type)>,
) -> TokenStream {
    let partial_row_ident = Ident::new(&format!("Partial{}", row_ident), row_ident.span());
    let filter_field_pairs: Vec<(&Ident, &Type)> = field_pairs
        .into_iter()
        .filter(|(ident, _)| ident.to_string() != primary_key.ident.as_ref().unwrap().to_string())
        .collect();
    let (partial_field_tokens, into_partial_field_tokens) = field_tokens(&filter_field_pairs);
    quote! {
        pub struct #partial_row_ident {
            #(#partial_field_tokens),*
        }

        impl Into<#partial_row_ident> for #row_ident {
            fn into(self) -> #partial_row_ident {
                #partial_row_ident {
                    #(#into_partial_field_tokens),*
                }
            }
        }

        impl lan_be_frame::db::Partial for #partial_row_ident {
            type Row = #row_ident;
        }
    }
}

fn field_tokens(field_pairs: &Vec<(&Ident, &Type)>) -> (Vec<TokenStream>, Vec<TokenStream>) {
    let mut partial_field_tokens = Vec::new();
    let mut into_partial_field_tokens = Vec::new();
    for (ident, ty) in field_pairs {
        let partial_field_token = quote! (pub #ident: Option<#ty>);
        let into_partial_field_token = quote! (#ident: Some(self.#ident));
        partial_field_tokens.push(partial_field_token);
        into_partial_field_tokens.push(into_partial_field_token);
    }
    (partial_field_tokens, into_partial_field_tokens)
}
