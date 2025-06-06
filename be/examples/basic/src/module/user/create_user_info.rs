use lan_be_frame::http::Request;
use serde::Deserialize;

use crate::db::table::users::PartialUser;

#[derive(Deserialize, Request)]
pub struct CreateUserInfoRequest {
    pub(super) email: String,
}

impl CreateUserInfoRequest {
    pub fn partial_user(self) -> PartialUser {
        PartialUser {
            email: Some(self.email),
        }
    }
}
