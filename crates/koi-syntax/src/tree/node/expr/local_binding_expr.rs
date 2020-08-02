use crate::tree::token::SyntaxToken;
use super::*;

#[derive(Clone, Debug, Eq)]
pub struct LocalBindingExpressionNode {
    pub(crate) let_keyword: SyntaxToken,
    pub(crate) identifier: SyntaxToken,
    pub(crate) equal_symbol: SyntaxToken,
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
    fn span(&self) -> TextSpan {
        TextSpan::from_spans(self.let_keyword.span(), self.expression.span())
    }

    fn full_span(&self) -> TextSpan {
        TextSpan::from_spans(
            self.let_keyword.full_span(),
            self.expression.full_span()
        )
    }
}
