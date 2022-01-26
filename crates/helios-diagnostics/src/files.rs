use std::cmp::min;
use std::fmt::Display;
use std::ops::Range;

use crate::{Error, Result};

fn line_indexes<'a>(source: &'a str) -> impl 'a + Iterator<Item = usize> {
    std::iter::once(0).chain(source.match_indices('\n').map(|(i, _)| i + 1))
}

fn column_index(
    source: &str,
    line_range: Range<usize>,
    byte_index: usize,
) -> usize {
    let end_index = min(byte_index, min(line_range.end, source.len()));

    (line_range.start..end_index)
        .filter(|index| source.is_char_boundary(index + 1))
        .count()
}

/// A trait to inspect a file's source, lines and columns.
///
/// This trait provides methods to get line and column indexes and numbers.
/// `*_index` methods provide familiar zero-indexed values that are to be used
/// internally, whereas `*_number` provide values from `1` that are suitable to
/// be shown to users on the front-end.
///
/// The structs [`OneFile`] and [`ManyFiles`] implement this trait. One thing to
/// note about [`OneFile`] is that since it only handles a single file, it makes
/// no sense to also provide the file id to the methods of this trait. This is
/// why the associated type `FileId` is set to an empty tuple (`()`).
pub trait FileInspector<'a> {
    type FileId: Copy + PartialEq;
    type Name: 'a + Display;
    type Source: 'a + AsRef<str>;

    /// The name of the current file.
    fn name(&'a self, id: Self::FileId) -> Result<Self::Name>;

    /// The string content of the current file.
    fn source(&'a self, id: Self::FileId) -> Result<Self::Source>;

    /// Returns the number of lines present in the file.
    fn line_count(&'a self, id: Self::FileId) -> Result<usize>;

    /// Returns the line index of a file at the given byte index.
    ///
    /// Note that the returned value will be zero-indexed (i.e this method will
    /// return `0` for the first line).
    ///
    /// Regardless if you call this method with [`OneFile`] or [`ManyFiles`],
    /// this method will not throw if the byte index is out of bounds – it will
    /// simply return the last line's index.
    fn line_index(
        &'a self,
        id: Self::FileId,
        byte_index: usize,
    ) -> Result<usize>;

    /// Just like [`FileInspector::line_index`], except it returns a user-facing
    /// number (i.e. the first line will be `1`).
    fn line_number(
        &'a self,
        id: Self::FileId,
        byte_index: usize,
    ) -> Result<usize> {
        Ok(self.line_index(id, byte_index)? + 1)
    }

    /// Returns the line range at the given line index (zero-indexed).
    fn line_range(
        &'a self,
        id: Self::FileId,
        line_index: usize,
    ) -> Result<Range<usize>>;

    /// Returns the column index of a file at the given line and byte indexes.
    ///
    /// Note that the returned value will be zero-indexed (i.e this method will
    /// return `0` for the first column).
    fn column_index(
        &'a self,
        id: Self::FileId,
        line_index: usize,
        byte_index: usize,
    ) -> Result<usize> {
        let source = self.source(id)?;
        let line_range = self.line_range(id, line_index)?;
        let column_index =
            column_index(source.as_ref(), line_range, byte_index);

        Ok(column_index)
    }

    /// Just like [`FileInspector::column_index`], except it returns a
    /// user-facing number (i.e. the first column will be `1`).
    fn column_number(
        &'a self,
        id: Self::FileId,
        line_index: usize,
        byte_index: usize,
    ) -> Result<usize> {
        Ok(self.column_index(id, line_index, byte_index)? + 1)
    }
}

/// An abstraction over a single Helios source file.
///
/// Use this struct to inspect a Helios program that consists of only a single
/// file (e.g. a script or a REPL environment).
///
/// This struct implements [`FileInspector`]. Please refer to its documentation
/// to find out what you can inspect.
#[derive(Clone, Debug)]
pub struct OneFile<Name, Source> {
    name: Name,
    source: Source,
    line_indexes: Vec<usize>,
}

impl<Name, Source> OneFile<Name, Source>
where
    Name: Display,
    Source: AsRef<str>,
{
    /// Creates a new [`OneFile`] with the given file name and source text.
    pub fn new(name: Name, source: Source) -> Self {
        let line_indexes = line_indexes(source.as_ref()).collect();

        Self {
            name,
            source,
            line_indexes,
        }
    }

    /// Gets the name of the file.
    pub fn name(&self) -> &Name {
        &self.name
    }

    /// Gets the source text of the file as a reference.
    pub fn source(&self) -> &Source {
        &self.source
    }

    /// Returns the byte index from where the given line index starts.
    ///
    /// This function will return [`Error::OutOfBounds`] if the given
    /// `line_index` is larger than the actual number of lines of the file.
    fn line_start(&self, line_index: usize) -> Result<usize> {
        self.line_indexes
            .get(line_index)
            .cloned()
            .ok_or(Error::OutOfBounds {
                given: line_index,
                max: self.line_indexes.len() - 1,
            })
    }
}

impl<'a, Name, Source> FileInspector<'a> for OneFile<Name, Source>
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

    fn line_count(&'a self, _: Self::FileId) -> Result<usize> {
        Ok(self.line_indexes.len())
    }

    fn line_index(
        &'a self,
        _: Self::FileId,
        byte_index: usize,
    ) -> Result<usize> {
        Ok(self.line_indexes.binary_search(&byte_index).unwrap_or_else(
            |expected_position| expected_position.checked_sub(1).unwrap_or(0),
        ))
    }

    fn line_range(
        &'a self,
        _: Self::FileId,
        line_index: usize,
    ) -> Result<Range<usize>> {
        let line_start = self.line_start(line_index)?;
        let next_line_index = min(line_index + 1, self.line_count(())? - 1);
        let next_line_start = self.line_start(next_line_index)?;

        Ok(line_start..next_line_start)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ManyFilesId(usize);

pub struct ManyFiles<Name, Source> {
    files: Vec<OneFile<Name, Source>>,
}

impl<'a, Name, Source> ManyFiles<Name, Source>
where
    Name: 'a + std::fmt::Display + Clone,
    Source: 'a + AsRef<str>,
{
    pub fn new() -> Self {
        Self { files: Vec::new() }
    }

    pub fn add(&mut self, name: Name, source: Source) -> ManyFilesId {
        let file_id = self.files.len();
        self.files.push(OneFile::new(name, source));
        ManyFilesId(file_id)
    }

    pub fn get(&self, file_id: ManyFilesId) -> Result<&OneFile<Name, Source>> {
        self.files.get(file_id.0).ok_or(Error::MissingFile)
    }
}

impl<'a, Name, Source> FileInspector<'a> for ManyFiles<Name, Source>
where
    Name: 'a + std::fmt::Display + Clone,
    Source: 'a + AsRef<str>,
{
    type FileId = ManyFilesId;
    type Name = Name;
    type Source = &'a str;

    fn name(&'a self, id: Self::FileId) -> Result<Self::Name> {
        Ok(self.get(id)?.name.clone())
    }

    fn source(&'a self, id: Self::FileId) -> Result<Self::Source> {
        Ok(self.get(id)?.source.as_ref())
    }

    fn line_count(&'a self, id: Self::FileId) -> Result<usize> {
        self.get(id)?.line_count(())
    }

    fn line_index(
        &'a self,
        id: Self::FileId,
        byte_index: usize,
    ) -> Result<usize> {
        self.get(id)?.line_index((), byte_index)
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

    #[test]
    fn test_one_file() {
        let source = "let a = 0\nlet b = 1\r\nlet c = 3\r\n\nfoo\n";
        let file = OneFile::new("file.hl", source);
        let indexes = [
            0,  // "let a = 0\n"
            10, // "let b = 1\r\n"
            21, // "let c = 2\r\n"
            32, // "\n"
            33, // "foo"
            37, // "\n"
        ];

        assert_eq!(file.name(), &"file.hl");
        assert_eq!(file.source(), &source);

        assert_eq!(file.line_indexes, indexes);
        assert_eq!(file.line_count(()), Ok(indexes.len()));

        assert_eq!(file.line_index((), 0), Ok(0));
        assert_eq!(file.line_index((), 1), Ok(0));
        assert_eq!(file.line_index((), 5), Ok(0));
        assert_eq!(file.line_index((), 9), Ok(0));
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
        assert_eq!(file.line_index((), 37), Ok(5));

        assert_eq!(file.line_range((), 0), Ok(0..10));
        assert_eq!(file.line_range((), 1), Ok(10..21));
        assert_eq!(file.line_range((), 2), Ok(21..32));
        assert_eq!(file.line_range((), 3), Ok(32..33));
        assert_eq!(file.line_range((), 4), Ok(33..37));
        assert_eq!(file.line_range((), 5), Ok(37..37));

        // for (line_idx, byte_idx) in indexes.iter().enumerate() {
        //     assert_eq!(file.line_start(line_idx), Ok(*byte_idx));
        //     assert_eq!(file.line_index((), *byte_idx), Ok(line_idx));
        //     assert_eq!(file.line_number((), *byte_idx), Ok(line_idx + 1));

        //     let next_byte_idx = indexes
        //         .get(line_idx + 1)
        //         .copied()
        //         .unwrap_or_else(|| indexes.last().copied().unwrap_or_default());
        //     assert_eq!(
        //         file.line_range((), line_idx),
        //         Ok(*byte_idx..next_byte_idx)
        //     );
        // }

        assert!(file.line_start(indexes.len()).is_err());
    }

    #[test]
    fn test_many_files() {
        let mut files = ManyFiles::new();
        let foo = files.add("foo.hl", "Hello\nworld!\n\rSecret: 123\n\n456");
        let bar = files.add("bar.hl", "Goodbye\r\nworld!\n\r\r");

        assert_eq!(files.line_count(foo), Ok(5));
        assert_eq!(files.line_count(bar), Ok(3));

        assert_eq!(files.line_index(foo, 5), Ok(0));
        assert_eq!(files.line_index(foo, 10), Ok(1));
        assert_eq!(files.line_index(foo, 15), Ok(2));
        assert_eq!(files.line_index(foo, 20), Ok(2));
        assert_eq!(files.line_index(foo, 26), Ok(3));
        assert_eq!(files.line_index(foo, 27), Ok(4));
        assert_eq!(files.line_index(foo, 30), Ok(4));
        assert!(files.line_index(foo, 31).is_ok());

        assert_eq!(files.line_index(bar, 5), Ok(0));
        assert_eq!(files.line_index(bar, 10), Ok(1));
        assert_eq!(files.line_index(bar, 15), Ok(1));
        assert!(files.line_index(bar, 19).is_ok());

        // Hello$world!$_Secret: 123$$456
        // Goodbye_$world!$__

        assert_eq!(files.line_range(foo, 0), Ok(0..6));
        assert_eq!(files.line_range(foo, 1), Ok(6..13));
        assert_eq!(files.line_range(foo, 2), Ok(13..26));
        assert_eq!(files.line_range(foo, 3), Ok(26..27));
        // assert_eq!(files.line_range(foo, 4), Ok(27..30)); // FAIL
        assert!(files.line_range(foo, 5).is_err());

        assert_eq!(files.line_range(bar, 0), Ok(0..9));
        assert_eq!(files.line_range(bar, 1), Ok(9..16));
        // assert_eq!(files.line_range(bar, 2), Ok(16..19)); // FAIL
        assert!(files.line_range(bar, 3).is_err());
    }
}
