use std::collections::VecDeque;
pub struct Queue<T> {
    pub items: VecDeque<T>,
}
impl<T> Queue<T> {
    pub fn new() -> Self {
        unimplemented!()
    }
    pub fn remove(self) {
        unimplemented!()
    }
    pub fn empty(&mut self) {
        unimplemented!()
    }
    pub fn enqueue(&mut self, item: T) {
        unimplemented!()
    }
    pub fn dequeue(&mut self) -> Option<T> {
        unimplemented!()
    }
    pub fn len(&self) -> usize {
        unimplemented!()
    }
}