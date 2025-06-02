mod connect;
mod repository;

use mysql::Pool;
use std::sync::{LazyLock, Mutex};

pub use connect::*;
// pub use repository::*;

#[macro_export]
macro_rules! db_init {
    ($info:expr) => {
        let pool = lan_be_frame::db::get_pool($info);
        lan_be_frame::db::LAZY_DB.update_pool(pool);
    };

    ($info:expr, $migration_folder:literal) => {
        let pool = lan_be_frame::db::get_pool($info);
        refinery::embed_migrations!($migration_folder);
        lan_be_frame::db::migrate(&pool, migrations::runner());
        lan_be_frame::db::LAZY_DB.update_pool(pool);
    };
}

pub static LAZY_DB: LazyLock<DB> = LazyLock::new(|| DB { pool: None });

#[derive(Debug)]
pub struct DB {
    pool: Option<Mutex<Pool>>,
}

impl DB {
    pub fn update_pool(&self, pool: Pool) -> Self {
        if self.pool.is_some() {
            panic!("DB pool already set");
        }
        Self {
            pool: Some(Mutex::new(pool)),
        }
    }
}
