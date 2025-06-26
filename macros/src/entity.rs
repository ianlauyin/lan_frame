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
        use sea_orm::entity::prelude::*;
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

// TODO: Add type check when parsing
// ref: https://docs.rs/sqlx/latest/sqlx/postgres/types/index.html
#[derive(Default)]
struct ModelFieldAttr {
    primary_key: bool,
    nullable: bool,
    unique: bool,
    indexed: bool,
    default_value: Option<String>,
    auto_increment: Option<bool>,
}

impl Into<TokenStream> for ModelFieldAttr {
    fn into(self) -> TokenStream {
        let mut tokens: Vec<TokenStream> = vec![];
        self.primary_key.then(|| tokens.push(quote!(primary_key)));
        self.nullable.then(|| tokens.push(quote!(nullable)));
        self.unique.then(|| tokens.push(quote!(unique)));
        self.indexed.then(|| tokens.push(quote!(indexed)));

        if let Some(default_value) = self.default_value {
            tokens.push(quote!(default_value = #default_value));
        }
        if let Some(auto_increment) = self.auto_increment {
            tokens.push(quote!(auto_increment = #auto_increment));
        }
        if tokens.is_empty() {
            return quote!();
        }
        quote! {
            #[sea_orm(
                #(#tokens,)*
            )]
        }
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
                check_meta_path("primary_key", &attr.meta);
                model_field_attr.primary_key = true;
            }
            "nullable" => {
                check_meta_path("nullable", &attr.meta);
                model_field_attr.nullable = true;
            }
            "unique" => {
                check_meta_path("unique", &attr.meta);
                model_field_attr.unique = true;
            }
            "indexed" => {
                check_meta_path("indexed", &attr.meta);
                model_field_attr.indexed = true;
            }
            "default_value" => {
                let lit = get_lit_from_meta_name_value("default_value", &attr.meta);
                let Lit::Str(lit_str) = lit else {
                    panic!("default_value must be a string");
                };
                model_field_attr.default_value = Some(lit_str.value());
            }
            "auto_increment" => {
                let lit = get_lit_from_meta_name_value("auto_increment", &attr.meta);
                let Lit::Bool(lit_bool) = lit else {
                    panic!("auto_increment must be a boolean");
                };
                model_field_attr.auto_increment = Some(lit_bool.value());
            }
            other => panic!("Unsupported field attribute: {}", other),
        },
    );
    model_field_attr
}

fn check_meta_path(attr_name: &str, meta: &Meta) {
    if !matches!(meta, Meta::Path(_)) {
        panic!("{} must be not be list or name-value pair", attr_name);
    }
}

fn get_lit_from_meta_name_value<'a>(attr_name: &str, meta: &'a Meta) -> &'a Lit {
    let Meta::NameValue(MetaNameValue { value: expr, .. }) = &meta else {
        panic!("{} must be a name-value pair", attr_name)
    };
    let Expr::Lit(ExprLit { lit, .. }) = &expr else {
        panic!("value expr of {} must be a literal", attr_name);
    };
    lit
}
