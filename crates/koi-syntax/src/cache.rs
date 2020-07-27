#![allow(dead_code)]

use std::collections::HashMap;
use std::hash::Hash;

#[macro_export]
macro_rules! cache {
    ( $( $key:expr => $value:expr ),* ) => {
        {
            use crate::cache::Cache;
            let mut cache = Cache::new();
            $(
                cache.insert($key, $value);
            )*
            cache
        }
    };
}

#[derive(Debug)]
pub struct Cache<K, V>(HashMap<K, V>);

impl<K, V> Cache<K, V> where K: Hash + Eq + Clone {
    /// Creates an empty `Cache`.
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Inserts a key-value pair into the cache.
    ///
    /// This method just calls the underlying hash map's `HashMap::insert`
    /// method with the given arguments. Refer to its documentation for more
    /// information.
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.0.insert(key, value)
    }

    /// Returns a reference to the value corresponding to the key.
    ///
    /// This method just calls the underlying hash map's `HashMap::get` method
    /// with the given arguments. Refer to its documentation for more
    /// information.
    pub fn get(&self, key: &K) -> Option<&V> {
        self.0.get(&key)
    }

    /// Looks up a value with the given key and, if found, returns the value,
    /// otherwise inserts a new value (the provided default value) at the
    /// given key before returning it.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use koi_syntax::cache::Cache;
    ///
    /// let mut cache = Cache::new();
    /// cache.insert("one".to_string(), 1);
    /// cache.insert("two".to_string(), 2);
    ///
    /// // The key `"ten"` does not exist.
    /// assert_eq!(cache.get(&"ten".to_string()), None);
    ///
    /// // Lookup an existing key-value pair.
    /// assert_eq!(cache.lookup("one".to_string(), 1), &1);
    ///
    /// // Lookup a non-existing key-value pair.
    /// assert_eq!(cache.lookup("ten".to_string(), 10), &10);
    ///
    /// // The new key-value pair is now in the cache.
    /// assert_eq!(cache.get(&"ten".to_string()), Some(&10));
    /// ```
    pub fn lookup(&mut self, key: K, default: V) -> &V {
        let value = self.0.entry(key.clone()).or_insert(default);
        value
    }

    /// Looks up a value with the given key and, if found, returns the value,
    /// otherwise inserts a new value (by calling the provided function) at the
    /// given key before returning it.
    ///
    /// # Discussion
    ///
    /// It is recommended to use this method if the new value is expensive to
    /// compute. The closure will not be evaluated until after it has been
    /// determined that the key does not exist. Otherwise, use `Cache::lookup`
    /// to pass the new value directly.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use koi_syntax::cache::Cache;
    ///
    /// let mut cache = Cache::new();
    /// cache.insert("one".to_string(), 1);
    /// cache.insert("two".to_string(), 2);
    ///
    /// // The key `"ten"` does not exist.
    /// assert_eq!(cache.get(&"ten".to_string()), None);
    ///
    /// // Lookup an existing key-value pair.
    /// assert_eq!(cache.lookup_with("one".to_string(), || 1), &1);
    ///
    /// // Lookup a non-existing key-value pair.
    /// assert_eq!(cache.lookup_with("ten".to_string(), || 10), &10);
    ///
    /// // The new key-value pair is now in the cache.
    /// assert_eq!(cache.get(&"ten".to_string()), Some(&10));
    /// ```
    pub fn lookup_with<F: FnOnce() -> V>(&mut self, key: K, create_value: F) -> &V {
        let value = self.0.entry(key.clone()).or_insert_with(|| create_value());
        value
    }

    /// Returns the number of elements in the cache.
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_cache_lookup() {
        // Create a cache with two entries.
        let mut cache = cache!["one" => 1, "two" => 2];
        assert_eq!(cache.len(), 2);

        // Lookup an existing key-value pair.
        assert_eq!(cache.lookup("one", 1), &1);

        // Lookup a non-existing key-value pair.
        assert_eq!(cache.lookup("ten", 10), &10);

        // The new key-value pair is now in the cache.
        assert_eq!(cache.get(&"ten"), Some(&10));
        assert_eq!(cache.len(), 3);
    }

    #[test]
    fn test_cache_lookup_with() {
        // Create a cache with two entries.
        let mut cache = cache!["one" => 1, "two" => 2];
        assert_eq!(cache.len(), 2);

        // Lookup an existing key-value pair.
        assert_eq!(cache.lookup_with("one", || 1), &1);

        // Lookup a non-existing key-value pair.
        assert_eq!(cache.lookup_with("ten", || 10), &10);

        // The new key-value pair is now in the cache.
        assert_eq!(cache.get(&"ten"), Some(&10));
        assert_eq!(cache.len(), 3);
    }
}
