use axum::Router;

pub use lan_frame_macros::{Module, delete, get, post, put};

pub trait Module {
    fn router(&self) -> Router;
}
