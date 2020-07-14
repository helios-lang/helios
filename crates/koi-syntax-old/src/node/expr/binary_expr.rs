use crate::token::Token;
use super::*;

#[derive(Clone, Debug, Eq)]
pub struct BinaryExpressionNode {
    pub(crate) operator: Token,
    pub(crate) lhs: Box<dyn ExpressionNode>,
    pub(crate) rhs: Box<dyn ExpressionNode>,
}

impl ExpressionNode for BinaryExpressionNode {
    fn span(&self) -> Span {
        Span::from_bounds(
            self.lhs.span(),
            self.rhs.span()
        )
    }
}

impl PartialEq for BinaryExpressionNode {
    fn eq(&self, other: &Self) -> bool {
        self.operator == other.operator
            && &self.lhs == &other.lhs
            && &self.rhs == &other.rhs
    }
}
