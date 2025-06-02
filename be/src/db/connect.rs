use mysql::{OptsBuilder, Pool};

#[macro_export]
macro_rules! db_init {
    ($info:expr) => {
        let pool = lan_be_frame::db::get_pool($info);
        lan_be_frame::db::LAZY_DB.update_pool(pool);
    };

    ($info:expr, $migration_folder:literal) => {
        let pool = lan_be_frame::db::get_pool($info);
        refinery::embed_migrations!($migration_folder);
        lan_be_frame::db::migrate(&pool, migrations::runner());
        lan_be_frame::db::LAZY_DB.update_pool(pool);
    };
}

pub struct DBConnectInfo<'a> {
    pub url: &'a str,
    pub user: &'a str,
    pub password: &'a str,
    pub schema_name: &'a str,
}

pub fn get_pool(info: DBConnectInfo) -> Pool {
    let opts = OptsBuilder::new()
        .ip_or_hostname(Some(info.url))
        .user(Some(info.user))
        .pass(Some(info.password))
        .db_name(Some(info.schema_name));
    let pool = Pool::new(opts).expect("Failed to get DB pool");
    println!("DB connected");
    pool
}

pub fn migrate(pool: &Pool, runner: refinery::Runner) {
    let mut conn = pool.get_conn().expect("Failed to get connection");
    match runner.run(&mut conn) {
        Ok(report) => {
            report.applied_migrations().iter().for_each(|migration| {
                println!("Applied migration: {}", migration.name());
            });
        }
        Err(e) => {
            panic!("Migration failed: {:?}", e);
        }
    }
}
