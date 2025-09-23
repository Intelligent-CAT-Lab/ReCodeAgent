use inversion_list::inversion_list::*;

#[test]
fn test() {
    // Simulate the original test:
    // We create a set with capacity=20 and values [1,2,3,4,10].
    let a = [1, 2, 3, 4, 10];
    let set = InversionList::new(20, &a).expect("Failed to create set");

    // We iterate over the half-open intervals (the "couples") of `set`.
    for (idx, (inf, sup)) in InversionListCoupleIterator::new(&set).enumerate() {
        // In C, the test asserts that set->couples[idx*2] == get_inf(), etc.
        // In this Rust library, we can directly get the stored intervals:
        let (expected_inf, expected_sup) = set.intervals[idx];
        assert_eq!(inf, expected_inf);
        assert_eq!(sup, expected_sup);
    }

    println!("test_couple_iterator passed.");
}
fn main(){}