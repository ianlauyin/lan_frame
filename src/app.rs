use axum::Router;
use std::io;
use tokio::net::TcpListener;
pub struct App {
    router: Router,
}

impl App {
    pub fn init() -> Self {
        App {
            router: Router::new(),
        }
    }

    pub async fn run(&self, local_addr: &str) -> Result<(), io::Error> {
        let mut listenfd = listenfd::ListenFd::from_env();
        let tcp_listener = if let Ok(Some(tcp)) = listenfd.take_tcp_listener(0) {
            tcp.set_nonblocking(true).unwrap();
            TcpListener::from_std(tcp).unwrap()
        } else {
            TcpListener::bind(local_addr).await.unwrap()
        };

        #[cfg(debug_assertions)]
        println!("Listening on {}", tcp_listener.local_addr().unwrap());

        axum::serve(tcp_listener, self.router.clone())
            .await
            .unwrap();
        Ok(())
    }
}
