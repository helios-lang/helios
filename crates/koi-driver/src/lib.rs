use koi_syntax::source::Source;
use koi_syntax::Ast;

pub fn tokenize<'a>(source: Source<'a>) -> Ast {
    koi_syntax::parse(source)
}
