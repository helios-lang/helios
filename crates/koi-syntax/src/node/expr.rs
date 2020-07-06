use crate::errors::LexerError;
use crate::source::Position;
use crate::token::*;
use std::default::Default;
use std::fmt::Debug;

#[derive(Clone, Debug, PartialEq)]
pub enum ExpressionNode {
    /// A named reference to an identifier.
    Identifier,

    /// A literal expression that is translated exactly as written in source.
    LiteralNode(LiteralNode),

    /// A local binding expression.
    LocalBindingNode(LocalBindingNode),

    /// An if-branching expression.
    IfExpressionNode(IfExpressionNode),

    /// A unary expression holding a token (signifying the operator) and an
    /// expression (signifying the right hand side of the operation).
    UnaryExpression(Token, Box<ExpressionNode>),

    /// A binary expression holding a token (signifying the operator) and two
    /// expressions (signifying the left and right hand sides of the operation).
    BinaryExpressionNode(BinaryExpressionNode),

    /// A grouped expression (constructed when an expression is parenthesised).
    GroupedExpressionNode(GroupedExpressionNode),

    /// An indented block of expressions.
    BlockExpressionNode(Vec<Box<ExpressionNode>>),

    /// A placeholder for unimplemented expressions.
    UnimplementedExpression,

    /// A missing expression node.
    MissingExpression(Position),

    /// An error token produced by the lexer, for example when a string
    /// literal is not terminated.
    Error(LexerError),

    /// An unexpected token kind.
    Unexpected(TokenKind, Position),
}

#[derive(Clone, Debug, PartialEq)]
pub enum LiteralNode {
    Boolean(bool),

    /// A float literal.
    Float(Token),

    /// An integer literal.
    Integer(Token),
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct LocalBindingNode {
    pub(crate) identifier: Option<Token>,
    pub(crate) equal_symbol: Option<Token>,
    pub(crate) expression: Option<Box<ExpressionNode>>,
}

impl LocalBindingNode {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn identifier<T: Into<Option<Token>>>(&mut self, identifier: T) -> &mut Self {
        self.identifier = identifier.into();
        self
    }

    pub fn equal_symbol<T: Into<Option<Token>>>(&mut self, equal_symbol: T) -> &mut Self {
        self.equal_symbol = equal_symbol.into();
        self
    }

    pub fn expression(&mut self, expression: ExpressionNode) -> &mut Self {
        self.expression = Some(Box::new(expression));
        self
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct IfExpressionNode {
    pub(crate) pattern: Option<Box<ExpressionNode>>,
    pub(crate) then_keyword: Option<Token>,
    pub(crate) expression: Option<Box<ExpressionNode>>,
    pub(crate) else_clause: Option<Box<ExpressionNode>>,
}

impl IfExpressionNode {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn pattern(&mut self, pattern: ExpressionNode) -> &mut Self {
        self.pattern = Some(Box::new(pattern));
        self
    }

    pub fn then_keyword<T: Into<Option<Token>>>(&mut self, then_keyword: T) -> &mut Self {
        self.then_keyword = then_keyword.into();
        self
    }

    pub fn expression(&mut self, expression: ExpressionNode) -> &mut Self {
        self.expression = Some(Box::new(expression));
        self
    }

    pub fn else_clause(&mut self, else_clause: ExpressionNode) -> &mut Self {
        self.else_clause = Some(Box::new(else_clause));
        self
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct BinaryExpressionNode {
    pub(crate) operator: Option<Token>,
    pub(crate) lhs: Option<Box<ExpressionNode>>,
    pub(crate) rhs: Option<Box<ExpressionNode>>,
}

impl BinaryExpressionNode {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn operator<T: Into<Option<Token>>>(&mut self, operator: T) -> &mut Self {
        self.operator = operator.into();
        self
    }

    pub fn lhs(&mut self, lhs: ExpressionNode) -> &mut Self {
        self.lhs = Some(Box::new(lhs));
        self
    }

    pub fn rhs(&mut self, rhs: ExpressionNode) -> &mut Self {
        self.rhs = Some(Box::new(rhs));
        self
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct GroupedExpressionNode {
    pub(crate) start_delimiter: Option<Token>,
    pub(crate) expression: Option<Box<ExpressionNode>>,
    pub(crate) end_delimiter: Option<Token>,
}

impl GroupedExpressionNode {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn start_delimiter<T: Into<Option<Token>>>(&mut self, start_delimiter: T) -> &mut Self {
        self.start_delimiter = start_delimiter.into();
        self
    }

    pub fn expression(&mut self, expression: ExpressionNode) -> &mut Self {
        self.expression = Some(Box::new(expression));
        self
    }

    pub fn end_delimiter<T: Into<Option<Token>>>(&mut self, end_delimiter: T) -> &mut Self {
        self.end_delimiter = end_delimiter.into();
        self
    }
}
