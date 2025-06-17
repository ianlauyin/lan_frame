use lan_be_frame::db::Entity;

#[derive(Entity)]
#[table_name = "book"]
pub struct Book {
    #[primary_key]
    id: i32,
    name: String,
    description: String,
}

// use lan_be_frame::sea_orm::entity::prelude::*;

// #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
// #[sea_orm(table_name = "book")]
// pub struct Model {
//     #[sea_orm(primary_key)]
//     pub id: i32,
//     pub name: String,
//     pub description: String,
// }

// impl ActiveModelBehavior for ActiveModel {}

// #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
// pub enum Relation {}
