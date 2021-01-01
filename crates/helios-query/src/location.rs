use crate::Input;
use std::{ops::Range, sync::Arc};

pub type FileId = usize;

#[salsa::query_group(InputLocationDatabase)]
pub trait InputLocation: Input {
    /// Calculates the indexes of each line in a file.
    ///
    /// The first element in the returned vector will always be `0`.
    fn source_line_indexes(&self, file_id: FileId) -> Arc<Vec<usize>>;

    fn source_line_start(&self, file_id: FileId, line_index: usize) -> usize;

    fn source_line_range(
        &self,
        file_id: FileId,
        byte_offset: usize,
    ) -> Range<usize>;

    fn source_line_index(&self, file_id: FileId, byte_offset: usize) -> usize;

    fn source_column_index(
        &self,
        file_id: FileId,
        line_index: usize,
        byte_offset: usize,
    ) -> usize;

    fn source_position_at_offset(
        &self,
        file_id: FileId,
        byte_offset: usize,
    ) -> (usize, usize);

    fn source_offset_at_position(
        &self,
        file_id: FileId,
        position: (usize, usize),
    ) -> usize;
}

fn source_line_indexes(
    db: &dyn InputLocation,
    file_id: FileId,
) -> Arc<Vec<usize>> {
    let source = db.source(file_id);
    let indexes = std::iter::once(0)
        .chain(source.match_indices('\n').map(|(i, _)| i + 1))
        .collect();

    Arc::new(indexes)
}

fn source_line_start(
    db: &dyn InputLocation,
    file_id: FileId,
    line_index: usize,
) -> usize {
    let line_indexes = db.source_line_indexes(file_id);

    if line_index == line_indexes.len() {
        db.source_len(file_id)
    } else {
        line_indexes
            .get(line_index)
            .cloned()
            .expect("Out of bounds")
    }
}

fn source_line_range(
    db: &dyn InputLocation,
    file_id: FileId,
    line_index: usize,
) -> Range<usize> {
    let line_start = db.source_line_start(file_id, line_index);
    let next_line_start = db.source_line_start(file_id, line_index + 1);

    line_start..next_line_start
}

fn source_line_index(
    db: &dyn InputLocation,
    file_id: FileId,
    byte_offset: usize,
) -> usize {
    db.source_line_indexes(file_id)
        .binary_search(&byte_offset)
        .unwrap_or_else(|expected| expected.checked_sub(1).unwrap_or(0))
}

fn source_column_index(
    db: &dyn InputLocation,
    file_id: FileId,
    line_index: usize,
    byte_offset: usize,
) -> usize {
    fn column_index(
        source: &str,
        line_range: Range<usize>,
        byte_offset: usize,
    ) -> usize {
        use std::cmp::min;
        let end_index = min(byte_offset, min(line_range.end, source.len()));

        (line_range.start..end_index)
            .filter(|index| source.is_char_boundary(index + 1))
            .count()
    }

    let source = db.source(file_id);
    let line_range = db.source_line_range(file_id, line_index);
    let column_index = column_index(source.as_ref(), line_range, byte_offset);

    column_index
}

fn source_position_at_offset(
    db: &dyn InputLocation,
    file_id: FileId,
    byte_offset: usize,
) -> (usize, usize) {
    let line_index = db.source_line_index(file_id, byte_offset);
    let column_index = db.source_column_index(file_id, line_index, byte_offset);

    (line_index, column_index)
}

fn source_offset_at_position(
    db: &dyn InputLocation,
    file_id: FileId,
    position: (usize, usize)
) -> usize {
    let line_indexes = db.source_line_indexes(file_id);
    line_indexes[position.0] + position.1
}
