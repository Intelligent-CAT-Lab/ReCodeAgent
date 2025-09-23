use fslib::fst::Fst;
#[test]
fn test_print() {
    let mut fst = Fst::new();
    let state_a = fst.add_state();
    let state_b = fst.add_state();
    for i in 0..10 {
        fst.add_arc(state_a, state_b, i, i, i as f32);
    }
    fst.set_final(state_a, 1.0);
    fst.set_final(state_b, 1.0);
    fst.print();
    assert_eq!(fst.n_states, 2);  
}
fn main() {
}