use std::fs::File;
use std::io::{self, Write};
use crate::fst::Fst;
use crate::symt::SymTable;
const HEADER: &str = "digraph T {\n\trankdir = LR;\n\torientation = Landscape;\n";
const FOOTER: &str = "}\n";
pub fn fst_draw(fst: &Fst, fout: &mut File) -> io::Result<()> {
    unimplemented!()
}
pub fn fst_draw_sym(fst: &Fst, fout: &mut File, ist: Option<&SymTable>, ost: Option<&SymTable>, sst: Option<&SymTable>) -> io::Result<()> {
    unimplemented!()
}
fn trn(st: &mut SymTable, id: usize, token: &str)-> String {
    unimplemented!()
}
fn trt(st: &mut SymTable, id: usize, token: &str)-> String {
    unimplemented!()
}