mod row;
use mysql::prelude::FromRow;

pub use lan_be_frame_macros::Table;
pub use row::*;
pub trait Table {
    type Row: Row;
    fn name(&self) -> &'static str;
    fn row_mapper(&self) -> impl FnMut(mysql::Row) -> Self::Row {
        Self::Row::from_row
    }
}
