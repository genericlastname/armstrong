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

use crate::ui::client::Client;

pub struct Tui {
    client: Client,
    siv: Cursive,
}

impl Tui {
    pub fn new() -> Tui {
        let mut siv = Cursive::new();

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

        Tui {
            client: Client::new(),
            siv: siv,
        }
    }

    pub fn goto(&mut self) {
        fn submit(siv: &mut Cursive, s: &str) {
            siv.pop_layer();
        }

        let layout = LinearLayout::vertical()
            .child(DummyView)
            .child(TextView::new("Example: gemini.circumlunar.space"))
            .child(EditView::new()
                .on_submit(submit)
                .with_name("urlbox"));

        self.siv.add_layer(Dialog::around(layout)
            .title("Enter URL")
            .button("Visit", |s| {
                let url = s.call_on_name("urlbox", |view: &mut EditView| {
                    view.get_content()
                }).unwrap();
                submit(s, &url);
            })
            .button("Cancel", |s| {
                s.pop_layer();
            }));
    }

    pub fn run(&mut self) {
        self.siv.run();
    }
}
