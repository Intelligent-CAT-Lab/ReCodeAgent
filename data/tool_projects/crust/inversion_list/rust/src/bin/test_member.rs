use inversion_list::inversion_list::*;

#[test]
fn test() {
    let a = [1, 2, 3, 5, 7, 8, 9, 2]; // duplicates 2
    let set = InversionList::new(20, &a).expect("Failed to create set");

    // The original test checks membership for certain non-members:
    assert!(!set.contains(0));
    assert!(!set.contains(4));
    assert!(!set.contains(10));
    assert!(!set.contains(11));

    // Then checks membership for each value in a[]:
    for &val in &a {
        assert!(set.contains(val));
    }

    println!("test_member passed: membership checks are correct.");
}
fn main(){}
