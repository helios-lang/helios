use crate::token::Token;
use super::*;
use std::sync::Arc;

#[derive(Debug)]
pub struct IfExpressionNode {
    pub(crate) if_keyword: Token,
    pub(crate) condition: Arc<dyn ExpressionNode>,
    pub(crate) then_keyword: Token,
    pub(crate) expression: Arc<dyn ExpressionNode>,
    pub(crate) else_clause: Option<Arc<ElseClauseExpressionNode>>,
}

impl ExpressionNode for IfExpressionNode {
    fn span(&self) -> Span {
        Span::from_bounds(
            self.if_keyword.span,
            self.else_clause
                .as_ref()
                .map(|clause| clause.expression.span())
                .unwrap_or(self.expression.span())
        )
    }
}

#[derive(Debug)]
pub struct ElseClauseExpressionNode {
    pub(crate) else_keyword: Token,
    pub(crate) expression: Arc<dyn ExpressionNode>,
}
