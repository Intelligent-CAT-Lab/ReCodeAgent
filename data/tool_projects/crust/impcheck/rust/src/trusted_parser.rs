use std::fs::File;
use std::io::Write;
pub struct IntVec;
pub struct TrustedParser {
f_out: File,
f: File,
}
impl TrustedParser {
pub fn tp_end(&mut self) {
unimplemented!()
}
pub fn output_literal_buffer(&self) {
unimplemented!()
}
pub fn append_integer(&self) {
unimplemented!()
}
pub fn tp_init(filename: &str, out: File) -> Self {
unimplemented!()
}
pub fn tp_parse(&mut self, sig: &mut Option<Vec<u8>>) -> bool {
unimplemented!()
}
pub fn process(&mut self, c: char) -> bool {
unimplemented!()
}
}