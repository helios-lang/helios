use std::fmt::{self, Display};
use std::ops::Range;

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
                "TextSpan end position ({}) must not be less than its start \
                 position ({})",
                end,
                start,
            }
        }

        let length = end - start;
        Self { start, length }
    }

    /// Creates a new `TextSpan` that covers the boundaries of the `TextSpan`s.
    ///
    /// The new `TextSpan` will start from the start offset of the first
    /// `TextSpan` and end at the end offset of the second `TextSpan`.
    pub fn from_spans(start: Self, end: Self) -> Self {
        Self::from_bounds(start.start(), end.end())
    }

    /// Creates a new zero-width `TextSpan` (for spanning items that have a
    /// length of 0).
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

impl From<Range<usize>> for TextSpan {
    fn from(range: Range<usize>) -> Self {
        Self {
            start: range.start,
            length: range.len(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(range: Range<usize>, span: TextSpan) {
        assert_eq!(TextSpan::from(range), span);
    }

    #[test]
    fn test_text_span_from_range() {
        check(0..2, TextSpan::new(0, 2));
        check(5..11, TextSpan::new(5, 6));
    }
}
