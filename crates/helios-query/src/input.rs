use flume::unbounded;
use helios_parser::{Message, Parse};

pub type FileId = usize;

#[salsa::query_group(InputStorage)]
pub trait Input: salsa::Database {
    #[salsa::input]
    fn source_text(&'a self, file_id: FileId) -> String;

    /// The length of the source's text.
    fn source_length(&self, file_id: FileId) -> usize;

    /// Calculates the offsets of a file that start a new line.
    ///
    /// This function handles both LF and CRLF end-of-line sequences. It can
    /// also handle the case where the given file contains both of these
    /// sequences mixed. A LF sequence is counted as one character offset while
    /// a CRLF sequence is counted as two character offsets.
    ///
    /// The first element in the returned vector will always be `0`.
    fn source_line_offsets(&self, file_id: FileId) -> Vec<usize>;

    /// Calculates a zero-indexed source offset from a given zero-indexed line
    /// and column editor position. Suitable for mapping the source index of a
    /// character to its editor location.
    fn source_offset_at_position(
        &self,
        file_id: FileId,
        line: usize,
        column: usize,
    ) -> usize;

    /// Calculates a zero-indexed line and column editor position from a given
    /// zero-indexed source offset. Suitable for mapping the editor location of
    /// a character to its text index.
    fn source_position_at_offset(
        &self,
        file_id: FileId,
        offset: usize,
    ) -> (usize, usize);

    /// Returns a parsed syntax tree of the given file.
    fn parse(&self, file_id: FileId) -> (Parse, Vec<Message>);
}

fn source_length(db: &impl Input, file_id: FileId) -> usize {
    let contents = db.source_text(file_id);
    contents.len()
}

fn source_line_offsets(db: &impl Input, file_id: FileId) -> Vec<usize> {
    let mut accumulator = 0;
    let contents = &db.source_text(file_id)[..];

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
    file_id: FileId,
    line: usize,
    column: usize,
) -> usize {
    let line_offsets = db.source_line_offsets(file_id);
    line_offsets[line] + column
}

fn source_position_at_offset(
    db: &impl Input,
    file_id: FileId,
    offset: usize,
) -> (usize, usize) {
    let offsets = &db.source_line_offsets(file_id)[..];
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

fn parse(db: &impl Input, file_id: FileId) -> (Parse, Vec<Message>) {
    let (messages_tx, messages_rx) = unbounded();
    let source = db.source_text(file_id);

    (
        helios_parser::parse(file_id, &source, messages_tx),
        messages_rx.iter().collect::<Vec<_>>(),
    )
}
