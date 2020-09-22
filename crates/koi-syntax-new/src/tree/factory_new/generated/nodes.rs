use super::*;

crate::make_node_constructor! {
    FunctionDeclaration {
        fun_keyword: FunKeyword,
        identifier: Identifier,
        equal_symbol: EqualSymbol,
    }
}

impl ToSyntax for FunctionDeclaration {
    fn to_syntax(&self, builder: &mut SyntaxBuilder) -> Syntax {
        let syntax = (|builder: &mut SyntaxBuilder| {
            let fun_keyword = self.fun_keyword.clone().unwrap();
            let identifier = self.identifier.clone().unwrap();
            let equal_symbol = self.equal_symbol.clone().unwrap();

            // Expect identifier to be at least after where fun_keyword starts
            assert!(
                identifier.start > fun_keyword.start,
                "in FunctionDeclaration: Identifier must follow FunKeyword"
            );

            // Expect equal_symbol to be at least after where identifier starts
            assert!(
                equal_symbol.start > identifier.start,
                "in FunctionDeclaration: EqualSymbol must follow Identifier"
            );

            let raw_fun_keyword =
                builder.cache.lookup(&"fun".to_string(), |text| {
                    Rc::new(RawSyntaxToken::with(
                        TokenKind::Keyword(Keyword::Fun),
                        text
                    ))
                });

            let raw_identifier =
                builder.cache.lookup(&identifier.text, |text| {
                    Rc::new(RawSyntaxToken::with(
                        TokenKind::Identifier,
                        text
                    ))
                });

            Syntax::Node(
                Rc::new(SyntaxNode::with(
                    Rc::new(RawSyntaxNode::with(
                        NodeKind::FunDecl,
                        vec![
                            RawSyntax::Token(raw_fun_keyword.clone()),
                            RawSyntax::Token(raw_identifier.clone()),
                        ]
                    )),
                    vec![
                        fun_keyword.to_syntax(builder),
                        identifier.to_syntax(builder),
                        equal_symbol.to_syntax(builder),

                    ]
                ))
            )
        })(builder);

        let node = builder.arena.insert(syntax.clone());
        if let Some(parent) = self.parent {
            parent.add_child(builder.arena, node);
        }

        syntax
    }
}

#[test]
fn test_function_declaration() {
    let mut arena = Arena::new();
    let mut cache = TokenCache::new();
    let mut builder = SyntaxBuilder::new(&mut arena, &mut cache);

    let syntax = SF::make_function_declaration(None)
        .fun_keyword(|parent| {
            SF::make_fun_keyword(parent)
                .start(0)
                .trailing_trivia(SyntaxTrivia::Space(1))
        })
        .identifier(|parent| {
            SF::make_identifier(parent)
                .start(4)
                .text("add".to_string())
                .trailing_trivia(SyntaxTrivia::Space(1))
        })
        .equal_symbol(|parent| {
            SF::make_equal_symbol(parent)
                .start(5)
                .trailing_trivia(SyntaxTrivia::Space(1))
        });

    print_syntax(&syntax.to_syntax(&mut builder), 0);
}
