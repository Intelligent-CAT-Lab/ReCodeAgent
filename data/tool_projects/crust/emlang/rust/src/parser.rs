use core::fmt;
use crate::em;
pub const PARSER_MAX_TOKEN_LENGTH: usize = 1024;
pub const PARSER_MAX_NESTS: usize = 256;
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParserError {
    UnexpectedEscape,
    UnknownEscape,
    UnterminatedQuotes,
    UnexpectedEnd,
    IllegalPrintNest,
    ExpectedEnd,
}
impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}
#[derive(Debug)]
pub struct ParserResult {
    pub path: String,
    pub row: usize,
    pub col: usize,
    pub prog: Result<em::Program, ParserError>,
}
#[derive(Debug)]
pub struct Parser {
    pub path: String,
    pub row: usize,
    pub col: usize,
    pub from_file: bool,
    pub input: String,
    pub ch: i32,
    pub pos: usize,
    pub tok: String,
    pub tok_len: usize,
    pub prog: em::Program,
}
impl Parser {
    pub fn new() -> Self {
        unimplemented!()
    }
    pub fn load_mem(&mut self, input: &str) {
        unimplemented!()
    }
    pub fn load_file(&mut self, path: &str) -> i32 {
        unimplemented!()
    }
    pub fn parse(&mut self) -> ParserResult {
        unimplemented!()
    }
    pub fn cross_ref(&mut self) -> ParserResult {
        unimplemented!()
    }
    pub fn advance(&mut self) {
        unimplemented!()
    }
    pub fn parse_plain(&mut self) -> ParserResult {
        unimplemented!()
    }
    pub fn parse_quotes(&mut self) -> ParserResult {
        unimplemented!()
    }
    pub fn parse_next(&mut self) -> ParserResult {
        unimplemented!()
    }
}