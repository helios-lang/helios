use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::path::Path;
use std::vec::IntoIter;

pub const EOF_CHAR: char = '\0';

#[allow(dead_code)]
#[derive(Default)]
pub struct Position {
    line: u32,
    column: u32,
    offset: usize,
}

#[derive(Copy, Clone)]
pub enum SourceType {
    Stdin,
    File,
}

pub struct Source<'a> {
    pub source_type: SourceType,
    input: Box<dyn BufRead + 'a>,
}

impl<'a> Source<'a> {
    #[allow(dead_code)]
    pub fn stdin(stdin: &'a io::Stdin) -> io::Result<Self> {
        Ok(Self { source_type: SourceType::Stdin, input: Box::new(stdin.lock()) })
    }

    #[allow(dead_code)]
    pub fn file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        File::open(path).map(|file| Self {
            source_type: SourceType::File,
            input: Box::new(BufReader::new(file)),
        })
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

#[allow(dead_code)]
pub struct Cursor<'a> {
    source: Source<'a>,
    pos: Position,
    chars: IntoIter<char>,
}

impl<'a> Cursor<'a> {
    pub fn with(source: Source<'a>) -> Self {
        let chars = Vec::new().into_iter();
        let mut cursor = Self { source, pos: Position::default(), chars };

        cursor.advance_line();
        cursor
    }

    pub fn advance(&mut self) -> Option<char> {
        match self.chars.next() {
            Some(c) => Some(c),
            None => {
                self.advance_line();
                self.chars.next()
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
    fn advance_line(&mut self) {
        let mut buffer = String::new();
        self.source.read_line(&mut buffer).expect("Failed to read line");
        self.chars = buffer.chars().collect::<Vec<_>>().into_iter();
    }
}
