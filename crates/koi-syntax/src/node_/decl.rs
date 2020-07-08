use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DeclarationNode {}

impl Spanning for DeclarationNode {
    fn span(&self) -> Span {
        Span::default()
    }
}
