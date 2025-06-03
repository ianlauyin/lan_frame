use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse2};

pub fn derive_request(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse2(input).unwrap();
    let req_name = &ast.ident;

    quote! {
        impl<S: Sync + Send> lan_be_frame::axum::extract::FromRequest<S> for #req_name {
            type Rejection = lan_be_frame::http::FromRequestRejection;

            async fn from_request(
                req: lan_be_frame::axum::extract::Request,
                state: &S,
            ) -> Result<Self, Self::Rejection> {
                match *req.method() {
                    lan_be_frame::axum::http::Method::GET => {
                        use lan_be_frame::axum::extract::Query;
                        let query: Query<#req_name> = Query::from_request(req, state)
                            .await
                            .map_err(|e| lan_be_frame::http::FromRequestRejection::Query(e))?;
                        Ok(query.0)
                    }
                    _ => {
                        use lan_be_frame::axum::Json;
                        let json: Json<#req_name> = Json::from_request(req, state)
                            .await
                            .map_err(|e| lan_be_frame::http::FromRequestRejection::Json(e))?;
                        Ok(json.0)
                    }
                }
            }
        }
    }
}

pub fn derive_response(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse2(input).unwrap();
    let res_name = &ast.ident;
    quote! {
        impl lan_be_frame::axum::response::IntoResponse for #res_name {
            fn into_response(self) -> lan_be_frame::axum::response::Response {
                use lan_be_frame::axum::Json;
                Json::into_response(Json(self))
            }
        }
    }
}

pub fn derive_path_params(input: TokenStream) -> TokenStream {
    // let ast: DeriveInput = parse2(input).unwrap();
    // let res_name = &ast.ident;
    // quote! {
    //     impl<S: Sync + Send> lan_be_frame::axum::extract::FromRequest<S> for #req_name {
    //         type Rejection = lan_be_frame::http::FromRequestRejection;

    //         async fn from_request(
    //             req: lan_be_frame::axum::extract::Request,
    //             state: &S,
    //         ) -> Result<Self, Self::Rejection> {
    //             match *req.method() {
    //                 lan_be_frame::axum::http::Method::GET => {
    //                     use lan_be_frame::axum::extract::Query;
    //                     let query: Query<#req_name> = Query::from_request(req, state)
    //                         .await
    //                         .map_err(|e| lan_be_frame::http::FromRequestRejection::Query(e))?;
    //                     Ok(query.0)
    //                 }
    //                 _ => {
    //                     use lan_be_frame::axum::Json;
    //                     let json: Json<#req_name> = Json::from_request(req, state)
    //                         .await
    //                         .map_err(|e| lan_be_frame::http::FromRequestRejection::Json(e))?;
    //                     Ok(json.0)
    //                 }
    //             }
    //         }
    //     }
    // }
    quote!()
}
