mod config;
mod handler;
mod middleware;
mod router;
mod server;
mod repository;

use server::Server;
use config::Config;

use tide::log;

fn main() {
    log::start(); // Initialize the logger
    log::info!("Starting Tide Server");

    let config = Config::from_file("config.toml");
    let server = Server::new(config);
    server.run();
}