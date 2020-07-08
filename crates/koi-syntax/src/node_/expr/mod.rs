pub mod binary_expr;
pub mod block_expr;
pub mod error_expr;
pub mod identifier_expr;
pub mod if_expr;
pub mod grouped_expr;
pub mod literal_expr;
pub mod local_binding_expr;
pub mod missing_expr;
pub mod unary_expr;
pub mod unexpected_expr;
pub mod unimplemented_expr;

pub use self::{
    binary_expr::*,
    block_expr::*,
    error_expr::*,
    identifier_expr::*,
    if_expr::*,
    grouped_expr::*,
    literal_expr::*,
    local_binding_expr::*,
    missing_expr::*,
    unary_expr::*,
    unexpected_expr::*,
    unimplemented_expr::*,
};
use super::Spanning;
use crate::source::Span;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExpressionNode {
    /// A reference to an identifier.
    IdentifierExpressionNode(IdentifierExpressionNode),

    /// A literal expression.
    LiteralExpressionNode(LiteralExpressionNode),

    /// A local binding expression.
    LocalBindingExpressionNode(LocalBindingExpressionNode),

    /// An if-branching expression node.
    IfExpressionNode(IfExpressionNode),

    /// A unary expression node.
    UnaryExpressionNode(UnaryExpressionNode),

    /// A binary expression node.
    BinaryExpressionNode(BinaryExpressionNode),

    /// A grouped expression (constructed when an expression is parenthesised).
    GroupedExpressionNode(GroupedExpressionNode),

    /// An indented block of expressions.
    BlockExpressionNode(BlockExpressionNode),

    /// A placeholder for unimplemented expressions.
    UnimplementedExpressionNode(UnaryExpressionNode),

    /// A missing expression node.
    MissingExpressionNode(MissingExpressionNode),

    /// An error token produced by the lexer, for example when a string literal
    /// is unterminated.
    ErrorToken(ErrorExpressionNode),

    /// An unexpected token.
    UnexpectedToken(UnexpectedExpressionNode),
}

impl Spanning for ExpressionNode {
    fn span(&self) -> Span {
        match self {
            ExpressionNode::IdentifierExpressionNode(node) => node.span(),
            ExpressionNode::LiteralExpressionNode(node) => node.span(),
            ExpressionNode::LocalBindingExpressionNode(node) => node.span(),
            ExpressionNode::IfExpressionNode(node) => node.span(),
            ExpressionNode::UnaryExpressionNode(node) => node.span(),
            ExpressionNode::BinaryExpressionNode(node) => node.span(),
            ExpressionNode::GroupedExpressionNode(node) => node.span(),
            ExpressionNode::BlockExpressionNode(node) => node.span(),
            ExpressionNode::UnimplementedExpressionNode(node) => node.span(),
            ExpressionNode::MissingExpressionNode(node) => node.span(),
            ExpressionNode::ErrorToken(node) => node.span(),
            ExpressionNode::UnexpectedToken(node) => node.span(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::source::Position;
    use crate::token::*;

    #[test]
    fn test_if_expr() {
        // if x then 0 else 1
        let if_expr = ExpressionNode::IfExpressionNode(if_expr::IfExpressionNode {
            if_keyword: Token::with(
                TokenKind::Keyword(Keyword::If),
                Span::new(
                    Position::new(0, 0, 0),
                    Position::new(0, 2, 2),
                )
            ),
            condition: Box::new(ExpressionNode::IdentifierExpressionNode(
                identifier_expr::IdentifierExpressionNode {
                    identifier: Token::with(
                        TokenKind::Identifier,
                        Span::new(
                            Position::new(0, 3, 3),
                            Position::new(0, 4, 4),
                        )
                    )
                }
            )),
            then_keyword: Token::with(
                TokenKind::Keyword(Keyword::Then),
                Span::new(
                    Position::new(0, 5, 5),
                    Position::new(0, 9, 9),
                )
            ),
            expression: Box::new(ExpressionNode::IdentifierExpressionNode(
                identifier_expr::IdentifierExpressionNode {
                    identifier: Token::with(
                        TokenKind::Identifier,
                        Span::new(
                            Position::new(0, 10, 10),
                            Position::new(0, 11, 11),
                        )
                    )
                }
            )),
            else_clause: Some(if_expr::ElseClauseExpressionNode {
                else_keyword: Token::with(
                    TokenKind::Keyword(Keyword::Then),
                    Span::new(
                        Position::new(0, 12, 12),
                        Position::new(0, 16, 16),
                    )
                ),
                expression: Box::new(ExpressionNode::IdentifierExpressionNode(
                    identifier_expr::IdentifierExpressionNode {
                        identifier: Token::with(
                            TokenKind::Identifier,
                            Span::new(
                                Position::new(0, 17, 17),
                                Position::new(0, 18, 18),
                            )
                        )
                    }
                )),
            })
        });

        println!("{:#?}", if_expr);
        println!("{}", if_expr.span());
    }
}
