use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Clone)]
pub enum Color {
    Red,
    Black,
}

pub type Key = i32;
pub type NodeRef = Rc<RefCell<Node>>;

#[derive(Debug, Clone)]
pub struct Node {
    pub key: Key,
    pub color: Color,
    pub left: Option<NodeRef>,   // None == NIL
    pub right: Option<NodeRef>,  // None == NIL
    pub parent: Option<NodeRef>, // None == NIL / root parent
}

#[derive(Debug, Clone)]
pub struct RBTree {
    /// Root of the tree. `None` means the tree is empty (NIL).
    pub root: Option<NodeRef>,
}

impl RBTree {
    pub fn new() -> Self {
        unimplemented!()
    }

    /// Performs a right rotation around non-NIL node `x`.
    pub fn right_rotate(&mut self, x: NodeRef) {
        unimplemented!()
    }

    /// Performs a left rotation around non-NIL node `x`.
    pub fn left_rotate(&mut self, x: NodeRef) {
        unimplemented!()
    }

    /// Recursively drops a subtree. `None` is treated as NIL.
    pub fn free_node(node: Option<NodeRef>) {
        unimplemented!()
    }

    /// Deletes the Red-Black Tree safely.
    pub fn delete_rbtree(self) {
        unimplemented!()
    }

    /// Fixes the Red-Black Tree after insertion (z must be non-NIL).
    pub fn rbtree_insert_fixup(&mut self, z: NodeRef) {
        unimplemented!()
    }

    /// Inserts a new key and returns the inserted node.
    pub fn rbtree_insert(&mut self, key: Key) -> Option<NodeRef> {
        unimplemented!()
    }

    /// Finds a node by key. Returns `None` if not found.
    pub fn rbtree_find(&self, key: Key) -> Option<NodeRef> {
        unimplemented!()
    }

    /// Returns the minimum node, or `None` if the tree is empty.
    pub fn rbtree_min(&self) -> Option<NodeRef> {
        unimplemented!()
    }

    /// Returns the maximum node, or `None` if the tree is empty.
    pub fn rbtree_max(&self) -> Option<NodeRef> {
        unimplemented!()
    }

    /// Replaces subtree rooted at `u` with subtree rooted at `v` (`None` == NIL).
    pub fn transplant(&mut self, u: NodeRef, v: Option<NodeRef>) {
        unimplemented!()
    }

    /// Fixes up after deletion starting from node `x` (`None` == NIL).
    pub fn delete_fixup(&mut self, x: Option<NodeRef>) {
        unimplemented!()
    }

    /// Erases node `p` (must be a valid non-NIL node in the tree).
    pub fn erase(&mut self, p: NodeRef) {
        unimplemented!()
    }

    /// In-order traversal of `curr` into `arr` until `n` elements (`None` == NIL).
    pub fn subtree_to_array(&self, curr: Option<NodeRef>, arr: &mut Vec<Key>, n: usize, count: &mut usize) {
        unimplemented!()
    }

    /// Returns up to `n` keys from the tree in-order.
    pub fn to_array(&self, n: usize) -> Vec<Key> {
        unimplemented!()
    }
}