pub struct Token {
    pub type_: TokenType,
    pub value: String,
}

impl Token {
    pub fn new(type_: TokenType, value: String) -> Token {
        Token { type_, value }
    }
    pub fn len(&self) -> usize {
        self.value.len()
    }

    pub fn eof() -> Token {
        Token {
            type_: TokenType::Eof,
            value: String::from("<EOF>"),
        }
    }
}

#[derive(PartialEq)]
pub enum TokenType {
    Eof,
    // Simple
    Underscore,
    Star,
    Newline,
    // Complex
    Text,
}

pub struct Tokenizer;
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

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_vecs(vec1: &Vec<Token>, vec2: &Vec<Token>) {
        assert!(vec1.len() == vec2.len());
        for i in 0..vec1.len() {
            assert!(vec1[i].type_ == vec2[i].type_);
            assert!(vec1[i].value == vec2[i].value);
        }
    }

    #[test]
    fn emphesize_underscore_simple() {
        let input = "_Hello_";
        let result = Tokenizer::tokenize(input);
        let expected = vec![
            Token::new(TokenType::Underscore, "_".to_string()),
            Token::new(TokenType::Text, "Hello".to_string()),
            Token::new(TokenType::Underscore, "_".to_string()),
            Token::eof(),
        ];
        assert_vecs(&result, &expected);
    }

    #[test]
    fn emphesize_star_simple() {
        let input = "*Hello*";
        let result = Tokenizer::tokenize(input);
        let expected = vec![
            Token::new(TokenType::Star, "*".to_string()),
            Token::new(TokenType::Text, "Hello".to_string()),
            Token::new(TokenType::Star, "*".to_string()),
            Token::eof(),
        ];
        assert_vecs(&result, &expected);
    }

    #[test]
    fn bold_underscore_simple() {
        let input = "__Hello__";
        let result = Tokenizer::tokenize(input);
        let expected = vec![
            Token::new(TokenType::Underscore, "_".to_string()),
            Token::new(TokenType::Underscore, "_".to_string()),
            Token::new(TokenType::Text, "Hello".to_string()),
            Token::new(TokenType::Underscore, "_".to_string()),
            Token::new(TokenType::Underscore, "_".to_string()),
            Token::eof(),
        ];
        assert_vecs(&result, &expected);
    }

    #[test]
    fn bold_star_simple() {
        let input = "**Hello**";
        let result = Tokenizer::tokenize(input);
        let expected = vec![
            Token::new(TokenType::Star, "*".to_string()),
            Token::new(TokenType::Star, "*".to_string()),
            Token::new(TokenType::Text, "Hello".to_string()),
            Token::new(TokenType::Star, "*".to_string()),
            Token::new(TokenType::Star, "*".to_string()),
            Token::eof(),
        ];
        assert_vecs(&result, &expected);
    }
}
