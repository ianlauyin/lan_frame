use axum::{Router, routing::MethodRouter};

/// You need the interface and Module have the same name
pub use lan_be_frame_macros::{Module, interface};

pub trait Interface {
    fn _get_all_routes(&self) -> Vec<(&str, MethodRouter)>;
}

pub trait Module: Interface {
    fn _name(&self) -> &str;
    fn _router(&self) -> Router {
        let mut router = Router::new();
        for route in self._get_all_routes() {
            router = router.route(&route.0, route.1);
        }
        router
    }
}
