use crate::tree::token::*;
use std::borrow::Borrow;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

#[derive(Debug, Eq)]
pub struct TokenCacheKey(Rc<RawSyntaxToken>);

impl PartialEq for TokenCacheKey {
    fn eq(&self, other: &Self) -> bool {
        self.0.text.eq(&other.0.text)
    }
}

impl Hash for TokenCacheKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

impl Borrow<String> for TokenCacheKey {
    fn borrow(&self) -> &String {
        &self.0.text
    }
}

pub struct TokenCache(HashSet<TokenCacheKey>);

impl TokenCache {
    pub fn new() -> Self {
        Self(HashSet::new())
    }

    pub fn lookup<F, Q>(&mut self, query: &Q, default: F) -> Rc<RawSyntaxToken>
    where
        F: FnOnce(&String) -> Rc<RawSyntaxToken>,
        Q: Borrow<String>,
    {
        if !self.0.contains(query.borrow()) {
            self.0.insert(TokenCacheKey(default(query.borrow())));
        }

        self.0.get(query.borrow()).unwrap().0.clone()
    }

    /// Returns the number of elements in the token cache.
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[test]
fn test_token_cache() {
    let mut cache = TokenCache::new();

    macro_rules! lookup_eq {
        ($query:expr, $kind:expr) => {{
            assert_eq!(
                $kind,
                cache.lookup(&$query.to_string(), |text| {
                    Rc::new(RawSyntaxToken::with($kind, text))
                }).kind,
            );
        }};
    }

    // let x = 10
    lookup_eq!("let", TokenKind::Keyword(Keyword::Let));
    lookup_eq!("x",   TokenKind::Identifier);
    lookup_eq!("=",   TokenKind::Symbol(Symbol::Eq));
    lookup_eq!("10",  TokenKind::Literal(Literal::Integer(Base::Decimal)));

    // let y = 10
    lookup_eq!("let", TokenKind::Keyword(Keyword::Let));
    lookup_eq!("y",   TokenKind::Identifier);
    lookup_eq!("=",   TokenKind::Symbol(Symbol::Eq));
    lookup_eq!("x",   TokenKind::Identifier);

    assert_eq!(cache.len(), 5);
}
