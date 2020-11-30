use super::Parser;
use crate::syntax::SyntaxKind;

/// Determines the prefix binding power of the given token. Currently, the only
/// legal prefix symbols are `SyntaxKind::Sym_Minus` and `SyntaxKind::Sym_Bang`.
#[allow(dead_code)]
fn prefix_binding_power(token: SyntaxKind) -> Option<u8> {
    use crate::Sym as S;
    let power = match token {
        S!["-"] | S!["!"] => 9,
        _ => return None,
    };

    Some(power)
}

/// Determines the infix binding power of the given token. A higher binding
/// power means higher precedence, meaning that it is more likely to hold onto
/// its adjacent operands.
fn infix_binding_power(token: SyntaxKind) -> Option<(u8, u8)> {
    use crate::Sym as S;
    let power = match token {
        S![";"] => (1, 2),
        S!["<-"] => (3, 2),
        S!["="] | S!["!="] => (4, 3),
        S!["<"] | S![">"] | S!["<="] | S![">="] => (5, 6),
        S!["+"] | S!["-"] => (7, 8),
        S!["*"] | S!["/"] => (9, 10),
        _ => return None,
    };

    Some(power)
}

pub fn parse_expr(parser: &mut Parser, min_binding_power: u8) {
    let checkpoint = parser.checkpoint();

    match parser.peek() {
        Some(SyntaxKind::Lit_Integer) | Some(SyntaxKind::Identifier) => {
            parser.bump()
        }
        _ => {}
    }

    loop {
        let op = match parser.peek() {
            Some(token) => token,
            _ => break,
        };

        if let Some((left_binding_pow, right_binding_pow)) =
            infix_binding_power(op)
        {
            if left_binding_pow < min_binding_power {
                return;
            }

            // Eat the operator's token.
            parser.bump();

            parser.start_node_at(checkpoint, SyntaxKind::Exp_Binary);
            parse_expr(parser, right_binding_pow);
            parser.finish_node();

            continue;
        }

        break;
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

    #[test]
    fn test_binary_expression_simple() {
        check(
            "1+2",
            expect![[r#"
Root@0..3
  Exp_Binary@0..3
    Lit_Integer@0..1 "1"
    Sym_Plus@1..2 "+"
    Lit_Integer@2..3 "2""#]],
        );
    }

    #[test]
    fn test_parse_binary_expression_left_associative() {
        check(
            "1+2+3+4",
            expect![[r#"
Root@0..7
  Exp_Binary@0..7
    Exp_Binary@0..5
      Exp_Binary@0..3
        Lit_Integer@0..1 "1"
        Sym_Plus@1..2 "+"
        Lit_Integer@2..3 "2"
      Sym_Plus@3..4 "+"
      Lit_Integer@4..5 "3"
    Sym_Plus@5..6 "+"
    Lit_Integer@6..7 "4""#]],
        );
    }

    #[test]
    fn test_parse_binary_expression_mixed_binding_powers() {
        check(
            "1+2*3-4",
            expect![[r#"
Root@0..7
  Exp_Binary@0..7
    Exp_Binary@0..5
      Lit_Integer@0..1 "1"
      Sym_Plus@1..2 "+"
      Exp_Binary@2..5
        Lit_Integer@2..3 "2"
        Sym_Asterisk@3..4 "*"
        Lit_Integer@4..5 "3"
    Sym_Minus@5..6 "-"
    Lit_Integer@6..7 "4""#]],
        );
    }
}
