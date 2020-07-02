use crate::expr::Expr;

#[derive(Debug)]
pub enum Decl {
    GlobalBinding(String, Expr),
    Module,
    Type,
}
