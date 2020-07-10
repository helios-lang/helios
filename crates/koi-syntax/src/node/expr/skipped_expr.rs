use crate::token::*;
use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SkippedTokenNode {
    pub(crate) token: Token,
}

impl ExpressionNode for SkippedTokenNode {
    fn span(&self) -> Span {
        self.token.span
    }
}
