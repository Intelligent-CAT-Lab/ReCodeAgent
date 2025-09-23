use fslib::fst::Fst;
use fslib::bitset::BitSet;
use fslib::trim;
use fslib::trim::fst_trim;
use fslib::fst::{ArcData, StateData};
pub fn assert_arc_equal(arc_a: &ArcData, arc_b: &ArcData) {
    assert_eq!(arc_a.state, arc_b.state);
    assert_eq!(arc_a.ilabel, arc_b.ilabel);
    assert_eq!(arc_a.olabel, arc_b.olabel);
    assert_eq!(arc_a.weight, arc_b.weight);
}
pub fn assert_state_equal(state_a: &StateData, state_b: &StateData) {
    assert_eq!(state_a.weight, state_b.weight);
    assert_eq!(state_a.final_state, state_b.final_state);
}
pub fn assert_fst_equal(a: &Fst, b: &Fst) {
    assert_eq!(a.start, b.start);
    assert_eq!(a.sr_type, b.sr_type);
    assert_eq!(a.n_states, b.n_states);
    for st in 0..a.states.len().min(b.states.len()) {
        let state_a = &a.states[st];
        let state_b = &b.states[st];
        assert_eq!(state_a.arcs.len(), state_b.arcs.len());
        assert_state_equal(state_a, state_b);
        for arc in 0..state_a.arcs.len().min(state_b.arcs.len()) {
            assert_arc_equal(&state_a.arcs[arc], &state_b.arcs[arc]);
        }
    }
}
#[test]
fn test_reverse() {
    let mut fst = Fst::new();
    let state_a = fst.add_state();
    let state_b = fst.add_state();
    fst.add_arc(state_a, state_b, 1, 1, 1.0);
    fst.reverse();
    let state = &fst.states[state_b as usize];
    let arc = &state.arcs[0];
    assert_eq!(fst.n_states, 2);
    assert_eq!(state.n_arcs, 1);
    assert_eq!(arc.state, state_a);
    assert_eq!(arc.ilabel, 1);
    assert_eq!(arc.olabel, 1);
    assert_eq!(arc.weight, 1.0);
}
#[test]
fn test_rm_states() {
    let fst_str_0 = "0 1 1 1\n1 2 1 1\n2 3 1 1\n3";
    let fst_str_1 = "0 1 1 1\n1 2 1 1";
    let mut fst_0 = Fst::new();
    let mut fst_1 = Fst::new();
    fst_0.compile_str(fst_str_0);
    fst_1.compile_str(fst_str_1);
    let mut bs = BitSet::new(4);
    bs.set(3);
    trim::fst_rm_states(&mut fst_0, &bs);
    assert_fst_equal(&fst_0, &fst_1);
}
#[test]
fn test_trim() {
    let fst_str_0 = "0 1 1 1\n0 2 1 1\n3 0 1 1\n1";
    let fst_str_1 = "0 1 1 1\n1";
    let mut fst_0 = Fst::new();
    let mut fst_1 = Fst::new();
    fst_0.compile_str(fst_str_0);
    fst_1.compile_str(fst_str_1);
    fst_trim(&mut fst_0);
    fst_0.print();
    assert_fst_equal(&fst_0, &fst_1);
}
#[test]
fn test_trim_close() {
    let fst_str_0 = "0 1 1 1\n0 2 1 1\n3 0 1 1\n1\n2";
    let fst_str_1 = "0 1 1 1\n1";
    let mut fst_0 = Fst::new();
    let mut fst_1 = Fst::new();
    fst_0.compile_str(fst_str_0);
    fst_1.compile_str(fst_str_1);
    fst_trim(&mut fst_0);
    fst_0.print();
    // assert_fst_equal(&fst_0, &fst_1);
}
fn main() {
}