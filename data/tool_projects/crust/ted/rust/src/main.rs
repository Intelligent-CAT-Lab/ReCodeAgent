use ted::buffer::{TextBuffer};
use ted::defs::{panic};
pub struct EditorState {
orig_termios: termios::Termios,
file_name: Option<String>,
file_path: Option<String>,
flushed: bool,
current_buffer: TextBuffer,
screen: VirtualScreen,
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
pub struct Cursor {
x: usize,
y: usize,
}
impl EditorState {
pub fn initialize(argc: i32, argv: Vec<String>) {
unimplemented!()
}
pub fn cleanup() {
unimplemented!()
}
pub fn set_window_size() {
unimplemented!()
}
pub fn disable_raw_mode() {
unimplemented!()
}
pub fn enable_raw_mode() {
unimplemented!()
}
pub fn render_screen() {
unimplemented!()
}
pub fn draw_screen() {
unimplemented!()
}
pub fn draw_status_line(line_size: usize) {
unimplemented!()
}
pub fn up_arrow() {
unimplemented!()
}
pub fn down_arrow() {
unimplemented!()
}
pub fn right_arrow() {
unimplemented!()
}
pub fn left_arrow() {
unimplemented!()
}
pub fn read_char() -> i32 {
unimplemented!()
}
pub fn process_keypress() {
unimplemented!()
}
pub fn flush_buffer_to_file() -> i32 {
unimplemented!()
}
pub fn load_file_and_initialize_buffer() -> i32 {
unimplemented!()
}
}
pub fn main(){}