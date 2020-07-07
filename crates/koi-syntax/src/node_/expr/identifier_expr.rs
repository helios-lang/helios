use crate::token::Token;
use super::*;

#[derive(Debug)]
pub struct IdentifierExpressionNode {
    pub(crate) identifier: Token,
}

impl ExpressionNode for IdentifierExpressionNode {
    fn span(&self) -> Span {
        self.identifier.span
    }
}
