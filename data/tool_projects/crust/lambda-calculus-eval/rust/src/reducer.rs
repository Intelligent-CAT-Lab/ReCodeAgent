use crate::{common, hash_table, parser, config};
pub const SIZE: usize = 122;
pub fn set_reduction_order(t: config::reduction_order_t) {
unimplemented!()
}
pub fn print_reduction_order(t: config::reduction_order_t) {
unimplemented!()
}
pub fn reduce(table: &mut hash_table::HashTable, n: &common::AstNode) -> common::AstNode {
unimplemented!()
}
pub fn expand_definitions(table: &mut hash_table::HashTable, n: &common::AstNode) {
unimplemented!()
}
pub fn replace(n: &mut common::AstNode, old: &str, new_name: &str) {
unimplemented!()
}
pub fn reduce_ast(table: &mut hash_table::HashTable, n: &common::AstNode) -> common::AstNode {
unimplemented!()
}
pub fn substitute(expression: &common::AstNode, variable: &str, replacement: &common::AstNode) -> common::AstNode {
unimplemented!()
}
pub fn deepcopy(n: &common::AstNode) -> common::AstNode {
unimplemented!()
}
pub fn deepcopy_application(function: &common::AstNode, argument: &common::AstNode) -> common::AstNode {
unimplemented!()
}
pub fn deepcopy_lambda_expr(parameter: &str, body: &common::AstNode, type_: &str) -> common::AstNode {
unimplemented!()
}
pub fn deepcopy_var(name: &str, type_: &str) -> common::AstNode {
unimplemented!()
}