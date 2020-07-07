use crate::token::Token;
use super::*;
use std::sync::Arc;

#[derive(Debug)]
pub struct LocalBindingExpressionNode {
    pub(crate) let_keyword: Token,
    pub(crate) identifier: Token,
    pub(crate) equal_symbol: Token,
    pub(crate) expression: Arc<dyn ExpressionNode>,
}

impl ExpressionNode for LocalBindingExpressionNode {
    fn span(&self) -> Span {
        Span::from_bounds(self.let_keyword.span, self.expression.span())
    }
}
