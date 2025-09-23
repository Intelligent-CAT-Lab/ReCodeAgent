use crate::csvfield::CsvField;
use std::fs::File;
use std::io::Write;
#[derive(Debug, Clone)]
pub struct CsvLine {
pub field: Vec<CsvField>,
pub fieldsize: usize,
pub current_idx: usize,
pub eol_str: String,
}
impl CsvLine {
pub fn new() -> Self {
    unimplemented!();
}
pub fn get_field_count(&self) -> usize {
    unimplemented!();
}
pub fn get_field(&self, idx: usize) -> Option<&str> {
    unimplemented!();
}
pub fn reset(&mut self) {
    unimplemented!();
}
pub fn add_field(&mut self, txtfield: &str, fieldstartidx: usize, fieldlen: usize) {
    unimplemented!();
}
pub fn append_field(&mut self, txtfield: &str, fieldstartidx: usize, fieldlen: usize) {
    unimplemented!();
}
pub fn print_to_file(&self, fp: &mut File) {
    unimplemented!();
}
}