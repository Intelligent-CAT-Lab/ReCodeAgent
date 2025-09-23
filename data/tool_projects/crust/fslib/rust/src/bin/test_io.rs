use fslib::fst::Fst;
#[test]
fn test_fst_read_write() {
    let filename = "tests/test_io.t";
    let mut fst = Fst::new();
    let state_a = fst.add_state();
    let state_b = fst.add_state();
    for i in 0..10 {
        fst.add_arc(state_a, state_b, i, i, i as f32);
    }
    fst.set_final(state_b, 1.0);
    fst.fwrite(filename).unwrap();
    fst.remove();
    let mut fst = Fst::new();
    fst.fread(filename).unwrap();
    let state = &fst.states[0];
    let mut count = 0;
    assert_eq!(fst.n_states, 2);
    assert_eq!(fst.states[0].final_state, false);
    assert_eq!(fst.states[1].final_state, true);
    assert_eq!(fst.states[1].weight, 1.0);
    for arc in &state.arcs {
        assert_eq!(arc.state, state_b);
        assert_eq!(arc.ilabel, count);
        assert_eq!(arc.olabel, count);
        assert_eq!(arc.weight, count as f32);
        count += 1;
    }
    assert_eq!(count, 10);
    fst.remove();
}
#[test]
fn test_fst_stress() {
    let mut fst = Fst::new();
    let sa = fst.add_state();
    let sb = fst.add_state();
    let arcs_size = 1000000;
    for i in 0..arcs_size {
        fst.add_arc(sa, sb, i, i, i as f32);
    }
    let state = &fst.states[sa as usize];
    let mut count = 0;
    assert_eq!(fst.n_states, 2);
    for arc in &state.arcs {
        assert_eq!(arc.state, sb);
        assert_eq!(arc.ilabel, count);
        assert_eq!(arc.olabel, count);
        assert_eq!(arc.weight, count as f32);
        count += 1;
    }
    assert_eq!(count, arcs_size);
    fst.remove();
}
fn main(){}