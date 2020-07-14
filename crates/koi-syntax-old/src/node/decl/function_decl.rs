use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FunctionDeclarationNode;

impl DeclarationNode for FunctionDeclarationNode {
    fn span(&self) -> Span {
        Span::default()
    }
}
