// use gemtext::parse_gemtext;
// use transaction::response::Response;
// use transaction::visit::visit;

pub mod gemtext;

pub mod transaction {
    pub mod dummy_verifier;
    pub mod response;
    pub mod visit;
}

fn main() {
    // println!("Hello I'm not setup yet, try running `cargo test`");
    let r: transaction::response::Response = transaction::visit::visit(
        "gemini",
        "carcosa.net",
        "1965",
        "");
    println!("{}", r.status);
    println!("{}", r.mimetype);
    println!("{}", r.body);
}
