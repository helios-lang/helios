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

    // pub fn make_function_declaration(parent: Option<NodeId>) -> FunctionDeclaration {
    //     FunctionDeclaration::new(parent)
    // }

    // pub fn make_fun_keyword(parent: Option<NodeId>) -> FunKeyword {
    //     FunKeyword::new(parent)
    // }

    // pub fn make_identifier(parent: Option<NodeId>) -> Identifier {
    //     Identifier::new(parent)
    // }
}

// #[derive(Clone, Debug, Default, Eq, PartialEq)]
// pub struct FunctionDeclaration {
//     parent: Option<NodeId>,
//     fun_keyword: Option<FunKeyword>,
//     identifier: Option<Identifier>,
// }

// impl FunctionDeclaration {
//     pub fn new(parent: Option<NodeId>) -> Self {
//         Self { parent, ..Self::default() }
//     }

//     pub fn fun_keyword<F>(mut self, constructor: F) -> Self
//     where
//         F: FnOnce(Option<NodeId>) -> FunKeyword,
//     {
//         assert!(self.fun_keyword == None, "Already constructed fun_keyword");
//         self.fun_keyword = Some(constructor(self.parent));
//         self
//     }

//     pub fn identifier<F>(mut self, constructor: F) -> Self
//     where
//         F: FnOnce(Option<NodeId>) -> Identifier,
//     {
//         assert!(self.identifier == None, "Already constructed identifier");
//         self.identifier = Some(constructor(self.parent));
//         self
//     }
// }

// impl ToSyntax for FunctionDeclaration {
//     fn to_syntax(&self, builder: &mut SyntaxBuilder) -> Syntax {
//         let fun_keyword = self.fun_keyword.clone().unwrap();
//         let identifier = self.identifier.clone().unwrap();

//         // Expect identifier to be at least after where fun_keyword starts
//         assert!(
//             identifier.start > fun_keyword.start,
//             "in FunctionDeclaration: Identifier must follow FunKeyword"
//         );

//         let raw_fun_keyword = builder.cache.lookup(&"fun".to_string(), |text| {
//             Rc::new(RawSyntaxToken::with(TokenKind::Keyword(Keyword::Fun), text))
//         });

//         let raw_identifier = builder.cache.lookup(&identifier.text, |text| {
//             Rc::new(RawSyntaxToken::with(TokenKind::Identifier, text))
//         });

//         let syntax = Syntax::Node(
//             Rc::new(SyntaxNode::with(
//                 Rc::new(RawSyntaxNode::with(
//                     NodeKind::FunDecl,
//                     vec![
//                         RawSyntax::Token(raw_fun_keyword.clone()),
//                         RawSyntax::Token(raw_identifier.clone()),
//                     ]
//                 )),
//                 vec![
//                     fun_keyword.to_syntax(builder),
//                     identifier.to_syntax(builder),
//                 ]
//             ))
//         );


//         let function_declaration = builder.arena.insert(syntax.clone());
//         if let Some(parent) = self.parent {
//             parent.add_child(builder.arena, function_declaration);
//         }

//         syntax
//     }
// }

// #[derive(Clone, Debug, Default, Eq, PartialEq)]
// pub struct FunKeyword {
//     parent: Option<NodeId>,
//     start: usize,
//     leading_trivia: SyntaxTriviaList,
//     trailing_trivia: SyntaxTriviaList,
// }

// impl FunKeyword {
//     pub fn new(parent: Option<NodeId>) -> Self {
//         Self { parent, ..Self::default() }
//     }

//     pub fn start(mut self, start: usize) -> Self {
//         self.start = start;
//         self
//     }

//     pub fn leading_trivia<S>(mut self, leading_trivia: S) -> Self
//     where
//         S: Into<SyntaxTriviaList>,
//     {
//         self.leading_trivia = leading_trivia.into();
//         self
//     }

//     pub fn trailing_trivia<S>(mut self, trailing_trivia: S) -> Self
//     where
//         S: Into<SyntaxTriviaList>,
//     {
//         self.trailing_trivia = trailing_trivia.into();
//         self
//     }
// }

// impl ToSyntax for FunKeyword {
//     fn to_syntax(&self, builder: &mut SyntaxBuilder) -> Syntax {
//         let text = "fun".to_string();
//         let text_len = text.len();

//         let syntax = Syntax::Token(
//             Rc::new(SyntaxToken::with_trivia(
//                 builder.cache.lookup(&text, |text| {
//                     Rc::new(RawSyntaxToken::with(
//                         TokenKind::Keyword(Keyword::Fun),
//                         text,
//                     ))
//                 }),
//                 TextSpan::new(self.start, text_len),
//                 self.leading_trivia.0.clone(),
//                 self.trailing_trivia.0.clone(),
//             ))
//         );

//         let fun_keyword = builder.arena.insert(syntax.clone());
//         if let Some(parent) = self.parent {
//             parent.add_child(builder.arena, fun_keyword);
//         }

//         syntax
//     }
// }

// #[derive(Clone, Debug, Default, Eq, PartialEq)]
// pub struct Identifier {
//     parent: Option<NodeId>,
//     start: usize,
//     text: String,
//     leading_trivia: SyntaxTriviaList,
//     trailing_trivia: SyntaxTriviaList,
// }

// impl Identifier {
//     pub fn new(parent: Option<NodeId>) -> Self {
//         Self { parent, ..Self::default() }
//     }

//     pub fn start(mut self, start: usize) -> Self {
//         self.start = start;
//         self
//     }

//     pub fn text<S>(mut self, text: S) -> Self
//     where
//         S: Into<String>,
//     {
//         self.text = text.into();
//         self
//     }

//     pub fn leading_trivia<S>(mut self, leading_trivia: S) -> Self
//     where
//         S: Into<SyntaxTriviaList>,
//     {
//         self.leading_trivia = leading_trivia.into();
//         self
//     }

//     pub fn trailing_trivia<S>(mut self, trailing_trivia: S) -> Self
//     where
//         S: Into<SyntaxTriviaList>,
//     {
//         self.trailing_trivia = trailing_trivia.into();
//         self
//     }
// }

// impl ToSyntax for Identifier {
//     fn to_syntax(&self, builder: &mut SyntaxBuilder) -> Syntax {
//         let syntax = Syntax::Token(
//             Rc::new(SyntaxToken::with_trivia(
//                 builder.cache.lookup(&self.text, |_| {
//                     Rc::new(RawSyntaxToken::with(
//                         TokenKind::Keyword(Keyword::Fun),
//                         self.text.clone(),
//                     ))
//                 }),
//                 TextSpan::new(self.start, self.text.len()),
//                 self.leading_trivia.0.clone(),
//                 self.trailing_trivia.0.clone(),
//             ))
//         );

//         let identifier = builder.arena.insert(syntax.clone());
//         if let Some(parent) = self.parent {
//             parent.add_child(builder.arena, identifier);
//         }

//         syntax
//     }
// }

// #[test]
// fn test_builder() {
//     let arena = &mut Arena::new();
//     let cache = &mut TokenCache::new();

//     let fun_decl =
//         SF::build_syntax(arena, cache, || {
//             SF::make_function_declaration(None)
//                 .fun_keyword(|parent| {
//                     SF::make_fun_keyword(parent)
//                         .start(0)
//                         .trailing_trivia(SyntaxTrivia::Space(1))
//                 })
//                 .identifier(|parent| {
//                     SF::make_identifier(parent)
//                         .start(4)
//                         .text("add")
//                         .trailing_trivia(SyntaxTrivia::Space(1))
//                 })
//         });

//     print_syntax(&fun_decl, 0);
//     println!("{}", fun_decl.full_span())
// }
