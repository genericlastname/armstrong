enum TokenKind {
    Text,
    Link,
    UnorderedList,
    Blockquotes,
    Heading,
    SubHeading,
    SubSubHeading,
    PreFormattedText,
}

#[derive(Debug)]
pub struct GemtextToken {
    pub kind: TokenKind,
    pub data: &str,
}

pub fn parse_gemtext(raw_text: &str) -> Vec<GemtextToken> {
    let text_tokens: vec<&str> = raw_text.split(" ").collect();
    let mut gemtext_token_chain = Vec::new();
    let mut mode: TokenKind = TokenKind::Text;
    let mut current: &str;

    for token in text_tokens.iter() {
        match token {
            "=>"  => { mode = TokenKind::Link; },
            "*"   => { mode = TokenKind::UnorderedList; },
            ">"   => { mode = TokenKind::Blockquotes; },
            "###" => { mode = TokenKind::SubSubHeading; },
            "##"  => { mode = TokenKind::SubHeading; },
            "#"   => { mode = TokenKind::Heading; },
            // TODO: Support finding both sides of PreFormattedText.
            _     => {
                current = concat!(current, " ", token);
            }
        }
        // Tie off the current GemtextToken and append it to the chain at
        // paragraph boundary.
        if token == "\n" || token == "\r\n" {
            gemtext_token_chain.push(GemtextToken {
                kind: mode,
                data: current,
            });
            current = "";
            mode = TokenKind::Text;
        }
    }
}
