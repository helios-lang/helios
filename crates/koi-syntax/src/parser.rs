use crate::lexer::Lexer;
use crate::syntax::{SyntaxKind, SyntaxNode};
use rowan::{GreenNode, GreenNodeBuilder};
use std::iter::Peekable;

pub struct Parser {
    lexer: Peekable<Lexer>,
    builder: GreenNodeBuilder<'static>,
}

impl Parser {
    pub fn new(source: String) -> Self {
        Self {
            lexer: Lexer::new(source).peekable(),
            builder: GreenNodeBuilder::new(),
        }
    }

    pub fn parse(mut self) -> ParserResult {
        self.builder.start_node(SyntaxKind::Root.into());

        match self.lexer.peek().map(|(kind, _)| *kind) {
            Some(SyntaxKind::Lit_Integer) | Some(SyntaxKind::Identifier) => {
                self.bump()
            },
            _ => {}
        }

        self.builder.finish_node();

        ParserResult {
            green_node: self.builder.finish(),
        }
    }

    fn bump(&mut self) {
        let (kind, text) = self.lexer.next().expect("Failed to get next token");
        self.builder.token(kind.into(), text.into())
    }
}

pub struct ParserResult {
    green_node: GreenNode,
}

impl ParserResult {
    pub fn debug_tree(&self) -> String {
        let syntax_node = SyntaxNode::new_root(self.green_node.clone());
        let formatted = format!("{:#?}", syntax_node);

        // trims newline at the end
        formatted[0..formatted.len() - 1].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use expect_test::{expect, Expect};

    fn check(input: &str, expected_tree: Expect) {
        let parse_result = Parser::new(input.to_string()).parse();
        expected_tree.assert_eq(&parse_result.debug_tree());
    }

    #[test]
    fn test_parse_nothing() {
        check("", expect![[r#"Root@0..0"#]]);
    }

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
