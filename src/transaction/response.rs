// A gemini Response containing:
//    - status,
//    - mimetype (default: text/gemini).
//    - charset {optional},
//    - body.
#[derive(Debug)]
pub struct Response {
    pub status: u8,
    pub mimetype: String,
    pub charset: String,
    pub body: String,
}

impl Response {
    pub fn new(data: &str) -> Self {
        // data_tokens[0] is the response header
        // data_tokens[1] is the response body
        let data_tokens: Vec<&str> = data.splitn(2, "\r\n").collect();

        // header_tokens[0] is the <STATUS> field.
        // header_tokens[1] is the <META> field.
        let header_tokens: Vec<&str> = data_tokens[0].splitn(2, " ").collect();
        let status: u8 = header_tokens[0].parse().unwrap();
        let mut meta: &str = header_tokens[1];
        let charset = "utf-8";  // TODO: will need to be mut once charset is extracted.

        match status {
            // TODO: Handle 1x statuses.
            20..=29 => {
                if meta == "" {
                    meta = "text/gemini";
                }
                // TODO: extract charset here.
                Response {
                    status: status,
                    mimetype: meta.to_owned(),
                    charset: charset.to_owned(),
                    body: data_tokens[1].to_owned(),
                }
            }
            _ => { 
                Response {
                    status: 20,
                    mimetype: "text/gemini".to_owned(),
                    charset: "utf-8".to_owned(),
                    body: format!("Status {} is currently unhandled", status),
                }
            }
        }
    }
}
