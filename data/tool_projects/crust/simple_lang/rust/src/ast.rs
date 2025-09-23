use crate::token;
pub const SIMPLE_LANG_AST_H: bool = true;
/// Safe, idiomatic representation of the ASTNode in Rust.
/// In C:
/// typedef struct ASTNode {
///     TokenType type;
///     struct ASTNode* left;
///     struct ASTNode* right;
///     char* value;
/// } ASTNode;
#[derive(Debug, Clone)]
pub struct ASTNode {
pub type_: token::TokenType,
pub left: Option<Box<ASTNode>>,
pub right: Option<Box<ASTNode>>,
pub value: String,
}
/// Replicates the C signature:
/// ASTNode* new_ast_node(TokenType type, const char* value);
pub fn new_ast_node(type_: token::TokenType, value: &str) -> Box<ASTNode> {
unimplemented!()
}
/// Replicates the C signature:
/// void free_ast_node(ASTNode* node);
pub fn free_ast_node(node: Box<ASTNode>) {
unimplemented!()
}