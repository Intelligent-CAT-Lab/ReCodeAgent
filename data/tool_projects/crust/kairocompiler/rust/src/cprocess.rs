use std::fs;
use std::io::Read;
use crate::compiler::{CompileProcess, CompileProcessInputFile, Pos};
/// Creates a new compile_process, reading in the entire input file. Returns None on failure.
pub fn compile_process_create(
filename: &str,
filename_out: &str,
flags: i32,
) -> Option<CompileProcess> {
    unimplemented!()
}
/// Mimics getc() by returning the next character in the process's file buffer.
pub fn compile_process_next_char(lex_process: &mut crate::lex_process::LexProcess) -> char {
    unimplemented!()
}
/// Peeks the next character without consuming it, or '\0' at EOF.
pub fn compile_process_peek_char(lex_process: &mut crate::lex_process::LexProcess) -> char {
    unimplemented!()
}
/// Ungets a character by moving the position back by one, ignoring if already at start of file.
pub fn compile_process_push_char(lex_process: &mut crate::lex_process::LexProcess, _c: char) {
    unimplemented!()
}