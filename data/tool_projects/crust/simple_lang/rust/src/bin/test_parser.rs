use simple_lang::lexer::tokenize;
use simple_lang::parser::{parse};
use simple_lang::token::TokenType;
#[test]
fn test_parse() {
    let source = "let x = 5 + 3 - 2; let y = x + 1; dis x + 1;";
    
    let tokens = tokenize(source);
    let ast_nodes = parse(&tokens);

    assert_eq!(ast_nodes[0].type_, TokenType::TOKEN_LET);
    assert_eq!(ast_nodes[0].left.as_ref().unwrap().type_, TokenType::TOKEN_MINUS);
    assert_eq!(ast_nodes[0].left.as_ref().unwrap().left.as_ref().unwrap().type_, TokenType::TOKEN_PLUS);
    assert_eq!(ast_nodes[0].left.as_ref().unwrap().left.as_ref().unwrap().left.as_ref().unwrap().type_, TokenType::TOKEN_INT);
    assert_eq!(ast_nodes[0].left.as_ref().unwrap().left.as_ref().unwrap().right.as_ref().unwrap().type_, TokenType::TOKEN_INT);
    assert_eq!(ast_nodes[0].left.as_ref().unwrap().right.as_ref().unwrap().type_, TokenType::TOKEN_INT);

    assert_eq!(ast_nodes[1].type_, TokenType::TOKEN_LET);
    assert_eq!(ast_nodes[1].left.as_ref().unwrap().type_, TokenType::TOKEN_PLUS);
    assert_eq!(ast_nodes[1].left.as_ref().unwrap().left.as_ref().unwrap().type_, TokenType::TOKEN_IDENTIFIER);
    assert_eq!(ast_nodes[1].left.as_ref().unwrap().right.as_ref().unwrap().type_, TokenType::TOKEN_INT);

    assert_eq!(ast_nodes[2].type_, TokenType::TOKEN_DIS);
    assert_eq!(ast_nodes[2].left.as_ref().unwrap().type_, TokenType::TOKEN_PLUS);
    assert_eq!(ast_nodes[2].left.as_ref().unwrap().left.as_ref().unwrap().type_, TokenType::TOKEN_IDENTIFIER);
    assert_eq!(ast_nodes[2].left.as_ref().unwrap().right.as_ref().unwrap().type_, TokenType::TOKEN_INT);

    println!("All Tests passed!");
}

fn main() {
    
}