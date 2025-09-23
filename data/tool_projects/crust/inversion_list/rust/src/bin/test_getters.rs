use inversion_list::inversion_list::*;

#[test]
fn test() {
    let a = [1, 2, 3, 5, 7, 8, 9, 0, 2];
    let set = InversionList::new(20, &a).expect("Failed to create set");
    // The C test checks capacity=20, support=8.
    assert_eq!(set.capacity(), 20);
    assert_eq!(set.support(), 8);

    println!("test_getters passed: capacity=20, support=8 as expected.");
}
fn main(){}