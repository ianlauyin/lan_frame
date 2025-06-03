use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Field, Fields, Ident, parse2};

pub fn derive_row(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse2(input).unwrap();
    let row = &ast.ident;
    let Data::Struct(DataStruct { fields, .. }) = &ast.data else {
        panic!("Row must be a struct");
    };
    let RowAttr {
        primary_key,
        field_idents,
    } = derive_fields(fields);
    let from_row_tokens = from_row_tokens(row, field_idents);
    let primary_key_tokens = primary_key_tokens(row, primary_key);
    quote! {
        #from_row_tokens
        #primary_key_tokens
    }
}

struct RowAttr<'a> {
    primary_key: &'a Field,
    field_idents: Vec<&'a Ident>,
}

fn derive_fields(fields: &Fields) -> RowAttr {
    let mut primary_key_op = None;
    let mut field_idents = Vec::new();
    for field in fields {
        if let Some(attr) = field.attrs.get(0) {
            if attr.path().is_ident("primary_key") {
                primary_key_op = Some(field);
            }
        }
        field_idents.push(field.ident.as_ref().unwrap());
    }
    let primary_key = primary_key_op.expect("primary key is missing");
    RowAttr {
        primary_key,
        field_idents,
    }
}

fn from_row_tokens(row: &Ident, field_idents: Vec<&Ident>) -> TokenStream {
    quote! {
        impl lan_be_frame::mysql::prelude::FromRow for #row {
            fn from_row_opt(row: lan_be_frame::mysql::Row) -> Result<Self, lan_be_frame::mysql::FromRowError> {
                let (#(#field_idents),*) = lan_be_frame::mysql::from_row(row);
                Ok(#row { #(#field_idents),* })
            }
        }
    }
}

fn primary_key_tokens(row: &Ident, primary_key: &Field) -> TokenStream {
    let pk_type = &primary_key.ty;
    let pk_field = primary_key.ident.as_ref().unwrap().to_string();
    quote! {
        impl lan_be_frame::db::PrimaryKey for #row {
            type PKType = #pk_type;
            fn name(&self) -> &'static str {
                #pk_field
            }
        }
    }
}
