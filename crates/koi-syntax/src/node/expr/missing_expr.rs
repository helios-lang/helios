use crate::source::Position;
use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissingExpressionNode {
    pub(crate) position: Position,
}

impl ExpressionNode for MissingExpressionNode {
    fn span(&self) -> Span {
        Span::new(self.position, self.position)
    }
}
