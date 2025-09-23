use crate::{parser, throw, slothvm};
pub struct ListNode {
pub data: i32,
pub next: Option<Box<ListNode>>,
}
pub struct Stack {
pub top: Option<Box<ListNode>>,
pub bottom: Option<Box<ListNode>>,
}
impl Stack {
pub fn new() -> Self {
unimplemented!()
}
pub fn push(&mut self, x: i32) {
unimplemented!()
}
pub fn is_empty(&self) -> bool {
    unimplemented!()
}
pub fn pop(&mut self) -> Option<i32> {
unimplemented!()
}
pub fn peek(&self, pos: usize) -> Option<i32> {
    unimplemented!()
}
pub fn print(&self) {
    unimplemented!()
}
}
impl Drop for Stack {
fn drop(&mut self) {
    unimplemented!()
}
}