mod diagnostic;

pub use diagnostic::Diagnostic;
use koi_parser::source::{Position, Source};
use koi_parser::Ast;

type Result<T> = std::result::Result<T, Vec<Diagnostic>>;

pub fn start(_file_name: &str) -> Result<()> {
    unimplemented!()
}

pub fn tokenize<'a>(source: Source<'a>) -> Ast {
    koi_parser::parse(source)
}
