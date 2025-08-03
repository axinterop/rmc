mod parsers;
mod tokenizer;
mod visitors;

use crate::parsers::{BaseParser, Parser};
use crate::tokenizer::Tokenizer;
use crate::visitors::{BaseVisitor, Visitor};

fn main() {
    let markdown = "__Foo__ and *bar*.\n\nAnother paragraph.";
    let mut tokens = Tokenizer::tokenize(markdown);
    let node = Parser::match_tokens(&mut tokens).unwrap();
    let html = Visitor::visit(node);
    println!("{}", html);
}
