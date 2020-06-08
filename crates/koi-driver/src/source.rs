use std::fs::File;
use std::fmt::{self, Display};
use std::io::{self, BufRead, BufReader, Read};
use std::path::Path;
use std::vec::IntoIter;

pub const EOF_CHAR: char = '\0';

#[derive(Copy, Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Position {
    pub line: usize,
    pub character: usize,
}

impl Position {
    pub fn new(line: usize, character: usize) -> Self {
        Self { line, character }
    }

    pub fn advance(&mut self) {
        self.character += 1;
    }

    pub fn advance_line(&mut self) {
        self.line += 1;
        self.character = 0;
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}:{}]", self.line, self.character)
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
    input: Box<dyn BufRead + 'a>,
}

// #[allow(dead_code)]
impl<'a> Source<'a> {
    pub fn stdin(stdin: &'a io::Stdin) -> io::Result<Self> {
        Ok(Self { source_type: SourceType::Stdin, input: Box::new(stdin.lock()) })
    }

    pub fn file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        File::open(path).map(|file| Self {
            source_type: SourceType::File,
            input: Box::new(BufReader::new(file)),
        })
    }

    pub fn stream(input: &'a mut dyn BufRead) -> io::Result<Self> {
        Ok(Self { source_type: SourceType::Stream, input: Box::new(input) })
    }
}

impl<'a> Read for Source<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.input.read(buf)
    }
}

impl<'a> BufRead for Source<'a> {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        self.input.fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        self.input.consume(amt);
    }
}

pub struct Cursor<'a> {
    source: Source<'a>,
    chars: IntoIter<char>,
    pub pos: Position,
}

impl<'a> Cursor<'a> {
    pub fn with(source: Source<'a>) -> Self {
        let chars = Vec::new().into_iter();
        let mut cursor = Self { source, chars, pos: Position::default() };

        match cursor.advance_line() {
            Some(chars) => cursor.chars = chars,
            None => cursor.chars = Vec::new().into_iter()
        }

        cursor
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
    /// * Otherwise, we have reached the end of the file, and thus we return
    ///   `None`.
    pub fn advance(&mut self) -> Option<char> {
        match self.chars.next() {
            Some(c) => {
                self.pos.advance();
                Some(c)
            },
            None => match self.advance_line() {
                Some(line) => {
                    self.chars = line;
                    self.pos.advance_line();
                    self.advance()
                },
                None => None
            }
        }
    }

    pub fn source_len(&self) -> usize {
        self.chars.len()
    }

    pub fn nth(&self, n: usize) -> char {
        self.chars.clone().nth(n).unwrap_or(EOF_CHAR)
    }
}

impl<'a> Cursor<'a> {
    fn advance_line(&mut self) -> Option<IntoIter<char>> {
        let mut buffer = String::new();
        match self.source.read_line(&mut buffer){
            Ok(bytes) if bytes == 0 => None,
            Ok(_) => Some(buffer.chars().collect::<Vec<_>>().into_iter()),
            Err(error) => {
                eprintln!("Failed to read line: {}", error);
                None
            }
        }
    }
}
