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
fn test_compose() {
    let fst_str_0 = "0 1 1 1\n1";
    let fst_str_1 = "0 1 1 1\n1";
    let fst_str_2 = "0 1 1 1\n1";
    let mut fst_0 = Fst::new();
    let mut fst_1 = Fst::new();
    let mut fst_2 = Fst::new();
    let mut fst_3 = Fst::new();
    fst_0.compile_str(fst_str_0);
    fst_1.compile_str(fst_str_1);
    fst_2.compile_str(fst_str_2);
    fst_3.compose(&fst_0, &mut fst_1);
    fst_3.print();
    assert_fst_equal(&fst_2, &fst_3);
}
#[test]
fn test_compose_1() {
    let fst_str_0 = "0 1 1 1\n1 2 1 1\n2";
    let fst_str_1 = "0 0 1 1\n0 0 1 0\n0 0 0 1\n0";
    let fst_str_2 = "0 0 0 1\n0 1 1 1\n0 1 1 0\n1 1 0 1\n1 2 1 1\n1 2 1 0\n2 2 0 1\n2";
    let mut fst_0 = Fst::new();
    let mut fst_1 = Fst::new();
    let mut fst_2 = Fst::new();
    let mut fst_3 = Fst::new();
    fst_0.compile_str(fst_str_0);
    fst_1.compile_str(fst_str_1);
    fst_2.compile_str(fst_str_2);
    fst_3.compose(&fst_0, &mut fst_1);
    fst_3.print();
    assert_fst_equal(&fst_2, &fst_3);
}
#[test]
fn test_compose_sorted_full_0() {
    let fst_str_0 = "0 1 1 1\n1";
    let fst_str_1 = "0 1 1 1\n1";
    let fst_str_2 = "0 1 1 1\n1";
    let mut fst_0 = Fst::new();
    let mut fst_1 = Fst::new();
    let mut fst_2 = Fst::new();
    let mut fst_3 = Fst::new();
    fst_0.compile_str(fst_str_0);
    fst_1.compile_str(fst_str_1);
    fst_2.compile_str(fst_str_2);
    fst_0.arc_sort(1);
    fst_1.arc_sort(0);
    fst_3.compose(&fst_0, &mut fst_1);
    fst_3.print();
    assert_fst_equal(&fst_2, &fst_3);
}
#[test]
fn test_compose_sorted_full_1() {
    let fst_str_0 = "0 1 1 1\n1 2 1 1\n2";
    let fst_str_1 = "0 0 1 1\n0 0 1 0\n0 0 0 1\n0";
    let fst_str_2 = "0 0 0 1\n0 1 1 1\n0 1 1 0\n1 1 0 1\n1 2 1 1\n1 2 1 0\n2 2 0 1\n2";
    let mut fst_0 = Fst::new();
    let mut fst_1 = Fst::new();
    let mut fst_2 = Fst::new();
    let mut fst_3 = Fst::new();
    fst_0.compile_str(fst_str_0);
    fst_1.compile_str(fst_str_1);
    fst_2.compile_str(fst_str_2);
    fst_0.arc_sort(1);
    fst_1.arc_sort(0);
    fst_3.compose(&fst_0, &mut fst_1);
    println!("Full sorted:");
    fst_3.print();
    assert_fst_equal(&fst_2, &fst_3);
}
#[test]
fn test_compose_sorted_half_0() {
    let fst_str_0 = "0 1 1 1\n1";
    let fst_str_1 = "0 1 1 1\n1";
    let fst_str_2 = "0 1 1 1\n1";
    let mut fst_0 = Fst::new();
    let mut fst_1 = Fst::new();
    let mut fst_2 = Fst::new();
    let mut fst_3 = Fst::new();
    fst_0.compile_str(fst_str_0);
    fst_1.compile_str(fst_str_1);
    fst_2.compile_str(fst_str_2);
    fst_1.arc_sort(0);
    fst_3.compose(&fst_0, &mut fst_1);
    fst_3.print();
    assert_fst_equal(&fst_2, &fst_3);
}
#[test]
fn test_compose_sorted_half_1() {
    let fst_str_0 = "0 1 1 1\n1 2 1 1\n2";
    let fst_str_1 = "0 0 1 1\n0 0 1 0\n0 0 0 1\n0";
    let fst_str_2 = "0 0 0 1\n0 1 1 1\n0 1 1 0\n1 1 0 1\n1 2 1 1\n1 2 1 0\n2 2 0 1\n2";
    let mut fst_0 = Fst::new();
    let mut fst_1 = Fst::new();
    let mut fst_2 = Fst::new();
    let mut fst_3 = Fst::new();
    fst_0.compile_str(fst_str_0);
    fst_1.compile_str(fst_str_1);
    fst_2.compile_str(fst_str_2);
    fst_1.arc_sort(0);
    fst_3.compose(&fst_0, &mut fst_1);
    println!("Half sorted direct:");
    fst_3.print();
    assert_fst_equal(&fst_2, &fst_3);
}
#[test]
fn test_compose_sorted_half_2() {
    let fst_str_0 = "0 1 1 1\n1 2 1 1\n2";
    let fst_str_1 = "0 0 1 1\n0 0 1 0\n0 0 0 1\n0";
    let fst_str_2 = "0 0 0 1\n0 1 1 1\n0 1 1 0\n1 1 0 1\n1 2 1 1\n1 2 1 0\n2 2 0 1\n2";
    let mut fst_0 = Fst::new();
    let mut fst_1 = Fst::new();
    let mut fst_2 = Fst::new();
    let mut fst_3 = Fst::new();
    fst_0.compile_str(fst_str_0);
    fst_1.compile_str(fst_str_1);
    fst_2.compile_str(fst_str_2);
    fst_0.arc_sort(1);
    fst_3.compose(&fst_0, &mut fst_1);
    println!("Half sorted reversed:");
    fst_3.print();
    fst_2.arc_sort(0);
    fst_3.arc_sort(0);
    assert_fst_equal(&fst_2, &mut fst_3);
}
fn main(){}