mod app;

pub mod db;
pub mod module;

pub use app::App;
pub use axum;
pub use mysql;
pub use refinery::embed_migrations;
pub use tokio;

// TODO: add a custom result type for lan_frame
