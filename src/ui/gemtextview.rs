use cursive::Printer;
use cursive::view::View;
use url::Url;

use crate::gemtext::{GemtextToken, parse_gemtext};
use crate::transaction::visit::visit;

pub struct GemtextView {
    chain: Vec<GemtextToken>,
    url: Url,
}

impl GemtextView {
    pub fn new(url: Url) -> Self {
        let response = visit(&url);
        GemtextView {
            chain: parse_gemtext(&response.body),
            url: url,
        }
    }

    pub fn set_url(&mut self, url: Url) {
        let response = visit(&url);
        self.chain = parse_gemtext(&response.body);
        self.url = url;
    }

    pub fn get_token_chain(&self) -> &Vec<GemtextToken> {
        &self.chain
    }
}

impl View for GemtextView {
    fn draw(&self, printer: &Printer) {

    }
}
