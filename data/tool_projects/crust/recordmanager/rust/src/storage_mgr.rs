use crate::dberror::RC;
use crate::tables::{Value, Schema};
pub struct SM_FileHandle {
pub file_name: String,
pub total_num_pages: i32,
pub cur_page_pos: i32,
pub mgmt_info: Option<Box<dyn std::any::Any>>,
}
pub type SM_PageHandle = String;
pub fn init_storage_manager() {
unimplemented!()
}
pub fn create_page_file(file_name: &str) -> RC {
unimplemented!()
}
pub fn open_page_file(file_name: &str, f_handle: &mut SM_FileHandle) -> RC {
unimplemented!()
}
pub fn close_page_file(f_handle: &mut SM_FileHandle) -> RC {
unimplemented!()
}
pub fn destroy_page_file(file_name: &str) -> RC {
unimplemented!()
}
pub fn read_block(page_num: i32, f_handle: &mut SM_FileHandle, mem_page: &mut SM_PageHandle) -> RC {
unimplemented!()
}
pub fn get_block_pos(f_handle: &SM_FileHandle) -> i32 {
unimplemented!()
}
pub fn read_first_block(f_handle: &mut SM_FileHandle, mem_page: &mut SM_PageHandle) -> RC {
unimplemented!()
}
pub fn read_previous_block(f_handle: &mut SM_FileHandle, mem_page: &mut SM_PageHandle) -> RC {
unimplemented!()
}
pub fn read_current_block(f_handle: &mut SM_FileHandle, mem_page: &mut SM_PageHandle) -> RC {
unimplemented!()
}
pub fn read_next_block(f_handle: &mut SM_FileHandle, mem_page: &mut SM_PageHandle) -> RC {
unimplemented!()
}
pub fn read_last_block(f_handle: &mut SM_FileHandle, mem_page: &mut SM_PageHandle) -> RC {
unimplemented!()
}
pub fn write_block(page_num: i32, f_handle: &mut SM_FileHandle, mem_page: &SM_PageHandle) -> RC {
unimplemented!()
}
pub fn write_current_block(f_handle: &mut SM_FileHandle, mem_page: &SM_PageHandle) -> RC {
unimplemented!()
}
pub fn append_empty_block(f_handle: &mut SM_FileHandle) -> RC {
unimplemented!()
}
pub fn ensure_capacity(number_of_pages: i32, f_handle: &mut SM_FileHandle) -> RC {
unimplemented!()
}