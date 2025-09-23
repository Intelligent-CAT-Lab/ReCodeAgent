use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::log::{LogType, Logger};
use crate::check;
use crate::hash::hash;
const LOAD_FACTOR: f32 = 1.0;
#[cfg(debug_assertions)]
const EPS: f32 = 1e-3;
pub struct CuckooEntry {
    key: Option<&'static str>,
    data: Option<&'static str>,
    marker: u32,
}
pub struct CuckooHashTable {
    cur_size: u32,
    cur_marker: u32,
    max_size: u32,
    first_arr: Vec<CuckooEntry>,
    second_arr: Vec<CuckooEntry>,
}
impl CuckooHashTable {
    pub fn new(initial_size: usize) -> Arc<RwLock<Self>> {
        unimplemented!()
    }
    pub fn insert(&mut self, key: &'static str, data: &'static str) {
        unimplemented!()
    }
    pub fn find(&self, key: &str) -> Option<&'static str> {
        unimplemented!()
    }
    fn resize(&mut self) {
        unimplemented!()
    }
    fn refill(&mut self) {
        unimplemented!()
    }
    fn recreate(&mut self, new_size: u32) {
        unimplemented!()
    }
    fn get_first_entry(&self, key: &str) -> &mut CuckooEntry {
        unimplemented!()
    }
    fn get_second_entry(&self, key: &str) -> &mut CuckooEntry {
        unimplemented!()
    }
    fn try_to_store(&mut self, key: &'static str, data: &'static str, entry: &mut CuckooEntry) -> bool {
        unimplemented!()
    }
    fn swap_key_data_entry(&mut self, key: &mut &'static str, data: &mut &'static str, entry: &mut CuckooEntry) {
        unimplemented!()
    }
    fn free_cukoo_hash_table(self) {
        unimplemented!()
    }
}