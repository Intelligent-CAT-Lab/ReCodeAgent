use std::fs::File;
pub struct TrustedChecker {
input: File,
output: File,
}
impl TrustedChecker {
pub fn tc_run(check_model: bool, lenient: bool) -> i32 {
unimplemented!()
}
pub fn tc_init(fifo_in: &str, fifo_out: &str) -> Self {
unimplemented!()
}
pub fn tc_end(&mut self) {
unimplemented!()
}
pub fn read_literals(&mut self, nb_lits: i32) {
unimplemented!()
}
pub fn read_hints(&mut self, nb_hints: i32) {
unimplemented!()
}
pub fn say_with_flush(&mut self, ok: bool) {
unimplemented!()
}
pub fn say(&mut self, ok: bool) {
unimplemented!()
}
}