use crate::token::Token;
use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdentifierExpressionNode {
    pub(crate) identifier: Token,
}

impl Spanning for IdentifierExpressionNode {
    fn span(&self) -> Span {
        self.identifier.span
    }
}
