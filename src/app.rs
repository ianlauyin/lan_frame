use axum::Router;
use listenfd::ListenFd;
use mysql::Pool;
use tokio::net::TcpListener;

use crate::module::Module;

pub struct App {
    modules: Vec<Box<dyn Module>>,
    db_pool: Option<Pool>,
}

impl App {
    pub fn new() -> App {
        Self {
            modules: Vec::new(),
            db_pool: None,
        }
    }

    pub fn add_db(&mut self, pool: Pool) {
        self.db_pool = Some(pool);
    }

    pub fn add_module(&mut self, module: Box<dyn Module>) {
        self.modules.push(module);
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let tcp_listener = Self::tcp_listener().await?;
        let mut router = Router::new().with_state(self.db_pool.clone());

        for module in self.modules.iter() {
            router = router.nest("/", module.router());
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
