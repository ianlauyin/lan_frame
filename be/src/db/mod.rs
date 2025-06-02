mod connect;
mod repository;

use mysql::Pool;
use std::sync::LazyLock;
use tokio::sync::Mutex;

pub use connect::*;
// pub use repository::*;

pub static LAZY_DB: LazyLock<DB> = LazyLock::new(|| DB { pool: None });

#[derive(Debug)]
pub struct DB {
    pool: Option<Mutex<Pool>>,
}

impl DB {
    pub fn update_pool(&mut self, pool: Pool) {
        if self.pool.is_some() {
            panic!("DB pool already set");
        }
        self.pool = Some(Mutex::new(pool));
    }
}
