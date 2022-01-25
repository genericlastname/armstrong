use std::io::{BufRead, BufReader};

use cursive::theme::{Effect, Style};
use cursive::utils::markup::StyledString;

#[derive(Copy, Clone, Debug, PartialEq)]
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

#[derive(Clone)]
pub struct GemtextToken {
    pub kind: TokenKind,
    pub data: String,
    pub extra: String,  // Right now this will be empty except when links are
                        // named, when it will hold the user friendly name.
}

impl GemtextToken {
    pub fn styled_string(&self) -> StyledString {
        let styled_string: StyledString;
        let data_copy = self.data.clone();

        match self.kind {
            TokenKind::Link => {
                // Don't need the cloned data since format! Copies anyway.
                let disp: String;
                if self.extra.is_empty() {
                    disp = format!("→ {}", self.data);
                } else {
                    disp = format!("→ {}", self.extra);
                }
                styled_string = StyledString::styled(disp,
                    Style::from(Effect::Underline));
            },
            TokenKind::Heading => {
                // TODO: figure out how to combine effects.
                // let effect = Effect::Underline & Effect::Bold;
                styled_string = StyledString::styled(data_copy.to_uppercase(),
                    Style::from(Effect::Bold));
            },
            TokenKind::SubHeading => {
                let mut style = Style::default();
                style.effects = Effect::Underline & Effect::Bold;
                styled_string = StyledString::styled(data_copy,
                    style);
            },
            TokenKind::SubSubHeading => {
                styled_string = StyledString::styled(data_copy,
                    Style::from(Effect::Bold));
            },
            TokenKind::PreFormattedText => {
                styled_string = StyledString::styled(data_copy,
                    Style::default());
            },
            _ => {
                styled_string = StyledString::styled(data_copy,
                    Style::default());
            }
        }

        styled_string
    }
}

// Returns a Vec<&str> from a given str with newline and linefeed bytes
// maintained.
fn split_keep_crlf(raw_text: &str) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    let mut buflen: usize;
    let mut current: String = String::new();
    let mut reader = BufReader::new(raw_text.as_bytes());
    loop {
        buflen = reader.read_line(&mut current)
            .expect("Couldn't read buffer.");
        if buflen == 0 { break; }
        let copy = current.clone();
        current.clear();
        lines.push(copy);
    }
    lines
}

// Take in a string of gemtext and convert it into a vector of GemtextTokens
// with a kind and data.
pub fn parse_gemtext(raw_text: &str) -> Vec<GemtextToken> {
    let mut gemtext_token_chain = Vec::new();
    let raw_text_lines = split_keep_crlf(raw_text);
    let mut current_pft_state = false;
    let mut pft_block = String::new();
    let mut pft_alt_text = String::new();

    for line in raw_text_lines {
        if !current_pft_state {
            if line.starts_with("=>") {
                // LINKS
                let link_parts: Vec<&str>
                    = line[2..].trim().splitn(2, ' ').collect();
                let data = link_parts[0];
                let extra;
                if link_parts.len() == 2 { extra = link_parts[1]; }
                else { extra = "" }

                gemtext_token_chain.push(GemtextToken {
                    kind: TokenKind::Link,
                    data: data.to_owned(),
                    extra: extra.to_owned(),
                });
            } else if line.starts_with("* ") {
                // UNORDERED LISTS
                gemtext_token_chain.push(GemtextToken {
                    kind: TokenKind::UnorderedList,
                    data: line[2..].to_owned(),
                    extra: "".to_owned(),
                });
            } else if line.starts_with(">") {
                // BLOCKQUOTES
                gemtext_token_chain.push(GemtextToken {
                    kind: TokenKind::Blockquote,
                    data: line[1..].trim_start().to_owned(),
                    extra: "".to_owned(),
                });
            } else if line.starts_with("### ") {
                // SUBSUBHEADING
                gemtext_token_chain.push(GemtextToken {
                    kind: TokenKind::SubSubHeading,
                    data: line[4..].to_owned(),
                    extra: "".to_owned(),
                });
            } else if line.starts_with("## ") {
                // SUBHEADING
                gemtext_token_chain.push(GemtextToken {
                    kind: TokenKind::SubHeading,
                    data: line[3..].to_owned(),
                    extra: "".to_owned(),
                });
            } else if line.starts_with("# ") {
                // HEADING
                gemtext_token_chain.push(GemtextToken {
                    kind: TokenKind::Heading,
                    data: line[2..].to_owned(),
                    extra: "".to_owned(),
                });
            } else if line.starts_with("```") {
                // PREFORMATTED TEXT
                current_pft_state = true;
                pft_alt_text = line[3..].trim().to_owned();
            } else {
                // TEXT
                gemtext_token_chain.push(GemtextToken {
                    kind: TokenKind::Text,
                    data: line.to_owned(),
                    extra: "".to_owned(),
                });
            }
        } else {
            if line.starts_with("```") {
                gemtext_token_chain.push(GemtextToken {
                    kind: TokenKind::PreFormattedText,
                    data: pft_block.clone(),
                    extra: pft_alt_text.clone(),
                });
                pft_block.clear();
                pft_alt_text.clear();
                current_pft_state = false;
            } else {
                pft_block.push_str(&line);
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
    fn parser_handles_links_without_space() {
        let raw_text_without_space = "=>www.example.com";
        let text_data = "www.example.com";
        let parsed_without_space: Vec<GemtextToken> =
            parse_gemtext(raw_text_without_space);
        assert_eq!(parsed_without_space.len(), 1);
        assert_eq!(parsed_without_space[0].kind, TokenKind::Link);
        assert_eq!(parsed_without_space[0].data, text_data);
    }

    #[test]
    fn parser_handles_links_with_space() {
        let raw_text_with_space = "=> www.example.com";
        let text_data = "www.example.com";
        let parsed_with_space: Vec<GemtextToken> =
            parse_gemtext(raw_text_with_space);
        assert_eq!(parsed_with_space.len(), 1);
        assert_eq!(parsed_with_space[0].kind, TokenKind::Link);
        assert_eq!(parsed_with_space[0].data, text_data);
    }

    #[test]
    fn parser_handles_links_with_names() {
        let raw_text_with_space = "=> www.example.com Example Link";
        let raw_text_without_space = "=>www.example.com Example Link";
        let text_data = "www.example.com";
        let extra_data = "Example Link";
        let parsed_with_space: Vec<GemtextToken>
            = parse_gemtext(raw_text_with_space);
        assert_eq!(parsed_with_space.len(), 1);
        assert_eq!(parsed_with_space[0].kind, TokenKind::Link);
        assert_eq!(parsed_with_space[0].data, text_data);
        assert_eq!(parsed_with_space[0].extra, extra_data);

        let parsed_without_space: Vec<GemtextToken>
            = parse_gemtext(raw_text_without_space);
        assert_eq!(parsed_without_space.len(), 1);
        assert_eq!(parsed_without_space[0].kind, TokenKind::Link);
        assert_eq!(parsed_without_space[0].data, text_data);
        assert_eq!(parsed_without_space[0].extra, extra_data);
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
        let line0 = "Heading\n";
        let line1 = "SubHeading\n";
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
            "```alt text\n\
            This text is unformatted.\n\
            This is the second line.\n\
            ```";
        let line = "This text is unformatted.\nThis is the second line.\n";
        let alt_text = "alt text";
        let parsed: Vec<GemtextToken> = parse_gemtext(raw_text);
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].kind, TokenKind::PreFormattedText);
        assert_eq!(parsed[0].data, line);
        assert_eq!(parsed[0].extra, alt_text);
    }
}
