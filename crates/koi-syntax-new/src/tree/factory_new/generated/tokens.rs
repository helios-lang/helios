use super::*;

crate::make_token_constructor! {
    FunKeyword {}
}

impl ToSyntax for FunKeyword {
    fn to_syntax(&self, builder: &mut SyntaxBuilder) -> Syntax {
        let syntax = (|builder: &mut SyntaxBuilder| {
            Syntax::Token(
                Rc::new(SyntaxToken::with_trivia(
                    builder.cache.lookup(&"fun".to_string(), |text| {
                        Rc::new(RawSyntaxToken::with(
                            TokenKind::Symbol(Symbol::Eq),
                            text,
                        ))
                    }),
                    TextSpan::new(self.start, 3),
                    self.leading_trivia.0.clone(),
                    self.trailing_trivia.0.clone(),
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

// ---

crate::make_token_constructor! {
    EqualSymbol {}
}

impl ToSyntax for EqualSymbol {
    fn to_syntax(&self, builder: &mut SyntaxBuilder) -> Syntax {
        let syntax = (|builder: &mut SyntaxBuilder| {
            Syntax::Token(
                Rc::new(SyntaxToken::with_trivia(
                    builder.cache.lookup(&"=".to_string(), |text| {
                        Rc::new(RawSyntaxToken::with(
                            TokenKind::Symbol(Symbol::Eq),
                            text,
                        ))
                    }),
                    TextSpan::new(self.start, 1),
                    self.leading_trivia.0.clone(),
                    self.trailing_trivia.0.clone(),
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

// ---

crate::make_token_constructor! {
    Identifier {
        text: String,
    }
}

impl ToSyntax for Identifier {
    fn to_syntax(&self, builder: &mut SyntaxBuilder) -> Syntax {
        let syntax = (|builder: &mut SyntaxBuilder| {
            Syntax::Token(
                Rc::new(SyntaxToken::with_trivia(
                    builder.cache.lookup(&self.text.to_string(), |text| {
                        Rc::new(RawSyntaxToken::with(
                            TokenKind::Symbol(Symbol::Eq),
                            text,
                        ))
                    }),
                    TextSpan::new(self.start, self.text.len()),
                    self.leading_trivia.0.clone(),
                    self.trailing_trivia.0.clone(),
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

// ---

// #[test]
// fn test_token_equal_symbol() {
//     let mut arena = Arena::new();
//     let mut cache = TokenCache::new();
//     let mut builder = SyntaxBuilder::new(&mut arena, &mut cache);

//     let syntax = SF::make_function_declaration(None)
//         .fun_keyword(|parent| {
//             SF::make_fun_keyword(parent)
//                 .start(0)
//                 .trailing_trivia(SyntaxTrivia::Space(1))
//         })
//         .identifier(|parent| {
//             SF::make_identifier(parent)
//                 .start(4)
//                 .text("add")
//                 .trailing_trivia(SyntaxTrivia::Space(1))
//         })
//         .equal_symbol(|parent| {
//             SF::make_equal_symbol(parent)
//                 .start(5)
//         });

//     print_syntax(&syntax.to_syntax(&mut builder), 0);
// }
