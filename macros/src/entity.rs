use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    Attribute, Data, DeriveInput, Expr, ExprLit, Field, Fields, Lit, Meta, MetaNameValue, parse2,
};

pub fn derive_entity(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse2(input).unwrap();
    let req_name = &ast.ident;
    let StructAttr { table_name } = parse_struct_attr(&ast.attrs);

    let Data::Struct(data) = &ast.data else {
        panic!("Entity can only be struct");
    };
    let model_field_tokens = get_model_field_tokens(&data.fields);

    quote! {
        impl lan_be_frame::db::EntityTrait for #req_name {
            type OrmEntity = Entity;
        }
        use lan_be_frame::sea_orm::entity::prelude::*;
        #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
        #[sea_orm(table_name = #table_name)]
        pub struct Model {
            #(#model_field_tokens,)*
        }

        impl ActiveModelBehavior for ActiveModel {}

        #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
        pub enum Relation {}
    }
}

struct StructAttr {
    table_name: String,
}

fn parse_struct_attr(attrs: &[Attribute]) -> StructAttr {
    let mut table_name_op = None;
    attrs.iter().for_each(
        |attr| match attr.path().get_ident().unwrap().to_string().as_str() {
            "table_name" => {
                table_name_op = Some(get_table_name(&attr.meta));
            }
            other => panic!("Unsupported struct attribute: {}", other),
        },
    );
    let Some(table_name) = table_name_op else {
        panic!("table_name is required in struct attribute");
    };
    StructAttr { table_name }
}

fn get_table_name(meta: &Meta) -> String {
    let Meta::NameValue(MetaNameValue {
        value: Expr::Lit(ExprLit {
            lit: Lit::Str(lit_str),
            ..
        }),
        ..
    }) = &meta
    else {
        panic!("table_name must be a string");
    };
    lit_str.value()
}

#[derive(Default)]
struct ModelFieldAttr {
    primary_key: bool,
    // nullable,
    // auto_increment,
    // unique,
    // indexed,
    // default_value,
}

impl Into<TokenStream> for ModelFieldAttr {
    fn into(self) -> TokenStream {
        let mut tokens = quote!();
        if self.primary_key {
            tokens.extend(quote! {
                #[sea_orm(primary_key)]
            });
        }
        tokens
    }
}

fn get_model_field_tokens(fields: &Fields) -> Vec<TokenStream> {
    let mut have_primary_key = false;
    let model_fields = fields
        .iter()
        .map(|field| {
            let model_field_attr = get_model_field_attr(&field.attrs);
            if model_field_attr.primary_key {
                match have_primary_key {
                    true => panic!("Only one primary key is allowed"),
                    false => have_primary_key = true,
                }
            }
            let model_field_attr_tokens: TokenStream = model_field_attr.into();
            let model_field = Field {
                attrs: Vec::new(),
                ..field.clone()
            };
            quote! {
                #model_field_attr_tokens
                #model_field
            }
        })
        .collect();
    if !have_primary_key {
        panic!("Primary key is required");
    }
    model_fields
}

fn get_model_field_attr(attrs: &[Attribute]) -> ModelFieldAttr {
    let mut model_field_attr = ModelFieldAttr::default();
    attrs.iter().for_each(
        |attr| match attr.path().get_ident().unwrap().to_string().as_str() {
            "primary_key" => {
                model_field_attr.primary_key = true;
            }
            other => panic!("Unsupported field attribute: {}", other),
        },
    );
    model_field_attr
}
