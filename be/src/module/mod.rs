use axum::{Router, routing::MethodRouter};

/// You need the interface and Module have the same name
pub use lan_be_frame_macros::{Module, interface};

pub trait Module {
    fn _name(&self) -> &str;
    fn _router(&self) -> Router;
}

pub trait Interface {
    fn _get_all_routes(&self) -> Vec<(&str, MethodRouter)>;
}
