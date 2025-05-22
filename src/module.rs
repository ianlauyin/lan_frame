use axum::Router;

pub use lan_frame_macros::Module;

pub trait Module {
    fn route(&self) -> &str;
    fn router(&self) -> Router;
}
