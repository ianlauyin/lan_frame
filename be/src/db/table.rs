// pub use lan_be_frame_macros::Table;
pub use lan_be_frame_macros::Row;

pub trait Table {
    fn name(&self) -> &'static str;
}
