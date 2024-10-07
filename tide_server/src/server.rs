use crate::config::Config;
use crate::router::configure_router;
use async_std::task;

pub struct Server {
    config: Config,
}

impl Server {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn run(&self) {
        let app = configure_router();
        let address = format!("{}:{}", self.config.address, self.config.port);
        task::block_on(app.listen(address)).unwrap();
    }
}