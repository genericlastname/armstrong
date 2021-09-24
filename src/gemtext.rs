#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Text,
    Link,
    UnorderedList,
    Blockquote,
    Heading,
    SubHeading,
    SubSubHeading,
    PreFormattedText,
}

pub struct GemtextToken {
    pub kind: TokenKind,
    pub data: String,
}

impl std::fmt::Display for GemtextToken {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(\n\t{:?}\n\t{}\n)", self.kind, self.data)
    }
}

// Take in a string of gemtext and convert it into a vector of GemtextTokens
// with a kind and data.
pub fn parse_gemtext(raw_text: &str) -> Vec<GemtextToken> {
    let mut gemtext_token_chain = Vec::new();
    let raw_text_lines: Vec<&str> = raw_text.split("\n").collect();

    for line in raw_text_lines {
        let token_data: String;
        let mode: TokenKind;
        let text_tokens: Vec<&str> = line.splitn(2, ' ').collect();
        match text_tokens[0] {
            "=>"  => { mode = TokenKind::Link; },
            "*"   => { mode = TokenKind::UnorderedList; },
            ">"   => { mode = TokenKind::Blockquote; },
            "###" => { mode = TokenKind::SubSubHeading; },
            "##"  => { mode = TokenKind::SubHeading; },
            "#"   => { mode = TokenKind::Heading; },
            // TODO: Support finding both sides of PreFormattedText.
            _     => { mode = TokenKind::Text; },
        }

        if mode == TokenKind::Text && text_tokens.len() > 1{
            token_data = format!("{} {}", text_tokens[0], text_tokens[1]);
        } else if mode != TokenKind::Text && text_tokens.len() > 1 {
            token_data = text_tokens[1].to_owned();
        } else {
            token_data = text_tokens[0].to_owned();
        }

        gemtext_token_chain.push(GemtextToken {
            kind: mode,
            data: token_data,
        });
    }
    
    gemtext_token_chain
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_handles_text() {
        let text = "Hello world this is example text";
        let parsed: Vec<GemtextToken> = parse_gemtext(text);
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].kind, TokenKind::Text);
        assert_eq!(parsed[0].data, text);
    }

    #[test]
    fn parser_handles_links() {
        let raw_text = "=> www.example.com";
        let text_data = "www.example.com";
        let parsed: Vec<GemtextToken> = parse_gemtext(raw_text);
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].kind, TokenKind::Link);
        assert_eq!(parsed[0].data, text_data);
    }

    #[test]
    fn parser_handles_lists() {
        let raw_text = "* Item";
        let text_data = "Item";  // The text data after parsing.
        let parsed: Vec<GemtextToken> = parse_gemtext(raw_text);
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].kind, TokenKind::UnorderedList);
        assert_eq!(parsed[0].data, text_data);
    }

    #[test]
    fn parser_handles_blockquotes() {
        let raw_text = "> block quote";
        let text_data = "block quote";
        let parsed: Vec<GemtextToken> = parse_gemtext(raw_text);
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].kind, TokenKind::Blockquote);
        assert_eq!(parsed[0].data, text_data);
    }

    #[test]
    fn parser_handles_headings() {
        let raw_text =
            "\
            # Heading\n\
            ## SubHeading\n\
            ### SubSubHeading";
        let line0 = "Heading";
        let line1 = "SubHeading";
        let line2 = "SubSubHeading";
        let parsed: Vec<GemtextToken> = parse_gemtext(raw_text);
        assert_eq!(parsed.len(), 3);
        assert_eq!(parsed[0].kind, TokenKind::Heading);
        assert_eq!(parsed[1].kind, TokenKind::SubHeading);
        assert_eq!(parsed[2].kind, TokenKind::SubSubHeading);
        assert_eq!(parsed[0].data, line0);
        assert_eq!(parsed[1].data, line1);
        assert_eq!(parsed[2].data, line2);
    }
}
