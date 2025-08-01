use std::collections::VecDeque;

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

#[derive(PartialEq)]
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

struct Node {
    type_: NodeType,
    value: String,
}

impl Node {
    fn new(type_: NodeType, value: String) -> Node {
        Node { type_, value }
    }
}

enum NodeType {
    Text,
    Emphasize,
    Bold,
}

trait Parser {
    fn match_tokens(tokens: &mut Vec<Token>) -> Option<Node>;
}

struct TextParser;
struct BoldParser;
struct EmphasizeParser;
struct SentenceParser;

impl Parser for TextParser {
    fn match_tokens(tokens: &mut Vec<Token>) -> Option<Node> {
        let f = tokens.first()?;

        if f.type_ == TokenType::Text {
            Some(Node::new(NodeType::Text, f.value.clone()))
        } else {
            None
        }
    }
}

fn main() {
    let markdown = "_Hello_";
    let tokens = Tokenizer::tokenize(markdown);
    for i in 0..tokens.len() {
        println!("{i}: {}", tokens[i].value);
    }
}

#[cfg(test)]
mod tests_tokenize {
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
        let tokens = Tokenizer::tokenize(input);
        let result = vec![
            Token::new(TokenType::Underscore, "_".to_string()),
            Token::new(TokenType::Text, "Hello".to_string()),
            Token::new(TokenType::Underscore, "_".to_string()),
            Token::eof(),
        ];
        assert_vecs(&tokens, &result);
    }

    #[test]
    fn emphesize_star_simple() {
        let input = "*Hello*";
        let tokens = Tokenizer::tokenize(input);
        let result = vec![
            Token::new(TokenType::Star, "*".to_string()),
            Token::new(TokenType::Text, "Hello".to_string()),
            Token::new(TokenType::Star, "*".to_string()),
            Token::eof(),
        ];
        assert_vecs(&tokens, &result);
    }

    #[test]
    fn bold_underscore_simple() {
        let input = "__Hello__";
        let tokens = Tokenizer::tokenize(input);
        let result = vec![
            Token::new(TokenType::Underscore, "_".to_string()),
            Token::new(TokenType::Underscore, "_".to_string()),
            Token::new(TokenType::Text, "Hello".to_string()),
            Token::new(TokenType::Underscore, "_".to_string()),
            Token::new(TokenType::Underscore, "_".to_string()),
            Token::eof(),
        ];
        assert_vecs(&tokens, &result);
    }

    #[test]
    fn bold_star_simple() {
        let input = "**Hello**";
        let tokens = Tokenizer::tokenize(input);
        let result = vec![
            Token::new(TokenType::Star, "*".to_string()),
            Token::new(TokenType::Star, "*".to_string()),
            Token::new(TokenType::Text, "Hello".to_string()),
            Token::new(TokenType::Star, "*".to_string()),
            Token::new(TokenType::Star, "*".to_string()),
            Token::eof(),
        ];
        assert_vecs(&tokens, &result);
    }
}
