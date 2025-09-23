use crate::{vm, ast, token, settings};
pub const SIMPLE_LANG_MISC_H: bool = true;
/// Replicates: void show_opcode(Instruction* instruction, int instr_count);
pub fn show_opcode(instruction: &vm::Instruction, instr_count: i32) {
unimplemented!()
}
/// Replicates: void show_tokens(Token* tokens);
/// Interpreted as working with a slice of tokens in Rust.
pub fn show_tokens(tokens: &[token::Token]) {
unimplemented!()
}
/// Replicates: void print_asts(ASTNode** asts);
/// Interpreted as working with a slice of ASTNode pointers in Rust.
pub fn print_asts(asts: &[Box<ast::ASTNode>]) {
unimplemented!()
}
/// Replicates: char* read_file(const char* source_path);
pub fn read_file(source_path: &str) -> String {
unimplemented!()
}
/// Replicates: void print_ast(ASTNode* node, int level);
pub fn print_ast(node: &ast::ASTNode, level: i32) {
unimplemented!()
}
/// Replicates: char* get_opcode_name(OpCode opcode);
pub fn get_opcode_name(opcode: vm::OpCode) -> String {
unimplemented!()
}