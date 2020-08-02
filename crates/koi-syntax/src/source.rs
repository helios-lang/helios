use std::vec::IntoIter;
use std::fmt::{self, Display};

pub const EOF_CHAR: char = '\0';

/// Describes the start offset and length of a given node, token or trivia.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct TextSpan {
    start: usize,
    length: usize,
}

impl TextSpan {
    /// Creates a new `TextSpan` with the given start offset and length.
    pub fn new(start: usize, length: usize) -> Self {
        Self { start, length }
    }

    /// Creates a new `TextSpan` within the bounds of the given start and end
    /// offsets. This function will assert that the end position is equal to or
    /// greater than the start position.
    pub fn from_bounds(start: usize, end: usize) -> Self {
        assert! {
            end >= start,
            format! {
                "end position of TextSpan ({}) must not be less than its start position ({})",
                end,
                start,
            }
        }

        let length = end - start;
        Self { start, length }
    }

    pub fn from_spans(start: Self, end: Self) -> Self {
        Self::from_bounds(start.start(), end.end())
    }

    pub fn zero_width(start: usize) -> Self {
        Self::new(start, 0)
    }

    /// The start position of the given spanning item.
    ///
    /// This offset is a zero-based UTF-8 character index into the source text.
    pub fn start(&self) -> usize {
        self.start
    }

    /// The length of the given spanning item.
    pub fn length(&self) -> usize {
        self.length
    }

    /// The end position of the given spanning item.
    pub fn end(&self) -> usize {
        self.start + self.length
    }
}

impl Display for TextSpan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}..{}", self.start, self.end())
    }
}

pub(crate) struct Cursor {
    chars: IntoIter<char>,
    pub(crate) pos: usize,
}

impl Cursor {
    pub(crate) fn with(source: String) -> Self {
        Self {
            chars: source.chars().collect::<Vec<_>>().into_iter(),
            pos: 0,
        }
    }

    /// Advances to the next character in the iterator.
    pub(crate) fn advance(&mut self) -> Option<char> {
        self.chars.next().map(|next_char| {
            self.pos += 1;
            next_char
        })
    }

    pub(crate) fn source_len(&self) -> usize {
        self.chars.len()
    }

    pub(crate) fn nth(&self, n: usize) -> char {
        self.chars.clone().nth(n).unwrap_or(EOF_CHAR)
    }
}
