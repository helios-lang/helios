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
    identifier: Option<Token>,
    equal_symbol: Option<Token>,
    expression: Option<Box<Expr>>,
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
    pattern: Option<Box<Expr>>,
    then_keyword: Option<Token>,
    expression: Option<Box<Expr>>,
    else_clause: Option<Box<Expr>>,
}

impl IfExpr {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn pattern(&mut self, pattern: Expr) -> &mut Self {
        self.pattern = Some(Box::new(pattern));
        self
    }

    pub fn then_keyword<T: Into<Option<Token>>>(&mut self, then_keyword: T) -> &mut Self {
        self.then_keyword = then_keyword.into();
        self
    }

    pub fn expression(&mut self, expression: Expr) -> &mut Self {
        self.expression = Some(Box::new(expression));
        self
    }

    pub fn else_clause(&mut self, else_clause: Expr) -> &mut Self {
        self.else_clause = Some(Box::new(else_clause));
        self
    }
}
