use bitset::bitset::Bitset;
use std::fs;
use std::path::Path;

#[test]
fn test_bitset_new() {
    let bs_null = Bitset::new(0);
    assert_eq!(bs_null.array.len(), 0);
    assert_eq!(bs_null.count(), 0);
    assert_eq!(bs_null.flip, false);
    assert_eq!(bs_null.size(), 0);

    let bs_pow2 = Bitset::new(1024);
    assert_eq!(bs_pow2.count(), 0);
    assert_eq!(bs_pow2.array.len(), 128); // 1024 bits / 8 bits per byte
    assert_eq!(bs_pow2.flip, false);
    assert_eq!(bs_pow2.size(), 1024);

    let bs_not_pow2 = Bitset::new(45);
    assert_eq!(bs_not_pow2.array.len(), 6); // 45 bits / 8 bits per byte
    assert_eq!(bs_not_pow2.count(), 0);
    assert_eq!(bs_not_pow2.flip, false);
    assert_eq!(bs_not_pow2.size(), 45);
}

#[test]
fn test_bitset_count() {
    let mut bs = Bitset::new(100);
    assert_eq!(bs.count(), 0);

    bs.set(5, true);
    assert_eq!(bs.count(), 1);

    bs.set(5, true);
    assert_eq!(bs.count(), 1);

    bs.set(1, false);
    assert_eq!(bs.count(), 1);

    bs.set(5, false);
    assert_eq!(bs.count(), 0);

    bs.set(5, true);
    assert_eq!(bs.count(), 1);

    bs.flip();
    assert_eq!(bs.count(), 99);
}

#[test]
fn test_bitset_size() {
    let bs_null = Bitset::new(0);
    assert_eq!(bs_null.size(), 0);

    let bs_pow2 = Bitset::new(1024);
    assert_eq!(bs_pow2.size(), 1024);

    let bs_not_pow2 = Bitset::new(45);
    assert_eq!(bs_not_pow2.size(), 45);
}

#[test]
fn test_bitset_test() {
    let mut bs = Bitset::new(8);
    for i in 0..8 {
        assert_eq!(bs.test(i), false);
    }

    bs.flip();
    for i in 0..8 {
        assert_eq!(bs.test(i), true);
    }

    for i in 0..8 {
        bs.set(i, false);
    }
    for i in 0..8 {
        assert_eq!(bs.test(i), false);
    }

    bs.flip();
    for i in 0..8 {
        assert_eq!(bs.test(i), true);
    }

    bs.set(4, false);
    assert_eq!(bs.test(4), false);

    bs.flip();
    assert_eq!(bs.test(4), true);
}

#[test]
fn test_bitset_any() {
    let mut bs = Bitset::new(8);
    assert_eq!(bs.any(), false);

    bs.flip();
    assert_eq!(bs.any(), true);

    for i in 0..8 {
        bs.set(i, false);
    }
    assert_eq!(bs.any(), false);

    bs.flip();
    assert_eq!(bs.any(), true);

    bs.set(0, false);
    assert_eq!(bs.any(), true);

    bs.flip();
    assert_eq!(bs.any(), true);
}

#[test]
fn test_bitset_none() {
    let mut bs = Bitset::new(8);
    assert_eq!(bs.none(), true);

    bs.flip();
    assert_eq!(bs.none(), false);

    for i in 0..8 {
        bs.set(i, false);
    }
    assert_eq!(bs.none(), true);

    bs.flip();
    assert_eq!(bs.none(), false);

    bs.set(0, false);
    assert_eq!(bs.none(), false);

    bs.flip();
    assert_eq!(bs.none(), false);
}

#[test]
fn test_bitset_all() {
    let mut bs = Bitset::new(8);
    assert_eq!(bs.all(), false);

    bs.flip();
    assert_eq!(bs.all(), true);

    for i in 0..8 {
        bs.set(i, false);
    }
    assert_eq!(bs.all(), false);

    bs.flip();
    assert_eq!(bs.all(), true);

    bs.set(0, false);
    assert_eq!(bs.all(), false);

    bs.flip();
    assert_eq!(bs.all(), false);
}

#[test]
fn test_bitset_set() {
    let mut bs = Bitset::new(100);
    bs.set(0, true);
    assert_eq!(bs.count(), 1);
    assert_eq!(bs.test(0), true);

    bs.set(0, false);
    assert_eq!(bs.count(), 0);
    assert_eq!(bs.test(0), false);

    bs.flip();
    bs.set(0, true);
    assert_eq!(bs.count(), 0);
    assert_eq!(bs.test(0), false);

    bs.set(0, false);
    assert_eq!(bs.count(), 1);
    assert_eq!(bs.test(0), true);
}

#[test]
fn test_bitset_reset() {
    let mut bs = Bitset::new(100);
    for idx in (0..50).step_by(2) {
        bs.set(idx, true);
    }
    bs.flip();
    bs.reset();
    for i in 0..bs.array.len() {
        assert_eq!(bs.array[i], 0);
    }
    assert_eq!(bs.flip, false);
    assert_eq!(bs.count(), 0);
}

#[test]
fn test_bitset_flip() {
    let mut bs = Bitset::new(100);
    assert_eq!(bs.flip, false);
    bs.flip();
    assert_eq!(bs.flip, true);
    bs.flip();
    assert_eq!(bs.flip, false);
}

#[test]
fn test_bitset_to_string() {
    let mut bs = Bitset::new(32);
    bs.set(9, true);
    let bs_string = bs.to_string();
    for (i, c) in bs_string.chars().enumerate() {
        if i != 9 {
            assert_eq!(c, '0');
        } else {
            assert_eq!(c, '1');
        }
    }

    bs.flip();
    let bs_string_flip = bs.to_string();
    for (i, c) in bs_string_flip.chars().enumerate() {
        if i != 9 {
            assert_eq!(c, '1');
        } else {
            assert_eq!(c, '0');
        }
    }
}

#[test]
fn test_bitset_read_write() {
    let mut bsw = Bitset::new(32);
    bsw.set(9, true);
    bsw.set(31, true);
    bsw.flip();
    let path = "./bitset.bs";
    bsw.write(path).unwrap();

    let bsr = Bitset::read(path).unwrap();
    assert_eq!(bsr.flip, bsw.flip);
    assert_eq!(bsr.size(), bsw.size());
    assert_eq!(bsr.count(), bsw.count());
    assert_eq!(bsr.array, bsw.array);

    // Clean up the test file
    fs::remove_file(path).unwrap();
}

fn main() {}
