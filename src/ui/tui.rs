use cursive::Cursive;
use cursive::event::Key;
use cursive::theme::{
    BorderStyle,
    BaseColor::*,
    Color::*,
    Palette,
    PaletteColor::*,
    // Style,
    Theme,
};
use cursive::traits::{Nameable, Resizable, Scrollable};
use cursive::utils::markup::StyledString;
use cursive::view::SizeConstraint;
use cursive::view::scroll::Scroller;
use cursive::views::{
    BoxedView,
    Dialog,
    DummyView,
    EditView,
    LinearLayout,
    OnEventView,
    PaddedView,
    Panel,
    ResizedView,
    ScrollView,
    SelectView,
    TextView,
};
use url::Url;

use crate::transaction::visit::visit;
use crate::gemtext::{GemtextToken, parse_gemtext, TokenKind};

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
    let content = LinearLayout::vertical()
        .with_name("page")
        .scrollable()
        .with_name("scroll");
    

    let bordered_content = Panel::new(
        PaddedView::lrtb(
            2, 2, 1, 1,
            content)
        .resized(SizeConstraint::Full, SizeConstraint::Full));

    let current_link = TextView::new("")
        .with_name("current_link");

    let layout = LinearLayout::vertical()
        .child(bordered_content)
        .child(current_link);

    let event_view = OnEventView::new(layout)
        .on_event('q', |s| quit_dialog(s))
        .on_event(Key::Esc, |s| quit_dialog(s))
        .on_event('g', |s| goto_dialog(s));

    app.add_layer(event_view);
    goto_dialog(&mut app);
    app
}

fn update_tab(app: &mut Cursive, s: &str) {
    let url = Url::parse(&s).unwrap();
    let response = visit(&url);
    let chain = parse_gemtext(&response.body);
    let mut page = app.find_name::<LinearLayout>("page").unwrap();
    // page.clear();

    for token in chain {
        if token.kind == TokenKind::Link {
            let selectview = SelectView::new()
                .item(token.styled_string(), token.data)
                .on_submit(|s, item| {
                    update_tab(s, &item);
                })
            .on_select(|s, item| {
                s.call_on_name("current_link", |view: &mut TextView| {
                    view.set_content(item);
                });
            });
            page.add_child(selectview);
        } else {
            page.add_child(TextView::new(token.styled_string()));
        }
    }
}

fn goto_dialog(app: &mut Cursive) {
    let layout = LinearLayout::vertical()
        .child(DummyView)
        .child(TextView::new("Example: gemini.circumlunar.space"))
        .child(EditView::new()
            .on_submit(update_tab)
            .filler(" ")
            .content("gemini://")
            .with_name("urlbox"));

    app.add_layer(
        OnEventView::new(
            Dialog::around(layout)
            .title("Enter URL")
            .button("Visit", |t: &mut Cursive| {
                let url = t.call_on_name("urlbox", |view: &mut EditView| {
                    view.get_content()
                }).unwrap();
                update_tab(t, &url);
            })
            .dismiss_button("Cancel"))
        .on_event(Key::Esc, |s| {
            s.pop_layer();
        }));
}

fn quit_dialog(app: &mut Cursive) {
    let layout = LinearLayout::vertical()
        .child(DummyView)
        .child(TextView::new(
            "Are you sure you want to quit? (Press q again to exit)"
    ));

    app.add_layer(
        OnEventView::new(
            Dialog::around(layout)
            .title("Quit")
            .button("Quit", |s| s.quit())
            .dismiss_button("Cancel")
        )
        .on_event('q', |s| s.quit())
        .on_event(Key::Esc, |s| { s.pop_layer(); })
    );
}

// Helper funcs
fn styled_string_from_token_chain(chain: &Vec<GemtextToken>) -> StyledString {
    let mut styled_page_text = StyledString::new();
    for token in chain {
        styled_page_text.append(token.styled_string());
    }
    styled_page_text
}
