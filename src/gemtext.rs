#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Text,
    Link,
    UnorderedList,
    Blockquotes,
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
            ">"   => { mode = TokenKind::Blockquotes; },
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
