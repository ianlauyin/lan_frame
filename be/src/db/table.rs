use std::io::{Error, ErrorKind};

use mysql::{Value, prelude::FromRow};

pub use lan_be_frame_macros::{Optional, Row, Table};

pub trait Table {
    type Row: Row;
    fn name(&self) -> &'static str;
    fn row_mapper(&self) -> impl FnMut(mysql::Row) -> Self::Row {
        Self::Row::from_row
    }
}

pub trait Row: FromRow {
    type PKType: Into<Value>;
    fn pk() -> &'static str;
}

pub trait Optional<Row> {
    fn to_data(self) -> Result<Row, Error>;
    fn unwrap_data<T>(data: Option<T>) -> Result<T, Error> {
        data.ok_or_else(|| Error::new(ErrorKind::InvalidData, "Missing data"))
    }
}
