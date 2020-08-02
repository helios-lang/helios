use crate::tree::token::SyntaxToken;
use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnexpectedTokenNode {
    pub(crate) token: SyntaxToken,
}

impl ExpressionNode for UnexpectedTokenNode {
    fn span(&self) -> TextSpan {
        self.token.span()
    }

    fn full_span(&self) -> TextSpan {
        self.token.full_span()
    }
}
