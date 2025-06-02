pub use lan_be_frame_macros::{Row, Table};

pub trait Table {
    fn name(&self) -> &'static str;
}
