use crate::{csvline::CsvLine};
pub const BUFSIZE: usize = 4096;
pub const MAXCOL: usize = 1024;
pub fn output_line(cline: &CsvLine) {
unimplemented!()
}
pub fn usage(fp: &mut std::fs::File) {
unimplemented!()
}
pub fn debug(level: i32, fmt: &str) {
unimplemented!()
}
pub fn is_bin_char(c: char) -> bool {
unimplemented!()
}
pub fn format_output(str: &mut [&str]) {
unimplemented!()
}
pub fn get_field(buf: &str, buflen: usize, end: &mut usize, in_quoted: &mut bool) -> i32 {
unimplemented!()
}
pub fn main(argc: i32, argv: &[&str]) -> i32 {
unimplemented!()
}