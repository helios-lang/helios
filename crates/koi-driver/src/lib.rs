use koi_parser::source::Source;
use koi_parser::Ast;
use koi_parser::reporter::Reporter;

// type Result<T> = std::result::Result<T, Vec<Diagnostic>>;

// pub fn start(_file_name: &str) -> Result<()> {
//     unimplemented!()
// }

pub fn tokenize<'a>(source: Source<'a>, reporter: Box<dyn Reporter>, should_consume_doc_comments: bool) -> Ast {
    koi_parser::parse(source, reporter, should_consume_doc_comments)
}
