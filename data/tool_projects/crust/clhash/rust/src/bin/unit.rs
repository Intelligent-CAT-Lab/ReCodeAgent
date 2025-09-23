use clhash::clhash::{clhash, get_random_key_for_clhash};

const RANDOM_BYTES_NEEDED_FOR_CLHASH: usize = 2;
fn flipbit(block: &mut [u8], bit: u32) {
let byte = (bit >> 3) as usize;
let bit = bit & 0x7;
block[byte] ^= 1 << bit;
}

#[test]
fn clhash_avalanche_test() {
const N: usize = 1024;
let mut array = vec![0u8; N];
let mut array1 = vec![0u8; N];
let mut rs = vec![0u64; RANDOM_BYTES_NEEDED_FOR_CLHASH];
for k in 0..RANDOM_BYTES_NEEDED_FOR_CLHASH {
rs[k] = (k + 1 - k * k) as u64;
}
let rs = [rs[0], rs[1]];
let rs_bytes: &[u8] = bytemuck::bytes_of(&rs);
const K: usize = 16;
println!("[clhashavalanchetest] Testing CLHASH avalanche effect.");
for bytelength in 1..K {
for whichcase in 0..256 {
for k in 0..bytelength {
array[k] = whichcase as u8;
array1[k] = (whichcase + 35) as u8;
}
let orighash = clhash(rs_bytes, &array[..bytelength]);
let orighash1 = clhash(rs_bytes, &array1[..bytelength]);
for z in 0..8 * bytelength {
flipbit(&mut array, z as u32);
let newhash = clhash(rs_bytes, &array[..bytelength]);
flipbit(&mut array, z as u32);
assert_ne!(orighash, newhash);
flipbit(&mut array1, z as u32);
let newhash1 = clhash(rs_bytes, &array1[..bytelength]);
flipbit(&mut array1, z as u32);
assert_ne!(orighash1, newhash1);
if bytelength <= std::mem::size_of::<u64>() {
assert_eq!(orighash ^ newhash, orighash1 ^ newhash1);
}
}
}
}
println!("Test passed!");
}

#[test]
fn clhash_collision_test() {
println!("[clhashcollisiontest] Testing whether we can induce collisions by hacking the right bytes (Eik List's test).");
const NUM_TRIALS: usize = 10;
const CLNH_NUM_BYTES_PER_BLOCK: usize = 1024;
const KEY_OFFSET: u8 = 0x63;
let mut k = vec![0u64; RANDOM_BYTES_NEEDED_FOR_CLHASH];
for j2 in 0..RANDOM_BYTES_NEEDED_FOR_CLHASH {
k[j2] = (j2 + KEY_OFFSET as usize) as u64 & 0xFF;
}
let k = (k[0], k[1]);
for i in 1..NUM_TRIALS {
for j in 1..=std::mem::size_of::<u64>() {
let mlen = i * CLNH_NUM_BYTES_PER_BLOCK + j;
let mut m = vec![0u8; mlen];
for j2 in 0..mlen {
m[j2] = (j2 & 0xFF) as u8;
}
let k = [k.0, k.1];
let actual1 = clhash(bytemuck::bytes_of(&k), &m);
m[mlen - 1] = (m[mlen - 1] + 1) & 0xFF;
let actual2 = clhash(bytemuck::bytes_of(&k), &m);
let are_equal = actual1 == actual2;
if are_equal {
println!("Testing {} bytes, H1: {:X}, H2: {:X}, equal: {}", mlen, actual1, actual2, are_equal);
}
assert!(!are_equal);
}
}
println!("Test passed!");
}

#[test]
fn clhash_test() {
const N: usize = 1024;
let mut array = vec![0u8; N];
let mut rs = vec![0u64; RANDOM_BYTES_NEEDED_FOR_CLHASH];
for k in 0..RANDOM_BYTES_NEEDED_FOR_CLHASH {
rs[k] = (1 - k) as u64;
}
// convert the vec into a tuple of (&[u8], &[u8])
let rs = [rs[0], rs[1]];
let rs_bytes: &[u8] = bytemuck::bytes_of(&rs);
println!("[clhashtest] checking that flipping a bit changes hash value with clhash");
for bit in 0..64 {
for length in ((bit + 8) / 8)..=std::mem::size_of::<u64>() {
let mut x: u64 = 0;
let orig = clhash(rs_bytes, &x.to_le_bytes()[..length]);
x ^= 1 << bit;
let flip = clhash(rs_bytes, &x.to_le_bytes()[..length]);
assert_ne!(flip, orig);
x ^= 1 << bit;
let back = clhash(rs_bytes, &x.to_le_bytes()[..length]);
assert_eq!(back, orig);
}
}
println!("Test passed!");
}
#[test]
fn demo() {
let random = get_random_key_for_clhash(0x23a23cf5033c3c81, 0xb3816f6a2c68e530);
let hashvalue1 = clhash(&random, b"my dog");
let hashvalue2 = clhash(&random, b"my cat");
let hashvalue3 = clhash(&random, b"my dog");
assert_eq!(hashvalue1, hashvalue3);
assert_ne!(hashvalue1, hashvalue2);
}

fn main() {

}

