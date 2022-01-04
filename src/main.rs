pub mod gemtext;

pub mod transaction {
    pub mod dummy_verifier;
    pub mod response;
    pub mod visit;
}

pub mod ui {
    pub mod client;
}

pub mod settings;

use ui::client::Client;
use cursive::{Cursive, CursiveExt};

fn main() {
    // println!("Hello I'm not setup yet, try running `cargo test`");
    let mut siv = Cursive::new();
    let mut client = Client::new(&mut siv);
    siv.run();
}
