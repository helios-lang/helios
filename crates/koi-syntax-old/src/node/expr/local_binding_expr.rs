use crate::token::Token;
use super::*;

#[derive(Clone, Debug, Eq)]
pub struct LocalBindingExpressionNode {
    pub(crate) let_keyword: Token,
    pub(crate) identifier: Token,
    pub(crate) equal_symbol: Token,
    pub(crate) expression: Box<dyn ExpressionNode>,
}

impl PartialEq for LocalBindingExpressionNode {
    fn eq(&self, other: &Self) -> bool {
        self.let_keyword == other.let_keyword
            && self.identifier == other.identifier
            && self.equal_symbol == other.equal_symbol
            && &self.expression == &other.expression
    }
}

impl ExpressionNode for LocalBindingExpressionNode {
    fn span(&self) -> Span {
        Span::from_bounds(self.let_keyword.span, self.expression.span())
    }
}
