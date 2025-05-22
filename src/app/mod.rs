mod macros;

use axum::Router;
use listenfd::ListenFd;
use mysql::Pool;
use std::collections::HashMap;
use tokio::net::TcpListener;

use crate::module::Module;

pub struct App {
    modules: HashMap<String, Box<dyn Module>>,
    db_pool: Option<Pool>,
}

impl App {
    pub fn new() -> App {
        Self {
            modules: HashMap::new(),
            db_pool: None,
        }
    }

    /// Start the application
    pub async fn run(&mut self) {
        let tcp_listener = Self::tcp_listener().await;
        let mut router = Router::new().with_state(self.db_pool.clone());

        for module in self.modules.values() {
            router = router.nest(module.route(), module.router());
        }

        println!("Listening on {}", tcp_listener.local_addr().unwrap());
        axum::serve(tcp_listener, router).await.unwrap();
    }

    async fn tcp_listener() -> TcpListener {
        #[cfg(debug_assertions)]
        {
            let mut listenfd = ListenFd::from_env();
            if let Ok(Some(tcp)) = listenfd.take_tcp_listener(0) {
                tcp.set_nonblocking(true).unwrap();
                return TcpListener::from_std(tcp).unwrap();
            }
        }
        TcpListener::bind("127.0.0.1:8000").await.unwrap()
    }

    #[doc(hidden)]
    pub fn _internal_add_db(&mut self, pool: Pool) {
        self.db_pool = Some(pool);
    }

    #[doc(hidden)]
    pub fn _internal_add_module(&mut self, module: Box<dyn Module>) {
        let name = module.name().to_string();
        if self.modules.contains_key(&name) {
            panic!("Module {} already exists", name);
        }
        self.modules.insert(name, module);
    }
}
