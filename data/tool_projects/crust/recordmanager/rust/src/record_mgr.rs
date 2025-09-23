use crate::{dberror::RC, expr::Expr, tables::{Record, Schema, RM_TableData, RID}};
use crate::buffer_mgr::{BM_BufferPool, BM_PageHandle};
use crate::tables::{DataType, Value};
pub struct RM_ScanHandle {
    pub rel: RM_TableData,
    pub mgmt_data: Option<Box<dyn std::any::Any>>,
}
pub struct TableManager{
    pub total_tuples: i32,
    pub rec_size : i32,
    pub first_free_page_num: i32,
    pub first_free_slot_num: i32,
    pub first_data_page_num: i32,
    pub buffer_pool: Option<BM_BufferPool>,
    pub page_handler: Option<BM_PageHandle>,
}
pub struct ScanManager {
pub total_entries: i32,
pub scan_index: i32,
pub current_page_num: i32,
pub current_slot_num: i32,
pub condition_expression: Option<Expr>,
pub scan_page_handle_ptr: Option<BM_PageHandle>,
}
pub struct PageHeader {
pub page_identifier: char,
pub total_tuples: i32,
pub free_slot_cnt: i32,
pub next_free_slot_ind: i32,
pub prev_free_page_index: i32,
pub next_free_page_index: i32,
pub prev_data_page_index: i32,
pub next_data_page_index: i32,
}
pub fn init_record_manager(mgmt_data: Option<Box<dyn std::any::Any>>) -> RC {
unimplemented!()
}
pub fn shutdown_record_manager() -> RC {
unimplemented!()
}
pub fn create_table(name: &str, schema: &Schema) -> RC {
unimplemented!()
}
pub fn open_table(rel: &mut RM_TableData, name: &str) -> RC {
unimplemented!()
}
pub fn close_table(rel: &mut RM_TableData) -> RC {
unimplemented!()
}
pub fn delete_table(name: &str) -> RC {
unimplemented!()
}
pub fn get_num_tuples(rel: &RM_TableData) -> i32 {
unimplemented!()
}
pub fn insert_record(rel: &mut RM_TableData, record: &Record) -> RC {
unimplemented!()
}
pub fn delete_record(rel: &mut RM_TableData, id: &RID) -> RC {
unimplemented!()
}
pub fn update_record(rel: &mut RM_TableData, record: &Record) -> RC {
unimplemented!()
}
pub fn get_record(rel: &RM_TableData, id: &RID, record: &mut Record) -> RC {
unimplemented!()
}
pub fn start_scan(rel: &RM_TableData, scan: &mut RM_ScanHandle, cond: &Expr) -> RC {
unimplemented!()
}
pub fn next(scan: &mut RM_ScanHandle, record: &mut Record) -> RC {
unimplemented!()
}
pub fn close_scan(scan: &mut RM_ScanHandle) -> RC {
unimplemented!()
}
pub fn get_record_size(schema: &Schema) -> i32 {
unimplemented!()
}
pub fn create_schema(num_attr: i32, attr_names: Vec<String>, data_types: Vec<DataType>, type_length: Vec<i32>, key_size: i32, keys: Vec<i32>) -> Schema {
unimplemented!();
}
pub fn free_schema(schema: &mut Schema) -> RC {
unimplemented!()
}
pub fn create_record(record: &mut Option<Record>, schema: &Schema) -> RC {
unimplemented!()
}
pub fn free_record(record: &mut Record) -> RC {
unimplemented!()
}
pub fn get_attr(record: &Record, schema: &Schema, attr_num: i32, value: &mut Value) -> RC {
unimplemented!()
}
pub fn set_attr(record: &mut Record, schema: &Schema, attr_num: i32, value: &Value) -> RC {
unimplemented!()
}
pub fn get_attr_pos(schema: &Schema, attr_num: i32) -> i32 {
unimplemented!()
}