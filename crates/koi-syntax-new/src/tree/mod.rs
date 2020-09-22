pub mod factory;
pub mod factory_new;
pub mod node;
pub mod token;

use crate::source::TextSpan;
use node::*;
use token::*;
use std::rc::Rc;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum RawSyntax {
    Node(Rc<RawSyntaxNode>),
    Token(Rc<RawSyntaxToken>),
}

impl RawSyntax {
    fn combined_text_value(&self) -> String {
        match self {
            Self::Node(node) => node.combined_text_value(),
            Self::Token(token) => token.text.clone(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Syntax {
    Node(Rc<SyntaxNode>),
    Token(Rc<SyntaxToken>),
}

impl Syntax {
    pub fn raw(&self) -> RawSyntax {
        match self {
            Self::Node(node) => RawSyntax::Node(Rc::clone(&node.raw)),
            Self::Token(token) => RawSyntax::Token(Rc::clone(&token.raw)),
        }
    }

    pub fn children(&self) -> Box<dyn Iterator<Item=&Syntax> + '_> {
        match self {
            Self::Node(node) => Box::new(node.children()),
            Self::Token(_) => Box::new(std::iter::empty()),
        }
    }

    pub fn span(&self) -> TextSpan {
        match self {
            Self::Node(node) => node.span(),
            Self::Token(token) => token.span(),
        }
    }

    pub fn full_span(&self) -> TextSpan {
        match self {
            Self::Node(node) => node.full_span(),
            Self::Token(token) => token.full_span(),
        }
    }
}

impl From<SyntaxNode> for Syntax {
    fn from(node: SyntaxNode) -> Self {
        Self::Node(Rc::new(node))
    }
}

impl From<SyntaxToken> for Syntax {
    fn from(token: SyntaxToken) -> Self {
        Self::Token(Rc::new(token))
    }
}
