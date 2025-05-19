// pub struct DB;

// impl DB {
//     pub fn get_pool() -> Pool {
//         let pool = connect();
//         migrate(&pool);
//         pool
//     }

//     fn connect() -> Pool {
//         let opts = OptsBuilder::new()
//             .ip_or_hostname(Some(DB_URL))
//             .user(Some(DB_USER))
//             .pass(Some(DB_PASSWORD))
//             .db_name(Some(DB_SCHEMA_NAME));
//         let pool = Pool::new(opts).expect("Failed to get DB pool");
//         println!("DB connected");
//         pool
//     }

//     fn migrate(pool: &Pool) {
//         let mut conn = pool.get_conn().expect("Failed to get connection");
//         match internal::embedded::migrations::runner().run(&mut conn) {
//             Ok(report) => {
//                 report.applied_migrations().iter().for_each(|migration| {
//                     println!("Applied migration: {}", migration.name());
//                 });
//             }
//             Err(e) => {
//                 panic!("Migration failed: {:?}", e);
//             }
//         }
//     }
// }
