use std::vec::IntoIter;

pub const EOF_CHAR: char = '\0';

pub struct Cursor {
    chars: IntoIter<char>,
    pub(crate) pos: usize,
}

impl Cursor {
    pub fn new(source: String) -> Self {
        Self {
            chars: source.chars().collect::<Vec<_>>().into_iter(),
            pos: 0,
        }
    }

    /// Advances to the next character in the iterator.
    pub fn advance(&mut self) -> Option<char> {
        self.chars.next().map(|next_char| {
            self.pos += 1;
            next_char
        })
    }

    pub fn source_len(&self) -> usize {
        self.chars.len()
    }

    pub fn nth(&self, n: usize) -> char {
        self.chars.clone().nth(n).unwrap_or(EOF_CHAR)
    }
}
