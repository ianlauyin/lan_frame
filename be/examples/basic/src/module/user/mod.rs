use lan_be_frame::module::{Module, handler, interface};

mod create_user_info;
mod get_user_info;

use create_user_info::*;
use get_user_info::*;

#[derive(Module)]
pub struct UserModule;

#[interface]
pub trait UserModule {
    #[get("/user/info/{id}")]
    async fn get_user_info(path_params: GetUserInfoPathParams) -> GetUserInfoResponse;
    #[post("/user/info")]
    async fn create_user_info(req: CreateUserInfoRequest) -> CreateUserInfoResponse;
}

#[handler]
impl UserModule {
    async fn get_user_info(path_params: GetUserInfoPathParams) -> GetUserInfoResponse {
        GetUserInfoResponse {
            id: path_params.id,
            email: "test@test.com".to_string(),
        }
    }

    async fn create_user_info(req: CreateUserInfoRequest) -> CreateUserInfoResponse {
        CreateUserInfoResponse { b: req.a }
    }
}
