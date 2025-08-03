use std::fs::File;
use std::io::Write;

use crate::parsers::*;
use crate::tokenizer::*;
use crate::visitors::*;

pub struct Markdown;

impl Markdown {
    pub fn parse(markdown: &str) -> String {
        let mut tokens = Tokenizer::tokenize(markdown);
        if let Some(node) = Parser::match_tokens(&mut tokens) {
            let html = Visitor::visit(node);
            return html;
        } else {
            panic!("Markdown::parse: node is None");
        }
    }
    pub fn parse_and_save(markdown: &str, path: &str) -> std::io::Result<()> {
        let html = Self::parse(markdown);
        let mut output = File::create(path)?;
        write!(output, "{}", html)
    }
}
