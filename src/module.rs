use axum::{Router, routing::MethodRouter};

/// You need the interface and Module have the same name
pub use lan_be_frame_macros::{Module, handler, interface};

pub trait Interface {
    fn _get_all_routes(&self) -> Vec<(&str, MethodRouter)>;
}

pub trait Module: Interface + 'static {
    fn name(&self) -> &str;
    fn router(&self) -> Router {
        let mut router = Router::new();
        for route in self._get_all_routes() {
            router = router.route(&route.0, route.1);
        }
        router
    }
}
