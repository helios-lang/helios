use crate::token::Token;

#[derive(Debug)]
pub enum Expr {
    /// A literal expression that is translated exactly as written in source.
    Literal(ExprLiteral),

    /// A named reference to an identifier.
    Identifier(String),

    /// A unary expression holding a token (signifying the operator) and an
    /// expression (signifying the right hand side of the operation).
    Unary(Token, Box<Expr>),

    /// A binary expression holding a token (signifying the operator) and two
    /// expressions (signifying the left and right hand sides of the operation).
    Binary(Token, Box<Expr>, Box<Expr>),

    /// A grouped expression (constructed when an expression is parenthesised).
    Grouping(Box<Expr>),

    /// A local binding expression.
    LocalBindingExpr(LocalBinding),

    /// An if-branching expression.
    IfExpr(IfExpr),
}

#[derive(Debug)]
pub enum ExprLiteral {
    /// A boolean literal.
    Bool(bool),

    /// A float literal.
    Float(f64),

    /// An integer literal.
    Int(i32),

    /// A string literal.
    Str(String),
}

#[derive(Debug)]
pub enum Pattern {
    Identifier(String),
    Missing,
}

#[derive(Debug, Default)]
pub struct LocalBinding {
    pub pattern: Option<Pattern>,
    pub equal_symbol: Option<Token>,
    pub expression: Option<Box<Expr>>,
}

impl LocalBinding {
    pub fn new() -> Self {
        Self::default()
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
