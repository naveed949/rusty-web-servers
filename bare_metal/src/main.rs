use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

const OK_STATUS: &str = "HTTP/1.1 200 OK";
const NOT_FOUND_STATUS: &str = "HTTP/1.1 404 NOT FOUND";

fn main() {
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
        handle_connection(stream);
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

    let (status_line, contents) = match request_line.as_str() {
        "GET / HTTP/1.1" => (OK_STATUS, "Hello, GET!".to_string()),
        "POST / HTTP/1.1" => (OK_STATUS, "Hello, POST!".to_string()),
        req if req.starts_with("GET /?param=") => {
            println!("Request: {}", req);
            let param_value = &req[12..req.find(" HTTP/1.1").unwrap_or(req.len())];
            let content = format!("Hello, GET with param: {}", param_value);
            (OK_STATUS, content)
        },
        _ => (NOT_FOUND_STATUS, "Not Found".to_string()),
    };

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    if let Err(e) = stream.write_all(response.as_bytes()) {
        eprintln!("Failed to write response: {}", e);
    }
}
