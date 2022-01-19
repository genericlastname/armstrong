use cursive::Cursive;
use cursive::event;
use cursive::theme::{
    BorderStyle,
    BaseColor::*,
    Color::*,
    Palette,
    PaletteColor::*,
    // Style,
    Theme,
};
use cursive::traits::{Nameable, Scrollable};
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
    let page_view = PaddedView::lrtb(
        0, 4, 0, 0,
        TextView::new("New tab").with_name("page"))
        .scrollable();

    let ui_view = LinearLayout::vertical()
        .child(Panel::new(
                PaddedView::lrtb(
                    4, 0, 1, 1,
                    ResizedView::with_max_width(100, page_view))));

    let event_view = OnEventView::new(ui_view)
        .on_event('q', |s| quit_dialog(s))
        .on_event(event::Key::Esc, |s| quit_dialog(s))
        .on_event(event::Event::Char('g'), |s: &mut Cursive| goto_dialog(s));

    app.add_layer(event_view);
    goto_dialog(&mut app);
    app
}

fn update_tab(app: &mut Cursive, s: &str) {
    let url = Url::parse(&s).unwrap();
    let response = visit(&url);
    let chain = parse_gemtext(&response.body);
    let ss = styled_string_from_token_chain(&chain);

    let mut page = app.find_name::<TextView>("page").unwrap();
    page.set_content(ss);
    app.pop_layer();
}

fn goto_dialog(app: &mut Cursive) {
    let layout = LinearLayout::vertical()
        .child(DummyView)
        .child(TextView::new("Example: gemini.circumlunar.space"))
        .child(EditView::new()
            .on_submit(update_tab)
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
        .on_event(event::Key::Esc, |s| {
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
        .on_event(event::Key::Esc, |s| { s.pop_layer(); })
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
