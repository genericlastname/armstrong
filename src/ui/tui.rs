// use cursive::align::Align;
use cursive::Cursive;
// use cursive::event;
// use cursive::theme::{Palette, Theme, Color::*};
use cursive::theme::Theme;
use cursive::utils::markup::StyledString;
use cursive::view::SizeConstraint;
use cursive::views::{
    LinearLayout,
    PaddedView,
    ResizedView,
    ScrollView,
    TextView,
};

use crate::transaction::response::Response;
use crate::gemtext::{GemtextToken, parse_gemtext};
// use crate::transaction::visit::visit;

pub fn configure_callbacks(app: &mut Cursive) {
    app.add_global_callback('q', |s| s.quit());
}

pub fn client_screen(
    s: &mut Cursive,
    r: &Response,  // TODO: replace with GemtextTokenChain
    t: &Theme)
{
    let token_chain: Vec<GemtextToken> = parse_gemtext(&r.body);
    // Generate StyledString from token_chain
    let mut styled_page_text = StyledString::new();
    for token in token_chain {
        // styled_page_text.append(token.styled_string());
        // styled_page_text.append("\n");
        styled_page_text.append(token.styled_string());
    }

    let text_area = ScrollView::new(TextView::new(styled_page_text))
        .scroll_y(true);
    let sized_view = ResizedView::new(
        SizeConstraint::AtMost(100),
        SizeConstraint::AtMost(40),
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
