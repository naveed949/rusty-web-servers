use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use threadpool::ThreadPool;
use serde::Serialize;
use serde_json::json;

#[derive(Serialize)]
struct JsonResponse {
    message: String,
    data: Option<String>,
}

const OK_STATUS: &str = "HTTP/1.1 200 OK";
const NOT_FOUND_STATUS: &str = "HTTP/1.1 404 NOT FOUND";

fn main() {
    // Create a thread pool with 4 threads
    let pool = ThreadPool::new(4);
    // Bind the TCP listener to the address and port
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap_or_else(|e| {
        eprintln!("Failed to bind to port 7878: {}", e);
        std::process::exit(1);
    });
    println!("Server running on port 7878");

    // Loop through incoming connections
    for stream in listener.incoming() {
        let stream = stream.unwrap_or_else(|e| {
            eprintln!("Failed to establish connection: {}", e);
            std::process::exit(1);
        });
        // Execute the connection handling in a thread from the pool
        pool.execute(move || handle_connection(stream));
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = match buf_reader.lines().next() {
        Some(Ok(line)) => line,
        _ => {
            eprintln!("Failed to read request line");
            return;
        }
    };

    let (status_line, contents, content_type) = route_request(&request_line);

    let response = format!(
        "{}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        content_type,
        contents.len(),
        contents
    );

    if let Err(e) = stream.write_all(response.as_bytes()) {
        eprintln!("Failed to write response: {}", e);
    }
}

fn route_request(request_line: &str) -> (&str, String, &str) {
    match request_line {
        "GET / HTTP/1.1" => handle_get_root(),
        "POST / HTTP/1.1" => handle_post_root(),
        req if req.starts_with("GET /?param=") => handle_get_with_param(req),
        "GET /json HTTP/1.1" => handle_get_json(),
        _ => handle_not_found(),
    }
}

fn handle_get_root() -> (&'static str, String, &'static str) {
    (OK_STATUS, "Hello, GET!".to_string(), "text/plain")
}

fn handle_post_root() -> (&'static str, String, &'static str) {
    (OK_STATUS, "Hello, POST!".to_string(), "text/plain")
}

fn handle_get_with_param(req: &str) -> (&'static str, String, &'static str) {
    let param_value = &req[12..req.find(" HTTP/1.1").unwrap_or(req.len())];
    let content = format!("Hello, GET with param: {}", param_value);
    (OK_STATUS, content, "text/plain")
}

fn handle_get_json() -> (&'static str, String, &'static str) {
    let response = JsonResponse {
        message: "Hello, JSON!".to_string(),
        data: None,
    };
    let content = json!(response).to_string();
    (OK_STATUS, content, "application/json")
}

fn handle_not_found() -> (&'static str, String, &'static str) {
    (NOT_FOUND_STATUS, "Not Found".to_string(), "text/plain")
}
