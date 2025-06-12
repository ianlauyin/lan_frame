use refinery::Runner;
use tokio_postgres::{NoTls, connect};

use crate::db::{DBConnectInfo, connection_url};

pub async fn migrate(info: &DBConnectInfo<'_>, runner: Runner) {
    let connection_url = connection_url(info);
    let (mut client, connection) = connect(&connection_url, NoTls).await.unwrap();

    let connection_handle = tokio::spawn(async move {
        if let Err(e) = connection.await {
            panic!("Connection error: {}", e);
        }
    });

    match runner.run_async(&mut client).await {
        Ok(report) => {
            report.applied_migrations().iter().for_each(|migration| {
                println!("Applied migration: {}", migration.name());
            });
        }
        Err(e) => {
            panic!("Migration failed: {:?}", e);
        }
    }

    connection_handle.abort();
}
