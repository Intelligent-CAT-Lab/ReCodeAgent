use crate::dberror::RC;
use crate::storage_mgr::{SM_FileHandle, SM_PageHandle};
pub struct Bufferpool {
pub num_read: i32,
pub num_write: i32,
pub total_pages: i32,
pub updated_strategy: i32,
pub free_space: i32,
pub updated_order: Vec<i32>,
pub bitdirty: Vec<bool>,
pub fix_count: Vec<i32>,
pub access_time: Vec<i32>,
pub pagenum: Vec<i32>,
pub pagedata: String,
pub fhl: SM_FileHandle,
}
pub struct BM_PageHandle {
pub page_num: PageNumber,
pub data: String
}
pub type PageNumber = i32;
pub const NO_PAGE: PageNumber = -1;
pub enum ReplacementStrategy {
RsFifo = 0,
RsLru = 1,
RsClock = 2,
RsLfu = 3,
RsLruK = 4,
}
pub struct BM_BufferPool {
pub page_file: String,
pub num_pages: i32,
pub strategy: ReplacementStrategy,
pub mgmt_data: Option<Box<dyn std::any::Any>>,
}
pub fn init_buffer_pool(bm: &mut BM_BufferPool, page_file_name: &str, num_pages: i32, strategy: ReplacementStrategy, strat_data: Option<Box<dyn std::any::Any>>) -> RC {
unimplemented!()
}
pub fn shutdown_buffer_pool(bm: &mut BM_BufferPool) -> RC {
unimplemented!()
}
pub fn force_flush_pool(bm: &mut BM_BufferPool) -> RC {
unimplemented!()
}
pub fn mark_dirty(bm: &mut BM_BufferPool, page: &mut BM_PageHandle) -> RC {
unimplemented!()
}
pub fn unpin_page(bm: &mut BM_BufferPool, page: &mut BM_PageHandle) -> RC {
unimplemented!()
}
pub fn force_page(bm: &mut BM_BufferPool, page: &mut BM_PageHandle) -> RC {
unimplemented!()
}
pub fn pin_page(bm: &mut BM_BufferPool, page: &mut BM_PageHandle, page_num: PageNumber) -> RC {
unimplemented!()
}
fn shift_updated_order(start: i32, end: i32, bm: &mut BM_BufferPool, page_num: i32){
unimplemented!()
}
fn update_bufferpool_stats(bm: &mut BM_BufferPool, address: i32, page_num: i32){
unimplemented!()
}
pub fn get_frame_contents(bm: &BM_BufferPool) -> Vec<PageNumber> {
unimplemented!()
}
pub fn get_dirty_flags(bm: &BM_BufferPool) -> Vec<bool> {
unimplemented!()
}
pub fn get_fix_counts(bm: &BM_BufferPool) -> Vec<i32> {
unimplemented!()
}
pub fn get_num_read_io(bm: &BM_BufferPool) -> i32 {
unimplemented!()
}
pub fn get_num_write_io(bm: &BM_BufferPool) -> i32 {
unimplemented!()
}