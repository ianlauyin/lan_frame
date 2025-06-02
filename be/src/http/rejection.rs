pub use axum::extract::rejection::{JsonRejection, QueryRejection};
use axum::response::{IntoResponse, Response};

pub enum FromRequestRejection {
    Json(JsonRejection),
    Query(QueryRejection),
}

impl IntoResponse for FromRequestRejection {
    fn into_response(self) -> Response {
        match self {
            Self::Json(rejection) => rejection.into_response(),
            Self::Query(rejection) => rejection.into_response(),
        }
    }
}
