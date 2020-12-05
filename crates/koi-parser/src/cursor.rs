//! Abstractions for iterating over the characters of a Koi source file.
//!
//! This module exports the [`Cursor`] structure, which is responsible for
//! iterating over the characters of a source text. It also provides methods for
//! advancing to the next character and peeking a character at a given index.

use std::vec::IntoIter;

/// End-of-file character.
pub const EOF_CHAR: char = '\0';

/// A structure representing the current position in a Koi source text.
///
/// This type should not be manipulated directly. The [`Lexer`] is another
/// abstraction over this type that handles the iteration and tokenization of
/// a Koi source text for you. Please refer to its documentation for more
/// information.
///
/// [`Lexer`]: crate::lexer::Lexer
pub struct Cursor {
    chars: IntoIter<char>,
    pub(crate) pos: usize,
}

impl Cursor {
    /// Construct a new `Cursor` with the given source text.
    pub fn new(source: String) -> Self {
        Self {
            chars: source.chars().collect::<Vec<_>>().into_iter(),
            pos: 0,
        }
    }

    /// Advance to the next character in the iterator.
    pub fn advance(&mut self) -> Option<char> {
        self.chars.next().map(|next_char| {
            self.pos += 1;
            next_char
        })
    }

    /// The number of characters of the source text.
    pub fn source_len(&self) -> usize {
        self.chars.len()
    }

    /// Get a character of the source text at the given index.
    pub fn nth(&self, n: usize) -> char {
        self.chars.clone().nth(n).unwrap_or(EOF_CHAR)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cursor_empty() {
        let mut cursor = Cursor::new("".into());
        assert_eq!(cursor.source_len(), 0);
        assert_eq!(cursor.nth(0), EOF_CHAR);

        assert_eq!(cursor.advance(), None);
        assert_eq!(cursor.pos, 0);
    }

    #[test]
    fn test_cursor_with_source() {
        let mut cursor = Cursor::new("abc123".into());
        assert_eq!(cursor.source_len(), 6);
        assert_eq!(cursor.nth(0), 'a');
        assert_eq!(cursor.nth(1), 'b');
        assert_eq!(cursor.nth(2), 'c');

        assert_eq!(cursor.advance(), Some('a'));
        assert_eq!(cursor.advance(), Some('b'));
        assert_eq!(cursor.advance(), Some('c'));
        assert_eq!(cursor.pos, 3);

        assert_eq!(cursor.source_len(), 3);
        assert_eq!(cursor.nth(0), '1');
        assert_eq!(cursor.nth(1), '2');
        assert_eq!(cursor.nth(2), '3');
    }
}
