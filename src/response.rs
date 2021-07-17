// The status code for a gemini Response along with a <META> field that holds
// more information about the Response body.
struct Status {
    code: u8,
    meta: String,
}

impl Status {
    fn new(header: &str) -> Self {
        let tokens: Vec<&str> = header.splitn(2, " ").collect();
        Status {
            code: tokens[0].parse().unwrap(),
            meta: tokens[1].to_owned()
        }
    }
}

// A gemini Response containing:
//      Status,
//      mimetype (default: text/gemini).
//      charset {optional},
//      body.
struct Response {
    status: Status,
    mimetype: String,
    charset: String,
    body: String,
}

impl Response {
    fn new(data: &str) -> Self {
        // Split response status and body on gemini spec CRLF.
        let tokens: Vec<&str> = data.splitn(2, "\r\n").collect();
        status = Status::new(tokens[0]);
        match status.code {
            // TODO: 1x input support
            20..29 => {
                let mimetype;
                let charset;
                if (status.meta != "") {
                    let mime_tokens: Vec<&str> = status.meta.split(";")
                        .collect()
                        .trim();
                    mimetype = mime_tokens[0];
                    if (mime_tokens.len() < 2) {
                        charset = "UTF-8";
                    } else {
                        // TODO: handle charset extraction from meta.
                    }
                } else {
                    mimetype = "text/gemini";
                    charset = "UTF-8";
                }
                Response { status, mimetype, charset, tokens[1]  }
            }
        }
    }
}
