use gemtext::parse_gemtext;
// use transaction::response::Response;
// use transaction::visit::visit;

pub mod gemtext;

pub mod transaction {
    pub mod response;
    pub mod visit;
}

fn main() {
    let tokens: Vec<gemtext::GemtextToken> = parse_gemtext("Hello world\n# Welcome to Rust\n");
    for token in tokens {
        println!("{}", token);
    }
}
