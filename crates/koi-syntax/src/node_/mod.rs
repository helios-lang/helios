pub mod expr;
pub mod decl;

use crate::source::Span;
pub use expr::*;
pub use decl::*;
use std::sync::Arc;

pub type Ast = Nodes;

#[derive(Debug)]
pub struct Nodes(Vec<Node>);

impl Nodes {
    pub fn span(&self) -> Span {
        Span::from_bounds(
            self.0.first().map(|node| node.span()).unwrap_or_default(),
            self.0.last().map(|node| node.span()).unwrap_or_default()
        )
    }
}

#[derive(Debug)]
enum Node {
    DeclarationNode(Arc<dyn DeclarationNode>),
    ExpressionNode(Arc<dyn ExpressionNode>),
}

impl Node {
    fn span(&self) -> Span {
        match self {
            Self::DeclarationNode(node) => node.span(),
            Self::ExpressionNode(node) => node.span(),
        }
    }
}
