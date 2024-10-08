use crate::config::Config;
use crate::router::configure_router;

pub struct Server {
    config: Config,
}

impl Server {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn run(&self) {
        let routes = configure_router();
        let address = format!("{}:{}", self.config.address, self.config.port);
        warp::serve(routes)
            .run(([127, 0, 0, 1], self.config.port))
            .await;

        log::info!("Server listening on http://{}", address);
    }
}
