use crate::token::Token;
use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BlockExpressionNode {
    pub(crate) begin_token: Token,
    pub(crate) expression_list: Vec<Box<dyn ExpressionNode>>,
    pub(crate) end_token: Token,
}

impl ExpressionNode for BlockExpressionNode {
    fn span(&self) -> Span {
        Span::from_bounds(self.begin_token.span, self.end_token.span)
    }
}
