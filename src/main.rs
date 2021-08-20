fn main() {
    let r: Response = visit::visit("gemini://gemini.conman.org", "1965");
    println!("{}", r.body);
}
