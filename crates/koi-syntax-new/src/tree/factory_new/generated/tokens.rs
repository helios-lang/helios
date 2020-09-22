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
                            TokenKind::Keyword(Keyword::Fun),
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
    LparenSymbol {}
}

impl ToSyntax for LparenSymbol {
    fn to_syntax(&self, builder: &mut SyntaxBuilder) -> Syntax {
        let syntax =
            Syntax::Token(
                Rc::new(SyntaxToken::with_trivia(
                    builder.cache.lookup(&"(".to_string(), |text| {
                        Rc::new(RawSyntaxToken::with(
                            TokenKind::Symbol(Symbol::LParen),
                            text,
                        ))
                    }),
                    TextSpan::new(self.start, 1),
                    self.leading_trivia.0.clone(),
                    self.trailing_trivia.0.clone(),
                ))
            );

        let node = builder.arena.insert(syntax.clone());
        if let Some(parent) = self.parent {
            parent.add_child(builder.arena, node);
        }

        syntax
    }
}

// ---

crate::make_token_constructor! {
    RparenSymbol {}
}

impl ToSyntax for RparenSymbol {
    fn to_syntax(&self, builder: &mut SyntaxBuilder) -> Syntax {
        let syntax = (|builder: &mut SyntaxBuilder| {
            Syntax::Token(
                Rc::new(SyntaxToken::with_trivia(
                    builder.cache.lookup(&")".to_string(), |text| {
                        Rc::new(RawSyntaxToken::with(
                            TokenKind::Symbol(Symbol::RParen),
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
                            TokenKind::Identifier,
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
