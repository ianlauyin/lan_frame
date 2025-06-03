use mysql::{PooledConn, prelude::FromRow};

use crate::db::LAZY_DB;

use super::{PrimaryKey, Table};

pub type PKType<T> = <<T as Table>::Row as PrimaryKey>::PKType;

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

    pub async fn get(&self, primary_key: PKType<T>) -> T::Row {
        todo!()
    }

    pub async fn insert(&self, primary_key: PKType<T>) {
        todo!()
    }

    pub async fn update(&self, primary_key: PKType<T>) {
        todo!()
    }

    pub async fn delete(&self, primary_key: PKType<T>) {
        todo!()
    }

    pub async fn select(&self) {
        todo!()
    }
}
