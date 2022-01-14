pub mod gemtext;

pub mod transaction {
    pub mod dummy_verifier;
    pub mod response;
    pub mod visit;
}

pub mod ui {
    pub mod browser;
    pub mod gemtextview;
    pub mod tui;
}

pub mod settings;

use cursive::CursiveExt;
use ui::tui::*;

fn main() {
    // println!("Hello I'm not setup yet, try running `cargo test`");
    let mut app = init_ui();
    app.run();
}
