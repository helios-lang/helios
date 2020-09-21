#[derive(Debug, Eq, PartialEq)]
pub struct Arena<T> {
    nodes: Vec<Node<T>>,
    root: Option<NodeId>,
}

impl<T> Arena<T> {
    /// Creates a new empty `Arena`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the number of nodes currently allocated in the `Arena` instance.
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Returns the root `NodeId` index of the `Arena`.
    pub fn root(&self) -> Option<NodeId> {
        self.root
    }

    /// Sets a new root for the `Arena`.
    pub fn set_root<OptionalId>(&mut self, root: OptionalId)
    where
        OptionalId: Into<Option<NodeId>>,
    {
        self.root = root.into()
    }

    /// Returns a reference to a `Node` at the given `NodeId` index.
    fn get(&self, id: NodeId) -> &Node<T> {
        &self.nodes[id.index()]
    }

    /// Returns a mutable reference to a `Node` at the given `NodeId` index.
    fn get_mut(&mut self, id: NodeId) -> &mut Node<T> {
        &mut self.nodes[id.index()]
    }

    /// Retrieves an optional reference to a `Node` at the given `NodeId` index.
    pub fn node_at(&self, id: NodeId) -> Option<&Node<T>> {
        self.nodes.get(id.index())
    }

    /// Returns a new vector of all the nodes present in the `Arena`.
    pub fn nodes(&self) -> Vec<Node<T>>
    where
        T: Clone,
    {
        self.nodes.clone()
    }

    /// Inserts a new value into the arena and returns its `NodeId` index.
    ///
    /// New data will be appended to the end of the arena's internal vector.
    /// If this is the first time a value is inserted into the `Arena`, its
    /// index will be set as the new root.
    pub fn insert(&mut self, data: T) -> NodeId {
        let index = self.nodes.len();
        self.nodes.push(Node::new(index, data));

        // Set new node as root if this is our first time inserting
        if index == 0 {
            self.root = Some(NodeId(index));
        }

        NodeId(index)
    }
}

impl<T> Default for Arena<T> {
    fn default() -> Self {
        Self { nodes: Vec::new(), root: None }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Node<T> {
    id: NodeId,
    data: T,
    parent: Option<NodeId>,
    children: Vec<NodeId>,
}

impl<T> Node<T> {
    /// Creates a new `Node` with the given index and data.
    pub fn new<Id>(id: Id, data: T) -> Self
    where
        Id: Into<NodeId>,
    {
        Self::with(id, data, None, Vec::new())
    }

    /// Creates a new `Node` with the given index, data, optional parent, and
    /// children.
    pub fn with<Id, P>(id: Id, data: T, parent: P, children: Vec<Id>) -> Self
    where
        Id: Into<NodeId>,
         P: Into<Option<NodeId>>,
    {
        Self {
            id: id.into(),
            data,
            parent: parent.into(),
            children: children.into_iter().map(Id::into).collect()
        }
    }

    /// Returns the `NodeId` identifier of the node.
    ///
    /// This value is used to uniquely identify a node in an `Arena`. It holds
    /// a `usize` that represents its position in the `Arena`.
    pub fn id(&self) -> NodeId {
        self.id
    }

    /// Returns a reference to the data contained in the node.
    pub fn data(&self) -> &T {
        &self.data
    }

    /// Returns the `NodeId` identifier of this node's parent.
    ///
    /// Because a `Node` does not need to have a parent, this function returns
    /// an `Option<NodeId>`.
    pub fn parent(&self) -> Option<NodeId> {
        self.parent
    }

    /// Sets the parent of this node.
    pub fn set_parent<OptionalId>(&mut self, new_parent: OptionalId)
    where
        OptionalId: Into<Option<NodeId>>,
    {
        self.parent = new_parent.into()
    }

    pub fn children(&self) -> &Vec<NodeId> {
        &self.children
    }

    pub fn add_child(&mut self, child: NodeId) {
        self.children.push(child)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NodeId(usize);

impl NodeId {
    /// Returns the index value stored in this `NodeId` instance.
    pub fn index(&self) -> usize {
        self.0
    }

    /// Retrieves the parent of the `Node` with this `NodeId`.
    pub fn parent<T>(&self, arena: &Arena<T>) -> Option<NodeId>
    where
        T: PartialEq,
    {
        arena.get(*self).parent()
    }

    /// Retrieves a reference to the children of the `Node` with this `NodeId`.
    pub fn children<'a, T>(&self, arena: &'a Arena<T>) -> &'a Vec<NodeId>
    where
        T: PartialEq,
    {
        arena.get(*self).children()
    }

    /// Sets a new parent for this `Node`.
    ///
    /// It is not recommended to directly call this method as child relations
    /// will not be made automatically; you will need to add them yourself. As
    /// a result, it is recommended to call `Node::add_child` instead, since
    /// it will automatically handle that for you.
    ///
    /// This method returns a reference to itself to allow convenient chaining
    /// of methods.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use crate::isaac::{Arena, NodeId};
    /// let mut arena = &mut Arena::new();
    /// let root_node = arena.insert("1");
    /// let second_node = arena.insert("2");
    ///
    /// // Make `root_node` the parent of `second_node`.
    /// second_node.set_parent(arena, root_node);
    /// assert_eq!(second_node.parent(arena), Some(NodeId::from(0)));
    ///
    /// // Note that this does not add `second_node` as a child to `root_node`;
    /// // this must be done manually. Use the `Node::add_child` method instead
    /// // to have this handled for you.
    /// root_node.add_child(arena, second_node);
    /// assert_eq!(root_node.children(arena), &vec![NodeId::from(1)]);
    /// ```
    pub fn set_parent<T, P>(&self, arena: &mut Arena<T>, new_parent: P) -> &Self
    where
        T: PartialEq,
        P: Into<Option<NodeId>>,
    {
        arena.get_mut(*self).set_parent(new_parent);
        self
    }

    /// Adds a new child to this `Node`.
    ///
    /// This method will first add the child `NodeId` to this node's children
    /// list, before setting the new child's parent to this `Node`. Finally,
    /// it will return a reference to itself to allow convenient chaining
    /// of methods.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use crate::isaac::{Arena, NodeId};
    /// let arena = &mut Arena::new();
    /// let root_node = arena.insert("1");
    /// let second_node = arena.insert("2");
    ///
    /// // Make `second_node` a child of `root_node`.
    /// root_node.add_child(arena, second_node);
    ///
    /// assert_eq!(root_node.children(arena), &vec![NodeId::from(1)]);
    /// assert_eq!(second_node.parent(arena), Some(NodeId::from(0)));
    /// ```
    ///
    /// Conveniently, this method returns a reference to the node being mutated
    /// in question, allowing methods to be chained in a readable fashion.
    ///
    /// ```rust
    /// # use crate::isaac::{Arena, NodeId};
    /// let arena = &mut Arena::new();
    /// let root_node = arena.insert("1");
    /// let second_node = arena.insert("2");
    /// let third_node = arena.insert("3");
    ///
    /// // Method chaining
    /// root_node
    ///     .add_child(arena, second_node)
    ///     .add_child(arena, third_node);
    ///
    /// assert_eq!(root_node.children(arena), &vec![
    ///     NodeId::from(1),
    ///     NodeId::from(2),
    /// ]);
    /// ```
    pub fn add_child<T>(&self, arena: &mut Arena<T>, child: NodeId) -> &Self
    where
        T: PartialEq,
    {
        arena.get_mut(*self).add_child(child);
        arena.get_mut(child).set_parent(*self);
        self
    }
}

impl From<usize> for NodeId {
    fn from(id: usize) -> Self {
        Self(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_syntax_arena_internal() {
        //     1
        //    / \
        //   2   3
        //       |
        //       4
        let arena = &mut Arena::new();

        let str_one   = arena.insert("1");
        let str_two   = arena.insert("2");
        let str_three = arena.insert("3");
        let str_four  = arena.insert("4");

        str_one
            .add_child(arena, str_two)
            .add_child(arena, str_three);

        str_three
            .add_child(arena, str_four);

        assert_eq!(*arena, Arena {
            nodes: vec![
                Node {
                    id: NodeId(0),
                    data: "1",
                    parent: None,
                    children: vec![NodeId(1), NodeId(2)],
                },
                Node {
                    id: NodeId(1),
                    data: "2",
                    parent: Some(NodeId(0)),
                    children: vec![],
                },
                Node {
                    id: NodeId(2),
                    data: "3",
                    parent: Some(NodeId(0)),
                    children: vec![NodeId(3)],
                },
                Node {
                    id: NodeId(3),
                    data: "4",
                    parent: Some(NodeId(2)),
                    children: vec![],
                },
            ],
            root: Some(NodeId(0)),
        });
    }

    #[test]
    fn test_syntax_arena() {
        //     1
        //    / \
        //   2   3
        //       |
        //       4
        let arena = &mut Arena::new();

        let str_1 = arena.insert("1");
        let str_2 = arena.insert("2");
        let str_3 = arena.insert("3");
        let str_4 = arena.insert("4");

        str_1
            .add_child(arena, str_2)
            .add_child(arena, str_3);

        str_3
            .add_child(arena, str_4);

        assert_eq!(str_1.parent(arena), None);
        assert_eq!(str_2.parent(arena), Some(str_1));
        assert_eq!(str_3.parent(arena), Some(str_1));
        assert_eq!(str_4.parent(arena), Some(str_3));

        assert_eq!(str_1.children(arena), &vec![str_2, str_3]);
        assert_eq!(str_2.children(arena), &vec![]);
        assert_eq!(str_3.children(arena), &vec![str_4]);
        assert_eq!(str_4.children(arena), &vec![]);
    }
}
