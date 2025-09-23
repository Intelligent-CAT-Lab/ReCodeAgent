// Frame types
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FrameType {
    Lsf,
    Str,
    Pkt,
}
// Preamble types
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PreambleType {
    Lsf,
    Bert,
}
// Link Setup Frame structure
#[derive(Default, Clone, Debug)]
pub struct LSF {
    pub dst: [u8; 6],
    pub src: [u8; 6],
    pub type_field: [u8; 2],
    pub meta: [u8; 14],
    pub crc: [u8; 2],
}
// Constants
pub const SYM_PER_SWD: usize = 8;
pub const FLT_SPAN: usize = 8;
pub const RRC_DEV: f32 = 7168.0;
pub const SYM_PER_PLD: usize = 184;
pub const SYM_PER_FRA: usize = 192;
pub const NUM_STATES: usize = 1 << (5 - 1);
pub const CHAR_MAP: &str = " ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-/.";
pub const U40_9: u64 = 262144000000000;
pub const U40_9_8: u64 = 268697600000000;
pub const BSB_SPS: usize = 10;
pub const SW_LEN: usize = BSB_SPS * SYM_PER_SWD;