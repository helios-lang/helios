use super::Node;
use crate::token::*;
use std::default::Default;
use std::fmt::Debug;

#[derive(Debug)]
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
    BinaryExpression(Token, Box<ExpressionNode>, Box<ExpressionNode>),

    /// A grouped expression (constructed when an expression is parenthesised).
    GroupedExpression(Box<ExpressionNode>),

    /// An indented block of expressions.
    BlockExpression(Vec<Box<ExpressionNode>>),

    /// An unexpected token kind.
    Unexpected(TokenKind),

    /// A missing expression node.
    Missing,
}

#[derive(Debug)]
pub enum LiteralNode {
    Boolean(bool),

    /// A float literal.
    Float(Base),

    /// An integer literal.
    Integer(Base),
}

#[derive(Debug, Default)]
pub struct LocalBindingNode {
    parent: Option<Box<Node>>,
    identifier: Option<Token>,
    equal_symbol: Option<Token>,
    expression: Option<Box<ExpressionNode>>,
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

#[derive(Debug, Default)]
pub struct IfExpressionNode {
    pattern: Option<Box<ExpressionNode>>,
    then_keyword: Option<Token>,
    expression: Option<Box<ExpressionNode>>,
    else_clause: Option<Box<ExpressionNode>>,
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
