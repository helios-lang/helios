use crate::tree::token::SyntaxToken;
use super::*;

#[derive(Clone, Debug, Eq)]
pub struct GroupedExpressionNode {
    pub(crate) start_delimiter: SyntaxToken,
    pub(crate) expression: Box<dyn ExpressionNode>,
    pub(crate) end_delimiter: SyntaxToken,
}

impl ExpressionNode for GroupedExpressionNode {
    fn span(&self) -> TextSpan {
        TextSpan::from_spans(
            self.start_delimiter.span(),
            self.end_delimiter.span()
        )
    }

    fn full_span(&self) -> TextSpan {
        TextSpan::from_spans(
            self.start_delimiter.full_span(),
            self.end_delimiter.full_span()
        )
    }
}

impl PartialEq for GroupedExpressionNode {
    fn eq(&self, other: &Self) -> bool {
        self.start_delimiter == other.start_delimiter
            && self.end_delimiter == other.end_delimiter
            && &self.expression == &other.expression
    }
}
