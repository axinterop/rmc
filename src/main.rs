mod parsers;
mod tokenizer;

use crate::tokenizer::Tokenizer;

fn main() {
    let markdown = "_Hello_";
    let tokens = Tokenizer::tokenize(markdown);
    for i in 0..tokens.len() {
        println!("{i}: {}", tokens[i].value);
    }
}
