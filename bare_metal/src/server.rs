use std::net::TcpListener;
use threadpool::ThreadPool;
use crate::config::Config;
use crate::router::Router;

pub struct Server {
    config: Config,
    router: Router,
}

impl Server {
    pub fn new(config: Config, router: Router) -> Self {
        Self { config, router }
    }

    pub fn run(&self) {
        let listener = TcpListener::bind(format!("{}:{}", self.config.address, self.config.port)).unwrap();
        let pool = ThreadPool::new(self.config.thread_pool_size);

        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let router = self.router.clone();
            pool.execute(move || {
                router.handle_connection(&mut stream);
            });
        }
    }
}