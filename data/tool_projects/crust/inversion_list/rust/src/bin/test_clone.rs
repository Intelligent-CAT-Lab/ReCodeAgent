use inversion_list::inversion_list::*;

#[test]
fn test() {
    let a = [1, 2, 3, 5, 7, 8, 9, 0, 2];
    let set = InversionList::new(20, &a).expect("Failed to create set");
    let clone = set.clone(); // or set.clone_list()

    // The C code checks struct internals. We'll check capacity & support:
    assert_eq!(clone.capacity(), 20);
    assert_eq!(clone.support(), 8);

    // The original C test had 'size == 6' for the C's flexible array.
    // In Rust, we store intervals in a Vec<(u32, u32)>. Let's just check them:
    // The intervals likely are: (0,4), (5,6), (7,10). That covers 8 distinct values: {0,1,2,3,5,7,8,9}.
    assert_eq!(clone.intervals.len(), 3);
    assert_eq!(clone.intervals, vec![(0, 4), (5, 6), (7, 10)]);

    println!("test_clone passed.");
}
fn main(){}
