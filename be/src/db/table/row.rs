use mysql::{Params, Value, prelude::FromRow};
use std::io::{Error, ErrorKind};

pub use lan_be_frame_macros::Row;

pub trait Row: FromRow {
    type PKType: Into<Value>;
    fn pk() -> &'static str;
    fn fields() -> Vec<String>;
    fn to_params(self) -> impl Into<Params>;
}

pub trait Partial {
    type Row: Row;
    fn into(self) -> Result<Self::Row, Error>;
    fn unwrap_data<T>(data: Option<T>) -> Result<T, Error> {
        data.ok_or_else(|| Error::new(ErrorKind::InvalidData, "Missing data"))
    }
}
