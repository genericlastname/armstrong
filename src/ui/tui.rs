use cursive::Cursive;
use cursive::event;
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
// use cursive::traits::*;
use cursive::view::{Nameable, Margins, SizeConstraint};
use cursive::utils::markup::StyledString;
use cursive::views::{
    Dialog,
    DummyView,
    EditView,
    LinearLayout,
    OnEventView,
    PaddedView,
    Panel,
    ResizedView,
    ScrollView,
    TextView,
};
use url::Url;

use crate::transaction::visit::visit;
use crate::gemtext::{GemtextToken, parse_gemtext};

pub fn init_ui() -> Cursive {
    let mut app = Cursive::new();

    let mut palette = Palette::default();
    let colors = vec![
        (Background, Rgb(0, 0, 0)),
        (View, Rgb(0, 0, 0)),
        (Primary, Light(White)),
        (Secondary, Light(White)),
        (TitlePrimary, Light(White)),
    ];
    palette.extend(colors);
    let theme = Theme {
        shadow: false,
        borders: BorderStyle::Simple,
        palette: palette,
    };
    app.set_theme(theme);

    // Create default layout
    let page_view = PaddedView::new(
        Margins::lrtb(4, 4, 1, 1),
        ScrollView::new(
            ResizedView::new(
                SizeConstraint::Fixed(100),
                SizeConstraint::Full,
                TextView::new("New tab")
                .with_name("page")
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
        .child(Panel::new(page_view));

    let event_view = OnEventView::new(ui_view)
        .on_event('q', |s| s.quit())
        .on_event(event::Event::Char('g'), |s: &mut Cursive|{
            s.add_layer(Dialog::around(goto_dialog_layout())
                .title("Enter URL")
                .button("Visit", |t: &mut Cursive| {
                    let url = t.call_on_name("urlbox", |view: &mut EditView| {
                        view.get_content()
                    }).unwrap();
                    goto(t, &url);
                })
                .dismiss_button("Cancel"))
        });

    app.add_layer(event_view);
    app
}

pub fn goto(app: &mut Cursive, s: &str) {
    let url = Url::parse(&s).unwrap();
    let response = visit(&url);
    let chain = parse_gemtext(&response.body);
    let ss = styled_string_from_token_chain(&chain);

    let mut page = app.find_name::<TextView>("page").unwrap();
    page.set_content(ss);
    app.pop_layer();
}

pub fn goto_dialog_layout() -> LinearLayout {
    LinearLayout::vertical()
        .child(DummyView)
        .child(TextView::new("Example: gemini.circumlunar.space"))
        .child(EditView::new()
            .on_submit(goto)
            .with_name("urlbox"))
}


// Helper funcs
fn styled_string_from_token_chain(chain: &Vec<GemtextToken>) -> StyledString {
    let mut styled_page_text = StyledString::new();
    for token in chain {
        styled_page_text.append(token.styled_string());
    }
    styled_page_text
}
