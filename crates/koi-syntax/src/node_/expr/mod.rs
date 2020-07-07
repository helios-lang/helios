use crate::source::Span;
use std::fmt::Debug;

pub mod block_expr;
pub mod identifier_expr;
pub mod if_expr;
pub mod local_binding_expr;

pub trait ExpressionNode: Debug {
    fn span(&self) -> Span;
}
