use lan_be_frame::{
    db::Repository,
    module::{Module, handler, interface},
};

mod create_user_info;
mod get_user_info;

use create_user_info::*;
use get_user_info::*;

use crate::db::table::users::UserTable;

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
        let user_repo = Repository::new(UserTable);
        let user = user_repo.get(path_params.id).await.unwrap();
        GetUserInfoResponse {
            id: user.id,
            email: user.email,
        }
    }

    async fn create_user_info(req: CreateUserInfoRequest) -> CreateUserInfoResponse {
        CreateUserInfoResponse { b: req.a }
    }
}
