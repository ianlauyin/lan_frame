use mysql::PooledConn;

use crate::db::LAZY_DB;

use super::Table;

pub struct Repository<T: Table> {
    table: T,
    pooled_conn: PooledConn,
}

impl<T: Table> Repository<T> {
    pub async fn new(table: T) -> Self {
        Self {
            table,
            pooled_conn: LAZY_DB.get_pool().await.get_conn().unwrap(),
        }
    }

    pub async fn get(&self) {
        todo!("after primary key supports")
    }

    pub async fn select(&self) {
        todo!()
    }

    pub async fn insert(&self) {
        todo!("after primary key supports")
    }

    pub async fn update(&self) {
        todo!("after primary key supports")
    }

    pub async fn delete(&self) {
        todo!("after primary key supports")
    }
}
