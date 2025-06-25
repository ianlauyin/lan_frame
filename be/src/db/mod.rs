mod connect;
mod entity;

#[cfg(feature = "migrations")]
mod migration;

use std::sync::LazyLock;

use sea_orm::DatabaseConnection;
use tokio::sync::RwLock;

pub use connect::*;
pub use entity::*;
pub mod column_type {
    pub use sea_orm::prelude::{
        Date, DateTime, DateTimeWithTimeZone, Decimal, Json, Time, TimeDate, TimeDateTime,
        TimeDateTimeWithTimeZone, TimeTime, Uuid,
    };
}

#[cfg(feature = "migrations")]
pub use migration::*;

#[macro_export]
macro_rules! db_init {
    ($info:expr) => {
        let connection = lan_be_frame::db::get_connection($info).await;
        lan_be_frame::db::add_db(connection).await;
    };

    ($info:expr, $migration_folder:literal) => {
        use refinery::embed_migrations;
        embed_migrations!($migration_folder);
        lan_be_frame::db::migrate(&$info, migrations::runner()).await;
        db_init!($info);
    };
}

pub static LAZY_DB: LazyLock<RwLock<Option<DatabaseConnection>>> = LazyLock::new(Default::default);

pub async fn add_db(connection: DatabaseConnection) {
    let mut db = LAZY_DB.write().await;
    if db.is_some() {
        panic!("DB already set");
    }
    *db = Some(connection);
}

pub async fn get_db() -> DatabaseConnection {
    let db = LAZY_DB.read().await;
    match db.as_ref() {
        Some(connection) => connection.clone(),
        None => panic!("DB not initialized"),
    }
}
