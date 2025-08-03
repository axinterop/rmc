use crate::tokenizer::*;

struct Node {
    type_: NodeType,
    value: String,
    children: Vec<Node>,
}

impl Node {
    fn new(type_: NodeType, value: String) -> Node {
        Node {
            type_,
            value,
            children: Vec::new(),
        }
    }

    fn new_paragraph() -> Node {
        Self::new(NodeType::Paragraph, "".to_string())
    }

    fn new_body() -> Node {
        Self::new(NodeType::Body, "".to_string())
    }
}

#[derive(PartialEq)]
enum NodeType {
    Text,
    Emphasize,
    Bold,

    Paragraph,
    Body,
}

trait Parser {
    fn match_tokens(tokens: &mut Vec<Token>) -> Option<Node>;
}

struct TextParser;
struct BoldParser;
struct EmphasizeParser;
struct SentenceParser;
struct ParagraphParser;
struct BodyParser;

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

impl Parser for SentenceParser {
    fn match_tokens(tokens: &mut Vec<Token>) -> Option<Node> {
        let parsers: Vec<fn(&mut Vec<Token>) -> Option<Node>> = vec![
            EmphasizeParser::match_tokens,
            BoldParser::match_tokens,
            TextParser::match_tokens,
        ];
        for parser in parsers {
            if let Some(node) = parser(tokens) {
                return Some(node);
            }
        }
        None
    }
}

impl Parser for ParagraphParser {
    fn match_tokens(tokens: &mut Vec<Token>) -> Option<Node> {
        if tokens.is_empty() {
            return None;
        };

        let mut paragraph = Node::new_paragraph();
        while let Some(node) = SentenceParser::match_tokens(tokens) {
            paragraph.children.push(node);
        }

        if tokens[0].type_ == TokenType::Eof {
            tokens.drain(0..1);
            return Some(paragraph);
        }

        match (&tokens[0].type_, &tokens[1].type_) {
            (TokenType::Newline, TokenType::Newline) => {
                tokens.drain(0..2);
                Some(paragraph)
            }
            (TokenType::Newline, TokenType::Eof) => {
                tokens.drain(0..2);
                Some(paragraph)
            }
            _ => None,
        }
    }
}

impl Parser for BodyParser {
    fn match_tokens(tokens: &mut Vec<Token>) -> Option<Node> {
        if tokens.is_empty() {
            return None;
        };

        let mut body = Node::new_body();
        while let Some(node) = ParagraphParser::match_tokens(tokens) {
            body.children.push(node);
        }

        if !body.children.is_empty() {
            Some(body)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests_parsers {
    use super::*;

    fn assert_vecs(vec1: &Vec<Node>, vec2: &Vec<Node>) {
        assert!(vec1.len() == vec2.len());
        for i in 0..vec1.len() {
            assert!(vec1[i].type_ == vec2[i].type_);
            assert!(vec1[i].value == vec2[i].value);
        }
    }

    #[test]
    fn body_simple() {
        let markdown = "__Foo__ and *bar*.\n\nAnother paragraph.";
        let mut tokens = Tokenizer::tokenize(markdown);
        let result = BodyParser::match_tokens(&mut tokens);
        assert!(result.is_some());
        let result = result.unwrap();

        assert!(tokens.len() == 0); // Consumed all tokens

        assert!(result.children.len() == 2); // 2 paragraphs
        assert!(result.children[0].type_ == NodeType::Paragraph);
        assert!(result.children[1].type_ == NodeType::Paragraph);

        assert!(result.children[0].children.len() == 4); // First paragraph has 4 sentences
        assert!(result.children[1].children.len() == 1); // First paragraph has 1 sentence
    }

    #[test]
    fn paragraph_simple() {
        let markdown = "__Foo__ and *bar*\n\n";
        let mut tokens = Tokenizer::tokenize(markdown);
        let result = ParagraphParser::match_tokens(&mut tokens);
        let expected_children = vec![
            Node::new(NodeType::Bold, "Foo".to_string()),
            Node::new(NodeType::Text, " and ".to_string()),
            Node::new(NodeType::Emphasize, "bar".to_string()),
        ];
        assert!(result.is_some());
        let result: Node = result.unwrap();

        assert!(result.type_ == NodeType::Paragraph);
        assert!(result.value == "".to_string());

        assert_vecs(&result.children, &expected_children);
    }

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
        assert!(tokens.len() == 4) // Should not consume any tokens
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

    #[test]
    fn sentence_simple_all() {
        let mut tokens = vec![Token::new(TokenType::Text, "Hello".to_string())];
        let result = SentenceParser::match_tokens(&mut tokens);
        assert!(result.is_some());
        assert!(result.unwrap().type_ == NodeType::Text);

        let mut tokens = vec![
            Token::new(TokenType::Underscore, "_".to_string()),
            Token::new(TokenType::Text, "Hello".to_string()),
            Token::new(TokenType::Underscore, "_".to_string()),
            Token::eof(),
        ];
        let result = SentenceParser::match_tokens(&mut tokens);
        assert!(result.is_some());
        assert!(result.unwrap().type_ == NodeType::Emphasize);

        let mut tokens = vec![
            Token::new(TokenType::Underscore, "_".to_string()),
            Token::new(TokenType::Underscore, "_".to_string()),
            Token::new(TokenType::Text, "Hello".to_string()),
            Token::new(TokenType::Underscore, "_".to_string()),
            Token::new(TokenType::Underscore, "_".to_string()),
            Token::eof(),
        ];
        let result = SentenceParser::match_tokens(&mut tokens);
        assert!(result.is_some());
        assert!(result.unwrap().type_ == NodeType::Bold);
    }
}
