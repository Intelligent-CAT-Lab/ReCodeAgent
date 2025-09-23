use rubiksolver::heap::Heap;

fn is_smaller_int(a: &i32, b: &i32) -> bool {
    a < b
}

#[test]
fn test_heap_with_strings() {
    let mut heap = Heap::new(10, |a: &&str, b: &&str| a < b);
    heap.insert("charlie");
    heap.insert("alpha");
    heap.insert("bravo");
    assert_eq!(heap.delete_min(), Some("alpha"));
    assert_eq!(heap.delete_min(), Some("bravo"));
    assert_eq!(heap.delete_min(), Some("charlie"));
    assert!(heap.is_empty());
}
#[test]
fn test_heap_with_integers() {
    let mut heap = Heap::new(10, is_smaller_int);
    // Insert numbers 30 down to 11.
    for i in (11..=30).rev() {
        heap.insert(i);
    }
    // The minimum should be 11.
    assert_eq!(heap.delete_min(), Some(11));
}
pub fn main(){}