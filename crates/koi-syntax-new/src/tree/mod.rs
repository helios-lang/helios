pub mod node;
pub mod token;

use node::SyntaxNode;
use token::SyntaxToken;
use std::rc::Rc;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Syntax {
    Node(Rc<SyntaxNode>),
    Token(Rc<SyntaxToken>),
}
