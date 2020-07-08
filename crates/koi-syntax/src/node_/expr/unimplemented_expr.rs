use crate::token::Token;
use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnimplementedExpressionNode {
    pub(crate) token: Token,
}

impl Spanning for UnimplementedExpressionNode {
    fn span(&self) -> Span {
        self.token.span
    }
}