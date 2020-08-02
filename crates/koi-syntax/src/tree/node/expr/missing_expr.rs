use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissingExpressionNode {
    pub(crate) position: usize,
}

impl ExpressionNode for MissingExpressionNode {
    fn span(&self) -> TextSpan {
        TextSpan::zero_width(self.position)
    }

    fn full_span(&self) -> TextSpan {
        TextSpan::zero_width(self.position)
    }
}
