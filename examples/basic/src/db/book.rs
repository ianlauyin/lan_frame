use lan_be_frame::db::{Entity, column_type::Uuid};

#[derive(Entity)]
#[table_name = "book"]
pub struct Book {
    #[primary_key]
    #[auto_increment = false]
    pub id: Uuid,
    #[default_value = "Book Template"]
    #[indexed]
    pub name: String,
    #[nullable]
    pub description: Option<String>,
}
