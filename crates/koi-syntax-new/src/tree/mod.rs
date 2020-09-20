pub mod node;
pub mod token;

use crate::source::TextSpan;
use crate::cache::{Cache, TokenCache};
use node::*;
use token::*;
use std::rc::Rc;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum RawSyntax {
    Node(Rc<RawSyntaxNode>),
    Token(Rc<RawSyntaxToken>),
}

impl RawSyntax {
    fn combined_text_value(&self) -> &String {
        match self {
            Self::Node(node) => &node.combined_text_value(),
            Self::Token(token) => &token.text,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Syntax {
    Node(Rc<SyntaxNode>),
    Token(Rc<SyntaxToken>),
}

impl Syntax {
    pub fn raw(&self) -> RawSyntax {
        match self {
            Self::Node(node) => RawSyntax::Node(Rc::clone(&node.raw)),
            Self::Token(token) => RawSyntax::Token(Rc::clone(&token.raw)),
        }
    }

    pub fn children(&self) -> Box<dyn Iterator<Item=&Syntax> + '_> {
        match self {
            Self::Node(node) => Box::new(node.children()),
            Self::Token(_) => Box::new(std::iter::empty()),
        }
    }
}

impl From<SyntaxNode> for Syntax {
    fn from(node: SyntaxNode) -> Self {
        Self::Node(Rc::new(node))
    }
}

impl From<SyntaxToken> for Syntax {
    fn from(token: SyntaxToken) -> Self {
        Self::Token(Rc::new(token))
    }
}

pub struct SyntaxFactory;

macro_rules! make_token_fn {
    ($text:expr, $kind:expr, $name:ident) => {
        pub fn $name<V1, V2>(
            cache: &mut TokenCache,
            leading_trivia: V1,
            trailing_trivia: V2,
        ) -> SyntaxToken
        where
            V1: Into<Option<Vec<SyntaxTrivia>>>,
            V2: Into<Option<Vec<SyntaxTrivia>>>,
        {
            Self::__make_token(
                cache,
                $kind,
                $text.to_string(),
                TextSpan::new(0, $text.len()),
                leading_trivia.into().unwrap_or_default(),
                trailing_trivia.into().unwrap_or_default(),
            )
        }
    };
    (integer => $text:expr, $base:ident, $name:ident) => {
        make_token_fn!(
            $text,
            TokenKind::Literal(Literal::Integer(Base::$base)),
            $name
        );
    };
    (keyword => $text:expr, $keyword:ident, $name:ident) => {
        make_token_fn!(
            $text,
            TokenKind::Keyword(Keyword::$keyword),
            $name
        );
    };
    (symbol => $text:expr, $symbol:ident, $name:ident) => {
        make_token_fn!(
            $text,
            TokenKind::Symbol(Symbol::$symbol),
            $name
        );
    };
}

macro_rules! create_make_node_function {
    ($kind:ident, $name:ident, $( $member:ident : $member_type:ty ),* $(,)?) => {
        pub fn $name($( $member : $member_type ),*) -> SyntaxNode {
            Self::__make_node(
                NodeKind::$kind,
                vec![ $( $member.into(), )* ],
            )
        }
    };
}

impl SyntaxFactory {
    fn __make_token(
        cache: &mut TokenCache,
        kind: TokenKind,
        text: String,
        span: TextSpan,
        leading_trivia: Vec<SyntaxTrivia>,
        trailing_trivia: Vec<SyntaxTrivia>,
    ) -> SyntaxToken {
        SyntaxToken::with_trivia(
            cache.lookup(&text, |text| {
                Rc::new(RawSyntaxToken::with(kind, text))
            }),
            span,
            leading_trivia,
            trailing_trivia,
        )
    }

    fn __make_node(kind: NodeKind, children: Vec<Syntax>) -> SyntaxNode {
        SyntaxNode::with(
            Rc::new(RawSyntaxNode::with(
                kind,
                children.iter().map(|syntax| syntax.raw()).collect::<Vec<_>>(),
            )),
            children
        )
    }

    pub fn make_identifier<S, V1, V2>(
        cache: &mut TokenCache,
        identifier: S,
        leading_trivia: V1,
        trailing_trivia: V2,
    ) -> SyntaxToken
    where
        S: Into<String>,
        V1: Into<Option<Vec<SyntaxTrivia>>>,
        V2: Into<Option<Vec<SyntaxTrivia>>>,
    {
        let identifier = identifier.into();
        let identifier_len = identifier.len();

        Self::__make_token(
            cache,
            TokenKind::Identifier,
            identifier,
            TextSpan::new(0, identifier_len),
            leading_trivia.into().unwrap_or_default(),
            trailing_trivia.into().unwrap_or_default(),
        )
    }

    pub fn make_literal<S, V1, V2>(
        cache: &mut TokenCache,
        kind: Literal,
        literal: S,
        leading_trivia: V1,
        trailing_trivia: V2,
    ) -> SyntaxToken
    where
        S: Into<String>,
        V1: Into<Option<Vec<SyntaxTrivia>>>,
        V2: Into<Option<Vec<SyntaxTrivia>>>,
    {
        let literal = literal.into();
        let literal_len = literal.len();

        Self::__make_token(
            cache,
            TokenKind::Literal(kind),
            literal,
            TextSpan::new(0, literal_len),
            leading_trivia.into().unwrap_or_default(),
            trailing_trivia.into().unwrap_or_default(),
        )
    }

    // -- TOKENS --

    // KEYWORDS
    make_token_fn!(keyword => "fun",    Fun,     make_fun_keyword);
    make_token_fn!(keyword => "struct", Struct,  make_struct_keyword);
    make_token_fn!(keyword => "type",   Type,    make_type_keyword);

    // SYMBOLS
    make_token_fn!(symbol  => "*",      Asterisk,make_asterisk_symbol);
    make_token_fn!(symbol  => "=",      Eq,      make_eq_symbol);
    make_token_fn!(symbol  => "-",      Minus,   make_minus_symbol);
    make_token_fn!(symbol  => "+",      Plus,    make_plus_symbol);
    make_token_fn!(symbol  => "{",      LBrace,  make_lbrace_symbol);
    make_token_fn!(symbol  => "}",      RBrace,  make_rbrace_symbol);
    make_token_fn!(symbol  => "(",      LParen,  make_lparen_symbol);
    make_token_fn!(symbol  => ")",      RParen,  make_rparen_symbol);

    // -- NODES --

    // EXPRESSIONS
    create_make_node_function!(BinaryExpr, make_binary_expr,
        lhs: SyntaxNode,
        operator: SyntaxToken,
        rhs: SyntaxNode,
    );
    create_make_node_function!(GroupedExpr, make_grouped_expr,
        lparen: SyntaxToken,
        expression: SyntaxNode,
        rparen: SyntaxToken,
    );
    create_make_node_function!(LiteralExpr, make_literal_expr,
        literal: SyntaxToken,
    );
    create_make_node_function!(UnaryExpr, make_unary_expr,
        operator: SyntaxToken,
        lhs: SyntaxNode,
    );

    // DECLARATIONS
    create_make_node_function!(FunDecl, make_fun_decl,
        fun_keyword: SyntaxToken,
        fun_identifier: SyntaxToken,
        lparen_symbol: SyntaxToken,
        rparen_symbol: SyntaxToken,
        eq_symbol: SyntaxToken,
        fun_body: SyntaxNode,
    );
    create_make_node_function!(StructDecl, make_struct_decl,
        type_keyword: SyntaxToken,
        type_identifier: SyntaxToken,
        eq_symbol: SyntaxToken,
        struct_keyword: SyntaxToken,
        lbrace_symbol: SyntaxToken,
        rbrace_symbol: SyntaxToken,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn test_syntax_factory_nested_binary_expression() {
        let cache = &mut TokenCache::new();

        // (5 * 5) + (5 * 5)
        let expr =
            SyntaxFactory::make_binary_expr(
                SyntaxFactory::make_grouped_expr(
                    SyntaxFactory::make_lparen_symbol(cache, None, None),
                    SyntaxFactory::make_binary_expr(
                        SyntaxFactory::make_literal_expr(
                            SyntaxFactory::make_literal(
                                cache,
                                Literal::Integer(Base::Decimal),
                                "5",
                                None,
                                None
                            ),
                        ),
                        SyntaxFactory::make_asterisk_symbol(cache, None, None),
                        SyntaxFactory::make_literal_expr(
                            SyntaxFactory::make_literal(
                                cache,
                                Literal::Integer(Base::Decimal),
                                "5",
                                None,
                                None
                            ),
                        ),
                    ),
                    SyntaxFactory::make_rparen_symbol(cache, None, None),
                ),
                SyntaxFactory::make_plus_symbol(cache, None, None),
                SyntaxFactory::make_grouped_expr(
                    SyntaxFactory::make_lparen_symbol(cache, None, None),
                    SyntaxFactory::make_binary_expr(
                        SyntaxFactory::make_literal_expr(
                            SyntaxFactory::make_literal(
                                cache,
                                Literal::Integer(Base::Decimal),
                                "5",
                                None,
                                None
                            ),
                        ),
                        SyntaxFactory::make_asterisk_symbol(cache, None, None),
                        SyntaxFactory::make_literal_expr(
                            SyntaxFactory::make_literal(
                                cache,
                                Literal::Integer(Base::Decimal),
                                "5",
                                None,
                                None
                            ),
                        ),
                    ),
                    SyntaxFactory::make_rparen_symbol(cache, None, None),
                ),
            );

        let root = Syntax::Node(Rc::new(expr));
        print_syntax(&root, 0);
    }

    #[test]
    #[rustfmt::skip]
    fn test_syntax_factory_function_declaration() {
        let cache = &mut TokenCache::new();

        let fun_decl = SyntaxFactory::make_fun_decl(
            SyntaxFactory::make_fun_keyword(cache, None, vec![SyntaxTrivia::Space(1)]),
            SyntaxFactory::make_identifier(cache, "add", None, vec![SyntaxTrivia::Space(1)]),
            SyntaxFactory::make_lparen_symbol(cache, None, vec![SyntaxTrivia::Space(1)]),
            SyntaxFactory::make_rparen_symbol(cache, None, vec![SyntaxTrivia::Space(1)]),
            SyntaxFactory::make_eq_symbol(cache, None, vec![SyntaxTrivia::Space(1)]),
            SyntaxFactory::make_binary_expr(
                SyntaxFactory::make_literal_expr(
                    SyntaxFactory::make_literal(cache, Literal::Integer(Base::Decimal), "1", None, vec![SyntaxTrivia::Space(1)]),
                ),
                SyntaxFactory::make_eq_symbol(cache, None, vec![SyntaxTrivia::Space(1)]),
                SyntaxFactory::make_literal_expr(
                    SyntaxFactory::make_literal(cache, Literal::Integer(Base::Decimal), "1", None, None),
                ),
            ),
        );

        let root = Syntax::Node(Rc::new(fun_decl));
        print_syntax(&root, 0);
    }
}
