use crate::token::Token;
use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalBindingExpressionNode {
    pub(crate) literal: Token,
}

impl Spanning for LocalBindingExpressionNode {
    fn span(&self) -> Span {
        self.literal.span
    }
}
