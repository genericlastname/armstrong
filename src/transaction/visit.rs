use std::convert::TryInto;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::Arc;

use url::{Url, ParseError};

use crate::transaction::response::{
    create_fake_response, 
    Response, 
};
use crate::transaction::dummy_verifier::DummyVerifier;

// Visits the specified url at the given port and returns the resulting
// Response.
pub fn visit(s: &str) -> Response {
    // TODO: Handle errors.
    let url = Url::parse(s).unwrap();

    // TODO: Handle a-base URLs (should fail).
    let for_tcp = format!("{}:{}", url.host_str().unwrap(), url.port().unwrap());
    let request = format!("{}://{}:{}/{}\r\n",
        url.scheme(),
        url.host_str().unwrap(),
        url.port().unwrap(),
        url.path());

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
    let hostname: rustls::ServerName = url.host_str().unwrap().try_into().unwrap();
    let mut client = match rustls::ClientConnection::new(rc_config, hostname) {
        Ok(client) => client,
        Err(error) => {
            return create_fake_response(20, &error.to_string());
        }
    };

    // Open gemini connection
    let mut socket = match TcpStream::connect(for_tcp) {
        Ok(socket) => socket,
        Err(error) => {
            return create_fake_response(20, &error.to_string());
        }
    };

    let mut stream = rustls::Stream::new(&mut client, &mut socket);

    // Get data
    let mut data = Vec::new();
    stream.write(request.as_bytes()).unwrap();
    while client.wants_read() {
        client.read_tls(&mut socket).unwrap();
        client.process_new_packets().unwrap();
    }
    let _ = client.reader().read_to_end(&mut data);
    let content = String::from_utf8_lossy(&data).to_string();

    let response = match Response::new(&content) {
        Ok(response) => response,
        Err(error) => {
            return create_fake_response(20, &error.to_string());
        }
    };
    response
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn visit_to_valid_site_returns_ok_status() {
        let response: Response = visit("gemini://carcosa.net:1965");
        assert_eq!(response.status, 20);
        assert_eq!(response.mimetype, "text/gemini");
        assert_eq!(response.charset, "utf-8");
    }
}
