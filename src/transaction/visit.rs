use std::io::{Read, Write};
use std::net::TcpStream;

mod response;

use response::Response;

// Visits the specified url at the given port and returns the resulting
// Response.
pub fn visit(url: &str, port: &str) -> Response {
    let destination = format!("{}:{}\r\n", url, port);
    let mut stream = TcpStream::connect(&destination)?;
    stream.write(url.as_bytes())?;

    let mut raw_data: Vec<u8> = vec![];
    stream.read_to_end(&mut raw_data)?;

    let content = String::from_utf8_lossy(&raw_data);
    Response::new(&content);
}
