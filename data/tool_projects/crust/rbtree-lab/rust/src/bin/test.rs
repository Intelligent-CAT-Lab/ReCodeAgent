use rbtree::rbtree::{Key, RBTree};

#[test]
fn test_init() {
    let t = RBTree::new();
    assert!(t.root.is_none());
}

#[test]
fn test_insert_single() {
    let mut t = RBTree::new();
    let key: i32 = 1024;
    let p = t.rbtree_insert(key);
    assert!(p.is_some());
    let p = p.unwrap();

    assert_eq!(t.root.as_ref().unwrap().borrow().key, key);
    assert_eq!(p.borrow().key, key);
    assert!(p.borrow().left.is_none());
    assert!(p.borrow().right.is_none());
    assert!(p.borrow().parent.is_none());
}

#[test]
fn test_find_single() {
    let mut t = RBTree::new();
    let key: Key = 512;
    let wrong_key: Key = 1024;

    let p = t.rbtree_insert(key);
    let q = t.rbtree_find(key);
    assert!(q.is_some());
    assert_eq!(q.as_ref().unwrap().borrow().key, key);
    assert_eq!(
        q.as_ref().unwrap().borrow().key,
        p.unwrap().borrow().key
    );

    let q = t.rbtree_find(wrong_key);
    assert!(q.is_none());
}

#[test]
fn test_erase_root() {
    let mut t = RBTree::new();
    let key: i32 = 128;
    let p = t.rbtree_insert(key);
    assert!(p.is_some());
    assert_eq!(t.root.as_ref().unwrap().borrow().key, key);

    t.erase(p.as_ref().unwrap().clone());
    assert!(t.root.is_none());
}

fn insert_arr(t: &mut RBTree, arr: &[i32]) {
    for &key in arr {
        t.rbtree_insert(key);
    }
}

// #[test]
// fn test_minmax() {
//     let mut arr = vec![10, 5, 8, 34, 67, 23, 156, 24, 2, 12];
//     let n = arr.len();
//     assert!(n > 0);
//     let mut t = RBTree::new();
//     insert_arr(&mut t, &arr);
//     arr.sort();
//     let p = t.rbtree_min();
//     assert!(p.is_some());
//     assert_eq!(p.unwrap().borrow().key, arr[0]);
//     let q = t.rbtree_max();
//     assert!(q.is_some());
//     assert_eq!(q.unwrap().borrow().key, arr[n - 1]);
//     t.erase(p.as_ref().unwrap().clone());
//     let p = t.rbtree_min();
//     assert!(p.is_some());
//     assert_eq!(p.as_ref().unwrap().borrow().key, arr[1]);
//     if n >= 2 {
//         t.erase(q.unwrap());
//         let q = t.rbtree_max();
//         assert!(q.is_some());
//         assert_eq!(q.unwrap().borrow().key, arr[n - 2]);
//     }
// }

#[test]
fn test_to_array() {
    let mut t = RBTree::new();
    let arr = vec![10, 5, 8, 34, 67, 23, 156, 24, 2, 12, 24, 36, 990, 25];
    let n = arr.len();
    insert_arr(&mut t, &arr);

    let mut sorted_arr = arr.clone();
    sorted_arr.sort();

    let res = t.to_array(n);
    for i in 0..n {
        assert_eq!(sorted_arr[i], res[i]);
    }
}

#[test]
fn test_multi_instance() {
    let mut t1 = RBTree::new();
    let mut t2 = RBTree::new();

    let arr1 = vec![10, 5, 8, 34, 67, 23, 156, 24, 2, 12, 24, 36, 990, 25];
    let n1 = arr1.len();
    insert_arr(&mut t1, &arr1);
    let mut sorted_arr1 = arr1.clone();
    sorted_arr1.sort();

    let arr2 = vec![4, 8, 10, 5, 3];
    let n2 = arr2.len();
    insert_arr(&mut t2, &arr2);
    let mut sorted_arr2 = arr2.clone();
    sorted_arr2.sort();

    let res1 = t1.to_array(n1);
    for i in 0..n1 {
        assert_eq!(sorted_arr1[i], res1[i]);
    }

    let res2 = t2.to_array(n2);
    for i in 0..n2 {
        assert_eq!(sorted_arr2[i], res2[i]);
    }
}

fn main() {}