use std::io::{Error, ErrorKind};

use mysql::{
    PooledConn, Statement, params,
    prelude::{AsStatement, Queryable},
};

mod delete_query;
mod insert_query;
mod select_query;
mod update_query;

use crate::db::{LAZY_DB, Optional, Row, repository::insert_query::InsertQuery};

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

    pub async fn get(&mut self, primary_key: <T::Row as Row>::PKType) -> Result<T::Row, Error> {
        let id_str = T::Row::pk();
        let table_name = self.table.name();
        let stmt = format!("SELECT * FROM {table_name} WHERE {id_str} = ?");
        let row = self
            .pooled_conn
            .exec_first(stmt, (primary_key,))
            .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;
        row.ok_or_else(|| Error::new(ErrorKind::NotFound, "Record not found"))
    }

    pub async fn select(&self) {
        todo!()
    }

    pub async fn insert(&self, optional_data: impl Optional<T::Row>) {
        // let query = InsertQuery::new(self.table.name());
        // self.pooled_conn.exec(query, optional_data).await.unwrap();
    }

    pub async fn update(&self) {
        todo!()
    }

    pub async fn delete(&self) {
        todo!()
    }
}
