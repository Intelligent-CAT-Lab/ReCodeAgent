pub struct Vector<T>{
    pub capacity: usize,
    pub size: usize,
    pub data: Vec<T>,
}
impl<T> Vector<T>{
    pub fn new(capacity: usize) -> Self {
        Vector {
            capacity,
            size: 0,
            data: Vec::with_capacity(capacity),
        }
    }
    pub fn release(&mut self) {
        unimplemented!()
    }
    pub fn size(&self) -> usize {
        unimplemented!()
    }
    pub fn push_back(&mut self, value: T) {
        unimplemented!()
    }
    pub fn get_element_at(&self, index: usize) -> Option<&T> {
        unimplemented!()
    }
    pub fn pop_back(&mut self) -> Option<T> {
        unimplemented!()
    }
    pub fn push_front(&mut self, value: T) {
        unimplemented!()
    }
    pub fn pop_front(&mut self) -> Option<T> {
        unimplemented!()
    }
    pub fn top_front(&self) -> Option<&T> {
        unimplemented!()
    }
    pub fn top_back(&self) -> Option<&T> {
        unimplemented!()
    }
    pub fn insert_at_index(&mut self, value: T, index: usize) {
        unimplemented!()
    }
    pub fn remove_at_index(&mut self, index: usize) -> Option<T> {
        unimplemented!()
    }
    pub fn index_of(&self, value: &T) -> Option<usize> {
        unimplemented!()
    }
    pub fn index_of_with_start(&self, value: &T, start: usize) -> Option<usize> {
        unimplemented!()
    }
}