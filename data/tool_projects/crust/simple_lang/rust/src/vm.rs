use crate::{ast, settings};
pub const SIMPLE_LANG_VM_H: bool = true;
/// Replicates the C enum:
/// typedef enum { LOAD_CONST, LOAD_NAME, STORE_NAME, BINARY_ADD, BINARY_SUB, STK_DIS } OpCode;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpCode {
LOAD_CONST,
LOAD_NAME,
STORE_NAME,
BINARY_ADD,
BINARY_SUB,
STK_DIS,
}
/// Replicates the C struct:
/// typedef struct {
///     OpCode opcode;
///     char* operand;
/// } Instruction;
#[derive(Debug, Clone)]
pub struct Instruction {
pub opcode: OpCode,
pub operand: String,
}
/// Replicates the C struct Frame, referencing STAKE_LENGTH from settings.
#[derive(Debug, Clone)]
pub struct Frame {
pub stack: [i32; settings::STAKE_LENGTH as usize],
pub sp: i32,
pub variables: [i32; 100],
pub var_names: [String; 100],
pub var_count: i32,
}
/// Replicates: Instruction* new_instruction(OpCode opcode, const char* operand);
pub fn new_instruction(opcode: OpCode, operand: &str) -> Instruction {
unimplemented!()
}
/// Replicates: void free_instruction(Instruction* instruction);
pub fn free_instruction(_instruction: &mut Instruction) {
unimplemented!()
}
/// Replicates: void eval(Frame* frame, Instruction* instructions, int instr_count);
/// In Rust: instructions is a slice of Instruction, instr_count is the length, or a separate i32.
pub fn eval(frame: &mut Frame, instructions: &[Instruction]) {
unimplemented!()
}
/// Replicates: Frame* init_frame();
pub fn init_frame() -> Frame {
unimplemented!()
}
/// Replicates: void free_frame(Frame* frame);
pub fn free_frame(_frame: &mut Frame) {
unimplemented!()
}