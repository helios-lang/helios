use crate::token::Token;
use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GroupedExpressionNode {
    pub(crate) start_delimiter: Token,
    pub(crate) expression: Box<ExpressionNode>,
    pub(crate) end_delimiter: Token,
}

impl Spanning for GroupedExpressionNode {
    fn span(&self) -> Span {
        Span::from_bounds(self.start_delimiter.span, self.end_delimiter.span)
    }
}
