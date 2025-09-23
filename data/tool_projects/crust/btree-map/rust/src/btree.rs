use std::collections::HashMap;
use std::sync::Arc;
use std::any::Any;
pub const BTREE_KEY_SIZE: usize = 10;
#[derive(Clone)]
pub struct BTreeKey {
    pub key: Vec<u8>,
    pub len: usize,
}
#[derive(Clone)]
pub struct Value {
    pub value: Vec<u8>,
    pub len: usize,
}
pub struct Entry {
    pub key: BTreeKey,
    pub value: Value,
}
pub struct EntryList {
    pub entries: Vec<Entry>,
    pub len: usize,
    pub cap: usize,
}
pub struct Node {
    pub key_hash: u32,
    pub p_key: [u8; BTREE_KEY_SIZE],
    pub key_len: usize,
    pub value: Value,
    pub child_left: Option<Arc<Node>>,
    pub child_right: Option<Arc<Node>>,
}
pub struct BTree {
    pub node: Option<Arc<Node>>,
}
impl BTree {
    pub fn new_btree() -> Self {
        BTree { node: None }
    }
    pub fn add_entry(&mut self, key: Vec<u8>, key_len: usize, value: Vec<u8>, value_len: usize) {
       unimplemented!()
    }
    pub fn list_entries(&self) -> EntryList {
        unimplemented!()
    }
    pub fn remove_entry(&mut self, key: &Vec<u8>, key_len: usize) {
        unimplemented!()
    }
    pub fn get_entry_count(&self) -> usize {
        unimplemented!()
    }
    pub fn find_entry(&self, key: &Vec<u8>, key_len: usize) -> Option<Value> {
        unimplemented!()
    }
    pub fn free_tree(&mut self) {
        unimplemented!()
    }
}
impl Node {
    pub fn new_node(key: Vec<u8>, key_len: usize, value: Vec<u8>, value_len: usize) -> Arc<Self> {
       unimplemented!() 
    }
    pub fn add_node(&mut self, n_node: Arc<Node>) {
        unimplemented!()
    }
    pub fn free_node(&mut self) {
        unimplemented!()
    }
    pub fn delete_node(root: &mut Arc<Node>, key_hash: u32, key: Vec<u8>, key_len: usize) -> Option<Arc<Node>> {
        unimplemented!()
    }
    pub fn get_node_count(&self) -> usize {
        unimplemented!()
    }
    pub fn list_node_entries(&self, list: &mut EntryList) {
        unimplemented!()
    }
    pub fn find_value(&self, key_hash: u32, key: Vec<u8>, key_len: usize) -> Option<Value> {
        unimplemented!()
    }
}
pub fn min_size(a: usize, b: usize) -> usize {
    unimplemented!()
}
pub fn calc_key_hash(key: &Vec<u8>, key_len: usize) -> u32 {
    unimplemented!()
}
pub fn btree_malloc<T: Any>(_: usize) -> T {
    unimplemented!()
}
// Not needed in Rust, but we can implement it
pub fn btree_free<T: Any>(_: &T) {
    unimplemented!()
}