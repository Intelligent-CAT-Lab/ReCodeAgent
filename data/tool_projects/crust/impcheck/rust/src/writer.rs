use std::fs::File;
pub struct Writer {
    file: File,
}
impl Writer {
    pub fn write_char(&mut self, c_int: i32) {
    unimplemented!()
    }
    pub fn write_int(&mut self, i: i32) {
    unimplemented!()
    }
    pub fn write_ints(&mut self, data: &[i32], nb_ints: u64) {
    unimplemented!()
    }
    pub fn write_sig(&mut self, sig: &[u8]) {
    unimplemented!()
    }
    pub fn writer_init(output_path: &str) -> Self {
    unimplemented!()
    }
    pub fn write_bool(&mut self, b: bool) {
    unimplemented!()
    }
    pub fn write_uls(&mut self, data: &[u64], nb_uls: u64) {
    unimplemented!()
    }
    pub fn write_ul(&mut self, ul: u64) {
    unimplemented!()
    }
}