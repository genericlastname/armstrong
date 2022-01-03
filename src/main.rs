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

fn main() {
    // println!("Hello I'm not setup yet, try running `cargo test`");
    let mut client = Client::new();
    client.display();
    client.url_dialog();
    client.run();
}
