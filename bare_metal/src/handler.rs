use std::net::TcpStream;
use serde::{Deserialize, Serialize};

use crate::request::Request;
use crate::response::Response;

pub trait Handler: Send + Sync {
    fn handle(&self, request: &mut Request) -> Response;
}

pub struct GetHandler;

impl Handler for GetHandler {
    fn handle(&self, request: &mut Request) -> Response {
        Response::new("HTTP/1.1 200 OK", "Hello, GET!", "text/plain")
    }
}

pub struct PostHandler;

impl Handler for PostHandler {
    fn handle(&self, request: &mut Request) -> Response {
        let body = request.body();
        let post_data: PostData = serde_json::from_str(body).unwrap();
        let response = JsonResponse {
            message: format!("Hello, {}!", post_data.name),
            data: None,
        };
        let content = serde_json::to_string(&response).unwrap();
        Response::new("HTTP/1.1 200 OK", &content, "application/json")
    }
}

#[derive(Serialize)]
struct JsonResponse {
    message: String,
    data: Option<String>,
}

#[derive(Deserialize)]
struct PostData {
    name: String,
}