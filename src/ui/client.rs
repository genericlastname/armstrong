use std::collections::HashMap;
use std::time::SystemTime;

use cursive::{Cursive, CursiveExt};
use cursive::theme::{
    BorderStyle,
    BaseColor::*,
    Color::*,
    Effect,
    Palette,
    PaletteColor::*,
    // Style,
    Theme,
};
use cursive::traits::*;
use cursive::utils::markup::StyledString;
use cursive::view::{Margins, SizeConstraint};
use cursive::views::{
    Button,
    Dialog,
    DummyView,
    EditView,
    LinearLayout,
    PaddedView,
    Panel,
    ResizedView,
    ScrollView,
    TextView,
};

use crate::gemtext::GemtextToken;
use crate::transaction::response::{create_fake_response, Response};
use crate::transaction::visit as t_visit;

struct History {
    url: String,
    timestamp: SystemTime,
}

impl History {
    fn new(&self, url: &str) -> History {
        History {
            url: url.to_owned(),
            timestamp: SystemTime::now(),
        }
    }
}

pub struct Client {
    history: Vec<History>,
    responses: HashMap<String, Response>,
    tabs: Vec<String>,
}

fn styled_string_from_token_chain(chain: &Vec<GemtextToken>) -> StyledString {
    let mut styled_page_text = StyledString::new();
    for token in chain {
        styled_page_text.append(token.styled_string());
    }
    styled_page_text
}
