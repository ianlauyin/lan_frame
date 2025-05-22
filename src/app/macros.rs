#[macro_export]
macro_rules! add_module {
    ($app:expr , $module:expr) => {
        $app._internal_add_module(Box::new($module));
    };
}

#[macro_export]
macro_rules! add_db {
    ($app:expr , $info:expr) => {
        use lan_frame::db::_internal_connect;
        let pool = _internal_connect($info);
        $app._internal_add_db(pool);
    };

    ($app:expr , $info:expr, $migration_folder:literal) => {{
        use lan_frame::db::{_internal_connect, _internal_migrate};
        mod embedded {
            use lan_frame::embed_migrations;
            embed_migrations!($migration_folder);
        }
        let pool = _internal_connect($info);
        _internal_migrate(&pool, embedded::migrations::runner());
        $app._internal_add_db(pool);
    }};
}
