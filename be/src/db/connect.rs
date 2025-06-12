use postgres::{Client, Config, NoTls};
use refinery::Runner;

#[macro_export]
macro_rules! db_init {
    ($info:expr) => {
        let client = lan_be_frame::db::get_client($info);
        lan_be_frame::db::LAZY_DB.add_client(client).await;
    };

    ($info:expr, $migration_folder:literal) => {
        let mut client = lan_be_frame::db::get_client($info);
        refinery::embed_migrations!($migration_folder);
        lan_be_frame::db::migrate(&mut client, migrations::runner());
        lan_be_frame::db::LAZY_DB.add_client(client).await;
    };
}

pub struct DBConnectInfo<'a> {
    pub url: &'a str,
    pub user: &'a str,
    pub password: &'a str,
    pub db_name: &'a str,
}

pub fn get_client(info: DBConnectInfo) -> Client {
    let mut config = Config::new();
    config.host(info.url);
    config.user(info.user);
    config.password(info.password);
    config.dbname(info.db_name);
    config.connect(NoTls).unwrap()
}

pub fn migrate(client: &mut Client, runner: Runner) {
    match runner.run(client) {
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
