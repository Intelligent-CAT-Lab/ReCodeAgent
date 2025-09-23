pub const RED: &str = "\x1b[0;31m";
pub const WHITE: &str = "\x1b[0;37m";
pub const COLOR_OFF: &str = "\x1b[0m";
pub const PURPLE: &str = "\x1b[0;35m";
pub const GREEN: &str = "\x1b[0;32m";
pub const YELLOW: &str = "\x1b[0;33m";
pub const BLUE: &str = "\x1b[0;34m";
pub const RIGHT_MARGIN: usize = 5;
pub const CYAN: &str = "\x1b[0;36m";
pub const DEFAULT_NAME: &str = "unnamed";
pub struct Command {
    pub key: char,
    pub name: String,
    pub command: String,
    pub children: Option<Box<Command>>,
    pub next: Option<Box<Command>>,
}
impl Command {
    pub fn new(key: char, name: String, command: String) -> Self {
        Command {
            key,
            name,
            command,
            children: None,
            next: None,
        }
    }
}
pub fn read_field(file: &mut &[u8], field: &str) -> String {
    unimplemented!()
}
pub fn load_file(c: &mut Command, file: &str) {
    unimplemented!()
}
pub fn start(c: &Command) {
    unimplemented!()
}
pub fn tree_add_command(tree: &mut Command, keys: &str, name: &str, command: &str) {
    unimplemented!()
}
pub fn read_line(c: &mut Command, file: &mut &[u8]) {
    unimplemented!()
}
pub fn command_add_child(c: &mut Command, child: Command) {
    unimplemented!()
}
pub fn getch() -> char {
    unimplemented!()
}
pub fn print_command(c: &Command) -> i32 {
    unimplemented!()
}
pub fn clear_lines(count: i32) {
    unimplemented!()
}
pub fn find_command(c: &Command, key: char) -> Option<&Command> {
    unimplemented!()
}
pub fn read_until_eol(file: &mut &[u8]) -> String {
    unimplemented!()
}
pub fn read_file(file: &str) -> String {
    unimplemented!()
}
pub fn command_run(c: &Command) -> i32 {
    unimplemented!()
}