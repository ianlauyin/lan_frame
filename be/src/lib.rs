mod app;

pub mod db;
pub mod http;
pub mod module;

pub use app::App;
pub use axum;
pub use mysql;
pub use tokio;

// TODO: Check license
// TODO: add a custom result type for lan_frame and remove all unwrap
// TODO: add lazylock logger
// TODO: update all reexport
// TODO: Update all docs
