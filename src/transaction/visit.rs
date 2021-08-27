use std::io::{Read, Write};
use std::net::TcpStream;

use crate::transaction::response::Response;

// Visits the specified url at the given port and returns the resulting
// Response.
pub fn visit(scheme: &str, address: &str, port: &str, path: &str) -> Response {
    let for_tcp = format!("{}:{}", address, port);
    let for_dns = format!("{}", address);
    let request = format!("{}://{}:{}/{}\r\n", scheme, address, port, path);
}
