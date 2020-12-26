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

use self::lexer::Lexer;
use self::parser::sink::Sink;
use self::parser::source::Source;
use self::parser::Parser;
use flume::Sender;
use helios_syntax::SyntaxNode;
pub use message::Message;
use rowan::GreenNode;

pub type FileId = usize;

/// The entry point of the parsing process.
///
/// This function parses the given source text (a `&str`) and returns a
/// [`Parse`], which holds a [`GreenNode`] tree describing the structure of a
/// Helios program.
pub fn parse(
    file_id: FileId,
    source: &str,
    messages_tx: Sender<Message>,
) -> Parse {
    let tokens =
        Lexer::new(file_id, source, messages_tx.clone()).collect::<Vec<_>>();
    let source = Source::new(&tokens);
    let parser = Parser::new(file_id, source, messages_tx.clone());
    let events = parser.parse();
    let sink = Sink::new(&tokens, events);

    sink.finish()
}

/// The result of parsing a source text.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Parse {
    /// The root green node of the syntax tree.
    green_node: GreenNode,
}

impl Parse {
    /// Construct a [`Parse`] with the given [`GreenNode`].
    pub fn new(green_node: GreenNode) -> Self {
        Self { green_node }
    }

    /// Returns a formatted string representation of the syntax tree.
    pub fn debug_tree(&self) -> String {
        let syntax_node = SyntaxNode::new_root(self.green_node.clone());
        format!("{:#?}", syntax_node)
    }
}

#[cfg(test)]
fn check(input: &str, expected_tree: expect_test::Expect) {
    let (messages_tx, _) = flume::unbounded();
    let parse = parse(0, input, messages_tx.clone());
    expected_tree.assert_eq(&parse.debug_tree());
}
