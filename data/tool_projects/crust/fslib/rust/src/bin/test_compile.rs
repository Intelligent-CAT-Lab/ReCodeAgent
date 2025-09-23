use fslib::fst::{Fst, StateData, ArcData};

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
fn test_fst_compile() {
    let mut fst = Fst::new();
    let input = "0 1 1 1 1.0\n1 1.0";
    fst.compile_str(input);
    assert_eq!(fst.n_states, 2);
    let s0 = &fst.states[0];
    let s1 = &fst.states[1];
    assert_eq!(s0.final_state, false);
    assert_eq!(s1.final_state, true);
    // assert_eq!(s1.weight, 1.0);
    assert_eq!(s0.arcs.len(), 1);
    assert_eq!(s0.arcs[0].ilabel, 1);
    assert_eq!(s0.arcs[0].olabel, 1);
    assert_eq!(s0.arcs[0].weight, 1.0);
}
#[test]
fn test_test_equal() {
    let mut fst_a = Fst::new();
    let mut fst_b = Fst::new();
    let str_a = "0 1 1 1\n1 1.0";
    let str_b = "0 1 1 1\n1 1.0";
    fst_a.compile_str(str_a);
    fst_b.compile_str(str_b);
    assert_fst_equal(&fst_a, &fst_b);
}
fn main (){}