use crate::{common, hash_table};
use std::fs::File;
pub fn parse_token(token: char) -> common::tokens_t {
unimplemented!()
}
pub fn p_print_token(token: common::tokens_t) {
unimplemented!()
}
pub fn p_print_astNode_type(n: &common::AstNode) {
unimplemented!()
}
pub fn print_ast(node: &common::AstNode) {
unimplemented!()
}
pub fn is_variable(token: char) -> bool {
unimplemented!()
}
pub fn peek(in_: &mut File) -> char {
unimplemented!()
}
pub fn peek_print(in_: &mut File, n: usize) {
unimplemented!()
}
pub fn consume(t: common::tokens_t, in_: &mut File, expected: &str) {
unimplemented!()
}
pub fn create_variable(name: &str, type_: &str) -> common::AstNode {
unimplemented!()
}
pub fn create_application(function: &common::AstNode, argument: &common::AstNode) -> common::AstNode {
unimplemented!()
}
pub fn create_lambda(variable: &str, body: &common::AstNode, type_: &str) -> common::AstNode {
unimplemented!()
}
pub fn alpha_convert(old: &str) -> String {
unimplemented!()
}
pub fn is_used(table: &hash_table::HashTable, variable: &str) -> bool {
unimplemented!()
}
pub fn parse_space_chars(in_: &mut File) {
unimplemented!()
}
pub fn parse_lambda(table: &mut hash_table::HashTable, in_: &mut File) -> common::AstNode {
unimplemented!()
}
pub fn parse_expression(table: &mut hash_table::HashTable, in_: &mut File) -> common::AstNode {
unimplemented!()
}
pub fn parse_import(table: &mut hash_table::HashTable, in_: &mut File) {
unimplemented!()
}
pub fn parse_definition(table: &mut hash_table::HashTable, in_: &mut File) {
unimplemented!()
}
pub fn is_uppercase(c: char) -> bool {
unimplemented!()
}
pub fn parse_type_definition(types_table: &mut hash_table::HashTable, in_: &mut File) {
unimplemented!()
}
pub fn parse_type(types_table: &mut hash_table::HashTable, in_: &mut File) -> String {
unimplemented!()
}
pub fn parse_variable(in_: &mut File) -> String {
unimplemented!()
}
pub fn expect(expected: &str, received: char) {
unimplemented!()
}
pub fn free_ast(node: &mut common::AstNode) {
unimplemented!()
}