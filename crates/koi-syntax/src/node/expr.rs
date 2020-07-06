use crate::errors::LexerError;
use crate::source::{Position, Span};
use crate::token::*;

#[derive(Clone, Debug, PartialEq)]
pub enum ExpressionNode {
    /// A reference to an identifier.
    Identifier(Span),

    /// A literal expression.
    LiteralNode(LiteralNode, Span),

    /// A local binding expression.
    LocalBindingNode {
        identifier: Token,
        equal_symbol: Token,
        expression: Box<ExpressionNode>,
    },

    /// An if-branching expression node.
    IfExpressionNode {
        pattern: Box<ExpressionNode>,
        then_keyword: Token,
        expression: Box<ExpressionNode>,
        else_clause: Option<Box<ExpressionNode>>,
    },

    /// A unary expression node.
    UnaryExpressionNode {
        operator: Token,
        expression: Box<ExpressionNode>,
    },

    /// A binary expression.
    BinaryExpressionNode {
        operator: Token,
        lhs: Box<ExpressionNode>,
        rhs: Box<ExpressionNode>,
    },

    /// A grouped expression (constructed when an expression is parenthesised).
    GroupedExpressionNode {
        start_delimiter: Token,
        expression: Box<ExpressionNode>,
        end_delimiter: Token,
    },

    /// An indented block of expressions.
    BlockExpressionNode {
        expressions: Vec<Box<ExpressionNode>>,
        end_token: Token,
    },

    /// A placeholder for unimplemented expressions.
    UnimplementedExpressionNode,

    /// A missing expression node.
    MissingExpressionNode(Position),

    /// An error token produced by the lexer, for example when a string
    /// literal is unterminated.
    Error(LexerError),

    /// An unexpected token kind.
    Unexpected(TokenKind, Position),
}

#[derive(Clone, Debug, PartialEq)]
pub enum LiteralNode {
    Boolean(bool),

    /// A float literal.
    Float,

    /// An integer literal.
    Integer(Base),
}
