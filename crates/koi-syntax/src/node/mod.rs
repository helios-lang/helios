pub mod expr;
pub mod decl;

use crate::source::Span;
pub use expr::*;
pub use decl::*;

#[derive(Debug, Eq, PartialEq)]
pub struct Ast(pub(crate) Vec<Node>);

impl Ast {
    pub fn span(&self) -> Span {
        Span::from_bounds(
            self.0.first().map(|node| node.span()).unwrap_or_default(),
            self.0.last().map(|node| node.span()).unwrap_or_default()
        )
    }

    pub fn nodes(&self) -> &Vec<Node> {
        &self.0
    }
}

#[derive(Debug, Eq)]
pub enum Node {
    DeclarationNode(Box<dyn DeclarationNode>),
    ExpressionNode(Box<dyn ExpressionNode>),
    Eof,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Node::DeclarationNode(lhs), Node::DeclarationNode(rhs)) => lhs == rhs,
            (Node::ExpressionNode(lhs), Node::ExpressionNode(rhs)) => lhs == rhs,
            (Node::Eof, Node::Eof) => true,
            _ => false,
        }
    }
}

impl Node {
    pub fn span(&self) -> Span {
        match self {
            Self::DeclarationNode(node) => node.span(),
            Self::ExpressionNode(node) => node.span(),
            _ => Span::default(),
        }
    }
}
