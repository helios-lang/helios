#![allow(dead_code)]

use std::fmt::Display;
use std::ops::Range;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
    MissingFile,
    OutOfBounds { given: usize, max: usize },
}

fn line_indexes<'a>(source: &'a str) -> impl 'a + Iterator<Item = usize> {
    std::iter::once(0).chain(source.match_indices('\n').map(|(i, _)| i + 1))
}

pub struct Location {
    line: usize,
    column: usize,
}

pub trait Files<'a> {
    type FileId: Copy + PartialEq;
    type Name: 'a + Display;
    type Source: 'a + AsRef<str>;

    fn name(&'a self, id: Self::FileId) -> Result<Self::Name>;

    fn source(&'a self, id: Self::FileId) -> Result<Self::Source>;

    fn line_index(
        &'a self,
        id: Self::FileId,
        byte_offset: usize,
    ) -> Result<usize>;

    fn line_range(
        &'a self,
        id: Self::FileId,
        line_index: usize,
    ) -> Result<Range<usize>>;
}

#[derive(Clone, Debug)]
pub struct SimpleFile<Name, Source> {
    name: Name,
    source: Source,
    line_indexes: Vec<usize>,
}

impl<Name, Source> SimpleFile<Name, Source>
where
    Name: Display,
    Source: AsRef<str>,
{
    pub fn new(name: Name, source: Source) -> Self {
        let line_indexes = line_indexes(source.as_ref()).collect();
        Self {
            name,
            source,
            line_indexes,
        }
    }

    fn line_start(&self, line_index: usize) -> Result<usize> {
        if line_index == self.line_indexes.len() {
            return Ok(self.source.as_ref().len());
        }

        self.line_indexes
            .get(line_index)
            .cloned()
            .ok_or(Error::OutOfBounds {
                given: line_index,
                max: self.line_indexes.len() - 1,
            })
    }
}

impl<'a, Name, Source> Files<'a> for SimpleFile<Name, Source>
where
    Name: 'a + std::fmt::Display + Clone,
    Source: 'a + AsRef<str>,
{
    type FileId = ();
    type Name = Name;
    type Source = &'a str;

    fn name(&'a self, _: Self::FileId) -> Result<Self::Name> {
        Ok(self.name.clone())
    }

    fn source(&'a self, _: Self::FileId) -> Result<Self::Source> {
        Ok(self.source.as_ref())
    }

    fn line_index(
        &'a self,
        _: Self::FileId,
        byte_offset: usize,
    ) -> Result<usize> {
        Ok(self
            .line_indexes
            .binary_search(&byte_offset)
            .unwrap_or_else(|expected| expected.checked_sub(1).unwrap_or(0)))
    }

    fn line_range(
        &'a self,
        _: Self::FileId,
        line_index: usize,
    ) -> Result<Range<usize>> {
        let line_start = self.line_start(line_index)?;
        let next_line_start = self.line_start(line_index + 1)?;

        Ok(line_start..next_line_start)
    }
}

pub struct SimpleFiles<Name, Source> {
    files: Vec<SimpleFile<Name, Source>>,
}

impl<'a, Name, Source> SimpleFiles<Name, Source>
where
    Name: 'a + std::fmt::Display + Clone,
    Source: 'a + AsRef<str>,
{
    pub fn new() -> Self {
        Self { files: Vec::new() }
    }

    pub fn add(&mut self, name: Name, source: Source) -> usize {
        let file_id = self.files.len();
        self.files.push(SimpleFile::new(name, source));
        file_id
    }

    pub fn get(&self, file_id: usize) -> Result<&SimpleFile<Name, Source>> {
        self.files.get(file_id).ok_or(Error::MissingFile)
    }
}

impl<'a, Name, Source> Files<'a> for SimpleFiles<Name, Source>
where
    Name: 'a + std::fmt::Display + Clone,
    Source: 'a + AsRef<str>,
{
    type FileId = usize;
    type Name = Name;
    type Source = &'a str;

    fn name(&'a self, id: Self::FileId) -> Result<Self::Name> {
        Ok(self.get(id)?.name.clone())
    }

    fn source(&'a self, id: Self::FileId) -> Result<Self::Source> {
        Ok(self.get(id)?.source.as_ref())
    }

    fn line_index(
        &'a self,
        id: Self::FileId,
        byte_offset: usize,
    ) -> Result<usize> {
        self.get(id)?.line_index((), byte_offset)
    }

    fn line_range(
        &'a self,
        id: Self::FileId,
        line_index: usize,
    ) -> Result<Range<usize>> {
        self.get(id)?.line_range((), line_index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FILE_NAME: &str = "Foo.he";
    const SOURCE: &str = "let a = 0\nlet b = 1\r\nlet c = 3\r\n\nfoo";

    #[test]
    fn test_simple_file() {
        let file = SimpleFile::new(FILE_NAME, SOURCE);

        assert_eq!(
            file.line_indexes,
            [
                0,  // "let a = 0\n"
                10, // "let b = 1\r\n"
                21, // "let c = 2\r\n"
                32, // "\n"
                33  // "foo"
            ]
        );

        assert_eq!(file.line_index((), 00), Ok(0));
        assert_eq!(file.line_index((), 01), Ok(0));
        assert_eq!(file.line_index((), 05), Ok(0));
        assert_eq!(file.line_index((), 09), Ok(0));
        assert_eq!(file.line_index((), 10), Ok(1));
        assert_eq!(file.line_index((), 11), Ok(1));
        assert_eq!(file.line_index((), 14), Ok(1));
        assert_eq!(file.line_index((), 20), Ok(1));
        assert_eq!(file.line_index((), 21), Ok(2));
        assert_eq!(file.line_index((), 22), Ok(2));
        assert_eq!(file.line_index((), 26), Ok(2));
        assert_eq!(file.line_index((), 31), Ok(2));
        assert_eq!(file.line_index((), 32), Ok(3));
        assert_eq!(file.line_index((), 33), Ok(4));
        assert_eq!(file.line_index((), 34), Ok(4));
        assert_eq!(file.line_index((), 36), Ok(4));

        assert_eq!(file.line_range((), 0), Ok(00..10));
        assert_eq!(file.line_range((), 1), Ok(10..21));
        assert_eq!(file.line_range((), 2), Ok(21..32));
        assert_eq!(file.line_range((), 3), Ok(32..33));
        assert_eq!(file.line_range((), 4), Ok(33..36));
    }
}
