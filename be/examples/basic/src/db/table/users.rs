use lan_be_frame::db::{Optional, Row, Table};

#[derive(Table)]
#[meta(name = "users")]
#[row(User)]
pub struct UserTable;

#[derive(Row, Optional)]
pub struct User {
    #[primary_key]
    pub id: i32,
    pub email: String,
}
