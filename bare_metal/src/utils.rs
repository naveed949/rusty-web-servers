use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::net::TcpStream;

pub fn parse_headers(stream: &mut TcpStream) -> HashMap<String, String> {
    let mut headers = HashMap::new();
    let mut buf_reader = BufReader::new(stream);
    let mut line = String::new();
    while buf_reader.read_line(&mut line).unwrap() > 0 {
        if line == "\r\n" {
            break;
        }
        let parts: Vec<&str> = line.split(": ").collect();
        headers.insert(parts[0].to_string(), parts[1].trim().to_string());
        line.clear();
    }
    headers
}