use crate::tree::token::SyntaxToken;
use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BlockDeclarationNode {
    pub(crate) begin_token: SyntaxToken,
    pub(crate) declaration_list: Vec<Box<dyn DeclarationNode>>,
    pub(crate) end_token: SyntaxToken,
}

impl DeclarationNode for BlockDeclarationNode {
    fn span(&self) -> TextSpan {
        TextSpan::from_spans(self.begin_token.span(), self.end_token.span())
    }

    fn full_span(&self) -> TextSpan {
        TextSpan::from_spans(
            self.begin_token.full_span(),
            self.end_token.full_span()
        )
    }
}
