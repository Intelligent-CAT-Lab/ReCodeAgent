use inversion_list::inversion_list::*;

#[test]
fn test() {
    let a = [1, 2, 3, 5, 7, 8, 9, 2];
    let set = InversionList::new(20, &a).expect("create failed");

    // The original test expects "[1, 2, 3, 5, 7, 8, 9]".
    assert_eq!(set.to_str(), "[1, 2, 3, 5, 7, 8, 9]");

    println!("test_to_string passed.");
}
fn main(){}