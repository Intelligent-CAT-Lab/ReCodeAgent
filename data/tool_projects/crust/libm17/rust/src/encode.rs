use crate::types::LSF;
// Puncture patterns as constant arrays
pub const PUNCTURE_PATTERN_1: [u8; 61] = [
    1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1,
    1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1,
];
pub const PUNCTURE_PATTERN_2: [u8; 12] = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0];
pub const PUNCTURE_PATTERN_3: [u8; 8] = [1, 1, 1, 1, 1, 1, 1, 0];
pub const SYMBOL_MAP: [i8; 4] = [1, 3, -1, -3];
pub const SYMBOL_LIST: [i8; 4] = [-3, -1, 1, 3];
pub const EOT_SYMBOLS: [f32; 8] = [3.0, 3.0, 3.0, 3.0, 3.0, 3.0, -3.0, 3.0];
pub fn conv_encode_stream_frame(out: &mut [u8], input: &[u8], fn_num: u16) {
    unimplemented!()
}
pub fn conv_encode_packet_frame(out: &mut [u8], input: &[u8]) {
    unimplemented!()
}
pub fn conv_encode_lsf(out: &mut [u8], input: &LSF) {
    unimplemented!()
}