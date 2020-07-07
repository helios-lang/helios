use koi_syntax::Ast;
use koi_syntax::source::Source;
use std::sync::Arc;

#[salsa::query_group(InputStorage)]
pub trait Input: salsa::Database {
    #[salsa::input]
    fn source_text(&self, path: String) -> Arc<String>;

    fn ast(&self, path: String) -> Arc<Ast>;

    fn length(&self, path: String) -> usize;
}

fn ast(db: &impl Input, path: String) -> Arc<Ast> {
    let contents = db.source_text(path);
    let mut contents = contents.as_bytes();
    let source = Source::stream(&mut contents).unwrap();
    Arc::new(koi_syntax::parse(source))
}

fn length(db: &impl Input, path: String) -> usize {
    let string = db.source_text(path);
    string.len()
}
