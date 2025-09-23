use inversion_list::inversion_list::*;

#[test]
fn test() {
    let a = [1, 2, 3];
    let b = [2];
    let c = [3, 4];

    let set = InversionList::new(20, &a).unwrap();
    let set2 = InversionList::new(20, &b).unwrap();
    let set3 = InversionList::new(20, &c).unwrap();

    // difference(set, set2, set3)
    let diff1 = set.difference(&set2).difference(&set3);
    assert_eq!(diff1.to_str(), "[1]");

    // difference(set, set2)
    let diff2 = set.difference(&set2);
    assert_eq!(diff2.to_str(), "[1, 3]");

    // symmetric_difference(set, set3)
    let symdiff = set.symmetric_difference(&set3);
    assert_eq!(symdiff.to_str(), "[1, 2, 4]");

    println!("test_difference passed.");
}
fn main(){}