use std::sync::Arc;

use bare_metal::config::Config;
use bare_metal::router::Router;
use bare_metal::server::Server;
use bare_metal::handler::{GetHandler, PostHandler};

fn main() {
    let config = Config::from_file("config.toml");
    let mut router = Router::new();
    router.add_route("/", Arc::new(GetHandler));
    router.add_route("/post", Arc::new(PostHandler));
    let server = Server::new(config, router);
    server.run();
}