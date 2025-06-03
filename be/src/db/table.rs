pub use lan_be_frame_macros::{Row, Table};
use mysql::{Row, prelude::FromRow};

pub trait Table {
    type Row: FromRow + PrimaryKey;
    fn name(&self) -> &'static str;
    fn row_mapper(&self) -> impl FnMut(Row) -> Self::Row {
        Self::Row::from_row
    }
}

pub trait PrimaryKey {
    type PKType;
    fn name(&self) -> &'static str;
}
