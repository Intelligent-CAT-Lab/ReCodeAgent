use inversion_list::inversion_list::*;

#[test]
fn test() {
    let a = [1, 2, 3, 5, 7, 8, 9, 0, 2];
    let set = InversionList::new(20, &a).expect("Failed to create set");
    assert_eq!(set.capacity(), 20);
    assert_eq!(set.support(), 8);
    // intervals => likely (0,4), (5,6), (7,10)
    assert_eq!(set.intervals.len(), 3);
    assert_eq!(set.intervals, vec![(0, 4), (5, 6), (7, 10)]);

    println!("test_create_destroy_3 passed.");
}
fn main(){}
