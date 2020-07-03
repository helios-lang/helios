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
    LocalBinding(Pattern, Box<Expr>),
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
