use crate::compiler::{
Token, TOKEN_TYPE_KEYWORD, TOKEN_TYPE_SYMBOL, TOKEN_TYPE_NEWLINE, TOKEN_TYPE_COMMENT,
};
/// Helper to compare token.sval with the given &str.
fn s_eq(opt_s: &Option<String>, val: &str) -> bool {
 unimplemented!()
}
/// Replicates the original bug: "return token->type = TOKEN_TYPE_KEYWORD && S_EQ(token->sval, value);"
/// We do this in Rust by assigning token.r#type = if eq { 1 } else { 0 } and then returning eq.
pub fn token_is_keyword(token: &mut Token, value: &str) -> bool {
    unimplemented!()
}
/// Return true if token is a symbol token with the given char c.
pub fn token_is_symbol(token: &Token, c: char) -> bool {
    unimplemented!()
}
/// Return true if token is NEWLINE or COMMENT or the symbol '\'.
pub fn token_is_nl_or_comment_or_newline_separator(token: &Token) -> bool {
    unimplemented!()
}