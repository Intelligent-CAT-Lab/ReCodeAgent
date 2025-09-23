use vec::vec;

#[test]
fn test_vec_push() {
    let mut v = Vec::new();
    for i in 0..1000 {
        v.push(i * 2);
    }
    assert_eq!(v[1], 2);
    assert_eq!(v[999], 999 * 2);
    assert_eq!(v.push(10), ());
}

#[test]
fn test_vec_pop() {
    let mut v = Vec::new();
    v.push(123);
    v.push(456);
    v.push(789);
    assert_eq!(v.pop(), Some(789));
    assert_eq!(v.pop(), Some(456));
    assert_eq!(v.pop(), Some(123));
}

#[test]
fn test_vec_splice() {
    let mut v: Vec<i32> = (0..1000).collect();
    vec::vec_splice(&mut v, 0, 10);
    assert_eq!(v[0], 10);
    vec::vec_splice(&mut v, 10, 10);
    assert_eq!(v[10], 30);
    let v_length = v.len();
    vec::vec_splice(&mut v, v_length - 50, 50);
    let v_length = v.len();
    assert_eq!(v[v_length - 1], 949);
}

#[test]
fn test_vec_swapsplice() {
    let mut v: Vec<i32> = (0..10).collect();
    vec::vec_swapsplice(&mut v, 0, 3);
    assert_eq!(v[0], 7);
    assert_eq!(v[1], 8);
    assert_eq!(v[2], 9);
    let v_length = v.len();
    vec::vec_swapsplice(&mut v, v_length - 1, 1);
    let v_length = v.len();
    assert_eq!(v[v_length - 1], 5);
}

#[test]
fn test_vec_insert() {
    let mut v: Vec<i32> = Vec::new();
    for i in 0..1000 {
        vec::vec_insert(&mut v, 0,i);
    }
    let mut v_length = v.len();
    // print first 5 elements of the vector v
    assert_eq!(v[0], 999);
    assert_eq!(v[v_length - 1], 0);
    vec::vec_insert(&mut v, 10, 123);
    assert_eq!(v[10], 123);
    assert_eq!(v.len(), 1001);
    v_length = v.len();
    vec::vec_insert(&mut v, v_length - 2,678);
    assert_eq!(v[999], 678);
    assert_eq!(vec::vec_insert(&mut v, 10,123), 0);
    v_length = v.len();
    vec::vec_insert(&mut v, v_length, 789);
    assert_eq!(v[v.len() - 1], 789);
}

#[test]
fn test_vec_sort() {
    let mut v = vec![3, -1, 0];
    v.sort();
    assert_eq!(v[0], -1);
    assert_eq!(v[1], 0);
    assert_eq!(v[2], 3);
}

#[test]
fn test_vec_swap() {
    let mut v = vec!['a', 'b', 'c'];
    vec::vec_swap(&mut v, 0, 2);
    assert_eq!(v[0], 'c');
    assert_eq!(v[2], 'a');
    vec::vec_swap(&mut v, 0, 1);
    assert_eq!(v[0], 'b');
    assert_eq!(v[1], 'c');
    vec::vec_swap(&mut v, 1, 2);
    assert_eq!(v[1], 'a');
    assert_eq!(v[2], 'c');
    vec::vec_swap(&mut v, 1, 1);
    assert_eq!(v[1], 'a');
}

#[test]
fn test_vec_truncate() {
    let mut v = vec![0; 1000];
    v.truncate(10000);
    assert_eq!(v.len(), 1000);
    v.truncate(900);
    assert_eq!(v.len(), 900);
}

#[test]
fn test_vec_clear() {
    let mut v = vec![1, 2];
    v.clear();
    assert_eq!(v.len(), 0);
}

#[test]
fn test_vec_first() {
    let mut v = vec![0xf00d, 0];
    assert_eq!(v.first(), Some(&0xf00d));
}

#[test]
fn test_vec_last() {
    let mut v = vec![0, 0xf00d];
    assert_eq!(v.last(), Some(&0xf00d));
}

#[test]
fn test_vec_reserve() {
    let mut v: Vec<i32> = Vec::with_capacity(100);
    assert_eq!(v.capacity(), 100);
    v.reserve(50);
    assert_eq!(v.capacity(), 100);
    let mut v: Vec<i32> = Vec::with_capacity(100);
    v.push(123);
    v.push(456);
    vec::vec_reserve(&mut v, 200);
    assert_eq!(v.capacity(), 200);
    assert_eq!(vec::vec_reserve(&mut v, 300), 0);
}

#[test]
fn test_vec_compact() {
    let mut v = vec![0; 1000];
    v.truncate(3);
    vec::vec_compact(&mut v);
    assert_eq!(v.len(), v.capacity());
    assert_eq!(vec::vec_compact(&mut v), 0);
}

#[test]
fn test_vec_pusharr() {
    let a = [5, 6, 7, 8, 9];
    let mut v = vec![1, 2];
    v.extend_from_slice(&a);
    assert_eq!(v[0], 1);
    assert_eq!(v[2], 5);
    assert_eq!(v[6], 9);
    let mut v = Vec::new();
    v.extend_from_slice(&a);
    assert_eq!(v[0], 5);
}

#[test]
fn test_vec_extend() {
    let mut v = vec![12, 34];
    let v2 = vec![56, 78];
    v.extend(v2);
    assert_eq!(v[0], 12);
    assert_eq!(v[1], 34);
    assert_eq!(v[2], 56);
    assert_eq!(v[3], 78);
    assert_eq!(v.len(), 4);
}

#[test]
fn test_vec_find() {
    let v: Vec<char> = ('a'..='z').collect();
    assert_eq!(v.iter().position(|&x| x == 'a'), Some(0));
    assert_eq!(v.iter().position(|&x| x == 'z'), Some(25));
    assert_eq!(v.iter().position(|&x| x == 'd'), Some(3));
    assert_eq!(v.iter().position(|&x| x == '_'), None);
}

#[test]
fn test_vec_remove() {
    let mut v: Vec<char> = ('a'..='z').collect();
    v.retain(|&x| x != '_');
    assert_eq!(v.len(), 26);
    v.retain(|&x| x != 'c');
    assert_eq!(v[0], 'a');
    assert_eq!(v[1], 'b');
    assert_eq!(v[2], 'd');
    assert_eq!(v[3], 'e');
    assert_eq!(v.len(), 25);
}

#[test]
fn test_vec_reverse() {
    let mut v = vec!['a', 'b', 'c', 'd'];
    v.reverse();
    assert_eq!(v.len(), 4);
    assert_eq!(v[0], 'd');
    assert_eq!(v[1], 'c');
    assert_eq!(v[2], 'b');
    assert_eq!(v[3], 'a');
}

#[test]
fn test_vec_foreach() {
    let v = vec![19, 31, 47];
    let mut count = 0;
    let mut acc = 1;
    for (i, &x) in v.iter().enumerate() {
        acc *= x + count;
        count += 1;
    }
    assert_eq!(acc, (19 + 0) * (31 + 1) * (47 + 2));
}

#[test]
fn test_vec_foreach_rev() {
    let v = vec![19, 31, 47];
    let mut count = 0;
    let mut acc = 1;
    for (i, &x) in v.iter().rev().enumerate() {
        acc *= x + count;
        count += 1;
    }
    assert_eq!(acc, (19 + 2) * (31 + 1) * (47 + 0));
}

#[test]
fn test_vec_foreach_ptr() {
    let v = vec![19, 31, 47];
    let mut count = 0;
    let mut acc = 1;
    for (i, x) in v.iter().enumerate() {
        acc *= x + count;
        count += 1;
    }
    assert_eq!(acc, (19 + 0) * (31 + 1) * (47 + 2));
}

#[test]
fn test_vec_foreach_ptr_rev() {
    let v = vec![19, 31, 47];
    let mut count = 0;
    let mut acc = 1;
    for (i, x) in v.iter().rev().enumerate() {
        acc *= x + count;
        count += 1;
    }
    assert_eq!(acc, (19 + 2) * (31 + 1) * (47 + 0));
}

fn main() {}