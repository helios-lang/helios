pub mod node;
pub mod token;

use crate::source::TextSpan;
use crate::cache::TokenCache;
use node::*;
use std::rc::Rc;
use token::*;

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

macro_rules! create_make_token_function {
    ($kind:expr, $name:ident) => {
        pub fn $name<V1, V2>(
            cache: &mut TokenCache,
            leading_trivia: V1,
            trailing_trivia: V2,
        ) -> SyntaxToken
        where
            V1: Into<Option<Vec<SyntaxTrivia>>>,
            V2: Into<Option<Vec<SyntaxTrivia>>>,
        {
            let text = stringify!($kind).to_string().to_lowercase();
            let text_len = text.len();

            Self::__make_token(
                cache,
                $kind,
                text,
                TextSpan::new(0, text_len),
                leading_trivia.into().unwrap_or_default(),
                trailing_trivia.into().unwrap_or_default(),
            )
        }
    };
    (integer => $base:ident, $name:ident) => {
        create_make_token_function!(
            TokenKind::Literal(Literal::Integer(Base::$base)),
            $name
        );
    };
    (keyword => $keyword:ident, $name:ident) => {
        create_make_token_function!(
            TokenKind::Keyword(Keyword::$keyword),
            $name
        );
    };
    (symbol => $symbol:ident, $name:ident) => {
        create_make_token_function!(
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
                children.iter().map(|syntax| syntax.raw()).collect(),
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

    create_make_token_function!(keyword => Fun,     make_fun_keyword);
    create_make_token_function!(keyword => Struct,  make_struct_keyword);
    create_make_token_function!(keyword => Type,    make_type_keyword);

    create_make_token_function!(symbol  => Eq,      make_eq_symbol);
    create_make_token_function!(symbol  => LBrace,  make_lbrace_symbol);
    create_make_token_function!(symbol  => RBrace,  make_rbrace_symbol);
    create_make_token_function!(symbol  => LParen,  make_lparen_symbol);
    create_make_token_function!(symbol  => RParen,  make_rparen_symbol);

    create_make_node_function!(BinaryExpr, make_binary_expr,
        lhs: SyntaxNode,
        operator: SyntaxToken,
        rhs: SyntaxNode,
    );
    create_make_node_function!(LiteralExpr, make_literal_expr,
        literal: SyntaxToken,
    );

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
    fn test_syntax_factory() {
        let cache = &mut TokenCache::new();

        let fun_decl =
            SyntaxFactory::make_fun_decl(
                SyntaxFactory::make_fun_keyword(
                    cache,
                    None,
                    vec![SyntaxTrivia::Space(1)]
                ),
                SyntaxFactory::make_identifier(
                    cache,
                    "add",
                    None,
                    vec![SyntaxTrivia::Space(1)]
                ),
                SyntaxFactory::make_lparen_symbol(
                    cache,
                    None,
                    vec![SyntaxTrivia::Space(1)]
                ),
                SyntaxFactory::make_rparen_symbol(
                    cache,
                    None,
                    vec![SyntaxTrivia::Space(1)]
                ),
                SyntaxFactory::make_eq_symbol(
                    cache,
                    None,
                    vec![SyntaxTrivia::Space(1)]
                ),
                SyntaxFactory::make_binary_expr(
                    SyntaxFactory::make_literal_expr(
                        SyntaxFactory::make_literal(
                            cache,
                            Literal::Integer(Base::Decimal),
                            "1",
                            None,
                            vec![SyntaxTrivia::Space(1)],
                        )
                    ),
                    SyntaxFactory::make_eq_symbol(
                        cache,
                        None,
                        vec![SyntaxTrivia::Space(1)]
                    ),
                    SyntaxFactory::make_literal_expr(
                        SyntaxFactory::make_literal(
                            cache,
                            Literal::Integer(Base::Decimal),
                            "1",
                            None,
                            None,
                        )
                    ),
                ),
            );

        print_syntax(&fun_decl.into(), 0);
    }
}
