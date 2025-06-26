use lan_be_frame::db::Entity;

#[derive(Entity)]
#[table_name = "user"]
pub struct User {
    #[primary_key]
    pub id: i32,
    pub name: String,
    pub email: String,
}
