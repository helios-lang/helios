use crate::token::Token;

#[derive(Debug)]
pub enum Expr {
    /// A literal expression that is translated exactly as written in source.
    Literal,

    /// A named reference to an identity.
    Ident(String),

    Unary(Token, Box<Expr>),

    /// A binary expression holding a token (signifying the operator) and two
    /// expressions (signifying the left and right hand sides of the operation).
    Binary(Token, Box<Expr>, Box<Expr>),

    /// A grouped expression (constructed when an expression is parenthesised).
    Group(Box<Expr>),

    /// An expression error variant signifying that the syntax tree contains a
    /// missing expression.
    Missing,

    /// An expression error variant signifying the following token was
    /// unexpected (will be used for diagnostics).
    Unexpected(Token)
}
