use crate::token::Token;
use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IfExpressionNode {
    pub(crate) if_keyword: Token,
    pub(crate) condition: Box<ExpressionNode>,
    pub(crate) then_keyword: Token,
    pub(crate) expression: Box<ExpressionNode>,
    pub(crate) else_clause: Option<ElseClauseExpressionNode>,
}

impl Spanning for IfExpressionNode {
    fn span(&self) -> Span {
        Span::from_bounds(
            self.if_keyword.span,
            self.else_clause
                .as_ref()
                .map(|clause| clause.span())
                .unwrap_or(self.expression.span())
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ElseClauseExpressionNode {
    pub(crate) else_keyword: Token,
    pub(crate) expression: Box<ExpressionNode>,
}

impl Spanning for ElseClauseExpressionNode {
    fn span(&self) -> Span {
        Span::from_bounds(self.else_keyword.span, self.expression.span())
    }
}
