use cursive::align::Align;
use cursive::Cursive;
use cursive::event;
use cursive::views::{LinearLayout, ResizedView, ScrollView, StackView, TextView};

use crate::transaction::response::Response;
// use crate::transaction::visit::visit;

pub fn configure_callbacks(app: &mut Cursive) {
    app.add_global_callback('q', |s| s.quit());
}

pub fn client_screen(s: &mut Cursive, r: &Response) {
    let text_area = ScrollView::new(TextView::new(&r.body))
        .scroll_y(true);
    let sized_view = ResizedView::with_fixed_size((200, 50), text_area);
    // let keybind_area = TextView::new(
    //     "Scroll: j/k, Quit: q"
    // );
    // let view = OnEventView::new(text_area)
    //     .on_event('j', |s| s.scroll_to_bottom());
    s.add_layer(
        LinearLayout::vertical()
            .child(sized_view));
}
