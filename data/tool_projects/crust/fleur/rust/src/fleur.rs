use std::{fmt, fs::File};
const M: u64 = 18446744073709551557;
const G: u64 = 18446744073709550147;
const FNV_PRIME: u64 = 1099511628211;
const FNV_OFFSET_BASIS: u64 = 14695981039346656037;
#[derive(Debug, Clone)]
pub struct Header {
    pub version: u64,
    pub n: u64,
    pub p: f64,
    pub k: u64,
    pub m: u64,
    pub n_value: u64,
}
impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}
#[derive(Debug, Clone)]
pub struct BloomFilter {
    pub version: u64,
    pub datasize: u64,
    pub h: Header,
    pub m: u64,
    pub v: Vec<u64>,
    pub data: Vec<u8>,
    pub modified: i32,
    pub error: i32,
}
impl fmt::Display for BloomFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}
impl BloomFilter {
    pub fn fingerprint(&self, buf: &[u8]) -> Vec<u64> {
        unimplemented!()
    }
    pub fn add(&mut self, buf: &[u8]) -> i32 {
        unimplemented!()
    }
    pub fn join(&mut self, bf: &BloomFilter) -> i32 {
        unimplemented!()
    }
    pub fn check(&self, buf: &[u8]) -> i32 {
        unimplemented!()
    }
    pub fn set_data(&mut self, buf: &[u8]) {
        unimplemented!()
    }
    pub fn to_file(&self, file: File) -> i32 {
        unimplemented!()
    }
    pub fn initialize(n: u64, p: f64, buf: &[u8]) -> BloomFilter {
        unimplemented!()
    }
}
impl Header {
    pub fn check(&self) -> bool {
        unimplemented!()
    }
}