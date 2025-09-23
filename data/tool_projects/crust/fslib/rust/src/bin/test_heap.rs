use fslib::heap::Heap;
use std::cmp::Ordering;
fn cmp_lower(a: &i32, b: &i32) -> Ordering {
    a.cmp(b)
}
#[test]
fn test_heap_create() {
    let mut h = Heap::new(cmp_lower, std::mem::size_of::<i32>(), 10, 0);
    assert_eq!(h.n_items, 0);
    assert_eq!(h.n_max, 10);
    h.remove();
}
#[test]
fn test_heap_pop() {
    let a = [1, 0, 4, 3, 2];
    let mut h = Heap::new(cmp_lower, std::mem::size_of::<i32>(), 10, 0);
    for &val in &a {
        h.insert(val);
    }
    assert_eq!(h.pop(), Some(0));
    assert_eq!(h.pop(), Some(1));
    assert_eq!(h.pop(), Some(2));
    assert_eq!(h.pop(), Some(3));
    assert_eq!(h.pop(), Some(4));
    assert_eq!(h.pop(), None);
    h.remove();
}
#[test]
fn test_heap_equal() {
    let a = [1, 1, 1, 0, 0];
    let mut h = Heap::new(cmp_lower, std::mem::size_of::<i32>(), 10, 0);
    for &val in &a {
        h.insert(val);
    }
    assert_eq!(h.pop(), Some(0));
    assert_eq!(h.pop(), Some(0));
    assert_eq!(h.pop(), Some(1));
    assert_eq!(h.pop(), Some(1));
    assert_eq!(h.pop(), Some(1));
    assert_eq!(h.pop(), None);
    h.remove();
}
#[test]
fn test_heap_update() {
    let mut a = [3, 2, 1];
    let mut h = Heap::new(cmp_lower, std::mem::size_of::<i32>(), 10, 0);
    h.insert(a[0]);
    h.insert(a[1]);
    h.update(a[2], 1);
    assert_eq!(h.pop(), Some(1));
    assert_eq!(h.pop(), Some(2));
    h.remove();
}
#[test]
fn test_heap_find() {
    let a = [3, 2, 1];
    let mut h = Heap::new(cmp_lower, std::mem::size_of::<i32>(), 10, 0);
    h.insert(a[0]);
    h.insert(a[1]);
    h.insert(a[2]);
    assert_eq!(h.find(&1), Some(0));
    h.remove();
}
#[test]
fn test_heap_resize() {
    let a = [0, 1, 3];
    let mut h = Heap::new(cmp_lower, std::mem::size_of::<i32>(), 1, 0);
    h.insert(a[0]);
    assert_eq!(h.n_items, 1);
    assert_eq!(h.n_max, 1);
    h.insert(a[1]);
    assert_eq!(h.n_items, 2);
    assert_eq!(h.n_max, 2);
    h.insert(a[2]);
    assert_eq!(h.n_items, 3);
    assert_eq!(h.n_max, 4);
    h.remove();
}
#[test]
fn test_heap_limit() {
    let a = [1, 0, 4, 3, 2];
    let mut h = Heap::new(cmp_lower, std::mem::size_of::<i32>(), 0, 3);
    for &val in &a {
        h.insert(val);
    }
    assert_eq!(h.pop(), Some(0));
    assert_eq!(h.pop(), Some(1));
    assert_eq!(h.pop(), Some(2));
    assert_eq!(h.pop(), None);
    h.remove();
}
fn main() {
}