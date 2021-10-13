use std::convert::TryInto;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::Arc;

use crate::transaction::response::Response;
use crate::transaction::dummy_verifier::DummyVerifier;

// Visits the specified url at the given port and returns the resulting
// Response.
pub fn visit(scheme: &str, address: &str, port: &str, path: &str) -> Response {
    let for_tcp = format!("{}:{}", address, port);
    // let for_dns = format!("{}", address);
    let request = format!("{}://{}:{}/{}\r\n", scheme, address, port, path);
    let mut data = Vec::new();

    // TLS stuff.
    let mut root_store = rustls::RootCertStore::empty();
    root_store.add_server_trust_anchors(
        webpki_roots::TLS_SERVER_ROOTS
        .0
        .iter()
        .map(|ta| {
            rustls::OwnedTrustAnchor::from_subject_spki_name_constraints(
                ta.subject,
                ta.spki,
                ta.name_constraints,
            )
        })
    );
    let mut cfg = rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_store)
        .with_no_client_auth();
    let mut config = rustls::client::DangerousClientConfig {
        cfg: &mut cfg,
    };
    let dummy_verifier = Arc::new(DummyVerifier::new());
    config.set_certificate_verifier(dummy_verifier);
    let rc_config = Arc::new(cfg);
    let hostname: rustls::ServerName = address.try_into().unwrap();
    let mut client = rustls::ClientConnection::new(rc_config, hostname)
        .expect("Can't open connection.");

    // Open gemini connection
    println!("{}", for_tcp);
    let mut socket = TcpStream::connect(for_tcp)
        .expect("Can't connect to socket");
    let mut stream = rustls::Stream::new(&mut client, &mut socket);

    stream.write(request.as_bytes()).unwrap();
    while client.wants_read() {
        client.read_tls(&mut socket).unwrap();
        client.process_new_packets().unwrap();
        let _ = client.reader().read_to_end(&mut data);
    }
    let raw_content = String::from_utf8_lossy(&data).to_string();

    Response::new(&raw_content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn visit_returns_dummy_response() {
        let response: Response = visit("", "", "", "");
        assert_eq!(response.status, 255);
        assert_eq!(response.mimetype, "Dummy mimetype");
        assert_eq!(response.charset, "Dummy charset");
        assert_eq!(response.body, "Dummy body");
    }
}
