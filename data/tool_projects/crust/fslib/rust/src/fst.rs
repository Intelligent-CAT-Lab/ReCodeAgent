use std::fs::File;
use std::io::{self, Read, Write};
use crate::sr::Sr;
use crate::bitset::BitSet;
use crate::queue::Queue;
use crate::symt::SymTable;
use std::collections::VecDeque;
pub type State = u32;
pub type Arc = u32;
pub type Label = u32;
pub type Weight = f32;
const FST_HEADER: u32 = 0x66733031;
const ISORT: u8 = 0x01;
const OSORT: u8 = 0x02;
const EPS: u32 = 0;
const EPS_L: i32 = -1;
pub const START_STATE: &str = "<start>";
pub struct Fst {
    pub start: State,
    pub n_states: State,
    pub n_max: State,
    pub sr_type: u8,
    pub flags: u8,
    pub states: Vec<StateData>,
}
pub struct StateData {
    pub n_arcs: Arc,
    pub n_max: Arc,
    pub weight: Weight,
    pub final_state: bool,
    pub arcs: Vec<ArcData>,
}
pub struct ArcData {
    pub state: State,
    pub weight: Weight,
    pub ilabel: Label,
    pub olabel: Label,
}
pub struct Spair {
    pub a: State,
    pub b: State,
}
pub struct Striple {
    pub a: State,
    pub b: State,
    pub c: State,
}
pub struct Apair {
    pub a: Arc,
    pub b: Arc,
}
pub struct ArcPair {
    pub a: ArcData,
    pub b: ArcData,
}
pub struct MatchItem {
    pub a: ArcData,
    pub b: ArcData,
}
impl Fst {
    pub fn new() -> Self {
        unimplemented!()
    }
    pub fn remove(&mut self) {
        unimplemented!()
    }
    pub fn empty(&mut self) {
        unimplemented!()
    }
    pub fn add_state(&mut self) -> State {
        unimplemented!()
    }
    pub fn add_arc(&mut self, src: State, dst: State, il: Label, ol: Label, weight: Weight) -> Arc {
        unimplemented!()
    }
    pub fn set_final(&mut self, s: State, w: Weight) {
        unimplemented!()
    }
    pub fn print(&self) {
        unimplemented!()
    }
    pub fn print_sym(&self, ist: &SymTable, ost: &SymTable, sst: &SymTable) {
        unimplemented!()
    }
    pub fn write(&self, fout: &mut File) -> io::Result<()> {
        unimplemented!()
    }
    pub fn read(&mut self, fin: &mut File) -> io::Result<()> {
        unimplemented!()
    }
    pub fn fwrite(&self, filename: &str) -> io::Result<()> {
        unimplemented!()
    }
    pub fn fread(&mut self, filename: &str) -> io::Result<()> {
        unimplemented!()
    }
    pub fn compile(&mut self, fin: &mut File, ist: &SymTable, ost: &SymTable, sst: &SymTable, is_acc: bool) -> Self {
        unimplemented!()
    }
    pub fn compile_str(&mut self, str_data: &str) -> Self {
        unimplemented!()
    }
    pub fn get_n_arcs(&self) -> Arc {
        unimplemented!()
    }
    pub fn arc_sort(&mut self, sort_outer: i32) {
        unimplemented!()
    }
    pub fn stack(&mut self, other: &Fst) {
        unimplemented!()
    }
    pub fn union(&mut self, other: &Fst) -> Self {
        unimplemented!()
    }
    pub fn draw(&self, fout: &mut File) -> io::Result<i32> {
        unimplemented!()
    }
    pub fn draw_sym(&self, fout: &mut File, ist: &SymTable, ost: &SymTable, sst: &SymTable) -> io::Result<i32> {
        unimplemented!()
    }
    pub fn copy(&self, copy: &mut Fst) {
        unimplemented!()
    }
    pub fn reverse(&mut self) {
        unimplemented!()
    }
    pub fn shortest(&self, path: &mut Fst) -> Self {
        unimplemented!()
    }
    pub fn rm_states(&mut self, visited: &BitSet) -> Self {
        unimplemented!()
    }
    pub fn trim(&mut self) -> Self {
        unimplemented!()
    }
    pub fn compose(&self, fst_b: &Fst, fst_c: &mut Fst) {
        unimplemented!()
    }
    pub fn relabel(&mut self, old: Label, new: Label, dir: i32) {
        unimplemented!()
    }
}
pub fn match_unsorted(a: &[ArcData], b: &[ArcData], m: Arc, n: Arc, q: &mut Queue<(ArcData, ArcData)>) {
    unimplemented!()
}
pub fn match_half_sorted(a: &[ArcData], b: &[ArcData], m: Arc, n: Arc, q: &mut Queue<(ArcData, ArcData)>) {
    unimplemented!()
}
pub fn match_half_sorted_rev(a: &[ArcData], b: &[ArcData], m: Arc, n: Arc, q: &mut Queue<(ArcData, ArcData)>) {
    unimplemented!()
}
pub fn match_full_sorted(a: &[ArcData], b: &[ArcData], m: Arc, n: Arc, q: &mut Queue<(ArcData, ArcData)>) {
    unimplemented!()
}
pub fn match_arcs(fst_a: &Fst, fst_b: &Fst, pair: &Spair, sr: &Sr, mq: &mut Queue<(ArcData, ArcData)>) {
    unimplemented!()
}