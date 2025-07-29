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

    fn eof() -> Token {
        Token {
            type_: TokenType::Eof,
            value: String::from("<EOF>"),
        }
    }

    fn dummy() -> Token {
        Token {
            type_: TokenType::Dummy,
            value: String::from("<DUMMY>"),
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
    // Complex
    Text,
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
        Some(Token::new(token_type, first_char.to_string()))
    }
}

impl Scanner for TextScanner {
    fn scan(input: &str) -> Option<Token> {
        let text = input
            .chars()
            .take_while(|&c| !matches!(c, '_' | '*' | '\n'))
            .collect::<String>();

        if text.is_empty() {
            None
        } else {
            Some(Token::new(TokenType::Text, text.to_string()))
        }
    }
}

impl Tokenizer {
    pub fn tokenize(plain_markdown: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut remaining = plain_markdown;
        while !remaining.is_empty() {
            if let Some(token) = Self::scan_one_token(remaining) {
                remaining = &remaining[token.len()..];
                tokens.push(token);
            } else {
                println!("Unknown character, stopping...");
                break;
            }
        }
        tokens.push(Token::eof());
        tokens
    }

    fn scan_one_token(input: &str) -> Option<Token> {
        SimpleScanner::scan(input).or_else(|| TextScanner::scan(input))
    }
}

fn main() {
    let markdown = "_Hello_";
    let tokens = Tokenizer::tokenize(markdown);
    for i in 0..tokens.len() {
        println!("{i}: {}", tokens[i].value);
    }
}
