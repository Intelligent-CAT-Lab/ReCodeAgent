use std::any::Any;
use std::cell::RefCell;
use std::rc::{Rc, Weak};
/// Represents an internal CBT node (non‐leaf).
#[derive(Debug)]
pub struct CbtNode {
    /// Critical bit position.
    pub crit: i16,
    /// Left child.
    pub left: Option<Box<CbtNode>>,
    /// Right child.
    pub right: Option<Box<CbtNode>>,
}
/// Represents a leaf node in the crit‐bit tree.
/// Leaves are also linked together in a doubly linked list.
#[derive(Debug)]
pub struct CbtLeaf {
    /// Critical bit for this leaf.
    pub crit: i16,
    /// Associated data.
    pub data: Box<dyn Any>,
    /// Key associated with this leaf.
    pub key: String,
    /// Previous leaf in the doubly linked list.
    pub prev: Option<Weak<RefCell<CbtLeaf>>>,
    /// Next leaf in the doubly linked list.
    pub next: Option<Rc<RefCell<CbtLeaf>>>,
}
/// A type alias for a reference‑counted, mutable leaf.
pub type CbtLeafPtr = Rc<RefCell<CbtLeaf>>;
/// Callback type for duplicating a key.
pub type DupFn = dyn Fn(&Cbt, &dyn Any) -> Box<dyn Any>;
/// Callback type for obtaining the length of a key.
pub type GetLenFn = dyn Fn(&Cbt, &dyn Any) -> i32;
/// Callback type for comparing two keys.
pub type CmpFn = dyn Fn(&Cbt, &dyn Any, &dyn Any) -> i32;
/// Callback type for determining the critical bit between two keys.
pub type GetCritFn = dyn Fn(&Cbt, &dyn Any, &dyn Any) -> i32;
/// Represents the entire crit‑bit tree.
pub struct Cbt {
    /// Number of elements in the tree.
    pub count: i32,
    /// Root of the internal node tree.
    pub root: Option<Box<CbtNode>>,
    /// Pointer to the first leaf in the linked list.
    pub first: Option<CbtLeafPtr>,
    /// Pointer to the last leaf in the linked list.
    pub last: Option<CbtLeafPtr>,
    /// Callback to duplicate a key.
    pub dup: Option<Box<DupFn>>,
    /// Callback to get the length of a key.
    pub getlen: Option<Box<GetLenFn>>,
    /// Callback to compare two keys.
    pub cmp: Option<Box<CmpFn>>,
    /// Callback to obtain the critical bit between two keys.
    pub getcrit: Option<Box<GetCritFn>>,
    /// Fixed key length (if applicable).
    pub len: i32,
}
impl Cbt {
    /// Creates a new crit‐bit tree with ASCIIZ keys.
    pub fn cbt_new() -> Self {
        unimplemented!();
    }
    /// Creates a new crit‐bit tree in "u" mode (fixed key length).
    pub fn cbt_new_u(len: i32) -> Self {
        unimplemented!();
    }
    /// Creates a new crit‐bit tree in "enc" mode.
    pub fn cbt_new_enc() -> Self {
        unimplemented!();
    }
    /// Deletes the crit‐bit tree.
    pub fn cbt_delete(self) {
        unimplemented!();
    }
    /// Returns the data stored at the given key.
    pub fn cbt_get_at(&self, key: &str) -> Option<Box<dyn Any>> {
        unimplemented!();
    }
    /// Inserts data at the given key and returns the corresponding leaf.
    pub fn cbt_put_at(&mut self, data: Box<dyn Any>, key: &str) -> CbtLeaf {
        unimplemented!();
    }
    /// Returns the number of keys in the tree.
    pub fn cbt_size(&self) -> i32 {
        unimplemented!();
    }
    /// Returns the first leaf in order.
    pub fn cbt_first(&self) -> Option<CbtLeaf> {
        unimplemented!();
    }
    /// Returns the last leaf in order.
    pub fn cbt_last(&self) -> Option<CbtLeaf> {
        unimplemented!();
    }
    /// Returns the next leaf after the given one.
    pub fn cbt_next(_leaf: &CbtLeaf) -> Option<CbtLeaf> {
        unimplemented!();
    }
    /// Replaces the data stored at the given leaf.
    pub fn cbt_put(&mut self, _leaf: &mut CbtLeaf, _data: Box<dyn Any>) {
        unimplemented!();
    }
    /// Retrieves the data stored at the given leaf.
    pub fn cbt_get(&self, _leaf: &CbtLeaf) -> Option<Box<dyn Any>> {
        unimplemented!();
    }
    /// Returns the key associated with the given leaf.
    pub fn cbt_key(&self, _leaf: &CbtLeaf) -> &str {
        unimplemented!();
    }
    /// Finds a leaf at the given key.
    pub fn cbt_at(&self, key: &str) -> Option<CbtLeaf> {
        unimplemented!();
    }
    /// Returns true if the tree contains the given key.
    pub fn cbt_has(&self, key: &str) -> bool {
        unimplemented!();
    }
    /// Iterates over all leaves, applying the given closure.
    pub fn cbt_forall<F: FnMut(&CbtLeaf)>(&self, _f: F) {
        unimplemented!();
    }
    /// Iterates over all entries, applying the given closure with data and key.
    pub fn cbt_forall_at<F: FnMut(Box<dyn Any>, &str)>(&self, _f: F) {
        unimplemented!();
    }
    /// Removes the entry with the given key.
    pub fn cbt_remove(&mut self, key: &str) -> Option<Box<dyn Any>> {
        unimplemented!();
    }
    /// Removes all entries from the tree.
    pub fn cbt_remove_all(&mut self) {
        unimplemented!();
    }
    /// Removes all entries, calling the provided function for each.
    pub fn cbt_remove_all_with<F: FnMut(Box<dyn Any>, &str)>(&mut self, _f: F) {
        unimplemented!();
    }
    /// Inserts an entry using a provided function and key, returning a leaf.
    pub fn cbt_put_with<F: FnMut(Box<dyn Any>) -> Box<dyn Any>>(
        &mut self,
        _f: F,
        key: &str,
    ) -> CbtLeaf {
        unimplemented!();
    }
    /// Inserts an entry with the given key; returns a tuple (is_new, leaf).
    pub fn cbt_insert(&mut self, key: &str) -> (bool, CbtLeaf) {
        unimplemented!();
    }
    /// Returns the overhead in bytes used by the tree.
    pub fn cbt_overhead(&self) -> usize {
        unimplemented!();
    }
}