use std::sync::Arc;

#[salsa::query_group(InputStorage)]
pub trait Input: salsa::Database {
    #[salsa::input]
    fn source_text(&self, path: String) -> Arc<String>;

    /// The length of the source's text.
    fn source_length(&self, path: String) -> usize;

    /// Calculates the offsets of a file that start a new line.
    ///
    /// This function handles both LF and CRLF end-of-line sequences. It can
    /// also handle the case where the given file contains both of these
    /// sequences mixed. A LF sequence is counted as one character offset while
    /// a CRLF sequence is counted as two character offsets.
    ///
    /// The first element in the returned vector will always be `0`.
    fn source_line_offsets(&self, path: String) -> Vec<usize>;

    /// Calculates a zero-indexed source offset from a given zero-indexed line
    /// and column editor position. Suitable for mapping the source index of a
    /// character to its editor location.
    fn source_offset_at_position(
        &self,
        path: String,
        line: usize,
        column: usize,
    ) -> usize;

    /// Calculates a zero-indexed line and column editor position from a given
    /// zero-indexed source offset. Suitable for mapping the editor location of
    /// a character to its text index.
    fn source_position_at_offset(
        &self,
        path: String,
        offset: usize,
    ) -> (usize, usize);

    // /// Returns a parsed syntax tree of the given file.
    // fn ast(&self, path: String) -> Arc<Ast>;
}

fn source_length(db: &impl Input, path: String) -> usize {
    let contents = db.source_text(path);
    contents.len()
}

fn source_line_offsets(db: &impl Input, path: String) -> Vec<usize> {
    let mut accumulator = 0;
    let contents = &db.source_text(path)[..];

    contents
        .lines()
        .map(|line| {
            let line_start = accumulator;
            accumulator += line.len();

            if contents[accumulator..].starts_with("\r\n") {
                accumulator += 2;
            } else if contents[accumulator..].starts_with("\n") {
                accumulator += 1;
            }

            line_start
        })
        .chain(std::iter::once(contents.len()))
        .collect()
}

fn source_offset_at_position(
    db: &impl Input,
    path: String,
    line: usize,
    column: usize,
) -> usize {
    let line_offsets = db.source_line_offsets(path);
    line_offsets[line] + column
}

fn source_position_at_offset(
    db: &impl Input,
    path: String,
    offset: usize,
) -> (usize, usize) {
    let offsets = &db.source_line_offsets(path)[..];
    match offsets.binary_search(&offset) {
        // The offset was a line-offset position
        Ok(line) => (line, 0),
        // Otherwise, we need to calculate the actual offset from the last
        // line-offset position
        Err(expected_index) => {
            let last_line = expected_index.checked_sub(1).unwrap_or(0);
            let column = offset.checked_sub(offsets[last_line]).unwrap_or(0);
            (last_line, column)
        }
    }
}
