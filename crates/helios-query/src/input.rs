#![allow(unused)]

use crate::interner::{BindingData, BindingId, Interner};
use helios_parser::Parse;
use std::sync::Arc;

pub type FileId = usize;

#[salsa::query_group(InputDatabase)]
pub trait Input: Interner {
    #[salsa::input]
    fn source(&self, file_id: FileId) -> Arc<String>;

    /// The length of the source's text.
    fn source_len(&self, file_id: FileId) -> usize;

    // /// Constructs a parsed syntax tree of the given file.
    // fn parse(&self, file_id: FileId) -> Parse;

    // fn all_bindings(&self, file_id: FileId) -> Arc<Vec<BindingId>>;
}

fn source_len(db: &dyn Input, file_id: FileId) -> usize {
    let source = db.source(file_id);
    source.len()
}

/*
fn parse(db: &dyn Input, file_id: FileId) -> Parse {
    let source = db.source(file_id);
    let (messages_tx, _) = flume::unbounded();

    helios_parser::parse(0, &source, messages_tx)
}

fn all_bindings(db: &dyn Input, file_id: FileId) -> Arc<Vec<BindingId>> {
    let tree = db.parse(file_id).debug_tree();

    use std::collections::HashSet;
    let bindings: HashSet<_> = tree
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
            let identifier = identifier.trim_matches('"').to_string();
            let binding_data = BindingData { identifier };
            db.intern_binding(binding_data)
        })
        .collect();

    Arc::new(bindings.into_iter().collect())
}
*/
