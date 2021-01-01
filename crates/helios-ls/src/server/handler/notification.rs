use super::*;
use crate::state::State;
use helios_query::*;
use std::sync::Arc;

pub fn initialized(_: &mut State, _: InitializedParams) {
    log::trace!("Successfully initialized");
}

pub fn did_open_text_document(
    state: &mut State,
    params: DidOpenTextDocumentParams,
) {
    log::trace!("Opened document: {}", params.text_document.uri.to_string());
    state.db.set_source(0, Arc::new(params.text_document.text));
}

pub fn did_change_text_document(
    state: &mut State,
    params: DidChangeTextDocumentParams,
) {
    log::trace!("Changed document: {}", params.text_document.uri.to_string());
    let file_id = 0;
    let source: Arc<String> = state.db.source(file_id);

    for change in params.content_changes {
        if let Some(range) = change.range {
            let start = range.start;
            let end = range.end;

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

            let edit_range = start_offset..end_offset;
            let new_capacity = std::cmp::max(source.len(), end_offset);

            let mut new_source = String::with_capacity(new_capacity);
            new_source.push_str(&source);
            new_source.replace_range(edit_range, &change.text);

            log::trace!("New source: {:?}", new_source);
            state.db.set_source(0, Arc::new(new_source));
        } else {
            log::trace!("New source: {:?}", change.text);
            state.db.set_source(0, Arc::new(change.text));
        }
    }
}
