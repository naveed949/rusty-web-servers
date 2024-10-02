use bare_metal::config::Config;
use bare_metal::router::Router;
use bare_metal::server::Server;

fn main() {
    let config = Config::from_file("config.toml");
    let router = Router::new();
    let server = Server::new(config, router);
    server.run();
}