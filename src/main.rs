pub mod gemtext;

pub mod transaction {
    pub mod dummy_verifier;
    pub mod response;
    pub mod visit;
}

pub mod ui {
    pub mod client;
    pub mod tui;
}

pub mod settings;

use cursive::{Cursive, CursiveExt};

fn main() {
    // println!("Hello I'm not setup yet, try running `cargo test`");
    let mut app = Cursive::new();
    ui::tui::configure_callbacks(&mut app);
    let mut url = String::new();
    let r = transaction::visit::visit("gemini", "breadpunk.club", "1965", "", &mut url);
    ui::tui::test_screen(&mut app, &r, &url);
    app.run();
}
