use crate::libutf_utf::*;
pub struct Utf8String<'a> {
pub data: &'a [Utf8],
pub len: usize,
pub cap: usize,
}
pub struct Utf8StringView<'a> {
pub data: &'a [Utf8],
pub len: usize,
}
impl Utf8String<'_> {
pub fn new() -> Self {
unimplemented!()
}
pub fn init(&mut self) {
    unimplemented!()
}
pub fn destroy(&mut self) {
    unimplemented!()
}
pub fn reserve(&mut self, len: usize) -> Result<(), ()> {
    unimplemented!()
}
pub fn shrink_to_fit(&mut self) -> Result<(), ()> {
    unimplemented!()
}
pub fn clear(&mut self) {
    unimplemented!()
}
pub fn is_empty(&self) -> bool {
    unimplemented!()
}
pub fn append(&mut self, other: &Utf8String) -> Result<(), ()> {
    unimplemented!()
}
pub fn append_view(&mut self, view: &Utf8StringView) -> Result<(), ()> {
    unimplemented!()
}
pub fn append_character(&mut self, c: Utf8) -> Result<(), ()> {
    unimplemented!()
}
pub fn append_literal(&mut self, literal: &[Utf8]) -> Result<(), ()> {
    unimplemented!()
}
pub fn prepend(&mut self, other: &Utf8String) -> Result<(), ()> {
    unimplemented!()
}
pub fn prepend_view(&mut self, view: &Utf8StringView) -> Result<(), ()> {
    unimplemented!()
}
pub fn prepend_character(&mut self, c: Utf8) -> Result<(), ()> {
    unimplemented!()
}
pub fn prepend_literal(&mut self, literal: &[Utf8]) -> Result<(), ()> {
    unimplemented!()
}
pub fn insert(&mut self, pos: usize, other: &Utf8String) -> Result<(), ()> {
    unimplemented!()
}
pub fn insert_view(&mut self, pos: usize, view: &Utf8StringView) -> Result<(), ()> {
    unimplemented!()
}
pub fn insert_character(&mut self, pos: usize, c: Utf8) -> Result<(), ()> {
    unimplemented!()
}
pub fn insert_literal(&mut self, pos: usize, literal: &[Utf8]) -> Result<(), ()> {
    unimplemented!()
}
pub fn replace(&mut self, pos: usize, len: usize, replacement: &Utf8String) -> Result<(), ()> {
    unimplemented!()
}
pub fn replace_view(&mut self, pos: usize, len: usize, replacement: &Utf8StringView) -> Result<(), ()> {
    unimplemented!()
}
pub fn replace_character(&mut self, pos: usize, len: usize, c: Utf8) -> Result<(), ()> {
    unimplemented!()
}
pub fn replace_literal(&mut self, pos: usize, len: usize, literal: &[Utf8]) -> Result<(), ()> {
    unimplemented!()
}
pub fn erase(&mut self, pos: usize, len: usize) -> Result<(), ()> {
    unimplemented!()
}
pub fn concat(&self, other: &Utf8String) -> Result<Utf8String, ()> {
    unimplemented!()
}
pub fn concat_view(&self, other: &Utf8StringView) -> Result<Utf8String, ()> {
    unimplemented!()
}
pub fn concat_character(&self, c: Utf8) -> Result<Utf8String, ()> {
    unimplemented!()
}
pub fn concat_literal(&self, literal: &[Utf8]) -> Result<Utf8String, ()> {
    unimplemented!()
}
pub fn compare(&self, other: &Utf8String) -> i32 {
    unimplemented!()
}
pub fn compare_literal(&self, literal: &[Utf8]) -> i32 {
    unimplemented!()
}
pub fn substring(&self, start: usize, end: usize) -> Utf8StringView {
    unimplemented!()
}
pub fn substring_copy(&self, start: usize, end: usize) -> Result<Utf8String, ()> {
    unimplemented!()
}
pub fn index_of_character(&self, pos: usize, c: Utf8) -> Option<usize> {
    unimplemented!()
}
pub fn last_index_of_character(&self, pos: usize, c: Utf8) -> Option<usize> {
    unimplemented!()
}
}
impl<'a> Utf8StringView<'a> {
pub fn is_empty(&self) -> bool {
    unimplemented!()
}
pub fn compare(&self, other: &Utf8StringView) -> i32 {
    unimplemented!()
}
pub fn compare_literal(&self, literal: &[Utf8]) -> i32 {
    unimplemented!()
}
pub fn substring(&self, start: usize, end: usize) -> Utf8StringView<'a> {
    unimplemented!()
}
pub fn substring_copy(&self, start: usize, end: usize) -> Result<Utf8String, ()> {
    unimplemented!()
}
pub fn index_of_character(&self, pos: usize, c: Utf8) -> Option<usize> {
    unimplemented!()
}
pub fn last_index_of_character(&self, pos: usize, c: Utf8) -> Option<usize> {
    unimplemented!()
}
}