use mysql::{OptsBuilder, Pool};
use refinery::Runner;

pub struct DBConnectInfo {
    pub url: String,
    pub user: String,
    pub password: String,
    pub schema_name: String,
}

pub struct DBConnect;

impl DBConnect {
    pub fn get_pool(info: DBConnectInfo, optional_migration_runner: Option<&Runner>) -> Pool {
        let pool = Self::connect(info);
        if let Some(runner) = optional_migration_runner {
            Self::migrate(&pool, runner);
        }
        pool
    }

    fn connect(info: DBConnectInfo) -> Pool {
        let opts = OptsBuilder::new()
            .ip_or_hostname(Some(info.url))
            .user(Some(info.user))
            .pass(Some(info.password))
            .db_name(Some(info.schema_name));
        let pool = Pool::new(opts).expect("Failed to get DB pool");
        println!("DB connected");
        pool
    }

    fn migrate(pool: &Pool, runner: &Runner) {
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
}
