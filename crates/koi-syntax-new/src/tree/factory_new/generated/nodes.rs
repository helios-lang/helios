use super::*;

crate::make_node_constructor! {
    FunctionDeclaration {
        function_keyword: FunctionKeyword,
        identifier: Identifier,
        lparen_symbol: LparenSymbol,
        rparen_symbol: RparenSymbol,
        equal_symbol: EqualSymbol,
    }
}

impl ToSyntax for FunctionDeclaration {
    fn to_syntax(&self, builder: &mut SyntaxBuilder) -> Syntax {
        let syntax = (|builder: &mut SyntaxBuilder| {
            fn missing(member: &str) -> String {
                format!("in FunctionDeclaration: {} is not constructed", member)
            }

            fn misplaced(fst: &str, snd: &str) -> String {
                format!("in FunctionDeclaration: {} must follow {}", fst, snd)
            }

            let (
                fun_keyword,
                identifier,
                lparen_symbol,
                rparen_symbol,
                equal_symbol,
            ) = (
                self.function_keyword
                    .clone()
                    .expect(missing("FunctionKeyword").as_str()),
                self.identifier
                    .clone()
                    .expect(missing("Identifier").as_str()),
                self.lparen_symbol
                    .clone()
                    .expect(missing("LparenSymbol").as_str()),
                self.rparen_symbol
                    .clone()
                    .expect(missing("RparenSymbol").as_str()),
                self.equal_symbol
                    .clone()
                    .expect(missing("EqualSymbol").as_str()),
            );

            assert!(
                identifier.start > fun_keyword.start,
                misplaced("Identifier", "FunctionKeyword")
            );
            assert!(
                lparen_symbol.start > identifier.start,
                misplaced("LParenSymbol", "Identifier")
            );
            assert!(
                rparen_symbol.start > lparen_symbol.start,
                misplaced("RParenSymbol", "LParenSymbol")
            );
            assert!(
                equal_symbol.start > rparen_symbol.start,
                misplaced("EqualSymbol", "RParenSymbol")
            );

            let raw_fun_keyword =
                builder.cache.lookup(&"function".to_string(), |text| {
                    Rc::new(RawSyntaxToken::with(
                        TokenKind::Keyword(Keyword::Function),
                        text,
                    ))
                });

            let raw_identifier =
                builder.cache.lookup(&identifier.text, |text| {
                    Rc::new(RawSyntaxToken::with(TokenKind::Identifier, text))
                });

            Syntax::Node(Rc::new(SyntaxNode::with(
                Rc::new(RawSyntaxNode::with(
                    NodeKind::FunctionDecl,
                    vec![
                        RawSyntax::Token(raw_fun_keyword.clone()),
                        RawSyntax::Token(raw_identifier.clone()),
                    ],
                )),
                vec![
                    fun_keyword.to_syntax(builder),
                    identifier.to_syntax(builder),
                    lparen_symbol.to_syntax(builder),
                    rparen_symbol.to_syntax(builder),
                    equal_symbol.to_syntax(builder),
                ],
            )))
        })(builder);

        let node = builder.arena.insert(syntax.clone());
        if let Some(parent) = self.parent {
            parent.add_child(builder.arena, node);
        }

        syntax
    }
}
