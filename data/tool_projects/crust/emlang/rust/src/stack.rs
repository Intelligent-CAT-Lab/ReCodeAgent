use crate::{data, utils};
pub const DEFAULT_POPPED_CAP: usize = 32;
pub const DEFAULT_STACK_CAP: usize = 1024;
#[derive(Debug)]
pub struct Popped {
    pub str: String,
    pub marked: bool,
}
#[derive(Debug)]
pub struct Stack {
    pub buf: Vec<data::Data>,
    pub cap: usize,
    pub size: usize,
    pub popped: Vec<Popped>,
    pub popped_cap: usize,
    pub popped_size: usize,
}
impl Stack {
    fn new(cap: usize, popped_cap: usize) -> Self {
        unimplemented!()
    }
    fn push(&mut self, data: data::Data) {
        unimplemented!()
    }
    fn pop(&mut self) -> Option<data::Data> {
        unimplemented!()
    }
    fn dup(&mut self, off: usize) -> i32 {
        unimplemented!()
    }
    fn swap(&mut self, off: usize) -> i32 {
        unimplemented!()
    }
    fn shrink_to(&mut self, size: usize) {
        unimplemented!()
    }
    fn clear(&mut self) {
        unimplemented!()
    }
    fn gc(&mut self) {
        unimplemented!()
    }
}