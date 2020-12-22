use helios_syntax::SyntaxKind;
use std::fmt;
use text_size::TextRange;

/// Uses `TextRange` for the range of the error because it uses `u32`s
/// internally, which makes `ParseError` much smaller in memory.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ParseError {
    pub(super) expected: Vec<SyntaxKind>,
    pub(super) found: Option<SyntaxKind>,
    pub(super) range: TextRange,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "error at {}..{}: expected ",
            u32::from(self.range.start()),
            u32::from(self.range.end()),
        )?;

        for (i, kind) in self.expected.iter().enumerate() {
            if i == 0 {
                write!(f, "{}", kind)?;
            } else if i == (self.expected.len() - 1) {
                write!(f, " or {}", kind)?;
            } else {
                write!(f, ", {}", kind)?;
            }
        }

        if let Some(found) = self.found {
            write!(f, ", found {}", found)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Range;

    fn check(
        expected: Vec<SyntaxKind>,
        found: Option<SyntaxKind>,
        range: Range<u32>,
        output: &str,
    ) {
        let error = ParseError {
            expected,
            found,
            range: {
                let start = range.start.into();
                let end = range.end.into();
                TextRange::new(start, end)
            },
        };

        assert_eq!(format!("{}", error), output)
    }

    #[test]
    fn test_error_expected_kind_found_none() {
        check(
            vec![SyntaxKind::Sym_RParen],
            None,
            5..6,
            "error at 5..6: expected `)`",
        )
    }

    #[test]
    fn test_error_expected_kind_found_identifier() {
        check(
            vec![SyntaxKind::Sym_Eq],
            Some(SyntaxKind::Identifier),
            10..20,
            "error at 10..20: expected `=`, found identifier",
        )
    }

    #[test]
    fn test_error_expected_kind_found_multiple() {
        check(
            vec![
                SyntaxKind::Lit_Integer,
                SyntaxKind::Lit_Float,
                SyntaxKind::Identifier,
                SyntaxKind::Sym_Minus,
                SyntaxKind::Sym_LParen,
            ],
            Some(SyntaxKind::Kwd_Let),
            100..105,
            "error at 100..105: expected integer literal, float literal, \
             identifier, `-` or `(`, found `let`",
        )
    }
}
