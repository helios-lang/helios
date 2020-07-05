mod expr;
mod decl;

pub use decl::*;
pub use expr::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Node {
    DeclarationNode(DeclarationNode),
    ExpressionNode(ExpressionNode),
    Eof,
}
