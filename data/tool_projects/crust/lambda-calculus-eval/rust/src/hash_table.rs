use crate::common;
use std::collections::HashMap;
pub const HASH_TABLE_SIZE: usize = 2000;
pub struct HashTable {
table: HashMap<String, common::AstNode>,
}
impl HashTable {
    pub fn new() -> Self {
        unimplemented!()
    }
    pub fn insert(&mut self, key: &str, value: common::AstNode) {
        unimplemented!()
    }
    pub fn table_exists(&self, key: &str) -> bool {
        unimplemented!()
    }
    pub fn search(&self, key: &str) -> Option<&common::AstNode> {
        unimplemented!()
    }
    pub fn delete(&mut self, key: &str) {
        unimplemented!()
    }
}
pub fn hash(key: &str) -> u32 {
    unimplemented!()
}
pub fn createHashTable() -> HashTable {
    unimplemented!()
}
pub fn destroyHashTable(_hashTable: HashTable) {
    unimplemented!()
}