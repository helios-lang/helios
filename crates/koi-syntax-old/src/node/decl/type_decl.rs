use crate::token::Token;
use super::*;

#[derive(Clone, Debug, Eq)]
pub struct TypeDeclarationNode {
    pub(crate) type_keyword: Token,
    pub(crate) identifier: Token,
    pub(crate) equal_symbol: Token,
    pub(crate) decl_block: Box<dyn DeclarationNode>,
}

impl DeclarationNode for TypeDeclarationNode {
    fn span(&self) -> Span {
        Span::from_bounds(self.type_keyword.span, self.equal_symbol.span)
    }
}

impl PartialEq for TypeDeclarationNode {
    fn eq(&self, other: &Self) -> bool {
        self.type_keyword == other.type_keyword
            && self.identifier == other.identifier
            && self.equal_symbol == other.equal_symbol
            && &self.decl_block == &other.decl_block
    }
}
