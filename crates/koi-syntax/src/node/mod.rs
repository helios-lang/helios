mod expr;
mod decl;

pub use decl::*;
pub use expr::*;

#[derive(Debug)]
pub enum Node {
    DeclarationNode(DeclarationNode),
    ExpressionNode(ExpressionNode),
    Eof,
}
