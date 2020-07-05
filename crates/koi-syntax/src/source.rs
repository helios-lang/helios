use std::fs::File;
use std::fmt::{self, Display};
use std::io::{BufRead, BufReader, Read, Result, Stdin};
use std::path::PathBuf;
use std::vec::IntoIter;

pub const EOF_CHAR: char = '\0';

#[derive(Copy, Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Position {
    pub line: usize,
    pub column: usize,
    offset: usize,
}

impl Position {
    pub fn new(line: usize, column: usize, offset: usize) -> Self {
        Self { line, column, offset }
    }

    pub fn advance(&mut self) {
        self.column += 1;
        self.offset += 1;
    }

    pub fn advance_line(&mut self) {
        self.line += 1;
        self.column = 0;
        self.offset += 1;
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

impl Span {
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }
}

#[derive(Copy, Clone)]
pub enum SourceType {
    File,
    Stdin,
    Stream,
}

pub struct Source<'a> {
    pub source_type: SourceType,
    pub file_name: Option<PathBuf>,
    input: Box<dyn BufRead + 'a>,
}

impl<'a> Source<'a> {
    pub fn stdin(stdin: &'a Stdin) -> Result<Self> {
        Ok(Self {
            source_type: SourceType::Stdin,
            file_name: None,
            input: Box::new(stdin.lock())
        })
    }

    pub fn file<P: Into<PathBuf> + Clone>(path: P) -> Result<Self> {
        File::open(path.clone().into()).map(|file|
            Self {
                source_type: SourceType::File,
                file_name: Some(path.into()),
                input: Box::new(BufReader::new(file)),
            }
        )
    }

    pub fn stream(input: &'a mut dyn BufRead) -> Result<Self> {
        Ok(Self {
            source_type: SourceType::Stream,
            file_name: None,
            input: Box::new(input)
        })
    }
}

impl<'a> Read for Source<'a> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.input.read(buf)
    }
}

impl<'a> BufRead for Source<'a> {
    fn fill_buf(&mut self) -> Result<&[u8]> {
        self.input.fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        self.input.consume(amt);
    }
}

pub struct Cursor {
    chars: IntoIter<char>,
    pub pos: Position,
}

impl Cursor {
    pub fn with(mut source: Source) -> Self {
        let mut buffer = String::new();
        let chars = match source.read_to_string(&mut buffer) {
            Ok(bytes) if bytes == 0 => None,
            Ok(_) => Some(buffer.chars().collect::<Vec<_>>().into_iter()),
            Err(error) => {
                eprintln!("Failed to read line: {}", error);
                None
            }
        };

        Self {
            chars: chars.unwrap_or(Vec::new().into_iter()),
            pos: Position::default(),
        }
    }

    /// Advances to the next character in the `Cursor` iterator.
    ///
    /// * If we still have characters left in the line, we'll return the next
    ///   character in queue.
    ///
    /// * If we received `None` (which means we reached the end of the line),
    ///   then we'll ask our `source` to give us the next line.
    ///
    /// * If we are given a new line (and thus the file still has contents in to
    ///   be processed), then we'll return the next character in our new queue.
    ///
    /// * Otherwise, we've reached the end of the file and so we'll return `None`.
    pub fn advance(&mut self) -> Option<char> {
        self.chars.next().map(|next_char| {
            if next_char == '\n' {
                self.pos.advance_line();
            } else {
                self.pos.advance();
            }

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
