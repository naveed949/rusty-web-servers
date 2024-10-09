mod config;
mod errors;
mod handler;
mod middleware;
mod repository;
mod router;
mod server;

use config::Config;
use server::Server;

#[tokio::main]
async fn main() {
    // Set the log level programmatically
    std::env::set_var("RUST_LOG", "info");
    // Initialize the logger
    pretty_env_logger::init();
    log::info!("Starting Warp Server");

    let config = Config::from_file("config.toml");
    let server = Server::new(config);
    server.run().await;
}
