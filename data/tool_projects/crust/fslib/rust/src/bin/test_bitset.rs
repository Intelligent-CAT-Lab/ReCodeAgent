use fslib::bitset::BitSet;
#[test]
fn test_create() {
    let mut bs = BitSet::new(8);
    bs.set(0);
    assert_eq!(bs.get(0), true);
}
#[test]
fn test_set_all() {
    let mut bs = BitSet::new(1024);
    bs.set_all();
    for i in 0..1024 {
        assert_eq!(bs.get(i), true);
    }
}
#[test]
fn test_clear_all() {
    let mut bs = BitSet::new(1024);
    bs.set_all();
    bs.clear_all();
    for i in 0..1024 {
        assert_eq!(bs.get(i), false);
    }
}
#[test]
fn test_clear() {
    let mut bs = BitSet::new(8);
    bs.set(0);
    assert_eq!(bs.get(0), true);
    bs.clear(0);
    assert_eq!(bs.get(0), false);
}
fn main(){}