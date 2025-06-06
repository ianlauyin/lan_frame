use lan_be_frame::db::{Row, Table};

#[derive(Table)]
#[meta(name = "books")]
#[row(Book)]
pub struct BookTable;

#[derive(Row)]
pub struct Book {
    #[primary_key]
    pub id: i32,
    pub title: String,
    pub author: String,
    pub price: f64,
}
