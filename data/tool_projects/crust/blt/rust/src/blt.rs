use std::any::Any;
/// The BLT tree structure.
#[derive(Debug)]
pub struct Blt {
    /// The root node.
    pub root: Box<BltNode>,
    /// Indicates whether the tree is empty.
    pub empty: i32,
}
/// A node in the BLT tree.
#[derive(Debug)]
pub enum BltNode {
    /// An internal node.
    Internal(InternalNode),
    /// A leaf node (external node).
    Leaf(BltIt),
}
/// Represents an internal node in the BLT tree.
#[derive(Debug)]
pub struct InternalNode {
    /// Byte number of difference (32 bits).
    pub byte: u32,
    /// Mask byte (8 bits).
    pub mask: u8,
    /// Padding (23 bits stored in a u32).
    pub padding: u32,
    /// The child node.
    pub kid: Box<BltNode>,
}
/// Represents a leaf node in the BLT tree.
#[derive(Debug)]
pub struct BltIt {
    /// The key associated with the leaf.
    pub key: String,
    /// Associated data.
    pub data: Option<Box<dyn Any>>,
}
impl Blt {
    /// Creates a new BLT tree.
    pub fn blt_new() -> Self {
        unimplemented!();
    }
    /// Clears (destroys) the tree.
    pub fn blt_clear(&mut self) {
        unimplemented!();
    }
    /// Retrieves the leaf node at the given key.
    pub fn blt_get(&self, key: &str) -> Option<BltIt> {
        unimplemented!();
    }
    /// Creates or retrieves the leaf node at the given key.
    pub fn blt_set(&mut self, key: &str) -> BltIt {
        unimplemented!();
    }
    /// Creates or retrieves the leaf node at the given key and returns a tuple (leaf, is_new).
    pub fn blt_setp(&mut self, key: &str) -> (BltIt, bool) {
        unimplemented!();
    }
    /// Inserts the given key/data pair and returns the corresponding leaf.
    pub fn blt_put(&mut self, key: &str, data: Box<dyn Any>) -> BltIt {
        unimplemented!();
    }
    /// Inserts the key/data pair only if the key is absent.
    /// Returns 0 on success or 1 if the key is already present.
    pub fn blt_put_if_absent(&mut self, key: &str, data: Box<dyn Any>) -> i32 {
        unimplemented!();
    }
    /// Deletes the given key from the tree.
    /// Returns 1 if a key was deleted, 0 otherwise.
    pub fn blt_delete(&mut self, key: &str) -> i32 {
        unimplemented!();
    }
    /// Iterates over all leaves with keys having the given prefix.
    /// The closure should return an i32; if it returns 0, iteration stops.
    pub fn blt_allprefixed<F: FnMut(&BltIt) -> i32>(&self, prefix: &str, mut fun: F) -> i32 {
        unimplemented!();
    }
    /// Iterates through all leaves in order and calls the provided closure.
    pub fn blt_forall<F: FnMut(&BltIt)>(&self, mut fun: F) {
        let _ = self.blt_allprefixed("", |it| {
            fun(it);
            1 // always continue iteration
        });
    }
    /// Returns the leaf with the smallest key.
    pub fn blt_first(&self) -> Option<BltIt> {
        unimplemented!();
    }
    /// Returns the leaf with the largest key.
    pub fn blt_last(&self) -> Option<BltIt> {
        unimplemented!();
    }
    /// Returns the next leaf (in order) after the given one.
    pub fn blt_next(&self, it: &BltIt) -> Option<BltIt> {
        unimplemented!();
    }
    /// Returns the previous leaf (in order) before the given one.
    pub fn blt_prev(&self, it: &BltIt) -> Option<BltIt> {
        unimplemented!();
    }
    /// Returns the leaf with the smallest key ≥ the given key.
    pub fn blt_ceil(&self, key: &str) -> Option<BltIt> {
        unimplemented!();
    }
    /// Returns the leaf with the largest key ≤ the given key.
    pub fn blt_floor(&self, key: &str) -> Option<BltIt> {
        unimplemented!();
    }
    /// Returns the number of bytes used by the tree (excluding key storage).
    pub fn blt_overhead(&self) -> usize {
        unimplemented!();
    }
    /// Returns true if the tree is empty.
    pub fn blt_empty(&self) -> bool {
        unimplemented!();
    }
    /// Returns the number of keys in the tree.
    pub fn blt_size(&self) -> i32 {
        unimplemented!();
    }
}