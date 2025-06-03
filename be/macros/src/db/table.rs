use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, DeriveInput, Expr, ExprAssign, ExprLit, ExprPath, Lit, Path, parse2};

pub fn derive_table(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse2(input).unwrap();
    let table = &ast.ident;
    let TableAttr { name, row } = derive_table_attr(&ast.attrs);

    quote!(
        impl lan_be_frame::db::Table for #table {
            type Row = #row;
            fn name(&self) -> &'static str {
                #name
            }
        }
    )
}

struct TableAttr {
    name: String,
    row: Path,
}

impl TableAttr {
    fn new(name_op: Option<String>, row_op: Option<Path>) -> Self {
        let name = name_op.expect("name is missing");
        let row = row_op.expect("row is missing");
        Self { name, row }
    }
}
fn derive_table_attr(attr: &Vec<Attribute>) -> TableAttr {
    let mut name_op = None;
    let mut row_op = None;

    for attr in attr {
        if attr.path().is_ident("meta") {
            name_op = Some(derive_table_meta(attr));
        }
        if attr.path().is_ident("row") {
            row_op = Some(derive_row_attr(attr));
        }
    }
    TableAttr::new(name_op, row_op)
}

fn derive_table_meta(attr: &Attribute) -> String {
    let expr_assign: ExprAssign = attr.parse_args().expect("meta should be a ExprAssign");
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

fn derive_row_attr(attr: &Attribute) -> Path {
    let path: Path = attr.parse_args().expect("row should be a Path");
    path.clone()
}
