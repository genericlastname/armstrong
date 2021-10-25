pub mod gemtext;

pub mod transaction {
    pub mod dummy_verifier;
    pub mod response;
    pub mod visit;
}

pub mod ui {
    pub mod tui;
}

pub mod settings;

use cursive::{Cursive, CursiveExt};

fn main() {
    // println!("Hello I'm not setup yet, try running `cargo test`");
    let mut app = Cursive::new();
    ui::tui::configure_callbacks(&mut app);
    let r = transaction::visit::visit("gemini", "carcosa.net", "1965", "");
    ui::tui::client_screen(&mut app, &r);
    app.run();
}
