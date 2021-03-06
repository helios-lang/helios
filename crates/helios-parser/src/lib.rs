//! Parsing Helios source files.
//!
//! The showrunner of this module is the [`parse`] function. It is responsible
//! for parsing a given input and returning a concrete syntax tree (CST) with
//! the [`rowan`] library.
//!
//! [`rowan`]: https://docs.rs/rowan/0.10.0/rowan

mod cursor;
mod grammar;
mod lexer;
pub mod message;
mod parser;

pub use self::message::*;

use self::lexer::{Lexer, Token};
use self::parser::sink::Sink;
use self::parser::source::Source;
use self::parser::Parser;
use helios_syntax::SyntaxNode;
use rowan::GreenNode;

pub fn tokenize<FileId>(
    file_id: FileId,
    source: &str,
) -> (Vec<Token>, Vec<Message<FileId>>)
where
    FileId: Clone + Default,
{
    let mut tokens = Vec::new();
    let mut errors = Vec::new();

    for (token, error) in Lexer::new(file_id, source) {
        tokens.push(token);
        if let Some(error) = error {
            errors.push(error.into());
        }
    }

    (tokens, errors)
}

/// The entry point of the parsing process.
///
/// This function parses the given source text (a `&str`) and returns a
/// [`Parse`], which holds a [`GreenNode`] tree describing the structure of a
/// Helios program.
pub fn parse<FileId>(file_id: FileId, source: &str) -> Parse<FileId>
where
    FileId: Clone + Default,
{
    let (tokens, mut messages) = tokenize(file_id.clone(), source);
    let source = Source::new(&tokens);

    let parser = Parser::new(file_id, source);
    let (events, parser_messages) = parser.parse();
    let sink = Sink::new(&tokens, events);

    messages.extend(parser_messages);
    sink.finish(messages)
}

/// The result of parsing a source text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parse<FileId> {
    /// The root green node of the syntax tree.
    green_node: GreenNode,
    messages: Vec<Message<FileId>>,
}

impl<FileId> Parse<FileId> {
    /// Construct a [`Parse`] with the given [`GreenNode`].
    pub fn new(green_node: GreenNode, messages: Vec<Message<FileId>>) -> Self {
        Self {
            green_node,
            messages,
        }
    }

    pub fn syntax(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.green_node.clone())
    }

    pub fn messages(&self) -> &[Message<FileId>] {
        &self.messages
    }

    /// Returns a formatted string representation of the syntax tree.
    pub fn debug_tree(&self) -> String {
        let syntax_node = SyntaxNode::new_root(self.green_node.clone());
        format!("{:#?}", syntax_node)
    }
}

#[cfg(test)]
fn check(input: &str, expected_tree: expect_test::Expect) {
    let parse = parse(0u8, input);
    expected_tree.assert_eq(&parse.debug_tree());
}
