use super::*;

pub(super) fn decl(parser: &mut Parser) -> Option<CompletedMarker> {
    if parser.is_at(SyntaxKind::Kwd_Let) {
        Some(global_binding(parser))
    } else {
        expr::expr(parser, 0)
    }
}

fn global_binding(parser: &mut Parser) -> CompletedMarker {
    assert!(parser.is_at(SyntaxKind::Kwd_Let));
    let m = parser.start();
    parser.bump();

    parser.expect(SyntaxKind::Identifier);
    parser.expect(SyntaxKind::Sym_Eq);

    expr::expr(parser, 0);

    m.complete(parser, SyntaxKind::Dec_GlobalBinding)
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
      Identifier@10..13 "bar""#]],
        );
    }

//     #[test]
//     fn test_parse_incomplete_global_declaration() {
//         check(
//             "let a =\nlet b = a",
//             expect![[r#"
// Root@0..17
//   Dec_GlobalBinding@0..8
//     Kwd_Let@0..3 "let"
//     Whitespace@3..4 " "
//     Identifier@4..5 "a"
//     Whitespace@5..6 " "
//     Sym_Eq@6..7 "="
//     Whitespace@7..8 "\n"
//   Dec_GlobalBinding@8..17
//     Kwd_Let@8..11 "let"
//     Whitespace@11..12 " "
//     Identifier@12..13 "b"
//     Whitespace@13..14 " "
//     Sym_Eq@14..15 "="
//     Whitespace@15..16 " "
//     Exp_VariableRef@16..17
//       Identifier@16..17 "a"
// ---
// error at 8..11: expected integer literal, float literal, identifier, `(`, `-` or `!`, found `let`"#]],
//         );
//     }
}
