use worsp::worsp::ParseState;
use worsp::worsp::TokenKind;
use worsp::worsp::next;

#[test]
fn next_single_char_symbol() {
let source = "a";
let mut state = ParseState { token: None, pos: 0 };
next(source, &mut state);
assert_eq!(state.token.as_ref().unwrap().kind, TokenKind::Symbol);
assert_eq!(state.token.as_ref().unwrap().str, "a");
}

#[test]
fn next_multiple_char_symbol() {
let source = "aaaa";
let mut state = ParseState { token: None, pos: 0 };
next(source, &mut state);
assert_eq!(state.token.as_ref().unwrap().kind, TokenKind::Symbol);
assert_eq!(state.token.as_ref().unwrap().str, "aaaa");
}

#[test]
fn next_paren_and_digit() {
let source = "(1)";
let mut state = ParseState { token: None, pos: 0 };
next(source, &mut state);
assert_eq!(state.token.as_ref().unwrap().kind, TokenKind::LParen);
next(source, &mut state);
assert_eq!(state.token.as_ref().unwrap().kind, TokenKind::Digit);
assert_eq!(state.token.as_ref().unwrap().val, 1);
next(source, &mut state);
assert_eq!(state.token.as_ref().unwrap().kind, TokenKind::RParen);
}

#[test]
fn next_if_and_set() {
let source = "(if (set a 1) (set b 2) (set c 3))";
let mut state = ParseState { token: None, pos: 0 };
next(source, &mut state);
assert_eq!(state.token.as_ref().unwrap().kind, TokenKind::LParen);
next(source, &mut state);
assert_eq!(state.token.as_ref().unwrap().kind, TokenKind::Symbol);
assert_eq!(state.token.as_ref().unwrap().str, "if");
next(source, &mut state);
assert_eq!(state.token.as_ref().unwrap().kind, TokenKind::LParen);
next(source, &mut state);
assert_eq!(state.token.as_ref().unwrap().kind, TokenKind::Symbol);
assert_eq!(state.token.as_ref().unwrap().str, "set");
next(source, &mut state);
assert_eq!(state.token.as_ref().unwrap().kind, TokenKind::Symbol);
assert_eq!(state.token.as_ref().unwrap().str, "a");
next(source, &mut state);
assert_eq!(state.token.as_ref().unwrap().kind, TokenKind::Digit);
assert_eq!(state.token.as_ref().unwrap().val, 1);
next(source, &mut state);
assert_eq!(state.token.as_ref().unwrap().kind, TokenKind::RParen);
next(source, &mut state);
assert_eq!(state.token.as_ref().unwrap().kind, TokenKind::LParen);
next(source, &mut state);
assert_eq!(state.token.as_ref().unwrap().kind, TokenKind::Symbol);
assert_eq!(state.token.as_ref().unwrap().str, "set");
next(source, &mut state);
assert_eq!(state.token.as_ref().unwrap().kind, TokenKind::Symbol);
assert_eq!(state.token.as_ref().unwrap().str, "b");
next(source, &mut state);
assert_eq!(state.token.as_ref().unwrap().kind, TokenKind::Digit);
assert_eq!(state.token.as_ref().unwrap().val, 2);
next(source, &mut state);
assert_eq!(state.token.as_ref().unwrap().kind, TokenKind::RParen);
next(source, &mut state);
assert_eq!(state.token.as_ref().unwrap().kind, TokenKind::LParen);
next(source, &mut state);
assert_eq!(state.token.as_ref().unwrap().kind, TokenKind::Symbol);
assert_eq!(state.token.as_ref().unwrap().str, "set");
next(source, &mut state);
assert_eq!(state.token.as_ref().unwrap().kind, TokenKind::Symbol);
assert_eq!(state.token.as_ref().unwrap().str, "c");
next(source, &mut state);
assert_eq!(state.token.as_ref().unwrap().kind, TokenKind::Digit);
assert_eq!(state.token.as_ref().unwrap().val, 3);
next(source, &mut state);
assert_eq!(state.token.as_ref().unwrap().kind, TokenKind::RParen);
next(source, &mut state);
assert_eq!(state.token.as_ref().unwrap().kind, TokenKind::RParen);
}


fn main() {
    println!("Hello, world!");
}
