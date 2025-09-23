use crate::queue::Queue;
use crate::fst::{ArcData, Fst};
use crate::bitset::BitSet;
pub fn fst_close(fst: &mut Fst, finals: &mut Queue<(ArcData, ArcData)>) {
    unimplemented!()
}
pub fn fst_reverse(fst: &mut Fst) {
    unimplemented!()
}
pub fn fst_rm_states(fst: &mut Fst, mask: &BitSet) {
    unimplemented!()
}
pub fn fst_get_finals(fst: &mut Fst, finals: &mut Queue<(ArcData, ArcData)>) {
    unimplemented!()
}
pub fn fst_trim(fst: &mut Fst) {
    unimplemented!()
}