use lan_be_frame::{
    db::{EntityTrait, condition},
    module::{Module, handler, interface},
};

mod create_user_info;
mod get_user_info;

use create_user_info::*;
use get_user_info::*;

use crate::db::user;

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
        let user = user::User::select()
            .filter(condition!(user, id = path_params.id))
            .one()
            .await
            .unwrap()
            .unwrap();
        GetUserInfoResponse {
            id: user.id,
            email: user.email,
        }
    }

    async fn create_user_info(req: CreateUserInfoRequest) {
        let email = req.email;
        println!("create_user_info: {email}");
    }
}
