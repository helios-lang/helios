use crate::token::Token;
use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LiteralExpressionNode {
    pub(crate) literal: Token,
}

impl ExpressionNode for LiteralExpressionNode {
    fn span(&self) -> Span {
        self.literal.span
    }
}
