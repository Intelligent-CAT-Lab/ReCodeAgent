use roaring_bitmap::rset::RSet;
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}
fn rset_new_items(items: &[u16]) -> RSet {
    let mut set = RSet::new();
    for &item in items {
        set.add(item);
    }
    set
}
#[test]
fn test_new() {
    let set = RSet::new();
    assert_eq!(set.cardinality(), 0);
    assert_eq!(set.length(), std::mem::size_of::<u16>() * 2);
}
#[test]
fn test_new_items() {
    let set = rset_new_items(&[]);
    assert_eq!(set.cardinality(), 0);
    assert_eq!(set.length(), std::mem::size_of::<u16>() * 2);
    let set = rset_new_items(&[1000, 2000, 3000]);
    assert_eq!(set.cardinality(), 3);
    assert_eq!(set.length(), std::mem::size_of::<u16>() * (3 + 1));
    assert_eq!(set.buffer[0], 3);
    assert_eq!(set.buffer[1], 1000);
    assert_eq!(set.buffer[2], 2000);
    assert_eq!(set.buffer[3], 3000);
}
#[test]
fn test_equals() {
    let set = rset_new_items(&[1000, 2000, 3000]);
    let mut comparison = RSet::new();
    assert!(!set.equals(&comparison));
    comparison.add(1000);
    assert!(!set.equals(&comparison));
    comparison.add(2000);
    assert!(!set.equals(&comparison));
    comparison.add(3000);
    assert!(set.equals(&comparison));
    let comparison = rset_new_items(&[1000, 2000, 3001]);
    assert!(!set.equals(&comparison));
    let comparison = rset_new_items(&[1000, 2000, 3000]);
    assert!(set.equals(&comparison));
}
#[test]
fn test_import_export() {
    let set = rset_new_items(&[1, 2, 3]);
    let exported = set.export();
    let copy = RSet::import(&exported, set.length());
    assert!(set.equals(&copy));
}
#[test]
fn test_copy() {
    let set = RSet::new();
    let copy = set.copy();
    assert!(set.equals(&copy));
    assert_eq!(set.cardinality(), copy.cardinality());
    assert_eq!(set.length(), copy.length());
    let set = rset_new_items(&[1, 2, 3, 4, 5]);
    let copy = set.copy();
    assert!(set.equals(&copy));
    assert_eq!(set.cardinality(), copy.cardinality());
    assert_eq!(set.length(), copy.length());
}
#[test]
fn test_truncate() {
    let mut set = rset_new_items(&[1, 2, 3, 4, 5]);
    assert_eq!(set.cardinality(), 5);
    set.truncate();
    assert_eq!(set.cardinality(), 0);
}
#[test]
fn test_buffer_resizing() {
    let mut set = RSet::new();
    let mut ctr: u16 = 0;
    for i in 0..1000 {
        set.add(i);
        ctr += 1;
    }
    println!("{}", ctr);
    assert_eq!(set.cardinality(), 1000);
}
#[test]
fn test_array_to_bitset() {
    let mut set = RSet::new();
    for i in 0..32768 {
        set.add(i * 2);
    }
    assert_eq!(set.cardinality(), 32768);
    // Additional checks can be added based on the specific implementation details
}
#[test]
fn test_bitset_to_inverted_array() {
    let mut set = RSet::new();
    for i in 0..=61440 {
        set.add(i);
    }
    assert_eq!(set.cardinality(), 61441);
    // Additional checks can be added based on the specific implementation details
}
#[test]
fn test_fill_ascending() {
    let mut set = RSet::new();
    let mut comparison = RSet::new();
    for i in 0..=65535 {
        set.add(i);
        comparison.add(i);
        assert!(set.equals(&comparison));
    }
    assert_eq!(set.cardinality(), 65536);
    assert_eq!(set.length(), std::mem::size_of::<u16>());
}
#[test]
fn test_fill_descending() {
    let mut set = RSet::new();
    let mut comparison = RSet::new();
    for i in (0..=65535).rev() {
        set.add(i);
        comparison.add(i);
        assert!(set.equals(&comparison));
    }
    assert_eq!(set.cardinality(), 65536);
    assert_eq!(set.length(), std::mem::size_of::<u16>());
}
#[test]
fn test_fill_optimal() {
    let mut set = RSet::new();
    let mut comparison = RSet::new();
    for i in 0..32768 {
        set.add(i);
        comparison.add(i);
        assert!(set.equals(&comparison));
    }
    for i in (32768..=65535).rev() {
        set.add(i);
        comparison.add(i);
        assert!(set.equals(&comparison));
    }
    assert_eq!(set.cardinality(), 65536);
    assert_eq!(set.length(), std::mem::size_of::<u16>());
}
#[test]
fn test_contains() {
    let mut set = RSet::new();
    for i in 0..=65535 {
        assert!(!set.contains(i));
        set.add(i);
        assert!(set.contains(i));
    }
}
#[test]
fn test_invert() {
    let mut set = RSet::new();
    for i in 4..=65535 {
        set.add(i);
    }
    let mut inverted = RSet::new();
    set.invert(&mut inverted);
    let expected = rset_new_items(&[0, 1, 2, 3]);
    assert!(inverted.equals(&expected));
    let mut inverted_twice = RSet::new();
    inverted.invert(&mut inverted_twice);
    assert!(set.equals(&inverted_twice));
    set.truncate();
    let mut expected = RSet::new();
    for i in 0..=65535 {
        expected.add(i);
    }
    set.invert(&mut inverted);
    assert_eq!(inverted.cardinality(), 65536);
    assert!(inverted.equals(&expected));
    inverted.invert(&mut inverted_twice);
    assert_eq!(inverted_twice.cardinality(), 0);
    assert!(set.equals(&inverted_twice));
    set.truncate();
    for i in 0..30000 {
        set.add(i);
    }
    expected.truncate();
    for i in 30000..=65535 {
        expected.add(i);
    }
    set.invert(&mut inverted);
    assert_eq!(inverted.cardinality(), 35536);
    assert!(inverted.equals(&expected));
    inverted.invert(&mut inverted_twice);
    assert_eq!(inverted_twice.cardinality(), 30000);
    assert!(set.equals(&inverted_twice));
}
#[test]
fn test_intersection() {
    let mut a = RSet::new();
    let mut b = rset_new_items(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let mut result = RSet::new();
    a.intersection(&b, &mut result);
    assert_eq!(result.cardinality(), 0);
    b.intersection(&a, &mut result);
    assert_eq!(result.cardinality(), 0);
    a.fill();
    a.intersection(&b, &mut result);
    assert!(b.equals(&result));
    b.intersection(&a, &mut result);
    assert!(b.equals(&result));
    a.truncate();
    for i in (0..100).step_by(2) {
        a.add(i);
    }
    a.intersection(&b, &mut result);
    let expected = rset_new_items(&[0, 2, 4, 6, 8]);
    assert!(result.equals(&expected));
    b.truncate();
    for i in (1..100).step_by(2) {
        b.add(i);
    }
    a.intersection(&b, &mut result);
    assert_eq!(result.cardinality(), 0);
}
fn main() {}