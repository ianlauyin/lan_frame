use axum::Router;

pub use lan_frame_macros::{interface, module};

pub trait Module {
    fn name(&self) -> &str;
    fn route(&self) -> &str;
    fn router(&self) -> Router;
}
