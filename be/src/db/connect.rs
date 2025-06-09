use postgres::{Client, Config, NoTls};

// #[macro_export]
// macro_rules! db_init {
//     ($info:expr) => {
//         let client = lan_be_frame::db::get_client($info);
//         lan_be_frame::db::LAZY_DB.update_client(client).await;
// };
// ($info:expr, $migration_folder:literal) => {
//     let client = lan_be_frame::db::get_client($info);
//     refinery::embed_migrations!($migration_folder);
//     lan_be_frame::db::migrate(&client, migrations::runner());
//     lan_be_frame::db::LAZY_DB.update_client(client).await;
// };
// }

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
    config
        .connect(NoTls)
        .inspect(|_| println!("Connected to database"))
        .unwrap()
}

// TODO: replace this function using diesel
// pub fn migrate(client: &Client, runner: refinery::Runner) {
//     let mut conn = client.get_conn().expect("Failed to get connection");
//     match runner.run(&mut conn) {
//         Ok(report) => {
//             report.applied_migrations().iter().for_each(|migration| {
//                 println!("Applied migration: {}", migration.name());
//             });
//         }
//         Err(e) => {
//             panic!("Migration failed: {:?}", e);
//         }
//     }
// }
