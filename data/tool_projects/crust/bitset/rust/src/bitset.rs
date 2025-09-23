use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
#[derive(Debug)]
pub struct Bitset {
    pub flip: bool,
    pub size: usize,
    pub count: usize,
    pub array: Vec<u8>,
}
impl Bitset {
    pub fn new(nb_bits: usize) -> Self {
       unimplemented!() 
    }
    pub fn delete(self) {
        unimplemented!()
    }
    pub fn count(&self) -> usize {
        unimplemented!()
    }
    pub fn size(&self) -> usize {
        unimplemented!()
    }
    pub fn test(&self, idx: usize) -> bool {
        unimplemented!()
    }
    pub fn any(&self) -> bool {
        unimplemented!()
    }
    pub fn none(&self) -> bool {
        unimplemented!()
    }
    pub fn all(&self) -> bool {
        unimplemented!()
    }
    pub fn set(&mut self, idx: usize, value: bool) {
        unimplemented!()
    }
    pub fn reset(&mut self) {
        unimplemented!()
    }
    pub fn flip(&mut self) {
        unimplemented!()
    }
    pub fn to_string(&self) -> String {
        unimplemented!()
    }
    pub fn write(&self, path: &str) -> io::Result<()> {
        unimplemented!()
    }
    pub fn read(path: &str) -> io::Result<Self> {
        unimplemented!()
    }
}