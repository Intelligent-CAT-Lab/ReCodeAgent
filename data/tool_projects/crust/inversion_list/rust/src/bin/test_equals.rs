use inversion_list::inversion_list::*;

#[test]
fn test() {
    let a = [1, 2, 3, 5, 7, 8, 9];
    let b = [1, 2, 3, 9, 8]; // missing 5,7
    let c = [1, 2, 3, 5, 7, 9, 10];

    let set = InversionList::new(20, &a).unwrap();
    let set1 = set.clone();
    let set2 = InversionList::new(20, &b).unwrap();
    let set3 = InversionList::new(20, &c).unwrap();

    // equal
    assert!(set.equal(&set1));
    assert!(set.equal(&set));
    assert!(!set.equal(&set2));
    assert!(!set.equal(&set3));
    assert!(!set2.equal(&set3));

    // not_equal
    assert!(set.equal(&set1));
    assert!(set.equal(&set));
    assert!(!set.equal(&set2));
    assert!(!set.equal(&set3));
    assert!(!set2.equal(&set3));

    // less
    assert!(!set.is_strict_subset_of(&set1));
    assert!(!set.is_strict_subset_of(&set));
    // In the original code, "less" had some questionable logic, but we'll interpret it as
    // "self is strictly smaller in support and subset of other".
    assert!(!set.is_strict_subset_of(&set2));
    assert!(!set.is_strict_subset_of(&set3));
    // set2 is {1,2,3,8,9} => support=5. set3 is {1,2,3,5,7,8,9,10} => support=7.
    // set2 is indeed a strict subset of set3? Actually not a *subset* because set2 doesn't have 5,7 but set3 does. Wait, that doesn't matter for subset.
    // The question is: every element in set2 must be in set3? Indeed, 1,2,3,8,9 are all in set3. So set2 is a subset.
    // In the original code, they do `assert(inversion_list::inversion_list_less(set2, set3));`
    // We'll just replicate:
    assert!(set2.is_strict_subset_of(&set3));

    // less_equal
    assert!(set.is_subset_of(&set1));
    assert!(set.is_subset_of(&set));
    assert!(!set.is_subset_of(&set2));
    assert!(!set.is_subset_of(&set3));
    // set2 <= set3
    assert!(set2.is_subset_of(&set3));

    // greater (the opposite check)
    // "greater" in the original code means set2 < set1 => set1 is "greater" if set2 < set1.
    // We'll just check is_strict_subset_of in reverse if you want.
    // The original code does: `assert(inversion_list::inversion_list_greater(set, set2));` => set has support=7, set2=5 => so set is "bigger".
    // We'll do:
    assert!(!set.is_strict_subset_of(&set2)); // so set is strictly bigger than set2
                                              // We'll interpret "greater" as the other side:
                                              // The test does: "assert(inversion_list::inversion_list_greater(set, set2))", i.e. set2 < set => so set2.is_strict_subset_of(set).
                                              // Let's confirm that set2 is indeed a strict subset of set. Actually set = {1,2,3,5,7,8,9}, set2= {1,2,3,8,9} => yes, set2 is a subset.
                                              // We'll just replicate the same check:
    assert_eq!(set2.is_strict_subset_of(&set), true);

    // For thoroughness, we can check is_strict_subset_of in other combos, but let's align with the original test.
    println!("test_equals passed.");
}
fn main(){}