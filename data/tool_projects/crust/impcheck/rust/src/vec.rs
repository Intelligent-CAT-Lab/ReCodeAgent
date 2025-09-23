pub struct IntVec<'a> {
    pub capacity: u64, 
    pub size: u64, 
    pub data: &'a [u8],
}
impl IntVec<'_> {
    pub fn vec_free(&mut self) {
    unimplemented!()
    }
    pub fn vec_clear(&mut self) {
    unimplemented!()
    }
    pub fn vec_reserve(&mut self, new_size: u64) {
    unimplemented!()
    }
    pub fn vec_push(&mut self, elem: i32) {
    unimplemented!()
    }
}