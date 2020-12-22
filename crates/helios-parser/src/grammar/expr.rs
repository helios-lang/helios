use super::*;
use helios_syntax::Sym;

const PREFIX_OPS: &[SyntaxKind] =
    &[SyntaxKind::Sym_Minus, SyntaxKind::Sym_Bang];

/// Determines the prefix binding power of the given token. Currently, the only
/// legal prefix symbols are `SyntaxKind::Sym_Minus` and `SyntaxKind::Sym_Bang`.
fn prefix_binding_power(kind: SyntaxKind) -> ((), u8) {
    match kind {
        Sym!["-"] | Sym!["!"] => ((), 11),
        _ => unreachable!("Invalid symbol as prefix operator: {:?}", kind),
    }
}

const INFIX_OPS: &[SyntaxKind] = &[
    SyntaxKind::Sym_Asterisk,
    SyntaxKind::Sym_BangEq,
    SyntaxKind::Sym_Eq,
    SyntaxKind::Sym_ForwardSlash,
    SyntaxKind::Sym_Gt,
    SyntaxKind::Sym_GtEq,
    SyntaxKind::Sym_Lt,
    SyntaxKind::Sym_LtEq,
    SyntaxKind::Sym_LThinArrow,
    SyntaxKind::Sym_Minus,
    SyntaxKind::Sym_Plus,
    SyntaxKind::Sym_Semicolon,
];

/// Determines the infix binding power of the given token. A higher binding
/// power means higher precedence, meaning that it is more likely to hold onto
/// its adjacent operands.
fn infix_binding_power(kind: SyntaxKind) -> (u8, u8) {
    match kind {
        Sym![";"] => (1, 2),
        Sym!["<-"] => (3, 2),
        Sym!["="] | Sym!["!="] => (4, 3),
        Sym!["<"] | Sym![">"] | Sym!["<="] | Sym![">="] => (5, 6),
        Sym!["+"] | Sym!["-"] => (7, 8),
        Sym!["*"] | Sym!["/"] => (9, 10),
        _ => unreachable!("Invalid symbol as infix operator: {:?}", kind),
    }
}

/// Parses an expression.
pub(super) fn expr(parser: &mut Parser, min_bp: u8) -> Option<CompletedMarker> {
    let mut lhs = lhs(parser)?;

    loop {
        // Peek the next `SyntaxKind`, assuming it's an operator
        if let Some(operator) = parser.is_at_either(INFIX_OPS) {
            // Get the left and right binding power of the operator
            let (left_bp, right_bp) = infix_binding_power(*operator);

            if left_bp < min_bp {
                break;
            }

            // Consume the operator token
            parser.bump();

            let m = lhs.precede(parser);
            let parsed_rhs = expr(parser, right_bp).is_some();
            lhs = m.complete(parser, SyntaxKind::Exp_Binary);

            if !parsed_rhs {
                break;
            }
        } else {
            // What we consumed wasn't an operator; we don't know what to do
            // next, so we'll return and let the caller decide
            break;
        }
    }

    Some(lhs)
}

const LHS_KINDS: &[SyntaxKind] = &[
    SyntaxKind::Lit_Integer,
    SyntaxKind::Lit_Float,
    SyntaxKind::Identifier,
    SyntaxKind::Sym_LParen,
];

/// Parses the left-hand side of an expression.
fn lhs(parser: &mut Parser) -> Option<CompletedMarker> {
    let lhs_kinds_or_prefix_ops = &[LHS_KINDS, PREFIX_OPS].concat();

    // We'll check if the next `SyntaxKind` can start a LHS expression (either
    // any of `LHS_KINDS` or `PREFIX_OPS`)
    let cm = if let Some(kind) = parser.is_at_either(lhs_kinds_or_prefix_ops) {
        match kind {
            SyntaxKind::Lit_Integer | SyntaxKind::Lit_Float => literal(parser),
            SyntaxKind::Identifier => variable_ref(parser),
            SyntaxKind::Sym_LParen => paren_expr(parser),
            kind if PREFIX_OPS.contains(kind) => unary_prefix_expr(parser),
            _ => unreachable!("Got unexpected kind for LHS: {:?}", kind),
        }
    } else {
        parser.error();
        return None;
    };

    Some(cm)
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
fn unary_prefix_expr(parser: &mut Parser) -> CompletedMarker {
    let m = parser.start();

    // Get the right binding power of the operator
    let operator = SyntaxKind::Sym_Minus;
    let ((), right_bp) = prefix_binding_power(operator);

    // Consume the operator token and the expression it holds
    parser.bump();
    expr(parser, right_bp);

    m.complete(parser, SyntaxKind::Exp_UnaryPrefix)
}

/// Parses an expression surrounded by parenthesis.
fn paren_expr(parser: &mut Parser) -> CompletedMarker {
    assert!(parser.is_at(SyntaxKind::Sym_LParen));

    let m = parser.start();

    // Consume the opening parenthesis and the expression inside
    parser.bump();
    expr(parser, 0);

    // Consume the closing parenthesis if possible
    parser.expect(SyntaxKind::Sym_RParen);

    m.complete(parser, SyntaxKind::Exp_Paren)
}

#[cfg(test)]
mod tests {
    use crate::check;
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
    fn test_parse_unclosed_parenthesized_expression() {
        check(
            "(foo",
            expect![[r#"
Root@0..4
  Exp_Paren@0..4
    Sym_LParen@0..1 "("
    Exp_VariableRef@1..4
      Identifier@1..4 "foo"
---
error at 1..4: expected `*`, `!=`, `=`, `/`, `>`, `>=`, `<`, `<=`, `<-`, `-`, `+`, `;` or `)`"#]],
        );
    }

    #[test]
    fn test_do_not_continue_to_parse_missing_rhs_of_expression() {
        check(
            "(1+",
            expect![[r#"
Root@0..3
  Exp_Paren@0..3
    Sym_LParen@0..1 "("
    Exp_Binary@1..3
      Exp_Literal@1..2
        Lit_Integer@1..2 "1"
      Sym_Plus@2..3 "+"
---
error at 2..3: expected integer literal, float literal, identifier, `(`, `-` or `!`
error at 2..3: expected `)`"#]],
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
