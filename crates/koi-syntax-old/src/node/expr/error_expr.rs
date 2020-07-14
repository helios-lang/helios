use crate::token::Token;
use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ErrorExpressionNode {
    pub(crate) token: Token,
}

impl ExpressionNode for ErrorExpressionNode {
    fn span(&self) -> Span {
        self.token.span
    }
}
