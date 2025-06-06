use mysql::{Value, prelude::FromRow};

/// Derive Row will also give you Partial<Row = Row>
pub use lan_be_frame_macros::Row;

pub trait Row: FromRow {
    type PKType: Into<Value>;
    fn pk() -> &'static str;
    fn fields() -> Vec<String>;
}

pub trait Partial {
    type Row: Row;
}
