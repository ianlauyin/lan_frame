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
        let db = lan_be_frame::db::get_db($info).await;
        lan_be_frame::db::LAZY_DB.add_db(db).await;
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
    pub async fn add_db(&self, db: DatabaseConnection) {
        let mut db_guard = self.inner.lock().await;
        if db_guard.is_some() {
            panic!("DB already set");
        }
        *db_guard = Some(db);
    }
}

// pub async fn get_client(&mut self) -> &mut Client {
//     let client_guard = self.client.lock().await;
// }
// }
