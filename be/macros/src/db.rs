use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Ident, parse2};

pub fn derive_row(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse2(input).unwrap();
    let row = &ast.ident;

    let fields = match &ast.data {
        syn::Data::Struct(data) => &data.fields,
        _ => panic!("Row must be a struct"),
    };
    let field_idents: Vec<&Ident> = fields
        .iter()
        .map(|field| field.ident.as_ref().unwrap())
        .collect();
    quote! {
    impl lan_be_frame::mysql::prelude::FromRow for #row {
            fn from_row_opt(row: lan_be_frame::mysql::Row) -> Result<Self, lan_be_frame::mysql::FromRowError> {
                let (#(#field_idents),*) = lan_be_frame::mysql::from_row(row);
                Ok(#row { #(#field_idents),* })
            }
        }
    }
}

// pub fn derive_table(input: TokenStream) -> TokenStream {
//     let ast: DeriveInput = parse2(input).unwrap();
//     let table = &ast.ident;
//     let table_struct_name = &table.to_string();
//     let table_name = derive_table_attr(&ast.attrs);
//     quote!(
//         impl lan_be_frame::db::Table for #table_struct_name {
//             fn name(&self) -> &str {
//                 #table_name
//             }
//         }
//     )
// }

// struct TableAttr<'a> {
//     table_name: &'a str,
//     data_row: &'a str,
// }

// fn derive_table_attr(attr: &Vec<Attribute>) -> &str {
//     for attr in attr {
//         if attr.path().is_ident("meta") {
//             let Ok(expr_assign) = attr.parse_args::<ExprAssign>() else {
//                 panic!("meta should be a ExprAssign");
//             };

//         }
//         if attr.path().is_ident("data_row") {
//             panic!("{:?}", attr.meta);
//         }
//     }

//     ""
// }
