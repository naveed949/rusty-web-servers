# Warp Web Server

This project is a simple web server built using the Warp framework in Rust. The server is designed to be modular and easy to extend, with separate modules for configuration, request handling, middleware, routing, and server management.

## Project Structure

The project is organized into the following modules:

- `config`: Handles configuration loading and management.
- `errors`: Defines custom error types and handling.
- `handler`: Contains request handlers.
- `middleware`: Defines middleware for the server.
- `router`: Sets up the routing for the server.
- `server`: Manages the server lifecycle.
- `repository`: Manages data persistence and retrieval.

## Warp Framework features covered

- Modular routing with nested routes
- Middleware integration
- Static file serving (commented out in the example)
- Cookie handling
- State management with shared repository
- Logging
- Fallback routes for unmatched paths
- JSON response handling
- Query parameter handling
- Path parameter handling
- POST request handling
- Configuration loading from file
- Server lifecycle management
- Data persistence and retrieval
- Custom error handling

## Main Components

### `main.rs`

The `main.rs` file is the entry point of the application. It initializes the logger, loads the configuration, creates a new server instance, and runs the server.

```rust
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
```

### Configuration

The `Config` module is responsible for loading the server configuration from a file, typically `config.toml`.

### Server

The `Server` module manages the server lifecycle. It initializes the server with the provided configuration, sets up routes, applies middleware, and starts the server.

### Logging

Logging is initialized at the start of the `main` function using `pretty_env_logger`. This helps in tracking the server's activities and debugging issues.

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Cargo (Rust package manager)

### Installation

1. Clone the repository:

    ```sh
    git clone https://github.com/naveed949/rusty-web-servers.git
    cd warp_server
    ```

2. Build the project:

    ```sh
    cargo build
    ```

3. Run the server:

    ```sh
    cargo run
    ```

### Configuration

Create a `config.toml` file in the root directory with the necessary configuration settings. Example:

```toml
[server]
port = 8080
host = "localhost"
```

## Contributing

Contributions are welcome! Please fork the repository and submit a pull request for any improvements or bug fixes.

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.

## Acknowledgements

- [Warp Framework](https://github.com/seanmonstar/warp) for providing a simple and powerful web framework for Rust.