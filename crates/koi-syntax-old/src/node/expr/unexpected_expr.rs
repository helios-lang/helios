use crate::token::Token;
use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnexpectedTokenNode {
    pub(crate) token: Token,
}

impl ExpressionNode for UnexpectedTokenNode {
    fn span(&self) -> Span {
        self.token.span
    }
}
