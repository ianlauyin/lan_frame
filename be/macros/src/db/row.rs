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
    let PartialAttr {
        names,
        field_tokens,
        into_partial_tokens,
        into_params_tokens,
    } = field_tokens(filter_field_pairs);
    quote! {
        pub struct #partial_row_ident {
            #(#field_tokens),*
        }

        impl Into<#partial_row_ident> for #row_ident {
            fn into(self) -> #partial_row_ident {
                #partial_row_ident {
                    #(#into_partial_tokens),*
                }
            }
        }

        impl lan_be_frame::db::PartialRow for #partial_row_ident {
            type Row = #row_ident;
            fn fields() -> Vec<String> {
                vec![#(#names.to_string()),*]
            }
            fn into_params(self) -> impl Into<lan_be_frame::mysql::Params> {
                (#(#into_params_tokens)*)
            }
        }
    }
}

struct PartialAttr {
    names: Vec<String>,
    field_tokens: Vec<TokenStream>,
    into_partial_tokens: Vec<TokenStream>,
    into_params_tokens: Vec<TokenStream>,
}

fn field_tokens(field_pairs: Vec<(&Ident, &Type)>) -> PartialAttr {
    let mut names = Vec::new();
    let mut field_tokens = Vec::new();
    let mut into_partial_tokens = Vec::new();
    let mut into_params_tokens = Vec::new();
    field_pairs.into_iter().for_each(|(ident, ty)| {
        names.push(ident.to_string());
        field_tokens.push(quote! (pub #ident: Option<#ty>));
        into_partial_tokens.push(quote! (#ident : Some(self.#ident)));
        into_params_tokens.push(quote! (self.#ident,));
    });
    PartialAttr {
        names,
        field_tokens,
        into_partial_tokens,
        into_params_tokens,
    }
}
