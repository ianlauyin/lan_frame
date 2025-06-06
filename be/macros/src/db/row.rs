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
    let from_row_tokens = from_row_tokens(row, &field_idents);
    let row_tokens = row_tokens(row, primary_key, &field_idents);
    quote! {
        #from_row_tokens
        #row_tokens
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
            fn to_params(self) -> impl Into<lan_be_frame::mysql::Params> {
                (#(self.#field_idents),*)
            }
        }
    }
}

// use proc_macro2::TokenStream;
// use quote::quote;
// use syn::{Data, DataStruct, DeriveInput, Field, Ident, parse2};

// pub fn derive_partial(input: TokenStream) -> TokenStream {
//     let ast: DeriveInput = parse2(input).unwrap();
//     let row_ident = &ast.ident;
//     let partial_row_ident = Ident::new(&format!("Partial{}", row_ident), row_ident.span());
//     let Data::Struct(DataStruct { fields, .. }) = &ast.data else {
//         panic!("Row must be a struct");
//     };

//     let ((partial_field_tokens, into_partial_field_tokens), to_data_fields_tokens): (
//         (Vec<TokenStream>, Vec<TokenStream>),
//         Vec<TokenStream>,
//     ) = fields.iter().map(field_tokens).unzip();

//     quote! {
//         pub struct #partial_row_ident {
//             #(#partial_field_tokens),*
//         }

//         impl Into<#partial_row_ident> for #row_ident {
//             fn into(self) -> #partial_row_ident {
//                 #partial_row_ident {
//                     #(#into_partial_field_tokens),*
//                 }
//             }
//         }

//         impl lan_be_frame::db::partial for #partial_row_ident {
//             type Row = #row_ident;
//             fn into(self) -> Result<Self::Row, std::io::Error> {
//                 Ok(#row_ident {
//                     #(#to_data_fields_tokens),*
//                 })
//             }
//         }
//     }
// }

// fn field_tokens(field: &Field) -> ((TokenStream, TokenStream), TokenStream) {
//     let field_ident = field.ident.as_ref().unwrap();
//     let field_type = &field.ty;
//     let partial_field_token = quote! (pub #field_ident: Option<#field_type>);
//     let into_partial_field_token = quote! (#field_ident: Some(self.#field_ident));
//     let to_data_fields_token = quote! (#field_ident: Self::unwrap_data(self.#field_ident)?);
//     (
//         (partial_field_token, into_partial_field_token),
//         to_data_fields_token,
//     )
// }
