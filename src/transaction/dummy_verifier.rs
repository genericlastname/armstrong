use std::time::SystemTime;
use rustls::{Certificate, Error, ServerName};
use rustls::client::{ServerCertVerifier, ServerCertVerified};

// For now a dummy TLS verifier that ignores certificates is fine. Down the
// line this will be replaced by a proper verifier.
pub struct DummyVerifier;

impl DummyVerifier {
    pub fn new() -> Self {
        DummyVerifier {}
    }
}

impl ServerCertVerifier for DummyVerifier {
    fn verify_server_cert(
        &self,
        _: &Certificate,
        _: &[Certificate],
        _: &ServerName,
        _: &mut dyn Iterator<Item = &[u8]>,
        _: &[u8],
        _: SystemTime
    ) -> Result<ServerCertVerified, Error> {
        return Ok(ServerCertVerified::assertion());
    }
}
