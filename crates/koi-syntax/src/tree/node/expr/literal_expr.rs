use crate::tree::token::SyntaxToken;
use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LiteralExpressionNode {
    pub(crate) literal: SyntaxToken,
}

impl ExpressionNode for LiteralExpressionNode {
    fn span(&self) -> TextSpan {
        self.literal.span()
    }

    fn full_span(&self) -> TextSpan {
        self.literal.full_span()
    }
}
