use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use threadpool::ThreadPool;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize)]
struct JsonResponse {
    message: String,
    data: Option<String>,
}
#[derive(Deserialize)]
struct PostData {
    name: String,
}

const OK_STATUS: &str = "HTTP/1.1 200 OK";
const NOT_FOUND_STATUS: &str = "HTTP/1.1 404 NOT FOUND";
const BAD_REQUEST_STATUS: &str = "HTTP/1.1 400 BAD REQUEST";

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
    let mut buf_reader = BufReader::new(&mut stream);
    let mut request_line = String::new();
    if buf_reader.read_line(&mut request_line).is_err() {
        eprintln!("Failed to read request line");
        return;
    }

    let (status_line, contents, content_type) = route_request(&request_line, &mut buf_reader);

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

fn route_request<'a>(request_line: &'a str, buff_reader: &'a mut BufReader<&'a mut TcpStream>) -> (&'a str, String, &'a str) {
    match request_line.trim() {
        "GET / HTTP/1.1" => handle_get_root(),
        "POST /json HTTP/1.1" => handle_post_root(buff_reader),
        req if req.starts_with("GET /?param=") => handle_get_with_param(req),
        "GET /json HTTP/1.1" => handle_get_json(),
        _ => handle_not_found(),
    }
}

fn handle_get_root() -> (&'static str, String, &'static str) {
    (OK_STATUS, "Hello, GET!".to_string(), "text/plain")
}

fn handle_post_root<'a>(buf_reader: &'a mut BufReader<&'a mut TcpStream>) -> (&'a str, String, &'a str) {
    let mut headers = String::new();
    let mut content_length = 0;

    // Read headers
    loop {
        let mut line = String::new();
        if buf_reader.read_line(&mut line).is_err() {
            return (BAD_REQUEST_STATUS, "Failed to read headers".to_string(), "text/plain");
        }
        if line == "\r\n" {
            break;
        }
        if let Some(cl) = line.strip_prefix("Content-Length: ") {
            content_length = cl.trim().parse::<usize>().unwrap_or(0);
        }
        headers.push_str(&line);
    }

    // Read body based on Content-Length
    let mut body = vec![0; content_length];
    if buf_reader.read_exact(&mut body).is_err() {
        return (BAD_REQUEST_STATUS, "Failed to read request body".to_string(), "text/plain");
    }

    let body_str = match String::from_utf8(body) {
        Ok(s) => s,
        Err(_) => return (BAD_REQUEST_STATUS, "Invalid UTF-8 in request body".to_string(), "text/plain"),
    };

    let post_data: PostData = match serde_json::from_str(&body_str) {
        Ok(data) => data,
        Err(_) => return (BAD_REQUEST_STATUS, "Invalid JSON".to_string(), "application/json"),
    };

    let response = JsonResponse {
        message: format!("Hello, {}!", post_data.name),
        data: None,
    };
    let content = serde_json::to_string(&response).unwrap();
    (OK_STATUS, content, "application/json")
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
    (NOT_FOUND_STATUS, "No Route Found".to_string(), "text/plain")
}
