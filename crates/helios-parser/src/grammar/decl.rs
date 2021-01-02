use super::*;

pub(super) fn decl<FileId>(p: &mut Parser<FileId>) -> Option<CompletedMarker>
where
    FileId: Clone + Default,
{
    if p.is_at(SyntaxKind::Kwd_Let) {
        Some(global_binding(p))
    } else {
        expr::expr(p, 0)
    }
}

fn global_binding<FileId>(p: &mut Parser<FileId>) -> CompletedMarker
where
    FileId: Clone + Default,
{
    assert!(p.is_at(SyntaxKind::Kwd_Let));
    let m = p.start();
    p.bump();

    p.expect(SyntaxKind::Identifier, SyntaxKind::Dec_GlobalBinding);
    p.expect(SyntaxKind::Sym_Eq, SyntaxKind::Dec_GlobalBinding);

    expr::expr(p, 0);

    m.complete(p, SyntaxKind::Dec_GlobalBinding)
}

#[cfg(test)]
mod tests {
    use crate::check;
    use expect_test::expect;

    #[test]
    fn test_parse_global_binding_declaration() {
        check(
            "let foo = bar",
            expect![[r#"
                Root@0..13
                  Dec_GlobalBinding@0..13
                    Kwd_Let@0..3 "let"
                    Whitespace@3..4 " "
                    Identifier@4..7 "foo"
                    Whitespace@7..8 " "
                    Sym_Eq@8..9 "="
                    Whitespace@9..10 " "
                    Exp_VariableRef@10..13
                      Identifier@10..13 "bar"
            "#]],
        );
    }
}
