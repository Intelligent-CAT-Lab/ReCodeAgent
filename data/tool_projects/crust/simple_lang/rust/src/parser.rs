use crate::{token, ast};
pub const SIMPLE_LANG_PARSER_H: bool = true;
/// This would represent a global tokens pointer in C:
/// Token* tokens;
/// Rust does not use global mutable state in the same way, so it is omitted.
/// Replicates: Token* consume();
pub fn consume() -> token::Token {
unimplemented!()
}
/// Replicates: ASTNode* parse_statement();
pub fn parse_statement() -> Box<ast::ASTNode> {
unimplemented!()
}
/// Replicates: ASTNode* parse_primary();
pub fn parse_primary() -> Box<ast::ASTNode> {
unimplemented!()
}
/// Replicates: ASTNode* parse_expression();
pub fn parse_expression() -> Box<ast::ASTNode> {
unimplemented!()
}
/// Replicates: ASTNode** parse(Token* tokens);
/// In Rust: returns a vector of Box<ASTNode>.
pub fn parse(tokens_array: &[token::Token]) -> Vec<Box<ast::ASTNode>> {
unimplemented!()
}
/// Replicates: Token* lookahead();
pub fn lookahead() -> token::Token {
unimplemented!()
}