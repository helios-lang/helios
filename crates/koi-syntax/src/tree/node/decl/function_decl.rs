use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FunctionDeclarationNode;

impl DeclarationNode for FunctionDeclarationNode {
    fn span(&self) -> TextSpan {
        TextSpan::default()
    }

    fn full_span(&self) -> TextSpan {
        TextSpan::default()
    }
}
