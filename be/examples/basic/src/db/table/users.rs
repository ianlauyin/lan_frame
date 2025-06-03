use lan_be_frame::db::{Row, Table};

#[derive(Table)]
#[meta(name = "users")]
#[row(User)]
pub struct UserTable;

#[derive(Row)]
pub struct User {
    #[primary_key]
    id: i32,
    email: String,
}
