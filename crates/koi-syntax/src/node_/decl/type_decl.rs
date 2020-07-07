use crate::token::Token;
use super::*;
use std::sync::Arc;

#[derive(Debug)]
pub struct TypeDeclarationNode {
    pub(crate) type_keyword: Token,
    pub(crate) identifier: Token,
    pub(crate) equal_symbol: Token,
    pub(crate) decl_block: Arc<dyn DeclarationNode>,
}

impl DeclarationNode for TypeDeclarationNode {
    fn span(&self) -> Span {
        Span::from_bounds(self.type_keyword.span, self.equal_symbol.span)
    }
}
