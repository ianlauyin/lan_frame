use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, DeriveInput, Expr, ExprAssign, ExprLit, ExprPath, Ident, Lit, Path, parse2};

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
    let table_attr = derive_table_attr(&ast.attrs);
    let table_name = table_attr.name.unwrap();
    let table_row = table_attr.row.unwrap();

    quote!(
        impl lan_be_frame::db::Table<#table_row> for #table {
            fn name(&self) -> &'static str {
                #table_name
            }
        }
    )
}

struct TableAttr {
    name: Option<String>,
    row: Option<Path>,
}

fn derive_table_attr(attr: &Vec<Attribute>) -> TableAttr {
    let mut table_attr = TableAttr {
        name: None,
        row: None,
    };
    for attr in attr {
        if attr.path().is_ident("meta") {
            table_attr.name = Some(derive_table_meta(attr));
        }
        if attr.path().is_ident("row") {
            table_attr.row = Some(derive_row_attr(attr));
        }
    }
    check_all_table_attr(&table_attr);
    table_attr
}

fn derive_table_meta(attr: &Attribute) -> String {
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

fn derive_row_attr<'a>(attr: &Attribute) -> Path {
    let Ok(path) = attr.parse_args::<Path>() else {
        panic!("row should be a Path");
    };
    path.clone()
}

fn check_all_table_attr(table_attr: &TableAttr) {
    if table_attr.name.is_none() {
        panic!("meta is missing");
    }
    if table_attr.row.is_none() {
        panic!("row is missing");
    }
}
