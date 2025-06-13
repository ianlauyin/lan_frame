mod connect;

#[cfg(feature = "migrations")]
mod migration;

use sea_orm::DatabaseConnection;
use std::sync::LazyLock;
use tokio::sync::Mutex;

pub use connect::*;

#[cfg(feature = "migrations")]
pub use migration::*;

#[macro_export]
macro_rules! db_init {
    ($info:expr) => {
        let connection = lan_be_frame::db::get_connection($info).await;
        lan_be_frame::db::LAZY_DB.add_connection(connection).await;
    };

    ($info:expr, $migration_folder:literal) => {
        use refinery::embed_migrations;
        embed_migrations!($migration_folder);
        lan_be_frame::db::migrate(&$info, migrations::runner()).await;
        db_init!($info);
    };
}

pub static LAZY_DB: LazyLock<DB> = LazyLock::new(|| DB {
    inner: Mutex::new(None),
});

pub struct DB {
    inner: Mutex<Option<DatabaseConnection>>,
}

impl DB {
    pub async fn add_connection(&self, connection: DatabaseConnection) {
        let mut db_guard = self.inner.lock().await;
        if db_guard.is_some() {
            panic!("DB already set");
        }
        *db_guard = Some(connection);
    }

    pub async fn get_connection(&self) -> DatabaseConnection {
        let connection_guard = self.inner.lock().await;
        match connection_guard.as_ref() {
            Some(connection) => connection.clone(), // This will clone the Arc Within the DatabaseConnection
            None => panic!("DB not initialized"),
        }
    }
}
