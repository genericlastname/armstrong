use cursive::Cursive;
use cursive::view::{Nameable, Resizable, Scrollable};
use cursive::views::{BoxView, Dialog, LinearLayout, TextView, ScrollView};

use crate::transaction::response::Response;
use crate::transaction::visit::visit;

pub fn configure_callbacks(app: &mut Cursive) {
    app.add_global_callback('q', |s| s.quit());
}

pub fn client_screen(
    s: &mut Cursive,
    response: &Response,
) {
    let text_area = ScrollView::new(TextView::new(&response.body));
    s.add_layer(LinearLayout::vertical()
        .child(text_area));
}
