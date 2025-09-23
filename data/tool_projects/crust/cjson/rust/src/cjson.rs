use std::collections::HashMap;
use std::fmt;
#[derive(Debug, Clone, PartialEq)]
pub enum CJson {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<CJson>),
    Object(HashMap<String, CJson>),
}
#[derive(Debug, Clone)]
pub enum CJsonError {
    UnexpectedEOF { pos: usize },
    UnexpectedToken { ch: char, pos: usize },
    InvalidLiteral { expected: &'static str, pos: usize },
    InvalidNumber { pos: usize },
    InvalidEscape { pos: usize },
    InvalidUnicodeEscape { pos: usize },
    ExpectedColon { pos: usize },
    ExpectedCommaOrEnd { pos: usize },
}
impl fmt::Display for CJsonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}
impl std::error::Error for CJsonError {}
struct Parser<'a> {
    input: &'a str,
    pos: usize,
}
impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        unimplemented!()
    }
    fn peek(&self) -> Option<char> {
        unimplemented!()
    }
    fn next_char(&mut self) -> Option<char> {
        unimplemented!()
    }
    fn take_while<F>(&mut self, mut predicate: F) -> &'a str
    where
        F: FnMut(char) -> bool,
    {
        unimplemented!()
    }
    fn skip_whitespace(&mut self) {
        unimplemented!()
    }
    fn expect_char(&mut self, expected: char) -> Result<(), CJsonError> {
        unimplemented!()
    }
    fn parse_value(&mut self) -> Result<CJson, CJsonError> {
        unimplemented!()
    }
    fn parse_null(&mut self) -> Result<CJson, CJsonError> {
        unimplemented!()
    }
    fn parse_bool(&mut self) -> Result<CJson, CJsonError> {
        unimplemented!()
    }
    fn parse_number(&mut self) -> Result<CJson, CJsonError> {
        unimplemented!()
    }
    fn parse_string(&mut self) -> Result<String, CJsonError> {
        unimplemented!()
    }
    fn parse_array(&mut self) -> Result<CJson, CJsonError> {
        unimplemented!()
    }
    fn parse_object(&mut self) -> Result<CJson, CJsonError> {
        unimplemented!()
    }
}
pub fn parse(input: &str, require_end: bool) -> Result<CJson, CJsonError> {
    unimplemented!()
}
fn escape_string(s: &str) -> String {
    unimplemented!()
}
fn write_json_compact(f: &mut impl fmt::Write, value: &CJson) -> fmt::Result {
    unimplemented!()
}
fn write_json_pretty(f: &mut impl fmt::Write, value: &CJson, indent: usize) -> fmt::Result {
    unimplemented!()
}
impl CJson {
    pub fn print_unformatted(&self) -> String {
        unimplemented!()
    }
    pub fn print_formatted(&self) -> String {
        unimplemented!()
    }
    pub fn get_array_size(&self) -> Option<usize> {
        unimplemented!()
    }
    pub fn get_array_item(&self, index: usize) -> Option<&CJson> {
        unimplemented!()
    }
    pub fn get_object_item(&self, key: &str) -> Option<&CJson> {
        unimplemented!()
    }
    pub fn create_null() -> Self {
        unimplemented!()
    }
    pub fn create_bool(b: bool) -> Self {
        unimplemented!()
    }
    pub fn create_number(n: f64) -> Self {
        unimplemented!()
    }
    pub fn create_string<S: Into<String>>(s: S) -> Self {
        unimplemented!()
    }
    pub fn create_array() -> Self {
        unimplemented!()
    }
    pub fn create_object() -> Self {
        unimplemented!()
    }
    pub fn add_item_to_array(&mut self, item: CJson) -> Result<(), &'static str> {
        unimplemented!()
    }
    pub fn add_item_to_object<S: Into<String>>(
        &mut self,
        key: S,
        value: CJson,
    ) -> Result<(), &'static str> {
        unimplemented!()
    }
}
impl fmt::Display for CJson {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}