use lan_be_frame::db::Entity;

#[derive(Entity)]
#[table_name = "user"]
pub struct User {
    #[primary_key]
    pub id: u16,
    pub name: String,
    pub email: String,
}

// use lan_be_frame::sea_orm::entity::prelude::*;

// #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
// #[sea_orm(table_name = "user")]
// pub struct Model {
//     #[sea_orm(primary_key)]
//     id: u16,
//     name: String,
//     email: String,
// }

// impl ActiveModelBehavior for ActiveModel {}

// #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
// pub enum Relation {}