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
use cursive::utils::markup::StyledString;
use cursive::view::{Margins, SizeConstraint};
use cursive::views::{
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

pub struct Client {
    app: Cursive,
    current_tab: u8,
    responses: Vec<Response>,
    tabs: Vec<StyledString>,
    titles: Vec<String>,
    theme: Theme,
}

impl Client {
    pub fn new() -> Client {
        let mut app = Cursive::new();

        // Callbacks and events.
        app.add_global_callback('q', |s| s.quit());

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

        let mut c = Client {
            app: app,
            current_tab: 0,
            responses: Vec::new(),
            tabs: Vec::new(),
            titles: Vec::new(),
            theme: theme,
        };
        c.new_tab();
        c
    }

    pub fn display(&mut self) {
        self.app.set_theme(self.theme.clone());
        self.app.add_layer(
            LinearLayout::vertical()
            .child(self.header_view())
            .child(self.page_view())
        );
    }

    pub fn run(&mut self) {
        self.app.run();
    }

    // Tab functions.
    fn new_tab(&mut self) {
        self.titles.push("New tab".to_owned());
        self.tabs.push(StyledString::from("New tab created."));
        self.responses.push(create_fake_response(20, "New tab"));
    }

    fn next_tab(&mut self) {
        if self.current_tab + 1 < self.tabs.len() as u8 { self.current_tab += 1; }
        else { self.current_tab = 0; }
    }

    fn prev_tab(&mut self) {
        if self.current_tab - 1 < self.tabs.len() as u8 { self.current_tab -= 1; }
        else { self.current_tab = self.tabs.len() as u8; }
    }

    // Views.
    fn header_view(&self) -> impl cursive::View {
        PaddedView::new(
            Margins::lrtb(1, 0, 0, 0),
            // TODO: Remove deprecated function below.
            TextView::new(self.titles[self.current_tab as usize].clone())
            .effect(Effect::Reverse))
    }

    fn page_view(&self) -> impl cursive::View {
        Panel::new(PaddedView::new(
                Margins::lrtb(4, 4, 1, 1),
                ScrollView::new(ResizedView::new(
                        SizeConstraint::Fixed(100),
                        SizeConstraint::Full,
                        TextView::new(
                            self.tabs[self.current_tab as usize].clone()
                        )))
                .scroll_y(true)))
    }

    // Dialogs.
    pub fn url_dialog(&mut self) {
        // self.app.add_layer(Dialog::around(TextArea::new())
        //     .title("Enter URL")
        //     .button("Ok", |s| s.quit()));

        self.app.add_layer(
            Dialog::around(
                LinearLayout::vertical()
                .child(DummyView)
                .child(TextView::new("Example: gemini.circumlunar.space"))
                .child(EditView::new()
                    .on_submit(visit)
                )
                // .child(LinearLayout::horizontal()
                //     .child(TextView::new("gemini://"))
                //     .child(EditView::new()
                //         .on_submit(visit)))
            )
            .title("Enter URL")
            .button("Go", |s| {
              s.noop() 
            })
            .dismiss_button("Cancel"));
    }
}

// Funcs to do actions
fn visit(s: &mut Cursive, url: &str) {
    // Split url in elements
}

fn styled_string_from_token_chain(chain: &Vec<GemtextToken>) -> StyledString {
    let mut styled_page_text = StyledString::new();
    for token in chain {
        styled_page_text.append(token.styled_string());
    }
    styled_page_text
}
