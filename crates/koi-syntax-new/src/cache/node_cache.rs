use crate::cache::Cache;
use crate::tree::RawSyntax;
use crate::tree::node::RawSyntaxNode;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Eq, Hash)]
struct NodeCacheKey(Rc<RawSyntaxNode>);

impl PartialEq for NodeCacheKey {
    fn eq(&self, other: &Self) -> bool {
        self.0.children.eq(&other.0.children)
    }
}

impl Borrow<Vec<RawSyntax>> for NodeCacheKey {
    fn borrow(&self) -> &Vec<RawSyntax> {
        &self.0.children
    }
}

impl Borrow<String> for NodeCacheKey {
    fn borrow(&self) -> &String {
        &self.0.combined_text_value()
    }
}

pub struct NodeCache(HashMap<String, NodeCacheKey>);

impl NodeCache {
    /// Creates an empty `NodeCache`.
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}

impl Cache for NodeCache {
    type Query = String;
    type Output = Rc<RawSyntaxNode>;

    fn lookup<F, Q>(&mut self, query: &Q, default: F) -> Self::Output
    where
        F: FnOnce(&Self::Query) -> Self::Output,
        Q: Borrow<Self::Query>
    {
        let entry =
            self.0.entry(query.borrow().to_owned()).or_insert_with(|| {
                NodeCacheKey(default(query.borrow()))
            });

        Rc::clone(&entry.0)
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}
