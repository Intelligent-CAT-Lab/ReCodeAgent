use fslib::fst::{ArcData, Fst, StateData};
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
fn test_union(){
    let mut a = Fst::new();
    let mut b = Fst::new();
    let mut c = Fst::new();
    let str_a: &str = "0 1 1 1\n1";
    let str_b: &str = "0 1 2 2\n1";
    let str_c: &str = "0 1 1 1\n0 2 3 2 2\n0 2 0 0\n1\n3";
    a.compile_str(str_a);
    b.compile_str(str_b);
    c.compile_str(str_c);
    a.union(&b);
    assert_fst_equal(&a, &c);
    a.remove();
    b.remove();
    c.remove();
}
fn main(){}