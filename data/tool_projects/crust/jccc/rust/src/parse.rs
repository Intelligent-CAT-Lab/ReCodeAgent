use crate::lex::Lexer;
use crate::cst::{FunctionDeclaration, ConcreteFileTree, Expression, BlockStatement};
/// Parses a function declaration from the lexer into a FunctionDeclaration object.
pub fn parse_funcdecl(l: &mut Lexer, fd: &mut FunctionDeclaration) -> i32 {
unimplemented!()
}
/// Creates a concrete syntax tree from the lexer.
pub fn make_cst(l: &mut Lexer, tree: &mut ConcreteFileTree) -> i32 {
unimplemented!()
}
/// Parses an expression from the Lexer into an Expression object.
pub fn parse_expr(l: &mut Lexer, ex: &mut Expression) -> i32 {
unimplemented!()
}
/// Parses a file and returns a status code.
pub fn parse(filename: &str) -> i32 {
unimplemented!()
}
/// Parses a simple main function (for testing).
pub fn parse_simple_main_func() -> i32 {
unimplemented!()
}
/// Parses a block statement from the lexer.
pub fn parse_blockstmt(l: &mut Lexer, bs: &mut BlockStatement) -> i32 {
unimplemented!()
}
/// Parses a function call from the lexer into an Expression object.
pub fn parse_funccall(l: &mut Lexer, ex: &mut Expression) -> i32 {
unimplemented!()
}