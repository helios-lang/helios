use crate::tree::token::*;
use crate::cache::Cache;
use std::borrow::Borrow;
use std::collections::HashSet;
use std::rc::Rc;

#[derive(Debug, Eq, Hash)]
struct TokenCacheKey(Rc<RawSyntaxToken>);

impl PartialEq for TokenCacheKey {
    fn eq(&self, other: &Self) -> bool {
        self.0.text.eq(&other.0.text)
    }
}

impl Borrow<String> for TokenCacheKey {
    fn borrow(&self) -> &String {
        &self.0.text
    }
}

pub struct TokenCache(HashSet<TokenCacheKey>);

impl TokenCache {
    /// Creates an empty `TokenCache`.
    pub fn new() -> Self {
        Self(HashSet::new())
    }
}

impl Cache for TokenCache {
    type Query = String;
    type Output = Rc<RawSyntaxToken>;

    fn lookup<F, Q>(&mut self, query: &Q, default: F) -> Self::Output
    where
        F: FnOnce(&Self::Query) -> Self::Output,
        Q: Borrow<Self::Query>
    {
        if !self.0.contains(query.borrow()) {
            self.0.insert(TokenCacheKey(default(query.borrow())));
        }

        Rc::clone(&self.0.get(query.borrow()).unwrap().0)
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
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
}
