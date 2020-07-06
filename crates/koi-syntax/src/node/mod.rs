mod expr;
mod decl;

pub use decl::*;
pub use expr::*;

#[derive(Clone, Eq, Debug, PartialEq)]
pub enum Node {
    DeclarationNode(DeclarationNode),
    ExpressionNode(ExpressionNode),
    Eof,
}
