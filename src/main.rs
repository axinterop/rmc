struct Token {
    type_: String,
    value: String,
}

impl Token {
    fn new(type_: String, value: String) -> Token {
        Token { type_, value }
    }
    fn len(&self) -> usize {
        self.value.len()
    }

    fn end_of_file() -> Token {
        Token {
            type_: String::from("EOF"),
            value: String::from(""),
        }
    }

    fn dummy() -> Token {
        Token {
            type_: String::from("Dummy"),
            value: String::from("x"),
        }
    }
}

struct Tokenizer;

// Tokenizer: take a markdown string and return list of Token objects

impl Tokenizer {
    pub fn tokenize(plain_markdown: String) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut draft_markdown = plain_markdown.clone();
        while !draft_markdown.is_empty() {
            let token: Token = Self::scan_one_token(&draft_markdown);
            draft_markdown.truncate(draft_markdown.len() - token.len());
            tokens.push(token);
        }
        tokens.push(Token::end_of_file());
        tokens
    }

    fn scan_one_token(markdown: &String) -> Token {
        Token::dummy()
    }
}

fn main() {
    let markdown = String::from("*Hello*");
    let tokens = Tokenizer::tokenize(markdown);
    for i in 0..tokens.len() {
        println!("{i}: {}", tokens[i].value);
    }
}
