use std::time::SystemTime;

use cursive::utils::markup::StyledString;
use url::Url;

use crate::gemtext::{GemtextToken, parse_gemtext};
use crate::transaction::visit::visit;

struct History {
    url: Url,
    timestamp: SystemTime,
}

impl History {
    fn new(&self, url: &str) -> History {
        History {
            url: Url::parse(url).unwrap(),  // TODO: remove unwrap().
            timestamp: SystemTime::now(),
        }
    }
}

pub struct Client {
    history: Vec<History>,
    content: Vec<StyledString>,
    urls: Vec<Url>,
    current_tab: usize,
}

impl Client {
    pub fn new() -> Client {
        Client {
            history: Vec::new(),
            content: Vec::new(),
            urls: Vec::new(),
            current_tab: 0,
        }
    }

    pub fn goto(&mut self, s: &str) {
        let url = Url::parse_with_params(s,
            &[
            ("scheme", "gemini"),
            ("port", "1965")
            ]).unwrap();
        self.urls[self.current_tab] = url.clone();
        let response = visit(url);
        let chain = parse_gemtext(&response.body);
        let ss = styled_string_from_token_chain(&chain);
        self.content[self.current_tab] = ss;
    }
}

fn styled_string_from_token_chain(chain: &Vec<GemtextToken>) -> StyledString {
    let mut styled_page_text = StyledString::new();
    for token in chain {
        styled_page_text.append(token.styled_string());
    }
    styled_page_text
}

// pub fn goto_dialog(client: &mut Client, siv: &mut Cursive) {
//     fn goto(siv: &mut Cursive, s: &str) {
//         let response = crate::transaction::visit::visit(s).unwrap();
//         let chain = parse_gemtext(&response.body);
//         let ss = styled_string_from_token_chain(&chain);
//         siv.call_on_name("content", |view: &mut TextView| {
//             view.set_content(ss);
//         });
//         siv.pop_layer();
//     }

//     let layout = LinearLayout::vertical()
//         .child(DummyView)
//         .child(TextView::new("Example: gemini.circumlunar.space"))
//         .child(EditView::new()
//             .on_submit(goto)
//             .with_name("urlbox"));

//     siv.add_layer(Dialog::around(layout)
//         .title("Enter URL")
//         .button("Visit", |s| {
//             let url = s.call_on_name("urlbox", |view: &mut EditView| {
//                 view.get_content()
//             }).unwrap();
//             goto(s, &url);
//         })
//         .button("Cancel", |s| {
//             s.pop_layer();
//         }));
// }

// Backbone functions.
// fn goto(client: &mut Client, s: &str) -> StyledString {
//     let response = visit(&s).unwrap();
//     let url = Url::parse(s).unwrap();

//     client.tabs[client.current_tab] = url;

//     let token_chain = parse_gemtext(&response.body);
//     client.styled_bodies[client.current_tab] = 
//         styled_string_from_token_chain(&token_chain.clone());
//     styled_string_from_token_chain(&token_chain.clone())
// }
