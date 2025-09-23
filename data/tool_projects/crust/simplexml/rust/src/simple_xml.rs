use crate::simple_vector::Vector;

const BEGIN_TAG_TOKEN: char = '<';
const END_TAG_TOKEN: char = '>';
const SPLASH_TOKEN: char = '/';

pub struct XMLElement {
    pub tag_name: String,
    pub value: String,
    pub parent: (),                   
    pub children: Vector<XMLElement>, // owns children
}

#[derive(Debug, Clone, PartialEq)]
pub enum XMLTokenType {
    BeginOpenTag,  // “<”
    BeginCloseTag, // “</”
    EndTag,        // “>”
    Text,          // non‑empty, trimmed text
}

pub struct XMLToken {
    pub token_type: XMLTokenType,
    pub data: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParseState {
    State1,
    State2,
    State3,
    State4,
    State5,
    State6,
    State7,
    State8,
    StateError,
}

pub struct StackElement {
    element: XMLElement,
    depth: usize,
}

impl StackElement {
    pub fn new(element: XMLElement, depth: usize) -> Self {
        unimplemented!() 
    }
    pub fn release(&mut self) {
        unimplemented!()
    }
}

impl XMLElement {
    pub fn new(tag_name: String, value: String) -> XMLElement {
        unimplemented!()
    }
}

pub struct XMLParser {
    input: String,
    position: usize,
    depth: usize,
    state: ParseState,

    tag_stack: Vector<String>,
    value_stack: Vector<String>,
    element_stack: Vector<StackElement>,
}

impl XMLParser {
    pub fn new() -> Self {
        unimplemented!()
    }

    pub fn parse(&mut self, text: &str) -> Result<XMLElement, String> {
        unimplemented!()
    }

    fn get_next_token(&mut self) -> Option<XMLToken> {
        unimplemented!()
    }

    fn get_text_token(&self, mut from: usize, mut to: usize) -> XMLToken {
        unimplemented!()
    }

    fn translate(state: ParseState, token: &XMLTokenType) -> ParseState {
        unimplemented!()
    }

    fn release(&mut self) {
        unimplemented!()
    }
}

pub fn parse_xml_from_text(text: &str) -> Result<XMLElement, String> {
    unimplemented!()
}
