use crate::tree::token::SyntaxToken;
use super::*;

#[derive(Clone, Debug, Eq)]
pub struct TypeDeclarationNode {
    pub(crate) type_keyword: SyntaxToken,
    pub(crate) identifier: SyntaxToken,
    pub(crate) equal_symbol: SyntaxToken,
    pub(crate) decl_block: Box<dyn DeclarationNode>,
}

impl DeclarationNode for TypeDeclarationNode {
    fn span(&self) -> TextSpan {
        TextSpan::from_spans(self.type_keyword.span(), self.equal_symbol.span())
    }

    fn full_span(&self) -> TextSpan {
        TextSpan::from_spans(
            self.type_keyword.full_span(),
            self.equal_symbol.full_span()
        )
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
