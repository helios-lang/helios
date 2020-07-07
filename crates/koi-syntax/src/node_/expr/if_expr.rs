use crate::token::Token;
use super::*;

#[derive(Clone, Debug, Eq)]
pub struct IfExpressionNode {
    pub(crate) if_keyword: Token,
    pub(crate) condition: Box<dyn ExpressionNode>,
    pub(crate) then_keyword: Token,
    pub(crate) expression: Box<dyn ExpressionNode>,
    pub(crate) else_clause: Option<Box<ElseClauseExpressionNode>>,
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

impl PartialEq for IfExpressionNode {
    fn eq(&self, other: &Self) -> bool {
        self.if_keyword == other.if_keyword
            && &self.condition == &other.condition
            && self.then_keyword == other.then_keyword
            && &self.expression == &other.expression
            && self.else_clause == other.else_clause
    }
}

#[derive(Clone, Debug, Eq)]
pub struct ElseClauseExpressionNode {
    pub(crate) else_keyword: Token,
    pub(crate) expression: Box<dyn ExpressionNode>,
}

impl PartialEq for ElseClauseExpressionNode {
    fn eq(&self, other: &Self) -> bool {
        self.else_keyword == other.else_keyword
            && &self.expression == &other.expression
    }
}
