use std::fs::File;
use std::io::Write;
#[derive(Debug, Clone)]
pub struct CsvField {
pub data: String,
pub len: usize,
}
impl CsvField {
pub fn new() -> Self {
    unimplemented!();
}
pub fn reset(&mut self) {
    unimplemented!();
}
pub fn set(&mut self, buf: &str, buf_start_idx: usize, len: usize) {
    unimplemented!();
}
pub fn append(&mut self, buf: &str, buf_start_idx: usize, buflen: usize) {
    unimplemented!();
}
pub fn print_to_file(&self, fp: &mut File) {
unimplemented!();
}
}