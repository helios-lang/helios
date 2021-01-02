use super::*;
use crate::state::State;
use helios_query::input::FileId;
use helios_query::*;
use std::ops::Range;
use std::sync::Arc;

pub fn initialized(_: &mut State, _: InitializedParams) {
    log::trace!("Successfully initialized");
}

pub fn did_open_text_document(
    state: &mut State,
    params: DidOpenTextDocumentParams,
) {
    state.db.set_source(0, Arc::new(params.text_document.text));
}

pub fn did_change_text_document(
    state: &mut State,
    params: DidChangeTextDocumentParams,
) {
    let file_id = 0;
    let old_source: Arc<String> = state.db.source(file_id);
    let mut source = (*old_source).clone();
    apply_content_changes(state, file_id, &mut source, params.content_changes);
    state.db.set_source(file_id, Arc::new(source));
}

fn apply_content_changes(
    state: &mut State,
    file_id: FileId,
    old_text: &mut String,
    content_changes: Vec<TextDocumentContentChangeEvent>,
) {
    for change in content_changes {
        if let Some(range) = change.range {
            let edit_range =
                range_from_positions(state, file_id, range.start, range.end);
            old_text.replace_range(edit_range, &change.text);
        } else {
            *old_text = change.text
        }
    }
}

fn range_from_positions(
    state: &mut State,
    file_id: FileId,
    start: Position,
    end: Position,
) -> Range<usize> {
    let (start_line, start_col) = (start.line, start.character);
    let (end_line, end_col) = (end.line, end.character);

    let start_offset = state.db.source_offset_at_position(
        file_id,
        (start_line as usize, start_col as usize),
    );

    let end_offset = state.db.source_offset_at_position(
        file_id,
        (end_line as usize, end_col as usize),
    );

    start_offset..end_offset
}
