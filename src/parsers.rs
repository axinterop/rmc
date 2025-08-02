use crate::tokenizer::*;

struct Node {
    type_: NodeType,
    value: String,
}

impl Node {
    fn new(type_: NodeType, value: String) -> Node {
        Node { type_, value }
    }
}

#[derive(PartialEq)]
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
        if tokens.is_empty() {
            return None;
        }

        if tokens[0].type_ == TokenType::Text {
            let f = tokens.remove(0);
            Some(Node::new(NodeType::Text, f.value.clone()))
        } else {
            None
        }
    }
}

impl Parser for EmphasizeParser {
    fn match_tokens(tokens: &mut Vec<Token>) -> Option<Node> {
        if tokens.len() < 3 {
            return None;
        };
        let rule_underscore = [
            TokenType::Underscore,
            TokenType::Text,
            TokenType::Underscore,
        ];
        let rule_star = [TokenType::Star, TokenType::Text, TokenType::Star];

        for i in 0..3 {
            if tokens[i].type_ != rule_underscore[i] && tokens[i].type_ != rule_star[i] {
                return None;
            }
        }
        let value = tokens[1].value.clone();
        tokens.drain(0..3);
        Some(Node::new(NodeType::Emphasize, value))
    }
}

impl Parser for BoldParser {
    fn match_tokens(tokens: &mut Vec<Token>) -> Option<Node> {
        if tokens.len() < 5 {
            return None;
        };
        let rule_underscore = [
            TokenType::Underscore,
            TokenType::Underscore,
            TokenType::Text,
            TokenType::Underscore,
            TokenType::Underscore,
        ];
        let rule_star = [
            TokenType::Star,
            TokenType::Star,
            TokenType::Text,
            TokenType::Star,
            TokenType::Star,
        ];

        for i in 0..5 {
            if tokens[i].type_ != rule_underscore[i] && tokens[i].type_ != rule_star[i] {
                return None;
            }
        }
        let value = tokens[2].value.clone();
        tokens.drain(0..5);
        Some(Node::new(NodeType::Bold, value))
    }
}

#[cfg(test)]
mod tests_parsers {
    use super::*;

    #[test]
    fn text_simple() {
        let mut tokens = vec![
            Token::new(TokenType::Text, "Hello".to_string()),
            Token::eof(),
        ];

        let result = TextParser::match_tokens(&mut tokens);
        let expected = Some(Node::new(NodeType::Text, "Hello".to_string()));
        assert!(result.is_some());
        assert!(expected.is_some());

        let result = result.unwrap();
        let expected = expected.unwrap();
        assert!(result.type_ == expected.type_);
        assert!(result.value == expected.value);

        assert!(tokens.len() == 1); // Should consume matched token
    }

    #[test]
    fn text_simple_none() {
        let mut tokens = vec![
            Token::new(TokenType::Underscore, "_".to_string()),
            Token::new(TokenType::Text, "Hello".to_string()),
            Token::new(TokenType::Underscore, "_".to_string()),
            Token::eof(),
        ];

        let result = TextParser::match_tokens(&mut tokens);
        assert!(result.is_none());
        assert!(tokens.len() == 4) // Should consume matched token
    }

    #[test]
    fn emphasize_underscore_simple() {
        let mut tokens = vec![
            Token::new(TokenType::Underscore, "_".to_string()),
            Token::new(TokenType::Text, "Hello".to_string()),
            Token::new(TokenType::Underscore, "_".to_string()),
            Token::eof(),
        ];

        let result = EmphasizeParser::match_tokens(&mut tokens);
        let expected = Some(Node::new(NodeType::Emphasize, "Hello".to_string()));
        assert!(result.is_some());
        assert!(expected.is_some());

        let result = result.unwrap();
        let expected = expected.unwrap();
        assert!(result.type_ == expected.type_);
        assert!(result.value == expected.value);

        assert!(tokens.len() == 1); // Should consume matched tokens
    }

    #[test]
    fn emphasize_star_simple() {
        let mut tokens = vec![
            Token::new(TokenType::Star, "*".to_string()),
            Token::new(TokenType::Text, "Hello".to_string()),
            Token::new(TokenType::Star, "*".to_string()),
            Token::eof(),
        ];

        let result = EmphasizeParser::match_tokens(&mut tokens);
        let expected = Some(Node::new(NodeType::Emphasize, "Hello".to_string()));
        assert!(result.is_some());
        assert!(expected.is_some());

        let result = result.unwrap();
        let expected = expected.unwrap();
        assert!(result.type_ == expected.type_);
        assert!(result.value == expected.value);

        assert!(tokens.len() == 1); // Should consume matched tokens
    }

    #[test]
    fn bold_underscore_simple() {
        let mut tokens = vec![
            Token::new(TokenType::Underscore, "_".to_string()),
            Token::new(TokenType::Underscore, "_".to_string()),
            Token::new(TokenType::Text, "Hello".to_string()),
            Token::new(TokenType::Underscore, "_".to_string()),
            Token::new(TokenType::Underscore, "_".to_string()),
            Token::eof(),
        ];

        let result = BoldParser::match_tokens(&mut tokens);
        let expected = Some(Node::new(NodeType::Bold, "Hello".to_string()));
        assert!(result.is_some());
        assert!(expected.is_some());

        let result = result.unwrap();
        let expected = expected.unwrap();
        assert!(result.type_ == expected.type_);
        assert!(result.value == expected.value);

        assert!(tokens.len() == 1); // Should consume matched tokens
    }

    #[test]
    fn bold_star_simple() {
        let mut tokens = vec![
            Token::new(TokenType::Star, "*".to_string()),
            Token::new(TokenType::Star, "*".to_string()),
            Token::new(TokenType::Text, "Hello".to_string()),
            Token::new(TokenType::Star, "*".to_string()),
            Token::new(TokenType::Star, "*".to_string()),
            Token::eof(),
        ];

        let result = BoldParser::match_tokens(&mut tokens);
        let expected = Some(Node::new(NodeType::Bold, "Hello".to_string()));
        assert!(result.is_some());
        assert!(expected.is_some());

        let result = result.unwrap();
        let expected = expected.unwrap();
        assert!(result.type_ == expected.type_);
        assert!(result.value == expected.value);

        assert!(tokens.len() == 1); // Should consume matched tokens
    }
}
