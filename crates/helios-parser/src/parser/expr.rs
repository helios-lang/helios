use super::marker::CompletedMarker;
use super::Parser;
use helios_syntax::{Sym, SyntaxKind};

/// Determines the prefix binding power of the given token. Currently, the only
/// legal prefix symbols are `SyntaxKind::Sym_Minus` and `SyntaxKind::Sym_Bang`.
fn prefix_binding_power(token: SyntaxKind) -> Option<((), u8)> {
    let power = match token {
        Sym!["-"] | Sym!["!"] => ((), 11),
        _ => return None,
    };

    Some(power)
}

/// Determines the infix binding power of the given token. A higher binding
/// power means higher precedence, meaning that it is more likely to hold onto
/// its adjacent operands.
fn infix_binding_power(token: SyntaxKind) -> Option<(u8, u8)> {
    let power = match token {
        Sym![";"] => (1, 2),
        Sym!["<-"] => (3, 2),
        Sym!["="] | Sym!["!="] => (4, 3),
        Sym!["<"] | Sym![">"] | Sym!["<="] | Sym![">="] => (5, 6),
        Sym!["+"] | Sym!["-"] => (7, 8),
        Sym!["*"] | Sym!["/"] => (9, 10),
        _ => return None,
    };

    Some(power)
}

/// Parses an expression.
pub(super) fn parse_expr(parser: &mut Parser, min_bp: u8) {
    let mut lhs = match lhs(parser) {
        Some(lhs) => lhs,
        None => return,
    };

    loop {
        // Peek the next token, assuming it's an operator
        let op = match parser.peek() {
            Some(token) => token,
            _ => return,
        };

        // Get the left and right binding power of the supposed operator
        if let Some((left_bp, right_bp)) = infix_binding_power(op) {
            if left_bp < min_bp {
                return;
            }

            // Consume the operator token
            parser.bump();

            let m = lhs.precede(parser);
            parse_expr(parser, right_bp);
            lhs = m.complete(parser, SyntaxKind::Exp_Binary);
        } else {
            return;
        }
    }
}

/// Parses the left-hand side of an expression.
fn lhs(parser: &mut Parser) -> Option<CompletedMarker> {
    use SyntaxKind::*;
    let completed_marker = match parser.peek() {
        Some(Lit_Integer) | Some(Lit_Float) => literal(parser),
        Some(Identifier) => variable_ref(parser),
        Some(Sym_Minus) => unary_prefix_expr(parser),
        Some(Sym_LParen) => paren_expr(parser),
        _ => return None,
    };

    Some(completed_marker)
}

/// Parses a literal that may stand alone as an expression.
fn literal(parser: &mut Parser) -> CompletedMarker {
    use SyntaxKind::*;
    assert!(parser.is_at(Lit_Integer) || parser.is_at(Lit_Float));

    let m = parser.start();
    parser.bump();
    m.complete(parser, Exp_Literal)
}

/// Parses an identifier as a variable reference.
fn variable_ref(parser: &mut Parser) -> CompletedMarker {
    assert!(parser.is_at(SyntaxKind::Identifier));

    let m = parser.start();
    parser.bump();
    m.complete(parser, SyntaxKind::Exp_VariableRef)
}

/// Parses a unary expression with a prefixed operator.
///
/// Currently, only [`SyntaxKind::Sym_Minus`] is a valid prefix operator.
fn unary_prefix_expr(parser: &mut Parser) -> CompletedMarker {
    assert!(parser.is_at(SyntaxKind::Sym_Minus));

    let m = parser.start();

    // Get the right binding power of the operator
    let op = SyntaxKind::Sym_Minus;
    let ((), right_bp) = prefix_binding_power(op).unwrap();

    // Consume the operator token and the expression it holds
    parser.bump();
    parse_expr(parser, right_bp);

    m.complete(parser, SyntaxKind::Exp_UnaryPrefix)
}

/// Parses an expression surrounded by parenthesis.
fn paren_expr(parser: &mut Parser) -> CompletedMarker {
    assert!(parser.is_at(SyntaxKind::Sym_LParen));

    let m = parser.start();

    // Consume the opening parenthesis and the expression inside
    parser.bump();
    parse_expr(parser, 0);

    // Consume the closing parenthesis if possible
    if let Some(SyntaxKind::Sym_RParen) = parser.peek() {
        parser.bump();
    }

    m.complete(parser, SyntaxKind::Exp_Paren)
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
  Exp_Literal@0..3
    Lit_Integer@0..3 "123""#]],
        );
    }

    #[test]
    fn test_parse_lone_identifier() {
        check(
            "hello_world",
            expect![[r#"
Root@0..11
  Exp_VariableRef@0..11
    Identifier@0..11 "hello_world""#]],
        );
    }

    #[test]
    fn test_unary_prefix_expression_simple() {
        check(
            "-10",
            expect![[r#"
Root@0..3
  Exp_UnaryPrefix@0..3
    Sym_Minus@0..1 "-"
    Exp_Literal@1..3
      Lit_Integer@1..3 "10""#]],
        );
    }

    #[test]
    fn test_parse_with_proper_binding_powers() {
        check(
            "-10+20",
            expect![[r#"
Root@0..6
  Exp_Binary@0..6
    Exp_UnaryPrefix@0..3
      Sym_Minus@0..1 "-"
      Exp_Literal@1..3
        Lit_Integer@1..3 "10"
    Sym_Plus@3..4 "+"
    Exp_Literal@4..6
      Lit_Integer@4..6 "20""#]],
        );
    }

    #[test]
    fn test_binary_expression_simple() {
        check(
            "1+2",
            expect![[r#"
Root@0..3
  Exp_Binary@0..3
    Exp_Literal@0..1
      Lit_Integer@0..1 "1"
    Sym_Plus@1..2 "+"
    Exp_Literal@2..3
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
        Exp_Literal@0..1
          Lit_Integer@0..1 "1"
        Sym_Plus@1..2 "+"
        Exp_Literal@2..3
          Lit_Integer@2..3 "2"
      Sym_Plus@3..4 "+"
      Exp_Literal@4..5
        Lit_Integer@4..5 "3"
    Sym_Plus@5..6 "+"
    Exp_Literal@6..7
      Lit_Integer@6..7 "4""#]],
        );
    }

    #[test]
    fn test_parse_binary_expression_with_mixed_binding_powers() {
        check(
            "1+2*3-4",
            expect![[r#"
Root@0..7
  Exp_Binary@0..7
    Exp_Binary@0..5
      Exp_Literal@0..1
        Lit_Integer@0..1 "1"
      Sym_Plus@1..2 "+"
      Exp_Binary@2..5
        Exp_Literal@2..3
          Lit_Integer@2..3 "2"
        Sym_Asterisk@3..4 "*"
        Exp_Literal@4..5
          Lit_Integer@4..5 "3"
    Sym_Minus@5..6 "-"
    Exp_Literal@6..7
      Lit_Integer@6..7 "4""#]],
        );
    }

    #[test]
    fn test_parenthesized_expression() {
        check(
            "5*(2+1)",
            expect![[r#"
Root@0..7
  Exp_Binary@0..7
    Exp_Literal@0..1
      Lit_Integer@0..1 "5"
    Sym_Asterisk@1..2 "*"
    Exp_Paren@2..7
      Sym_LParen@2..3 "("
      Exp_Binary@3..6
        Exp_Literal@3..4
          Lit_Integer@3..4 "2"
        Sym_Plus@4..5 "+"
        Exp_Literal@5..6
          Lit_Integer@5..6 "1"
      Sym_RParen@6..7 ")""#]],
        );
    }

    #[test]
    fn test_parse_complex_expression() {
        check(
            "-(2-((10+10)))*20+-5",
            expect![[r#"
Root@0..20
  Exp_Binary@0..20
    Exp_Binary@0..17
      Exp_UnaryPrefix@0..14
        Sym_Minus@0..1 "-"
        Exp_Paren@1..14
          Sym_LParen@1..2 "("
          Exp_Binary@2..13
            Exp_Literal@2..3
              Lit_Integer@2..3 "2"
            Sym_Minus@3..4 "-"
            Exp_Paren@4..13
              Sym_LParen@4..5 "("
              Exp_Paren@5..12
                Sym_LParen@5..6 "("
                Exp_Binary@6..11
                  Exp_Literal@6..8
                    Lit_Integer@6..8 "10"
                  Sym_Plus@8..9 "+"
                  Exp_Literal@9..11
                    Lit_Integer@9..11 "10"
                Sym_RParen@11..12 ")"
              Sym_RParen@12..13 ")"
          Sym_RParen@13..14 ")"
      Sym_Asterisk@14..15 "*"
      Exp_Literal@15..17
        Lit_Integer@15..17 "20"
    Sym_Plus@17..18 "+"
    Exp_UnaryPrefix@18..20
      Sym_Minus@18..19 "-"
      Exp_Literal@19..20
        Lit_Integer@19..20 "5""#]],
        )
    }

    #[test]
    fn test_parse_number_preceded_by_whitespace() {
        check(
            "   9876",
            expect![[r#"
Root@0..7
  Whitespace@0..3 "   "
  Exp_Literal@3..7
    Lit_Integer@3..7 "9876""#]],
        );
    }

    #[test]
    fn test_parse_number_followed_by_whitespace() {
        check(
            "1234   ",
            expect![[r#"
Root@0..7
  Exp_Literal@0..7
    Lit_Integer@0..4 "1234"
    Whitespace@4..7 "   ""#]],
        );
    }

    #[test]
    fn test_parse_number_surrounded_by_whitespace() {
        check(
            " 123     ",
            expect![[r#"
Root@0..9
  Whitespace@0..1 " "
  Exp_Literal@1..9
    Lit_Integer@1..4 "123"
    Whitespace@4..9 "     ""#]],
        );
    }

    #[test]
    fn test_parse_binary_expression_interspersed_with_comments() {
        check(
            "
1
  + 1 // Add one
  + 10 // Add ten",
            expect![[r#"
Root@0..37
  Whitespace@0..1 "\n"
  Exp_Binary@1..37
    Exp_Binary@1..22
      Exp_Literal@1..5
        Lit_Integer@1..2 "1"
        Whitespace@2..5 "\n  "
      Sym_Plus@5..6 "+"
      Whitespace@6..7 " "
      Exp_Literal@7..22
        Lit_Integer@7..8 "1"
        Whitespace@8..9 " "
        Comment@9..19 "// Add one"
        Whitespace@19..22 "\n  "
    Sym_Plus@22..23 "+"
    Whitespace@23..24 " "
    Exp_Literal@24..37
      Lit_Integer@24..26 "10"
      Whitespace@26..27 " "
      Comment@27..37 "// Add ten""#]],
        );
    }
}
