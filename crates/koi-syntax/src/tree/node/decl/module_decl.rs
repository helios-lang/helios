use crate::tree::token::SyntaxToken;
use super::*;

#[derive(Clone, Debug, Eq)]
pub struct ModuleDeclarationNode {
    pub(crate) module_keyword: SyntaxToken,
    pub(crate) identifier: SyntaxToken,
    pub(crate) equal_symbol: SyntaxToken,
    pub(crate) decl_block: Box<dyn DeclarationNode>,
}

impl DeclarationNode for ModuleDeclarationNode {
    fn span(&self) -> TextSpan {
        TextSpan::from_spans(self.module_keyword.span(), self.decl_block.span())
    }

    fn full_span(&self) -> TextSpan {
        TextSpan::from_spans(
            self.module_keyword.full_span(),
            self.decl_block.full_span()
        )
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
