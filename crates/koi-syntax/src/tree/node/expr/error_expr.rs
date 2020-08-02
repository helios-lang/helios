use crate::tree::token::SyntaxToken;
use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ErrorExpressionNode {
    pub(crate) token: SyntaxToken,
}

impl ExpressionNode for ErrorExpressionNode {
    fn span(&self) -> TextSpan {
        self.token.span()
    }
    
    fn full_span(&self) -> TextSpan {
        self.token.full_span()
    }
}
