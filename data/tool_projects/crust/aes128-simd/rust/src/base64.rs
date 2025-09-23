use crate::aes::{NB};
use std::arch::x86_64::__m128i;
pub fn base64_encode(input: &[u8], len: usize) -> String {
    unimplemented!()
}
pub fn g_mult_sse_byte(first: u8, second: u8) -> u8 {
    unimplemented!()
}
pub fn g_mult_sse(first: __m128i, second: __m128i) -> __m128i {
    unimplemented!()
}
pub fn columns_sse(state: &mut [[u8; NB]; 4]) {
    unimplemented!()
}