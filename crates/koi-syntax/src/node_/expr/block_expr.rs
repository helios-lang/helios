use crate::token::Token;
use super::*;
use std::sync::Arc;

#[derive(Debug)]
pub struct BlockExpressionNode {
    pub(crate) begin_token: Token,
    pub(crate) expression_list: Vec<Arc<dyn ExpressionNode>>,
    pub(crate) end_token: Token,
}

impl ExpressionNode for BlockExpressionNode {
    fn span(&self) -> Span {
        Span::from_bounds(self.begin_token.span, self.end_token.span)
    }
}
