use simple_lang::compiler::{compile};
use simple_lang::vm::OpCode;
#[test]
fn test_compiler() {
    let source = "let x = 5 + 3 - 2; dis x;";
    let instructions = compile(source,&mut 0);

    assert_eq!(instructions[0].opcode, OpCode::LOAD_CONST);
    assert_eq!(instructions[0].operand, "5");

    assert_eq!(instructions[1].opcode, OpCode::LOAD_CONST);
    assert_eq!(instructions[1].operand, "3");

    assert_eq!(instructions[2].opcode, OpCode::BINARY_ADD);

    assert_eq!(instructions[3].opcode, OpCode::LOAD_CONST);
    assert_eq!(instructions[3].operand, "2");

    assert_eq!(instructions[4].opcode, OpCode::BINARY_SUB);

    assert_eq!(instructions[5].opcode, OpCode::STORE_NAME);
    assert_eq!(instructions[5].operand, "x");

    assert_eq!(instructions[6].opcode, OpCode::LOAD_NAME);
    assert_eq!(instructions[6].operand, "x");

    assert_eq!(instructions[7].opcode, OpCode::STK_DIS);

    println!("All Tests passed!");
}

fn main() {
}