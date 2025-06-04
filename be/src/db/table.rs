use std::io::{Error, ErrorKind};

use mysql::{Row, prelude::FromRow};

pub use lan_be_frame_macros::{Optional, Row, Table};

pub trait Table {
    type Row: FromRow + PrimaryKey;
    fn name(&self) -> &'static str;
    fn row_mapper(&self) -> impl FnMut(Row) -> Self::Row {
        Self::Row::from_row
    }
}

pub trait Optional<Data> {
    fn to_data(self) -> Result<Data, Error>;
    fn unwrap_data<T>(data: Option<T>) -> Result<T, Error> {
        data.ok_or_else(|| Error::new(ErrorKind::InvalidData, "Missing data"))
    }
}

pub trait PrimaryKey {
    type PKType;
    fn name(&self) -> &'static str;
}
