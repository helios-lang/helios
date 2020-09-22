mod generated;

use crate::cache::{Cache, TokenCache};
use crate::source::TextSpan;
use crate::tree::{RawSyntax, Syntax};
use crate::tree::node::*;
use crate::tree::token::*;
use koi_arena::{Arena, NodeId};
use std::rc::Rc;

pub use generated::*;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SyntaxTriviaList(Vec<SyntaxTrivia>);

impl From<SyntaxTrivia> for SyntaxTriviaList {
    fn from(trivia: SyntaxTrivia) -> Self {
        Self(vec![trivia])
    }
}

impl From<Vec<SyntaxTrivia>> for SyntaxTriviaList {
    fn from(trivia_list: Vec<SyntaxTrivia>) -> Self {
        Self(trivia_list)
    }
}

pub type SF = SyntaxFactory;

pub trait ToSyntax {
    fn to_syntax(&self, builder: &mut SyntaxBuilder) -> Syntax;
}

pub struct SyntaxBuilder<'arena, 'cache> {
    arena: &'arena mut Arena<Syntax>,
    cache: &'cache mut TokenCache,
}

impl<'arena, 'cache> SyntaxBuilder<'arena, 'cache> {
    pub fn new(arena: &'arena mut Arena<Syntax>, cache: &'cache mut TokenCache) -> Self {
        Self { arena, cache }
    }
}

pub struct SyntaxFactory;

impl SyntaxFactory {
    pub fn build_syntax<F, S>(arena: &mut Arena<Syntax>, cache: &mut TokenCache, constructor: F) -> Syntax
    where
        F: FnOnce() -> S,
        S: ToSyntax,
    {
        let mut builder = SyntaxBuilder::new(arena, cache);
        let syntax = constructor().to_syntax(&mut builder);
        arena.insert(syntax.clone());
        syntax
    }
}

#[test]
fn test_builder() {
    let arena = &mut Arena::new();
    let cache = &mut TokenCache::new();

    // fun add() = ???
    let fun_decl =
        SF::build_syntax(arena, cache, || {
            SF::make_function_declaration(None)
                .fun_keyword(|parent| {
                    SF::make_fun_keyword(parent)
                        .start(0)
                        .trailing_trivia(SyntaxTrivia::Space(1))
                })
                .identifier(|parent| {
                    SF::make_identifier(parent)
                        .start(4)
                        .text("add".to_string())
                })
                .lparen_symbol(|parent| {
                    SF::make_lparen_symbol(parent)
                        .start(7)
                })
                .rparen_symbol(|parent| {
                    SF::make_rparen_symbol(parent)
                        .start(8)
                        .trailing_trivia(SyntaxTrivia::Space(1))
                })
                .equal_symbol(|parent| {
                    SF::make_equal_symbol(parent)
                        .start(10)
                        .trailing_trivia(SyntaxTrivia::Space(1))
                })
        });

    print_syntax(&fun_decl, 0);
    println!("{:#?}", arena);
}
