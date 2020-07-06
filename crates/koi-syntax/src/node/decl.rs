use std::fmt::Debug;

#[derive(Clone, Eq, Debug, PartialEq)]
pub enum DeclarationNode {
    GlobalBindingDeclaration,
    ModuleDeclaration,
    TypeDeclaration,
}
