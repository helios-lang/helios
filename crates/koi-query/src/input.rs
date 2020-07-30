use koi_syntax_old::Ast;
use koi_syntax_old::source::Source;
use std::sync::Arc;

#[salsa::query_group(InputStorage)]
pub trait Input: salsa::Database {
    #[salsa::input]
    fn source_text(&self, path: String) -> Arc<String>;

    fn source_length(&self, path: String) -> usize;

    fn line_offsets(&self, path: String) -> Vec<usize>;

    fn ast(&self, path: String) -> Arc<Ast>;
}

fn source_length(db: &impl Input, path: String) -> usize {
    let contents = db.source_text(path);
    contents.len()
}

/// Returns a vector of offsets for each line in the source. The last element
/// is the length of the whole source text.
fn line_offsets(db: &impl Input, path: String) -> Vec<usize> {
    let mut accumulator = 0;
    let contents = &db.source_text(path)[..];

    contents.lines()
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

fn ast(db: &impl Input, path: String) -> Arc<Ast> {
    let contents = db.source_text(path);
    let mut contents = contents.as_bytes();
    let source = Source::stream(&mut contents).unwrap();
    Arc::new(koi_syntax_old::parse(source))
}
