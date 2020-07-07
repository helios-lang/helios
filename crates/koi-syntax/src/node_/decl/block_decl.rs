use crate::token::Token;
use super::*;
use std::sync::Arc;

#[derive(Debug)]
pub struct BlockDeclarationNode {
    pub(crate) begin_token: Token,
    pub(crate) declaration_list: Vec<Arc<dyn DeclarationNode>>,
    pub(crate) end_token: Token,
}

impl DeclarationNode for BlockDeclarationNode {
    fn span(&self) -> Span {
        Span::from_bounds(self.begin_token.span, self.end_token.span)
    }
}
