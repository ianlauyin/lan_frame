use mysql::{PooledConn, prelude::Queryable};
use std::io::{Error, ErrorKind};

use super::super::{LAZY_DB, PartialRow, Row, Table};

pub struct Repository<T: Table> {
    table: T,
    pooled_conn: PooledConn,
}

// TODO: Need to update the Error here
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

    // Single Operation
    pub fn select_by_pk(&mut self, primary_key: <T::Row as Row>::PKType) -> Result<T::Row, Error> {
        let pk_stmt_postfix = self.pk_stmt_postfix();
        let stmt = format!("SELECT * FROM {pk_stmt_postfix}");
        let row = self
            .pooled_conn
            .exec_first(stmt, (primary_key,))
            .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;
        row.ok_or_else(|| Error::new(ErrorKind::NotFound, "Record not found"))
    }

    pub fn delete_by_pk(&mut self, primary_key: <T::Row as Row>::PKType) -> Result<(), Error> {
        let pk_stmt_postfix = self.pk_stmt_postfix();
        let stmt = format!("DELETE FROM {pk_stmt_postfix}");
        self.pooled_conn
            .exec_drop(stmt, (primary_key,))
            .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))
    }

    pub fn insert_one(&mut self, partial_row: impl PartialRow<Row = T::Row>) -> Result<(), Error> {
        let table_name = self.table.name();
        let fields = T::Row::fields().join(", ");
        let stmt = format!("INSERT INTO {table_name} ({fields}) VALUES (?)");
        self.pooled_conn
            .exec_drop(stmt, partial_row.into_params())
            .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))
    }

    pub fn update_one(&self, primary_key: <T::Row as Row>::PKType) {
        todo!()
    }

    // Batch Operation
    pub fn select(&self) {
        todo!()
    }

    pub fn delete(&self) {
        todo!()
    }

    pub fn insert(&mut self) {
        todo!()
    }

    pub fn update(&mut self) {
        todo!()
    }

    fn pk_stmt_postfix(&self) -> String {
        let id_str = T::Row::pk();
        let table_name = self.table.name();
        format!("{table_name} WHERE {id_str} = ?")
    }
}
