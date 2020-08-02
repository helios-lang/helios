use crate::tree::token::SyntaxToken;
use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdentifierExpressionNode {
    pub(crate) identifier: SyntaxToken,
}

impl ExpressionNode for IdentifierExpressionNode {
    fn span(&self) -> TextSpan {
        self.identifier.span()
    }

    fn full_span(&self) -> TextSpan {
        self.identifier.full_span()
    }
}
