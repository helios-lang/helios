use super::*;
use crate::protocol::Notification;
use crate::state::State;
use helios_diagnostics::{Diagnostic as HDiagnostic, Severity};
use helios_query::input::FileId;
use helios_query::*;
use std::ops::Range;
use std::sync::Arc;

// FIXME: See `positions_from_range`.
#[allow(unused)]
#[allow(unreachable_code)]
fn publish_diagnostics(
    state: &mut State,
    file_id: FileId,
    uri: Url,
    version: Option<i32>,
) {
    // Just return for now
    return;

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

// FIXME: These positions assume a UTF-8 input, which the LSP does not provide.
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

/// The initialized notification is sent from the client to the server after
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
    let file_id = FileId(0);
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
    let file_id = FileId(0);
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
            let edited_range = range_over(&utf16_bytes, range);
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

/// Calculates the byte offset range over a UTF-16-encoded string (in bytes)
/// from the given [`lsp_types::Range`].
fn range_over(bytes: &Vec<u16>, range: lsp_types::Range) -> Range<usize> {
    let (s_l, s_c) = (range.start.line, range.start.character);
    let (e_l, e_c) = (range.end.line, range.end.character);

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

/// The document save notification is sent from the client to the server when
/// the document was saved in the client.
pub fn did_save_text_document(_: &mut State, _: DidSaveTextDocumentParams) {
    // Nothing to do...
}

pub fn did_change_configuration(
    _: &mut State,
    params: DidChangeConfigurationParams,
) {
    log::trace!("{:?}", params);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_content_changes() {
        macro_rules! changes {
            (
                $(
                    $changed_text:tt @ $s_l:tt:$s_c:tt => $e_l:tt:$e_c:tt
                ),*
                $(,)?
            ) => {{
                type ChangeEvent = TextDocumentContentChangeEvent;
                let mut changes: Vec<ChangeEvent> = Vec::new();

                $(
                    let change_event = serde_json::json!({
                        "text": $changed_text,
                        "range": {
                            "start": { "line": $s_l, "character": $s_c },
                            "end": { "line": $e_l, "character": $e_c },
                        }
                    });

                    changes.push(serde_json::from_value(change_event).unwrap());
                )*

                changes
            }};
        }

        macro_rules! check {
            (
                $old_text:tt,
                $changed_text:tt @ $s_l:tt:$s_c:tt => $e_l:tt:$e_c:tt,
                $expected_text:tt
            ) => {{
                let new_text = apply_content_changes(
                    $old_text,
                    changes![$changed_text @ $s_l:$s_c => $e_l:$e_c],
                );
                assert_eq!(new_text, $expected_text);
                $expected_text
            }};
            ($old_text:tt, $changes:expr, $expected_text:tt) => {{
                let new_text = apply_content_changes($old_text, $changes);
                assert_eq!(new_text, $expected_text);
                $expected_text
            }};
        }

        // Check at every change event
        let text = "let a = 1";
        let text = check!(text, "🍕" @ 0:4 => 0:5, "let 🍕 = 1");
        let text = check!(text, "🚀" @ 0:6 => 0:6, "let 🍕🚀 = 1");
        let text = check!(text, "\n" @ 0:6 => 0:6, "let 🍕\n🚀 = 1");
        let text = check!(text, "." @ 1:0 => 1:0, "let 🍕\n.🚀 = 1");
        assert_eq!(text, "let 🍕\n.🚀 = 1");

        // Check after all change events
        check!(
            "let a = 1",
            changes![
                "🍕" @ 0:4 => 0:5,
                "🚀" @ 0:6 => 0:6,
                "\n" @ 0:6 => 0:6,
                "." @ 1:0 => 1:0,
            ],
            "let 🍕\n.🚀 = 1"
        );
    }
}
