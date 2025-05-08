use axum::{Router, routing::MethodRouter};
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

    pub fn add_route(&mut self, path: &str, method_router: MethodRouter) {
        self.router = self.router.clone().route(path, method_router)
    }

    pub async fn run(&self) {
        let tcp_listener = Self::tcp_listener().await;

        println!("Listening on {}", tcp_listener.local_addr().unwrap());
        axum::serve(tcp_listener, self.router.clone())
            .await
            .unwrap();
    }

    async fn tcp_listener() -> TcpListener {
        #[cfg(debug_assertions)]
        {
            let mut listenfd = listenfd::ListenFd::from_env();
            if let Ok(Some(tcp)) = listenfd.take_tcp_listener(0) {
                tcp.set_nonblocking(true).unwrap();
                return TcpListener::from_std(tcp).unwrap();
            }
        }
        TcpListener::bind("127.0.0.1:8000").await.unwrap()
    }
}
