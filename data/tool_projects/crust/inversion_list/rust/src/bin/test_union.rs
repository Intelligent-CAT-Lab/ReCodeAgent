use inversion_list::inversion_list::*;

#[test]
fn test() {
    let a = [1, 2, 3, 5, 7, 9];
    let b = [1, 2, 3, 5, 7, 9, 10];
    let c = [23, 12, 1];

    let set = InversionList::new(20, &a).unwrap();
    let set2 = InversionList::new(20, &b).unwrap();
    let set3 = InversionList::new(30, &c).unwrap();

    // In the C code, they did a varargs union: set1 = union(set, set2, set3, NULL).
    // In Rust, we chain union calls or do a small helper. Let's chain them here:
    let set1 = set.union(&set2).union(&set3);
    assert_eq!(set1.to_str(), "[1, 2, 3, 5, 7, 9, 10, 12, 23]");

    // Another union: set, set2 only
    let set1b = set.union(&set2);
    assert_eq!(set1b.to_str(), "[1, 2, 3, 5, 7, 9, 10]");

    println!("test_union passed.");
}
fn main(){}