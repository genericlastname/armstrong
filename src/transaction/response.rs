// A gemini Response containing:
//    - status,
//    - mimetype (default: text/gemini).
//    - charset (default: charset=utf-8),
//    - body.
#[derive(Debug)]
pub struct Response {
    pub status: u8,
    pub mimetype: String,
    pub charset: String,
    pub body: String,
}

impl Response {
    pub fn new(data: &str) -> Result<Response, ResponseError> {
        // data_tokens[0] is the response header
        // data_tokens[1] is the response body
        // TODO: Support \r\n AND bare \n as valid separators
        let data_tokens: Vec<&str> = data.splitn(2, "\r\n").collect();
        if data_tokens.len() < 2 {
            // This should never happen unless a gemini header is malformed or
            // missing.
            return Err(ResponseError::new("<META> is missing from, header may be malformed"))
        }

        // header_tokens[0] is the <STATUS> field.
        // header_tokens[1] is the <META> field.
        let header_tokens: Vec<&str> = data_tokens[0].splitn(2, " ").collect();
        let status = header_tokens[0].parse();
        let status: u8 = match status {
            Ok(_s) => status.unwrap(),
            Err(_e) => return Err(ResponseError::new("<STATUS> is missing, header may be malformed"))
        };
        let meta: &str;
        let charset: &str;

        match status {
            // TODO: Handle 1x statuses.
            20..=29 => {
                // Set default meta if not delivered.
                if header_tokens.len() < 2 {
                    meta = "text/gemini;charset=utf-8";
                } else {
                    meta = header_tokens[1];
                }
                
                // Split meta into MIME and charset and set defaults properly.
                let meta_tokens: Vec<&str> = meta.split(";").collect();
                let mime = meta_tokens[0];
                if meta_tokens.len() < 2 && mime[..5].eq("text/") {
                    charset = "utf-8";
                } else {
                    charset = &meta_tokens[1][8..];
                }
                Ok(Response {
                    status: status,
                    mimetype: mime.to_owned(),
                    charset: charset.to_owned(),
                    body: data_tokens[1].to_owned(),
                })
            }
            _ => { 
                Ok(Response {
                    status: 20,
                    mimetype: "text/gemini".to_owned(),
                    charset: "utf-8".to_owned(),
                    body: format!("Status {} is currently unhandled", status),
                })
            }
        }
    }
}

// Handles errors in malformed server Responses
#[derive(Clone, Debug)]
pub struct ResponseError {
    details: String,
}

impl std::fmt::Display for ResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Malformed response!")
    }
}

impl ResponseError {
    fn new(message: &str) -> ResponseError {
        ResponseError { details: message.to_owned(), }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_ok_response_builds() {
        let data = "20 text/gemini;charset=utf-8\r\nBody";
        let r = Response::new(data).unwrap();
        assert_eq!(r.status, 20);
        assert_eq!(r.mimetype, "text/gemini");
        assert_eq!(r.charset, "utf-8");
        assert_eq!(r.body, "Body");
    }

    #[test]
    fn partial_ok_response_builds() {
        let data = "20 text/gemini\r\nBody";
        let r = Response::new(data).unwrap();
        assert_eq!(r.status, 20);
        assert_eq!(r.mimetype, "text/gemini");
        assert_eq!(r.charset, "utf-8");
        assert_eq!(r.body, "Body");
    }

    #[test]
    fn nonexistent_meta_response_builds() {
        let data = "\r\nBody";
        assert!(Response::new(data).is_err());
    }
}
