use crate::gap::{GapBuffer};
pub const DEFAULT_GAP_BUF_CAP: usize = 100;
pub const DEFAULT_CAPACITY: usize = 100;
pub struct TextBuffer {
pub lines: Vec<Option<GapBuffer>>,
pub lines_capacity: usize,
pub cursor_row: usize,
pub cursor_col: usize,
pub cursor_col_moved: bool,
pub last_line_loc: usize,
}
impl TextBuffer {
pub fn create(lines: usize, line_size: usize) -> Option<Self> {
unimplemented!()
}
pub fn destroy(self) {
unimplemented!()
}
pub fn move_cursor(&mut self, row: usize, col: usize) {
unimplemented!()
}
pub fn insert(&mut self, ch: char) -> i32 {
unimplemented!()
}
pub fn backspace(&mut self) -> i32 {
unimplemented!()
}
pub fn new_line(&mut self) -> i32 {
unimplemented!()
}
pub fn get_line(&self, row: usize) -> Option<String> {
unimplemented!()
}
pub fn create_from_file(fp: &std::fs::File) -> Option<Self> {
unimplemented!()
}
}