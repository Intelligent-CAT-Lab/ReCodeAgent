pub const RANDOM_64BITWORDS_NEEDED_FOR_CLHASH: usize = 133;
pub const RANDOM_BYTES_NEEDED_FOR_CLHASH: usize = 133 * 8;
pub fn clhash(random: &[u8], stringbyte: &[u8]) -> u64 {
unimplemented!();
}
pub fn get_random_key_for_clhash(seed1: u64, seed2: u64) -> Vec<u8> {
unimplemented!();
}
pub struct ClHasher {
random_data: Vec<u8>,
}
impl ClHasher {
pub fn new(seed1: u64, seed2: u64) -> Self {
    unimplemented!();
}
pub fn hash<T: AsRef<[u8]>>(&self, data: T) -> u64 {
    unimplemented!();
}
}
impl Drop for ClHasher {
fn drop(&mut self) {
unimplemented!();
}
}