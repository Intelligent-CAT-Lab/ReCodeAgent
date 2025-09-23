use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
const SYM_INIT_SIZE: usize = 1024;
const TOKEN_SIZE: usize = 1024;
type Token = String;
pub fn fnv32(token: &str) -> usize{
    unimplemented!()
}
pub struct SymTable {
    pub n_items: usize,
    pub n_max: usize,
    pub sym: Vec<Option<String>>, // Symbol storage
    pub rev: HashMap<String, usize>, // Reverse lookup
}
impl SymTable {
    pub fn new() -> Self {
        unimplemented!()
    }
    pub fn remove(self) {
        unimplemented!()
    }
    pub fn reverse(&self) -> &HashMap<String, usize> {
        unimplemented!()
    }
    pub fn read(&mut self, fin: &mut dyn BufRead) -> io::Result<()> {
        unimplemented!()
    }
    pub fn fread(&mut self, filename: &str) -> io::Result<()> {
        unimplemented!()
    }
    pub fn print(&self) {
        unimplemented!()
    }
    pub fn add(&mut self, id: i32, token: &str) -> Option<String> {
        unimplemented!()
    }
    pub fn getr(&self, token: &str) -> Option<i32> {
        unimplemented!()
    }
    pub fn get(&self, id: i32) -> Option<&str> {
        unimplemented!()
    }
    pub fn compile(&mut self, token: &str) {
        unimplemented!()
    }
}