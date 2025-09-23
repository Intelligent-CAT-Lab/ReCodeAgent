use fslib::hash::HashTable;
fn int_hash(x: &i32) -> usize {
    *x as usize
}
#[test]
fn test_hash_table() {
    let mut hash_table = HashTable::new(int_hash, 1003);
    assert!(hash_table.get(&0).is_none());
    for i in 0..1500 {
        hash_table.insert(i, i);
    }
    for i in 0..1500 {
        assert_eq!(hash_table.get(&i), Some(&i));
    }
    assert!(hash_table.get(&1500).is_none());
}
#[test]
fn test_hash_resize() {
    let mut hash_table = HashTable::new(int_hash, 1);
    for i in 0..4 {
        hash_table.insert(i, i);
    }
    assert!(hash_table.buckets.len() >= 4);
}
#[test]
fn test_load_int() {
    let mut hash_table = HashTable::new(int_hash, 10000);
    for i in 0..100000 {
        hash_table.insert(i, i);
    }
    for i in 0..100000 {
        assert_eq!(hash_table.get(&i), Some(&i));
    }
}
#[test]
fn test_hash_update() {
    let mut hash_table = HashTable::new(int_hash, 1);
    hash_table.insert(0, 0);
    assert_eq!(hash_table.get(&0), Some(&0));
    hash_table.insert(0, 1);
    assert_eq!(hash_table.get(&0), Some(&1));
}
#[test]
fn test_hash_delete() {
    let mut hash_table = HashTable::new(int_hash, 1);
    hash_table.insert(0, 0);
    assert_eq!(hash_table.get(&0), Some(&0));
    hash_table.remove(&0);
    assert!(hash_table.get(&0).is_none());
}
fn main() {}