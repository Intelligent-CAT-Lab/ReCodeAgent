use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::log::{LogType, Logger};
use crate::check;
use crate::hash::hash;
const LOAD_FACTOR: f32 = 0.6;
#[cfg(debug_assertions)]
const EPS: f32 = 1e-3;
pub struct OpenEntry {
    key: Option<&'static str>,
    data: Option<&'static str>,
}
pub struct OpenHashTable {
    cur_size: usize,
    max_size: usize,
    arr: Vec<OpenEntry>,
}
impl OpenHashTable {
    pub fn new(initial_size: usize) -> Arc<RwLock<Self>> {
       unimplemented!() 
    }
    fn query(&self, key: &str) -> usize {
        unimplemented!()
    }
    pub fn insert(&mut self, key: &'static str, data: &'static str) {
        unimplemented!()
    }
    pub fn find(&self, key: &str) -> Option<&'static str> {
        unimplemented!()
    }
    pub fn free_open_hash_table(&mut self) {
        unimplemented!()
    }
}