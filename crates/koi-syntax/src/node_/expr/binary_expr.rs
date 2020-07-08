use crate::token::Token;
use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BinaryExpressionNode {
    pub(crate) operator: Token,
    pub(crate) lhs: Box<ExpressionNode>,
    pub(crate) rhs: Box<ExpressionNode>,
}

impl Spanning for BinaryExpressionNode {
    fn span(&self) -> Span {
        Span::from_bounds(self.lhs.span(), self.rhs.span())
    }
}
