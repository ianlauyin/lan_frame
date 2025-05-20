use axum::{Router, routing::MethodRouter};
use listenfd::ListenFd;
use mysql::Pool;
use tokio::net::TcpListener;

pub struct App {
    routes: Vec<(String, MethodRouter)>,
    db_pool: Option<Pool>,
}

impl App {
    pub fn new() -> App {
        Self {
            routes: Vec::new(),
            db_pool: None,
        }
    }

    pub fn add_db(&mut self, pool: Pool) {
        self.db_pool = Some(pool);
    }

    // Remove it and add add_module instead
    pub fn add_route(&mut self, path: &str, method: MethodRouter) {
        self.routes.push((path.to_string(), method));
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let tcp_listener = Self::tcp_listener().await?;
        let mut router = Router::new().with_state(self.db_pool.clone());

        for (path, method) in self.routes.iter() {
            router = router.route(path, method.clone());
        }

        println!("Listening on {}", tcp_listener.local_addr()?);
        axum::serve(tcp_listener, router).await?;
        Ok(())
    }

    async fn tcp_listener() -> Result<TcpListener, Box<dyn std::error::Error>> {
        #[cfg(debug_assertions)]
        {
            let mut listenfd = ListenFd::from_env();
            if let Ok(Some(tcp)) = listenfd.take_tcp_listener(0) {
                tcp.set_nonblocking(true)?;
                return Ok(TcpListener::from_std(tcp)?);
            }
        }
        Ok(TcpListener::bind("127.0.0.1:8000").await?)
    }
}
