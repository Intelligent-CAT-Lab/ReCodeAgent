use carrays::carrays;
// use carrays::circ_array;
use std::cmp::Ordering;
use rand::Rng;

#[test]
fn test_round() {
println!("Testing roundup32() / roundup64()");
assert_eq!(carrays::gca_roundup32(0), 0);
assert_eq!(carrays::gca_roundup64(0), 0);
assert_eq!(carrays::gca_roundup32(1), 1);
assert_eq!(carrays::gca_roundup64(1), 1);
assert_eq!(carrays::gca_roundup32(2), 2);
assert_eq!(carrays::gca_roundup64(2), 2);
assert_eq!(carrays::gca_roundup32(3), 4);
assert_eq!(carrays::gca_roundup64(3), 4);
assert_eq!(carrays::gca_roundup32(4), 4);
assert_eq!(carrays::gca_roundup64(4), 4);
assert_eq!(carrays::gca_roundup32(5), 8);
assert_eq!(carrays::gca_roundup64(5), 8);
assert_eq!(carrays::gca_roundup32(6), 8);
assert_eq!(carrays::gca_roundup64(6), 8);
assert_eq!(carrays::gca_roundup32(7), 8);
assert_eq!(carrays::gca_roundup64(7), 8);
assert_eq!(carrays::gca_roundup32(8), 8);
assert_eq!(carrays::gca_roundup64(8), 8);
assert_eq!(carrays::gca_roundup32(100), 128);
assert_eq!(carrays::gca_roundup64(100), 128);
for i in 2..32 {
assert_eq!(carrays::gca_roundup32((1u32 << i) - 1), 1u32 << i);
assert_eq!(carrays::gca_roundup32(1u32 << i), 1u32 << i);
}
for i in 2..64 {
assert_eq!(carrays::gca_roundup64((1u64 << i) - 1), 1u64 << i);
assert_eq!(carrays::gca_roundup64(1u64 << i), 1u64 << i);
}
}

#[test]
fn test_gcd() {
println!("Testing calculating GCD...");
for i in 0..100 {
assert_eq!(carrays::gca_calc_gcd(i, i), i);
assert_eq!(carrays::gca_calc_gcd(i, 0), i);
assert_eq!(carrays::gca_calc_gcd(0, i), i);
assert_eq!(carrays::gca_calc_gcd(i, 1), 1);
assert_eq!(carrays::gca_calc_gcd(1, i), 1);
}
assert_eq!(carrays::gca_calc_gcd(2, 4), 2);
assert_eq!(carrays::gca_calc_gcd(4, 2), 2);
assert_eq!(carrays::gca_calc_gcd(6, 9), 3);
assert_eq!(carrays::gca_calc_gcd(9, 6), 3);
assert_eq!(carrays::gca_calc_gcd(0, 0), 0);
assert_eq!(carrays::gca_calc_gcd(10, 0), 10);
assert_eq!(carrays::gca_calc_gcd(0, 10), 10);
assert_eq!(carrays::gca_calc_gcd(2, 2), 2);
assert_eq!(carrays::gca_calc_gcd(1, 1), 1);
assert_eq!(carrays::gca_calc_gcd(1, 2), 1);
assert_eq!(carrays::gca_calc_gcd(1, 100), 1);
assert_eq!(carrays::gca_calc_gcd(2, 4), 2);
assert_eq!(carrays::gca_calc_gcd(7, 5), 1);
assert_eq!(carrays::gca_calc_gcd(18, 6), 6);
assert_eq!(carrays::gca_calc_gcd(3, 6), 3);
assert_eq!(carrays::gca_calc_gcd(100, 120), 20);
assert_eq!(carrays::gca_calc_gcd(100, 125), 25);
}

// Additional tests for other functions would follow a similar pattern.

fn main() {
// In Rust, we don't need a main function for tests.
// The `cargo test` command will automatically run all functions marked with `#[test]`.
}
