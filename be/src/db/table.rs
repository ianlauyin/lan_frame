use mysql::{Params, Value, prelude::FromRow};

pub use lan_be_frame_macros::{Row, Table};

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

pub trait PartialRow {
    type Row: Row;
    fn fields() -> Vec<String>;
    fn into_params(self) -> impl Into<Params>;
}
