use crate::tree::token::SyntaxToken;
use super::*;

#[derive(Clone, Debug, Eq)]
pub struct BinaryExpressionNode {
    pub(crate) operator: SyntaxToken,
    pub(crate) lhs: Box<dyn ExpressionNode>,
    pub(crate) rhs: Box<dyn ExpressionNode>,
}

impl ExpressionNode for BinaryExpressionNode {
    fn span(&self) -> TextSpan {
        TextSpan::from_spans(self.lhs.span(), self.rhs.span())
    }

    fn full_span(&self) -> TextSpan {
        TextSpan::from_spans(self.lhs.full_span(), self.rhs.full_span())
    }
}

impl PartialEq for BinaryExpressionNode {
    fn eq(&self, other: &Self) -> bool {
        self.operator == other.operator
            && &self.lhs == &other.lhs
            && &self.rhs == &other.rhs
    }
}
