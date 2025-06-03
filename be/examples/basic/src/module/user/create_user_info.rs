use lan_be_frame::http::{Request, Response};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Request)]
pub struct CreateUserInfoRequest {
    pub(super) a: String,
}

#[derive(Serialize, Response)]
pub struct CreateUserInfoResponse {
    pub(super) b: String,
}
