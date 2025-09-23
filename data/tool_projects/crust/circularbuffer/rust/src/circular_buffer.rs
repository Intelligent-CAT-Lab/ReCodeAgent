use std::vec::Vec;
pub struct CircularBuffer {
    size: usize,
    data_size: usize,
    tail_offset: usize,
    head_offset: usize,
    buffer: Vec<u8>,
}
impl CircularBuffer {
    pub fn new(size: usize) -> Self {
        CircularBuffer {
            size,
            data_size: 0,
            tail_offset: 0,
            head_offset: 0,
            buffer: vec![0; size],
        }
    }
    pub fn pop(&mut self, length: usize, data_out: &mut [u8]) -> usize {
        unimplemented!()
    }
    pub fn read(&self, length: usize, data_out: &mut [u8]) -> usize {
        unimplemented!()
    }
    pub fn get_size(&self) -> usize {
        unimplemented!()
    }
    pub fn print(&self, hex: bool) {
        unimplemented!()
    }
    pub fn free(self) {
        unimplemented!()
    }
    pub fn reset(&mut self) {
        unimplemented!()
    }
    pub fn get_capacity(&self) -> usize {
        unimplemented!()
    }
    pub fn push(&mut self, src: &[u8], length: usize) {
        unimplemented!()
    }
    pub fn inter_read(&mut self, length: usize, data_out: &mut [u8], reset_head: bool) -> usize {
        unimplemented!()
    }
    pub fn get_data_size(&self) -> usize {
        unimplemented!()
    }
}