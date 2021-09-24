// use std::io::{Read, Write};
// use std::net::TcpStream;

use crate::transaction::response::Response;

// Visits the specified url at the given port and returns the resulting
// Response.
pub fn visit(scheme: &str, address: &str, port: &str, path: &str) -> Response {
    let _for_tcp = format!("{}:{}", address, port);
    let _for_dns = format!("{}", address);
    let _request = format!("{}://{}:{}/{}\r\n", scheme, address, port, path);
    Response {
        status: 255,
        mimetype: "Dummy mimetype".to_owned(),
        charset: "Dummy charset".to_owned(),
        body: "Dummy body".to_owned(),
    }
}
