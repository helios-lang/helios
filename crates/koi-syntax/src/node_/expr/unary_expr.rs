use crate::token::Token;
use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnaryExpressionNode {
    pub(crate) operator: Token,
    pub(crate) operand: Box<ExpressionNode>,
}

impl Spanning for UnaryExpressionNode {
    fn span(&self) -> Span {
        Span::from_bounds(self.operator.span, self.operand.span())
    }
}
