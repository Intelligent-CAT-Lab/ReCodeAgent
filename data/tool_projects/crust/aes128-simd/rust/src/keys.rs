use crate::aes::{NB, NR, NK};
pub fn add(state: &mut [[u8; NB]; 4], round_key: &[[u8; NB]; 4]) {
    unimplemented!()
}
pub fn expansion(key: &[u8; 4 * NK], w: &mut [u8; 4 * NB * (NR + 1)]) {
    unimplemented!()
}