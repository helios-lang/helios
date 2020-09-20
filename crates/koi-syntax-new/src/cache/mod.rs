pub mod node_cache;
pub mod token_cache;

pub use token_cache::TokenCache;
use std::borrow::Borrow;

pub trait Cache {
    /// The base type used to search through this cache.
    type Query;

    /// The result type expected to be constructed/returned when looking through
    /// the cache.
    type Output;

    /// Looks for an item in the cache with the given key.
    ///
    /// If the cache does not find a item with the given `Borrow<Cache::Query>`
    /// value (i.e. it isn't cached), the provided closure is invoked to cache a
    /// new `Cache::Output` before finally returning it.
    fn lookup<F, Q>(&mut self, query: &Q, default: F) -> Self::Output
    where
        F: FnOnce(&Self::Query) -> Self::Output,
        Q: Borrow<Self::Query>;

    /// Returns the number of elements present in the cache.
    fn len(&self) -> usize;
}
