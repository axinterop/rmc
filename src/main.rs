mod markdown;
mod parsers;
mod tokenizer;
mod visitors;

use crate::markdown::Markdown;

fn main() {
    let markdown = "__Foo__ and *bar*.\nAnother paragraph.";
    let html = Markdown::parse_and_save(markdown, "./output.html");
}
