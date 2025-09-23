use crate::token::{Token, TokenType};
pub const TOKEN_PUTBACKS: usize = 5;
/// Represents a lexer for tokenizing input.
#[derive(Debug)]
pub struct Lexer {
/// An optional file; in C this would be a FILE*, here we keep it as a File for demonstration.
pub fp: Option<std::fs::File>,
pub current_file: String,
pub buffer: [u8; 1],
pub position: i64,
pub last_column: i32,
pub column: i32,
pub line: i32,
pub unlexed: [Token; TOKEN_PUTBACKS],
pub unlexed_count: u32,
}
/// Gets the next character from the lexer.
pub fn lexer_getchar(l: &mut Lexer) -> i32 {
unimplemented!()
}
/// Retrieves the next token from the lexer.
pub fn real_lex(l: &mut Lexer, t: &mut Token) -> i32 {
unimplemented!()
}
/// Un-gets (pushes back) the last character.
pub fn lexer_ungetchar(l: &mut Lexer) -> i32 {
unimplemented!()
}
/// Determines a token type from a multi-character sequence.
pub fn ttype_many_chars(contents: &str) -> TokenType {
unimplemented!()
}
/// Tests the function that identifies token types by name.
pub fn test_ttype_name() -> i32 {
unimplemented!()
}
/// Determines a token type from a single character.
pub fn ttype_one_char(c: char) -> TokenType {
unimplemented!()
}
/// Returns the name of a token type as a string.
pub fn ttype_name(tt: TokenType) -> String {
unimplemented!()
}
/// Checks if the provided operator sequence is valid.
pub fn valid_operator_sequence(op: &str) -> i32 {
unimplemented!()
}
/// Main lex function to tokenize input into a given Token.
pub fn lex(l: &mut Lexer, token: &mut Token) -> i32 {
unimplemented!()
}
/// Tests the function that determines token types for a single character.
pub fn test_ttype_one_char() -> i32 {
unimplemented!()
}
/// Checks if a character starts an operator sequence.
pub fn starts_operator(c: char) -> i32 {
unimplemented!()
}
/// Tests the function that determines token types from a string.
pub fn test_ttype_from_string() -> i32 {
unimplemented!()
}
/// Derives a TokenType from a string input.
pub fn ttype_from_string(contents: &str) -> TokenType {
unimplemented!()
}
/// Checks if character c is in the string s.
pub fn in_string(c: char, s: &str) -> i32 {
unimplemented!()
}
/// Tests the function that determines token types from multiple characters.
pub fn test_ttype_many_chars() -> i32 {
unimplemented!()
}
/// Skips characters until the next token is found.
pub fn skip_to_token(l: &mut Lexer) -> i32 {
unimplemented!()
}
/// Pushes a token back into the lexer's buffer.
pub fn unlex(l: &mut Lexer, t: &Token) -> i32 {
unimplemented!()
}
/// Checks if c is a valid numeric or identifier character.
pub fn is_valid_numeric_or_id_char(c: char) -> i32 {
unimplemented!()
}