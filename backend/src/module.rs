use axum::{Router, routing::MethodRouter};

pub use lan_be_frame_macros::{interface, module};

// TODO: _get_user calls normal fn and returns a String and MethodRoute pair
// TODO: Remove _add_route and routers field, Add get_all route in interface for self.router()
pub trait Module {
    fn _name(&self) -> &str;
    fn _router(&self) -> Router;
    fn _add_route(&mut self, route: &str, handler: MethodRouter);
}
