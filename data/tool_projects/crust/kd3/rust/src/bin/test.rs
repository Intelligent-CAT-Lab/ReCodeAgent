use kd3::kdtree;
#[test]
fn test_kd() {
    let mut x: Vec<f64> = vec![0.5, 0.5, 0.5, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0];
    let mut y: Vec<f64> = vec![0.5, 0.5, 0.5, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0];
    let mut z: Vec<f64> = vec![0.5, 0.5, 0.5, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0];
    let mut tree: kdtree::KDTree = kdtree::KDTree::new();
    tree.build(&mut x, &mut y, &mut z, 11);
    let mut iter: Option<kdtree::KDTreeIterator> = None;
    /* match none */
    tree.search(&mut iter, -10.0, 0.0, 0.0, 9.999);
    let mut e0: Vec<usize> = vec![0];
    validate(&mut iter.as_mut().unwrap(), 1, &e0);
    /* match one */
    tree.search(&mut iter, 0.0, 0.0, 0.0, 0.499);
    let mut e1: Vec<usize> = vec![3];
    validate(&mut iter.as_mut().unwrap(), 1, &e1);
    /* match all. intersect borders */
    tree.search(&mut iter, 0.5, 0.5, 0.5, 0.5);
    let mut e2: Vec<usize> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    validate(&mut iter.as_mut().unwrap(), 11, &e2);
    /* match all. beyond borders */
    tree.search(&mut iter, 0.5, 0.5, 0.5, 100.0);
    validate(&mut iter.as_mut().unwrap(), 11, &e2);
    /* front slice */
    tree.search(&mut iter, 0.5, 0.5, 0.0, 0.5);
    let mut e3: Vec<usize> = vec![0, 1, 2, 3, 4, 5, 6];
    validate(&mut iter.as_mut().unwrap(), 7, &e3);
    /* back slice */
    tree.search(&mut iter, 0.5, 0.5, 1.0, 0.5);
    let mut e4: Vec<usize> = vec![0, 1, 2, 7, 8, 9, 10];
    validate(&mut iter.as_mut().unwrap(), 7, &e4);
    /* using generic box search to search exactly top slice */
    tree.search_space(0.0, 1.0, 0.5, 1.0, 0.0, 1.0);
    let mut e5: Vec<usize> = vec![0, 1, 2, 5, 6, 9, 10];
    validate(&mut iter.as_mut().unwrap(), 7, &e5);
}
fn validate(iter: &mut kdtree::KDTreeIterator, count: usize, v: &[usize]) {
    assert!(iter.size == count);
    let mut content = vec![0; count];
    for i in 0..count {
        content[i] = iter.get_next().as_mut().unwrap().clone();
    }
    assert!(iter.get_next().is_none());
    // sort the content
    content.sort();
    for i in 0..count {
        assert!(content[i] == v[i]);
    }
}
fn main() {}
