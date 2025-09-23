use fslib::fst::{Fst};
use fslib::iter::FstIter;
#[test]
fn test_iter_create() {
    let mut fst = Fst::new();
    let state_a = fst.add_state();
    let state_b = fst.add_state();
    let state_c = fst.add_state();
    fst.add_arc(state_a, state_b, 1, 1, 1.0);
    fst.add_arc(state_b, state_c, 1, 1, 1.0);
    let mut iter = FstIter::new(&fst);
    assert_eq!(iter.next(), Some(state_a));
    assert_eq!(iter.state, state_a);
    assert_eq!(iter.next(), Some(state_b));
    assert_eq!(iter.state, state_b);
    assert_eq!(iter.next(), Some(state_c));
    assert_eq!(iter.state, state_c);
    assert_eq!(iter.next(), None);
}
#[test]
fn test_iter_cyclic() {
    let mut fst = Fst::new();
    let state_a = fst.add_state();
    let state_b = fst.add_state();
    let state_c = fst.add_state();
    fst.add_arc(state_a, state_b, 1, 1, 1.0);
    fst.add_arc(state_a, state_c, 1, 1, 1.0);
    fst.add_arc(state_c, state_a, 1, 1, 1.0);
    let mut iter = FstIter::new(&fst);
    assert_eq!(iter.next(), Some(state_a));
    assert_eq!(iter.state, state_a);
    assert_eq!(iter.next(), Some(state_b));
    assert_eq!(iter.state, state_b);
    assert_eq!(iter.next(), Some(state_c));
    assert_eq!(iter.state, state_c);
    assert_eq!(iter.next(), None);
}
#[test]
fn test_iter_nonaccessible() {
    let mut fst = Fst::new();
    let state_a = fst.add_state();
    let state_b = fst.add_state();
    fst.add_arc(state_b, state_a, 1, 1, 1.0);
    let mut iter = FstIter::new(&fst);
    assert_eq!(iter.next(), Some(state_a));
    assert_eq!(iter.state, state_a);
    assert_eq!(iter.next(), None);
}
#[test]
fn test_iter_visited() {
    let mut fst = Fst::new();
    let state_a = fst.add_state();
    let state_b = fst.add_state();
    fst.add_arc(state_b, state_a, 1, 1, 1.0);
    let mut iter = FstIter::new(&fst);
    while iter.next().is_some() {}
    assert_eq!(iter.visited(state_a), true);
    assert_eq!(iter.visited(state_b), false);
}
fn main() {
}