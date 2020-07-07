use crate::token::Token;
use super::*;

#[derive(Clone, Debug, Eq)]
pub struct ModuleDeclarationNode {
    pub(crate) module_keyword: Token,
    pub(crate) identifier: Token,
    pub(crate) equal_symbol: Token,
    pub(crate) decl_block: Box<dyn DeclarationNode>,
}

impl DeclarationNode for ModuleDeclarationNode {
    fn span(&self) -> Span {
        Span::from_bounds(self.module_keyword.span, self.decl_block.span())
    }
}

impl PartialEq for ModuleDeclarationNode {
    fn eq(&self, other: &Self) -> bool {
        self.module_keyword == other.module_keyword
            && self.identifier == other.identifier
            && self.equal_symbol == other.equal_symbol
            && &self.decl_block == &other.decl_block
    }
}
