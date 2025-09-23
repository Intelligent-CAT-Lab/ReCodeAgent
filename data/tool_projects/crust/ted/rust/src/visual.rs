use crate::buffer::{TextBuffer};
pub struct Cursor {
x: usize,
y: usize,
}
pub struct VirtualScreen {
buffer: Vec<char>,
buf_pos: usize,
len: usize,
cursor: Cursor,
width: usize,
height: usize,
render_start_line: usize,
}
impl VirtualScreen {
pub fn screen_append(&mut self, str: &str, size: usize) {
unimplemented!()
}
pub fn required_screen_rows(line_length: usize, screen_width: usize) -> i32 {
unimplemented!()
}
pub fn move_cursor_in_view(buffer: &TextBuffer, screen: &mut VirtualScreen) {
unimplemented!()
}
pub fn draw_editor_window(buffer: &TextBuffer, screen: &mut VirtualScreen) {
unimplemented!()
}
pub fn set_virtual_cursor_position(buffer: &TextBuffer, screen: &mut VirtualScreen) {
unimplemented!()
}
}