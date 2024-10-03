# Bare Metal Web Server

A bare-metal web server implemented in Rust, designed for high performance and minimal dependencies. This project demonstrates how to build a web server from scratch using Rust's standard library and a few essential crates.

## Features

- **Configuration Management**: Load server configuration from a file.
- **Routing**: Map URL paths to handler functions.
- **Request Handling**: Support for GET and POST requests.
- **Response Handling**: Send HTTP responses with status codes and content types.
- **Middleware**: Add middleware for logging, authentication, etc.
- **Concurrency**: Handle multiple connections using a thread pool.

## Project Structure

``` plaintext
bare_metal/
├── src/
│   ├── config.rs
│   ├── handler.rs
│   ├── lib.rs
│   ├── main.rs
│   ├── middleware.rs
│   ├── request.rs
│   ├── response.rs
│   ├── router.rs
│   └── server.rs
├── Cargo.toml
└── README.md
```

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)

### Installation

1. Clone the repository:

    ```sh
    git clone https://github.com/yourusername/bare_metal.git
    cd bare_metal
    ```

2. Build the project:

    ```sh
    cargo build
    ```

3. Run the server:

    ```sh
    cargo run
    ```

## Configuration

The server configuration is loaded from a `config.toml` file. Here is an example configuration:

```toml
address = "127.0.0.1"
port = 7878
thread_pool_size = 4
```

## Usage

The server is configured to handle the following routes:

- `GET /`: Returns a simple "Hello, GET!" message.
- `POST /post`: Handles POST requests and returns a JSON response.

Example Request:

- GET Request:

    ```sh
    curl -X GET http://localhost:7878/
    ```

```sh
curl -X POST http://localhost:7878/
```

- POST Request:

    ```sh
    curl -X POST http://localhost:7878/post -d '{"name": "Rust"}'
    ```

## Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request if you have any improvements or new features to add.

## License

Distributed under the MIT License. See `LICENSE` for more information.
