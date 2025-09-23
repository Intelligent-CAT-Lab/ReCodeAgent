use crate::token;
pub const SIMPLE_LANG_LEXER_H: bool = true;
/// Replicates: Token* tokenize(const char* source);
/// In Rust: returns a vector of Tokens.
pub fn tokenize(source: &str) -> Vec<token::Token> {
unimplemented!()
}
/// Replicates: void free_token(Token* token);
pub fn free_token(_token: token::Token) {
unimplemented!()
}
/// Replicates: Token* new_token(TokenType type, const char* value);
/// In Rust: returns a new Token struct.
pub fn new_token(type_: token::TokenType, value: &str) -> token::Token {
unimplemented!()
}