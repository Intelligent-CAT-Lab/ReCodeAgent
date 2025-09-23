use crate::sr;
use crate::fst;
use crate::symt;
use std::io::{self, BufRead};
use crate::symt::SymTable;
fn trn(token: &str, symt: &SymTable)-> usize{
    unimplemented!()
}
fn trt(token: &str, symt: &SymTable)-> usize{
    unimplemented!()
}
fn add_arc(
    fst: &mut fst::Fst,
    sa: usize,
    sb: usize,
    li: usize,
    lo: usize,
    w: f32,
) {
    unimplemented!()
}
fn add_final(fst: &mut fst::Fst, s: usize, w: f32) {
    unimplemented!()
}
fn parse_line(fst: &mut fst::Fst, buf: &mut str)->i32{
    unimplemented!()
}
fn parse_line_sym(fst: &mut fst::Fst, buf: &mut str, ist: &SymTable, ost: &SymTable, sst: &SymTable)->i32{
    unimplemented!()
}
fn parse_line_sym_acc(fst: &mut fst::Fst, buf: &mut str, ist: &SymTable, ost: &SymTable, sst: &SymTable)->i32{
    unimplemented!()
}
fn fst_compile(fst: &mut fst::Fst, fin: &mut dyn BufRead, ist: &SymTable, ost: &SymTable, sst: &SymTable, is_acc: bool)-> fst::Fst{
    unimplemented!()
}
fn fst_compile_str(fst: &mut fst::Fst, s: &str) -> fst::Fst{
    unimplemented!()
}