use transaction::response::Response;
use transaction::visit::visit;

pub mod transaction {
    pub mod response;
    pub mod visit;
}

fn main() {
    let r: Response = visit("gemini", "gemini.circumlunar.space", "1965", "");
    println!("{}", r.body);
}
