use mysql::{OptsBuilder, Pool};

#[cfg(feature = "refinery")]
use refinery::Runner;

pub struct DBConnectInfo<'a> {
    pub url: &'a str,
    pub user: &'a str,
    pub password: &'a str,
    pub schema_name: &'a str,
}

pub fn _internal_get_pool(info: DBConnectInfo) -> Pool {
    let opts = OptsBuilder::new()
        .ip_or_hostname(Some(info.url))
        .user(Some(info.user))
        .pass(Some(info.password))
        .db_name(Some(info.schema_name));
    let pool = Pool::new(opts).expect("Failed to get DB pool");
    println!("DB connected");
    pool
}

#[cfg(feature = "refinery")]
pub fn _internal_migrate(pool: &Pool, runner: Runner) {
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
