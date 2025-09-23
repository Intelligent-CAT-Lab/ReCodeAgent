pub const KNORMAL: &str = "\x1B[0m";
pub const KRED: &str = "\x1B[31m";
pub const KGREEN: &str = "\x1B[32m";
pub const KYELLOW: &str = "\x1B[33m";
pub const KBLUE: &str = "\x1B[34m";
pub const KMAGENTA: &str = "\x1B[35m";
pub const KCYAN: &str = "\x1B[36m";
pub const KWHITE: &str = "\x1B[37m";
pub const COLOR_BOLD: &str = "\x1B[1m";
pub const COLOR_OFF: &str = "\x1B[m";
pub enum LOGGING_TAG {
    INFO_TAG = 1,
    ERROR_TAG = 2,
    WARNING_TAG = 3,
}
pub fn logger(tag: LOGGING_TAG, message: &str) {
    unimplemented!()
}