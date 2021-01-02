//! Module responsible for describing how to parse nodes.

use crate::parser::marker::CompletedMarker;
use crate::parser::Parser;
use helios_syntax::SyntaxKind;

mod decl;
mod expr;

pub(crate) fn root<FileId>(p: &mut Parser<FileId>) -> CompletedMarker
where
    FileId: Clone + Default,
{
    let m = p.start();

    while !p.is_at_end() {
        decl::decl(p);
    }

    m.complete(p, SyntaxKind::Root)
}

#[cfg(test)]
mod tests {
    use crate::check;
    use expect_test::expect;

    #[test]
    fn test_parse_multiple_declarations() {
        check(
            "let a = 1\na",
            expect![[r#"
                Root@0..11
                  Dec_GlobalBinding@0..10
                    Kwd_Let@0..3 "let"
                    Whitespace@3..4 " "
                    Identifier@4..5 "a"
                    Whitespace@5..6 " "
                    Sym_Eq@6..7 "="
                    Whitespace@7..8 " "
                    Exp_Literal@8..10
                      Lit_Integer@8..9 "1"
                      Whitespace@9..10 "\n"
                  Exp_VariableRef@10..11
                    Identifier@10..11 "a"
            "#]],
        );
    }
}
