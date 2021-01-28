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

use self::lexer::{Lexer, Token};
pub use self::message::*;
use self::parser::sink::Sink;
use self::parser::source::Source;
use self::parser::Parser;
use helios_syntax::{SyntaxKind, SyntaxNode};
use rowan::GreenNode;

/// Tokenizes the given source text.
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

#[allow(dead_code)]
fn process_indentations(tokens: Vec<Token>) -> Vec<Token> {
    macro_rules! last {
        ($stack:expr) => {
            *$stack.last().unwrap_or(&0)
        };
    }

    let mut processed_tokens = tokens.clone();
    let mut indent_stack = vec![0];

    for (i, token) in tokens.iter().enumerate() {
        if token.kind == SyntaxKind::Newline {
            // Skip the newline character and count the number of spaces left.
            let curr_indent = token.text[1..].len();
            let last_indent = last!(indent_stack);

            if curr_indent > last_indent {
                indent_stack.push(curr_indent);
                processed_tokens[i].kind = SyntaxKind::Indent;
            } else if curr_indent < last_indent {
                let mut dedents = Vec::new();
                let curr_indent = curr_indent;

                // If our current indent is still smaller than the last indent
                // level, we'll continue popping and inserting a zero-width
                // `Dedent` token.
                while curr_indent < last!(indent_stack) {
                    indent_stack.pop();

                    if indent_stack.is_empty() {
                        break;
                    }

                    dedents.push(Token::new(
                        SyntaxKind::Dedent,
                        token.text,
                        token.range.end..token.range.end,
                    ))
                }

                // Replace the current `Newline` token with the appropriate
                // number of dedents.
                processed_tokens.splice(i..(i + 1), dedents);
            }
        }
    }

    // Push the remaining number of `Dedent` tokens left.
    let last_token_range = processed_tokens.last().unwrap().range.clone();
    while let Some(indent) = indent_stack.pop() {
        // We won't push a `Dedent` token if we're at the 0th column.
        if indent == 0 {
            break;
        }

        processed_tokens.push(Token::new(
            SyntaxKind::Dedent,
            "",
            last_token_range.end..last_token_range.end,
        ))
    }

    processed_tokens
}

// #[test]
// fn test_whitespace() {
//     let source = "\
// let
//   x = 1
//   y = 2
//   z = 3
// let
//   a =
//     x + y
//   b = a * 2";
//     let (tokens, _) = tokenize(0u8, source);
//     let tokens = process_indentations(tokens);
//     for token in tokens {
//         println!("{:?}", token);
//     }
// }

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
