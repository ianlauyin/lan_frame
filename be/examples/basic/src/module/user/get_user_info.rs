use lan_be_frame::http::{PathParams, Response};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, PathParams)]
pub struct GetUserInfoPathParams {
    pub(super) id: i32,
}

#[derive(Serialize, Response)]
pub struct GetUserInfoResponse {
    pub(super) id: i32,
    pub(super) email: String,
}
