use crate::token::{Base, Token, TokenKind};

#[derive(Debug)]
pub enum Expr {
    /// A literal expression that is translated exactly as written in source.
    Literal(ExprLiteral),

    /// A named reference to an identifier.
    Identifier,

    /// A unary expression holding a token (signifying the operator) and an
    /// expression (signifying the right hand side of the operation).
    Unary(Token, Box<Expr>),

    /// A binary expression holding a token (signifying the operator) and two
    /// expressions (signifying the left and right hand sides of the operation).
    Binary(Token, Box<Expr>, Box<Expr>),

    /// A grouped expression (constructed when an expression is parenthesised).
    Grouping(Box<Expr>),

    /// An indented block of expressions.
    ExprBlock(Vec<Box<Expr>>),

    /// A local binding expression.
    LocalBindingExpr(LocalBinding),

    /// An if-branching expression.
    IfExpr(IfExpr),

    /// An unexpected token.
    Unexpected(TokenKind),

    /// A missing expression node.
    Missing,
}

#[derive(Debug)]
pub enum ExprLiteral {
    /// A boolean literal.
    Bool(bool),

    /// A float literal.
    Float(Base),

    /// An integer literal.
    Integer(Base),

    /// A string literal.
    String(String),
}

#[derive(Debug, Default)]
pub struct LocalBinding {
    pub identifier: Option<Token>,
    pub equal_symbol: Option<Token>,
    pub expression: Option<Box<Expr>>,
}

impl LocalBinding {
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

    pub fn expression(&mut self, expression: Expr) -> &mut Self {
        self.expression = Some(Box::new(expression));
        self
    }
}

#[derive(Debug, Default)]
pub struct IfExpr {
    pub pattern: Option<Box<Expr>>,
    pub then_keyword: Option<Token>,
    pub expression: Option<Box<Expr>>,
    pub else_clause: Option<Box<Expr>>,
}

impl IfExpr {
    pub fn new() -> Self {
        Self::default()
    }
}
