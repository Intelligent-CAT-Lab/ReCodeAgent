use inversion_list::inversion_list::*;

#[test]
fn test() {
    // We'll define helper to check the intervals after complement:
    fn check_complement(
        a: &[u32],
        cap: u32,
        expected_support: u32,
        expected_intervals: &[(u32, u32)],
    ) {
        let set = InversionList::new(cap, a).expect("failed to create set");
        let complement = set.complement();
        assert_eq!(complement.capacity(), cap);
        assert_eq!(complement.support(), expected_support);
        assert_eq!(complement.intervals, expected_intervals);
        println!("Complement of {:?} in [0..{}): OK", a, cap);
    }

    // 1) a=[1,2,3,5,7,8,9,0,2], capacity=20 => expect support=12, intervals=6 pairs => 3 intervals in Rust
    //   The original C code: couples = [4,5,6,7,10,20]
    check_complement(
        &[1, 2, 3, 5, 7, 8, 9, 0, 2],
        20,
        12,
        &[(4, 5), (6, 7), (10, 20)],
    );

    // 2) a=[1,2,3,5,7,8,9,2], capacity=20 => support=13 => intervals => [0..1,4..5,6..7,10..20]?
    check_complement(
        &[1, 2, 3, 5, 7, 8, 9, 2],
        20,
        13,
        &[(0, 1), (4, 5), (6, 7), (10, 20)],
    );

    // 3) a=[1,2,3,5,7,8,9,2,19], capacity=20 => support=12 => intervals => [0..1,4..5,6..7,10..19]
    check_complement(
        &[1, 2, 3, 5, 7, 8, 9, 2, 19],
        20,
        12,
        &[(0, 1), (4, 5), (6, 7), (10, 19)],
    );

    // 4) a=[1,2,3,5,7,8,9,2,19,0], capacity=20 => support=11 => intervals => [4..5,6..7,10..19]
    check_complement(
        &[1, 2, 3, 5, 7, 8, 9, 2, 19, 0],
        20,
        11,
        &[(4, 5), (6, 7), (10, 19)],
    );

    println!("test_complement passed.");
}
fn main() {} // This is the entry point for the binary
