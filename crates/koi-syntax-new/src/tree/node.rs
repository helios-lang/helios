use crate::tree::token::*;
use crate::source::TextSpan;
use std::fmt::Debug;
use std::rc::Rc;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Syntax {
    Node(Rc<SyntaxNode>),
    Token(Rc<SyntaxToken>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SyntaxNode {
    raw: Rc<RawSyntaxNode>,
    span: TextSpan,
    children: Vec<Syntax>,
}

impl SyntaxNode {
    /// The kind of the token.
    pub fn kind(&self) -> NodeKind {
        self.raw.kind.clone()
    }

    /// The span of the token.
    ///
    /// This span does not include any leading or trailing trivia.
    pub fn span(&self) -> TextSpan {
        self.span
    }

    /// The full span of the token.
    ///
    /// A token's full span is it's normal span, plus the span of any leading
    /// and trailing trivia it may have.
    pub fn full_span(&self) -> TextSpan {
        fn get_full_span(child: &Syntax) -> TextSpan {
            match child {
                Syntax::Node(node) => node.full_span(),
                Syntax::Token(token) => token.full_span(),
            }
        }

        TextSpan::from_spans(
            self.children.first().map_or(self.span(), get_full_span),
            self.children.last().map_or(self.span(), get_full_span),
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RawSyntax {
    Node(Rc<RawSyntaxNode>),
    Token(Rc<RawSyntaxToken>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RawSyntaxNode {
    kind: NodeKind,
    children: Vec<RawSyntax>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum NodeKind {
    LiteralExpr,
    GroupedExpr,
    BinaryExpr,
    UnaryExpr,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    fn print_syntax(syntax: &Syntax, level: usize) {
        match syntax {
            Syntax::Token(token) => {
                println!("{}- TOK {:p} is {:p} => {:?} @{} (@{})",
                    "    ".repeat(level),
                    token,
                    token.raw,
                    token.kind(),
                    token.span(),
                    token.full_span(),
                );
            }
            Syntax::Node(node) => {
                println!("{}- NOD {:p} is {:p} => {:?} @{} (@{})",
                    "    ".repeat(level),
                    node,
                    node.raw,
                    node.kind(),
                    node.span(),
                    node.full_span(),
                );

                node.children.iter().for_each(|child| {
                    print_syntax(child, level + 1);
                });
            }
        }
    }
    #[test]
    #[rustfmt::skip]
    fn test_syntax_node_nested_expr() {
        // Test string:
        /*
        (foo + bar - 2.0) * (foo + bar - 2.0) + foo
        */

        // -- RAW SYNTAX ---

        let raw_sym_lpr = Rc::new(RawSyntaxToken::with(TokenKind::Symbol(Symbol::LParen), "(".to_string()));
        let raw_idn_foo = Rc::new(RawSyntaxToken::with(TokenKind::Identifier, "foo".to_string()));
        let raw_sym_pls = Rc::new(RawSyntaxToken::with(TokenKind::Symbol(Symbol::Plus), "+".to_string()));
        let raw_idn_bar = Rc::new(RawSyntaxToken::with(TokenKind::Identifier, "bar".to_string()));
        let raw_sym_mns = Rc::new(RawSyntaxToken::with(TokenKind::Symbol(Symbol::Minus), "-".to_string()));
        let raw_lit_2fl = Rc::new(RawSyntaxToken::with(TokenKind::Literal(Literal::Float), "2.0".to_string()));
        let raw_sym_rpr = Rc::new(RawSyntaxToken::with(TokenKind::Symbol(Symbol::RParen), ")".to_string()));
        let raw_sym_ast = Rc::new(RawSyntaxToken::with(TokenKind::Symbol(Symbol::Asterisk), "*".to_string()));

        // RAW `(foo + bar - 2.0)`
        let raw_grp_expr_idn_foo_sym_pls_idn_bar_sym_mns_lit_2fl =
            Rc::new(RawSyntaxNode {
                kind: NodeKind::GroupedExpr,
                children: vec![
                    RawSyntax::Token(Rc::clone(&raw_sym_lpr)),
                    RawSyntax::Token(Rc::clone(&raw_idn_foo)),
                    RawSyntax::Token(Rc::clone(&raw_sym_pls)),
                    RawSyntax::Token(Rc::clone(&raw_idn_bar)),
                    RawSyntax::Token(Rc::clone(&raw_sym_mns)),
                    RawSyntax::Token(Rc::clone(&raw_lit_2fl)),
                    RawSyntax::Token(Rc::clone(&raw_sym_rpr)),
                ],
            });

        // RAW `_ * _`
        let raw_bin_expr_grp_expr_sym_ast_grp_expr =
            Rc::new(RawSyntaxNode {
                kind: NodeKind::BinaryExpr,
                children: vec![
                    RawSyntax::Node(Rc::clone(&raw_grp_expr_idn_foo_sym_pls_idn_bar_sym_mns_lit_2fl)),
                    RawSyntax::Token(Rc::clone(&raw_sym_ast)),
                    RawSyntax::Node(Rc::clone(&raw_grp_expr_idn_foo_sym_pls_idn_bar_sym_mns_lit_2fl)),
                ],
            });

        // RAW `_ + foo`
        let raw_bin_expr_bin_expr_sym_pls_idn_foo =
            Rc::new(RawSyntaxNode {
                kind: NodeKind::BinaryExpr,
                children: vec![
                    RawSyntax::Node(Rc::clone(&raw_bin_expr_grp_expr_sym_ast_grp_expr)),
                    RawSyntax::Token(Rc::clone(&raw_sym_pls)),
                    RawSyntax::Token(Rc::clone(&raw_idn_foo)),
                ],
            });

        // -- CONCRETE SYNTAX ---

        let con_sym_lpr_1 = Rc::new(SyntaxToken::with       (Rc::clone(&raw_sym_lpr), TextSpan::new( 0, 1)));
        let con_idn_foo_1 = Rc::new(SyntaxToken::with_trivia(Rc::clone(&raw_idn_foo), TextSpan::new( 1, 3), Vec::new(), vec![SyntaxTrivia::Space(1)]));
        let con_sym_pls_1 = Rc::new(SyntaxToken::with_trivia(Rc::clone(&raw_sym_pls), TextSpan::new( 5, 3), Vec::new(), vec![SyntaxTrivia::Space(1)]));
        let con_idn_bar_1 = Rc::new(SyntaxToken::with_trivia(Rc::clone(&raw_idn_bar), TextSpan::new( 7, 3), Vec::new(), vec![SyntaxTrivia::Space(1)]));
        let con_sym_mns_1 = Rc::new(SyntaxToken::with_trivia(Rc::clone(&raw_sym_mns), TextSpan::new(11, 1), Vec::new(), vec![SyntaxTrivia::Space(1)]));
        let con_lit_2fl_1 = Rc::new(SyntaxToken::with       (Rc::clone(&raw_lit_2fl), TextSpan::new(13, 3)));
        let con_sym_rpr_1 = Rc::new(SyntaxToken::with_trivia(Rc::clone(&raw_sym_rpr), TextSpan::new(16, 1), Vec::new(), vec![SyntaxTrivia::Space(1)]));
        let con_sym_ast_1 = Rc::new(SyntaxToken::with_trivia(Rc::clone(&raw_sym_ast), TextSpan::new(18, 1), Vec::new(), vec![SyntaxTrivia::Space(1)]));
        let con_sym_lpr_2 = Rc::new(SyntaxToken::with       (Rc::clone(&raw_sym_lpr), TextSpan::new(20, 1)));
        let con_idn_foo_2 = Rc::new(SyntaxToken::with_trivia(Rc::clone(&raw_idn_foo), TextSpan::new(21, 3), Vec::new(), vec![SyntaxTrivia::Space(1)]));
        let con_sym_pls_2 = Rc::new(SyntaxToken::with_trivia(Rc::clone(&raw_sym_pls), TextSpan::new(25, 1), Vec::new(), vec![SyntaxTrivia::Space(1)]));
        let con_idn_bar_2 = Rc::new(SyntaxToken::with_trivia(Rc::clone(&raw_idn_bar), TextSpan::new(27, 3), Vec::new(), vec![SyntaxTrivia::Space(1)]));
        let con_sym_mns_2 = Rc::new(SyntaxToken::with_trivia(Rc::clone(&raw_sym_mns), TextSpan::new(31, 1), Vec::new(), vec![SyntaxTrivia::Space(1)]));
        let con_lit_2fl_2 = Rc::new(SyntaxToken::with       (Rc::clone(&raw_lit_2fl), TextSpan::new(33, 3)));
        let con_sym_rpr_2 = Rc::new(SyntaxToken::with_trivia(Rc::clone(&raw_sym_rpr), TextSpan::new(36, 1), Vec::new(), vec![SyntaxTrivia::Space(1)]));
        let con_sym_pls_3 = Rc::new(SyntaxToken::with_trivia(Rc::clone(&raw_sym_pls), TextSpan::new(38, 1), Vec::new(), vec![SyntaxTrivia::Space(1)]));
        let con_idn_foo_3 = Rc::new(SyntaxToken::with       (Rc::clone(&raw_idn_foo), TextSpan::new(40, 3)));

        // CON `(foo + bar - 2.0)` 1
        let con_grp_expr_idn_foo_sym_pls_idn_bar_sym_mns_lit_2fl_1 =
            Rc::new(SyntaxNode {
                raw: Rc::clone(&raw_grp_expr_idn_foo_sym_pls_idn_bar_sym_mns_lit_2fl),
                span: TextSpan::new(0, 17),
                children: vec![
                    Syntax::Token(Rc::clone(&con_sym_lpr_1)),
                    Syntax::Token(Rc::clone(&con_idn_foo_1)),
                    Syntax::Token(Rc::clone(&con_sym_pls_1)),
                    Syntax::Token(Rc::clone(&con_idn_bar_1)),
                    Syntax::Token(Rc::clone(&con_sym_mns_1)),
                    Syntax::Token(Rc::clone(&con_lit_2fl_1)),
                    Syntax::Token(Rc::clone(&con_sym_rpr_1)),
                ]
            });

        // CON `(foo + bar - 2.0)` 2
        let con_grp_expr_idn_foo_sym_pls_idn_bar_sym_mns_lit_2fl_2 =
            Rc::new(SyntaxNode {
                raw: Rc::clone(&raw_grp_expr_idn_foo_sym_pls_idn_bar_sym_mns_lit_2fl),
                span: TextSpan::new(20, 17),
                children: vec![
                    Syntax::Token(Rc::clone(&con_sym_lpr_2)),
                    Syntax::Token(Rc::clone(&con_idn_foo_2)),
                    Syntax::Token(Rc::clone(&con_sym_pls_2)),
                    Syntax::Token(Rc::clone(&con_idn_bar_2)),
                    Syntax::Token(Rc::clone(&con_sym_mns_2)),
                    Syntax::Token(Rc::clone(&con_lit_2fl_2)),
                    Syntax::Token(Rc::clone(&con_sym_rpr_2)),
                ]
            });

        // CON `_ * _`
        let con_bin_expr_grp_expr_sym_ast_grp_expr_1 =
            Rc::new(SyntaxNode {
                raw: Rc::clone(&raw_bin_expr_grp_expr_sym_ast_grp_expr),
                span: TextSpan::new(0, 37),
                children: vec![
                    Syntax::Node(Rc::clone(&con_grp_expr_idn_foo_sym_pls_idn_bar_sym_mns_lit_2fl_1)),
                    Syntax::Token(Rc::clone(&con_sym_ast_1)),
                    Syntax::Node(Rc::clone(&con_grp_expr_idn_foo_sym_pls_idn_bar_sym_mns_lit_2fl_2)),
                ]
            });

        // CON `_ + foo`
        let con_bin_expr_bin_expr_sym_pls_idn_foo_1 =
            Rc::new(SyntaxNode {
                // raw: Rc::clone(&raw_bin_expr_grp_expr_sym_ast_grp_expr),
                raw: Rc::clone(&raw_bin_expr_bin_expr_sym_pls_idn_foo),
                span: TextSpan::new(0, 43),
                children: vec![
                    Syntax::Node(Rc::clone(&con_bin_expr_grp_expr_sym_ast_grp_expr_1)),
                    Syntax::Token(Rc::clone(&con_sym_pls_3)),
                    Syntax::Token(Rc::clone(&con_idn_foo_3)),
                ]
            });

        let root = Syntax::Node(Rc::clone(&con_bin_expr_bin_expr_sym_pls_idn_foo_1));
        print_syntax(&root, 0);
    }

    #[test]
    #[rustfmt::skip]
    fn test_syntax_node_nested_expr_trivia() {
        // Test string:
        /*
        (  foo
        +bar -      2.0
        )*(
           foo
            + bar
          - 2.0) + foo

        */

        assert!(true);
    }
}
