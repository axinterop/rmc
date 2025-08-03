use crate::parsers::*;

pub struct Visitor;
struct TextVisitor;
struct BoldVisitor;
struct EmphasizeVisitor;
struct SentenceVisitor;
struct ParagraphVisitor;
struct BodyVisitor;

pub trait BaseVisitor {
    fn visit(node: Node) -> String;
}

impl BaseVisitor for Visitor {
    fn visit(node: Node) -> String {
        BodyVisitor::visit(node)
    }
}

impl BaseVisitor for TextVisitor {
    fn visit(node: Node) -> String {
        if node.type_ != NodeType::Text {
            panic!("TextVisitor: node's type is not NodeType::Text")
        }
        node.value
    }
}

impl BaseVisitor for BoldVisitor {
    fn visit(node: Node) -> String {
        if node.type_ != NodeType::Bold {
            panic!("BoldVisitor: node's type is not NodeType::Bold")
        }
        format!("<strong>{}</strong>", node.value)
    }
}

impl BaseVisitor for EmphasizeVisitor {
    fn visit(node: Node) -> String {
        if node.type_ != NodeType::Emphasize {
            panic!("EmphasizeVisitor: node's type is not NodeType::Emphasize")
        }
        format!("<em>{}</em>", node.value)
    }
}

impl BaseVisitor for SentenceVisitor {
    fn visit(node: Node) -> String {
        let mut result = String::new();
        match node.type_ {
            NodeType::Emphasize => result.push_str(&EmphasizeVisitor::visit(node)),
            NodeType::Bold => result.push_str(&BoldVisitor::visit(node)),
            NodeType::Text => result.push_str(&TextVisitor::visit(node)),
            _ => panic!("SentenceVisitor: unexpected node's type"),
        }
        result
    }
}

impl BaseVisitor for ParagraphVisitor {
    fn visit(node: Node) -> String {
        if node.type_ != NodeType::Paragraph {
            panic!("ParagraphVisitor: node's type is not NodeType::Paragraph");
        }
        let mut result = String::new();
        result.push_str("<p>");
        for child in node.children {
            result.push_str(&SentenceVisitor::visit(child));
        }
        result.push_str("</p>");
        result
    }
}

impl BaseVisitor for BodyVisitor {
    fn visit(node: Node) -> String {
        if node.type_ != NodeType::Body {
            panic!("BodyVisitor: node's type is not NodeType::Body");
        }
        let mut result = String::new();
        for child in node.children {
            result.push_str(&ParagraphVisitor::visit(child));
        }
        result
    }
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn body_visitor() {}
// }
