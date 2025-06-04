use std::io::{Error, ErrorKind};

use mysql::{PooledConn, prelude::Queryable};

use crate::db::{LAZY_DB, Optional, Row};

use super::Table;

pub struct Repository<T: Table> {
    table: T,
}

impl<T: Table> Repository<T> {
    pub fn new(table: T) -> Self {
        Self { table }
    }

    pub async fn get(&self, primary_key: <T::Row as Row>::PKType) -> Result<T::Row, Error> {
        let mut pooled_conn = self.get_pooled_conn().await;
        let id_str = T::Row::pk();
        let table_name = self.table.name();
        let query = format!("SELECT * FROM {table_name} WHERE {id_str} = '{primary_key}'");
        let row = pooled_conn
            .query_first(query)
            .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;
        row.ok_or_else(|| Error::new(ErrorKind::NotFound, "Record not found"))
    }

    pub async fn insert(&self, data: T::Row) {
        todo!()
    }

    pub async fn update(&self, optional_data: impl Optional<T::Row>) {
        todo!()
    }

    pub async fn delete(&self) {
        todo!()
    }

    pub async fn select(&self) -> Vec<T::Row> {
        todo!()
    }

    async fn get_pooled_conn(&self) -> PooledConn {
        LAZY_DB.get_pool().await.get_conn().unwrap()
    }
}
