// Synchronization symbols for LSF (Link Setup Frame)
pub const LSF_SYNC_SYMBOLS: [i8; 8] = [3, 3, 3, 3, -3, -3, 3, -3];
// Synchronization symbols for Stream
pub const STR_SYNC_SYMBOLS: [i8; 8] = [-3, -3, -3, -3, 3, 3, -3, 3];
// Synchronization symbols for Packet
pub const PKT_SYNC_SYMBOLS: [i8; 8] = [3, -3, 3, 3, -3, -3, -3, -3];
// Symbol levels for modulation
pub const SYMBOL_LEVELS: [f32; 4] = [-3.0, -1.0, 1.0, 3.0];
pub const NUM_STATES: usize = 1 << (5 - 1);
static mut PREV_METRICS: [u32; NUM_STATES] = [0; NUM_STATES];
static mut CURR_METRICS: [u32; NUM_STATES] = [0; NUM_STATES];
static mut PREV_METRICS_DATA: [u32; NUM_STATES] = [0; NUM_STATES];
static mut CURR_METRICS_DATA: [u32; NUM_STATES] = [0; NUM_STATES];
static mut VITERBI_HISTORY: [u16; 244] = [0; 244];
pub fn viterbi_decode(out: &mut [u8], input: &[u16], len: u16) -> u32 {
    unimplemented!()
}
pub fn viterbi_decode_punctured(
    out: &mut [u8],
    input: &[u16],
    punct: &[u8],
    in_len: u16,
    p_len: u16,
) -> u32 {
    unimplemented!()
}
pub fn viterbi_decode_bit(s0: u16, s1: u16, pos: usize) {
    unimplemented!()
}
fn viterbi_chainback(out: &mut [u8], pos: usize, len: u16) -> u32 {
    unimplemented!()
}
fn viterbi_reset() {
    unimplemented!()
}