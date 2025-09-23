/// Initializes the bm subsystem.
pub fn bm_init() {
    unimplemented!();
}
/// Reports a message.
pub fn bm_report(msg: &str) {
    unimplemented!();
}
/// Reads keys and calls the provided callback with a slice of keys and an integer.
/// The callback receives a slice of string slices (`&[&str]`) and an `i32`.
pub fn bm_read_keys<F>(mut cb: F)
where
    F: FnMut(&[&str], i32),
{
    unimplemented!();
}