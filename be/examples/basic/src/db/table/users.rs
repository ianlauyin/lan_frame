use lan_be_frame::db::{Row, Table};

#[derive(Table)]
#[meta(name = "users")]
#[row(User)]
pub struct UserTable;

#[derive(Row)]
pub struct User {
    id: i32,
    email: String,
}
