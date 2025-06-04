mod app;

pub mod db;
pub mod http;
pub mod module;

pub use app::App;
pub use axum;
pub use mysql;
pub use tokio;

// TODO: add a custom result type for lan_frame and remove all unwrap
// TODO: add lazylock logger
// TODO: remove println!() and use logger
// TODO: add test
// TODO: update all reexport
// TODO: Update all docs
