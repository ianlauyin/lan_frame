use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Ident, parse2};

pub fn derive_optional(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse2(input).unwrap();
    let row_ident = &ast.ident;
    let optional_row_ident = Ident::new(&format!("Optional{}", row_ident), row_ident.span());
    let Data::Struct(DataStruct { fields, .. }) = &ast.data else {
        panic!("Row must be a struct");
    };
    let (optional_field_tokens, to_data_fields_tokens): (Vec<TokenStream>, Vec<TokenStream>) =
        fields
            .iter()
            .map(|field| {
                let field_ident = field.ident.as_ref().unwrap();
                let field_type = &field.ty;
                let optional_field_token = quote! (#field_ident: Option<#field_type>);
                let to_data_fields_token =
                    quote! (#field_ident: Self::unwrap_data(self.#field_ident)?);
                (optional_field_token, to_data_fields_token)
            })
            .unzip();
    quote! {
        pub struct #optional_row_ident {
            #(#optional_field_tokens),*
        }

        impl lan_be_frame::db::Optional<#row_ident> for #optional_row_ident {
            fn to_data(self) -> Result<#row_ident, std::io::Error> {
                Ok(#row_ident {
                    #(#to_data_fields_tokens),*
                })
            }
        }
    }
}
