mod app;

pub mod http;
pub mod module;

pub use app::App;
pub use axum;
pub use tokio;

#[cfg(feature = "db")]
pub mod db;

// TODO: add a custom result type for lan_frame and remove all unwrap
// TODO: update all reexport
// TODO: add test
// TODO: Update all docs

// TODO: Future Impl:
// TODO: Lazylock logger, remove println!() and use logger
