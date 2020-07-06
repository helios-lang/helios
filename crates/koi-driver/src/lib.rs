use koi_syntax::source::Source;
use koi_syntax::Ast;

pub fn parse<'a>(source: Source<'a>) -> Ast {
    koi_syntax::parse(source)
}
