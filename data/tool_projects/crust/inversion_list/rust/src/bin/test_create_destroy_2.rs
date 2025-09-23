use inversion_list::inversion_list::*;

#[test]
fn test() {
    let a = [1, 2, 3, 5, 7, 8, 9, 0];
    let set = InversionList::new(20, &a).expect("Failed to create set");

    // Check capacity, support, intervals, etc. as in the C test:
    assert_eq!(set.capacity(), 20);
    assert_eq!(set.support(), 8);
    // "size == 6" in the original C means 3 intervals in Rust:
    assert_eq!(set.intervals.len(), 3);
    assert_eq!(set.intervals, vec![(0, 4), (5, 6), (7, 10)]);

    println!("test_create_destroy_2 passed.");
}
fn main(){}