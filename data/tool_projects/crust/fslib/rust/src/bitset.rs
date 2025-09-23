type word_t = u32;
const WORD_SIZE: usize = std::mem::size_of::<word_t>() * 8;
#[inline]
fn bindex(b: usize) -> usize {
    b / WORD_SIZE
}
#[inline]
fn boffset(b: usize) -> usize {
    b % WORD_SIZE
}
pub struct BitSet {
    words: Vec<word_t>,
}
impl BitSet {
    pub fn new(nbits: usize) -> Self {
        unimplemented!()
    }
    pub fn remove(&mut self) {
        unimplemented!()
    }
    pub fn set(&mut self, b: usize) {
        unimplemented!()
    }
    pub fn clear(&mut self, b: usize) {
        unimplemented!()
    }
    pub fn get(&self, b: usize) -> bool {
        unimplemented!()
    }
    pub fn clear_all(&mut self) {
        unimplemented!()
    }
    pub fn set_all(&mut self) {
        unimplemented!()
    }
    pub fn union(&mut self, other: &BitSet) {
        unimplemented!()
    }
    pub fn intersect(&mut self, other: &BitSet) {
        unimplemented!()
    }
    pub fn toggle_all(&self) -> Self {
        unimplemented!()
    }
}