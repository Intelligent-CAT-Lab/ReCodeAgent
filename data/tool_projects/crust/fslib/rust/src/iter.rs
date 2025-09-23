use crate::fst::Fst;
use crate::bitset::BitSet;
use crate::queue::Queue;
pub struct FstIter<'a, T> {
    pub fst: &'a Fst,
    pub marked: BitSet,
    pub queue: Queue<T>,
    pub state: u32,
}
impl<'a, T> FstIter<'a, T> {
    pub fn new(fst: &'a Fst) -> Self {
        unimplemented!()
    }
    pub fn next(&mut self) -> Option<T> {
        unimplemented!()
    }
    pub fn remove(self) {
        unimplemented!()
    }
    pub fn visited(&self, state: T) -> bool {
        unimplemented!()
    }
}