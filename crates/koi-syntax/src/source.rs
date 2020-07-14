#![allow(dead_code)]

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
