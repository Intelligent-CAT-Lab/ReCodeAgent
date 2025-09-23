use inversion_list::inversion_list::*;

#[test]
fn test() {
    let a = [1, 2, 4, 10];
    let set = InversionList::new(20, &a).expect("create failed");

    // The C test loops with inversion_list::inversion_list_iterator_create, next, valid, etc.
    // In Rust, we can just do:
    for x in InversionListIterator::new(&set) {
        // The test verifies membership:
        assert!(set.contains(x));
    }

    println!("test_iterator passed.");
}
fn main(){}
