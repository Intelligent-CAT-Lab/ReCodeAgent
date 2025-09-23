use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::fmt;
use crate::check;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LogType {
    Log = 0,
    UnitTest = 1,
    OpenHash = 2,
    CuckooHash = 3,
}
pub struct LogInfo {
    priority: u8,
    prefix: &'static str,
    suffix: &'static str,
}
pub struct Logger {
    log_info_table: RwLock<HashMap<LogType, LogInfo>>,
}
impl Logger {
    pub fn new() -> Arc<Self> {
       unimplemented!() 
    }
    pub fn insert_log(&self, log_type: LogType, priority: u8, format: fmt::Arguments) {
        unimplemented!()
    }
    pub fn change_log_priority(&self, log_type: LogType, new_priority: u8) -> u8 {
        unimplemented!()
    }
}