#![allow(unused)]

use crate::interner::{BindingData, BindingId, Interner};
use helios_diagnostics::Diagnostic;
use helios_parser::Parse;
use std::sync::Arc;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FileId(pub u32);

#[salsa::query_group(InputDatabase)]
pub trait Input: Interner {
    /// The source text of a file.
    #[salsa::input]
    fn source(&self, file_id: FileId) -> Arc<String>;

    /// The length of a file's source text.
    fn source_len(&self, file_id: FileId) -> usize;

    /// The parsed syntax tree of the given file.
    fn parse(&self, file_id: FileId) -> Parse<FileId>;

    /// Diagnostics emitted by the parser for a given file.
    fn diagnostics(&self, file_id: FileId) -> Arc<Vec<Diagnostic<FileId>>>;
}

fn source_len(db: &dyn Input, file_id: FileId) -> usize {
    let source = db.source(file_id);
    source.len()
}

fn parse(db: &dyn Input, file_id: FileId) -> Parse<FileId> {
    let source = db.source(file_id);
    helios_parser::parse(file_id, &source)
}

fn diagnostics(
    db: &dyn Input,
    file_id: FileId,
) -> Arc<Vec<Diagnostic<FileId>>> {
    let parse = db.parse(file_id);
    let messages = parse.messages();
    Arc::new(messages.iter().map(|message| message.into()).collect())
}
