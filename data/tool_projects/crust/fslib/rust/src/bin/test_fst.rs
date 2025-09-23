use fslib::fst::{Fst, State, StateData};
#[test]
fn test_fst_create(){
    let mut fst = Fst::new();
    let state_a: State = fst.add_state();
    let state_b: State = fst.add_state();
    for i in 0..10 {
        fst.add_arc(state_a, state_b, i, i, i as f32);
    }
    let mut count: u32 = 0;
    let state: &StateData = &fst.states[state_a as usize];
    assert_eq!(fst.n_states, 2);
    for a in 0..state.n_arcs{
        let arc = &state.arcs[a as usize];
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
fn test_fst_stress(){
    let mut fst = Fst::new();
    let sa = fst.add_state();
    let sb = fst.add_state();
    let arc_size: usize = 1000000;
    for i in 0..arc_size {
        fst.add_arc(sa, sb, i as u32, i as u32, i as f32);
    }
    let mut count: u32 = 0;
    assert_eq!(fst.n_states, 2);
    for i in 0..arc_size {
        let state: &StateData = &fst.states[sa as usize];
        let arc = &state.arcs[i];
        assert_eq!(arc.state, sb);
        assert_eq!(arc.ilabel, count );
        assert_eq!(arc.olabel, count);
        assert_eq!(arc.weight, count as f32);
        count += 1;
    }
    assert_eq!(count, arc_size as u32);
    fst.remove();
}
#[test]
fn test_fst_copy(){
    let mut fst_a = Fst::new();
    let mut fst_b = Fst::new();
    let sa = fst_a.add_state();
    let sb = fst_a.add_state();
    for i in 0..10 {
        fst_a.add_arc(sa, sb, i as u32, i as u32, i as f32);
    }
    fst_b.copy(&mut fst_a);
    let state = &fst_b.states[sa as usize];
    for a in 0..state.n_arcs{
        let arc = &state.arcs[a as usize];
        assert_eq!(arc.state, sb);
        assert_eq!(arc.ilabel, a);
        assert_eq!(arc.olabel, a);
        assert_eq!(arc.weight, a as f32);
    }
}
fn main (){}