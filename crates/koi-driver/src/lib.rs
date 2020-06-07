mod diagnostic;
mod parser;
mod source;

pub use diagnostic::Diagnostic;
pub use source::Source;
pub use parser::Ast;

type Result<T> = std::result::Result<T, Vec<Diagnostic>>;

pub fn start(_file_name: &str) -> Result<()> {
    unimplemented!()
}

pub fn tokenize<'a>(source: Source<'a>) -> Ast {
    parser::parse(source)
}
