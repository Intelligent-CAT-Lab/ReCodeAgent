use FastHamming::fast_hamming::{log2uint8};

#[test]
fn test_log2uint8() {
    assert_eq!(log2uint8(1), 0);
    assert_eq!(log2uint8(2), 1);
    assert_eq!(log2uint8(3), 1);
    assert_eq!(log2uint8(4), 2);
    assert_eq!(log2uint8(7), 2);
    assert_eq!(log2uint8(8), 3);
    assert_eq!(log2uint8(16), 4);
    assert_eq!(log2uint8(32), 5);
    assert_eq!(log2uint8(64), 6);
    assert_eq!(log2uint8(128), 7);
    assert_eq!(log2uint8(255), 7);
}
fn main(){}