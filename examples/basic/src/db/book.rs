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

// use lan_be_frame::sea_orm::entity::prelude::*;

// #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
// #[sea_orm(table_name = "book")]
// pub struct Model {
//     #[sea_orm(primary_key, auto_increment = false)]
//     pub id: Uuid,
//     #[sea_orm(default_value = "Book Template", indexed)]
//     pub name: String,
//     #[sea_orm(nullable)]
//     pub description: Option<String>,
// }

// impl ActiveModelBehavior for ActiveModel {}

// #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
// pub enum Relation {}
