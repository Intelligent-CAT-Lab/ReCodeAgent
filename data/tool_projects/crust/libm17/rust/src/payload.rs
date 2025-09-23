use crate::types::{CHAR_MAP, LSF};
const U40_9: u64 = 40_u64.pow(9);
const U40_9_8: u64 = U40_9 + 40_u64.pow(8);
pub fn decode_callsign_value(outp: &mut [u8], inp: u64) {
    unimplemented!()
}
pub fn decode_callsign_bytes(outp: &mut [u8], inp: &[u8; 6]) {
    unimplemented!()
}
pub fn encode_callsign_value(inp: &[u8]) -> Option<u64> {
    unimplemented!()
}
pub fn encode_callsign_bytes(inp: &[u8]) -> Option<[u8; 6]> {
    unimplemented!()
}
const M17_CRC_POLY: u16 = 0x5935;
pub fn crc_m17(input: &[u8]) -> u16 {
    unimplemented!()
}
pub fn lsf_crc(input: &LSF) -> u16 {
    unimplemented!()
}
pub fn extract_lich(outp: &mut [u8; 6], cnt: u8, inp: &LSF) {
    unimplemented!()
}
pub fn unpack_lich(out: &mut [u8], input: &[u8; 12]) {
    unimplemented!()
}