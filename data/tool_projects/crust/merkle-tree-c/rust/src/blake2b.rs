pub const BLAKE2B_BLOCKBYTES: usize = 128;
pub const BLAKE2B_OUTBYTES: usize = 64;
pub const BLAKE2B_KEYBYTES: usize = 64;
pub const BLAKE2B_SALTBYTES: usize = 16;
pub const BLAKE2B_PERSONALBYTES: usize = 16;
#[derive(Debug, Clone)]
pub struct Blake2bState {
    pub h: [u64; 8],
    pub t: [u64; 2],
    pub f: [u64; 2],
    pub buf: [u8; BLAKE2B_BLOCKBYTES],
    pub buflen: usize,
    pub outlen: usize,
    pub last_node: u8,
}
#[repr(packed)]
#[derive(Debug, Clone)]
pub struct Blake2bParam {
    pub digest_length: u8,
    pub key_length: u8,
    pub fanout: u8,
    pub depth: u8,
    pub leaf_length: u32,
    pub node_offset: u32,
    pub xof_length: u32,
    pub node_depth: u8,
    pub inner_length: u8,
    pub reserved: [u8; 14],
    pub salt: [u8; BLAKE2B_SALTBYTES],
    pub personal: [u8; BLAKE2B_PERSONALBYTES],
}
// Streaming API
pub fn blake2b_init(state: &mut Blake2bState, outlen: usize) -> i32 {
    unimplemented!()
}
pub fn blake2b_init_key(state: &mut Blake2bState, outlen: usize, key: &[u8]) -> i32 {
    unimplemented!()
}
pub fn blake2b_init_param(state: &mut Blake2bState, param: &Blake2bParam) -> i32 {
    unimplemented!()
}
pub fn blake2b_update(state: &mut Blake2bState, input: &[u8]) -> i32 {
    unimplemented!()
}
pub fn blake2b_final(state: &mut Blake2bState, out: &mut [u8]) -> i32 {
    unimplemented!()
}
// Simple API (key may be provided as Option)
pub fn blake2b(out: &mut [u8], input: &[u8], key: Option<&[u8]>) -> i32 {
    unimplemented!()
}
// This is simply an alias for blake2b
pub fn blake2(out: &mut [u8], input: &[u8], key: Option<&[u8]>) -> i32 {
    blake2b(out, input, key)
}