use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    Attribute, DeriveInput, Expr, ExprAssign, ExprLit, ExprPath, Ident, Lit, LitStr, parse2,
};

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

pub fn derive_table(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse2(input).unwrap();
    let table = &ast.ident;
    let table_name = derive_table_attr(&ast.attrs);
    quote!(
        impl lan_be_frame::db::Table for #table {
            fn name(&self) -> &'static str {
                #table_name
            }
        }
    )
}

// TODO: Add Row struct
fn derive_table_attr(attr: &Vec<Attribute>) -> String {
    for attr in attr {
        if attr.path().is_ident("meta") {
            let Ok(expr_assign) = attr.parse_args::<ExprAssign>() else {
                panic!("meta should be a ExprAssign");
            };
            let Expr::Path(ExprPath { path, .. }) = *expr_assign.left else {
                panic!("meta left assignment should be a ExprPath");
            };
            if path.segments.len() != 1 {
                panic!("meta left assignment should be a single segment path");
            }
            let meta_name = path.segments[0].ident.to_string();
            match meta_name.as_str() {
                "name" => {
                    let Expr::Lit(ExprLit {
                        lit: Lit::Str(lit_str),
                        ..
                    }) = *expr_assign.right
                    else {
                        panic!("meta name should be a literal");
                    };
                    return lit_str.value();
                }
                _ => panic!("meta name should be name"),
            }
        }
    }
    panic!("meta attribute not found");
}
