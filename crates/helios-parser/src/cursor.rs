//! Abstractions for iterating over the characters of a Helios source file.
//!
//! This module exports the [`Cursor`] structure, which is responsible for
//! iterating over the characters of a source text. It also provides methods for
//! advancing to the next character and peeking a character at a given index.

use std::ops::Range;
use std::str::Chars;

/// End-of-file character.
const EOF_CHAR: char = '\0';

/// A structure representing the current position in a Helios source text.
///
/// This type should not be manipulated directly. The [`Lexer`] is another
/// abstraction over this type that handles the iteration and tokenization of
/// a Helios source text for you. Please refer to its documentation for more
/// information.
///
/// [`Lexer`]: crate::lexer::Lexer
pub struct Cursor<'source> {
    chars: Chars<'source>,
    source: &'source str,
    pos: usize,
    checkpoints: Vec<usize>,
}

impl<'source> Cursor<'source> {
    /// Construct a new `Cursor` with the given source text.
    pub fn new(source: &'source str) -> Self {
        Self {
            chars: source.chars(),
            source,
            pos: 0,
            checkpoints: Vec::new(),
        }
    }

    /// Advance to the next character in the iterator.
    pub fn advance(&mut self) -> Option<char> {
        self.chars.next().map(|next_char| {
            self.pos += next_char.len_utf8();
            next_char
        })
    }

    /// Creates a new checkpoint.
    ///
    /// A checkpoint is a marked position of interest in the source text. It is
    /// used by the [`Cursor::slice`] method to return a slice of
    /// consumed<sup>[1](#mark-checkpoint-footnote)</sup> characters from the
    /// last-marked checkpoint. Use this method to guarantee that the marked
    /// position is always valid.
    ///
    /// <a name="mark-checkpoint-footnote">1</a>: Technically, the cursor
    /// doesn't “consume” the input (it merely acts as a window over a `&str`),
    /// however, this detail is abstracted away and thus doesn't really matter.
    ///
    /// [`Cursor::slice`]: crate::cursor::Cursor::slice
    #[inline]
    pub fn checkpoint(&mut self) {
        self.checkpoints.push(self.pos);
    }

    /// The range of the consumed tokens from the last-marked checkpoint.
    ///
    /// This method will remove the most recent checkpoint from the stack before
    /// returning the range (hence why this method requires `&mut self`).
    #[inline]
    pub fn span(&mut self) -> Range<usize> {
        (self.checkpoints.pop().unwrap_or_default())..self.pos
    }

    /// Returns a slice of the source text from the last-marked checkpoint to
    /// the current cursor position.
    ///
    /// The most recent checkpoint will be removed from the stack. A checkpoint
    /// can be created with the [`Cursor::checkpoint`] method. Refer to
    /// its documentation for more information.
    ///
    /// [`Cursor::checkpoint`]: crate::cursor::Cursor::checkpoint
    #[inline]
    pub fn slice(&mut self) -> &'source str {
        unsafe { self.source.get_unchecked(self.span()) }
    }

    /// The number of characters of the source text in full.
    #[inline]
    pub fn source_len(&self) -> usize {
        self.source.len()
    }

    /// The remaining length of the unprocessed input.
    #[allow(dead_code)]
    #[inline]
    pub fn remaining_len(&self) -> usize {
        self.source_len() - self.pos()
    }

    /// The current position of the cursor.
    #[inline]
    pub fn pos(&self) -> usize {
        self.pos
    }

    /// Returns the character of the source text at the given index.
    #[inline]
    pub fn nth(&self, n: usize) -> char {
        self.chars.clone().nth(n).unwrap_or(EOF_CHAR)
    }

    /// Checks if the cursor has reached the end of the input.
    #[inline]
    pub fn is_at_end(&self) -> bool {
        self.pos() >= self.source_len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cursor_empty() {
        let mut cursor = Cursor::new("");
        assert_eq!(cursor.source_len(), 0);

        // Peeking out-of-bounds character
        assert_eq!(cursor.remaining_len(), 0);
        assert_eq!(cursor.nth(0), EOF_CHAR);

        // Try to consume out-of-bounds character
        assert_eq!(cursor.advance(), None);
        assert_eq!(cursor.pos(), 0);
        assert!(cursor.is_at_end());
    }

    #[test]
    fn test_cursor_with_source() {
        let mut cursor = Cursor::new("abc123");
        assert_eq!(cursor.source_len(), 6);

        // Peeking first three characters
        assert_eq!(cursor.remaining_len(), 6);
        assert_eq!(cursor.nth(0), 'a');
        assert_eq!(cursor.nth(1), 'b');
        assert_eq!(cursor.nth(2), 'c');

        // Consuming first three characters
        assert_eq!(cursor.advance(), Some('a'));
        assert_eq!(cursor.advance(), Some('b'));
        assert_eq!(cursor.advance(), Some('c'));
        assert_eq!(cursor.pos(), 3);
        assert!(!cursor.is_at_end());

        // Peeking next three characters
        assert_eq!(cursor.remaining_len(), 3);
        assert_eq!(cursor.nth(0), '1');
        assert_eq!(cursor.nth(1), '2');
        assert_eq!(cursor.nth(2), '3');

        // Consuming next three characters
        assert_eq!(cursor.advance(), Some('1'));
        assert_eq!(cursor.advance(), Some('2'));
        assert_eq!(cursor.advance(), Some('3'));
        assert_eq!(cursor.pos(), 6);
        assert!(cursor.is_at_end());

        // Peeking out-of-bounds character
        assert_eq!(cursor.remaining_len(), 0);
        assert_eq!(cursor.nth(0), EOF_CHAR);

        // Try to consume out-of-bounds character
        assert_eq!(cursor.advance(), None);
        assert_eq!(cursor.pos(), 6);
        assert!(cursor.is_at_end());
    }

    #[test]
    fn test_cursor_slice() {
        let mut cursor = Cursor::new("hello, world!");
        assert_eq!(cursor.source_len(), 13);

        cursor.checkpoint();
        assert_eq!(cursor.slice(), "");

        cursor.checkpoint();
        assert_eq!(cursor.advance(), Some('h'));
        assert_eq!(cursor.advance(), Some('e'));
        assert_eq!(cursor.advance(), Some('l'));
        assert_eq!(cursor.advance(), Some('l'));
        assert_eq!(cursor.advance(), Some('o'));
        assert_eq!(cursor.slice(), "hello");

        cursor.checkpoint();
        assert_eq!(cursor.advance(), Some(','));
        assert_eq!(cursor.advance(), Some(' '));
        assert_eq!(cursor.advance(), Some('w'));
        assert_eq!(cursor.advance(), Some('o'));
        assert_eq!(cursor.advance(), Some('r'));
        assert_eq!(cursor.advance(), Some('l'));
        assert_eq!(cursor.advance(), Some('d'));
        assert_eq!(cursor.slice(), ", world");

        cursor.checkpoint();
        assert_eq!(cursor.advance(), Some('!'));
        assert_eq!(cursor.advance(), None);
        assert_eq!(cursor.slice(), "!");
    }

    #[test]
    fn test_cursor_slice_with_unicode() {
        let mut cursor = Cursor::new("héllö, wørl∂¿");
        assert_eq!(cursor.source_len(), 19);

        cursor.checkpoint();
        assert_eq!(cursor.advance(), Some('h'));
        assert_eq!(cursor.advance(), Some('é'));
        assert_eq!(cursor.advance(), Some('l'));
        assert_eq!(cursor.advance(), Some('l'));
        assert_eq!(cursor.advance(), Some('ö'));
        assert_eq!(cursor.slice(), "héllö");

        cursor.checkpoint();
        assert_eq!(cursor.advance(), Some(','));
        assert_eq!(cursor.advance(), Some(' '));
        assert_eq!(cursor.advance(), Some('w'));
        assert_eq!(cursor.advance(), Some('ø'));
        assert_eq!(cursor.advance(), Some('r'));
        assert_eq!(cursor.advance(), Some('l'));
        assert_eq!(cursor.advance(), Some('∂'));
        assert_eq!(cursor.slice(), ", wørl∂");

        cursor.checkpoint();
        assert_eq!(cursor.advance(), Some('¿'));
        assert_eq!(cursor.advance(), None);
        assert_eq!(cursor.slice(), "¿");
    }
}
