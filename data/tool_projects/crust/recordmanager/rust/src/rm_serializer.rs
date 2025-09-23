use crate::{dberror::RC, tables::{RM_TableData, Schema, Record, Value}};
pub struct VarString {
pub buf: String,
pub size: usize,
pub bufsize: usize,
}
pub fn attr_offset(schema: &Schema, attr_num: i32, result: &mut i32) -> RC {
unimplemented!()
}
pub fn serialize_table_info(rel: &RM_TableData) -> String {
unimplemented!()
}
pub fn serialize_table_content(rel: &RM_TableData) -> String {
unimplemented!()
}
pub fn serialize_schema(schema: &Schema) -> String {
unimplemented!()
}
pub fn serialize_record(record: &Record, schema: &Schema) -> String {
unimplemented!()
}
pub fn serialize_attr(record: &Record, schema: &Schema, attr_num: i32) -> String {
unimplemented!()
}
pub fn serialize_value(val: &Value) -> String {
unimplemented!()
}
pub fn string_to_value(val: &str) -> Value {
unimplemented!()
}