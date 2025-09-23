use std::fs::{File, OpenOptions};
use std::io::{self, Write};
pub fn create_file(path: &str) -> io::Result<File> {
unimplemented!()
}
pub fn get_file(path: &str, mode: &str) -> io::Result<File> {
unimplemented!()
}
pub fn write_to_file(file: &mut File, content: &str) -> io::Result<()> {
unimplemented!()
}
pub fn delete_file(path: &str) -> io::Result<()> {
unimplemented!()
}
pub fn close_file(file: File) -> io::Result<()> {
unimplemented!()
}
pub fn next(file: &mut File) -> io::Result<char> {
unimplemented!()
}