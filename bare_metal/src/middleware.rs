use crate::request::Request;
use crate::response::Response;

pub trait Middleware: Send + Sync {
    fn handle(&self, request: &mut Request, response: &mut Response);
}

pub struct Logger;

impl Middleware for Logger {
    fn handle(&self, request: &mut Request, response: &mut Response) {
        println!("Request: {} {}", request.method(), request.path());
        println!("Response: {}", response.status());
    }
}