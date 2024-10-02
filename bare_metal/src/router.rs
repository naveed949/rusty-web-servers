use std::net::TcpStream;
use std::sync::Arc;
use std::collections::HashMap;
use crate::handler::Handler;
use crate::request::Request;
use crate::response::Response;

#[derive(Default, Clone)]
pub struct Router {
    routes: HashMap<String, Arc<dyn Handler>>,
}

impl Router {
    pub fn new() -> Self {
        Self { routes: HashMap::new() }
    }

    pub fn add_route(&mut self, path: &str, handler: Arc<dyn Handler>) {
        self.routes.insert(path.to_string(), handler);
    }

    pub fn handle_connection(&self, stream: &mut TcpStream) {
        let mut request = Request::from_stream(&stream);
        let response = self.route_request(&mut request);
        response.send(stream);
    }

    fn route_request(&self, request: &mut Request) -> Response {
        match self.routes.get(request.path()) {
            Some(handler) => handler.handle(request),
            None => Response::new("HTTP/1.1 404 NOT FOUND", "No Route Found", "text/plain"),
        }
    }
}