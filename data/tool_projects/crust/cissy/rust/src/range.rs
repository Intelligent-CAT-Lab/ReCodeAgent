pub mod range {
#[derive(Debug, Clone, PartialEq)]
pub enum RangeType {
Empty,
Single,
StartEnd,
GreaterEqual,
}
#[derive(Debug, Clone)]
pub struct RangeElement {
pub start: u32,
pub end: u32,
pub rangetype: RangeType,
pub next: Option<Box<RangeElement>>,
}
impl RangeElement {
pub fn new() -> Self {
    unimplemented!();
}
pub fn add_single(num: u32, start_of_list: Option<Box<RangeElement>>) -> Option<Box<RangeElement>> {
unimplemented!();
}
pub fn add_start_end(start: u32, end: u32, start_of_list: Option<Box<RangeElement>>) -> Option<Box<RangeElement>> {
unimplemented!();
}
pub fn add_greater_equal(num: u32, start_of_list: Option<Box<RangeElement>>) -> Option<Box<RangeElement>> {
unimplemented!();
}
pub fn contains_num(num: u32, start_of_list: &Option<Box<RangeElement>>) -> bool {
unimplemented!();
}
pub fn to_string(&self, buf: &mut String, bufsize: usize) -> &str {
unimplemented!();
}
pub fn list_to_string<'a>(buf: &'a mut String, bufsize: usize, start_of_list: &'a Option<Box<RangeElement>>) -> &'a str {
unimplemented!();
}
pub fn parse_int_ranges(text: &str) -> Option<Box<RangeElement>> {
unimplemented!();
}
}
}