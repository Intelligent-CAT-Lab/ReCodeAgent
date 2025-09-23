pub const SIMPLE_LANG_TOKEN_H: bool = true;
/// Replicates the C enum:
/// typedef enum {
///     TOKEN_INT,
///     TOKEN_IDENTIFIER,
///     TOKEN_ASSIGN,
///     TOKEN_PLUS,
///     TOKEN_MINUS,
///     TOKEN_SEMICOLON,
///     TOKEN_LET,
///     TOKEN_EOF,
///     TOKEN_DIS
/// } TokenType;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
TOKEN_INT,
TOKEN_IDENTIFIER,
TOKEN_ASSIGN,
TOKEN_PLUS,
TOKEN_MINUS,
TOKEN_SEMICOLON,
TOKEN_LET,
TOKEN_EOF,
TOKEN_DIS,
}
/// Replicates the C struct:
/// typedef struct {
///     TokenType type;
///     char* value;
/// } Token;
#[derive(Debug, Clone)]
pub struct Token {
pub token_type: TokenType,
pub value: String,
}
/// Replicates: Token* new_token(TokenType type, const char* value);
pub fn new_token(type_: TokenType, value: &str) -> Token {
unimplemented!()
}
/// Replicates: void free_token(Token* token);
pub fn free_token(_token: Token) {
unimplemented!()
}