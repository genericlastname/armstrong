use cursive::Cursive;
use cursive::theme::{
    BorderStyle,
    BaseColor::*,
    Color::*,
    Effect,
    Palette,
    PaletteColor::*,
    Style,
    Theme
};
use cursive::utils::markup::StyledString;
use cursive::view::{Margins, SizeConstraint};
use cursive::views::{
    LinearLayout,
    PaddedView,
    Panel,
    ResizedView,
    ScrollView,
    TextView,
};

use crate::transaction::response::Response;
use crate::gemtext::{GemtextToken, parse_gemtext};
// use crate::transaction::visit::visit;

fn styled_string_from_token_chain(chain: &Vec<GemtextToken>) -> StyledString {
    let mut styled_page_text = StyledString::new();
    for token in chain {
        styled_page_text.append(token.styled_string());
    }
    styled_page_text
}

pub fn configure_callbacks(app: &mut Cursive) {
    app.add_global_callback('q', |s| s.quit());
}

pub fn client_screen(
    s: &mut Cursive,
    r: &Response,
    _t: &Theme)
{
    let token_chain: Vec<GemtextToken> = parse_gemtext(&r.body);
    let styled_page_text = styled_string_from_token_chain(&token_chain);

    let text_area = ScrollView::new(TextView::new(styled_page_text))
        .scroll_y(true);
    let sized_view = ResizedView::new(
        SizeConstraint::Fixed(100),
        SizeConstraint::Full,
        text_area
    );
    let keybind_area = PaddedView::lrtb(
        0,
        0,
        2,
        0,
        TextView::new(
            "Scroll: j/k, Quit: q"
    ));
    // let view = OnEventView::new(text_area)
    //     .on_event('j', |s| s.scroll_to_bottom());
    s.add_layer(
        LinearLayout::vertical()
            .child(sized_view)
            .child(keybind_area));
}

pub fn test_screen(
    s: &mut Cursive,
    r: &Response,
    url: &str
)
{
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

    let token_chain: Vec<GemtextToken> = parse_gemtext(&r.body);
    let styled_page_text = styled_string_from_token_chain(&token_chain);

    // header view
    let header_view = PaddedView::new(
        Margins::lrtb(1, 0, 0, 0),
        TextView::new(url).effect(Effect::Reverse));

    // main page view
    let text_view = ResizedView::new(
        SizeConstraint::Fixed(100),
        SizeConstraint::Full,
        TextView::new(styled_page_text));
    let final_view = Panel::new(
        PaddedView::new(
            Margins::lrtb(4, 4, 1, 1),
            ScrollView::new(text_view).scroll_y(true)));

    s.set_theme(theme);
    s.add_layer(
        LinearLayout::vertical()
        .child(header_view)
        .child(final_view));
}
