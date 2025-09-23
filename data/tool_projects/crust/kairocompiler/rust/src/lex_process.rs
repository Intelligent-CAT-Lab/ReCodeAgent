use crate::compiler::CompileProcess;
use crate::vector::{vector_create, vector_free, Vector};
use crate::compiler::Pos;
/// Function pointer table for reading chars, peeking, and ungetting.
/// Marked Copy and Clone so we can move it freely (fixing E0507).
#[derive(Clone, Copy, Debug)]
pub struct LexProcessFunctions {
pub next_char: fn(&mut LexProcess) -> char,
pub peek_char: fn(&mut LexProcess) -> char,
pub push_char: fn(&mut LexProcess, char),
}
/// The LexProcess struct, referencing a CompileProcess, token vector, etc.
#[derive(Debug, Default, Clone)]
pub struct LexProcess {
pub pos: Pos,
pub token_vec: Option<Vector>,
pub compiler: Option<Box<CompileProcess>>,
pub function: Option<LexProcessFunctions>,
pub private: Option<()>,
}
/// Creates a new LexProcess, allocating a Vector to store tokens, referencing the given CompileProcess.
pub fn lex_process_create(
compiler: CompileProcess,
functions: LexProcessFunctions,
private: Option<()>,
) -> LexProcess {
    unimplemented!()
}
/// Frees the lex process, including the token vector. In Rust, dropping is enough.
pub fn lex_process_free(process: LexProcess) {
    unimplemented!()
}
/// Returns the private data pointer (always None in this safe version).
pub fn lex_process_private(process: &LexProcess) -> Option<()> {
    unimplemented!()
}
/// Returns a reference to the token vector if any.
pub fn lex_process_tokens(process: &LexProcess) -> Option<&Vector> {
    unimplemented!()
}