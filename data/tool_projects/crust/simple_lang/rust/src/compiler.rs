use crate::{vm, lexer, parser, ast};
pub const SIMPLE_LANG_COMPILER_H: bool = true;
/// Replicates: Instruction* compile_asts(ASTNode** asts, int* instr_count);
/// In Rust: returns a vector of Instructions, updates an integer by reference.
pub fn compile_asts(asts: &[Box<ast::ASTNode>], instr_count: &mut i32) -> Vec<vm::Instruction> {
unimplemented!()
}
/// Replicates: Instruction* compile(const char* source, int* instr_count);
pub fn compile(source: &str, instr_count: &mut i32) -> Vec<vm::Instruction> {
unimplemented!()
}
/// Replicates: void compile_expression(ASTNode* node, Instruction* instructions, int* count);
/// In Rust: modifies a mutable vector of Instructions and a count reference.
pub fn compile_expression(
node: &ast::ASTNode,
instructions: &mut Vec<vm::Instruction>,
count: &mut i32,
) {
unimplemented!()
}
/// Replicates: Instruction* compile_statement(ASTNode* ast, int* instr_count);
pub fn compile_statement(ast: &ast::ASTNode, instr_count: &mut i32) -> Vec<vm::Instruction> {
unimplemented!()
}
/// Replicates: void emit(Instruction* instructions, int* count, OpCode opcode, const char* operand);
pub fn emit(
instructions: &mut Vec<vm::Instruction>,
count: &mut i32,
opcode: vm::OpCode,
operand: &str,
) {
unimplemented!()
}