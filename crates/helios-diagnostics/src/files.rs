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

/// A trait to inspect the texts, lines and columns of files.
///
/// This trait's primary purpose is to get the line and column positions of a
/// particular file. There are two types of methods: `*_index` methods, which
/// provide familiar zero-indexed values that are to be used internally; and
/// `*_number` methods, which provide values indexed from `1` suitable to be
/// shown to users on the front-end.
///
/// The structs [`OneFile`] and [`ManyFiles`] implement this trait. One thing to
/// note about [`OneFile`] is that since it only handles a single file, it makes
/// no sense to also provide the file id to the methods of this trait. This is
/// why the associated type `FileId` is set to the empty tuple (`()`).
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
        // Because `line_indexes` is already sorted, we'll do a binary search
        // to get the expected position of the line index. It's most likely the
        // given `byte_index` will NOT be in the vector (meaning `byte_index`
        // is somewhere inside a line), so we'll decrement the expected position
        // by one to get the line's actual index.
        Ok(self
            .line_indexes
            .binary_search(&byte_index)
            .unwrap_or_else(|expected| expected.checked_sub(1).unwrap_or(0)))
    }

    fn line_range(
        &'a self,
        _: Self::FileId,
        line_index: usize,
    ) -> Result<Range<usize>> {
        let line_start = self.line_start(line_index)?;
        let next_line_start = min(line_index + 1, self.line_count(())? - 1);
        let line_end = if next_line_start == line_index {
            // We're already at the last line, so we'll set `line_end` to the
            // remaining length of the source text.
            self.source.as_ref().len()
        } else {
            self.line_start(next_line_start)?
        };

        Ok(line_start..line_end)
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

    const FILE_A_NAME: &str = "a.hl";
    const FILE_B_NAME: &str = "b.hl";

    const FILE_A_SOURCE: &str = "let a = 0\nlet b = 1\r\nlet x = 2\r\n\nfoo\n";
    const FILE_B_SOURCE: &str = "let x = 0\r\nlet y = 1\n\n\r\nlet z =\nbar";

    const FILE_A_LINE_INDEXES: &[usize] = &[
        0,  // "let a = 0\n"
        10, // "let b = 1\r\n"
        21, // "let c = 2\r\n"
        32, // "\n"
        33, // "foo"
        37, // "\n"
    ];

    const FILE_B_LINE_INDEXES: &[usize] = &[
        0,  // "let x = 0\r\n"
        11, // "let y = 1\n"
        21, // "\n"
        22, // "\r\n"
        24, // "let z =\n"
        32, // "bar"
    ];

    fn check_line_indexes_and_ranges<
        Name: Clone + Display,
        Source: AsRef<str>,
    >(
        file: &OneFile<Name, Source>,
        indexes: &[usize],
    ) {
        for (line_idx, byte_idx) in indexes.iter().enumerate() {
            let line_start = *byte_idx;
            assert_eq!(file.line_start(line_idx), Ok(line_start));
            assert_eq!(file.line_index((), line_start), Ok(line_idx));
            assert_eq!(file.line_number((), line_start), Ok(line_idx + 1));

            // FIXME: This implementation is very similar to
            // `OneFile::line_range`, which means we can't verify if the
            // implementation itself is correct.
            let line_end = {
                let next_line_start =
                    indexes.get(line_idx + 1).copied().unwrap_or_else(|| {
                        indexes.last().copied().unwrap_or_default()
                    });

                if next_line_start == line_start {
                    // We're already at the last line, so we'll return the
                    // remaining length of the source text.
                    file.source().as_ref().len()
                } else {
                    next_line_start
                }
            };

            let expected_line_range = line_start..line_end;
            assert_eq!(file.line_range((), line_idx), Ok(expected_line_range));

            // Since we already checked the line index at `byte_idx`, we start
            // checking 5 positions after. It won't matter if the new index
            // turns out to be larger than than `next_byte_idx` since the range
            // will be empty (and thus this for loop won't run).
            for inner_idx in ((line_start + 5)..line_end).step_by(5) {
                assert_eq!(file.line_index((), inner_idx), Ok(line_idx));
            }
        }

        // This should return `Error::OutOfBounds`.
        assert!(file.line_start(indexes.len()).is_err());
    }

    fn check_last_line_is_empty<Name: Clone + Display, Source: AsRef<str>>(
        file: &OneFile<Name, Source>,
        indexes: &[usize],
        expected: bool,
    ) {
        assert_eq!(
            file.line_range((), indexes.len() - 1)
                .map(|range| range.is_empty()),
            Ok(expected)
        );
    }

    #[test]
    fn test_one_file_a() {
        let file_a = OneFile::new(FILE_A_NAME, FILE_A_SOURCE);
        assert_eq!(file_a.name(), &FILE_A_NAME);
        assert_eq!(file_a.source(), &FILE_A_SOURCE);
        assert_eq!(file_a.line_indexes, FILE_A_LINE_INDEXES);
        assert_eq!(file_a.line_count(()), Ok(FILE_A_LINE_INDEXES.len()));
        check_line_indexes_and_ranges(&file_a, FILE_A_LINE_INDEXES);
        check_last_line_is_empty(&file_a, FILE_A_LINE_INDEXES, true);
    }

    #[test]
    fn test_one_file_b() {
        let file_b = OneFile::new(FILE_B_NAME, FILE_B_SOURCE);
        assert_eq!(file_b.name(), &FILE_B_NAME);
        assert_eq!(file_b.source(), &FILE_B_SOURCE);
        assert_eq!(file_b.line_indexes, FILE_B_LINE_INDEXES);
        assert_eq!(file_b.line_count(()), Ok(FILE_B_LINE_INDEXES.len()));
        check_line_indexes_and_ranges(&file_b, FILE_B_LINE_INDEXES);
        check_last_line_is_empty(&file_b, FILE_B_LINE_INDEXES, false);
    }

    #[test]
    fn test_many_files() {
        let mut files = ManyFiles::new();
        let file_a = files.add(FILE_A_NAME, FILE_A_SOURCE);
        let file_b = files.add(FILE_B_NAME, FILE_B_SOURCE);

        assert!(files.get(file_a).is_ok());
        assert_eq!(files.name(file_a), Ok(FILE_A_NAME));
        assert_eq!(files.source(file_a), Ok(FILE_A_SOURCE));
        assert_eq!(files.line_count(file_a), Ok(FILE_A_LINE_INDEXES.len()));

        assert!(files.get(file_b).is_ok());
        assert_eq!(files.name(file_b), Ok(FILE_B_NAME));
        assert_eq!(files.source(file_b), Ok(FILE_B_SOURCE));
        assert_eq!(files.line_count(file_b), Ok(FILE_B_LINE_INDEXES.len()));
    }
}
