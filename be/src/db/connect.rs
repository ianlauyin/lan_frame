use sea_orm::{Database, DatabaseConnection};

pub struct DBConnectInfo<'a> {
    pub url: &'a str,
    pub user: &'a str,
    pub password: &'a str,
    pub db_name: &'a str,
}

pub async fn get_db(info: DBConnectInfo<'_>) -> DatabaseConnection {
    let connection_string = format!(
        "postgresql://{}/{}?user={}&password={}",
        info.url, info.db_name, info.user, info.password
    );
    match Database::connect(connection_string).await {
        Ok(db) => {
            println!("Connected to database");
            db
        }
        Err(e) => {
            panic!("Failed to connect to database: {:?}", e);
        }
    }
}
