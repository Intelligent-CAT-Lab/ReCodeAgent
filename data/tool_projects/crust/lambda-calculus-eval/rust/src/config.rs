use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
pub const CONFIG_PATH: &str = "config";
#[derive(Debug)]
pub enum reduction_order_t {
    APPLICATIVE,
    NORMAL,
}
#[derive(Debug, PartialEq, Eq)]
pub enum option_type_t {
    FILENAME,
    STEP_REDUCTION,
    REDUCTION_ORDER,
    CONFIG_ERROR,
}
pub struct Options {
pub file: File,
pub step_by_step_reduction: bool,
pub reduction_order: reduction_order_t,
}
pub fn trim(str: &mut String) {
unimplemented!()
}
pub fn get_config_type(key: &str) -> option_type_t {
unimplemented!()
}
pub fn parse_config(line: &str, key: &mut String, value: &mut String) {
unimplemented!()
}
pub fn get_config_from_file() -> Options {
unimplemented!()
}