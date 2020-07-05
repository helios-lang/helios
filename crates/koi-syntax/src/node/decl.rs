use std::fmt::Debug;

#[derive(Clone, Debug, PartialEq)]
pub enum DeclarationNode {
    GlobalBindingDeclaration,
    ModuleDeclaration,
    TypeDeclaration,
}
