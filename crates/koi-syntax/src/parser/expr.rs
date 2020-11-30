use super::Parser;
use crate::syntax::SyntaxKind;

pub fn parse_expr(parser: &mut Parser) {
    match parser.peek() {
        Some(SyntaxKind::Lit_Integer) | Some(SyntaxKind::Identifier) => {
            parser.bump()
        },
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::super::check;
    use expect_test::expect;

    #[test]
    fn test_parse_lone_integer() {
        check(
            "123",
            expect![[r#"
Root@0..3
  Lit_Integer@0..3 "123""#]],
        );
    }

    #[test]
    fn test_parse_lone_identifier() {
        check(
            "counter",
            expect![[r#"
Root@0..7
  Identifier@0..7 "counter""#]],
        );
    }
}
