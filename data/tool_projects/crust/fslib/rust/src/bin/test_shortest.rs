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
fn test_shortest() {
    let fst_str = "0 1 1 1 1.0\n0 1 2 2 2.0\n1 2 3 3 1.0\n1 2 4 4 2.0\n2";
    let fst_str_path = "0 1 1 1 1.0\n1 2 3 3 1.0\n2";
    let mut fst = Fst::new();
    let mut path = Fst::new();
    let mut expected_path = Fst::new();
    fst.compile_str(fst_str);
    expected_path.compile_str(fst_str_path);
    fst.shortest(&mut path);
    assert_fst_equal(&expected_path, &path);
    fst.remove();
    path.remove();
    expected_path.remove();
}
#[test]
fn test_shortest_three() {
    let fst_str = "0 1 1 1 1.0\n0 2 2 2 2.0\n1 3 3 3 1.0\n2 3 4 4 2.0\n0 3 5 5 4.0\n3";
    let fst_str_path = "0 1 1 1 1.0\n1 2 3 3 1.0\n2";
    let mut fst = Fst::new();
    let mut path = Fst::new();
    let mut expected_path = Fst::new();
    fst.compile_str(fst_str);
    expected_path.compile_str(fst_str_path);
    fst.shortest(&mut path);
    expected_path.print();
    path.print();
    assert_fst_equal(&expected_path, &path);
    fst.remove();
    path.remove();
    expected_path.remove();
}
#[test]
fn test_shortest_cyclic() {
    let fst_str = "0 1 1 1 1.0\n1 2 2 2 1.0\n1 0 3 3 1.0\n2";
    let fst_str_path = "0 1 1 1 1.0\n1 2 2 2 1.0\n2";
    let mut fst = Fst::new();
    let mut path = Fst::new();
    let mut expected_path = Fst::new();
    fst.compile_str(fst_str);
    expected_path.compile_str(fst_str_path);
    fst.shortest(&mut path);
    assert_fst_equal(&expected_path, &path);
    fst.remove();
    path.remove();
    expected_path.remove();
}
#[test]
fn test_shortest_reflexive() {
    let fst_str = "0 1 1 1 1.0\n1 2 2 2 1.0\n1 1 3 3 1.0\n2";
    let fst_str_path = "0 1 1 1 1.0\n1 2 2 2 1.0\n2";
    let mut fst = Fst::new();
    let mut path = Fst::new();
    let mut expected_path = Fst::new();
    fst.compile_str(fst_str);
    expected_path.compile_str(fst_str_path);
    fst.shortest(&mut path);
    assert_fst_equal(&expected_path, &path);
    fst.remove();
    path.remove();
    expected_path.remove();
}
fn main() {
}