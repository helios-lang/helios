use crate::tree::token::SyntaxToken;
use super::*;

#[derive(Clone, Debug, Eq)]
pub struct UnaryExpressionNode {
    pub(crate) operator: SyntaxToken,
    pub(crate) operand: Box<dyn ExpressionNode>,
}

impl ExpressionNode for UnaryExpressionNode {
    fn span(&self) -> TextSpan {
        TextSpan::from_spans(self.operator.span(), self.operand.span())
    }

    fn full_span(&self) -> TextSpan {
        TextSpan::from_spans(
            self.operator.full_span(),
            self.operand.full_span()
        )
    }
}

impl PartialEq for UnaryExpressionNode {
    fn eq(&self, other: &Self) -> bool {
        self.operator == other.operator
            && &self.operand == &other.operand
    }
}
