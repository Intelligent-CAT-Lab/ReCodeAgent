use crate::utils::buffer::PgnBuffer;
const PGN_METADATA_INITIAL_SIZE: usize = 8;
const PGN_METADATA_GROW_SIZE: usize = 8;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PgnMetadataItem {
    key: PgnBuffer,
    value: PgnBuffer,
}
impl PgnMetadataItem {
    pub fn new() -> Self {
        unimplemented!()
    }
}
use std::collections::HashMap;
#[derive(Debug)]
pub struct PgnMetadata {
    items: HashMap<String, String>,
}
impl PgnMetadata {
    pub fn new() -> Self {
        unimplemented!()
    }
    pub fn from_string_with_consumption(s: &str, consumed: &mut usize) -> Self {
        unimplemented!()
    }
    pub fn from_string(s: &str) -> Self {
        unimplemented!()
    }
    pub fn insert(&mut self, key: &str, value: &str) {
        unimplemented!()
    }
    pub fn get(&self, key: &str) -> Option<&str> {
        unimplemented!()
    }
    pub fn delete(&mut self, key: &str) {
        unimplemented!()
    }
}