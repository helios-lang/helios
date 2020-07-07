use crate::token::Token;
use super::*;

#[derive(Clone, Debug, Eq)]
pub struct GroupedExpressionNode {
    pub(crate) start_delimiter: Token,
    pub(crate) expression: Box<dyn ExpressionNode>,
    pub(crate) end_delimiter: Token,
}

impl ExpressionNode for GroupedExpressionNode {
    fn span(&self) -> Span {
        Span::from_bounds(
            self.start_delimiter.span,
            self.end_delimiter.span
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
