mod connect;

pub use connect::*;

// pub static LAZY_DB: LazyLock<DB> = LazyLock::new(|| DB {
//     client: Mutex::new(None),
// });

// pub struct DB {
//     client: Mutex<Option<Client>>,
// }

// impl DB {
//     pub async fn update_client(&self, client: Client) {
//         let mut client_guard = self.client.lock().await;
//         if client_guard.is_some() {
//             panic!("DB client already set");
//         }
//         *client_guard = Some(client);
//     }

//     pub async fn get_client(&self) -> &Client {
//         let client_guard = self.client.lock().await;
//         client_guard.as_ref().expect("client not initialized")
//     }
// }
