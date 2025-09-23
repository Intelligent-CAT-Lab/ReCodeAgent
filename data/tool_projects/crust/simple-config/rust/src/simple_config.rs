use std::{fmt::Display, fs::File};
// Constants
pub const CFG_FILE_EXT: &str = ".cfg";
pub const CFG_MAX_KEY: usize = 32;
pub const CFG_MAX_VAL: usize = 64;
pub const CFG_MAX_ERR: usize = 64;
// Structures and Enums
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct CfgColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CfgError {
    pub off: i32,
    pub col: i32,
    pub row: i32,
    pub msg: String,
}
impl Default for CfgError {
    fn default() -> Self {
        CfgError {
            off: 0,
            col: 0,
            row: 0,
            msg: String::new(),
        }
    }
}
#[derive(Clone, PartialEq, Debug)]
pub enum CfgVal {
    String(String),
    Boolean(bool),
    Int(i32),
    Float(f32),
    Color(CfgColor),
}
impl From<&str> for CfgVal {
    fn from(s: &str) -> Self {
        CfgVal::String(s.to_string())
    }
}
impl From<String> for CfgVal {
    fn from(s: String) -> Self {
        CfgVal::String(s)
    }
}
impl From<bool> for CfgVal {
    fn from(b: bool) -> Self {
        CfgVal::Boolean(b)
    }
}
impl From<i32> for CfgVal {
    fn from(i: i32) -> Self {
        CfgVal::Int(i)
    }
}
impl From<f32> for CfgVal {
    fn from(f: f32) -> Self {
        CfgVal::Float(f)
    }
}
impl From<CfgColor> for CfgVal {
    fn from(c: CfgColor) -> Self {
        CfgVal::Color(c)
    }
}
impl From<(u8, u8, u8, u8)> for CfgColor {
    fn from((r, g, b, a): (u8, u8, u8, u8)) -> Self {
        CfgColor { r, g, b, a }
    }
}
#[derive(Clone, PartialEq, Debug)]
pub struct CfgEntry {
    pub key: String,
    pub val: CfgVal,
}
#[derive(Clone, PartialEq, Debug)]
pub struct Cfg {
    pub entries: Vec<CfgEntry>,
    pub count: i32,
    pub capacity: usize,
}
impl Display for CfgError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}
impl Display for CfgColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}
impl Display for CfgVal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}
impl Display for CfgEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}
impl Display for Cfg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}
pub struct Scanner {
    pub src: String,
    pub len: i32,
    pub cur: i32,
}
// Public Functions
pub fn cfg_parse(src: &str) -> Result<Cfg, CfgError> {
    unimplemented!()
}
pub fn cfg_parse_file(filename: &str) -> Result<Cfg, CfgError> {
    unimplemented!()
}
pub fn cfg_get_string<'a>(cfg: &Cfg, key: &str, fallback: &'a str) -> &'a str {
    unimplemented!()
}
pub fn cfg_get_bool(cfg: &Cfg, key: &str, fallback: bool) -> bool {
    unimplemented!()
}
pub fn cfg_get_int(cfg: &Cfg, key: &str, fallback: i32) -> i32 {
    unimplemented!()
}
pub fn cfg_get_float(cfg: &Cfg, key: &str, fallback: f32) -> f32 {
    unimplemented!()
}
pub fn cfg_get_color(cfg: &Cfg, key: &str, fallback: CfgColor) -> CfgColor {
    unimplemented!()
}
pub fn cfg_get_int_min(cfg: &Cfg, key: &str, fallback: i32, min: i32) -> i32 {
    unimplemented!()
}
pub fn cfg_get_int_max(cfg: &Cfg, key: &str, fallback: i32, max: i32) -> i32 {
    unimplemented!()
}
pub fn cfg_get_int_range(cfg: &Cfg, key: &str, fallback: i32, min: i32, max: i32) -> i32 {
    unimplemented!()
}
pub fn cfg_get_float_min(cfg: &Cfg, key: &str, fallback: f32, min: f32) -> f32 {
    unimplemented!()
}
pub fn cfg_get_float_max(cfg: &Cfg, key: &str, fallback: f32, max: f32) -> f32 {
    unimplemented!()
}
pub fn cfg_get_float_range(cfg: &Cfg, key: &str, fallback: f32, min: f32, max: f32) -> f32 {
    unimplemented!()
}
pub fn cfg_fprint(file: &mut File, cfg: &Cfg) {
    unimplemented!()
}
pub fn cfg_fprint_error(file: &mut File, err: &CfgError) {
    unimplemented!()
}