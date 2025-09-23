
use simple_lang::lexer::tokenize;
use simple_lang::token::{TokenType, Token};
#[test]
fn test_tokenize() {
    let source = "let x = 5 + 3 - 2; dis x;";
    let tokens:Vec<Token> = tokenize(source);

    assert_eq!(tokens[0].token_type, TokenType::TOKEN_LET);
    assert_eq!(tokens[1].token_type, TokenType::TOKEN_IDENTIFIER);
    assert_eq!(tokens[2].token_type, TokenType::TOKEN_ASSIGN);
    assert_eq!(tokens[3].token_type, TokenType::TOKEN_INT);
    assert_eq!(tokens[4].token_type, TokenType::TOKEN_PLUS);
    assert_eq!(tokens[5].token_type, TokenType::TOKEN_INT);
    assert_eq!(tokens[6].token_type, TokenType::TOKEN_MINUS);
    assert_eq!(tokens[7].token_type, TokenType::TOKEN_INT);
    assert_eq!(tokens[8].token_type, TokenType::TOKEN_SEMICOLON);
    assert_eq!(tokens[9].token_type, TokenType::TOKEN_DIS);
    assert_eq!(tokens[10].token_type, TokenType::TOKEN_IDENTIFIER);
    assert_eq!(tokens[11].token_type, TokenType::TOKEN_SEMICOLON);
    assert_eq!(tokens[12].token_type, TokenType::TOKEN_EOF);

    println!("All Tests passed!");
}
fn main() {
}