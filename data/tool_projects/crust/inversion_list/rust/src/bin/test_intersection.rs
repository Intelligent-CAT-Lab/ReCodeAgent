use inversion_list::inversion_list::*;

#[test]
fn test() {
    let a = [1, 2, 3, 5, 7, 9];
    let b = [1, 2, 3, 5, 7, 9, 10];
    let c = [23, 12, 1];

    let set = InversionList::new(20, &a).unwrap();
    let set2 = InversionList::new(20, &b).unwrap();
    let set3 = InversionList::new(20, &c).unwrap();

    // intersection(set, set2, set3)
    // In Rust, we can do set.intersection(&set2).intersection(&set3)
    let inter1 = set.intersection(&set2).intersection(&set3);
    assert_eq!(inter1.to_str(), "[1, 2, 3, 5, 7, 9]");

    // intersection(set, set2)
    let inter2 = set.intersection(&set2);
    assert_eq!(inter2.to_str(), "[1, 2, 3, 5, 7, 9]");

    println!("test_intersection passed.");
}
fn main(){}
