use axum::{Router, routing::MethodRouter};

pub use lan_be_frame_macros::{interface, module};

pub trait Module {
    fn _name(&self) -> &str;
    fn _router(&self) -> Router;
    fn _add_route(&mut self, route: &str, handler: MethodRouter);
}
