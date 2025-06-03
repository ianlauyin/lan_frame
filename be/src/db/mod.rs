mod connect;
mod repository;
mod table;

use mysql::Pool;
use std::sync::LazyLock;
use tokio::sync::Mutex;

pub use connect::*;
pub use repository::*;
pub use table::*;

pub static LAZY_DB: LazyLock<DB> = LazyLock::new(|| DB {
    pool: Mutex::new(None),
});

pub struct DB {
    pool: Mutex<Option<Pool>>,
}

impl DB {
    pub async fn update_pool(&self, pool: Pool) {
        let mut pool_guard = self.pool.lock().await;
        if pool_guard.is_some() {
            panic!("DB pool already set");
        }
        *pool_guard = Some(pool);
    }

    pub async fn get_pool(&self) -> Pool {
        let pool_guard = self.pool.lock().await;
        pool_guard.as_ref().expect("Pool not initialized").clone()
    }
}
