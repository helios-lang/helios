use super::*;

#[derive(Debug)]
pub struct FunctionDeclarationNode;

impl DeclarationNode for FunctionDeclarationNode {
    fn span(&self) -> Span {
        Span::default()
    }
}
