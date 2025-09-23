use cset::cset;
#[test]
fn test_cset_init() {
    let mut cset_int = cset::Cset::<i32>::new();
    assert_eq!(cset_int.size(), 0);
    assert_eq!(cset_int.capacity(), 2);
}
#[test]
fn test_cset_add() {
    let mut cset_int = cset::Cset::<i32>::new();
    cset_int.add(34);
    assert_eq!(cset_int.size(), 1);
    cset_int.add(35);
    assert_eq!(cset_int.size(), 2);
    cset_int.add(36);
    cset_int.add(37);
    cset_int.add(38);
}
#[test]
fn test_cset_contains() {
let mut cset_int = cset::Cset::<i32>::new();
cset_int.add(34);
cset_int.add(36);
cset_int.remove(36);
assert!(!cset_int.contains(&12));
assert!(cset_int.contains(&34));
cset_int.add(50);
assert!(!cset_int.contains(&45));
assert_eq!(cset_int.size(), 2);
}
#[test]
fn test_cset_iteration() {
    let mut cset_int = cset::Cset::<i32>::new();
    for i in 0..3200 {
    cset_int.add(i);
    }
    for value in cset_int.iter() {
    assert!(cset_int.contains(&value));
    }
}
#[test]
fn test_cset_unique() {
    let mut cset_int = cset::Cset::<i32>::new();
    cset_int.add(45);
    cset_int.add(46);
    cset_int.add(57);
    assert_eq!(cset_int.size(), 3);
    cset_int.add(45);
    assert_eq!(cset_int.size(), 3);
}
#[derive(PartialEq, Eq, Hash)]
struct Node {
    x: i32,
    y: i32,
}
#[test]
fn test_cset_struct() {
    let mut cset_node = cset::Cset::<Node>::new();
    cset_node.add(Node { x: 4, y: 4 });
    assert_eq!(cset_node.size(), 1);
    cset_node.add(Node { x: 5, y: 4 });
    assert_eq!(cset_node.size(), 2);
    cset_node.add(Node { x: 5, y: 4 });
    assert_eq!(cset_node.size(), 2);
    cset_node.add(Node { x: 5, y: 8 });
    assert_eq!(cset_node.size(), 3);
}
#[test]
fn test_cset_remove() {
    let mut cset_int = cset::Cset::<i32>::new();
    cset_int.add(45);
    cset_int.add(34);
    cset_int.add(10);
    assert_eq!(cset_int.size(), 3);
    cset_int.remove(45);
    assert_eq!(cset_int.size(), 2);
    cset_int.remove(45);
    assert_eq!(cset_int.size(), 2);
    cset_int.remove(34);
    assert_eq!(cset_int.size(), 1);
    for value in cset_int.iter() {
        assert_eq!(value, 10);
    }
    cset_int.remove(10);
    assert_eq!(cset_int.size(), 0);
}
#[test]
fn test_cset_resize() {
    let mut cset_int = cset::Cset::<i32>::new();
    for i in 0..1500 {
    cset_int.add(i);
    }
}
#[test]
fn test_default_bytes_comparator() {
    let mut cset_int = cset::Cset::<i32>::new();
    cset_int.add(45);
    cset_int.add(46);
    cset_int.add(67);
    assert!(cset_int.contains(&45));
    assert!(!cset_int.contains(&68));
    assert!(cset_int.contains(&46));
    cset_int.remove(46);
    assert!(!cset_int.contains(&46));
    cset_int.remove(46);
    assert!(!cset_int.contains(&46));
    assert_eq!(cset_int.size(), 2);
    cset_int.remove(45);
    assert_eq!(cset_int.size(), 1);
    cset_int.remove(67);
    assert_eq!(cset_int.size(), 0);
    cset_int.remove(67);
    assert_eq!(cset_int.size(), 0);
    for i in 0..2000 {
    cset_int.add(i);
    }
    assert_eq!(cset_int.size(), 2000);
}
fn node_pointer_comparator(selfs: &Node, other: &Node) -> bool {
    selfs.x == other.x
}
fn custom_node_compare(selfs: &Node, other: &Node) -> bool {
    selfs.x == other.x
}
#[test]
fn test_custom_comparator() {
    let mut cset_node = cset::Cset::<Node>::new();  
    cset_node.set_comparator(custom_node_compare);
    cset_node.add(Node { x: 4, y: 4 });
    cset_node.add(Node { x: 4, y: 4 });
    assert_eq!(cset_node.size(), 1);
    cset_node.add(Node { x: 1, y: 2 });
    assert_eq!(cset_node.size(), 2);
    cset_node.remove(Node { x: 1, y: 45 });
    assert_eq!(cset_node.size(), 1);
}
#[test]
fn test_cset_clear() {
    let mut cset_int = cset::Cset::<i32>::new();
    cset_int.add(12);
    cset_int.add(14);
    cset_int.add(15);
    assert_eq!(cset_int.size(), 3);
    cset_int.clear();
    assert_eq!(cset_int.size(), 0);
    cset_int.add(45);
    assert_eq!(cset_int.size(), 1);
}
#[test]
fn test_cset_intersection() {
    let mut cset_int_a = cset::Cset::<i32>::new();
    let mut cset_int_b = cset::Cset::<i32>::new();
    cset_int_a.add(12);
    cset_int_a.add(13);
    cset_int_a.add(14);
    cset_int_b.add(12);
    cset_int_b.add(13);
    cset_int_b.add(16);
    let mut result = cset::Cset::<i32>::new();
    result.intersect(&cset_int_a, &cset_int_b);
    assert_eq!(result.size(), 2);
    cset_int_b.add(14);
    result.intersect(&cset_int_a, &cset_int_b);
    assert_eq!(result.size(), 3);
}
#[test]
fn test_cset_union() {
    let mut cset_int_a = cset::Cset::<i32>::new();
    let mut cset_int_b = cset::Cset::<i32>::new();
    let mut cset_result = cset::Cset::<i32>::new();
    cset_int_a.add(34);
    cset_int_a.add(25);
    cset_int_a.add(12);
    cset_int_b.add(1);
    cset_int_b.add(4);
    cset_int_b.add(34);
    cset_result.union(&cset_int_a, &cset_int_b);
    assert_eq!(cset_result.size(), 5);
    cset_int_b.add(100);
    cset_result.union(&cset_int_a, &cset_int_b);
    assert_eq!(cset_result.size(), 6);
}
#[test]
fn test_cset_disjoint() {
    let mut cset_a = cset::Cset::<char>::new();
    let mut cset_b = cset::Cset::<char>::new();
    cset_a.add('a');
    cset_a.add('b');
    cset_b.add('c');
    cset_b.add('d');
    assert!(cset_a.is_disjoint(&cset_b));
    cset_b.add('a');
    assert!(!cset_a.is_disjoint(&cset_b));
}
#[test]
fn test_cset_difference() {
    let mut cset_a = cset::Cset::<i32>::new();
    let mut cset_b = cset::Cset::<i32>::new();
    let mut result = cset::Cset::<i32>::new();
    result.difference(&cset_a, &cset_b);
    assert_eq!(result.size(), 0);
    cset_a.add(45);
    cset_a.add(46);
    cset_a.add(58);
    cset_b.add(12);
    cset_b.add(11);
    cset_b.add(45);
    result.difference(&cset_a, &cset_b);
    assert_eq!(result.size(), 2);
    assert!(result.contains(&46));
    assert!(result.contains(&58));
    assert!(!result.contains(&45));
    result.clear();
    cset_b.add(46);
    cset_b.add(58);
    result.difference(&cset_a, &cset_b);
    assert_eq!(result.size(), 0);
    result.difference(&cset_b, &cset_a);
    assert_eq!(result.size(), 2);
}
fn main() {}