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
    pub extra: String,  // Right now this will be empty except when links are
                        // named, when it will hold the user friendly name.
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
    let mut curr_pft_state: bool = false;
    let mut prev_pft_state: bool = false;

    for line in raw_text_lines {
        let mode: TokenKind;
        let text_tokens: Vec<&str> = line.splitn(3, ' ').collect();

        match text_tokens[0] {
            "=>"  => { mode = TokenKind::Link; },
            "*"   => { mode = TokenKind::UnorderedList; },
            ">"   => { mode = TokenKind::Blockquote; },
            "###" => { mode = TokenKind::SubSubHeading; },
            "##"  => { mode = TokenKind::SubHeading; },
            "#"   => { mode = TokenKind::Heading; },
            "```" => { 
                curr_pft_state = !curr_pft_state;
                prev_pft_state = false;
                mode = TokenKind::PreFormattedText;
            },
            _     => {
                if curr_pft_state {
                    mode = TokenKind::PreFormattedText;
                } else {
                    mode = TokenKind::Text;
                }
            },
        }

        if !curr_pft_state {
            match text_tokens.len() {
                3 => {
                    if mode == TokenKind::Link {
                        gemtext_token_chain.push(GemtextToken {
                            kind: mode,
                            data: text_tokens[1].to_owned(),
                            extra: text_tokens[2].to_owned(),
                        });
                    } else if mode == TokenKind::Text {
                        // Combine [0], [1], and [2] since Text doesn't have a leading symbol.
                        gemtext_token_chain.push(GemtextToken {
                            kind: mode,
                            data: format!("{} {} {}",
                                text_tokens[0],
                                text_tokens[1],
                                text_tokens[2]),
                            extra: "".to_owned(),
                        });
                    } else {
                        // Combine [1] and [2] in other parse modes.
                        gemtext_token_chain.push(GemtextToken {
                            kind: mode,
                            data: format!("{} {}", text_tokens[1], text_tokens[2]),
                            extra: "".to_owned(),
                        });
                    }
                },
                2 => {
                    gemtext_token_chain.push(GemtextToken {
                        kind: mode,
                        data: text_tokens[1].to_owned(),
                        extra: "".to_owned(),
                    });
                },
                _ => {},
            }
        } else {
            if prev_pft_state {
                gemtext_token_chain.push(GemtextToken {
                    kind: mode,
                    data: format!("{} {} {}", text_tokens[0], text_tokens[1], text_tokens[2]),
                    extra: "".to_owned(),
                });
            } else {
                prev_pft_state = true;
            }
        }
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
    fn parser_handles_links_with_names() {
        let raw_text = "=> www.example.com Example Link";
        let text_data = "www.example.com";
        let extra_data = "Example Link";
        let parsed: Vec<GemtextToken> = parse_gemtext(raw_text);
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].kind, TokenKind::Link);
        assert_eq!(parsed[0].data, text_data);
        assert_eq!(parsed[0].extra, extra_data);
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

    #[test]
    fn parser_handles_pft() {
        let raw_text =
            "```\n\
            This text is unformatted.\n\
            This is the second line.\n\
            ```";
        let line0 = "This text is unformatted.";
        let line1 = "This is the second line.";
        let parsed: Vec<GemtextToken> = parse_gemtext(raw_text);
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].kind, TokenKind::PreFormattedText);
        assert_eq!(parsed[1].kind, TokenKind::PreFormattedText);
        assert_eq!(parsed[0].data, line0);
        assert_eq!(parsed[1].data, line1);
    }
}
