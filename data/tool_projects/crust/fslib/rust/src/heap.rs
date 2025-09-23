use std::collections::HashMap;
use std::cmp::Ordering;
const HEAP_INIT_SIZE: usize = 0xff;
const HEAP_RESIZE_FACTOR: usize = 2;
pub type HeapCmp<T> = fn(&T, &T) -> Ordering;
pub struct Heap<T> {
    pub n_items: usize,
    pub n_max: usize,
    pub limit: usize,
    pub cmp: HeapCmp<T>,
    pub ht: HashMap<T, usize>,
    pub items: Vec<T>,
}
pub fn parent(i: usize) -> usize {
    unimplemented!()
}
pub fn left(i: usize) -> usize {
    unimplemented!()
}
pub fn right(i: usize) -> usize {
    unimplemented!()
}
impl<T: Ord + Clone + std::hash::Hash + Eq> Heap<T> {
    pub fn new(cmp: HeapCmp<T>, item_size: usize, init_size: usize, limit: usize) -> Self {
        unimplemented!()
    }
    pub fn index(&mut self, hsh: fn(&T) -> u64, hcmp: fn(&T, &T) -> bool) {
        unimplemented!()
    }
    pub fn remove(&mut self) {
        unimplemented!()
    }
    pub fn pop(&mut self) -> Option<T> {
        unimplemented!()
    }
    pub fn heapify(&mut self, i: usize) {
        unimplemented!()
    }
    pub fn insert(&mut self, item: T) {
        unimplemented!()
    }
    pub fn update(&mut self, item: T, i: usize) {
        unimplemented!()
    }
    pub fn find(&self, item: &T) -> Option<usize> {
        unimplemented!()
    }
    fn swap_items(&mut self, i: usize, j: usize) {
        unimplemented!() 
    }
}