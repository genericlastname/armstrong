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
use cursive::view::{Nameable, Margins, SizeConstraint};
use cursive::views::{
    Button,
    Dialog,
    DummyView,
    EditView,
    LinearLayout,
    ListView,
    PaddedView,
    Panel,
    ResizedView,
    ScrollView,
    TextView,
};
use url::{Url, ParseError};

use crate::gemtext::GemtextToken;
use crate::transaction::response::{create_fake_response, Response};
use crate::transaction::visit as t_visit;

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
    responses: HashMap<Url, Response>,
    tabs: Vec<Url>,
    current_tab: usize,
}

impl Client {
    pub fn new(siv: &mut Cursive) -> Client {
        let mut palette = Palette::default();
        let colors = vec![
            (Background, Rgb(0, 0, 0)),
            (View, Rgb(0, 0, 0)),
            (Primary, Light(White))
        ];
        palette.extend(colors);
        let theme = Theme {
            shadow: false,
            borders: BorderStyle::Simple,
            palette: palette,
        };
        siv.set_theme(theme);

        // Create default layout
        let content_view = PaddedView::new(
            Margins::lrtb(4, 4, 1, 1),
            ScrollView::new(
                ResizedView::new(
                    SizeConstraint::Fixed(100),
                    SizeConstraint::Full,
                    TextView::new("New tab")
                    .with_name("content")
                )
            )
        );

        let ui_view = LinearLayout::vertical()
            .child(PaddedView::new(
                    Margins::lr(1, 0),
                    LinearLayout::horizontal()
                    .child(TextView::new("New tab"))
                    .with_name("tab_bar")
            ))
            .child(Panel::new(content_view));

        siv.add_layer(ui_view);

        Client {
            history: Vec::new(),
            responses: HashMap::new(),
            tabs: Vec::new(),
            current_tab: 0,
        }
    }

}

fn styled_string_from_token_chain(chain: &Vec<GemtextToken>) -> StyledString {
    let mut styled_page_text = StyledString::new();
    for token in chain {
        styled_page_text.append(token.styled_string());
    }
    styled_page_text
}
