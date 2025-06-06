use lan_be_frame::http::Request;
use serde::Deserialize;

#[derive(Deserialize, Request)]
pub struct CreateUserInfoRequest {
    pub(super) a: String,
}
