use lan_be_frame::{App, db_init, tokio};

mod db;
mod module;

#[tokio::main]
async fn main() {
    db_init!(db::DB_CONNECT_INFO, "./migrations");

    let mut app = App::new();
    app.add_module(module::UserModule);

    app.run().await;
}
