use super::*;
use crate::protocol::Notification;
use crate::state::State;
use helios_diagnostics::{Diagnostic as HDiagnostic, Severity};
use helios_query::input::FileId;
use helios_query::*;
use std::ops::Range;
use std::sync::Arc;

/// The `initialized` notification is sent from the client to the server after
/// the client received the result of the `initialize` request but before the
/// client is sending any other request or notification to the server.
pub fn initialized(_: &mut State, _: InitializedParams) {
    log::trace!("Successfully initialized");
}

/// The document open notification is sent from the client to the server to
/// signal newly opened text documents. Open in this sense means it is managed
/// by the client.
pub fn did_open_text_document(
    state: &mut State,
    params: DidOpenTextDocumentParams,
) {
    // We need to generate a new file id when we open a document
    let file_id = 0;
    let source = Arc::new(params.text_document.text);
    state.db.set_source(file_id, source);

    publish_diagnostics(
        state,
        file_id,
        params.text_document.uri,
        Some(params.text_document.version),
    );
}

/// The document change notification is sent from the client to the server to
/// signal changes to a text document.
pub fn did_change_text_document(
    state: &mut State,
    params: DidChangeTextDocumentParams,
) {
    let file_id = 0;
    let old_source: Arc<String> = state.db.source(file_id);
    let new_source = apply_content_changes(&old_source, params.content_changes);

    log::trace!("New source: {:?}", new_source);
    state.db.set_source(file_id, Arc::new(new_source));

    publish_diagnostics(
        state,
        file_id,
        params.text_document.uri,
        Some(params.text_document.version),
    );
}

/// Applies `TextDocumentContentChangeEvent` changes over a provided string,
/// returning a new string with the changes applied.
fn apply_content_changes(
    old_text: &str,
    content_changes: Vec<TextDocumentContentChangeEvent>,
) -> String {
    // LSP encodes character offsets based on a UTF-16 string representation
    let mut utf16_bytes = old_text.encode_utf16().collect::<Vec<_>>();

    for change in content_changes {
        if let Some(range) = change.range {
            let edited_range = range_at(&utf16_bytes, range.start, range.end);
            utf16_bytes.splice(edited_range, change.text.encode_utf16());
        } else {
            // If no range is given, the user has replaced all the characters
            // in the file with the given text
            utf16_bytes.clear();
            utf16_bytes.extend(change.text.encode_utf16());
        }
    }

    // For now we'll ignore invalid characters
    String::from_utf16_lossy(&utf16_bytes)
}

/// Calculates the byte offset range over the UTF-16-encoded bytes with the
/// given start and end [`Position`]s.
fn range_at(bytes: &Vec<u16>, start: Position, end: Position) -> Range<usize> {
    let (s_l, s_c) = (start.line, start.character);
    let (e_l, e_c) = (end.line, end.character);

    fn line_indices(bytes: &Vec<u16>) -> Vec<usize> {
        std::iter::once(0)
            .chain(
                bytes
                    .iter()
                    .copied()
                    .enumerate()
                    .filter(|(_, byte)| *byte == b'\n' as u16)
                    .map(|(i, _)| i + 1),
            )
            .collect()
    }

    let indices = line_indices(bytes);
    let start = indices[s_l as usize] + s_c as usize;
    let end = indices[e_l as usize] + e_c as usize;

    start..end
}

fn publish_diagnostics(
    state: &mut State,
    file_id: FileId,
    uri: Url,
    version: Option<i32>,
) {
    let mut emitted_ranges = Vec::new();
    let mut diagnostics = Vec::new();
    let h_diagnostics: Arc<Vec<HDiagnostic<_>>> = state.db.diagnostics(file_id);

    for h_diagnostic in h_diagnostics.iter() {
        let (start, end) = positions_from_range(
            state,
            file_id,
            h_diagnostic.location.range.clone(),
        );

        let range = lsp_types::Range::new(start, end);

        if emitted_ranges.contains(&range) {
            continue;
        } else {
            emitted_ranges.push(range);
        }

        let source = Some("helios-ls".to_string());
        let message = format!("{}", h_diagnostic.title);
        let related_message = format!("{}", h_diagnostic.message);

        let severity = Some(match h_diagnostic.severity {
            Severity::Bug | Severity::Error => DiagnosticSeverity::Error,
            Severity::Warning => DiagnosticSeverity::Warning,
            Severity::Note => DiagnosticSeverity::Information,
        });

        let related_information = Some(vec![DiagnosticRelatedInformation {
            location: Location::new(uri.clone(), range),
            message: related_message.trim_end().to_string(),
        }]);

        diagnostics.push(Diagnostic {
            range,
            source,
            message,
            severity,
            related_information,
            ..Diagnostic::default()
        })
    }

    let params = PublishDiagnosticsParams {
        uri,
        version,
        diagnostics,
    };

    state.send(Notification::new("textDocument/publishDiagnostics", params));
}

fn positions_from_range(
    state: &mut State,
    file_id: FileId,
    range: Range<usize>,
) -> (Position, Position) {
    let (s_ln, s_cl) = state.db.source_position_at_offset(file_id, range.start);
    let (e_ln, e_cl) = state.db.source_position_at_offset(file_id, range.end);

    let start = Position::new(s_ln as u32, s_cl as u32);
    let end = Position::new(e_ln as u32, e_cl as u32);

    (start, end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_content_changes() {
        let old_text = "let a = 1";

        // "let 🍕 = 1"
        let event_1 = serde_json::json!({
            "text": "🍕",
            "range": {
                "start": { "line": 0, "character": 4 },
                "end": { "line": 0, "character": 5 },
            }
        });

        // "let 🍕🚀 = 1"
        let event_2 = serde_json::json!({
            "text": "🚀",
            "range": {
                "start": { "line": 0, "character": 6 },
                "end": { "line": 0, "character": 6 },
            }
        });

        // "let 🍕\n🚀 = 1"
        let event_3 = serde_json::json!({
            "text": "\n",
            "range": {
                "start": { "line": 0, "character": 6 },
                "end": { "line": 0, "character": 6 },
            }
        });

        // "let 🍕\n.🚀 = 1"
        let event_4 = serde_json::json!({
            "text": ".",
            "range": {
                "start": { "line": 1, "character": 0 },
                "end": { "line": 1, "character": 0 },
            }
        });

        let content_changes = vec![event_1, event_2, event_3, event_4]
            .into_iter()
            .map(|event| serde_json::from_value(event).unwrap())
            .collect();

        let new_text = apply_content_changes(old_text, content_changes);
        assert_eq!(new_text, "let 🍕\n.🚀 = 1".to_string());
    }
}
