use std::fmt;
#[derive(Debug, Clone, PartialEq)]
pub enum DataType {
    Int,
    Str,
}
#[derive(Debug, Clone)]
pub struct Data {
    pub dtype: DataType,
    pub value: DataValue,
}
#[derive(Debug, Clone)]
pub enum DataValue {
    Int(i64),
    Str(String),
}
impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}
impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}
impl Data {
    pub fn new(dtype: DataType) -> Data {
        unimplemented!()
    }
    pub fn new_int(val: i64) -> Data {
        unimplemented!()
    }
    pub fn new_str(val: String) -> Data {
        unimplemented!()
    }
}