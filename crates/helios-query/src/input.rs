use helios_parser::Parse;

use crate::interner::{BindingData, BindingId, Interner};
use std::sync::Arc;

pub type FileId = usize;

#[salsa::query_group(InputDatabase)]
pub trait Input: Interner {
    #[salsa::input]
    fn source(&self, file_id: FileId) -> Arc<String>;

    /// The length of the source's text.
    fn source_len(&self, file_id: FileId) -> usize;

    /// Calculates the indexes of each line in a file.
    ///
    /// The first element in the returned vector will always be `0`.
    fn source_line_indexes(&self, file_id: FileId) -> Arc<Vec<usize>>;

    /// Constructs a parsed syntax tree of the given file.
    fn parse(&self, file_id: FileId) -> Parse;

    fn all_bindings(&self, file_id: FileId) -> Arc<Vec<BindingId>>;
}

fn source_len(db: &dyn Input, file_id: FileId) -> usize {
    let source = db.source(file_id);
    source.len()
}

fn source_line_indexes(db: &dyn Input, file_id: FileId) -> Arc<Vec<usize>> {
    let source = db.source(file_id);
    let indexes = std::iter::once(0)
        .chain(source.match_indices('\n').map(|(i, _)| i + 1))
        .collect();

    Arc::new(indexes)
}

fn parse(db: &dyn Input, file_id: FileId) -> Parse {
    let source = db.source(file_id);
    let (messages_tx, _) = flume::unbounded();

    helios_parser::parse(0, &source, messages_tx)
}

fn all_bindings(db: &dyn Input, file_id: FileId) -> Arc<Vec<BindingId>> {
    let tree = db.parse(file_id).debug_tree();

    let bindings: Vec<_> = tree
        .split('\n')
        .map(|line| {
            let line = line.trim();
            let split = line.split(' ').collect::<Vec<_>>();

            let kind = split.get(0).unwrap().to_string();
            let text = split.get(1).map(|s| s.to_string());

            (kind, text)
        })
        .filter(|(kind, source)| {
            kind.starts_with("Identifier") && source.is_some()
        })
        .map(|(_, identifier)| {
            let identifier = identifier.unwrap_or_default();
            let binding_data = BindingData { identifier };
            db.intern_binding(binding_data)
        })
        .collect();

    Arc::new(bindings)
}
