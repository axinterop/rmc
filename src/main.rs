mod markdown;
mod parsers;
mod tokenizer;
mod visitors;

use crate::markdown::Markdown;

fn main() {
    let markdown = "__Foo__ and *bar*.\n\nAnother paragraph.";
    let _html = Markdown::parse_and_save(markdown, "./output.html");
}
