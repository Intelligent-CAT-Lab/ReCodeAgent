use crate::psbt::PsbtResult;
pub fn compactsize_read(_data: &[u8]) -> (u64, PsbtResult) {
    unimplemented!()
}
pub fn compactsize_length(_data: u64) -> u32 {
    unimplemented!()
}
pub fn compactsize_write(_dest: &mut [u8], _size: u64) {
    unimplemented!()
}
pub fn compactsize_peek_length(_chsize: u8) -> u32 {
    unimplemented!()
}