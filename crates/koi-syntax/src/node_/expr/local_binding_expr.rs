use crate::token::Token;
use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalBindingExpressionNode {
    pub(crate) let_keyword: Token,
    pub(crate) identifier: Token,
    pub(crate) equal_symbol: Token,
    pub(crate) expression: Box<ExpressionNode>,
}

impl Spanning for LocalBindingExpressionNode {
    fn span(&self) -> Span {
        Span::from_bounds(self.let_keyword.span, self.expression.span())
    }
}
