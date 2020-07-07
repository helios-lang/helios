use crate::token::Token;
use super::*;

#[derive(Clone, Debug, Eq)]
pub struct UnaryExpressionNode {
    pub(crate) operator: Token,
    pub(crate) operand: Box<dyn ExpressionNode>,
}

impl ExpressionNode for UnaryExpressionNode {
    fn span(&self) -> Span {
        Span::from_bounds(
            self.operator.span,
            self.operand.span(),
        )
    }
}

impl PartialEq for UnaryExpressionNode {
    fn eq(&self, other: &Self) -> bool {
        self.operator == other.operator
            && &self.operand == &other.operand
    }
}
