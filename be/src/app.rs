use axum::Router;
use listenfd::ListenFd;
use std::collections::HashMap;
use tokio::net::TcpListener;

use crate::module::Module;

pub struct App {
    modules: HashMap<String, Box<dyn Module>>,
}

impl App {
    pub fn new() -> App {
        Self {
            modules: HashMap::new(),
        }
    }

    /// Start the application
    pub async fn run(&mut self) {
        let tcp_listener = Self::tcp_listener().await;
        let mut router = Router::new();

        for module in self.modules.values() {
            router = router.merge(module._router());
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

    pub fn add_module(&mut self, module: impl Module) {
        let name = module._name().to_string();
        if self.modules.contains_key(&name) {
            panic!("Module {} already exists", name);
        }
        self.modules.insert(name, Box::new(module));
    }
}
