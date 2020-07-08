pub mod expr;
pub mod decl;

pub use expr::*;
pub use decl::DeclarationNode;
use crate::source::Span;

#[derive(Clone, Debug, Eq, PartialEq)]
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Node {
    DeclarationNode(DeclarationNode),
    ExpressionNode(ExpressionNode),
    Eof,
}

trait Spanning {
    /// The span of the given node.
    ///
    /// This should be the bounds established by the start position of its first
    /// child to the end position of its last child.
    fn span(&self) -> Span;
}

impl Spanning for Node {
    fn span(&self) -> Span {
        match self {
            Node::DeclarationNode(node) => node.span(),
            Node::ExpressionNode(node) => node.span(),
            Node::Eof => Span::default(),
        }
    }
}
