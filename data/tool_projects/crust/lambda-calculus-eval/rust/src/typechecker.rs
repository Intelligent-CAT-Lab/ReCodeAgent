use crate::{common};
pub struct Type {
pub expr: common::AstNode,
pub type_: String,
pub return_type: String,
}
pub struct TypeEnv {
pub type_: Type,
pub next: Option<Box<TypeEnv>>,
}
pub fn assert_(expr: bool, error_msg: &str) {
unimplemented!()
}
pub fn typecheck(expr: &common::AstNode, env: Option<&TypeEnv>) -> Type {
unimplemented!()
}
pub fn type_equal(a: &Type, b: &Type) -> bool {
unimplemented!()
}
pub fn get_type_from_expr(expr: &common::AstNode) -> String {
unimplemented!()
}
pub fn p_print_type(t: &Type) {
unimplemented!()
}
pub fn create_type(type_: &str, return_type: &str, expr: &common::AstNode) -> Type {
unimplemented!()
}
pub fn parse_function_type(type_: &str) -> Type {
unimplemented!()
}
pub fn expr_type_equal(t: &Type, expr: &common::AstNode) -> bool {
unimplemented!()
}
pub fn add_to_env(env: &mut Option<Box<TypeEnv>>, type_: Type) {
unimplemented!()
}
pub fn lookup_type(env: &TypeEnv, expr: &common::AstNode) -> Type {
unimplemented!()
}