use cursive::Cursive;
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
use cursive::utils::markup::StyledString;

use crate::transaction::response::Response;

pub struct Client {
    app: Cursive,
    responses: Vec<Response>,
    tabs: Vec<StyledString>,
    titles: Vec<String>,
    theme: Theme,
}

impl Client {
    pub fn new(&self) -> Client {
        let app = Cursive::new();

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

        Client {
            app: app,
            responses: Vec::new(),
            tabs: Vec::new(),
            titles: Vec::new(),
            theme: theme,
        }
    }
}
