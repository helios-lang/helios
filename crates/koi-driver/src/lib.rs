use koi_syntax_old::source::Source;
use koi_syntax_old::Ast;

pub fn parse<'a>(source: Source<'a>) -> Ast {
    koi_syntax_old::parse(source)
}
