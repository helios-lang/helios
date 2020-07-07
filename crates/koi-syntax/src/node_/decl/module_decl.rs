use crate::token::Token;
use super::*;
use std::sync::Arc;

#[derive(Debug)]
pub struct ModuleDeclarationNode {
    pub(crate) module_keyword: Token,
    pub(crate) identifier: Token,
    pub(crate) equal_symbol: Token,
    pub(crate) decl_block: Arc<dyn DeclarationNode>,
}

impl DeclarationNode for ModuleDeclarationNode {
    fn span(&self) -> Span {
        Span::from_bounds(self.module_keyword.span, self.decl_block.span())
    }
}
