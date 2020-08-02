use crate::tree::token::SyntaxToken;
use super::*;

#[derive(Clone, Debug, Eq)]
pub struct IfExpressionNode {
    pub(crate) if_keyword: SyntaxToken,
    pub(crate) condition: Box<dyn ExpressionNode>,
    pub(crate) open_brace: SyntaxToken,
    pub(crate) expression: Box<dyn ExpressionNode>,
    pub(crate) close_brace: SyntaxToken,
    pub(crate) else_clause: Option<ElseClauseExpressionNode>,
}

impl ExpressionNode for IfExpressionNode {
    fn span(&self) -> TextSpan {
        TextSpan::from_spans(
            self.if_keyword.span(),
            self.else_clause
                .as_ref()
                .map(|clause| clause.expression.span())
                .unwrap_or(self.expression.span())
        )
    }

    fn full_span(&self) -> TextSpan {
        TextSpan::from_spans(
            self.if_keyword.full_span(),
            self.else_clause
                .as_ref()
                .map(|clause| clause.expression.full_span())
                .unwrap_or(self.expression.full_span())
        )
    }
}

impl PartialEq for IfExpressionNode {
    fn eq(&self, other: &Self) -> bool {
        self.if_keyword == other.if_keyword
            && &self.condition == &other.condition
            && &self.open_brace == &other.open_brace
            && &self.expression == &other.expression
            && &self.close_brace == &other.close_brace
            && self.else_clause == other.else_clause
    }
}

#[derive(Clone, Debug, Eq)]
pub struct ElseClauseExpressionNode {
    pub(crate) else_keyword: SyntaxToken,
    pub(crate) open_brace: SyntaxToken,
    pub(crate) expression: Box<dyn ExpressionNode>,
    pub(crate) close_brace: SyntaxToken,
}

impl PartialEq for ElseClauseExpressionNode {
    fn eq(&self, other: &Self) -> bool {
        self.else_keyword == other.else_keyword
            && &self.open_brace == &other.open_brace
            && &self.expression == &other.expression
            && &self.close_brace == &other.close_brace
    }
}
