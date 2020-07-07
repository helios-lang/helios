use crate::source::Span;
use std::fmt::Debug;

pub mod block_expr;
pub mod identifier_expr;
pub mod if_expr;
pub mod local_binding_expr;

pub trait ExpressionNode: ExpressionNodeClone + Debug {
    fn span(&self) -> Span;
}

pub trait ExpressionNodeClone {
    fn clone_box(&self) -> Box<dyn ExpressionNode>;
}

impl<T: 'static + ExpressionNode + Clone> ExpressionNodeClone for T {
    fn clone_box(&self) -> Box<dyn ExpressionNode> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn ExpressionNode> {
    fn clone(&self) -> Box<dyn ExpressionNode> {
        self.clone_box()
    }
}

impl<'a> PartialEq for Box<dyn ExpressionNode + 'a> {
    fn eq(&self, other: &Self) -> bool {
        &self == &other
    }
}

impl Eq for Box<dyn ExpressionNode> {}
