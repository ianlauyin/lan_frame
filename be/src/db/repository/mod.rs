use std::io::{Error, ErrorKind};

use mysql::{PooledConn, prelude::Queryable};

mod delete_query;
mod insert_query;
mod select_query;
mod update_query;

use crate::db::{LAZY_DB, Row};

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

    pub fn raw_query(&mut self, query: &str) -> Result<Vec<T::Row>, Error> {
        self.pooled_conn
            .query_map(query, |row| row)
            .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))
    }

    pub fn select_by_pk(&mut self, primary_key: <T::Row as Row>::PKType) -> Result<T::Row, Error> {
        let stmt = self.pk_stmt("SELECT");
        let row = self
            .pooled_conn
            .exec_first(stmt, (primary_key,))
            .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;
        row.ok_or_else(|| Error::new(ErrorKind::NotFound, "Record not found"))
    }

    pub fn delete_by_pk(&mut self, primary_key: <T::Row as Row>::PKType) -> Result<(), Error> {
        let stmt = self.pk_stmt("DELETE");
        self.pooled_conn
            .exec_drop(stmt, (primary_key,))
            .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))
    }

    pub fn insert(&mut self) {
        todo!()
    }

    pub fn select(&self) {
        todo!()
    }

    pub fn update(&self) {
        todo!()
    }

    pub fn delete(&self) {
        todo!()
    }

    fn pk_stmt(&self, action: &str) -> String {
        let id_str = T::Row::pk();
        let table_name = self.table.name();
        format!("{action} FROM {table_name} WHERE {id_str} = ?")
    }
}
