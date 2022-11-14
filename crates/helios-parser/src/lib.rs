//! Parsing Helios source files.
//!
//! The showrunner of this module is the [`parse`] function. It is responsible
//! for parsing a given input and returning a concrete syntax tree (CST) with
//! the [`rowan`] library.
//!
//! [`rowan`]: https://docs.rs/rowan/0.15.3/rowan

mod cursor;
mod grammar;
mod lexer;
pub mod message;
mod parser;

use helios_syntax::{SyntaxKind, SyntaxNode};
use rowan::GreenNode;
use std::cmp::Ordering;

use crate::lexer::{Lexer, Token};
pub use crate::message::*;
use crate::parser::sink::Sink;
use crate::parser::source::Source;
use crate::parser::Parser;

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
            errors.push(error);
        }
    }

    (tokens, errors)
}

/// Processes indentation for a given vector of tokens by inserting indent and
/// dedent tokens where appropriate and returning a new vector with these
/// changes.
///
/// Since the [`Lexer`] structure and the [`tokenize`] function only emit
/// `Newline` tokens on every line feed it encounters, any subsequent operations
/// that require a tokenized output with indentation tokens (such as the parser)
/// cannot use their outputs. This function is able to act as an intermediary by
/// processing these indentations for you. It is ideal to call this function
/// right after tokenizing.
pub fn process_indents<'source>(
    source: &'source str,
    tokens: Vec<Token<'source>>,
) -> Vec<Token<'source>> {
    // Our resulting vector will have at least the same size as the input vector
    // (in the case that there is no indentation to be processed).
    let mut processed_tokens = Vec::with_capacity(tokens.capacity());
    let mut indent_stack = vec![0];

    let mut i = 0;
    while i < tokens.len() {
        // TODO: assert!(indent_stack.is_sorted());
        let curr_token = tokens[i].clone();

        if curr_token.kind == SyntaxKind::Newline {
            // Skip the newline character and count the number of spaces.
            let curr_indent = curr_token.text[1..].len();
            let last_indent = indent_stack.last().unwrap_or(&0);

            match curr_indent.cmp(last_indent) {
                // We haven't indented or dedented, so push the token as-is.
                Ordering::Equal => {
                    processed_tokens.push(curr_token);
                    i += 1;
                }
                // We've indented, so we'll push an `Indent` token.
                Ordering::Greater => {
                    indent_stack.push(curr_indent);
                    processed_tokens.push(Token {
                        kind: SyntaxKind::Indent,
                        ..curr_token
                    });
                    i += 1;
                }
                // We've dedented, so we'll push as many `Dedent` tokens
                // necessary to get the new indentation level.
                Ordering::Less => {
                    'emit_dedents: loop {
                        // We won't push a dedent token just yet because we need
                        // to make sure the current indent is NOT greater than
                        // the second-last indent (`new_last_indent`).
                        let old_indent = indent_stack.pop().unwrap();
                        let new_last_indent = indent_stack.last().unwrap_or(&0);

                        match curr_indent.cmp(new_last_indent) {
                            // We can emit a dedent token for the old indent and
                            // continue this loop.
                            Ordering::Less => {
                                processed_tokens.push(Token {
                                    kind: SyntaxKind::Dedent,
                                    ..curr_token.clone()
                                });
                                continue 'emit_dedents;
                            }
                            // We can emit a dedent token for the old indent and
                            // break out of this loop.
                            Ordering::Equal => {
                                processed_tokens.push(Token {
                                    kind: SyntaxKind::Dedent,
                                    ..curr_token.clone()
                                });

                                // We've finished dealing with the current
                                // token, so we'll increment `i`.
                                i += 1;
                                break 'emit_dedents;
                            }
                            // The current indent is between the second-last
                            // and the last indents, signifying an incorrect
                            // dedent. Thus, we'll invalidate the whole line and
                            // emit an error token instead.
                            Ordering::Greater => {
                                let start = curr_token.range.start;
                                let mut end = curr_token.range.end;

                                // Skip the current newline token.
                                i += 1;

                                // Skip until we find the next newline token.
                                while i < tokens.len() {
                                    if tokens[i].kind == SyntaxKind::Newline {
                                        break;
                                    }

                                    end = tokens[i].range.end;
                                    i += 1;
                                }

                                processed_tokens.push(Token {
                                    kind: SyntaxKind::Error,
                                    text: &source[start..end],
                                    range: start..end,
                                });

                                // Put the old indent back as an indentation
                                // error doesn't indicate a dedent.
                                indent_stack.push(old_indent);
                                break 'emit_dedents;
                            }
                        }
                    }
                }
            }
        } else {
            // Push the token as-is.
            processed_tokens.push(curr_token);
            i += 1;
        }
    }

    // Emit any remainder dedents required.
    let end = processed_tokens.last().map(|t| t.range.end).unwrap_or(0);
    while let Some(indent) = indent_stack.pop() {
        // We won't emit a dedent token for the first column.
        if indent == 0 {
            break;
        }

        // Zero-width dedent token.
        processed_tokens.push(Token::new(SyntaxKind::Dedent, "", end..end));
    }

    processed_tokens
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
    let tokens = process_indents(source, tokens);
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

#[cfg(test)]
mod tests {
    use super::*;

    fn check(input: &str, expected_tokens: Vec<Token>) {
        let (tokens, _) = tokenize(0u8, input);
        let tokens = process_indents(input, tokens);
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_tokenize_empty_input() {
        check("", vec![]);
    }

    #[test]
    fn test_tokenize_simple_input() {
        check(
            "let x = 1",
            vec![
                Token::new(SyntaxKind::Kwd_Let, "let", 0..3),
                Token::new(SyntaxKind::Whitespace, " ", 3..4),
                Token::new(SyntaxKind::Identifier, "x", 4..5),
                Token::new(SyntaxKind::Whitespace, " ", 5..6),
                Token::new(SyntaxKind::Sym_Eq, "=", 6..7),
                Token::new(SyntaxKind::Whitespace, " ", 7..8),
                Token::new(SyntaxKind::Lit_Integer, "1", 8..9),
            ],
        );
    }

    #[test]
    fn test_tokenize_indented_input() {
        check(
            "let\n  x = 1\n  y = 2",
            vec![
                Token::new(SyntaxKind::Kwd_Let, "let", 0..3),
                Token::new(SyntaxKind::Indent, "\n  ", 3..6),
                Token::new(SyntaxKind::Identifier, "x", 6..7),
                Token::new(SyntaxKind::Whitespace, " ", 7..8),
                Token::new(SyntaxKind::Sym_Eq, "=", 8..9),
                Token::new(SyntaxKind::Whitespace, " ", 9..10),
                Token::new(SyntaxKind::Lit_Integer, "1", 10..11),
                Token::new(SyntaxKind::Newline, "\n  ", 11..14),
                Token::new(SyntaxKind::Identifier, "y", 14..15),
                Token::new(SyntaxKind::Whitespace, " ", 15..16),
                Token::new(SyntaxKind::Sym_Eq, "=", 16..17),
                Token::new(SyntaxKind::Whitespace, " ", 17..18),
                Token::new(SyntaxKind::Lit_Integer, "2", 18..19),
                Token::new(SyntaxKind::Dedent, "", 19..19),
            ],
        );
    }
}
