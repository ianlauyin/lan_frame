mod connect;
mod table;

use mysql::Pool;
use std::sync::LazyLock;
use tokio::sync::Mutex;

pub use connect::*;
pub use table::*;

pub static LAZY_DB: LazyLock<Mutex<DB>> = LazyLock::new(|| Mutex::new(DB { pool: None }));

pub struct DB {
    pool: Option<Pool>,
}

impl DB {
    pub fn update_pool(&mut self, pool: Pool) {
        if self.pool.is_some() {
            panic!("DB pool already set");
        }
        self.pool = Some(pool);
    }

    pub fn get_pool(&self) -> &Pool {
        if let Some(pool) = &self.pool {
            pool
        } else {
            panic!("DB pool not set");
        }
    }
}
