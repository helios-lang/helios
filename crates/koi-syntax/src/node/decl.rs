use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub enum DeclarationNode {
    GlobalBindingDeclaration,
    ModuleDeclaration,
    TypeDeclaration,
}
