pub mod connect;
mod repository;

pub use connect::*;
pub use repository::*;

// #[macro_export]
// macro_rules! add_db {
//     ($app:expr , $info:expr) => {
//         let pool = lan_be_frame::db::_internal_get_pool($info);
//         $app._internal_add_db(pool);
//     };

//     ($app:expr , $info:expr, $migration_folder:literal) => {{
//         let pool = lan_be_frame::db::_internal_get_pool($info);
//         refinery::embed_migrations!($migration_folder);
//         lan_be_frame::db::_internal_migrate(&pool, migrations::runner());
//         $app._internal_add_db(pool);
//     }};
// }
