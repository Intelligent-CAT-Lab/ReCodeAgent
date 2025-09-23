use std::fmt;
use crate::{
    em::{self, Program},
    stack,
};
pub const GC_FREQUENCY_IN_TICKS: usize = 64;
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RuntimeError {
    StackUnderflow,
    InvalidAccess,
    DivByZero,
    IncorrectType,
}
impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}
#[derive(Debug)]
pub struct RuntimeResult {
    pub ex: i64,
    pub em: Result<em::Em, RuntimeError>,
}
#[derive(Debug)]
pub struct Env<'a> {
    pub prog: &'a Program,
    pub stack: stack::Stack,
    pub ip: usize,
    pub ex: usize,
    pub tick: usize,
    pub halt: bool,
    pub print: bool,
    pub print_from: usize,
}
impl<'a> Env<'a> {
    pub fn new(stack_cap: usize, popped_cap: usize) -> Self {
        unimplemented!()
    }
    pub fn run(&mut self, prog: &Program) -> RuntimeResult {
        unimplemented!()
    }
    pub fn gc(&mut self) {
        unimplemented!()
    }
}