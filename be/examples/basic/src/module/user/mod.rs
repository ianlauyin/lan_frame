use lan_be_frame::{
    axum::extract::path,
    module::{Module, handler, interface},
};

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
    async fn create_user_info(req: CreateUserInfoRequest);
}

#[handler]
impl UserModule {
    async fn get_user_info(path_params: GetUserInfoPathParams) -> GetUserInfoResponse {
        GetUserInfoResponse {
            id: path_params.id,
            email: "test@test.com".to_string(),
        }
    }

    async fn create_user_info(req: CreateUserInfoRequest) {
        let email = req.email;
        println!("create_user_info: {email}");
    }
}
