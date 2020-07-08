pub mod expr;
pub mod decl;

pub use expr::ExpressionNode;
pub use decl::DeclarationNode;
use crate::source::Span;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Ast(pub(crate) Vec<Node>);

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
