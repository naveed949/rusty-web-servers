use std::io::Write;
use std::net::TcpStream;

pub struct Response {
    status: String,
    body: String,
    content_type: String,
}

impl Response {
    pub fn new(status: &str, body: &str, content_type: &str) -> Self {
        Self {
            status: status.to_string(),
            body: body.to_string(),
            content_type: content_type.to_string(),
        }
    }

    pub fn send(&self, stream: &mut TcpStream) {
        let response = format!(
            "{}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
            self.status,
            self.content_type,
            self.body.len(),
            self.body
        );
        stream.write_all(response.as_bytes()).unwrap();
    }

    pub fn status(&self) -> &str {
        &self.status
    }
}