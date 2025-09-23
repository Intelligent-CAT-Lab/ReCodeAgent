use std::fmt;
#[derive(Debug, PartialEq, Eq)]
pub enum tokens_t{
    L_PAREN,
    R_PAREN,
    LAMBDA,
    DOT,
    VARIABLE,
    ERROR,
    WHITESPACE,
    NEWLINE,
    EQ,
    QUOTE,
    COLON,
} 
#[derive(Debug, PartialEq, Eq)]
pub enum AstNodeType {
LAMBDA_EXPR,
APPLICATION,
VAR,
DEFINITION,
}
#[derive(Debug)]
pub struct LambdaExpression {
pub parameter: String,
pub type_: String,
pub body: Option<Box<AstNode>>,
}
#[derive(Debug)]
pub struct Application {
pub function: Option<Box<AstNode>>,
pub argument: Option<Box<AstNode>>,
}
#[derive(Debug)]
pub struct Variable {
pub name: String,
pub type_: String,
}
#[derive(Debug)]
pub enum AstNodeUnion {
LambdaExpr(LambdaExpression),
Application(Application),
Variable(Variable),
}
#[derive(Debug)]
pub struct AstNode {
pub type_: AstNodeType,
pub node: AstNodeUnion,
}
impl Default for AstNode {
    fn default() -> Self {
        unimplemented!()
    }
}
pub fn set_verbose(verbose: bool) {
unimplemented!()
}
pub fn print_ast_verbose(n: &AstNode) {
unimplemented!()
}
pub fn print_verbose(format: &str, args: fmt::Arguments) {
unimplemented!()
}
pub fn error(msg: &str, file: &str, line: i32, func: &str) {
unimplemented!()
}
pub fn format(fmt: &str, args: fmt::Arguments) -> String {
unimplemented!()
}
pub fn append_to_buffer(buffer: &mut String, str: &str) {
unimplemented!()
}
pub fn append_ast_to_buffer(buffer: &mut String, node: &AstNode) {
unimplemented!()
}
pub fn ast_to_string(node: &AstNode) -> String {
unimplemented!()
}