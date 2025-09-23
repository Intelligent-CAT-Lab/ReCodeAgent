use crate::fst::{Fst, ArcData};
use crate::sr::Sr;
use crate::heap::Heap;
use std::collections::HashMap;
use std::cmp::Ordering;
pub struct ShortestPath {
    sr: Sr,
    weights: Vec<f32>,
    backtrack: Vec<Option<ArcData>>,
}
impl ShortestPath {
    pub fn new(fst: &Fst) -> Self {
        unimplemented!()
    }
    fn backtrace(&self, path: &mut Fst, final_state: u32) {
        unimplemented!()
    }
    pub fn find_shortest_path(fst: &Fst, path: &mut Fst) {
        unimplemented!()
    }
    fn states_cmp(&self, a: &u32, b: &u32) -> Ordering {
       unimplemented!() 
    }
}
fn states_hash(a: &u32) -> u64 {
    unimplemented!()
}
fn states_key_eq(a: &u32, b: &u32) -> bool {
    unimplemented!()
}