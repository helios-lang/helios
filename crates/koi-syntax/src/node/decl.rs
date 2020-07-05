use std::fmt::Debug;

#[derive(Debug)]
pub enum DeclarationNode {
    GlobalBindingDeclaration,
    ModuleDeclaration,
    TypeDeclaration,
}
