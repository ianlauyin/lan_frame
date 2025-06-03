pub use lan_be_frame_macros::{Row, Table};
use mysql::{FromRowError, Row, prelude::FromRow};

pub trait Table<RowData: FromRow> {
    fn name(&self) -> &'static str;
    fn row_mapper(&self) -> impl FnMut(Row) -> Result<RowData, FromRowError> {
        RowData::from_row_opt
    }
}
