mod expr;
mod decl;

pub use decl::*;
pub use expr::*;

#[derive(Debug, PartialEq)]
pub enum Node {
    DeclarationNode(DeclarationNode),
    ExpressionNode(ExpressionNode),
    Eof,
}
