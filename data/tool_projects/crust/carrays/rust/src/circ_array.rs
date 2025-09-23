use std::alloc::{alloc, dealloc, realloc, Layout};
use std::ptr::{self, NonNull};
use std::slice;
use std::mem;
pub struct CircBuf {
el: usize,
start: usize,
n: usize,
size: usize,
mask: usize,
b: NonNull<u8>,
}
impl CircBuf {
pub fn new(el: usize, size: usize) -> Self {
    unimplemented!();
}
pub fn dealloc(&mut self) {
    unimplemented!();
}
pub fn resize(&mut self, size: usize) {
    unimplemented!();
}
pub fn capacity(&mut self, size: usize) {
    unimplemented!();
}
pub fn push(&mut self) -> &mut [u8] {
    unimplemented!();
}
pub fn pop(&mut self) -> &mut [u8] {
    unimplemented!();
}
pub fn unshift(&mut self) -> &mut [u8] {
    unimplemented!();
}
pub fn shift(&mut self) -> &mut [u8] {
    unimplemented!();
}
pub fn norm(&mut self) {
    unimplemented!();
}
fn get(&mut self, idx: usize) -> &mut [u8] {
    unimplemented!();
}
}
fn roundup64(x: u64) -> u64 {
    unimplemented!();
}
fn gca_cycle_left(ptr: &mut [u8], n: usize, es: usize, shift: usize) {
    unimplemented!();
}
fn gca_calc_gcd(mut a: u32, mut b: u32) -> u32 {
    unimplemented!();
}