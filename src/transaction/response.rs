// The status code for a gemini Response along with a <META> field that holds
// more information about the Response body.
pub struct Status {
    pub code: u8,
    pub meta: String,
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
//    - Status,
//    - mimetype (default: text/gemini).
//    - charset {optional},
//    - body.
pub struct Response {
    pub status: Status,
    pub mimetype: String,
    pub charset: String,
    pub body: String,
}

impl Response {
    fn new(data: &str) -> Self {
        let tokens: Vec<&str> = data.splitn(2, "\r\n").collect();
        let status = Status::new(tokens[0]);
        let mimetype: &str;
        let charset: &str;

        match status.code {
            // TODO: 1x input support
            20..=29 => {
                if status.meta != "" {
                    let mime_tokens: Vec<&str> = status.meta.split(";").collect();
                    mimetype = mime_tokens[0].trim();
                    if mime_tokens.len() < 2 {
                        charset = "UTF-8";
                    } else {
                        // TODO: handle charset extraction from meta.
                    }
                } else {
                    mimetype = "text/gemini";
                    charset = "UTF-8";
                }
            }
        }

        Response {
            status: status,
            mimetype: mimetype.to_owned(),
            charset: charset.to_owned(),
            body: tokens[1].to_owned(),
        }
    }
}
