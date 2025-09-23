pub const C_ROUNDS: u32 = 2;
pub const D_ROUNDS: u32 = 4;
pub struct SipHash {
kk: Vec<u8>,
out: Vec<u8>,
buf: Vec<u8>,
}
impl SipHash {
pub fn siphash_update(&mut self, data: &[u8], nb_bytes: u64) {
unimplemented!()
}
pub fn process_next_block(&mut self) {
unimplemented!()
}
pub fn process_final_block(&mut self) {
unimplemented!()
}
pub fn siphash_pad(&mut self, nb_bytes: u64) {
unimplemented!()
}
pub fn siphash_init(key_128bit: &[u8]) -> Self {
unimplemented!()
}
pub fn siphash_reset(&mut self) {
unimplemented!()
}
pub fn siphash_digest(&self) -> Vec<u8> {
unimplemented!()
}
pub fn siphash_free(&mut self) {
unimplemented!()
}
}