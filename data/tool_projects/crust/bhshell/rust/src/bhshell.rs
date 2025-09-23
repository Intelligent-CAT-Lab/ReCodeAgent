use crate::input::Command;
pub const BUF_SIZE: usize = 64;
/// Runs the main bhshell loop.
pub fn bhshell_loop() {
unimplemented!()
}
/// Executes the given command.
/// Returns an integer status code (C equivalent of int).
pub fn bhshell_execute(_cmd: &mut Command) -> i32 {
unimplemented!()
}
/// Launches the given command.
/// Returns an integer status code (C equivalent of int).
pub fn bhshell_launch(_cmd: &mut Command) -> i32 {
unimplemented!()
}
/// Changes the current directory.
/// Returns an integer status code (C equivalent of int).
pub fn bhshell_cd(_args: &[String]) -> i32 {
unimplemented!()
}
/// Prints help information.
/// Returns an integer status code (C equivalent of int).
pub fn bhshell_help(_args: &[String]) -> i32 {
unimplemented!()
}
/// Handles exit request.
/// Returns an integer status code (C equivalent of int).
pub fn bhshell_exit(_args: &[String]) -> i32 {
unimplemented!()
}
/// Returns the number of built-in commands.
pub fn bhshell_num_builtins() -> i32 {
unimplemented!()
}
/// Writes to a redirected file descriptor array.
/// In C, this took an array 'int redirect_fd[2]' and a pointer to 'command'.
pub fn write_to_redirect(_redirect_fd: &mut [i32; 2], _cmd: &mut Command) {
unimplemented!()
}