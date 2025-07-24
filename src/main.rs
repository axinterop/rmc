struct Token {
    type_: TokenType,
    value: String,
}

impl Token {
    fn new(type_: TokenType, value: String) -> Token {
        Token { type_, value }
    }
    fn len(&self) -> usize {
        self.value.len()
    }

    fn end_of_file() -> Token {
        Token {
            type_: TokenType::Eof,
            value: String::from(""),
        }
    }

    fn dummy() -> Token {
        Token {
            type_: TokenType::Dummy,
            value: String::from("x"),
        }
    }
}

enum TokenType {
    Eof,
    Dummy, // temporary
    // Simple
    Underscore,
    Star,
    Newline,
}

struct Tokenizer;
struct SimpleScanner;
struct TextScanner;

trait Scanner {
    fn scan(input: &str) -> Option<Token>;
}

impl Scanner for SimpleScanner {
    fn scan(input: &str) -> Option<Token> {
        let first_char = input.chars().nth(0)?;
        let token_type = match first_char {
            '_' => TokenType::Underscore,
            '*' => TokenType::Star,
            '\n' => TokenType::Newline,
            _ => return None,
        };
        Some(Token {
            type_: token_type,
            value: first_char.to_string(),
        })
    }
}

impl Tokenizer {
    pub fn tokenize(plain_markdown: String) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut draft_markdown = plain_markdown.clone();
        while !draft_markdown.is_empty() {
            let token = Self::scan_one_token(&draft_markdown);
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
