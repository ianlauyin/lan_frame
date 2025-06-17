use lan_be_frame::db::Entity;

#[derive(Entity)]
#[table_name = "user"]
pub struct User {
    #[primary_key]
    id: i32,
    name: String,
    email: String,
}

// use lan_be_frame::sea_orm::entity::prelude::*;

// #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
// #[sea_orm(table_name = "user")]
// pub struct Model {
//     #[sea_orm(primary_key)]
//     pub id: i32,
//     pub name: String,
// }

// impl ActiveModelBehavior for ActiveModel {}

// #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
// pub enum Relation {}
