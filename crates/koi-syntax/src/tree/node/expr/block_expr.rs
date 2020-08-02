use crate::tree::token::SyntaxToken;
use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BlockExpressionNode {
    pub(crate) open_brace: SyntaxToken,
    pub(crate) expression_list: Vec<Box<dyn ExpressionNode>>,
    pub(crate) close_brace: SyntaxToken,
}

impl ExpressionNode for BlockExpressionNode {
    fn span(&self) -> TextSpan {
        TextSpan::from_spans(self.open_brace.span(), self.close_brace.span())
    }

    fn full_span(&self) -> TextSpan {
        TextSpan::from_spans(
            self.open_brace.full_span(),
            self.close_brace.full_span()
        )
    }
}
